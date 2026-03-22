use std::env;
use std::path::PathBuf;

fn main() {
    println!("cargo:rerun-if-env-changed=SIMPLEITK_DIR");
    println!("cargo:rerun-if-changed=CMakeLists.txt");
    println!("cargo:rerun-if-changed=src/lib.rs");
    println!("cargo:rerun-if-changed=src/sitk_bridge.h");
    println!("cargo:rerun-if-changed=src/sitk_bridge.cpp");

    let install_dir = resolve_simpleitk();
    let include_dir = find_include_dir(&install_dir);
    let lib_dir = install_dir.join("lib");

    println!("cargo:rustc-link-search={}", lib_dir.display());

    link_simpleitk_libs(&lib_dir);

    // macOS requires these system frameworks for a static ITK/SimpleITK build
    if cfg!(target_os = "macos") {
        println!("cargo:rustc-link-lib=framework=CoreFoundation");
        println!("cargo:rustc-link-lib=framework=CoreServices");
        println!("cargo:rustc-link-lib=c++");
    } else {
        println!("cargo:rustc-link-lib=stdc++");
    }

    run_cxx_build(&include_dir);
}

// ---------------------------------------------------------------------------
// Locate or build SimpleITK
// ---------------------------------------------------------------------------

fn resolve_simpleitk() -> PathBuf {
    // 1. Explicit override
    if let Ok(dir) = env::var("SIMPLEITK_DIR") {
        let p = PathBuf::from(dir);
        eprintln!("cargo:warning=Using SIMPLEITK_DIR={}", p.display());
        return p;
    }

    // 2. Common system locations
    for base in ["/usr/local", "/opt/homebrew", "/usr"] {
        let candidate = PathBuf::from(base);
        if candidate.join("include").join("SimpleITK.h").exists()
            || candidate.join("include").join("SimpleITK").join("SimpleITK.h").exists()
        {
            eprintln!("cargo:warning=Found system SimpleITK at {base}");
            return candidate;
        }
    }

    // 3. Build from source via CMake
    eprintln!("cargo:warning=SimpleITK not found on system — building from source (this will take a while)");
    build_simpleitk()
}

fn build_simpleitk() -> PathBuf {
    let out_dir = PathBuf::from(env::var("OUT_DIR").unwrap());
    let build_dir = out_dir.join("simpleitk_cmake_build");
    let install_dir = build_dir.join("install");
    let jobs = env::var("NUM_JOBS").unwrap_or_else(|_| num_cpus().to_string());

    std::fs::create_dir_all(&build_dir).unwrap();

    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    // Configure — call cmake directly to avoid the cmake crate injecting
    // Rust's cross-compilation flags (e.g. -mmacosx-version-min) into ITK.
    let status = std::process::Command::new("cmake")
        .arg(&manifest_dir)
        .arg("-B").arg(&build_dir)
        .arg(format!("-DCMAKE_INSTALL_PREFIX={}", install_dir.display()))
        .arg(format!("-DCMAKE_BUILD_PARALLEL_LEVEL={jobs}"))
        .arg("-DCMAKE_BUILD_TYPE=Release")
        .status()
        .expect("cmake configure failed — is cmake installed?");
    assert!(status.success(), "cmake configure step failed");

    // Build the ALL target — ExternalProject_Add is part of ALL and handles
    // its own install into CMAKE_INSTALL_PREFIX.
    let status = std::process::Command::new("cmake")
        .arg("--build").arg(&build_dir)
        .arg("--parallel").arg(&jobs)
        .status()
        .expect("cmake build failed");
    assert!(status.success(), "cmake build step failed");

    // The SuperBuild installs ITK into its own prefix (ITK-prefix/) separate
    // from the SimpleITK install. Also install ITK into our unified install_dir
    // so all .a files are in one place for the linker.
    let itk_build = build_dir
        .join("SimpleITK-prefix/src/SimpleITK-build/ITK-build");
    if itk_build.exists() {
        let status = std::process::Command::new("cmake")
            .arg("--install").arg(&itk_build)
            .arg("--prefix").arg(&install_dir)
            .status()
            .expect("cmake install of ITK failed");
        assert!(status.success(), "ITK install step failed");
    }

    install_dir
}

fn num_cpus() -> usize {
    std::thread::available_parallelism()
        .map(|n| n.get())
        .unwrap_or(4)
}

// ---------------------------------------------------------------------------
// Locate the SimpleITK.h header
// ---------------------------------------------------------------------------

fn find_include_dir(install_dir: &PathBuf) -> PathBuf {
    let include = install_dir.join("include");

    for candidate in [include.clone(), include.join("SimpleITK")] {
        if candidate.join("SimpleITK.h").exists() {
            return candidate;
        }
    }

    // SimpleITK installs headers under a versioned dir: include/SimpleITK-X.Y/
    if let Ok(entries) = std::fs::read_dir(&include) {
        for entry in entries.flatten() {
            let name = entry.file_name();
            let name = name.to_string_lossy();
            if name.starts_with("SimpleITK-") {
                let candidate = entry.path();
                if candidate.join("SimpleITK.h").exists() {
                    return candidate;
                }
            }
        }
    }

    include
}

// ---------------------------------------------------------------------------
// Link all installed SimpleITK/ITK static libraries
// ---------------------------------------------------------------------------

fn link_simpleitk_libs(lib_dir: &PathBuf) {
    if let Ok(entries) = std::fs::read_dir(lib_dir) {
        let mut libs: Vec<String> = entries
            .flatten()
            .filter_map(|e| {
                let name = e.file_name();
                let s = name.to_string_lossy();
                if s.starts_with("lib") && s.ends_with(".a") {
                    Some(s[3..s.len() - 2].to_string())
                } else {
                    None
                }
            })
            .collect();
        // Alphabetical sort: Common < IO < Registration < _ITK* (works for SITK layout)
        libs.sort();
        for lib in libs {
            println!("cargo:rustc-link-lib=static={lib}");
        }
    }
}

// ---------------------------------------------------------------------------
// Compile the CXX bridge
// ---------------------------------------------------------------------------

fn run_cxx_build(include_dir: &PathBuf) {
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());

    let mut build = cxx_build::bridge("src/lib.rs");
    build
        .file(manifest_dir.join("src/sitk_bridge.cpp"))
        .include(include_dir)   // .../include/SimpleITK-2.4/
        .include(&manifest_dir) // resolves include!("src/sitk_bridge.h")
        .std("c++17")
        .warnings(false);       // ITK headers produce many warnings

    // macOS: Command Line Tools split libc++ between the compiler built-ins
    // and the SDK. Use -nostdinc++ and add the SDK's c++/v1 path explicitly
    // so all standard headers (including cxxabi.h) are found consistently.
    if cfg!(target_os = "macos") {
        if let Ok(out) = std::process::Command::new("xcrun")
            .args(["--show-sdk-path"])
            .output()
        {
            let sdk = String::from_utf8_lossy(&out.stdout).trim().to_string();
            build
                .flag("-nostdinc++")
                .flag(&format!("-isystem{}/usr/include/c++/v1", sdk));
        }
    }

    build.compile("simpleitk-sys-bridge");
}

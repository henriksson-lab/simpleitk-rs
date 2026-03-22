# simpleitk-rs

Rust bindings for [SimpleITK](https://simpleitk.org/) — a high-level image processing library built on ITK, widely used in medical imaging.

## Overview

The workspace contains two crates:

| Crate | Purpose |
|-------|---------|
| `simpleitk-sys` | Low-level CXX bridge to the `itk::simple` C++ API |
| `simpleitk` | Safe, idiomatic Rust wrapper (public API) |

Bindings are generated at build time using the [`cxx`](https://cxx.rs) crate, which provides safe interop with C++ without requiring a hand-written C API layer.

## Requirements

- Rust 1.75+
- CMake 3.20+
- A C++17-capable compiler (AppleClang, GCC, or MSVC)
- Internet access on first build (downloads SimpleITK 2.4.0 ~300 MB)

On macOS, Xcode Command Line Tools are sufficient.

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
simpleitk = { path = "path/to/simpleitk-rs/simpleitk" }
```

```rust
use simpleitk::io;

fn main() {
    let image = io::read_image("input.nii").expect("failed to read");
    println!("{}x{}x{} ({}D)", image.width(), image.height(), image.depth(), image.dimension());
    io::write_image(&image, "output.nii.gz").expect("failed to write");
}
```

## Building

### Automatic (vendored build)

The first build downloads and compiles SimpleITK 2.4.0 from source. This takes 30–60 minutes but is fully automatic:

```sh
cargo build
```

### Using a pre-installed SimpleITK

Point `SIMPLEITK_DIR` at an existing install prefix to skip the vendored build:

```sh
SIMPLEITK_DIR=/path/to/simpleitk/install cargo build
```

The install prefix must contain `lib/libSimpleITK*.a` and `include/SimpleITK*/SimpleITK.h`. Both the SimpleITK and ITK static libraries must be present under `lib/`.

### Running the example

```sh
cargo run --example read_write -- input.jpg output.png
```

## API

### `simpleitk::io`

```rust
// Read any format SimpleITK supports (DICOM, NIfTI, MetaImage, PNG, TIFF, JPEG, ...)
pub fn read_image(path: &str) -> Result<Image, String>

// Write — format inferred from file extension
pub fn write_image(image: &Image, path: &str) -> Result<(), String>
```

### `simpleitk::Image`

```rust
pub fn width(&self)      -> u32
pub fn height(&self)     -> u32
pub fn depth(&self)      -> u32   // 0 for 2D images
pub fn dimension(&self)  -> u32   // 2 or 3
pub fn num_pixels(&self) -> u64
pub fn pixel_id(&self)   -> i32   // itk::simple::PixelIDValueEnum
```

`Image` implements `Debug`, `Send`, and drops the underlying C++ object automatically.

## Architecture

```
simpleitk-sys/
├── src/
│   ├── lib.rs            # #[cxx::bridge] — declares the FFI interface
│   ├── sitk_bridge.h     # C++ glue declarations (sitk_rs namespace)
│   └── sitk_bridge.cpp   # C++ glue implementations
├── build.rs              # Locates/builds SimpleITK; drives cxx_build
└── CMakeLists.txt        # Vendored SimpleITK 2.4.0 ExternalProject

simpleitk/
├── src/
│   ├── image.rs          # Image struct wrapping cxx::UniquePtr<ffi::Image>
│   ├── io.rs             # read_image / write_image
│   └── lib.rs
└── examples/
    └── read_write.rs
```

The `cxx` bridge wraps `itk::simple::Image` as an opaque type accessed through `UniquePtr<Image>`. Drop, move semantics, and exception propagation are all handled by the bridge automatically.

## Expanding Coverage

After a successful build, the installed SimpleITK headers are at:

```
target/*/build/simpleitk-sys-*/out/simpleitk_cmake_build/install/include/SimpleITK-2.4/
```

To add a new function:

1. Declare it in `simpleitk-sys/src/sitk_bridge.h` (in the `sitk_rs` namespace)
2. Implement it in `simpleitk-sys/src/sitk_bridge.cpp`
3. Add the signature to the `#[cxx::bridge]` in `simpleitk-sys/src/lib.rs`
4. Expose a safe wrapper in `simpleitk/src/`

## Platform Notes

**macOS 26.x (Sequoia/Tahoe beta):** The Command Line Tools ship a split libc++ where `cxxabi.h` lives inside the SDK rather than the compiler's built-in include path. `build.rs` detects this automatically and passes `-nostdinc++ -isystem <SDK>/usr/include/c++/v1` to the C++ compiler.

## License

Apache 2.0 — same as SimpleITK and ITK.

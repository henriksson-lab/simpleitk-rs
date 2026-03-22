# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project goal

Rust FFI bindings for [SimpleITK](https://simpleitk.org/) — a high-level C++ image processing library built on ITK, used widely in medical imaging.

## Build commands

```sh
# Check the workspace compiles (requires SimpleITK to be reachable)
cargo check

# Full build — downloads and compiles SimpleITK on first run (~30–60 min)
cargo build

# Use a pre-installed SimpleITK to skip the vendored build
SIMPLEITK_DIR=/path/to/install cargo build

# Run the read/write example
cargo run --example read_write -- input.nii output.nii
```

Tests: there are no tests yet. When adding them, run with `cargo test -p simpleitk`.

## Architecture

The workspace follows the standard `-sys` crate pattern:

```
simpleitk-sys/   — raw unsafe bindings
simpleitk/       — safe Rust wrapper (public API)
```

### `simpleitk-sys`

- `build.rs` — locates SimpleITK in three ways, in priority order:
  1. `SIMPLEITK_DIR` env var
  2. Common system paths (`/usr/local`, `/opt/homebrew`, `/usr`)
  3. Vendored build via `cmake::Config` + `CMakeLists.txt` (downloads SimpleITK 2.4.0 from GitHub using `ExternalProject_Add`)
- `CMakeLists.txt` — drives the vendored build; installs into `$OUT_DIR/simpleitk_cmake_build/build/install/`
- `src/lib.rs` — thin shell that `include!`s the `bindgen`-generated `bindings.rs` from `OUT_DIR`

`bindgen` is configured to emit only `sitk_*` symbols (the SimpleITK C API) and suppress `std_*` to avoid pulling in the entire C++ stdlib.

### `simpleitk` (safe wrapper)

- `src/image.rs` — `Image` struct wrapping `*mut sitk_Image`; implements `Drop` (calls `sitk_Image_delete`) and `Send`
- `src/io.rs` — `read_image` / `write_image` free functions using `CString` for path conversion
- `src/lib.rs` — re-exports `Image`; modules are `image` and `io`

## Expanding coverage

After a successful build, inspect available C API symbols:

```sh
grep "^pub fn" target/*/build/simpleitk-sys-*/out/bindings.rs
```

Add new safe wrappers in `simpleitk/src/` as additional modules. Each new `sitk_*` type needs:
1. A struct wrapping the raw pointer
2. A `Drop` impl calling the corresponding `sitk_*_delete`
3. Methods delegating to `sitk_*` functions via `unsafe`

## SimpleITK C API conventions

- Types follow `sitk_TypeName` (opaque pointer)
- Constructors: `sitk_TypeName_new(...)` → `*mut sitk_TypeName`
- Destructors: `sitk_TypeName_delete(ptr)`
- Methods: `sitk_TypeName_MethodName(ptr, ...)`
- Top-level functions: `sitk_ReadImage`, `sitk_WriteImage`, etc.

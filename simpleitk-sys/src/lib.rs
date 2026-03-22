//! CXX bridge to SimpleITK (itk::simple namespace).
//! Prefer the `simpleitk` crate for the public API.

#[cxx::bridge(namespace = "sitk_rs")]
pub mod ffi {
    unsafe extern "C++" {
        include!("src/sitk_bridge.h");

        type Image;

        fn read_image(path: &str) -> Result<UniquePtr<Image>>;
        fn write_image(image: &Image, path: &str) -> Result<()>;

        fn get_width(image: &Image) -> u32;
        fn get_height(image: &Image) -> u32;
        fn get_depth(image: &Image) -> u32;
        fn get_dimension(image: &Image) -> u32;
        fn get_number_of_pixels(image: &Image) -> u64;
        fn get_pixel_id_value(image: &Image) -> i32;
    }
}

use simpleitk_sys::ffi;
use crate::image::Image;

/// Read an image from disk. Supports any format SimpleITK supports
/// (DICOM, NIfTI, MetaImage, PNG, TIFF, ...).
pub fn read_image(path: &str) -> Result<Image, String> {
    ffi::read_image(path)
        .map(Image::from_unique_ptr)
        .map_err(|e| format!("SimpleITK failed to read '{path}': {e}"))
}

/// Write an image to disk. Format is inferred from the file extension.
pub fn write_image(image: &Image, path: &str) -> Result<(), String> {
    ffi::write_image(&image.inner, path)
        .map_err(|e| format!("SimpleITK failed to write '{path}': {e}"))
}

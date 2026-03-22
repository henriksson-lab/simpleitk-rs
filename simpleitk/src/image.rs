use cxx::UniquePtr;
use simpleitk_sys::ffi;

/// A SimpleITK image. Owns the underlying C++ object; frees it on drop.
pub struct Image {
    pub(crate) inner: UniquePtr<ffi::Image>,
}

// itk::simple::Image owns its pixel data exclusively — safe to move across threads.
unsafe impl Send for Image {}

impl Image {
    pub(crate) fn from_unique_ptr(ptr: UniquePtr<ffi::Image>) -> Self {
        assert!(!ptr.is_null(), "null Image pointer from SimpleITK");
        Self { inner: ptr }
    }

    pub fn width(&self) -> u32 {
        ffi::get_width(&self.inner)
    }

    pub fn height(&self) -> u32 {
        ffi::get_height(&self.inner)
    }

    pub fn depth(&self) -> u32 {
        ffi::get_depth(&self.inner)
    }

    pub fn dimension(&self) -> u32 {
        ffi::get_dimension(&self.inner)
    }

    pub fn num_pixels(&self) -> u64 {
        ffi::get_number_of_pixels(&self.inner)
    }

    pub fn pixel_id(&self) -> i32 {
        ffi::get_pixel_id_value(&self.inner)
    }
}

impl std::fmt::Debug for Image {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Image")
            .field("width", &self.width())
            .field("height", &self.height())
            .field("depth", &self.depth())
            .field("dimension", &self.dimension())
            .finish()
    }
}

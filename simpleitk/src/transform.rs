use simpleitk_sys::ffi;
use cxx::UniquePtr;
use crate::image::Image;

pub struct Transform {
    pub(crate) inner: UniquePtr<ffi::Transform>,
}

unsafe impl Send for Transform {}

impl Transform {
    fn from_ptr(ptr: UniquePtr<ffi::Transform>) -> Self {
        assert!(!ptr.is_null());
        Self { inner: ptr }
    }

    fn wrap_t(result: Result<UniquePtr<ffi::Transform>, cxx::Exception>) -> Result<Transform, String> {
        result.map(Transform::from_ptr).map_err(|e| e.to_string())
    }

    // ── Factories ──────────────────────────────────────────────────────────

    pub fn affine(dimensions: u32) -> Result<Transform, String> {
        Self::wrap_t(ffi::new_affine_transform(dimensions))
    }
    pub fn translation(dimensions: u32) -> Result<Transform, String> {
        Self::wrap_t(ffi::new_translation_transform(dimensions))
    }
    pub fn scale(dimensions: u32) -> Result<Transform, String> {
        Self::wrap_t(ffi::new_scale_transform(dimensions))
    }
    pub fn euler2d() -> Result<Transform, String> {
        Self::wrap_t(ffi::new_euler2d_transform())
    }
    pub fn euler3d() -> Result<Transform, String> {
        Self::wrap_t(ffi::new_euler3d_transform())
    }
    pub fn similarity2d() -> Result<Transform, String> {
        Self::wrap_t(ffi::new_similarity2d_transform())
    }
    pub fn similarity3d() -> Result<Transform, String> {
        Self::wrap_t(ffi::new_similarity3d_transform())
    }
    pub fn versor() -> Result<Transform, String> {
        Self::wrap_t(ffi::new_versor_transform())
    }
    pub fn versor_rigid3d() -> Result<Transform, String> {
        Self::wrap_t(ffi::new_versor_rigid3d_transform())
    }
    pub fn scale_versor3d() -> Result<Transform, String> {
        Self::wrap_t(ffi::new_scale_versor3d_transform())
    }
    pub fn scale_skew_versor3d() -> Result<Transform, String> {
        Self::wrap_t(ffi::new_scale_skew_versor3d_transform())
    }
    pub fn composite(dimensions: u32) -> Result<Transform, String> {
        Self::wrap_t(ffi::new_composite_transform(dimensions))
    }
    pub fn displacement_field(dimensions: u32) -> Result<Transform, String> {
        Self::wrap_t(ffi::new_displacement_field_transform(dimensions))
    }
    pub fn read(path: &str) -> Result<Transform, String> {
        Self::wrap_t(ffi::read_transform(path))
    }

    // ── Base methods (immutable) ───────────────────────────────────────────

    pub fn get_parameters(&self) -> Vec<f64> {
        ffi::transform_get_parameters(&self.inner).into_iter().collect()
    }
    pub fn get_fixed_parameters(&self) -> Vec<f64> {
        ffi::transform_get_fixed_parameters(&self.inner).into_iter().collect()
    }
    pub fn number_of_parameters(&self) -> u64 {
        ffi::transform_get_number_of_parameters(&self.inner)
    }
    pub fn number_of_fixed_parameters(&self) -> u64 {
        ffi::transform_get_number_of_fixed_parameters(&self.inner)
    }
    pub fn transform_point(&self, point: &[f64]) -> Vec<f64> {
        ffi::transform_transform_point(&self.inner, point).into_iter().collect()
    }
    pub fn is_linear(&self) -> bool {
        ffi::transform_is_linear(&self.inner)
    }
    pub fn dimension(&self) -> u32 {
        ffi::transform_get_dimension(&self.inner)
    }
    pub fn name(&self) -> String {
        ffi::transform_get_name(&self.inner)
    }
    pub fn write(&self, path: &str) -> Result<(), String> {
        ffi::transform_write(&self.inner, path).map_err(|e| e.to_string())
    }
    pub fn inverse(&self) -> Result<Transform, String> {
        Self::wrap_t(ffi::transform_get_inverse(&self.inner))
    }

    // ── Base methods (mutable) ─────────────────────────────────────────────

    pub fn set_parameters(&mut self, params: &[f64]) {
        ffi::transform_set_parameters(self.inner.pin_mut(), params);
    }
    pub fn set_fixed_parameters(&mut self, params: &[f64]) {
        ffi::transform_set_fixed_parameters(self.inner.pin_mut(), params);
    }
    pub fn set_identity(&mut self) {
        ffi::transform_set_identity(self.inner.pin_mut());
    }

    // ── AffineTransform ────────────────────────────────────────────────────

    pub fn affine_get_matrix(&self) -> Vec<f64> {
        ffi::affine_get_matrix(&self.inner).into_iter().collect()
    }
    pub fn affine_get_center(&self) -> Vec<f64> {
        ffi::affine_get_center(&self.inner).into_iter().collect()
    }
    pub fn affine_get_translation(&self) -> Vec<f64> {
        ffi::affine_get_translation(&self.inner).into_iter().collect()
    }
    pub fn affine_set_matrix(&mut self, matrix: &[f64]) {
        ffi::affine_set_matrix(self.inner.pin_mut(), matrix);
    }
    pub fn affine_set_center(&mut self, center: &[f64]) {
        ffi::affine_set_center(self.inner.pin_mut(), center);
    }
    pub fn affine_set_translation(&mut self, translation: &[f64]) {
        ffi::affine_set_translation(self.inner.pin_mut(), translation);
    }

    // ── TranslationTransform ───────────────────────────────────────────────

    pub fn translation_get_offset(&self) -> Vec<f64> {
        ffi::translation_get_offset(&self.inner).into_iter().collect()
    }
    pub fn translation_set_offset(&mut self, offset: &[f64]) {
        ffi::translation_set_offset(self.inner.pin_mut(), offset);
    }

    // ── ScaleTransform ─────────────────────────────────────────────────────

    pub fn scale_get_scale(&self) -> Vec<f64> {
        ffi::scale_get_scale(&self.inner).into_iter().collect()
    }
    pub fn scale_get_center(&self) -> Vec<f64> {
        ffi::scale_get_center(&self.inner).into_iter().collect()
    }
    pub fn scale_set_scale(&mut self, scale: &[f64]) {
        ffi::scale_set_scale(self.inner.pin_mut(), scale);
    }
    pub fn scale_set_center(&mut self, center: &[f64]) {
        ffi::scale_set_center(self.inner.pin_mut(), center);
    }

    // ── Euler2DTransform ───────────────────────────────────────────────────

    pub fn euler2d_get_center(&self) -> Vec<f64> {
        ffi::euler2d_get_center(&self.inner).into_iter().collect()
    }
    pub fn euler2d_get_angle(&self) -> f64 {
        ffi::euler2d_get_angle(&self.inner)
    }
    pub fn euler2d_get_translation(&self) -> Vec<f64> {
        ffi::euler2d_get_translation(&self.inner).into_iter().collect()
    }
    pub fn euler2d_get_matrix(&self) -> Vec<f64> {
        ffi::euler2d_get_matrix(&self.inner).into_iter().collect()
    }
    pub fn euler2d_set_center(&mut self, center: &[f64]) {
        ffi::euler2d_set_center(self.inner.pin_mut(), center);
    }
    pub fn euler2d_set_angle(&mut self, angle: f64) {
        ffi::euler2d_set_angle(self.inner.pin_mut(), angle);
    }
    pub fn euler2d_set_translation(&mut self, translation: &[f64]) {
        ffi::euler2d_set_translation(self.inner.pin_mut(), translation);
    }
    pub fn euler2d_set_matrix(&mut self, matrix: &[f64]) {
        ffi::euler2d_set_matrix(self.inner.pin_mut(), matrix);
    }

    // ── Euler3DTransform ───────────────────────────────────────────────────

    pub fn euler3d_get_center(&self) -> Vec<f64> {
        ffi::euler3d_get_center(&self.inner).into_iter().collect()
    }
    pub fn euler3d_get_angle_x(&self) -> f64 { ffi::euler3d_get_angle_x(&self.inner) }
    pub fn euler3d_get_angle_y(&self) -> f64 { ffi::euler3d_get_angle_y(&self.inner) }
    pub fn euler3d_get_angle_z(&self) -> f64 { ffi::euler3d_get_angle_z(&self.inner) }
    pub fn euler3d_get_translation(&self) -> Vec<f64> {
        ffi::euler3d_get_translation(&self.inner).into_iter().collect()
    }
    pub fn euler3d_get_matrix(&self) -> Vec<f64> {
        ffi::euler3d_get_matrix(&self.inner).into_iter().collect()
    }
    pub fn euler3d_set_center(&mut self, center: &[f64]) {
        ffi::euler3d_set_center(self.inner.pin_mut(), center);
    }
    pub fn euler3d_set_rotation(&mut self, angle_x: f64, angle_y: f64, angle_z: f64) {
        ffi::euler3d_set_rotation(self.inner.pin_mut(), angle_x, angle_y, angle_z);
    }
    pub fn euler3d_set_translation(&mut self, translation: &[f64]) {
        ffi::euler3d_set_translation(self.inner.pin_mut(), translation);
    }
    pub fn euler3d_set_matrix(&mut self, matrix: &[f64]) {
        ffi::euler3d_set_matrix(self.inner.pin_mut(), matrix);
    }
    pub fn euler3d_set_compute_zyx(&mut self, compute_zyx: bool) {
        ffi::euler3d_set_compute_zyx(self.inner.pin_mut(), compute_zyx);
    }

    // ── Similarity2DTransform ──────────────────────────────────────────────

    pub fn similarity2d_get_center(&self) -> Vec<f64> {
        ffi::similarity2d_get_center(&self.inner).into_iter().collect()
    }
    pub fn similarity2d_get_angle(&self) -> f64 { ffi::similarity2d_get_angle(&self.inner) }
    pub fn similarity2d_get_scale(&self) -> f64 { ffi::similarity2d_get_scale(&self.inner) }
    pub fn similarity2d_get_translation(&self) -> Vec<f64> {
        ffi::similarity2d_get_translation(&self.inner).into_iter().collect()
    }
    pub fn similarity2d_get_matrix(&self) -> Vec<f64> {
        ffi::similarity2d_get_matrix(&self.inner).into_iter().collect()
    }
    pub fn similarity2d_set_center(&mut self, center: &[f64]) {
        ffi::similarity2d_set_center(self.inner.pin_mut(), center);
    }
    pub fn similarity2d_set_angle(&mut self, angle: f64) {
        ffi::similarity2d_set_angle(self.inner.pin_mut(), angle);
    }
    pub fn similarity2d_set_scale(&mut self, scale: f64) {
        ffi::similarity2d_set_scale(self.inner.pin_mut(), scale);
    }
    pub fn similarity2d_set_translation(&mut self, translation: &[f64]) {
        ffi::similarity2d_set_translation(self.inner.pin_mut(), translation);
    }
    pub fn similarity2d_set_matrix(&mut self, matrix: &[f64]) {
        ffi::similarity2d_set_matrix(self.inner.pin_mut(), matrix);
    }

    // ── Similarity3DTransform ──────────────────────────────────────────────

    pub fn similarity3d_get_center(&self) -> Vec<f64> {
        ffi::similarity3d_get_center(&self.inner).into_iter().collect()
    }
    pub fn similarity3d_get_versor(&self) -> Vec<f64> {
        ffi::similarity3d_get_versor(&self.inner).into_iter().collect()
    }
    pub fn similarity3d_get_scale(&self) -> f64 { ffi::similarity3d_get_scale(&self.inner) }
    pub fn similarity3d_get_translation(&self) -> Vec<f64> {
        ffi::similarity3d_get_translation(&self.inner).into_iter().collect()
    }
    pub fn similarity3d_get_matrix(&self) -> Vec<f64> {
        ffi::similarity3d_get_matrix(&self.inner).into_iter().collect()
    }
    pub fn similarity3d_set_center(&mut self, center: &[f64]) {
        ffi::similarity3d_set_center(self.inner.pin_mut(), center);
    }
    pub fn similarity3d_set_rotation_versor(&mut self, versor: &[f64]) {
        ffi::similarity3d_set_rotation_versor(self.inner.pin_mut(), versor);
    }
    pub fn similarity3d_set_rotation_axis_angle(&mut self, axis: &[f64], angle: f64) {
        ffi::similarity3d_set_rotation_axis_angle(self.inner.pin_mut(), axis, angle);
    }
    pub fn similarity3d_set_scale(&mut self, scale: f64) {
        ffi::similarity3d_set_scale(self.inner.pin_mut(), scale);
    }
    pub fn similarity3d_set_translation(&mut self, translation: &[f64]) {
        ffi::similarity3d_set_translation(self.inner.pin_mut(), translation);
    }
    pub fn similarity3d_set_matrix(&mut self, matrix: &[f64]) {
        ffi::similarity3d_set_matrix(self.inner.pin_mut(), matrix);
    }

    // ── VersorTransform ────────────────────────────────────────────────────

    pub fn versor_get_center(&self) -> Vec<f64> {
        ffi::versor_get_center(&self.inner).into_iter().collect()
    }
    pub fn versor_get_versor(&self) -> Vec<f64> {
        ffi::versor_get_versor(&self.inner).into_iter().collect()
    }
    pub fn versor_get_matrix(&self) -> Vec<f64> {
        ffi::versor_get_matrix(&self.inner).into_iter().collect()
    }
    pub fn versor_set_center(&mut self, center: &[f64]) {
        ffi::versor_set_center(self.inner.pin_mut(), center);
    }
    pub fn versor_set_rotation_versor(&mut self, versor: &[f64]) {
        ffi::versor_set_rotation_versor(self.inner.pin_mut(), versor);
    }
    pub fn versor_set_rotation_axis_angle(&mut self, axis: &[f64], angle: f64) {
        ffi::versor_set_rotation_axis_angle(self.inner.pin_mut(), axis, angle);
    }
    pub fn versor_set_matrix(&mut self, matrix: &[f64]) {
        ffi::versor_set_matrix(self.inner.pin_mut(), matrix);
    }

    // ── VersorRigid3DTransform ─────────────────────────────────────────────

    pub fn versor_rigid3d_get_translation(&self) -> Vec<f64> {
        ffi::versor_rigid3d_get_translation(&self.inner).into_iter().collect()
    }
    pub fn versor_rigid3d_set_translation(&mut self, translation: &[f64]) {
        ffi::versor_rigid3d_set_translation(self.inner.pin_mut(), translation);
    }

    // ── ScaleVersor3DTransform ─────────────────────────────────────────────

    pub fn scale_versor3d_get_scale(&self) -> Vec<f64> {
        ffi::scale_versor3d_get_scale(&self.inner).into_iter().collect()
    }
    pub fn scale_versor3d_set_scale(&mut self, scale: &[f64]) {
        ffi::scale_versor3d_set_scale(self.inner.pin_mut(), scale);
    }

    // ── ScaleSkewVersor3DTransform ─────────────────────────────────────────

    pub fn scale_skew_versor3d_get_skew(&self) -> Vec<f64> {
        ffi::scale_skew_versor3d_get_skew(&self.inner).into_iter().collect()
    }
    pub fn scale_skew_versor3d_set_skew(&mut self, skew: &[f64]) {
        ffi::scale_skew_versor3d_set_skew(self.inner.pin_mut(), skew);
    }

    // ── CompositeTransform ─────────────────────────────────────────────────

    pub fn composite_add(&mut self, t: &Transform) -> Result<(), String> {
        ffi::composite_add_transform(self.inner.pin_mut(), &t.inner).map_err(|e| e.to_string())
    }
    pub fn composite_count(&self) -> Result<u32, String> {
        ffi::composite_get_number_of_transforms(&self.inner).map_err(|e| e.to_string())
    }
    pub fn composite_clear(&mut self) -> Result<(), String> {
        ffi::composite_clear_transforms(self.inner.pin_mut()).map_err(|e| e.to_string())
    }
    pub fn composite_flatten(&mut self) -> Result<(), String> {
        ffi::composite_flatten_transform(self.inner.pin_mut()).map_err(|e| e.to_string())
    }

    // ── DisplacementFieldTransform ─────────────────────────────────────────

    pub fn displacement_field_get(&self) -> Result<Image, String> {
        ffi::displacement_field_get_displacement_field(&self.inner)
            .map(Image::from_unique_ptr)
            .map_err(|e| e.to_string())
    }
    pub fn displacement_field_set(&mut self, field: &Image) -> Result<(), String> {
        ffi::displacement_field_set_displacement_field(self.inner.pin_mut(), &field.inner)
            .map_err(|e| e.to_string())
    }
    pub fn displacement_field_set_smoothing_off(&mut self) -> Result<(), String> {
        ffi::displacement_field_set_smoothing_off(self.inner.pin_mut()).map_err(|e| e.to_string())
    }
    pub fn displacement_field_set_interpolator(&mut self, interpolator: i32) -> Result<(), String> {
        ffi::displacement_field_set_interpolator(self.inner.pin_mut(), interpolator)
            .map_err(|e| e.to_string())
    }
}

impl std::fmt::Debug for Transform {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Transform")
            .field("name", &self.name())
            .field("dimension", &self.dimension())
            .field("n_params", &self.number_of_parameters())
            .finish()
    }
}

// ── Resample filters ───────────────────────────────────────────────────────

pub fn resample(
    image: &Image,
    transform: &Transform,
    interpolator: i32,
    default_pixel_value: f64,
    output_pixel_type: i32,
) -> Result<Image, String> {
    ffi::filter_resample(&image.inner, &transform.inner, interpolator, default_pixel_value, output_pixel_type)
        .map(Image::from_unique_ptr)
        .map_err(|e| e.to_string())
}

pub fn resample_to_ref(
    image: &Image,
    ref_image: &Image,
    transform: &Transform,
    interpolator: i32,
    default_pixel_value: f64,
    output_pixel_type: i32,
) -> Result<Image, String> {
    ffi::filter_resample_to_ref(&image.inner, &ref_image.inner, &transform.inner, interpolator, default_pixel_value, output_pixel_type)
        .map(Image::from_unique_ptr)
        .map_err(|e| e.to_string())
}

pub fn transform_to_displacement_field(
    transform: &Transform,
    output_pixel_type: i32,
    size: &[u32],
    origin: &[f64],
    spacing: &[f64],
) -> Result<Image, String> {
    ffi::filter_transform_to_displacement_field(&transform.inner, output_pixel_type, size, origin, spacing)
        .map(Image::from_unique_ptr)
        .map_err(|e| e.to_string())
}

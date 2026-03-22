use simpleitk_sys::ffi;
use crate::image::Image;

/// Hash function values for `hash()`
pub mod hash_function {
    pub const SHA1: i32 = 0;
    pub const MD5: i32 = 1;
}

/// Compute a hash string of the image.
/// `hash_function`: `hash_function::SHA1` or `hash_function::MD5`
pub fn hash(image: &Image, hash_function: i32) -> Result<String, String> {
    ffi::measure_hash(&image.inner, hash_function).map_err(|e| e.to_string())
}

/// Returns `[min, max]`.
pub fn min_max(image: &Image) -> (f64, f64) {
    let v: Vec<f64> = ffi::measure_min_max(&image.inner).into_iter().collect();
    (v[0], v[1])
}

/// Returns `(min, max, mean, sigma, variance, sum)`.
pub fn statistics(image: &Image) -> (f64, f64, f64, f64, f64, f64) {
    let v: Vec<f64> = ffi::measure_statistics(&image.inner).into_iter().collect();
    (v[0], v[1], v[2], v[3], v[4], v[5])
}

/// Dice similarity coefficient between two binary images (values in [0, 1]).
pub fn similarity_index(img1: &Image, img2: &Image) -> f64 {
    ffi::measure_similarity_index(&img1.inner, &img2.inner)
}

/// Returns `(hausdorff_distance, average_hausdorff_distance)`.
pub fn hausdorff_distance(img1: &Image, img2: &Image) -> (f64, f64) {
    let v: Vec<f64> = ffi::measure_hausdorff_distance(&img1.inner, &img2.inner).into_iter().collect();
    (v[0], v[1])
}

/// Global label overlap measures.
/// Returns `(dice, jaccard, volume_similarity, union_overlap, mean_overlap, false_positive, false_negative, false_discovery)`.
pub fn label_overlap(source: &Image, target: &Image) -> (f64, f64, f64, f64, f64, f64, f64, f64) {
    let v: Vec<f64> = ffi::measure_label_overlap(&source.inner, &target.inner).into_iter().collect();
    (v[0], v[1], v[2], v[3], v[4], v[5], v[6], v[7])
}

/// Returns all label values present in the label image (for `label_stats_for_label`).
pub fn label_stats_labels(image: &Image, label_image: &Image) -> Vec<i64> {
    ffi::measure_label_stats_labels(&image.inner, &label_image.inner).into_iter().collect()
}

/// Per-label statistics for a single label.
/// Returns `(min, max, mean, sigma, variance, sum, count)`.
pub fn label_stats_for_label(image: &Image, label_image: &Image, label: i64) -> (f64, f64, f64, f64, f64, f64, f64) {
    let v: Vec<f64> = ffi::measure_label_stats_for_label(&image.inner, &label_image.inner, label).into_iter().collect();
    (v[0], v[1], v[2], v[3], v[4], v[5], v[6])
}

/// Returns all label values present in the label image (for `label_shape_for_label`).
pub fn label_shape_labels(image: &Image, background_value: f64) -> Vec<i64> {
    ffi::measure_label_shape_labels(&image.inner, background_value).into_iter().collect()
}

/// Per-label shape statistics for a single label.
/// Returns `(number_of_pixels, physical_size, elongation, roundness, flatness, equivalent_spherical_radius, centroid...)`.
pub fn label_shape_for_label(image: &Image, label: i64, background_value: f64) -> Vec<f64> {
    ffi::measure_label_shape_for_label(&image.inner, label, background_value).into_iter().collect()
}

/// Returns all label values present after executing LabelIntensityStatisticsImageFilter.
pub fn label_intensity_labels(image: &Image, feature_image: &Image, background_value: f64) -> Vec<i64> {
    ffi::measure_label_intensity_labels(&image.inner, &feature_image.inner, background_value).into_iter().collect()
}

/// Per-label intensity statistics for a single label.
/// Returns `(min, max, mean, std_dev, variance, sum, n_pixels, elongation, roundness)`.
pub fn label_intensity_for_label(image: &Image, feature_image: &Image, label: i64, background_value: f64) -> Vec<f64> {
    ffi::measure_label_intensity_for_label(&image.inner, &feature_image.inner, label, background_value).into_iter().collect()
}

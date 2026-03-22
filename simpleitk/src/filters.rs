//! Safe wrappers around all SimpleITK filter functions.
//!
//! Kernel type constants (for morphological filters):
//! - `KERNEL_ANNULUS` = 0
//! - `KERNEL_BALL`    = 1
//! - `KERNEL_BOX`     = 2
//! - `KERNEL_CROSS`   = 3

use simpleitk_sys::ffi;
use crate::image::Image;

/// KernelEnum values for morphological filters.
pub mod kernel {
    pub const ANNULUS: i32 = 0;
    pub const BALL: i32 = 1;
    pub const BOX: i32 = 2;
    pub const CROSS: i32 = 3;
}

/// RecursiveGaussian order types.
pub mod recursive_gaussian_order {
    pub const ZERO_ORDER: u32 = 0;
    pub const FIRST_ORDER: u32 = 1;
    pub const SECOND_ORDER: u32 = 2;
}

#[inline]
fn wrap(result: Result<cxx::UniquePtr<ffi::Image>, cxx::Exception>) -> Result<Image, String> {
    result.map(Image::from_unique_ptr).map_err(|e| e.to_string())
}

// ── Unary filters ──────────────────────────────────────────────────────────

pub fn abs(image: &Image) -> Result<Image, String> {
    wrap(ffi::filter_abs(&image.inner))
}

pub fn acos(image: &Image) -> Result<Image, String> {
    wrap(ffi::filter_acos(&image.inner))
}

pub fn adaptive_histogram_equalization(image: &Image, radius: &[u32], alpha: f32, beta: f32) -> Result<Image, String> {
    wrap(ffi::filter_adaptive_histogram_equalization(&image.inner, radius, alpha, beta))
}

pub fn additive_gaussian_noise(image: &Image, standard_deviation: f64, mean: f64, seed: u32) -> Result<Image, String> {
    wrap(ffi::filter_additive_gaussian_noise(&image.inner, standard_deviation, mean, seed))
}

pub fn anti_alias_binary(image: &Image, maximum_rms_error: f64, number_of_iterations: u32) -> Result<Image, String> {
    wrap(ffi::filter_anti_alias_binary(&image.inner, maximum_rms_error, number_of_iterations))
}

pub fn approximate_signed_distance_map(image: &Image, inside_value: f64, outside_value: f64) -> Result<Image, String> {
    wrap(ffi::filter_approximate_signed_distance_map(&image.inner, inside_value, outside_value))
}

pub fn area_closing(image: &Image, lambda: f64, use_image_spacing: bool, fully_connected: bool) -> Result<Image, String> {
    wrap(ffi::filter_area_closing(&image.inner, lambda, use_image_spacing, fully_connected))
}

pub fn area_opening(image: &Image, lambda: f64, use_image_spacing: bool, fully_connected: bool) -> Result<Image, String> {
    wrap(ffi::filter_area_opening(&image.inner, lambda, use_image_spacing, fully_connected))
}

pub fn asin(image: &Image) -> Result<Image, String> {
    wrap(ffi::filter_asin(&image.inner))
}

pub fn atan(image: &Image) -> Result<Image, String> {
    wrap(ffi::filter_atan(&image.inner))
}

pub fn bilateral(image: &Image, domain_sigma: f64, range_sigma: f64, number_of_range_gaussian_samples: u32) -> Result<Image, String> {
    wrap(ffi::filter_bilateral(&image.inner, domain_sigma, range_sigma, number_of_range_gaussian_samples))
}

pub fn bin_shrink(image: &Image, shrink_factors: &[u32]) -> Result<Image, String> {
    wrap(ffi::filter_bin_shrink(&image.inner, shrink_factors))
}

pub fn binary_closing_by_reconstruction(image: &Image, kernel_radius: &[u32], kernel_type: i32, foreground_value: f64, fully_connected: bool) -> Result<Image, String> {
    wrap(ffi::filter_binary_closing_by_reconstruction(&image.inner, kernel_radius, kernel_type, foreground_value, fully_connected))
}

pub fn binary_contour(image: &Image, fully_connected: bool, background_value: f64, foreground_value: f64) -> Result<Image, String> {
    wrap(ffi::filter_binary_contour(&image.inner, fully_connected, background_value, foreground_value))
}

pub fn binary_dilate(image: &Image, kernel_radius: &[u32], kernel_type: i32, background_value: f64, foreground_value: f64, boundary_to_foreground: bool) -> Result<Image, String> {
    wrap(ffi::filter_binary_dilate(&image.inner, kernel_radius, kernel_type, background_value, foreground_value, boundary_to_foreground))
}

pub fn binary_erode(image: &Image, kernel_radius: &[u32], kernel_type: i32, background_value: f64, foreground_value: f64, boundary_to_foreground: bool) -> Result<Image, String> {
    wrap(ffi::filter_binary_erode(&image.inner, kernel_radius, kernel_type, background_value, foreground_value, boundary_to_foreground))
}

pub fn binary_fillhole(image: &Image, fully_connected: bool, foreground_value: f64) -> Result<Image, String> {
    wrap(ffi::filter_binary_fillhole(&image.inner, fully_connected, foreground_value))
}

pub fn binary_grind_peak(image: &Image, fully_connected: bool, foreground_value: f64, background_value: f64) -> Result<Image, String> {
    wrap(ffi::filter_binary_grind_peak(&image.inner, fully_connected, foreground_value, background_value))
}

pub fn binary_median(image: &Image, radius: &[u32], foreground_value: f64, background_value: f64) -> Result<Image, String> {
    wrap(ffi::filter_binary_median(&image.inner, radius, foreground_value, background_value))
}

pub fn binary_min_max_curvature_flow(image: &Image, time_step: f64, number_of_iterations: u32, stencil_radius: i32, threshold: f64) -> Result<Image, String> {
    wrap(ffi::filter_binary_min_max_curvature_flow(&image.inner, time_step, number_of_iterations, stencil_radius, threshold))
}

pub fn binary_morphological_closing(image: &Image, kernel_radius: &[u32], kernel_type: i32, foreground_value: f64, safe_edge_handling: bool) -> Result<Image, String> {
    wrap(ffi::filter_binary_morphological_closing(&image.inner, kernel_radius, kernel_type, foreground_value, safe_edge_handling))
}

pub fn binary_morphological_opening(image: &Image, kernel_radius: &[u32], kernel_type: i32, background_value: f64, foreground_value: f64) -> Result<Image, String> {
    wrap(ffi::filter_binary_morphological_opening(&image.inner, kernel_radius, kernel_type, background_value, foreground_value))
}

pub fn binary_not(image: &Image, foreground_value: f64, background_value: f64) -> Result<Image, String> {
    wrap(ffi::filter_binary_not(&image.inner, foreground_value, background_value))
}

pub fn binary_opening_by_reconstruction(image: &Image, kernel_radius: &[u32], kernel_type: i32, foreground_value: f64, fully_connected: bool) -> Result<Image, String> {
    wrap(ffi::filter_binary_opening_by_reconstruction(&image.inner, kernel_radius, kernel_type, foreground_value, fully_connected))
}

pub fn binary_projection(image: &Image, projection_dimension: u32, foreground_value: f64, background_value: f64) -> Result<Image, String> {
    wrap(ffi::filter_binary_projection(&image.inner, projection_dimension, foreground_value, background_value))
}

pub fn binary_pruning(image: &Image, iteration: u32) -> Result<Image, String> {
    wrap(ffi::filter_binary_pruning(&image.inner, iteration))
}

pub fn binary_thinning(image: &Image) -> Result<Image, String> {
    wrap(ffi::filter_binary_thinning(&image.inner))
}

pub fn binary_threshold(image: &Image, lower_threshold: f64, upper_threshold: f64, inside_value: u8, outside_value: u8) -> Result<Image, String> {
    wrap(ffi::filter_binary_threshold(&image.inner, lower_threshold, upper_threshold, inside_value, outside_value))
}

pub fn binary_threshold_projection(image: &Image, projection_dimension: u32, threshold_value: f64, foreground_value: u8, background_value: u8) -> Result<Image, String> {
    wrap(ffi::filter_binary_threshold_projection(&image.inner, projection_dimension, threshold_value, foreground_value, background_value))
}

pub fn binomial_blur(image: &Image, repetitions: u32) -> Result<Image, String> {
    wrap(ffi::filter_binomial_blur(&image.inner, repetitions))
}

pub fn bitwise_not(image: &Image) -> Result<Image, String> {
    wrap(ffi::filter_bitwise_not(&image.inner))
}

pub fn black_top_hat(image: &Image, kernel_radius: &[u32], kernel_type: i32, safe_border: bool) -> Result<Image, String> {
    wrap(ffi::filter_black_top_hat(&image.inner, kernel_radius, kernel_type, safe_border))
}

pub fn bounded_reciprocal(image: &Image) -> Result<Image, String> {
    wrap(ffi::filter_bounded_reciprocal(&image.inner))
}

pub fn box_mean(image: &Image, radius: &[u32]) -> Result<Image, String> {
    wrap(ffi::filter_box_mean(&image.inner, radius))
}

pub fn box_sigma(image: &Image, radius: &[u32]) -> Result<Image, String> {
    wrap(ffi::filter_box_sigma(&image.inner, radius))
}

pub fn canny_edge_detection(image: &Image, lower_threshold: f64, upper_threshold: f64, variance: f64, maximum_error: f64) -> Result<Image, String> {
    wrap(ffi::filter_canny_edge_detection(&image.inner, lower_threshold, upper_threshold, variance, maximum_error))
}

/// Cast image to a different pixel type. Use `PixelId` constants or the raw i32 value.
pub fn cast(image: &Image, pixel_id: i32) -> Result<Image, String> {
    wrap(ffi::filter_cast(&image.inner, pixel_id))
}

pub fn clamp(image: &Image, lower_bound: f64, upper_bound: f64) -> Result<Image, String> {
    wrap(ffi::filter_clamp(&image.inner, lower_bound, upper_bound))
}

pub fn closing_by_reconstruction(image: &Image, kernel_radius: &[u32], kernel_type: i32, fully_connected: bool, preserve_intensities: bool) -> Result<Image, String> {
    wrap(ffi::filter_closing_by_reconstruction(&image.inner, kernel_radius, kernel_type, fully_connected, preserve_intensities))
}

pub fn complex_to_imaginary(image: &Image) -> Result<Image, String> {
    wrap(ffi::filter_complex_to_imaginary(&image.inner))
}

pub fn complex_to_modulus(image: &Image) -> Result<Image, String> {
    wrap(ffi::filter_complex_to_modulus(&image.inner))
}

pub fn complex_to_phase(image: &Image) -> Result<Image, String> {
    wrap(ffi::filter_complex_to_phase(&image.inner))
}

pub fn complex_to_real(image: &Image) -> Result<Image, String> {
    wrap(ffi::filter_complex_to_real(&image.inner))
}

pub fn connected_component(image: &Image, fully_connected: bool) -> Result<Image, String> {
    wrap(ffi::filter_connected_component(&image.inner, fully_connected))
}

pub fn cos(image: &Image) -> Result<Image, String> {
    wrap(ffi::filter_cos(&image.inner))
}

pub fn curvature_anisotropic_diffusion(image: &Image, time_step: f64, conductance_parameter: f64, conductance_scaling_update_interval: u32, number_of_iterations: u32) -> Result<Image, String> {
    wrap(ffi::filter_curvature_anisotropic_diffusion(&image.inner, time_step, conductance_parameter, conductance_scaling_update_interval, number_of_iterations))
}

pub fn curvature_flow(image: &Image, time_step: f64, number_of_iterations: u32) -> Result<Image, String> {
    wrap(ffi::filter_curvature_flow(&image.inner, time_step, number_of_iterations))
}

pub fn cyclic_shift(image: &Image, shift: &[i32]) -> Result<Image, String> {
    wrap(ffi::filter_cyclic_shift(&image.inner, shift))
}

pub fn danielsson_distance_map(image: &Image, input_is_binary: bool, squared_distance: bool, use_image_spacing: bool) -> Result<Image, String> {
    wrap(ffi::filter_danielsson_distance_map(&image.inner, input_is_binary, squared_distance, use_image_spacing))
}

pub fn dilate_object_morphology(image: &Image, kernel_radius: &[u32], kernel_type: i32, object_value: f64, background_value: f64) -> Result<Image, String> {
    wrap(ffi::filter_dilate_object_morphology(&image.inner, kernel_radius, kernel_type, object_value, background_value))
}

pub fn discrete_gaussian(image: &Image, variance: f64, maximum_kernel_width: u32, maximum_error: f64, use_image_spacing: bool) -> Result<Image, String> {
    wrap(ffi::filter_discrete_gaussian(&image.inner, variance, maximum_kernel_width, maximum_error, use_image_spacing))
}

pub fn discrete_gaussian_derivative(image: &Image, variance: f64, order: u32, maximum_kernel_width: u32, maximum_error: f64, normalize_across_scale: bool) -> Result<Image, String> {
    wrap(ffi::filter_discrete_gaussian_derivative(&image.inner, variance, order, maximum_kernel_width, maximum_error, normalize_across_scale))
}

pub fn displacement_field_jacobian_determinant(image: &Image, use_image_spacing: bool) -> Result<Image, String> {
    wrap(ffi::filter_displacement_field_jacobian_determinant(&image.inner, use_image_spacing))
}

pub fn double_threshold(image: &Image, threshold1: f64, threshold2: f64, threshold3: f64, threshold4: f64, inside_value: u8, outside_value: u8, fully_connected: bool) -> Result<Image, String> {
    wrap(ffi::filter_double_threshold(&image.inner, threshold1, threshold2, threshold3, threshold4, inside_value, outside_value, fully_connected))
}

pub fn edge_potential(image: &Image) -> Result<Image, String> {
    wrap(ffi::filter_edge_potential(&image.inner))
}

pub fn erode_object_morphology(image: &Image, kernel_radius: &[u32], kernel_type: i32, object_value: f64, background_value: f64) -> Result<Image, String> {
    wrap(ffi::filter_erode_object_morphology(&image.inner, kernel_radius, kernel_type, object_value, background_value))
}

pub fn exp(image: &Image) -> Result<Image, String> {
    wrap(ffi::filter_exp(&image.inner))
}

pub fn exp_negative(image: &Image) -> Result<Image, String> {
    wrap(ffi::filter_exp_negative(&image.inner))
}

pub fn fast_approximate_rank(image: &Image, radius: &[u32], rank: f64) -> Result<Image, String> {
    wrap(ffi::filter_fast_approximate_rank(&image.inner, radius, rank))
}

pub fn fft_shift(image: &Image, inverse: bool) -> Result<Image, String> {
    wrap(ffi::filter_fft_shift(&image.inner, inverse))
}

pub fn flip_image(image: &Image, flip_axes: &[bool], flip_about_origin: bool) -> Result<Image, String> {
    wrap(ffi::filter_flip_image(&image.inner, flip_axes, flip_about_origin))
}

pub fn gradient_anisotropic_diffusion(image: &Image, time_step: f64, conductance_parameter: f64, conductance_scaling_update_interval: u32, number_of_iterations: u32) -> Result<Image, String> {
    wrap(ffi::filter_gradient_anisotropic_diffusion(&image.inner, time_step, conductance_parameter, conductance_scaling_update_interval, number_of_iterations))
}

pub fn gradient_magnitude(image: &Image) -> Result<Image, String> {
    wrap(ffi::filter_gradient_magnitude(&image.inner))
}

pub fn gradient_magnitude_recursive_gaussian(image: &Image, sigma: f64, normalize_across_scale: bool) -> Result<Image, String> {
    wrap(ffi::filter_gradient_magnitude_recursive_gaussian(&image.inner, sigma, normalize_across_scale))
}

pub fn grayscale_dilate(image: &Image, kernel_radius: &[u32], kernel_type: i32) -> Result<Image, String> {
    wrap(ffi::filter_grayscale_dilate(&image.inner, kernel_radius, kernel_type))
}

pub fn grayscale_erode(image: &Image, kernel_radius: &[u32], kernel_type: i32) -> Result<Image, String> {
    wrap(ffi::filter_grayscale_erode(&image.inner, kernel_radius, kernel_type))
}

pub fn grayscale_fillhole(image: &Image, fully_connected: bool) -> Result<Image, String> {
    wrap(ffi::filter_grayscale_fillhole(&image.inner, fully_connected))
}

pub fn grayscale_grind_peak(image: &Image, fully_connected: bool) -> Result<Image, String> {
    wrap(ffi::filter_grayscale_grind_peak(&image.inner, fully_connected))
}

pub fn grayscale_morphological_closing(image: &Image, kernel_radius: &[u32], kernel_type: i32, safe_border: bool) -> Result<Image, String> {
    wrap(ffi::filter_grayscale_morphological_closing(&image.inner, kernel_radius, kernel_type, safe_border))
}

pub fn grayscale_morphological_opening(image: &Image, kernel_radius: &[u32], kernel_type: i32, safe_border: bool) -> Result<Image, String> {
    wrap(ffi::filter_grayscale_morphological_opening(&image.inner, kernel_radius, kernel_type, safe_border))
}

pub fn h_concave(image: &Image, height: f64, fully_connected: bool) -> Result<Image, String> {
    wrap(ffi::filter_h_concave(&image.inner, height, fully_connected))
}

pub fn h_convex(image: &Image, height: f64, fully_connected: bool) -> Result<Image, String> {
    wrap(ffi::filter_h_convex(&image.inner, height, fully_connected))
}

pub fn h_maxima(image: &Image, height: f64, fully_connected: bool) -> Result<Image, String> {
    wrap(ffi::filter_h_maxima(&image.inner, height, fully_connected))
}

pub fn h_minima(image: &Image, height: f64, fully_connected: bool) -> Result<Image, String> {
    wrap(ffi::filter_h_minima(&image.inner, height, fully_connected))
}

pub fn huang_threshold(image: &Image, inside_value: u8, outside_value: u8, number_of_histogram_bins: u32) -> Result<Image, String> {
    wrap(ffi::filter_huang_threshold(&image.inner, inside_value, outside_value, number_of_histogram_bins))
}

pub fn intensity_windowing(image: &Image, window_minimum: f64, window_maximum: f64, output_minimum: f64, output_maximum: f64) -> Result<Image, String> {
    wrap(ffi::filter_intensity_windowing(&image.inner, window_minimum, window_maximum, output_minimum, output_maximum))
}

pub fn intermodes_threshold(image: &Image, inside_value: u8, outside_value: u8, number_of_histogram_bins: u32) -> Result<Image, String> {
    wrap(ffi::filter_intermodes_threshold(&image.inner, inside_value, outside_value, number_of_histogram_bins))
}

pub fn invert_intensity(image: &Image, maximum: f64) -> Result<Image, String> {
    wrap(ffi::filter_invert_intensity(&image.inner, maximum))
}

pub fn iso_contour_distance(image: &Image, level_set_value: f64, far_value: f64) -> Result<Image, String> {
    wrap(ffi::filter_iso_contour_distance(&image.inner, level_set_value, far_value))
}

pub fn iso_data_threshold(image: &Image, inside_value: u8, outside_value: u8, number_of_histogram_bins: u32) -> Result<Image, String> {
    wrap(ffi::filter_iso_data_threshold(&image.inner, inside_value, outside_value, number_of_histogram_bins))
}

pub fn kittler_illingworth_threshold(image: &Image, inside_value: u8, outside_value: u8, number_of_histogram_bins: u32) -> Result<Image, String> {
    wrap(ffi::filter_kittler_illingworth_threshold(&image.inner, inside_value, outside_value, number_of_histogram_bins))
}

pub fn label_contour(image: &Image, fully_connected: bool, background_value: f64) -> Result<Image, String> {
    wrap(ffi::filter_label_contour(&image.inner, fully_connected, background_value))
}

pub fn laplacian_recursive_gaussian(image: &Image, sigma: f64, normalize_across_scale: bool) -> Result<Image, String> {
    wrap(ffi::filter_laplacian_recursive_gaussian(&image.inner, sigma, normalize_across_scale))
}

pub fn laplacian_sharpening(image: &Image, use_image_spacing: bool) -> Result<Image, String> {
    wrap(ffi::filter_laplacian_sharpening(&image.inner, use_image_spacing))
}

pub fn li_threshold(image: &Image, inside_value: u8, outside_value: u8, number_of_histogram_bins: u32) -> Result<Image, String> {
    wrap(ffi::filter_li_threshold(&image.inner, inside_value, outside_value, number_of_histogram_bins))
}

pub fn log(image: &Image) -> Result<Image, String> {
    wrap(ffi::filter_log(&image.inner))
}

pub fn log10(image: &Image) -> Result<Image, String> {
    wrap(ffi::filter_log10(&image.inner))
}

pub fn maximum_entropy_threshold(image: &Image, inside_value: u8, outside_value: u8, number_of_histogram_bins: u32) -> Result<Image, String> {
    wrap(ffi::filter_maximum_entropy_threshold(&image.inner, inside_value, outside_value, number_of_histogram_bins))
}

pub fn maximum_projection(image: &Image, projection_dimension: u32) -> Result<Image, String> {
    wrap(ffi::filter_maximum_projection(&image.inner, projection_dimension))
}

pub fn mean(image: &Image, radius: &[u32]) -> Result<Image, String> {
    wrap(ffi::filter_mean(&image.inner, radius))
}

pub fn mean_projection(image: &Image, projection_dimension: u32) -> Result<Image, String> {
    wrap(ffi::filter_mean_projection(&image.inner, projection_dimension))
}

pub fn median(image: &Image, radius: &[u32]) -> Result<Image, String> {
    wrap(ffi::filter_median(&image.inner, radius))
}

pub fn median_projection(image: &Image, projection_dimension: u32) -> Result<Image, String> {
    wrap(ffi::filter_median_projection(&image.inner, projection_dimension))
}

pub fn minimum_projection(image: &Image, projection_dimension: u32) -> Result<Image, String> {
    wrap(ffi::filter_minimum_projection(&image.inner, projection_dimension))
}

pub fn mirror_pad(image: &Image, pad_lower_bound: &[u32], pad_upper_bound: &[u32]) -> Result<Image, String> {
    wrap(ffi::filter_mirror_pad(&image.inner, pad_lower_bound, pad_upper_bound))
}

pub fn moments_threshold(image: &Image, inside_value: u8, outside_value: u8, number_of_histogram_bins: u32) -> Result<Image, String> {
    wrap(ffi::filter_moments_threshold(&image.inner, inside_value, outside_value, number_of_histogram_bins))
}

pub fn morphological_gradient(image: &Image, kernel_radius: &[u32], kernel_type: i32) -> Result<Image, String> {
    wrap(ffi::filter_morphological_gradient(&image.inner, kernel_radius, kernel_type))
}

pub fn morphological_watershed(image: &Image, level: f64, mark_watershed_line: bool, fully_connected: bool) -> Result<Image, String> {
    wrap(ffi::filter_morphological_watershed(&image.inner, level, mark_watershed_line, fully_connected))
}

pub fn noise_image_filter(image: &Image) -> Result<Image, String> {
    wrap(ffi::filter_noise_image_filter(&image.inner))
}

pub fn normalize(image: &Image) -> Result<Image, String> {
    wrap(ffi::filter_normalize(&image.inner))
}

pub fn normalize_to_constant(image: &Image, constant: f64) -> Result<Image, String> {
    wrap(ffi::filter_normalize_to_constant(&image.inner, constant))
}

pub fn not(image: &Image) -> Result<Image, String> {
    wrap(ffi::filter_not(&image.inner))
}

pub fn opening_by_reconstruction(image: &Image, kernel_radius: &[u32], kernel_type: i32, fully_connected: bool, preserve_intensities: bool) -> Result<Image, String> {
    wrap(ffi::filter_opening_by_reconstruction(&image.inner, kernel_radius, kernel_type, fully_connected, preserve_intensities))
}

pub fn otsu_multiple_thresholds(image: &Image, number_of_thresholds: u32, label_offset: u8, number_of_histogram_bins: u32, valley_emphasis: bool) -> Result<Image, String> {
    wrap(ffi::filter_otsu_multiple_thresholds(&image.inner, number_of_thresholds, label_offset, number_of_histogram_bins, valley_emphasis))
}

pub fn otsu_threshold(image: &Image, inside_value: u8, outside_value: u8, number_of_histogram_bins: u32) -> Result<Image, String> {
    wrap(ffi::filter_otsu_threshold(&image.inner, inside_value, outside_value, number_of_histogram_bins))
}

pub fn rank(image: &Image, radius: &[u32], rank_val: f64) -> Result<Image, String> {
    wrap(ffi::filter_rank(&image.inner, radius, rank_val))
}

/// `order`: 0=ZeroOrder, 1=FirstOrder, 2=SecondOrder.
pub fn recursive_gaussian(image: &Image, sigma: f64, normalize_across_scale: bool, order: u32, direction: u32) -> Result<Image, String> {
    wrap(ffi::filter_recursive_gaussian(&image.inner, sigma, normalize_across_scale, order, direction))
}

pub fn regional_maxima(image: &Image, background_value: f64, fully_connected: bool, flat_is_maxima: bool) -> Result<Image, String> {
    wrap(ffi::filter_regional_maxima(&image.inner, background_value, fully_connected, flat_is_maxima))
}

pub fn regional_minima(image: &Image, background_value: f64, fully_connected: bool, flat_is_minima: bool) -> Result<Image, String> {
    wrap(ffi::filter_regional_minima(&image.inner, background_value, fully_connected, flat_is_minima))
}

pub fn relabel_component(image: &Image, minimum_object_size: u64, sort_by_object_size: bool) -> Result<Image, String> {
    wrap(ffi::filter_relabel_component(&image.inner, minimum_object_size, sort_by_object_size))
}

pub fn renyi_entropy_threshold(image: &Image, inside_value: u8, outside_value: u8, number_of_histogram_bins: u32) -> Result<Image, String> {
    wrap(ffi::filter_renyi_entropy_threshold(&image.inner, inside_value, outside_value, number_of_histogram_bins))
}

pub fn rescale_intensity(image: &Image, output_minimum: f64, output_maximum: f64) -> Result<Image, String> {
    wrap(ffi::filter_rescale_intensity(&image.inner, output_minimum, output_maximum))
}

pub fn round(image: &Image) -> Result<Image, String> {
    wrap(ffi::filter_round(&image.inner))
}

pub fn salt_and_pepper_noise(image: &Image, probability: f64, seed: u32) -> Result<Image, String> {
    wrap(ffi::filter_salt_and_pepper_noise(&image.inner, probability, seed))
}

pub fn shanbhag_threshold(image: &Image, inside_value: u8, outside_value: u8, number_of_histogram_bins: u32) -> Result<Image, String> {
    wrap(ffi::filter_shanbhag_threshold(&image.inner, inside_value, outside_value, number_of_histogram_bins))
}

pub fn shift_scale(image: &Image, shift: f64, scale: f64) -> Result<Image, String> {
    wrap(ffi::filter_shift_scale(&image.inner, shift, scale))
}

pub fn shot_noise(image: &Image, scale: f64, seed: u32) -> Result<Image, String> {
    wrap(ffi::filter_shot_noise(&image.inner, scale, seed))
}

pub fn shrink(image: &Image, shrink_factors: &[u32]) -> Result<Image, String> {
    wrap(ffi::filter_shrink(&image.inner, shrink_factors))
}

pub fn sigmoid(image: &Image, alpha: f64, beta: f64, output_maximum: f64, output_minimum: f64) -> Result<Image, String> {
    wrap(ffi::filter_sigmoid(&image.inner, alpha, beta, output_maximum, output_minimum))
}

pub fn signed_danielsson_distance_map(image: &Image, inside_is_positive: bool, squared_distance: bool, use_image_spacing: bool) -> Result<Image, String> {
    wrap(ffi::filter_signed_danielsson_distance_map(&image.inner, inside_is_positive, squared_distance, use_image_spacing))
}

pub fn signed_maurer_distance_map(image: &Image, inside_is_positive: bool, squared_distance: bool, use_image_spacing: bool, background_value: f64) -> Result<Image, String> {
    wrap(ffi::filter_signed_maurer_distance_map(&image.inner, inside_is_positive, squared_distance, use_image_spacing, background_value))
}

pub fn sin(image: &Image) -> Result<Image, String> {
    wrap(ffi::filter_sin(&image.inner))
}

pub fn smoothing_recursive_gaussian(image: &Image, sigma: &[f64], normalize_across_scale: bool) -> Result<Image, String> {
    wrap(ffi::filter_smoothing_recursive_gaussian(&image.inner, sigma, normalize_across_scale))
}

pub fn speckle_noise(image: &Image, standard_deviation: f64, seed: u32) -> Result<Image, String> {
    wrap(ffi::filter_speckle_noise(&image.inner, standard_deviation, seed))
}

pub fn sqrt(image: &Image) -> Result<Image, String> {
    wrap(ffi::filter_sqrt(&image.inner))
}

pub fn square(image: &Image) -> Result<Image, String> {
    wrap(ffi::filter_square(&image.inner))
}

pub fn standard_deviation_projection(image: &Image, projection_dimension: u32) -> Result<Image, String> {
    wrap(ffi::filter_standard_deviation_projection(&image.inner, projection_dimension))
}

pub fn sum_projection(image: &Image, projection_dimension: u32) -> Result<Image, String> {
    wrap(ffi::filter_sum_projection(&image.inner, projection_dimension))
}

pub fn tan(image: &Image) -> Result<Image, String> {
    wrap(ffi::filter_tan(&image.inner))
}

pub fn threshold(image: &Image, lower_threshold: f64, upper_threshold: f64, outside_value: f64) -> Result<Image, String> {
    wrap(ffi::filter_threshold(&image.inner, lower_threshold, upper_threshold, outside_value))
}

pub fn triangle_threshold(image: &Image, inside_value: u8, outside_value: u8, number_of_histogram_bins: u32) -> Result<Image, String> {
    wrap(ffi::filter_triangle_threshold(&image.inner, inside_value, outside_value, number_of_histogram_bins))
}

pub fn unary_minus(image: &Image) -> Result<Image, String> {
    wrap(ffi::filter_unary_minus(&image.inner))
}

pub fn unsharp_mask(image: &Image, sigmas: &[f64], amount: f64, threshold: f64) -> Result<Image, String> {
    wrap(ffi::filter_unsharp_mask(&image.inner, sigmas, amount, threshold))
}

pub fn valued_regional_maxima(image: &Image, fully_connected: bool) -> Result<Image, String> {
    wrap(ffi::filter_valued_regional_maxima(&image.inner, fully_connected))
}

pub fn valued_regional_minima(image: &Image, fully_connected: bool) -> Result<Image, String> {
    wrap(ffi::filter_valued_regional_minima(&image.inner, fully_connected))
}

pub fn vector_index_selection_cast(image: &Image, index: u32, output_pixel_type: i32) -> Result<Image, String> {
    wrap(ffi::filter_vector_index_selection_cast(&image.inner, index, output_pixel_type))
}

pub fn vector_magnitude(image: &Image) -> Result<Image, String> {
    wrap(ffi::filter_vector_magnitude(&image.inner))
}

pub fn voting_binary(image: &Image, radius: &[u32], birth_threshold: u32, survival_threshold: u32, foreground_value: f64, background_value: f64) -> Result<Image, String> {
    wrap(ffi::filter_voting_binary(&image.inner, radius, birth_threshold, survival_threshold, foreground_value, background_value))
}

pub fn voting_binary_hole_filling(image: &Image, radius: &[u32], majority_threshold: u32, foreground_value: f64, background_value: f64, maximum_number_of_iterations: u32) -> Result<Image, String> {
    wrap(ffi::filter_voting_binary_hole_filling(&image.inner, radius, majority_threshold, foreground_value, background_value, maximum_number_of_iterations))
}

pub fn voting_binary_iterative_hole_filling(image: &Image, radius: &[u32], majority_threshold: u32, foreground_value: f64, background_value: f64, maximum_number_of_iterations: u32) -> Result<Image, String> {
    wrap(ffi::filter_voting_binary_iterative_hole_filling(&image.inner, radius, majority_threshold, foreground_value, background_value, maximum_number_of_iterations))
}

pub fn white_top_hat(image: &Image, kernel_radius: &[u32], kernel_type: i32, safe_border: bool) -> Result<Image, String> {
    wrap(ffi::filter_white_top_hat(&image.inner, kernel_radius, kernel_type, safe_border))
}

pub fn yen_threshold(image: &Image, inside_value: u8, outside_value: u8, number_of_histogram_bins: u32) -> Result<Image, String> {
    wrap(ffi::filter_yen_threshold(&image.inner, inside_value, outside_value, number_of_histogram_bins))
}

pub fn zero_crossing(image: &Image, foreground_value: u8, background_value: u8) -> Result<Image, String> {
    wrap(ffi::filter_zero_crossing(&image.inner, foreground_value, background_value))
}

pub fn zero_crossing_based_edge_detection(image: &Image, variance: f64, maximum_error: f64) -> Result<Image, String> {
    wrap(ffi::filter_zero_crossing_based_edge_detection(&image.inner, variance, maximum_error))
}

pub fn zero_flux_neumann_pad(image: &Image, pad_lower_bound: &[u32], pad_upper_bound: &[u32]) -> Result<Image, String> {
    wrap(ffi::filter_zero_flux_neumann_pad(&image.inner, pad_lower_bound, pad_upper_bound))
}

pub fn constant_pad(image: &Image, pad_lower_bound: &[u32], pad_upper_bound: &[u32], constant: f64) -> Result<Image, String> {
    wrap(ffi::filter_constant_pad(&image.inner, pad_lower_bound, pad_upper_bound, constant))
}

pub fn wrap_pad(image: &Image, pad_lower_bound: &[u32], pad_upper_bound: &[u32]) -> Result<Image, String> {
    wrap(ffi::filter_wrap_pad(&image.inner, pad_lower_bound, pad_upper_bound))
}

// ── Binary filters ─────────────────────────────────────────────────────────

pub fn absolute_value_difference(img1: &Image, img2: &Image) -> Result<Image, String> {
    wrap(ffi::filter_absolute_value_difference(&img1.inner, &img2.inner))
}

pub fn add(img1: &Image, img2: &Image) -> Result<Image, String> {
    wrap(ffi::filter_add(&img1.inner, &img2.inner))
}

pub fn and(img1: &Image, img2: &Image) -> Result<Image, String> {
    wrap(ffi::filter_and(&img1.inner, &img2.inner))
}

pub fn atan2(img1: &Image, img2: &Image) -> Result<Image, String> {
    wrap(ffi::filter_atan2(&img1.inner, &img2.inner))
}

pub fn binary_magnitude(img1: &Image, img2: &Image) -> Result<Image, String> {
    wrap(ffi::filter_binary_magnitude(&img1.inner, &img2.inner))
}

pub fn binary_reconstruction_by_dilation(img1: &Image, img2: &Image, background_value: f64, foreground_value: f64, fully_connected: bool) -> Result<Image, String> {
    wrap(ffi::filter_binary_reconstruction_by_dilation(&img1.inner, &img2.inner, background_value, foreground_value, fully_connected))
}

pub fn binary_reconstruction_by_erosion(img1: &Image, img2: &Image, background_value: f64, foreground_value: f64, fully_connected: bool) -> Result<Image, String> {
    wrap(ffi::filter_binary_reconstruction_by_erosion(&img1.inner, &img2.inner, background_value, foreground_value, fully_connected))
}

pub fn checker_board(img1: &Image, img2: &Image, checker_pattern: &[u32]) -> Result<Image, String> {
    wrap(ffi::filter_checker_board(&img1.inner, &img2.inner, checker_pattern))
}

pub fn divide(img1: &Image, img2: &Image) -> Result<Image, String> {
    wrap(ffi::filter_divide(&img1.inner, &img2.inner))
}

pub fn divide_floor(img1: &Image, img2: &Image) -> Result<Image, String> {
    wrap(ffi::filter_divide_floor(&img1.inner, &img2.inner))
}

pub fn divide_real(img1: &Image, img2: &Image) -> Result<Image, String> {
    wrap(ffi::filter_divide_real(&img1.inner, &img2.inner))
}

pub fn equal(img1: &Image, img2: &Image) -> Result<Image, String> {
    wrap(ffi::filter_equal(&img1.inner, &img2.inner))
}

pub fn grayscale_geodesic_dilate(marker: &Image, mask: &Image, run_one_iteration: bool, fully_connected: bool) -> Result<Image, String> {
    wrap(ffi::filter_grayscale_geodesic_dilate(&marker.inner, &mask.inner, run_one_iteration, fully_connected))
}

pub fn grayscale_geodesic_erode(marker: &Image, mask: &Image, run_one_iteration: bool, fully_connected: bool) -> Result<Image, String> {
    wrap(ffi::filter_grayscale_geodesic_erode(&marker.inner, &mask.inner, run_one_iteration, fully_connected))
}

pub fn greater(img1: &Image, img2: &Image) -> Result<Image, String> {
    wrap(ffi::filter_greater(&img1.inner, &img2.inner))
}

pub fn greater_equal(img1: &Image, img2: &Image) -> Result<Image, String> {
    wrap(ffi::filter_greater_equal(&img1.inner, &img2.inner))
}

pub fn histogram_matching(image: &Image, reference: &Image, number_of_histogram_levels: u32, number_of_match_points: u32, threshold_at_mean_intensity: bool) -> Result<Image, String> {
    wrap(ffi::filter_histogram_matching(&image.inner, &reference.inner, number_of_histogram_levels, number_of_match_points, threshold_at_mean_intensity))
}

pub fn less(img1: &Image, img2: &Image) -> Result<Image, String> {
    wrap(ffi::filter_less(&img1.inner, &img2.inner))
}

pub fn less_equal(img1: &Image, img2: &Image) -> Result<Image, String> {
    wrap(ffi::filter_less_equal(&img1.inner, &img2.inner))
}

pub fn magnitude_and_phase_to_complex(img1: &Image, img2: &Image) -> Result<Image, String> {
    wrap(ffi::filter_magnitude_and_phase_to_complex(&img1.inner, &img2.inner))
}

pub fn mask_image(image: &Image, mask: &Image, outside_value: f64, masking_value: f64) -> Result<Image, String> {
    wrap(ffi::filter_mask_image(&image.inner, &mask.inner, outside_value, masking_value))
}

pub fn mask_negated_image(image: &Image, mask: &Image, outside_value: f64, masking_value: f64) -> Result<Image, String> {
    wrap(ffi::filter_mask_negated_image(&image.inner, &mask.inner, outside_value, masking_value))
}

pub fn morphological_watershed_from_markers(image: &Image, marker: &Image, mark_watershed_line: bool, fully_connected: bool) -> Result<Image, String> {
    wrap(ffi::filter_morphological_watershed_from_markers(&image.inner, &marker.inner, mark_watershed_line, fully_connected))
}

pub fn multiply(img1: &Image, img2: &Image) -> Result<Image, String> {
    wrap(ffi::filter_multiply(&img1.inner, &img2.inner))
}

pub fn not_equal(img1: &Image, img2: &Image) -> Result<Image, String> {
    wrap(ffi::filter_not_equal(&img1.inner, &img2.inner))
}

pub fn or(img1: &Image, img2: &Image) -> Result<Image, String> {
    wrap(ffi::filter_or(&img1.inner, &img2.inner))
}

pub fn pow(img1: &Image, img2: &Image) -> Result<Image, String> {
    wrap(ffi::filter_pow(&img1.inner, &img2.inner))
}

pub fn real_and_imaginary_to_complex(img1: &Image, img2: &Image) -> Result<Image, String> {
    wrap(ffi::filter_real_and_imaginary_to_complex(&img1.inner, &img2.inner))
}

pub fn reconstruction_by_dilation(img1: &Image, img2: &Image, fully_connected: bool) -> Result<Image, String> {
    wrap(ffi::filter_reconstruction_by_dilation(&img1.inner, &img2.inner, fully_connected))
}

pub fn reconstruction_by_erosion(img1: &Image, img2: &Image, fully_connected: bool) -> Result<Image, String> {
    wrap(ffi::filter_reconstruction_by_erosion(&img1.inner, &img2.inner, fully_connected))
}

pub fn squared_difference(img1: &Image, img2: &Image) -> Result<Image, String> {
    wrap(ffi::filter_squared_difference(&img1.inner, &img2.inner))
}

pub fn subtract(img1: &Image, img2: &Image) -> Result<Image, String> {
    wrap(ffi::filter_subtract(&img1.inner, &img2.inner))
}

pub fn xor(img1: &Image, img2: &Image) -> Result<Image, String> {
    wrap(ffi::filter_xor(&img1.inner, &img2.inner))
}

// ── New filters ─────────────────────────────────────────────────────────────

// Unary (no params)

pub fn sobel_edge_detection(image: &Image) -> Result<Image, String> {
    wrap(ffi::filter_sobel_edge_detection(&image.inner))
}

pub fn forward_fft(image: &Image) -> Result<Image, String> {
    wrap(ffi::filter_forward_fft(&image.inner))
}

pub fn inverse_fft(image: &Image) -> Result<Image, String> {
    wrap(ffi::filter_inverse_fft(&image.inner))
}

pub fn real_to_half_hermitian_forward_fft(image: &Image) -> Result<Image, String> {
    wrap(ffi::filter_real_to_half_hermitian_forward_fft(&image.inner))
}

pub fn toboggan(image: &Image) -> Result<Image, String> {
    wrap(ffi::filter_toboggan(&image.inner))
}

pub fn label_map_to_label(image: &Image) -> Result<Image, String> {
    wrap(ffi::filter_label_map_to_label(&image.inner))
}

// Unary with simple params

pub fn derivative(image: &Image, direction: u32, order: u32, use_image_spacing: bool) -> Result<Image, String> {
    wrap(ffi::filter_derivative(&image.inner, direction, order, use_image_spacing))
}

pub fn half_hermitian_to_real_inverse_fft(image: &Image, actual_x_dimension_is_odd: bool) -> Result<Image, String> {
    wrap(ffi::filter_half_hermitian_to_real_inverse_fft(&image.inner, actual_x_dimension_is_odd))
}

pub fn fft_pad(image: &Image, boundary_condition: i32, size_greatest_prime_factor: i32) -> Result<Image, String> {
    wrap(ffi::filter_fft_pad(&image.inner, boundary_condition, size_greatest_prime_factor))
}

pub fn vector_connected_component(image: &Image, distance_threshold: f64, fully_connected: bool) -> Result<Image, String> {
    wrap(ffi::filter_vector_connected_component(&image.inner, distance_threshold, fully_connected))
}

pub fn label_map_to_binary(image: &Image, background_value: f64, foreground_value: f64) -> Result<Image, String> {
    wrap(ffi::filter_label_map_to_binary(&image.inner, background_value, foreground_value))
}

pub fn objectness_measure(image: &Image, alpha: f64, beta: f64, gamma: f64, scale_objectness_measure: bool, object_dimension: u32, bright_object: bool) -> Result<Image, String> {
    wrap(ffi::filter_objectness_measure(&image.inner, alpha, beta, gamma, scale_objectness_measure, object_dimension, bright_object))
}

pub fn threshold_maximum_connected_components(image: &Image, minimum_object_size_in_pixels: u32, upper_boundary: f64, inside_value: u8, outside_value: u8) -> Result<Image, String> {
    wrap(ffi::filter_threshold_maximum_connected_components(&image.inner, minimum_object_size_in_pixels, upper_boundary, inside_value, outside_value))
}

pub fn gradient(image: &Image, use_image_spacing: bool, use_image_direction: bool) -> Result<Image, String> {
    wrap(ffi::filter_gradient(&image.inner, use_image_spacing, use_image_direction))
}

pub fn gradient_recursive_gaussian(image: &Image, sigma: f64, normalize_across_scale: bool, use_image_direction: bool) -> Result<Image, String> {
    wrap(ffi::filter_gradient_recursive_gaussian(&image.inner, sigma, normalize_across_scale, use_image_direction))
}

pub fn dicom_orient(image: &Image, orientation: &str) -> Result<Image, String> {
    wrap(ffi::filter_dicom_orient(&image.inner, orientation))
}

pub fn scalar_connected_component(image: &Image, distance_threshold: f64, fully_connected: bool) -> Result<Image, String> {
    wrap(ffi::filter_scalar_connected_component(&image.inner, distance_threshold, fully_connected))
}

pub fn scalar_to_rgb_colormap(image: &Image, colormap: i32, use_input_image_extrema_for_scaling: bool) -> Result<Image, String> {
    wrap(ffi::filter_scalar_to_rgb_colormap(&image.inner, colormap, use_input_image_extrema_for_scaling))
}

pub fn staple(image: &Image, confidence_weight: f64, foreground_value: f64, maximum_iterations: u32) -> Result<Image, String> {
    wrap(ffi::filter_staple(&image.inner, confidence_weight, foreground_value, maximum_iterations))
}

pub fn multi_label_staple(image: &Image, termination_threshold: f32, max_iterations: u32) -> Result<Image, String> {
    wrap(ffi::filter_multi_label_staple(&image.inner, termination_threshold, max_iterations))
}

// Unary with vector params

pub fn grayscale_connected_closing(image: &Image, seed: &[u32], fully_connected: bool) -> Result<Image, String> {
    wrap(ffi::filter_grayscale_connected_closing(&image.inner, seed, fully_connected))
}

pub fn grayscale_connected_opening(image: &Image, seed: &[u32], fully_connected: bool) -> Result<Image, String> {
    wrap(ffi::filter_grayscale_connected_opening(&image.inner, seed, fully_connected))
}

pub fn permute_axes(image: &Image, order: &[u32]) -> Result<Image, String> {
    wrap(ffi::filter_permute_axes(&image.inner, order))
}

pub fn slic(image: &Image, super_grid_size: &[u32], spatial_proximity_weight: f64, maximum_number_of_iterations: u32, enforce_connectivity: bool, initialization_perturbation: bool) -> Result<Image, String> {
    wrap(ffi::filter_slic(&image.inner, super_grid_size, spatial_proximity_weight, maximum_number_of_iterations, enforce_connectivity, initialization_perturbation))
}

pub fn region_of_interest(image: &Image, size: &[u32], index: &[i32]) -> Result<Image, String> {
    wrap(ffi::filter_region_of_interest(&image.inner, size, index))
}

pub fn simple_contour_extractor(image: &Image, input_foreground_value: f64, input_background_value: f64, radius: &[u32], output_foreground_value: f64, output_background_value: f64) -> Result<Image, String> {
    wrap(ffi::filter_simple_contour_extractor(&image.inner, input_foreground_value, input_background_value, radius, output_foreground_value, output_background_value))
}

pub fn slice(image: &Image, start: &[i32], stop: &[i32], step: &[i32]) -> Result<Image, String> {
    wrap(ffi::filter_slice(&image.inner, start, stop, step))
}

pub fn expand(image: &Image, expand_factors: &[u32], interpolator: i32) -> Result<Image, String> {
    wrap(ffi::filter_expand(&image.inner, expand_factors, interpolator))
}

pub fn stochastic_fractal_dimension(image: &Image, neighborhood_radius: &[u32]) -> Result<Image, String> {
    wrap(ffi::filter_stochastic_fractal_dimension(&image.inner, neighborhood_radius))
}

pub fn scalar_image_kmeans(image: &Image, class_means: &[f64], use_non_contiguous_labels: bool) -> Result<Image, String> {
    wrap(ffi::filter_scalar_image_kmeans(&image.inner, class_means, use_non_contiguous_labels))
}

pub fn n4_bias_field_correction(image: &Image, convergence_threshold: f64, max_iterations: &[u32], num_histogram_bins: u32, num_control_points: &[u32], spline_order: u32) -> Result<Image, String> {
    wrap(ffi::filter_n4_bias_field_correction(&image.inner, convergence_threshold, max_iterations, num_histogram_bins, num_control_points, spline_order))
}

// Binary filters (two-image)

pub fn convolution(image: &Image, kernel: &Image, normalize: bool, boundary_condition: i32, output_region_mode: i32) -> Result<Image, String> {
    wrap(ffi::filter_convolution(&image.inner, &kernel.inner, normalize, boundary_condition, output_region_mode))
}

pub fn fft_convolution(image: &Image, kernel: &Image, normalize: bool, boundary_condition: i32, output_region_mode: i32) -> Result<Image, String> {
    wrap(ffi::filter_fft_convolution(&image.inner, &kernel.inner, normalize, boundary_condition, output_region_mode))
}

pub fn label_overlay(image: &Image, label: &Image, opacity: f64, background_value: f64) -> Result<Image, String> {
    wrap(ffi::filter_label_overlay(&image.inner, &label.inner, opacity, background_value))
}

pub fn inverse_deconvolution(image: &Image, kernel: &Image, kernel_zero_magnitude_threshold: f64, normalize: bool, boundary_condition: i32, output_region_mode: i32) -> Result<Image, String> {
    wrap(ffi::filter_inverse_deconvolution(&image.inner, &kernel.inner, kernel_zero_magnitude_threshold, normalize, boundary_condition, output_region_mode))
}

pub fn tikhonov_deconvolution(image: &Image, kernel: &Image, regularization_constant: f64, normalize: bool, boundary_condition: i32, output_region_mode: i32) -> Result<Image, String> {
    wrap(ffi::filter_tikhonov_deconvolution(&image.inner, &kernel.inner, regularization_constant, normalize, boundary_condition, output_region_mode))
}

pub fn wiener_deconvolution(image: &Image, kernel: &Image, noise_variance: f64, normalize: bool, boundary_condition: i32, output_region_mode: i32) -> Result<Image, String> {
    wrap(ffi::filter_wiener_deconvolution(&image.inner, &kernel.inner, noise_variance, normalize, boundary_condition, output_region_mode))
}

pub fn richardson_lucy(image: &Image, kernel: &Image, number_of_iterations: i32, normalize: bool, boundary_condition: i32, output_region_mode: i32) -> Result<Image, String> {
    wrap(ffi::filter_richardson_lucy(&image.inner, &kernel.inner, number_of_iterations, normalize, boundary_condition, output_region_mode))
}

pub fn landweber_deconvolution(image: &Image, kernel: &Image, alpha: f64, number_of_iterations: i32, normalize: bool, boundary_condition: i32, output_region_mode: i32) -> Result<Image, String> {
    wrap(ffi::filter_landweber_deconvolution(&image.inner, &kernel.inner, alpha, number_of_iterations, normalize, boundary_condition, output_region_mode))
}

pub fn projected_landweber_deconvolution(image: &Image, kernel: &Image, alpha: f64, number_of_iterations: i32, normalize: bool, boundary_condition: i32, output_region_mode: i32) -> Result<Image, String> {
    wrap(ffi::filter_projected_landweber_deconvolution(&image.inner, &kernel.inner, alpha, number_of_iterations, normalize, boundary_condition, output_region_mode))
}

pub fn fft_normalized_correlation(fixed_image: &Image, moving_image: &Image, required_number_of_overlapping_pixels: u64, required_fraction_of_overlapping_pixels: f64) -> Result<Image, String> {
    wrap(ffi::filter_fft_normalized_correlation(&fixed_image.inner, &moving_image.inner, required_number_of_overlapping_pixels, required_fraction_of_overlapping_pixels))
}

// Three-image filters

pub fn ternary_add(img1: &Image, img2: &Image, img3: &Image) -> Result<Image, String> {
    wrap(ffi::filter_ternary_add(&img1.inner, &img2.inner, &img3.inner))
}

pub fn ternary_magnitude(img1: &Image, img2: &Image, img3: &Image) -> Result<Image, String> {
    wrap(ffi::filter_ternary_magnitude(&img1.inner, &img2.inner, &img3.inner))
}

pub fn ternary_magnitude_squared(img1: &Image, img2: &Image, img3: &Image) -> Result<Image, String> {
    wrap(ffi::filter_ternary_magnitude_squared(&img1.inner, &img2.inner, &img3.inner))
}

pub fn masked_fft_normalized_correlation(fixed_image: &Image, moving_image: &Image, fixed_mask: &Image, moving_mask: &Image, required_overlapping_pixels: u64, required_fraction: f32) -> Result<Image, String> {
    wrap(ffi::filter_masked_fft_normalized_correlation(&fixed_image.inner, &moving_image.inner, &fixed_mask.inner, &moving_mask.inner, required_overlapping_pixels, required_fraction))
}

// NeighborhoodConnected (no seeds)

pub fn neighborhood_connected(image: &Image, lower: f64, upper: f64, radius: &[u32], replace_value: f64) -> Result<Image, String> {
    wrap(ffi::filter_neighborhood_connected(&image.inner, lower, upper, radius, replace_value))
}

// ── Group 1: Binary image+image ────────────────────────────────────────────

pub fn maximum(img1: &Image, img2: &Image) -> Result<Image, String> {
    wrap(ffi::filter_maximum(&img1.inner, &img2.inner))
}

pub fn minimum(img1: &Image, img2: &Image) -> Result<Image, String> {
    wrap(ffi::filter_minimum(&img1.inner, &img2.inner))
}

pub fn modulus(img1: &Image, img2: &Image) -> Result<Image, String> {
    wrap(ffi::filter_modulus(&img1.inner, &img2.inner))
}

// ── Group 2: Unary with simple scalar/bool params ──────────────────────────

pub fn laplacian(image: &Image, use_image_spacing: bool) -> Result<Image, String> {
    wrap(ffi::filter_laplacian(&image.inner, use_image_spacing))
}

pub fn bspline_decomposition(image: &Image, spline_order: u32) -> Result<Image, String> {
    wrap(ffi::filter_bspline_decomposition(&image.inner, spline_order))
}

pub fn relabel_label_map(image: &Image, reverse_ordering: bool) -> Result<Image, String> {
    wrap(ffi::filter_relabel_label_map(&image.inner, reverse_ordering))
}

pub fn label_unique_label_map(image: &Image, reverse_ordering: bool) -> Result<Image, String> {
    wrap(ffi::filter_label_unique_label_map(&image.inner, reverse_ordering))
}

pub fn label_to_rgb(image: &Image, background_value: f64) -> Result<Image, String> {
    wrap(ffi::filter_label_to_rgb(&image.inner, background_value))
}

pub fn label_voting(image: &Image) -> Result<Image, String> {
    wrap(ffi::filter_label_voting(&image.inner))
}

pub fn binary_image_to_label_map(image: &Image, fully_connected: bool, input_foreground_value: f64, output_background_value: f64) -> Result<Image, String> {
    wrap(ffi::filter_binary_image_to_label_map(&image.inner, fully_connected, input_foreground_value, output_background_value))
}

pub fn label_image_to_label_map(image: &Image, background_value: f64) -> Result<Image, String> {
    wrap(ffi::filter_label_image_to_label_map(&image.inner, background_value))
}

pub fn min_max_curvature_flow(image: &Image, time_step: f64, number_of_iterations: u32, stencil_radius: i32) -> Result<Image, String> {
    wrap(ffi::filter_min_max_curvature_flow(&image.inner, time_step, number_of_iterations, stencil_radius))
}

pub fn fast_marching(image: &Image, normalization_factor: f64, stopping_value: f64) -> Result<Image, String> {
    wrap(ffi::filter_fast_marching(&image.inner, normalization_factor, stopping_value))
}

pub fn connected_threshold(image: &Image, lower: f64, upper: f64, replace_value: u8, connectivity: i32) -> Result<Image, String> {
    wrap(ffi::filter_connected_threshold(&image.inner, lower, upper, replace_value, connectivity))
}

// ── Group 3: Unary with vector params ──────────────────────────────────────

pub fn crop(image: &Image, lower_size: &[u32], upper_size: &[u32]) -> Result<Image, String> {
    wrap(ffi::filter_crop(&image.inner, lower_size, upper_size))
}

pub fn tile(image: &Image, layout: &[u32], default_pixel_value: f64) -> Result<Image, String> {
    wrap(ffi::filter_tile(&image.inner, layout, default_pixel_value))
}

pub fn compose(image: &Image) -> Result<Image, String> {
    wrap(ffi::filter_compose(&image.inner))
}

// ── Group 4: Two-image with params ─────────────────────────────────────────

pub fn masked_assign(image: &Image, mask: &Image, assign_constant: f64) -> Result<Image, String> {
    wrap(ffi::filter_masked_assign(&image.inner, &mask.inner, assign_constant))
}

pub fn label_map_overlay(label_map: &Image, feature_image: &Image, opacity: f64) -> Result<Image, String> {
    wrap(ffi::filter_label_map_overlay(&label_map.inner, &feature_image.inner, opacity))
}

pub fn label_map_mask(label_map: &Image, feature_image: &Image, label: u64, background_value: f64, negated: bool, crop: bool) -> Result<Image, String> {
    wrap(ffi::filter_label_map_mask(&label_map.inner, &feature_image.inner, label, background_value, negated, crop))
}

pub fn join_series_two(img1: &Image, img2: &Image, origin: f64, spacing: f64) -> Result<Image, String> {
    wrap(ffi::filter_join_series_two(&img1.inner, &img2.inner, origin, spacing))
}

pub fn isolated_connected(image: &Image, seed1: &[u32], seed2: &[u32], lower: f64, upper: f64, replace_value: u8, tolerance: f64, find_upper: bool) -> Result<Image, String> {
    wrap(ffi::filter_isolated_connected(&image.inner, seed1, seed2, lower, upper, replace_value, tolerance, find_upper))
}

pub fn isolated_watershed(image: &Image, seed1: &[u32], seed2: &[u32], threshold: f64, upper_value_limit: f64, tolerance: f64, replace1: u8, replace2: u8) -> Result<Image, String> {
    wrap(ffi::filter_isolated_watershed(&image.inner, seed1, seed2, threshold, upper_value_limit, tolerance, replace1, replace2))
}

// ── Group 5: Three-image ───────────────────────────────────────────────────

pub fn normalized_correlation(image: &Image, mask_image: &Image, template_image: &Image) -> Result<Image, String> {
    wrap(ffi::filter_normalized_correlation(&image.inner, &mask_image.inner, &template_image.inner))
}

pub fn label_map_contour_overlay(label_map: &Image, feature_image: &Image, opacity: f64, dilation_radius: &[u32], contour_thickness: &[u32], slice_dimension: u32, contour_type: i32, priority: i32) -> Result<Image, String> {
    wrap(ffi::filter_label_map_contour_overlay(&label_map.inner, &feature_image.inner, opacity, dilation_radius, contour_thickness, slice_dimension, contour_type, priority))
}

pub fn merge_label_map_two(img1: &Image, img2: &Image, method: i32) -> Result<Image, String> {
    wrap(ffi::filter_merge_label_map_two(&img1.inner, &img2.inner, method))
}

// ── Group 6: Image sources (no Image input) ────────────────────────────────

pub fn source_gaussian(pixel_type: i32, size: &[u32], sigma: &[f64], mean: &[f64], scale: f64) -> Result<Image, String> {
    wrap(ffi::source_gaussian(pixel_type, size, sigma, mean, scale))
}

pub fn source_gabor(pixel_type: i32, size: &[u32], sigma: &[f64], mean: &[f64], frequency: f64) -> Result<Image, String> {
    wrap(ffi::source_gabor(pixel_type, size, sigma, mean, frequency))
}

pub fn source_grid(pixel_type: i32, size: &[u32], sigma: &[f64], grid_spacing: &[f64], grid_offset: &[f64], scale: f64) -> Result<Image, String> {
    wrap(ffi::source_grid(pixel_type, size, sigma, grid_spacing, grid_offset, scale))
}

pub fn source_physical_point(pixel_type: i32, size: &[u32]) -> Result<Image, String> {
    wrap(ffi::source_physical_point(pixel_type, size))
}

pub fn aggregate_label_map(image: &Image) -> Result<Image, String> {
    wrap(ffi::filter_aggregate_label_map(&image.inner))
}

pub fn label_map_to_rgb(image: &Image) -> Result<Image, String> {
    wrap(ffi::filter_label_map_to_rgb(&image.inner))
}

pub fn extract(image: &Image, size: &[u32], index: &[i32]) -> Result<Image, String> {
    wrap(ffi::filter_extract(&image.inner, size, index))
}

pub fn nary_add_two(img1: &Image, img2: &Image) -> Result<Image, String> {
    wrap(ffi::filter_nary_add_two(&img1.inner, &img2.inner))
}

pub fn nary_maximum_two(img1: &Image, img2: &Image) -> Result<Image, String> {
    wrap(ffi::filter_nary_maximum_two(&img1.inner, &img2.inner))
}

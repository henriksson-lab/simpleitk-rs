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

        // ── Unary filters ────────────────────────────────────────────────
        fn filter_abs(img: &Image) -> Result<UniquePtr<Image>>;
        fn filter_acos(img: &Image) -> Result<UniquePtr<Image>>;
        fn filter_adaptive_histogram_equalization(img: &Image, radius: &[u32], alpha: f32, beta: f32) -> Result<UniquePtr<Image>>;
        fn filter_additive_gaussian_noise(img: &Image, standard_deviation: f64, mean: f64, seed: u32) -> Result<UniquePtr<Image>>;
        fn filter_anti_alias_binary(img: &Image, maximum_rms_error: f64, number_of_iterations: u32) -> Result<UniquePtr<Image>>;
        fn filter_approximate_signed_distance_map(img: &Image, inside_value: f64, outside_value: f64) -> Result<UniquePtr<Image>>;
        fn filter_area_closing(img: &Image, lambda: f64, use_image_spacing: bool, fully_connected: bool) -> Result<UniquePtr<Image>>;
        fn filter_area_opening(img: &Image, lambda: f64, use_image_spacing: bool, fully_connected: bool) -> Result<UniquePtr<Image>>;
        fn filter_asin(img: &Image) -> Result<UniquePtr<Image>>;
        fn filter_atan(img: &Image) -> Result<UniquePtr<Image>>;
        fn filter_bilateral(img: &Image, domain_sigma: f64, range_sigma: f64, number_of_range_gaussian_samples: u32) -> Result<UniquePtr<Image>>;
        fn filter_bin_shrink(img: &Image, shrink_factors: &[u32]) -> Result<UniquePtr<Image>>;
        fn filter_binary_closing_by_reconstruction(img: &Image, kernel_radius: &[u32], kernel_type: i32, foreground_value: f64, fully_connected: bool) -> Result<UniquePtr<Image>>;
        fn filter_binary_contour(img: &Image, fully_connected: bool, background_value: f64, foreground_value: f64) -> Result<UniquePtr<Image>>;
        fn filter_binary_dilate(img: &Image, kernel_radius: &[u32], kernel_type: i32, background_value: f64, foreground_value: f64, boundary_to_foreground: bool) -> Result<UniquePtr<Image>>;
        fn filter_binary_erode(img: &Image, kernel_radius: &[u32], kernel_type: i32, background_value: f64, foreground_value: f64, boundary_to_foreground: bool) -> Result<UniquePtr<Image>>;
        fn filter_binary_fillhole(img: &Image, fully_connected: bool, foreground_value: f64) -> Result<UniquePtr<Image>>;
        fn filter_binary_grind_peak(img: &Image, fully_connected: bool, foreground_value: f64, background_value: f64) -> Result<UniquePtr<Image>>;
        fn filter_binary_median(img: &Image, radius: &[u32], foreground_value: f64, background_value: f64) -> Result<UniquePtr<Image>>;
        fn filter_binary_min_max_curvature_flow(img: &Image, time_step: f64, number_of_iterations: u32, stencil_radius: i32, threshold: f64) -> Result<UniquePtr<Image>>;
        fn filter_binary_morphological_closing(img: &Image, kernel_radius: &[u32], kernel_type: i32, foreground_value: f64, safe_edge_handling: bool) -> Result<UniquePtr<Image>>;
        fn filter_binary_morphological_opening(img: &Image, kernel_radius: &[u32], kernel_type: i32, background_value: f64, foreground_value: f64) -> Result<UniquePtr<Image>>;
        fn filter_binary_not(img: &Image, foreground_value: f64, background_value: f64) -> Result<UniquePtr<Image>>;
        fn filter_binary_opening_by_reconstruction(img: &Image, kernel_radius: &[u32], kernel_type: i32, foreground_value: f64, fully_connected: bool) -> Result<UniquePtr<Image>>;
        fn filter_binary_projection(img: &Image, projection_dimension: u32, foreground_value: f64, background_value: f64) -> Result<UniquePtr<Image>>;
        fn filter_binary_pruning(img: &Image, iteration: u32) -> Result<UniquePtr<Image>>;
        fn filter_binary_thinning(img: &Image) -> Result<UniquePtr<Image>>;
        fn filter_binary_threshold(img: &Image, lower_threshold: f64, upper_threshold: f64, inside_value: u8, outside_value: u8) -> Result<UniquePtr<Image>>;
        fn filter_binary_threshold_projection(img: &Image, projection_dimension: u32, threshold_value: f64, foreground_value: u8, background_value: u8) -> Result<UniquePtr<Image>>;
        fn filter_binomial_blur(img: &Image, repetitions: u32) -> Result<UniquePtr<Image>>;
        fn filter_bitwise_not(img: &Image) -> Result<UniquePtr<Image>>;
        fn filter_black_top_hat(img: &Image, kernel_radius: &[u32], kernel_type: i32, safe_border: bool) -> Result<UniquePtr<Image>>;
        fn filter_bounded_reciprocal(img: &Image) -> Result<UniquePtr<Image>>;
        fn filter_box_mean(img: &Image, radius: &[u32]) -> Result<UniquePtr<Image>>;
        fn filter_box_sigma(img: &Image, radius: &[u32]) -> Result<UniquePtr<Image>>;
        fn filter_canny_edge_detection(img: &Image, lower_threshold: f64, upper_threshold: f64, variance: f64, maximum_error: f64) -> Result<UniquePtr<Image>>;
        fn filter_cast(img: &Image, pixel_id: i32) -> Result<UniquePtr<Image>>;
        fn filter_clamp(img: &Image, lower_bound: f64, upper_bound: f64) -> Result<UniquePtr<Image>>;
        fn filter_closing_by_reconstruction(img: &Image, kernel_radius: &[u32], kernel_type: i32, fully_connected: bool, preserve_intensities: bool) -> Result<UniquePtr<Image>>;
        fn filter_complex_to_imaginary(img: &Image) -> Result<UniquePtr<Image>>;
        fn filter_complex_to_modulus(img: &Image) -> Result<UniquePtr<Image>>;
        fn filter_complex_to_phase(img: &Image) -> Result<UniquePtr<Image>>;
        fn filter_complex_to_real(img: &Image) -> Result<UniquePtr<Image>>;
        fn filter_connected_component(img: &Image, fully_connected: bool) -> Result<UniquePtr<Image>>;
        fn filter_cos(img: &Image) -> Result<UniquePtr<Image>>;
        fn filter_curvature_anisotropic_diffusion(img: &Image, time_step: f64, conductance_parameter: f64, conductance_scaling_update_interval: u32, number_of_iterations: u32) -> Result<UniquePtr<Image>>;
        fn filter_curvature_flow(img: &Image, time_step: f64, number_of_iterations: u32) -> Result<UniquePtr<Image>>;
        fn filter_cyclic_shift(img: &Image, shift: &[i32]) -> Result<UniquePtr<Image>>;
        fn filter_danielsson_distance_map(img: &Image, input_is_binary: bool, squared_distance: bool, use_image_spacing: bool) -> Result<UniquePtr<Image>>;
        fn filter_dilate_object_morphology(img: &Image, kernel_radius: &[u32], kernel_type: i32, object_value: f64, background_value: f64) -> Result<UniquePtr<Image>>;
        fn filter_discrete_gaussian(img: &Image, variance: f64, maximum_kernel_width: u32, maximum_error: f64, use_image_spacing: bool) -> Result<UniquePtr<Image>>;
        fn filter_discrete_gaussian_derivative(img: &Image, variance: f64, order: u32, maximum_kernel_width: u32, maximum_error: f64, normalize_across_scale: bool) -> Result<UniquePtr<Image>>;
        fn filter_displacement_field_jacobian_determinant(img: &Image, use_image_spacing: bool) -> Result<UniquePtr<Image>>;
        fn filter_double_threshold(img: &Image, threshold1: f64, threshold2: f64, threshold3: f64, threshold4: f64, inside_value: u8, outside_value: u8, fully_connected: bool) -> Result<UniquePtr<Image>>;
        fn filter_edge_potential(img: &Image) -> Result<UniquePtr<Image>>;
        fn filter_erode_object_morphology(img: &Image, kernel_radius: &[u32], kernel_type: i32, object_value: f64, background_value: f64) -> Result<UniquePtr<Image>>;
        fn filter_exp(img: &Image) -> Result<UniquePtr<Image>>;
        fn filter_exp_negative(img: &Image) -> Result<UniquePtr<Image>>;
        fn filter_fast_approximate_rank(img: &Image, radius: &[u32], rank: f64) -> Result<UniquePtr<Image>>;
        fn filter_fft_shift(img: &Image, inverse: bool) -> Result<UniquePtr<Image>>;
        fn filter_flip_image(img: &Image, flip_axes: &[bool], flip_about_origin: bool) -> Result<UniquePtr<Image>>;
        fn filter_gradient_anisotropic_diffusion(img: &Image, time_step: f64, conductance_parameter: f64, conductance_scaling_update_interval: u32, number_of_iterations: u32) -> Result<UniquePtr<Image>>;
        fn filter_gradient_magnitude(img: &Image) -> Result<UniquePtr<Image>>;
        fn filter_gradient_magnitude_recursive_gaussian(img: &Image, sigma: f64, normalize_across_scale: bool) -> Result<UniquePtr<Image>>;
        fn filter_grayscale_dilate(img: &Image, kernel_radius: &[u32], kernel_type: i32) -> Result<UniquePtr<Image>>;
        fn filter_grayscale_erode(img: &Image, kernel_radius: &[u32], kernel_type: i32) -> Result<UniquePtr<Image>>;
        fn filter_grayscale_fillhole(img: &Image, fully_connected: bool) -> Result<UniquePtr<Image>>;
        fn filter_grayscale_grind_peak(img: &Image, fully_connected: bool) -> Result<UniquePtr<Image>>;
        fn filter_grayscale_morphological_closing(img: &Image, kernel_radius: &[u32], kernel_type: i32, safe_border: bool) -> Result<UniquePtr<Image>>;
        fn filter_grayscale_morphological_opening(img: &Image, kernel_radius: &[u32], kernel_type: i32, safe_border: bool) -> Result<UniquePtr<Image>>;
        fn filter_h_concave(img: &Image, height: f64, fully_connected: bool) -> Result<UniquePtr<Image>>;
        fn filter_h_convex(img: &Image, height: f64, fully_connected: bool) -> Result<UniquePtr<Image>>;
        fn filter_h_maxima(img: &Image, height: f64, fully_connected: bool) -> Result<UniquePtr<Image>>;
        fn filter_h_minima(img: &Image, height: f64, fully_connected: bool) -> Result<UniquePtr<Image>>;
        fn filter_huang_threshold(img: &Image, inside_value: u8, outside_value: u8, number_of_histogram_bins: u32) -> Result<UniquePtr<Image>>;
        fn filter_intensity_windowing(img: &Image, window_minimum: f64, window_maximum: f64, output_minimum: f64, output_maximum: f64) -> Result<UniquePtr<Image>>;
        fn filter_intermodes_threshold(img: &Image, inside_value: u8, outside_value: u8, number_of_histogram_bins: u32) -> Result<UniquePtr<Image>>;
        fn filter_invert_intensity(img: &Image, maximum: f64) -> Result<UniquePtr<Image>>;
        fn filter_iso_contour_distance(img: &Image, level_set_value: f64, far_value: f64) -> Result<UniquePtr<Image>>;
        fn filter_iso_data_threshold(img: &Image, inside_value: u8, outside_value: u8, number_of_histogram_bins: u32) -> Result<UniquePtr<Image>>;
        fn filter_kittler_illingworth_threshold(img: &Image, inside_value: u8, outside_value: u8, number_of_histogram_bins: u32) -> Result<UniquePtr<Image>>;
        fn filter_label_contour(img: &Image, fully_connected: bool, background_value: f64) -> Result<UniquePtr<Image>>;
        fn filter_laplacian_recursive_gaussian(img: &Image, sigma: f64, normalize_across_scale: bool) -> Result<UniquePtr<Image>>;
        fn filter_laplacian_sharpening(img: &Image, use_image_spacing: bool) -> Result<UniquePtr<Image>>;
        fn filter_li_threshold(img: &Image, inside_value: u8, outside_value: u8, number_of_histogram_bins: u32) -> Result<UniquePtr<Image>>;
        fn filter_log(img: &Image) -> Result<UniquePtr<Image>>;
        fn filter_log10(img: &Image) -> Result<UniquePtr<Image>>;
        fn filter_maximum_entropy_threshold(img: &Image, inside_value: u8, outside_value: u8, number_of_histogram_bins: u32) -> Result<UniquePtr<Image>>;
        fn filter_maximum_projection(img: &Image, projection_dimension: u32) -> Result<UniquePtr<Image>>;
        fn filter_mean(img: &Image, radius: &[u32]) -> Result<UniquePtr<Image>>;
        fn filter_mean_projection(img: &Image, projection_dimension: u32) -> Result<UniquePtr<Image>>;
        fn filter_median(img: &Image, radius: &[u32]) -> Result<UniquePtr<Image>>;
        fn filter_median_projection(img: &Image, projection_dimension: u32) -> Result<UniquePtr<Image>>;
        fn filter_minimum_projection(img: &Image, projection_dimension: u32) -> Result<UniquePtr<Image>>;
        fn filter_mirror_pad(img: &Image, pad_lower_bound: &[u32], pad_upper_bound: &[u32]) -> Result<UniquePtr<Image>>;
        fn filter_moments_threshold(img: &Image, inside_value: u8, outside_value: u8, number_of_histogram_bins: u32) -> Result<UniquePtr<Image>>;
        fn filter_morphological_gradient(img: &Image, kernel_radius: &[u32], kernel_type: i32) -> Result<UniquePtr<Image>>;
        fn filter_morphological_watershed(img: &Image, level: f64, mark_watershed_line: bool, fully_connected: bool) -> Result<UniquePtr<Image>>;
        fn filter_noise_image_filter(img: &Image) -> Result<UniquePtr<Image>>;
        fn filter_normalize(img: &Image) -> Result<UniquePtr<Image>>;
        fn filter_normalize_to_constant(img: &Image, constant: f64) -> Result<UniquePtr<Image>>;
        fn filter_not(img: &Image) -> Result<UniquePtr<Image>>;
        fn filter_opening_by_reconstruction(img: &Image, kernel_radius: &[u32], kernel_type: i32, fully_connected: bool, preserve_intensities: bool) -> Result<UniquePtr<Image>>;
        fn filter_otsu_multiple_thresholds(img: &Image, number_of_thresholds: u32, label_offset: u8, number_of_histogram_bins: u32, valley_emphasis: bool) -> Result<UniquePtr<Image>>;
        fn filter_otsu_threshold(img: &Image, inside_value: u8, outside_value: u8, number_of_histogram_bins: u32) -> Result<UniquePtr<Image>>;
        fn filter_rank(img: &Image, radius: &[u32], rank: f64) -> Result<UniquePtr<Image>>;
        fn filter_recursive_gaussian(img: &Image, sigma: f64, normalize_across_scale: bool, order: u32, direction: u32) -> Result<UniquePtr<Image>>;
        fn filter_regional_maxima(img: &Image, background_value: f64, fully_connected: bool, flat_is_maxima: bool) -> Result<UniquePtr<Image>>;
        fn filter_regional_minima(img: &Image, background_value: f64, fully_connected: bool, flat_is_minima: bool) -> Result<UniquePtr<Image>>;
        fn filter_relabel_component(img: &Image, minimum_object_size: u64, sort_by_object_size: bool) -> Result<UniquePtr<Image>>;
        fn filter_renyi_entropy_threshold(img: &Image, inside_value: u8, outside_value: u8, number_of_histogram_bins: u32) -> Result<UniquePtr<Image>>;
        fn filter_rescale_intensity(img: &Image, output_minimum: f64, output_maximum: f64) -> Result<UniquePtr<Image>>;
        fn filter_round(img: &Image) -> Result<UniquePtr<Image>>;
        fn filter_salt_and_pepper_noise(img: &Image, probability: f64, seed: u32) -> Result<UniquePtr<Image>>;
        fn filter_shanbhag_threshold(img: &Image, inside_value: u8, outside_value: u8, number_of_histogram_bins: u32) -> Result<UniquePtr<Image>>;
        fn filter_shift_scale(img: &Image, shift: f64, scale: f64) -> Result<UniquePtr<Image>>;
        fn filter_shot_noise(img: &Image, scale: f64, seed: u32) -> Result<UniquePtr<Image>>;
        fn filter_shrink(img: &Image, shrink_factors: &[u32]) -> Result<UniquePtr<Image>>;
        fn filter_sigmoid(img: &Image, alpha: f64, beta: f64, output_maximum: f64, output_minimum: f64) -> Result<UniquePtr<Image>>;
        fn filter_signed_danielsson_distance_map(img: &Image, inside_is_positive: bool, squared_distance: bool, use_image_spacing: bool) -> Result<UniquePtr<Image>>;
        fn filter_signed_maurer_distance_map(img: &Image, inside_is_positive: bool, squared_distance: bool, use_image_spacing: bool, background_value: f64) -> Result<UniquePtr<Image>>;
        fn filter_sin(img: &Image) -> Result<UniquePtr<Image>>;
        fn filter_smoothing_recursive_gaussian(img: &Image, sigma: &[f64], normalize_across_scale: bool) -> Result<UniquePtr<Image>>;
        fn filter_speckle_noise(img: &Image, standard_deviation: f64, seed: u32) -> Result<UniquePtr<Image>>;
        fn filter_sqrt(img: &Image) -> Result<UniquePtr<Image>>;
        fn filter_square(img: &Image) -> Result<UniquePtr<Image>>;
        fn filter_standard_deviation_projection(img: &Image, projection_dimension: u32) -> Result<UniquePtr<Image>>;
        fn filter_sum_projection(img: &Image, projection_dimension: u32) -> Result<UniquePtr<Image>>;
        fn filter_tan(img: &Image) -> Result<UniquePtr<Image>>;
        fn filter_threshold(img: &Image, lower_threshold: f64, upper_threshold: f64, outside_value: f64) -> Result<UniquePtr<Image>>;
        fn filter_triangle_threshold(img: &Image, inside_value: u8, outside_value: u8, number_of_histogram_bins: u32) -> Result<UniquePtr<Image>>;
        fn filter_unary_minus(img: &Image) -> Result<UniquePtr<Image>>;
        fn filter_unsharp_mask(img: &Image, sigmas: &[f64], amount: f64, threshold: f64) -> Result<UniquePtr<Image>>;
        fn filter_valued_regional_maxima(img: &Image, fully_connected: bool) -> Result<UniquePtr<Image>>;
        fn filter_valued_regional_minima(img: &Image, fully_connected: bool) -> Result<UniquePtr<Image>>;
        fn filter_vector_index_selection_cast(img: &Image, index: u32, output_pixel_type: i32) -> Result<UniquePtr<Image>>;
        fn filter_vector_magnitude(img: &Image) -> Result<UniquePtr<Image>>;
        fn filter_voting_binary(img: &Image, radius: &[u32], birth_threshold: u32, survival_threshold: u32, foreground_value: f64, background_value: f64) -> Result<UniquePtr<Image>>;
        fn filter_voting_binary_hole_filling(img: &Image, radius: &[u32], majority_threshold: u32, foreground_value: f64, background_value: f64, maximum_number_of_iterations: u32) -> Result<UniquePtr<Image>>;
        fn filter_voting_binary_iterative_hole_filling(img: &Image, radius: &[u32], majority_threshold: u32, foreground_value: f64, background_value: f64, maximum_number_of_iterations: u32) -> Result<UniquePtr<Image>>;
        fn filter_white_top_hat(img: &Image, kernel_radius: &[u32], kernel_type: i32, safe_border: bool) -> Result<UniquePtr<Image>>;
        fn filter_yen_threshold(img: &Image, inside_value: u8, outside_value: u8, number_of_histogram_bins: u32) -> Result<UniquePtr<Image>>;
        fn filter_zero_crossing(img: &Image, foreground_value: u8, background_value: u8) -> Result<UniquePtr<Image>>;
        fn filter_zero_crossing_based_edge_detection(img: &Image, variance: f64, maximum_error: f64) -> Result<UniquePtr<Image>>;
        fn filter_zero_flux_neumann_pad(img: &Image, pad_lower_bound: &[u32], pad_upper_bound: &[u32]) -> Result<UniquePtr<Image>>;
        fn filter_constant_pad(img: &Image, pad_lower_bound: &[u32], pad_upper_bound: &[u32], constant: f64) -> Result<UniquePtr<Image>>;
        fn filter_wrap_pad(img: &Image, pad_lower_bound: &[u32], pad_upper_bound: &[u32]) -> Result<UniquePtr<Image>>;

        // ── Binary filters ───────────────────────────────────────────────
        fn filter_absolute_value_difference(img1: &Image, img2: &Image) -> Result<UniquePtr<Image>>;
        fn filter_add(img1: &Image, img2: &Image) -> Result<UniquePtr<Image>>;
        fn filter_and(img1: &Image, img2: &Image) -> Result<UniquePtr<Image>>;
        fn filter_atan2(img1: &Image, img2: &Image) -> Result<UniquePtr<Image>>;
        fn filter_binary_magnitude(img1: &Image, img2: &Image) -> Result<UniquePtr<Image>>;
        fn filter_binary_reconstruction_by_dilation(img1: &Image, img2: &Image, background_value: f64, foreground_value: f64, fully_connected: bool) -> Result<UniquePtr<Image>>;
        fn filter_binary_reconstruction_by_erosion(img1: &Image, img2: &Image, background_value: f64, foreground_value: f64, fully_connected: bool) -> Result<UniquePtr<Image>>;
        fn filter_checker_board(img1: &Image, img2: &Image, checker_pattern: &[u32]) -> Result<UniquePtr<Image>>;
        fn filter_divide(img1: &Image, img2: &Image) -> Result<UniquePtr<Image>>;
        fn filter_divide_floor(img1: &Image, img2: &Image) -> Result<UniquePtr<Image>>;
        fn filter_divide_real(img1: &Image, img2: &Image) -> Result<UniquePtr<Image>>;
        fn filter_equal(img1: &Image, img2: &Image) -> Result<UniquePtr<Image>>;
        fn filter_grayscale_geodesic_dilate(marker: &Image, mask: &Image, run_one_iteration: bool, fully_connected: bool) -> Result<UniquePtr<Image>>;
        fn filter_grayscale_geodesic_erode(marker: &Image, mask: &Image, run_one_iteration: bool, fully_connected: bool) -> Result<UniquePtr<Image>>;
        fn filter_greater(img1: &Image, img2: &Image) -> Result<UniquePtr<Image>>;
        fn filter_greater_equal(img1: &Image, img2: &Image) -> Result<UniquePtr<Image>>;
        fn filter_histogram_matching(img: &Image, reference: &Image, number_of_histogram_levels: u32, number_of_match_points: u32, threshold_at_mean_intensity: bool) -> Result<UniquePtr<Image>>;
        fn filter_less(img1: &Image, img2: &Image) -> Result<UniquePtr<Image>>;
        fn filter_less_equal(img1: &Image, img2: &Image) -> Result<UniquePtr<Image>>;
        fn filter_magnitude_and_phase_to_complex(img1: &Image, img2: &Image) -> Result<UniquePtr<Image>>;
        fn filter_mask_image(img: &Image, mask: &Image, outside_value: f64, masking_value: f64) -> Result<UniquePtr<Image>>;
        fn filter_mask_negated_image(img: &Image, mask: &Image, outside_value: f64, masking_value: f64) -> Result<UniquePtr<Image>>;
        fn filter_morphological_watershed_from_markers(img: &Image, marker: &Image, mark_watershed_line: bool, fully_connected: bool) -> Result<UniquePtr<Image>>;
        fn filter_multiply(img1: &Image, img2: &Image) -> Result<UniquePtr<Image>>;
        fn filter_not_equal(img1: &Image, img2: &Image) -> Result<UniquePtr<Image>>;
        fn filter_or(img1: &Image, img2: &Image) -> Result<UniquePtr<Image>>;
        fn filter_pow(img1: &Image, img2: &Image) -> Result<UniquePtr<Image>>;
        fn filter_real_and_imaginary_to_complex(img1: &Image, img2: &Image) -> Result<UniquePtr<Image>>;
        fn filter_reconstruction_by_dilation(img1: &Image, img2: &Image, fully_connected: bool) -> Result<UniquePtr<Image>>;
        fn filter_reconstruction_by_erosion(img1: &Image, img2: &Image, fully_connected: bool) -> Result<UniquePtr<Image>>;
        fn filter_squared_difference(img1: &Image, img2: &Image) -> Result<UniquePtr<Image>>;
        fn filter_subtract(img1: &Image, img2: &Image) -> Result<UniquePtr<Image>>;
        fn filter_xor(img1: &Image, img2: &Image) -> Result<UniquePtr<Image>>;

        // ── New filters ──────────────────────────────────────────────────

        // Unary (no params)
        fn filter_sobel_edge_detection(img: &Image) -> Result<UniquePtr<Image>>;
        fn filter_forward_fft(img: &Image) -> Result<UniquePtr<Image>>;
        fn filter_inverse_fft(img: &Image) -> Result<UniquePtr<Image>>;
        fn filter_real_to_half_hermitian_forward_fft(img: &Image) -> Result<UniquePtr<Image>>;
        fn filter_toboggan(img: &Image) -> Result<UniquePtr<Image>>;
        fn filter_label_map_to_label(img: &Image) -> Result<UniquePtr<Image>>;

        // Unary with simple params
        fn filter_derivative(img: &Image, direction: u32, order: u32, use_image_spacing: bool) -> Result<UniquePtr<Image>>;
        fn filter_half_hermitian_to_real_inverse_fft(img: &Image, actual_x_dimension_is_odd: bool) -> Result<UniquePtr<Image>>;
        fn filter_fft_pad(img: &Image, boundary_condition: i32, size_greatest_prime_factor: i32) -> Result<UniquePtr<Image>>;
        fn filter_vector_connected_component(img: &Image, distance_threshold: f64, fully_connected: bool) -> Result<UniquePtr<Image>>;
        fn filter_label_map_to_binary(img: &Image, background_value: f64, foreground_value: f64) -> Result<UniquePtr<Image>>;
        fn filter_objectness_measure(img: &Image, alpha: f64, beta: f64, gamma: f64, scale_objectness_measure: bool, object_dimension: u32, bright_object: bool) -> Result<UniquePtr<Image>>;
        fn filter_threshold_maximum_connected_components(img: &Image, minimum_object_size_in_pixels: u32, upper_boundary: f64, inside_value: u8, outside_value: u8) -> Result<UniquePtr<Image>>;
        fn filter_gradient(img: &Image, use_image_spacing: bool, use_image_direction: bool) -> Result<UniquePtr<Image>>;
        fn filter_gradient_recursive_gaussian(img: &Image, sigma: f64, normalize_across_scale: bool, use_image_direction: bool) -> Result<UniquePtr<Image>>;
        fn filter_dicom_orient(img: &Image, orientation: &str) -> Result<UniquePtr<Image>>;
        fn filter_scalar_connected_component(img: &Image, distance_threshold: f64, fully_connected: bool) -> Result<UniquePtr<Image>>;
        fn filter_scalar_to_rgb_colormap(img: &Image, colormap: i32, use_input_image_extrema_for_scaling: bool) -> Result<UniquePtr<Image>>;
        fn filter_staple(img: &Image, confidence_weight: f64, foreground_value: f64, maximum_iterations: u32) -> Result<UniquePtr<Image>>;
        fn filter_multi_label_staple(img: &Image, termination_threshold: f32, max_iterations: u32) -> Result<UniquePtr<Image>>;

        // Unary with vector params
        fn filter_grayscale_connected_closing(img: &Image, seed: &[u32], fully_connected: bool) -> Result<UniquePtr<Image>>;
        fn filter_grayscale_connected_opening(img: &Image, seed: &[u32], fully_connected: bool) -> Result<UniquePtr<Image>>;
        fn filter_permute_axes(img: &Image, order: &[u32]) -> Result<UniquePtr<Image>>;
        fn filter_slic(img: &Image, super_grid_size: &[u32], spatial_proximity_weight: f64, maximum_number_of_iterations: u32, enforce_connectivity: bool, initialization_perturbation: bool) -> Result<UniquePtr<Image>>;
        fn filter_region_of_interest(img: &Image, size: &[u32], index: &[i32]) -> Result<UniquePtr<Image>>;
        fn filter_simple_contour_extractor(img: &Image, input_foreground_value: f64, input_background_value: f64, radius: &[u32], output_foreground_value: f64, output_background_value: f64) -> Result<UniquePtr<Image>>;
        fn filter_slice(img: &Image, start: &[i32], stop: &[i32], step: &[i32]) -> Result<UniquePtr<Image>>;
        fn filter_expand(img: &Image, expand_factors: &[u32], interpolator: i32) -> Result<UniquePtr<Image>>;
        fn filter_stochastic_fractal_dimension(img: &Image, neighborhood_radius: &[u32]) -> Result<UniquePtr<Image>>;
        fn filter_scalar_image_kmeans(img: &Image, class_means: &[f64], use_non_contiguous_labels: bool) -> Result<UniquePtr<Image>>;
        fn filter_n4_bias_field_correction(img: &Image, convergence_threshold: f64, max_iterations: &[u32], num_histogram_bins: u32, num_control_points: &[u32], spline_order: u32) -> Result<UniquePtr<Image>>;

        // Binary filters (two-image)
        fn filter_convolution(img: &Image, kernel: &Image, normalize: bool, boundary_condition: i32, output_region_mode: i32) -> Result<UniquePtr<Image>>;
        fn filter_fft_convolution(img: &Image, kernel: &Image, normalize: bool, boundary_condition: i32, output_region_mode: i32) -> Result<UniquePtr<Image>>;
        fn filter_label_overlay(img: &Image, label: &Image, opacity: f64, background_value: f64) -> Result<UniquePtr<Image>>;
        fn filter_inverse_deconvolution(img: &Image, kernel: &Image, kernel_zero_magnitude_threshold: f64, normalize: bool, boundary_condition: i32, output_region_mode: i32) -> Result<UniquePtr<Image>>;
        fn filter_tikhonov_deconvolution(img: &Image, kernel: &Image, regularization_constant: f64, normalize: bool, boundary_condition: i32, output_region_mode: i32) -> Result<UniquePtr<Image>>;
        fn filter_wiener_deconvolution(img: &Image, kernel: &Image, noise_variance: f64, normalize: bool, boundary_condition: i32, output_region_mode: i32) -> Result<UniquePtr<Image>>;
        fn filter_richardson_lucy(img: &Image, kernel: &Image, number_of_iterations: i32, normalize: bool, boundary_condition: i32, output_region_mode: i32) -> Result<UniquePtr<Image>>;
        fn filter_landweber_deconvolution(img: &Image, kernel: &Image, alpha: f64, number_of_iterations: i32, normalize: bool, boundary_condition: i32, output_region_mode: i32) -> Result<UniquePtr<Image>>;
        fn filter_projected_landweber_deconvolution(img: &Image, kernel: &Image, alpha: f64, number_of_iterations: i32, normalize: bool, boundary_condition: i32, output_region_mode: i32) -> Result<UniquePtr<Image>>;
        fn filter_fft_normalized_correlation(fixed_image: &Image, moving_image: &Image, required_number_of_overlapping_pixels: u64, required_fraction_of_overlapping_pixels: f64) -> Result<UniquePtr<Image>>;

        // Three-image filters
        fn filter_ternary_add(img1: &Image, img2: &Image, img3: &Image) -> Result<UniquePtr<Image>>;
        fn filter_ternary_magnitude(img1: &Image, img2: &Image, img3: &Image) -> Result<UniquePtr<Image>>;
        fn filter_ternary_magnitude_squared(img1: &Image, img2: &Image, img3: &Image) -> Result<UniquePtr<Image>>;
        fn filter_masked_fft_normalized_correlation(fixed_image: &Image, moving_image: &Image, fixed_mask: &Image, moving_mask: &Image, required_overlapping_pixels: u64, required_fraction: f32) -> Result<UniquePtr<Image>>;

        // NeighborhoodConnected
        fn filter_neighborhood_connected(img: &Image, lower: f64, upper: f64, radius: &[u32], replace_value: f64) -> Result<UniquePtr<Image>>;

        // ── Group 1: Binary image+image ──────────────────────────────────
        fn filter_maximum(img1: &Image, img2: &Image) -> Result<UniquePtr<Image>>;
        fn filter_minimum(img1: &Image, img2: &Image) -> Result<UniquePtr<Image>>;
        fn filter_modulus(img1: &Image, img2: &Image) -> Result<UniquePtr<Image>>;

        // ── Group 2: Unary with simple scalar/bool params ────────────────
        fn filter_laplacian(img: &Image, use_image_spacing: bool) -> Result<UniquePtr<Image>>;
        fn filter_bspline_decomposition(img: &Image, spline_order: u32) -> Result<UniquePtr<Image>>;
        fn filter_relabel_label_map(img: &Image, reverse_ordering: bool) -> Result<UniquePtr<Image>>;
        fn filter_label_unique_label_map(img: &Image, reverse_ordering: bool) -> Result<UniquePtr<Image>>;
        fn filter_label_to_rgb(img: &Image, background_value: f64) -> Result<UniquePtr<Image>>;
        fn filter_label_voting(img: &Image) -> Result<UniquePtr<Image>>;
        fn filter_binary_image_to_label_map(img: &Image, fully_connected: bool, input_foreground_value: f64, output_background_value: f64) -> Result<UniquePtr<Image>>;
        fn filter_label_image_to_label_map(img: &Image, background_value: f64) -> Result<UniquePtr<Image>>;
        fn filter_min_max_curvature_flow(img: &Image, time_step: f64, number_of_iterations: u32, stencil_radius: i32) -> Result<UniquePtr<Image>>;
        fn filter_fast_marching(img: &Image, normalization_factor: f64, stopping_value: f64) -> Result<UniquePtr<Image>>;
        fn filter_connected_threshold(img: &Image, lower: f64, upper: f64, replace_value: u8, connectivity: i32) -> Result<UniquePtr<Image>>;

        // ── Group 3: Unary with vector params ────────────────────────────
        fn filter_crop(img: &Image, lower_size: &[u32], upper_size: &[u32]) -> Result<UniquePtr<Image>>;
        fn filter_tile(img: &Image, layout: &[u32], default_pixel_value: f64) -> Result<UniquePtr<Image>>;
        fn filter_compose(img: &Image) -> Result<UniquePtr<Image>>;

        // ── Group 4: Two-image with params ───────────────────────────────
        fn filter_masked_assign(img: &Image, mask: &Image, assign_constant: f64) -> Result<UniquePtr<Image>>;
        fn filter_label_map_overlay(label_map: &Image, feature_image: &Image, opacity: f64) -> Result<UniquePtr<Image>>;
        fn filter_label_map_mask(label_map: &Image, feature_image: &Image, label: u64, background_value: f64, negated: bool, crop: bool) -> Result<UniquePtr<Image>>;
        fn filter_join_series_two(img1: &Image, img2: &Image, origin: f64, spacing: f64) -> Result<UniquePtr<Image>>;
        fn filter_isolated_connected(img: &Image, seed1: &[u32], seed2: &[u32], lower: f64, upper: f64, replace_value: u8, tolerance: f64, find_upper: bool) -> Result<UniquePtr<Image>>;
        fn filter_isolated_watershed(img: &Image, seed1: &[u32], seed2: &[u32], threshold: f64, upper_value_limit: f64, tolerance: f64, replace1: u8, replace2: u8) -> Result<UniquePtr<Image>>;

        // ── Group 5: Three-image ─────────────────────────────────────────
        fn filter_normalized_correlation(image: &Image, mask_image: &Image, template_image: &Image) -> Result<UniquePtr<Image>>;
        fn filter_label_map_contour_overlay(label_map: &Image, feature_image: &Image, opacity: f64, dilation_radius: &[u32], contour_thickness: &[u32], slice_dimension: u32, contour_type: i32, priority: i32) -> Result<UniquePtr<Image>>;
        fn filter_merge_label_map_two(img1: &Image, img2: &Image, method: i32) -> Result<UniquePtr<Image>>;

        // ── Group 6: Image sources (no Image input) ──────────────────────
        fn source_gaussian(pixel_type: i32, size: &[u32], sigma: &[f64], mean: &[f64], scale: f64) -> Result<UniquePtr<Image>>;
        fn source_gabor(pixel_type: i32, size: &[u32], sigma: &[f64], mean: &[f64], frequency: f64) -> Result<UniquePtr<Image>>;
        fn source_grid(pixel_type: i32, size: &[u32], sigma: &[f64], grid_spacing: &[f64], grid_offset: &[f64], scale: f64) -> Result<UniquePtr<Image>>;
        fn source_physical_point(pixel_type: i32, size: &[u32]) -> Result<UniquePtr<Image>>;
        // ── Final batch ──────────────────────────────────────────────────
        fn filter_aggregate_label_map(img: &Image) -> Result<UniquePtr<Image>>;
        fn filter_label_map_to_rgb(img: &Image) -> Result<UniquePtr<Image>>;
        fn filter_extract(img: &Image, size: &[u32], index: &[i32]) -> Result<UniquePtr<Image>>;
        fn filter_nary_add_two(img1: &Image, img2: &Image) -> Result<UniquePtr<Image>>;
        fn filter_nary_maximum_two(img1: &Image, img2: &Image) -> Result<UniquePtr<Image>>;

        // ── Remaining image filters ──────────────────────────────────────
        fn filter_change_label_label_map(img: &Image, change_map_pairs: &[f64]) -> Result<UniquePtr<Image>>;
        fn filter_colliding_fronts(img: &Image, seeds1: &[u32], seeds2: &[u32], dim: u32, apply_connectivity: bool, negative_epsilon: f64, stop_on_targets: bool) -> Result<UniquePtr<Image>>;
        fn filter_confidence_connected(img: &Image, seeds: &[u32], dim: u32, number_of_iterations: u32, multiplier: f64, initial_neighborhood_radius: u32, replace_value: u8) -> Result<UniquePtr<Image>>;
        fn filter_fast_marching_upwind_gradient(img: &Image, trial_points: &[u32], dim: u32, normalization_factor: f64, stopping_value: f64) -> Result<UniquePtr<Image>>;
        fn filter_inverse_displacement_field(img: &Image, size: &[u32], output_origin: &[f64], output_spacing: &[f64], subsampling_factor: u32) -> Result<UniquePtr<Image>>;
        fn filter_invert_displacement_field(img: &Image, max_iterations: u32, max_error_tolerance: f64, mean_error_tolerance: f64, enforce_boundary_condition: bool) -> Result<UniquePtr<Image>>;
        fn filter_iterative_inverse_displacement_field(img: &Image, number_of_iterations: u32, stop_value: f64) -> Result<UniquePtr<Image>>;
        fn filter_paste(dst: &Image, src: &Image, source_size: &[u32], source_index: &[i32], destination_index: &[i32]) -> Result<UniquePtr<Image>>;
        fn filter_patch_based_denoising(img: &Image, kernel_bandwidth_sigma: f64, patch_radius: u32, number_of_iterations: u32, number_of_sample_patches: u32, noise_model: i32, noise_sigma: f64) -> Result<UniquePtr<Image>>;
        fn filter_canny_segmentation_level_set(initial: &Image, feature: &Image, threshold: f64, variance: f64, max_rms_error: f64, propagation_scaling: f64, curvature_scaling: f64, advection_scaling: f64, number_of_iterations: u32, reverse_expansion_direction: bool) -> Result<UniquePtr<Image>>;
        fn filter_geodesic_active_contour_level_set(initial: &Image, feature: &Image, max_rms_error: f64, propagation_scaling: f64, curvature_scaling: f64, advection_scaling: f64, number_of_iterations: u32, reverse_expansion_direction: bool) -> Result<UniquePtr<Image>>;
        fn filter_laplacian_segmentation_level_set(initial: &Image, feature: &Image, max_rms_error: f64, propagation_scaling: f64, curvature_scaling: f64, number_of_iterations: u32, reverse_expansion_direction: bool) -> Result<UniquePtr<Image>>;
        fn filter_shape_detection_level_set(initial: &Image, feature: &Image, max_rms_error: f64, propagation_scaling: f64, curvature_scaling: f64, number_of_iterations: u32, reverse_expansion_direction: bool) -> Result<UniquePtr<Image>>;
        fn filter_threshold_segmentation_level_set(initial: &Image, feature: &Image, lower_threshold: f64, upper_threshold: f64, max_rms_error: f64, propagation_scaling: f64, curvature_scaling: f64, number_of_iterations: u32, reverse_expansion_direction: bool) -> Result<UniquePtr<Image>>;
        fn filter_scalar_chan_vese_level_set(initial: &Image, feature: &Image, max_rms_error: f64, number_of_iterations: u32, lambda1: f64, lambda2: f64, epsilon: f64, curvature_weight: f64, area_weight: f64) -> Result<UniquePtr<Image>>;
        fn filter_vector_confidence_connected(img: &Image, seeds: &[u32], dim: u32, number_of_iterations: u32, multiplier: f64, initial_neighborhood_radius: u32, replace_value: u8) -> Result<UniquePtr<Image>>;
        fn filter_warp_image(img: &Image, displacement_field: &Image, interpolator: i32, edge_padding_value: f64) -> Result<UniquePtr<Image>>;
        fn filter_transform_geometry(img: &Image, tx: &Transform) -> Result<UniquePtr<Image>>;

        // ── Measurement functions ────────────────────────────────────────
        fn measure_hash(img: &Image, hash_function: i32) -> Result<String>;
        fn measure_min_max(img: &Image) -> Vec<f64>;
        fn measure_statistics(img: &Image) -> Vec<f64>;
        fn measure_similarity_index(img1: &Image, img2: &Image) -> f64;
        fn measure_hausdorff_distance(img1: &Image, img2: &Image) -> Vec<f64>;
        fn measure_label_overlap(source: &Image, target: &Image) -> Vec<f64>;
        fn measure_label_stats_for_label(img: &Image, label_img: &Image, label: i64) -> Vec<f64>;
        fn measure_label_stats_labels(img: &Image, label_img: &Image) -> Vec<i64>;
        fn measure_label_shape_for_label(img: &Image, label: i64, background_value: f64) -> Vec<f64>;
        fn measure_label_shape_labels(img: &Image, background_value: f64) -> Vec<i64>;
        fn measure_label_intensity_for_label(img: &Image, feature_img: &Image, label: i64, background_value: f64) -> Vec<f64>;
        fn measure_label_intensity_labels(img: &Image, feature_img: &Image, background_value: f64) -> Vec<i64>;

        // ── Transforms ───────────────────────────────────────────────────
        type Transform;

        // Factory functions
        fn new_affine_transform(dimensions: u32) -> Result<UniquePtr<Transform>>;
        fn new_translation_transform(dimensions: u32) -> Result<UniquePtr<Transform>>;
        fn new_scale_transform(dimensions: u32) -> Result<UniquePtr<Transform>>;
        fn new_euler2d_transform() -> Result<UniquePtr<Transform>>;
        fn new_euler3d_transform() -> Result<UniquePtr<Transform>>;
        fn new_similarity2d_transform() -> Result<UniquePtr<Transform>>;
        fn new_similarity3d_transform() -> Result<UniquePtr<Transform>>;
        fn new_versor_transform() -> Result<UniquePtr<Transform>>;
        fn new_versor_rigid3d_transform() -> Result<UniquePtr<Transform>>;
        fn new_scale_versor3d_transform() -> Result<UniquePtr<Transform>>;
        fn new_scale_skew_versor3d_transform() -> Result<UniquePtr<Transform>>;
        fn new_composite_transform(dimensions: u32) -> Result<UniquePtr<Transform>>;
        fn new_displacement_field_transform(dimensions: u32) -> Result<UniquePtr<Transform>>;
        fn read_transform(path: &str) -> Result<UniquePtr<Transform>>;

        // Base Transform methods (immutable)
        fn transform_get_parameters(t: &Transform) -> Vec<f64>;
        fn transform_get_fixed_parameters(t: &Transform) -> Vec<f64>;
        fn transform_get_number_of_parameters(t: &Transform) -> u64;
        fn transform_get_number_of_fixed_parameters(t: &Transform) -> u64;
        fn transform_transform_point(t: &Transform, point: &[f64]) -> Vec<f64>;
        fn transform_is_linear(t: &Transform) -> bool;
        fn transform_get_dimension(t: &Transform) -> u32;
        fn transform_get_name(t: &Transform) -> String;
        fn transform_write(t: &Transform, path: &str) -> Result<()>;
        fn transform_get_inverse(t: &Transform) -> Result<UniquePtr<Transform>>;

        // Base Transform methods (mutable)
        fn transform_set_parameters(t: Pin<&mut Transform>, params: &[f64]);
        fn transform_set_fixed_parameters(t: Pin<&mut Transform>, params: &[f64]);
        fn transform_set_identity(t: Pin<&mut Transform>);

        // AffineTransform
        fn affine_get_matrix(t: &Transform) -> Vec<f64>;
        fn affine_get_center(t: &Transform) -> Vec<f64>;
        fn affine_get_translation(t: &Transform) -> Vec<f64>;
        fn affine_set_matrix(t: Pin<&mut Transform>, matrix: &[f64]);
        fn affine_set_center(t: Pin<&mut Transform>, center: &[f64]);
        fn affine_set_translation(t: Pin<&mut Transform>, translation: &[f64]);

        // TranslationTransform
        fn translation_get_offset(t: &Transform) -> Vec<f64>;
        fn translation_set_offset(t: Pin<&mut Transform>, offset: &[f64]);

        // ScaleTransform
        fn scale_get_scale(t: &Transform) -> Vec<f64>;
        fn scale_get_center(t: &Transform) -> Vec<f64>;
        fn scale_set_scale(t: Pin<&mut Transform>, scale: &[f64]);
        fn scale_set_center(t: Pin<&mut Transform>, center: &[f64]);

        // Euler2DTransform
        fn euler2d_get_center(t: &Transform) -> Vec<f64>;
        fn euler2d_get_angle(t: &Transform) -> f64;
        fn euler2d_get_translation(t: &Transform) -> Vec<f64>;
        fn euler2d_get_matrix(t: &Transform) -> Vec<f64>;
        fn euler2d_set_center(t: Pin<&mut Transform>, center: &[f64]);
        fn euler2d_set_angle(t: Pin<&mut Transform>, angle: f64);
        fn euler2d_set_translation(t: Pin<&mut Transform>, translation: &[f64]);
        fn euler2d_set_matrix(t: Pin<&mut Transform>, matrix: &[f64]);

        // Euler3DTransform
        fn euler3d_get_center(t: &Transform) -> Vec<f64>;
        fn euler3d_get_angle_x(t: &Transform) -> f64;
        fn euler3d_get_angle_y(t: &Transform) -> f64;
        fn euler3d_get_angle_z(t: &Transform) -> f64;
        fn euler3d_get_translation(t: &Transform) -> Vec<f64>;
        fn euler3d_get_matrix(t: &Transform) -> Vec<f64>;
        fn euler3d_set_center(t: Pin<&mut Transform>, center: &[f64]);
        fn euler3d_set_rotation(t: Pin<&mut Transform>, angle_x: f64, angle_y: f64, angle_z: f64);
        fn euler3d_set_translation(t: Pin<&mut Transform>, translation: &[f64]);
        fn euler3d_set_matrix(t: Pin<&mut Transform>, matrix: &[f64]);
        fn euler3d_set_compute_zyx(t: Pin<&mut Transform>, compute_zyx: bool);

        // Similarity2DTransform
        fn similarity2d_get_center(t: &Transform) -> Vec<f64>;
        fn similarity2d_get_angle(t: &Transform) -> f64;
        fn similarity2d_get_scale(t: &Transform) -> f64;
        fn similarity2d_get_translation(t: &Transform) -> Vec<f64>;
        fn similarity2d_get_matrix(t: &Transform) -> Vec<f64>;
        fn similarity2d_set_center(t: Pin<&mut Transform>, center: &[f64]);
        fn similarity2d_set_angle(t: Pin<&mut Transform>, angle: f64);
        fn similarity2d_set_scale(t: Pin<&mut Transform>, scale: f64);
        fn similarity2d_set_translation(t: Pin<&mut Transform>, translation: &[f64]);
        fn similarity2d_set_matrix(t: Pin<&mut Transform>, matrix: &[f64]);

        // Similarity3DTransform
        fn similarity3d_get_center(t: &Transform) -> Vec<f64>;
        fn similarity3d_get_versor(t: &Transform) -> Vec<f64>;
        fn similarity3d_get_scale(t: &Transform) -> f64;
        fn similarity3d_get_translation(t: &Transform) -> Vec<f64>;
        fn similarity3d_get_matrix(t: &Transform) -> Vec<f64>;
        fn similarity3d_set_center(t: Pin<&mut Transform>, center: &[f64]);
        fn similarity3d_set_rotation_versor(t: Pin<&mut Transform>, versor: &[f64]);
        fn similarity3d_set_rotation_axis_angle(t: Pin<&mut Transform>, axis: &[f64], angle: f64);
        fn similarity3d_set_scale(t: Pin<&mut Transform>, scale: f64);
        fn similarity3d_set_translation(t: Pin<&mut Transform>, translation: &[f64]);
        fn similarity3d_set_matrix(t: Pin<&mut Transform>, matrix: &[f64]);

        // VersorTransform
        fn versor_get_center(t: &Transform) -> Vec<f64>;
        fn versor_get_versor(t: &Transform) -> Vec<f64>;
        fn versor_get_matrix(t: &Transform) -> Vec<f64>;
        fn versor_set_center(t: Pin<&mut Transform>, center: &[f64]);
        fn versor_set_rotation_versor(t: Pin<&mut Transform>, versor: &[f64]);
        fn versor_set_rotation_axis_angle(t: Pin<&mut Transform>, axis: &[f64], angle: f64);
        fn versor_set_matrix(t: Pin<&mut Transform>, matrix: &[f64]);

        // VersorRigid3DTransform
        fn versor_rigid3d_get_translation(t: &Transform) -> Vec<f64>;
        fn versor_rigid3d_set_translation(t: Pin<&mut Transform>, translation: &[f64]);

        // ScaleVersor3DTransform
        fn scale_versor3d_get_scale(t: &Transform) -> Vec<f64>;
        fn scale_versor3d_set_scale(t: Pin<&mut Transform>, scale: &[f64]);

        // ScaleSkewVersor3DTransform
        fn scale_skew_versor3d_get_skew(t: &Transform) -> Vec<f64>;
        fn scale_skew_versor3d_set_skew(t: Pin<&mut Transform>, skew: &[f64]);

        // CompositeTransform
        fn composite_add_transform(t: Pin<&mut Transform>, to_add: &Transform) -> Result<()>;
        fn composite_get_number_of_transforms(t: &Transform) -> Result<u32>;
        fn composite_clear_transforms(t: Pin<&mut Transform>) -> Result<()>;
        fn composite_flatten_transform(t: Pin<&mut Transform>) -> Result<()>;

        // DisplacementFieldTransform
        fn displacement_field_get_displacement_field(t: &Transform) -> Result<UniquePtr<Image>>;
        fn displacement_field_set_displacement_field(t: Pin<&mut Transform>, field: &Image) -> Result<()>;
        fn displacement_field_set_smoothing_off(t: Pin<&mut Transform>) -> Result<()>;
        fn displacement_field_set_interpolator(t: Pin<&mut Transform>, interpolator: i32) -> Result<()>;

        // Resample filters
        fn filter_resample(img: &Image, tx: &Transform, interpolator: i32, default_pixel_value: f64, output_pixel_type: i32) -> Result<UniquePtr<Image>>;
        fn filter_resample_to_ref(img: &Image, ref_img: &Image, tx: &Transform, interpolator: i32, default_pixel_value: f64, output_pixel_type: i32) -> Result<UniquePtr<Image>>;
        fn filter_transform_to_displacement_field(tx: &Transform, output_pixel_type: i32, size: &[u32], origin: &[f64], spacing: &[f64]) -> Result<UniquePtr<Image>>;
    }
}

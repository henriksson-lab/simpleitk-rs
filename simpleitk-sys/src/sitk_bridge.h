#pragma once
#include "rust/cxx.h"
#include <SimpleITK.h>
#include <memory>

namespace sitk_rs {

using Image = itk::simple::Image;

std::unique_ptr<Image> read_image(rust::Str path);
void write_image(const Image& image, rust::Str path);

uint32_t get_width(const Image& img);
uint32_t get_height(const Image& img);
uint32_t get_depth(const Image& img);
uint32_t get_dimension(const Image& img);
uint64_t get_number_of_pixels(const Image& img);
int32_t  get_pixel_id_value(const Image& img);

// ── Unary filters ──────────────────────────────────────────────────────────
std::unique_ptr<Image> filter_abs(const Image& img);
std::unique_ptr<Image> filter_acos(const Image& img);
std::unique_ptr<Image> filter_adaptive_histogram_equalization(const Image& img, rust::Slice<const uint32_t> radius, float alpha, float beta);
std::unique_ptr<Image> filter_additive_gaussian_noise(const Image& img, double standard_deviation, double mean, uint32_t seed);
std::unique_ptr<Image> filter_anti_alias_binary(const Image& img, double maximum_rms_error, uint32_t number_of_iterations);
std::unique_ptr<Image> filter_approximate_signed_distance_map(const Image& img, double inside_value, double outside_value);
std::unique_ptr<Image> filter_area_closing(const Image& img, double lambda, bool use_image_spacing, bool fully_connected);
std::unique_ptr<Image> filter_area_opening(const Image& img, double lambda, bool use_image_spacing, bool fully_connected);
std::unique_ptr<Image> filter_asin(const Image& img);
std::unique_ptr<Image> filter_atan(const Image& img);
std::unique_ptr<Image> filter_bilateral(const Image& img, double domain_sigma, double range_sigma, uint32_t number_of_range_gaussian_samples);
std::unique_ptr<Image> filter_bin_shrink(const Image& img, rust::Slice<const uint32_t> shrink_factors);
std::unique_ptr<Image> filter_binary_closing_by_reconstruction(const Image& img, rust::Slice<const uint32_t> kernel_radius, int32_t kernel_type, double foreground_value, bool fully_connected);
std::unique_ptr<Image> filter_binary_contour(const Image& img, bool fully_connected, double background_value, double foreground_value);
std::unique_ptr<Image> filter_binary_dilate(const Image& img, rust::Slice<const uint32_t> kernel_radius, int32_t kernel_type, double background_value, double foreground_value, bool boundary_to_foreground);
std::unique_ptr<Image> filter_binary_erode(const Image& img, rust::Slice<const uint32_t> kernel_radius, int32_t kernel_type, double background_value, double foreground_value, bool boundary_to_foreground);
std::unique_ptr<Image> filter_binary_fillhole(const Image& img, bool fully_connected, double foreground_value);
std::unique_ptr<Image> filter_binary_grind_peak(const Image& img, bool fully_connected, double foreground_value, double background_value);
std::unique_ptr<Image> filter_binary_median(const Image& img, rust::Slice<const uint32_t> radius, double foreground_value, double background_value);
std::unique_ptr<Image> filter_binary_min_max_curvature_flow(const Image& img, double time_step, uint32_t number_of_iterations, int32_t stencil_radius, double threshold);
std::unique_ptr<Image> filter_binary_morphological_closing(const Image& img, rust::Slice<const uint32_t> kernel_radius, int32_t kernel_type, double foreground_value, bool safe_edge_handling);
std::unique_ptr<Image> filter_binary_morphological_opening(const Image& img, rust::Slice<const uint32_t> kernel_radius, int32_t kernel_type, double background_value, double foreground_value);
std::unique_ptr<Image> filter_binary_not(const Image& img, double foreground_value, double background_value);
std::unique_ptr<Image> filter_binary_opening_by_reconstruction(const Image& img, rust::Slice<const uint32_t> kernel_radius, int32_t kernel_type, double foreground_value, bool fully_connected);
std::unique_ptr<Image> filter_binary_projection(const Image& img, uint32_t projection_dimension, double foreground_value, double background_value);
std::unique_ptr<Image> filter_binary_pruning(const Image& img, uint32_t iteration);
std::unique_ptr<Image> filter_binary_thinning(const Image& img);
std::unique_ptr<Image> filter_binary_threshold(const Image& img, double lower_threshold, double upper_threshold, uint8_t inside_value, uint8_t outside_value);
std::unique_ptr<Image> filter_binary_threshold_projection(const Image& img, uint32_t projection_dimension, double threshold_value, uint8_t foreground_value, uint8_t background_value);
std::unique_ptr<Image> filter_binomial_blur(const Image& img, uint32_t repetitions);
std::unique_ptr<Image> filter_bitwise_not(const Image& img);
std::unique_ptr<Image> filter_black_top_hat(const Image& img, rust::Slice<const uint32_t> kernel_radius, int32_t kernel_type, bool safe_border);
std::unique_ptr<Image> filter_bounded_reciprocal(const Image& img);
std::unique_ptr<Image> filter_box_mean(const Image& img, rust::Slice<const uint32_t> radius);
std::unique_ptr<Image> filter_box_sigma(const Image& img, rust::Slice<const uint32_t> radius);
std::unique_ptr<Image> filter_canny_edge_detection(const Image& img, double lower_threshold, double upper_threshold, double variance, double maximum_error);
std::unique_ptr<Image> filter_cast(const Image& img, int32_t pixel_id);
std::unique_ptr<Image> filter_clamp(const Image& img, double lower_bound, double upper_bound);
std::unique_ptr<Image> filter_closing_by_reconstruction(const Image& img, rust::Slice<const uint32_t> kernel_radius, int32_t kernel_type, bool fully_connected, bool preserve_intensities);
std::unique_ptr<Image> filter_complex_to_imaginary(const Image& img);
std::unique_ptr<Image> filter_complex_to_modulus(const Image& img);
std::unique_ptr<Image> filter_complex_to_phase(const Image& img);
std::unique_ptr<Image> filter_complex_to_real(const Image& img);
std::unique_ptr<Image> filter_connected_component(const Image& img, bool fully_connected);
std::unique_ptr<Image> filter_cos(const Image& img);
std::unique_ptr<Image> filter_curvature_anisotropic_diffusion(const Image& img, double time_step, double conductance_parameter, uint32_t conductance_scaling_update_interval, uint32_t number_of_iterations);
std::unique_ptr<Image> filter_curvature_flow(const Image& img, double time_step, uint32_t number_of_iterations);
std::unique_ptr<Image> filter_cyclic_shift(const Image& img, rust::Slice<const int32_t> shift);
std::unique_ptr<Image> filter_danielsson_distance_map(const Image& img, bool input_is_binary, bool squared_distance, bool use_image_spacing);
std::unique_ptr<Image> filter_dilate_object_morphology(const Image& img, rust::Slice<const uint32_t> kernel_radius, int32_t kernel_type, double object_value, double background_value);
std::unique_ptr<Image> filter_discrete_gaussian(const Image& img, double variance, uint32_t maximum_kernel_width, double maximum_error, bool use_image_spacing);
std::unique_ptr<Image> filter_discrete_gaussian_derivative(const Image& img, double variance, uint32_t order, uint32_t maximum_kernel_width, double maximum_error, bool normalize_across_scale);
std::unique_ptr<Image> filter_displacement_field_jacobian_determinant(const Image& img, bool use_image_spacing);
std::unique_ptr<Image> filter_double_threshold(const Image& img, double threshold1, double threshold2, double threshold3, double threshold4, uint8_t inside_value, uint8_t outside_value, bool fully_connected);
std::unique_ptr<Image> filter_edge_potential(const Image& img);
std::unique_ptr<Image> filter_erode_object_morphology(const Image& img, rust::Slice<const uint32_t> kernel_radius, int32_t kernel_type, double object_value, double background_value);
std::unique_ptr<Image> filter_exp(const Image& img);
std::unique_ptr<Image> filter_exp_negative(const Image& img);
std::unique_ptr<Image> filter_fast_approximate_rank(const Image& img, rust::Slice<const uint32_t> radius, double rank);
std::unique_ptr<Image> filter_fft_shift(const Image& img, bool inverse);
std::unique_ptr<Image> filter_flip_image(const Image& img, rust::Slice<const bool> flip_axes, bool flip_about_origin);
std::unique_ptr<Image> filter_gradient_anisotropic_diffusion(const Image& img, double time_step, double conductance_parameter, uint32_t conductance_scaling_update_interval, uint32_t number_of_iterations);
std::unique_ptr<Image> filter_gradient_magnitude(const Image& img);
std::unique_ptr<Image> filter_gradient_magnitude_recursive_gaussian(const Image& img, double sigma, bool normalize_across_scale);
std::unique_ptr<Image> filter_grayscale_dilate(const Image& img, rust::Slice<const uint32_t> kernel_radius, int32_t kernel_type);
std::unique_ptr<Image> filter_grayscale_erode(const Image& img, rust::Slice<const uint32_t> kernel_radius, int32_t kernel_type);
std::unique_ptr<Image> filter_grayscale_fillhole(const Image& img, bool fully_connected);
std::unique_ptr<Image> filter_grayscale_grind_peak(const Image& img, bool fully_connected);
std::unique_ptr<Image> filter_grayscale_morphological_closing(const Image& img, rust::Slice<const uint32_t> kernel_radius, int32_t kernel_type, bool safe_border);
std::unique_ptr<Image> filter_grayscale_morphological_opening(const Image& img, rust::Slice<const uint32_t> kernel_radius, int32_t kernel_type, bool safe_border);
std::unique_ptr<Image> filter_h_concave(const Image& img, double height, bool fully_connected);
std::unique_ptr<Image> filter_h_convex(const Image& img, double height, bool fully_connected);
std::unique_ptr<Image> filter_h_maxima(const Image& img, double height, bool fully_connected);
std::unique_ptr<Image> filter_h_minima(const Image& img, double height, bool fully_connected);
std::unique_ptr<Image> filter_huang_threshold(const Image& img, uint8_t inside_value, uint8_t outside_value, uint32_t number_of_histogram_bins);
std::unique_ptr<Image> filter_intensity_windowing(const Image& img, double window_minimum, double window_maximum, double output_minimum, double output_maximum);
std::unique_ptr<Image> filter_intermodes_threshold(const Image& img, uint8_t inside_value, uint8_t outside_value, uint32_t number_of_histogram_bins);
std::unique_ptr<Image> filter_invert_intensity(const Image& img, double maximum);
std::unique_ptr<Image> filter_iso_contour_distance(const Image& img, double level_set_value, double far_value);
std::unique_ptr<Image> filter_iso_data_threshold(const Image& img, uint8_t inside_value, uint8_t outside_value, uint32_t number_of_histogram_bins);
std::unique_ptr<Image> filter_kittler_illingworth_threshold(const Image& img, uint8_t inside_value, uint8_t outside_value, uint32_t number_of_histogram_bins);
std::unique_ptr<Image> filter_label_contour(const Image& img, bool fully_connected, double background_value);
std::unique_ptr<Image> filter_laplacian_recursive_gaussian(const Image& img, double sigma, bool normalize_across_scale);
std::unique_ptr<Image> filter_laplacian_sharpening(const Image& img, bool use_image_spacing);
std::unique_ptr<Image> filter_li_threshold(const Image& img, uint8_t inside_value, uint8_t outside_value, uint32_t number_of_histogram_bins);
std::unique_ptr<Image> filter_log(const Image& img);
std::unique_ptr<Image> filter_log10(const Image& img);
std::unique_ptr<Image> filter_maximum_entropy_threshold(const Image& img, uint8_t inside_value, uint8_t outside_value, uint32_t number_of_histogram_bins);
std::unique_ptr<Image> filter_maximum_projection(const Image& img, uint32_t projection_dimension);
std::unique_ptr<Image> filter_mean(const Image& img, rust::Slice<const uint32_t> radius);
std::unique_ptr<Image> filter_mean_projection(const Image& img, uint32_t projection_dimension);
std::unique_ptr<Image> filter_median(const Image& img, rust::Slice<const uint32_t> radius);
std::unique_ptr<Image> filter_median_projection(const Image& img, uint32_t projection_dimension);
std::unique_ptr<Image> filter_minimum_projection(const Image& img, uint32_t projection_dimension);
std::unique_ptr<Image> filter_mirror_pad(const Image& img, rust::Slice<const uint32_t> pad_lower_bound, rust::Slice<const uint32_t> pad_upper_bound);
std::unique_ptr<Image> filter_moments_threshold(const Image& img, uint8_t inside_value, uint8_t outside_value, uint32_t number_of_histogram_bins);
std::unique_ptr<Image> filter_morphological_gradient(const Image& img, rust::Slice<const uint32_t> kernel_radius, int32_t kernel_type);
std::unique_ptr<Image> filter_morphological_watershed(const Image& img, double level, bool mark_watershed_line, bool fully_connected);
std::unique_ptr<Image> filter_noise_image_filter(const Image& img);
std::unique_ptr<Image> filter_normalize(const Image& img);
std::unique_ptr<Image> filter_normalize_to_constant(const Image& img, double constant);
std::unique_ptr<Image> filter_not(const Image& img);
std::unique_ptr<Image> filter_opening_by_reconstruction(const Image& img, rust::Slice<const uint32_t> kernel_radius, int32_t kernel_type, bool fully_connected, bool preserve_intensities);
std::unique_ptr<Image> filter_otsu_multiple_thresholds(const Image& img, uint32_t number_of_thresholds, uint8_t label_offset, uint32_t number_of_histogram_bins, bool valley_emphasis);
std::unique_ptr<Image> filter_otsu_threshold(const Image& img, uint8_t inside_value, uint8_t outside_value, uint32_t number_of_histogram_bins);
std::unique_ptr<Image> filter_rank(const Image& img, rust::Slice<const uint32_t> radius, double rank);
std::unique_ptr<Image> filter_recursive_gaussian(const Image& img, double sigma, bool normalize_across_scale, uint32_t order, uint32_t direction);
std::unique_ptr<Image> filter_regional_maxima(const Image& img, double background_value, bool fully_connected, bool flat_is_maxima);
std::unique_ptr<Image> filter_regional_minima(const Image& img, double background_value, bool fully_connected, bool flat_is_minima);
std::unique_ptr<Image> filter_relabel_component(const Image& img, uint64_t minimum_object_size, bool sort_by_object_size);
std::unique_ptr<Image> filter_renyi_entropy_threshold(const Image& img, uint8_t inside_value, uint8_t outside_value, uint32_t number_of_histogram_bins);
std::unique_ptr<Image> filter_rescale_intensity(const Image& img, double output_minimum, double output_maximum);
std::unique_ptr<Image> filter_round(const Image& img);
std::unique_ptr<Image> filter_salt_and_pepper_noise(const Image& img, double probability, uint32_t seed);
std::unique_ptr<Image> filter_shanbhag_threshold(const Image& img, uint8_t inside_value, uint8_t outside_value, uint32_t number_of_histogram_bins);
std::unique_ptr<Image> filter_shift_scale(const Image& img, double shift, double scale);
std::unique_ptr<Image> filter_shot_noise(const Image& img, double scale, uint32_t seed);
std::unique_ptr<Image> filter_shrink(const Image& img, rust::Slice<const uint32_t> shrink_factors);
std::unique_ptr<Image> filter_sigmoid(const Image& img, double alpha, double beta, double output_maximum, double output_minimum);
std::unique_ptr<Image> filter_signed_danielsson_distance_map(const Image& img, bool inside_is_positive, bool squared_distance, bool use_image_spacing);
std::unique_ptr<Image> filter_signed_maurer_distance_map(const Image& img, bool inside_is_positive, bool squared_distance, bool use_image_spacing, double background_value);
std::unique_ptr<Image> filter_sin(const Image& img);
std::unique_ptr<Image> filter_smoothing_recursive_gaussian(const Image& img, rust::Slice<const double> sigma, bool normalize_across_scale);
std::unique_ptr<Image> filter_speckle_noise(const Image& img, double standard_deviation, uint32_t seed);
std::unique_ptr<Image> filter_sqrt(const Image& img);
std::unique_ptr<Image> filter_square(const Image& img);
std::unique_ptr<Image> filter_standard_deviation_projection(const Image& img, uint32_t projection_dimension);
std::unique_ptr<Image> filter_sum_projection(const Image& img, uint32_t projection_dimension);
std::unique_ptr<Image> filter_tan(const Image& img);
std::unique_ptr<Image> filter_threshold(const Image& img, double lower_threshold, double upper_threshold, double outside_value);
std::unique_ptr<Image> filter_triangle_threshold(const Image& img, uint8_t inside_value, uint8_t outside_value, uint32_t number_of_histogram_bins);
std::unique_ptr<Image> filter_unary_minus(const Image& img);
std::unique_ptr<Image> filter_unsharp_mask(const Image& img, rust::Slice<const double> sigmas, double amount, double threshold);
std::unique_ptr<Image> filter_valued_regional_maxima(const Image& img, bool fully_connected);
std::unique_ptr<Image> filter_valued_regional_minima(const Image& img, bool fully_connected);
std::unique_ptr<Image> filter_vector_index_selection_cast(const Image& img, uint32_t index, int32_t output_pixel_type);
std::unique_ptr<Image> filter_vector_magnitude(const Image& img);
std::unique_ptr<Image> filter_voting_binary(const Image& img, rust::Slice<const uint32_t> radius, uint32_t birth_threshold, uint32_t survival_threshold, double foreground_value, double background_value);
std::unique_ptr<Image> filter_voting_binary_hole_filling(const Image& img, rust::Slice<const uint32_t> radius, uint32_t majority_threshold, double foreground_value, double background_value, uint32_t maximum_number_of_iterations);
std::unique_ptr<Image> filter_voting_binary_iterative_hole_filling(const Image& img, rust::Slice<const uint32_t> radius, uint32_t majority_threshold, double foreground_value, double background_value, uint32_t maximum_number_of_iterations);
std::unique_ptr<Image> filter_white_top_hat(const Image& img, rust::Slice<const uint32_t> kernel_radius, int32_t kernel_type, bool safe_border);
std::unique_ptr<Image> filter_yen_threshold(const Image& img, uint8_t inside_value, uint8_t outside_value, uint32_t number_of_histogram_bins);
std::unique_ptr<Image> filter_zero_crossing(const Image& img, uint8_t foreground_value, uint8_t background_value);
std::unique_ptr<Image> filter_zero_crossing_based_edge_detection(const Image& img, double variance, double maximum_error);
std::unique_ptr<Image> filter_zero_flux_neumann_pad(const Image& img, rust::Slice<const uint32_t> pad_lower_bound, rust::Slice<const uint32_t> pad_upper_bound);
std::unique_ptr<Image> filter_constant_pad(const Image& img, rust::Slice<const uint32_t> pad_lower_bound, rust::Slice<const uint32_t> pad_upper_bound, double constant);
std::unique_ptr<Image> filter_wrap_pad(const Image& img, rust::Slice<const uint32_t> pad_lower_bound, rust::Slice<const uint32_t> pad_upper_bound);

// ── Binary filters ─────────────────────────────────────────────────────────
std::unique_ptr<Image> filter_absolute_value_difference(const Image& img1, const Image& img2);
std::unique_ptr<Image> filter_add(const Image& img1, const Image& img2);
std::unique_ptr<Image> filter_and(const Image& img1, const Image& img2);
std::unique_ptr<Image> filter_atan2(const Image& img1, const Image& img2);
std::unique_ptr<Image> filter_binary_magnitude(const Image& img1, const Image& img2);
std::unique_ptr<Image> filter_binary_reconstruction_by_dilation(const Image& img1, const Image& img2, double background_value, double foreground_value, bool fully_connected);
std::unique_ptr<Image> filter_binary_reconstruction_by_erosion(const Image& img1, const Image& img2, double background_value, double foreground_value, bool fully_connected);
std::unique_ptr<Image> filter_checker_board(const Image& img1, const Image& img2, rust::Slice<const uint32_t> checker_pattern);
std::unique_ptr<Image> filter_divide(const Image& img1, const Image& img2);
std::unique_ptr<Image> filter_divide_floor(const Image& img1, const Image& img2);
std::unique_ptr<Image> filter_divide_real(const Image& img1, const Image& img2);
std::unique_ptr<Image> filter_equal(const Image& img1, const Image& img2);
std::unique_ptr<Image> filter_grayscale_geodesic_dilate(const Image& marker, const Image& mask, bool run_one_iteration, bool fully_connected);
std::unique_ptr<Image> filter_grayscale_geodesic_erode(const Image& marker, const Image& mask, bool run_one_iteration, bool fully_connected);
std::unique_ptr<Image> filter_greater(const Image& img1, const Image& img2);
std::unique_ptr<Image> filter_greater_equal(const Image& img1, const Image& img2);
std::unique_ptr<Image> filter_histogram_matching(const Image& img, const Image& reference, uint32_t number_of_histogram_levels, uint32_t number_of_match_points, bool threshold_at_mean_intensity);
std::unique_ptr<Image> filter_less(const Image& img1, const Image& img2);
std::unique_ptr<Image> filter_less_equal(const Image& img1, const Image& img2);
std::unique_ptr<Image> filter_magnitude_and_phase_to_complex(const Image& img1, const Image& img2);
std::unique_ptr<Image> filter_mask_image(const Image& img, const Image& mask, double outside_value, double masking_value);
std::unique_ptr<Image> filter_mask_negated_image(const Image& img, const Image& mask, double outside_value, double masking_value);
std::unique_ptr<Image> filter_morphological_watershed_from_markers(const Image& img, const Image& marker, bool mark_watershed_line, bool fully_connected);
std::unique_ptr<Image> filter_multiply(const Image& img1, const Image& img2);
std::unique_ptr<Image> filter_not_equal(const Image& img1, const Image& img2);
std::unique_ptr<Image> filter_or(const Image& img1, const Image& img2);
std::unique_ptr<Image> filter_pow(const Image& img1, const Image& img2);
std::unique_ptr<Image> filter_real_and_imaginary_to_complex(const Image& img1, const Image& img2);
std::unique_ptr<Image> filter_reconstruction_by_dilation(const Image& img1, const Image& img2, bool fully_connected);
std::unique_ptr<Image> filter_reconstruction_by_erosion(const Image& img1, const Image& img2, bool fully_connected);
std::unique_ptr<Image> filter_squared_difference(const Image& img1, const Image& img2);
std::unique_ptr<Image> filter_subtract(const Image& img1, const Image& img2);
std::unique_ptr<Image> filter_xor(const Image& img1, const Image& img2);

// ── New filters ────────────────────────────────────────────────────────────

// Unary (no params)
std::unique_ptr<Image> filter_sobel_edge_detection(const Image& img);
std::unique_ptr<Image> filter_forward_fft(const Image& img);
std::unique_ptr<Image> filter_inverse_fft(const Image& img);
std::unique_ptr<Image> filter_real_to_half_hermitian_forward_fft(const Image& img);
std::unique_ptr<Image> filter_toboggan(const Image& img);
std::unique_ptr<Image> filter_label_map_to_label(const Image& img);

// Unary with simple params
std::unique_ptr<Image> filter_derivative(const Image& img, uint32_t direction, uint32_t order, bool use_image_spacing);
std::unique_ptr<Image> filter_half_hermitian_to_real_inverse_fft(const Image& img, bool actual_x_dimension_is_odd);
std::unique_ptr<Image> filter_fft_pad(const Image& img, int32_t boundary_condition, int32_t size_greatest_prime_factor);
std::unique_ptr<Image> filter_vector_connected_component(const Image& img, double distance_threshold, bool fully_connected);
std::unique_ptr<Image> filter_label_map_to_binary(const Image& img, double background_value, double foreground_value);
std::unique_ptr<Image> filter_objectness_measure(const Image& img, double alpha, double beta, double gamma, bool scale_objectness_measure, uint32_t object_dimension, bool bright_object);
std::unique_ptr<Image> filter_threshold_maximum_connected_components(const Image& img, uint32_t minimum_object_size_in_pixels, double upper_boundary, uint8_t inside_value, uint8_t outside_value);
std::unique_ptr<Image> filter_gradient(const Image& img, bool use_image_spacing, bool use_image_direction);
std::unique_ptr<Image> filter_gradient_recursive_gaussian(const Image& img, double sigma, bool normalize_across_scale, bool use_image_direction);
std::unique_ptr<Image> filter_dicom_orient(const Image& img, rust::Str orientation);
std::unique_ptr<Image> filter_scalar_connected_component(const Image& img, double distance_threshold, bool fully_connected);
std::unique_ptr<Image> filter_scalar_to_rgb_colormap(const Image& img, int32_t colormap, bool use_input_image_extrema_for_scaling);
std::unique_ptr<Image> filter_staple(const Image& img, double confidence_weight, double foreground_value, uint32_t maximum_iterations);
std::unique_ptr<Image> filter_multi_label_staple(const Image& img, float termination_threshold, uint32_t max_iterations);

// Unary with vector params
std::unique_ptr<Image> filter_grayscale_connected_closing(const Image& img, rust::Slice<const uint32_t> seed, bool fully_connected);
std::unique_ptr<Image> filter_grayscale_connected_opening(const Image& img, rust::Slice<const uint32_t> seed, bool fully_connected);
std::unique_ptr<Image> filter_permute_axes(const Image& img, rust::Slice<const uint32_t> order);
std::unique_ptr<Image> filter_slic(const Image& img, rust::Slice<const uint32_t> super_grid_size, double spatial_proximity_weight, uint32_t maximum_number_of_iterations, bool enforce_connectivity, bool initialization_perturbation);
std::unique_ptr<Image> filter_region_of_interest(const Image& img, rust::Slice<const uint32_t> size, rust::Slice<const int32_t> index);
std::unique_ptr<Image> filter_simple_contour_extractor(const Image& img, double input_foreground_value, double input_background_value, rust::Slice<const uint32_t> radius, double output_foreground_value, double output_background_value);
std::unique_ptr<Image> filter_slice(const Image& img, rust::Slice<const int32_t> start, rust::Slice<const int32_t> stop, rust::Slice<const int32_t> step);
std::unique_ptr<Image> filter_expand(const Image& img, rust::Slice<const uint32_t> expand_factors, int32_t interpolator);
std::unique_ptr<Image> filter_stochastic_fractal_dimension(const Image& img, rust::Slice<const uint32_t> neighborhood_radius);
std::unique_ptr<Image> filter_scalar_image_kmeans(const Image& img, rust::Slice<const double> class_means, bool use_non_contiguous_labels);
std::unique_ptr<Image> filter_n4_bias_field_correction(const Image& img, double convergence_threshold, rust::Slice<const uint32_t> max_iterations, uint32_t num_histogram_bins, rust::Slice<const uint32_t> num_control_points, uint32_t spline_order);

// Binary filters (two-image)
std::unique_ptr<Image> filter_convolution(const Image& img, const Image& kernel, bool normalize, int32_t boundary_condition, int32_t output_region_mode);
std::unique_ptr<Image> filter_fft_convolution(const Image& img, const Image& kernel, bool normalize, int32_t boundary_condition, int32_t output_region_mode);
std::unique_ptr<Image> filter_label_overlay(const Image& img, const Image& label, double opacity, double background_value);
std::unique_ptr<Image> filter_inverse_deconvolution(const Image& img, const Image& kernel, double kernel_zero_magnitude_threshold, bool normalize, int32_t boundary_condition, int32_t output_region_mode);
std::unique_ptr<Image> filter_tikhonov_deconvolution(const Image& img, const Image& kernel, double regularization_constant, bool normalize, int32_t boundary_condition, int32_t output_region_mode);
std::unique_ptr<Image> filter_wiener_deconvolution(const Image& img, const Image& kernel, double noise_variance, bool normalize, int32_t boundary_condition, int32_t output_region_mode);
std::unique_ptr<Image> filter_richardson_lucy(const Image& img, const Image& kernel, int32_t number_of_iterations, bool normalize, int32_t boundary_condition, int32_t output_region_mode);
std::unique_ptr<Image> filter_landweber_deconvolution(const Image& img, const Image& kernel, double alpha, int32_t number_of_iterations, bool normalize, int32_t boundary_condition, int32_t output_region_mode);
std::unique_ptr<Image> filter_projected_landweber_deconvolution(const Image& img, const Image& kernel, double alpha, int32_t number_of_iterations, bool normalize, int32_t boundary_condition, int32_t output_region_mode);
std::unique_ptr<Image> filter_fft_normalized_correlation(const Image& fixed_image, const Image& moving_image, uint64_t required_number_of_overlapping_pixels, double required_fraction_of_overlapping_pixels);

// Three-image filters
std::unique_ptr<Image> filter_ternary_add(const Image& img1, const Image& img2, const Image& img3);
std::unique_ptr<Image> filter_ternary_magnitude(const Image& img1, const Image& img2, const Image& img3);
std::unique_ptr<Image> filter_ternary_magnitude_squared(const Image& img1, const Image& img2, const Image& img3);
std::unique_ptr<Image> filter_masked_fft_normalized_correlation(const Image& fixed_image, const Image& moving_image, const Image& fixed_mask, const Image& moving_mask, uint64_t required_overlapping_pixels, float required_fraction);

// NeighborhoodConnected (class API, no seeds)
std::unique_ptr<Image> filter_neighborhood_connected(const Image& img, double lower, double upper, rust::Slice<const uint32_t> radius, double replace_value);

// ── Group 1: Binary image+image ────────────────────────────────────────────
std::unique_ptr<Image> filter_maximum(const Image& img1, const Image& img2);
std::unique_ptr<Image> filter_minimum(const Image& img1, const Image& img2);
std::unique_ptr<Image> filter_modulus(const Image& img1, const Image& img2);

// ── Group 2: Unary with simple scalar/bool params ──────────────────────────
std::unique_ptr<Image> filter_laplacian(const Image& img, bool use_image_spacing);
std::unique_ptr<Image> filter_bspline_decomposition(const Image& img, uint32_t spline_order);
std::unique_ptr<Image> filter_relabel_label_map(const Image& img, bool reverse_ordering);
std::unique_ptr<Image> filter_label_unique_label_map(const Image& img, bool reverse_ordering);
std::unique_ptr<Image> filter_label_to_rgb(const Image& img, double background_value);
std::unique_ptr<Image> filter_label_voting(const Image& img);
std::unique_ptr<Image> filter_binary_image_to_label_map(const Image& img, bool fully_connected, double input_foreground_value, double output_background_value);
std::unique_ptr<Image> filter_label_image_to_label_map(const Image& img, double background_value);
std::unique_ptr<Image> filter_min_max_curvature_flow(const Image& img, double time_step, uint32_t number_of_iterations, int32_t stencil_radius);
std::unique_ptr<Image> filter_fast_marching(const Image& img, double normalization_factor, double stopping_value);
std::unique_ptr<Image> filter_connected_threshold(const Image& img, double lower, double upper, uint8_t replace_value, int32_t connectivity);

// ── Group 3: Unary with vector params ──────────────────────────────────────
std::unique_ptr<Image> filter_crop(const Image& img, rust::Slice<const uint32_t> lower_size, rust::Slice<const uint32_t> upper_size);
std::unique_ptr<Image> filter_tile(const Image& img, rust::Slice<const uint32_t> layout, double default_pixel_value);
std::unique_ptr<Image> filter_compose(const Image& img);

// ── Group 4: Two-image with params ─────────────────────────────────────────
std::unique_ptr<Image> filter_masked_assign(const Image& img, const Image& mask, double assign_constant);
std::unique_ptr<Image> filter_label_map_overlay(const Image& label_map, const Image& feature_image, double opacity);
std::unique_ptr<Image> filter_label_map_mask(const Image& label_map, const Image& feature_image, uint64_t label, double background_value, bool negated, bool crop);
std::unique_ptr<Image> filter_join_series_two(const Image& img1, const Image& img2, double origin, double spacing);
std::unique_ptr<Image> filter_isolated_connected(const Image& img, rust::Slice<const uint32_t> seed1, rust::Slice<const uint32_t> seed2, double lower, double upper, uint8_t replace_value, double tolerance, bool find_upper);
std::unique_ptr<Image> filter_isolated_watershed(const Image& img, rust::Slice<const uint32_t> seed1, rust::Slice<const uint32_t> seed2, double threshold, double upper_value_limit, double tolerance, uint8_t replace1, uint8_t replace2);

// ── Group 5: Three-image ───────────────────────────────────────────────────
std::unique_ptr<Image> filter_normalized_correlation(const Image& image, const Image& mask_image, const Image& template_image);
std::unique_ptr<Image> filter_label_map_contour_overlay(const Image& label_map, const Image& feature_image, double opacity, rust::Slice<const uint32_t> dilation_radius, rust::Slice<const uint32_t> contour_thickness, uint32_t slice_dimension, int32_t contour_type, int32_t priority);
std::unique_ptr<Image> filter_merge_label_map_two(const Image& img1, const Image& img2, int32_t method);

// ── Group 6: Image sources (no Image input) ────────────────────────────────
std::unique_ptr<Image> source_gaussian(int32_t pixel_type, rust::Slice<const uint32_t> size, rust::Slice<const double> sigma, rust::Slice<const double> mean, double scale);
std::unique_ptr<Image> source_gabor(int32_t pixel_type, rust::Slice<const uint32_t> size, rust::Slice<const double> sigma, rust::Slice<const double> mean, double frequency);
std::unique_ptr<Image> source_grid(int32_t pixel_type, rust::Slice<const uint32_t> size, rust::Slice<const double> sigma, rust::Slice<const double> grid_spacing, rust::Slice<const double> grid_offset, double scale);
std::unique_ptr<Image> source_physical_point(int32_t pixel_type, rust::Slice<const uint32_t> size);

// ── Final batch ────────────────────────────────────────────────────────────
std::unique_ptr<Image> filter_aggregate_label_map(const Image& img);
std::unique_ptr<Image> filter_label_map_to_rgb(const Image& img);
std::unique_ptr<Image> filter_extract(const Image& img, rust::Slice<const uint32_t> size, rust::Slice<const int32_t> index);
std::unique_ptr<Image> filter_nary_add_two(const Image& img1, const Image& img2);
std::unique_ptr<Image> filter_nary_maximum_two(const Image& img1, const Image& img2);


// ── Transforms ─────────────────────────────────────────────────────────────
using Transform = itk::simple::Transform;

// Factory functions
std::unique_ptr<Transform> new_affine_transform(uint32_t dimensions);
std::unique_ptr<Transform> new_translation_transform(uint32_t dimensions);
std::unique_ptr<Transform> new_scale_transform(uint32_t dimensions);
std::unique_ptr<Transform> new_euler2d_transform();
std::unique_ptr<Transform> new_euler3d_transform();
std::unique_ptr<Transform> new_similarity2d_transform();
std::unique_ptr<Transform> new_similarity3d_transform();
std::unique_ptr<Transform> new_versor_transform();
std::unique_ptr<Transform> new_versor_rigid3d_transform();
std::unique_ptr<Transform> new_scale_versor3d_transform();
std::unique_ptr<Transform> new_scale_skew_versor3d_transform();
std::unique_ptr<Transform> new_composite_transform(uint32_t dimensions);
std::unique_ptr<Transform> read_transform(rust::Str path);

// Base Transform methods (immutable)
rust::Vec<double> transform_get_parameters(const Transform& t);
rust::Vec<double> transform_get_fixed_parameters(const Transform& t);
uint64_t          transform_get_number_of_parameters(const Transform& t);
uint64_t          transform_get_number_of_fixed_parameters(const Transform& t);
rust::Vec<double> transform_transform_point(const Transform& t, rust::Slice<const double> point);
bool              transform_is_linear(const Transform& t);
uint32_t          transform_get_dimension(const Transform& t);
rust::String      transform_get_name(const Transform& t);
void              transform_write(const Transform& t, rust::Str path);
std::unique_ptr<Transform> transform_get_inverse(const Transform& t);

// Base Transform methods (mutable)
void transform_set_parameters(Transform& t, rust::Slice<const double> params);
void transform_set_fixed_parameters(Transform& t, rust::Slice<const double> params);
void transform_set_identity(Transform& t);

// AffineTransform methods
rust::Vec<double> affine_get_matrix(const Transform& t);
rust::Vec<double> affine_get_center(const Transform& t);
rust::Vec<double> affine_get_translation(const Transform& t);
void affine_set_matrix(Transform& t, rust::Slice<const double> matrix);
void affine_set_center(Transform& t, rust::Slice<const double> center);
void affine_set_translation(Transform& t, rust::Slice<const double> translation);

// TranslationTransform methods
rust::Vec<double> translation_get_offset(const Transform& t);
void translation_set_offset(Transform& t, rust::Slice<const double> offset);

// ScaleTransform methods
rust::Vec<double> scale_get_scale(const Transform& t);
rust::Vec<double> scale_get_center(const Transform& t);
void scale_set_scale(Transform& t, rust::Slice<const double> scale);
void scale_set_center(Transform& t, rust::Slice<const double> center);

// Euler2DTransform methods
rust::Vec<double> euler2d_get_center(const Transform& t);
double            euler2d_get_angle(const Transform& t);
rust::Vec<double> euler2d_get_translation(const Transform& t);
rust::Vec<double> euler2d_get_matrix(const Transform& t);
void euler2d_set_center(Transform& t, rust::Slice<const double> center);
void euler2d_set_angle(Transform& t, double angle);
void euler2d_set_translation(Transform& t, rust::Slice<const double> translation);
void euler2d_set_matrix(Transform& t, rust::Slice<const double> matrix);

// Euler3DTransform methods
rust::Vec<double> euler3d_get_center(const Transform& t);
double            euler3d_get_angle_x(const Transform& t);
double            euler3d_get_angle_y(const Transform& t);
double            euler3d_get_angle_z(const Transform& t);
rust::Vec<double> euler3d_get_translation(const Transform& t);
rust::Vec<double> euler3d_get_matrix(const Transform& t);
void euler3d_set_center(Transform& t, rust::Slice<const double> center);
void euler3d_set_rotation(Transform& t, double angle_x, double angle_y, double angle_z);
void euler3d_set_translation(Transform& t, rust::Slice<const double> translation);
void euler3d_set_matrix(Transform& t, rust::Slice<const double> matrix);
void euler3d_set_compute_zyx(Transform& t, bool compute_zyx);

// Similarity2DTransform methods
rust::Vec<double> similarity2d_get_center(const Transform& t);
double            similarity2d_get_angle(const Transform& t);
double            similarity2d_get_scale(const Transform& t);
rust::Vec<double> similarity2d_get_translation(const Transform& t);
rust::Vec<double> similarity2d_get_matrix(const Transform& t);
void similarity2d_set_center(Transform& t, rust::Slice<const double> center);
void similarity2d_set_angle(Transform& t, double angle);
void similarity2d_set_scale(Transform& t, double scale);
void similarity2d_set_translation(Transform& t, rust::Slice<const double> translation);
void similarity2d_set_matrix(Transform& t, rust::Slice<const double> matrix);

// Similarity3DTransform methods
rust::Vec<double> similarity3d_get_center(const Transform& t);
rust::Vec<double> similarity3d_get_versor(const Transform& t);
double            similarity3d_get_scale(const Transform& t);
rust::Vec<double> similarity3d_get_translation(const Transform& t);
rust::Vec<double> similarity3d_get_matrix(const Transform& t);
void similarity3d_set_center(Transform& t, rust::Slice<const double> center);
void similarity3d_set_rotation_versor(Transform& t, rust::Slice<const double> versor);
void similarity3d_set_rotation_axis_angle(Transform& t, rust::Slice<const double> axis, double angle);
void similarity3d_set_scale(Transform& t, double scale);
void similarity3d_set_translation(Transform& t, rust::Slice<const double> translation);
void similarity3d_set_matrix(Transform& t, rust::Slice<const double> matrix);

// VersorTransform methods
rust::Vec<double> versor_get_center(const Transform& t);
rust::Vec<double> versor_get_versor(const Transform& t);
rust::Vec<double> versor_get_matrix(const Transform& t);
void versor_set_center(Transform& t, rust::Slice<const double> center);
void versor_set_rotation_versor(Transform& t, rust::Slice<const double> versor);
void versor_set_rotation_axis_angle(Transform& t, rust::Slice<const double> axis, double angle);
void versor_set_matrix(Transform& t, rust::Slice<const double> matrix);

// VersorRigid3DTransform methods (same as Versor + translation)
rust::Vec<double> versor_rigid3d_get_translation(const Transform& t);
void versor_rigid3d_set_translation(Transform& t, rust::Slice<const double> translation);

// ScaleVersor3DTransform methods
rust::Vec<double> scale_versor3d_get_scale(const Transform& t);
void scale_versor3d_set_scale(Transform& t, rust::Slice<const double> scale);

// ScaleSkewVersor3DTransform methods
rust::Vec<double> scale_skew_versor3d_get_skew(const Transform& t);
void scale_skew_versor3d_set_skew(Transform& t, rust::Slice<const double> skew);

// CompositeTransform methods
void composite_add_transform(Transform& t, const Transform& to_add);
uint32_t composite_get_number_of_transforms(const Transform& t);
void composite_clear_transforms(Transform& t);
void composite_flatten_transform(Transform& t);

// DisplacementFieldTransform factory + methods
std::unique_ptr<Transform> new_displacement_field_transform(uint32_t dimensions);
std::unique_ptr<Image> displacement_field_get_displacement_field(const Transform& t);
void displacement_field_set_displacement_field(Transform& t, const Image& field);
void displacement_field_set_smoothing_off(Transform& t);
void displacement_field_set_interpolator(Transform& t, int32_t interpolator);

// Resample filter using a Transform
std::unique_ptr<Image> filter_resample(const Image& img, const Transform& tx, int32_t interpolator, double default_pixel_value, int32_t output_pixel_type);
std::unique_ptr<Image> filter_resample_to_ref(const Image& img, const Image& ref_img, const Transform& tx, int32_t interpolator, double default_pixel_value, int32_t output_pixel_type);
std::unique_ptr<Image> filter_transform_to_displacement_field(const Transform& tx, int32_t output_pixel_type, rust::Slice<const uint32_t> size, rust::Slice<const double> origin, rust::Slice<const double> spacing);

} // namespace sitk_rs

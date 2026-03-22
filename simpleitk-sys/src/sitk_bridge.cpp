#include "src/sitk_bridge.h"

namespace sitk_rs {

std::unique_ptr<Image> read_image(rust::Str path) {
    return std::make_unique<Image>(
        itk::simple::ReadImage(std::string(path.data(), path.size())));
}

void write_image(const Image& image, rust::Str path) {
    itk::simple::WriteImage(image, std::string(path.data(), path.size()));
}

uint32_t get_width(const Image& img)            { return img.GetWidth(); }
uint32_t get_height(const Image& img)           { return img.GetHeight(); }
uint32_t get_depth(const Image& img)            { return img.GetDepth(); }
uint32_t get_dimension(const Image& img)        { return img.GetDimension(); }
uint64_t get_number_of_pixels(const Image& img) { return img.GetNumberOfPixels(); }
int32_t  get_pixel_id_value(const Image& img)   { return img.GetPixelIDValue(); }

// ── Unary filters ──────────────────────────────────────────────────────────

std::unique_ptr<Image> filter_abs(const Image& img) {
    return std::make_unique<Image>(itk::simple::Abs(img));
}

std::unique_ptr<Image> filter_acos(const Image& img) {
    return std::make_unique<Image>(itk::simple::Acos(img));
}

std::unique_ptr<Image> filter_adaptive_histogram_equalization(const Image& img, rust::Slice<const uint32_t> radius, float alpha, float beta) {
    std::vector<unsigned int> r(radius.begin(), radius.end());
    return std::make_unique<Image>(itk::simple::AdaptiveHistogramEqualization(img, r, alpha, beta));
}

std::unique_ptr<Image> filter_additive_gaussian_noise(const Image& img, double standard_deviation, double mean, uint32_t seed) {
    return std::make_unique<Image>(itk::simple::AdditiveGaussianNoise(img, standard_deviation, mean, seed));
}

std::unique_ptr<Image> filter_anti_alias_binary(const Image& img, double maximum_rms_error, uint32_t number_of_iterations) {
    return std::make_unique<Image>(itk::simple::AntiAliasBinary(img, maximum_rms_error, number_of_iterations));
}

std::unique_ptr<Image> filter_approximate_signed_distance_map(const Image& img, double inside_value, double outside_value) {
    return std::make_unique<Image>(itk::simple::ApproximateSignedDistanceMap(img, inside_value, outside_value));
}

std::unique_ptr<Image> filter_area_closing(const Image& img, double lambda, bool use_image_spacing, bool fully_connected) {
    return std::make_unique<Image>(itk::simple::AreaClosing(img, lambda, use_image_spacing, fully_connected));
}

std::unique_ptr<Image> filter_area_opening(const Image& img, double lambda, bool use_image_spacing, bool fully_connected) {
    return std::make_unique<Image>(itk::simple::AreaOpening(img, lambda, use_image_spacing, fully_connected));
}

std::unique_ptr<Image> filter_asin(const Image& img) {
    return std::make_unique<Image>(itk::simple::Asin(img));
}

std::unique_ptr<Image> filter_atan(const Image& img) {
    return std::make_unique<Image>(itk::simple::Atan(img));
}

std::unique_ptr<Image> filter_bilateral(const Image& img, double domain_sigma, double range_sigma, uint32_t number_of_range_gaussian_samples) {
    return std::make_unique<Image>(itk::simple::Bilateral(img, domain_sigma, range_sigma, number_of_range_gaussian_samples));
}

std::unique_ptr<Image> filter_bin_shrink(const Image& img, rust::Slice<const uint32_t> shrink_factors) {
    std::vector<unsigned int> sf(shrink_factors.begin(), shrink_factors.end());
    return std::make_unique<Image>(itk::simple::BinShrink(img, sf));
}

std::unique_ptr<Image> filter_binary_closing_by_reconstruction(const Image& img, rust::Slice<const uint32_t> kernel_radius, int32_t kernel_type, double foreground_value, bool fully_connected) {
    std::vector<unsigned int> r(kernel_radius.begin(), kernel_radius.end());
    return std::make_unique<Image>(itk::simple::BinaryClosingByReconstruction(img, r, static_cast<itk::simple::KernelEnum>(kernel_type), foreground_value, fully_connected));
}

std::unique_ptr<Image> filter_binary_contour(const Image& img, bool fully_connected, double background_value, double foreground_value) {
    return std::make_unique<Image>(itk::simple::BinaryContour(img, fully_connected, background_value, foreground_value));
}

std::unique_ptr<Image> filter_binary_dilate(const Image& img, rust::Slice<const uint32_t> kernel_radius, int32_t kernel_type, double background_value, double foreground_value, bool boundary_to_foreground) {
    std::vector<unsigned int> r(kernel_radius.begin(), kernel_radius.end());
    return std::make_unique<Image>(itk::simple::BinaryDilate(img, r, static_cast<itk::simple::KernelEnum>(kernel_type), background_value, foreground_value, boundary_to_foreground));
}

std::unique_ptr<Image> filter_binary_erode(const Image& img, rust::Slice<const uint32_t> kernel_radius, int32_t kernel_type, double background_value, double foreground_value, bool boundary_to_foreground) {
    std::vector<unsigned int> r(kernel_radius.begin(), kernel_radius.end());
    return std::make_unique<Image>(itk::simple::BinaryErode(img, r, static_cast<itk::simple::KernelEnum>(kernel_type), background_value, foreground_value, boundary_to_foreground));
}

std::unique_ptr<Image> filter_binary_fillhole(const Image& img, bool fully_connected, double foreground_value) {
    return std::make_unique<Image>(itk::simple::BinaryFillhole(img, fully_connected, foreground_value));
}

std::unique_ptr<Image> filter_binary_grind_peak(const Image& img, bool fully_connected, double foreground_value, double background_value) {
    return std::make_unique<Image>(itk::simple::BinaryGrindPeak(img, fully_connected, foreground_value, background_value));
}

std::unique_ptr<Image> filter_binary_median(const Image& img, rust::Slice<const uint32_t> radius, double foreground_value, double background_value) {
    std::vector<unsigned int> r(radius.begin(), radius.end());
    return std::make_unique<Image>(itk::simple::BinaryMedian(img, r, foreground_value, background_value));
}

std::unique_ptr<Image> filter_binary_min_max_curvature_flow(const Image& img, double time_step, uint32_t number_of_iterations, int32_t stencil_radius, double threshold) {
    return std::make_unique<Image>(itk::simple::BinaryMinMaxCurvatureFlow(img, time_step, number_of_iterations, stencil_radius, threshold));
}

std::unique_ptr<Image> filter_binary_morphological_closing(const Image& img, rust::Slice<const uint32_t> kernel_radius, int32_t kernel_type, double foreground_value, bool safe_edge_handling) {
    std::vector<unsigned int> r(kernel_radius.begin(), kernel_radius.end());
    return std::make_unique<Image>(itk::simple::BinaryMorphologicalClosing(img, r, static_cast<itk::simple::KernelEnum>(kernel_type), foreground_value, safe_edge_handling));
}

std::unique_ptr<Image> filter_binary_morphological_opening(const Image& img, rust::Slice<const uint32_t> kernel_radius, int32_t kernel_type, double background_value, double foreground_value) {
    std::vector<unsigned int> r(kernel_radius.begin(), kernel_radius.end());
    return std::make_unique<Image>(itk::simple::BinaryMorphologicalOpening(img, r, static_cast<itk::simple::KernelEnum>(kernel_type), background_value, foreground_value));
}

std::unique_ptr<Image> filter_binary_not(const Image& img, double foreground_value, double background_value) {
    return std::make_unique<Image>(itk::simple::BinaryNot(img, foreground_value, background_value));
}

std::unique_ptr<Image> filter_binary_opening_by_reconstruction(const Image& img, rust::Slice<const uint32_t> kernel_radius, int32_t kernel_type, double foreground_value, bool fully_connected) {
    std::vector<unsigned int> r(kernel_radius.begin(), kernel_radius.end());
    return std::make_unique<Image>(itk::simple::BinaryOpeningByReconstruction(img, r, static_cast<itk::simple::KernelEnum>(kernel_type), foreground_value, fully_connected));
}

std::unique_ptr<Image> filter_binary_projection(const Image& img, uint32_t projection_dimension, double foreground_value, double background_value) {
    return std::make_unique<Image>(itk::simple::BinaryProjection(img, projection_dimension, foreground_value, background_value));
}

std::unique_ptr<Image> filter_binary_pruning(const Image& img, uint32_t iteration) {
    return std::make_unique<Image>(itk::simple::BinaryPruning(img, iteration));
}

std::unique_ptr<Image> filter_binary_thinning(const Image& img) {
    return std::make_unique<Image>(itk::simple::BinaryThinning(img));
}

std::unique_ptr<Image> filter_binary_threshold(const Image& img, double lower_threshold, double upper_threshold, uint8_t inside_value, uint8_t outside_value) {
    return std::make_unique<Image>(itk::simple::BinaryThreshold(img, lower_threshold, upper_threshold, inside_value, outside_value));
}

std::unique_ptr<Image> filter_binary_threshold_projection(const Image& img, uint32_t projection_dimension, double threshold_value, uint8_t foreground_value, uint8_t background_value) {
    return std::make_unique<Image>(itk::simple::BinaryThresholdProjection(img, projection_dimension, threshold_value, foreground_value, background_value));
}

std::unique_ptr<Image> filter_binomial_blur(const Image& img, uint32_t repetitions) {
    return std::make_unique<Image>(itk::simple::BinomialBlur(img, repetitions));
}

std::unique_ptr<Image> filter_bitwise_not(const Image& img) {
    return std::make_unique<Image>(itk::simple::BitwiseNot(img));
}

std::unique_ptr<Image> filter_black_top_hat(const Image& img, rust::Slice<const uint32_t> kernel_radius, int32_t kernel_type, bool safe_border) {
    std::vector<unsigned int> r(kernel_radius.begin(), kernel_radius.end());
    return std::make_unique<Image>(itk::simple::BlackTopHat(img, r, static_cast<itk::simple::KernelEnum>(kernel_type), safe_border));
}

std::unique_ptr<Image> filter_bounded_reciprocal(const Image& img) {
    return std::make_unique<Image>(itk::simple::BoundedReciprocal(img));
}

std::unique_ptr<Image> filter_box_mean(const Image& img, rust::Slice<const uint32_t> radius) {
    std::vector<unsigned int> r(radius.begin(), radius.end());
    return std::make_unique<Image>(itk::simple::BoxMean(img, r));
}

std::unique_ptr<Image> filter_box_sigma(const Image& img, rust::Slice<const uint32_t> radius) {
    std::vector<unsigned int> r(radius.begin(), radius.end());
    return std::make_unique<Image>(itk::simple::BoxSigma(img, r));
}

std::unique_ptr<Image> filter_canny_edge_detection(const Image& img, double lower_threshold, double upper_threshold, double variance, double maximum_error) {
    std::vector<double> var_vec(3, variance);
    std::vector<double> err_vec(3, maximum_error);
    return std::make_unique<Image>(itk::simple::CannyEdgeDetection(img, lower_threshold, upper_threshold, var_vec, err_vec));
}

std::unique_ptr<Image> filter_cast(const Image& img, int32_t pixel_id) {
    auto f = itk::simple::CastImageFilter();
    f.SetOutputPixelType(static_cast<itk::simple::PixelIDValueEnum>(pixel_id));
    return std::make_unique<Image>(f.Execute(img));
}

std::unique_ptr<Image> filter_clamp(const Image& img, double lower_bound, double upper_bound) {
    auto f = itk::simple::ClampImageFilter();
    f.SetLowerBound(lower_bound);
    f.SetUpperBound(upper_bound);
    return std::make_unique<Image>(f.Execute(img));
}

std::unique_ptr<Image> filter_closing_by_reconstruction(const Image& img, rust::Slice<const uint32_t> kernel_radius, int32_t kernel_type, bool fully_connected, bool preserve_intensities) {
    std::vector<unsigned int> r(kernel_radius.begin(), kernel_radius.end());
    return std::make_unique<Image>(itk::simple::ClosingByReconstruction(img, r, static_cast<itk::simple::KernelEnum>(kernel_type), fully_connected, preserve_intensities));
}

std::unique_ptr<Image> filter_complex_to_imaginary(const Image& img) {
    return std::make_unique<Image>(itk::simple::ComplexToImaginary(img));
}

std::unique_ptr<Image> filter_complex_to_modulus(const Image& img) {
    return std::make_unique<Image>(itk::simple::ComplexToModulus(img));
}

std::unique_ptr<Image> filter_complex_to_phase(const Image& img) {
    return std::make_unique<Image>(itk::simple::ComplexToPhase(img));
}

std::unique_ptr<Image> filter_complex_to_real(const Image& img) {
    return std::make_unique<Image>(itk::simple::ComplexToReal(img));
}

std::unique_ptr<Image> filter_connected_component(const Image& img, bool fully_connected) {
    return std::make_unique<Image>(itk::simple::ConnectedComponent(img, fully_connected));
}

std::unique_ptr<Image> filter_cos(const Image& img) {
    return std::make_unique<Image>(itk::simple::Cos(img));
}

std::unique_ptr<Image> filter_curvature_anisotropic_diffusion(const Image& img, double time_step, double conductance_parameter, uint32_t conductance_scaling_update_interval, uint32_t number_of_iterations) {
    return std::make_unique<Image>(itk::simple::CurvatureAnisotropicDiffusion(img, time_step, conductance_parameter, conductance_scaling_update_interval, number_of_iterations));
}

std::unique_ptr<Image> filter_curvature_flow(const Image& img, double time_step, uint32_t number_of_iterations) {
    return std::make_unique<Image>(itk::simple::CurvatureFlow(img, time_step, number_of_iterations));
}

std::unique_ptr<Image> filter_cyclic_shift(const Image& img, rust::Slice<const int32_t> shift) {
    std::vector<int> s(shift.begin(), shift.end());
    return std::make_unique<Image>(itk::simple::CyclicShift(img, s));
}

std::unique_ptr<Image> filter_danielsson_distance_map(const Image& img, bool input_is_binary, bool squared_distance, bool use_image_spacing) {
    return std::make_unique<Image>(itk::simple::DanielssonDistanceMap(img, input_is_binary, squared_distance, use_image_spacing));
}

std::unique_ptr<Image> filter_dilate_object_morphology(const Image& img, rust::Slice<const uint32_t> kernel_radius, int32_t kernel_type, double object_value, double /*background_value*/) {
    std::vector<unsigned int> r(kernel_radius.begin(), kernel_radius.end());
    return std::make_unique<Image>(itk::simple::DilateObjectMorphology(img, r, static_cast<itk::simple::KernelEnum>(kernel_type), object_value));
}

std::unique_ptr<Image> filter_discrete_gaussian(const Image& img, double variance, uint32_t maximum_kernel_width, double maximum_error, bool use_image_spacing) {
    std::vector<double> var_vec(3, variance);
    std::vector<double> err_vec(3, maximum_error);
    return std::make_unique<Image>(itk::simple::DiscreteGaussian(img, var_vec, maximum_kernel_width, err_vec, use_image_spacing));
}

std::unique_ptr<Image> filter_discrete_gaussian_derivative(const Image& img, double variance, uint32_t order, uint32_t maximum_kernel_width, double maximum_error, bool normalize_across_scale) {
    std::vector<double> var_vec(3, variance);
    std::vector<unsigned int> ord_vec(3, order);
    return std::make_unique<Image>(itk::simple::DiscreteGaussianDerivative(img, var_vec, ord_vec, maximum_kernel_width, maximum_error, true, normalize_across_scale));
}

std::unique_ptr<Image> filter_displacement_field_jacobian_determinant(const Image& img, bool use_image_spacing) {
    return std::make_unique<Image>(itk::simple::DisplacementFieldJacobianDeterminant(img, use_image_spacing));
}

std::unique_ptr<Image> filter_double_threshold(const Image& img, double threshold1, double threshold2, double threshold3, double threshold4, uint8_t inside_value, uint8_t outside_value, bool fully_connected) {
    return std::make_unique<Image>(itk::simple::DoubleThreshold(img, threshold1, threshold2, threshold3, threshold4, inside_value, outside_value, fully_connected));
}

std::unique_ptr<Image> filter_edge_potential(const Image& img) {
    return std::make_unique<Image>(itk::simple::EdgePotential(img));
}

std::unique_ptr<Image> filter_erode_object_morphology(const Image& img, rust::Slice<const uint32_t> kernel_radius, int32_t kernel_type, double object_value, double background_value) {
    std::vector<unsigned int> r(kernel_radius.begin(), kernel_radius.end());
    return std::make_unique<Image>(itk::simple::ErodeObjectMorphology(img, r, static_cast<itk::simple::KernelEnum>(kernel_type), object_value, background_value));
}

std::unique_ptr<Image> filter_exp(const Image& img) {
    return std::make_unique<Image>(itk::simple::Exp(img));
}

std::unique_ptr<Image> filter_exp_negative(const Image& img) {
    return std::make_unique<Image>(itk::simple::ExpNegative(img));
}

std::unique_ptr<Image> filter_fast_approximate_rank(const Image& img, rust::Slice<const uint32_t> radius, double rank) {
    std::vector<unsigned int> r(radius.begin(), radius.end());
    return std::make_unique<Image>(itk::simple::FastApproximateRank(img, rank, r));
}

std::unique_ptr<Image> filter_fft_shift(const Image& img, bool inverse) {
    return std::make_unique<Image>(itk::simple::FFTShift(img, inverse));
}

std::unique_ptr<Image> filter_flip_image(const Image& img, rust::Slice<const bool> flip_axes, bool flip_about_origin) {
    std::vector<bool> fa(flip_axes.begin(), flip_axes.end());
    return std::make_unique<Image>(itk::simple::Flip(img, fa, flip_about_origin));
}

std::unique_ptr<Image> filter_gradient_anisotropic_diffusion(const Image& img, double time_step, double conductance_parameter, uint32_t conductance_scaling_update_interval, uint32_t number_of_iterations) {
    return std::make_unique<Image>(itk::simple::GradientAnisotropicDiffusion(img, time_step, conductance_parameter, conductance_scaling_update_interval, number_of_iterations));
}

std::unique_ptr<Image> filter_gradient_magnitude(const Image& img) {
    return std::make_unique<Image>(itk::simple::GradientMagnitude(img));
}

std::unique_ptr<Image> filter_gradient_magnitude_recursive_gaussian(const Image& img, double sigma, bool normalize_across_scale) {
    return std::make_unique<Image>(itk::simple::GradientMagnitudeRecursiveGaussian(img, sigma, normalize_across_scale));
}

std::unique_ptr<Image> filter_grayscale_dilate(const Image& img, rust::Slice<const uint32_t> kernel_radius, int32_t kernel_type) {
    std::vector<unsigned int> r(kernel_radius.begin(), kernel_radius.end());
    return std::make_unique<Image>(itk::simple::GrayscaleDilate(img, r, static_cast<itk::simple::KernelEnum>(kernel_type)));
}

std::unique_ptr<Image> filter_grayscale_erode(const Image& img, rust::Slice<const uint32_t> kernel_radius, int32_t kernel_type) {
    std::vector<unsigned int> r(kernel_radius.begin(), kernel_radius.end());
    return std::make_unique<Image>(itk::simple::GrayscaleErode(img, r, static_cast<itk::simple::KernelEnum>(kernel_type)));
}

std::unique_ptr<Image> filter_grayscale_fillhole(const Image& img, bool fully_connected) {
    return std::make_unique<Image>(itk::simple::GrayscaleFillhole(img, fully_connected));
}

std::unique_ptr<Image> filter_grayscale_grind_peak(const Image& img, bool fully_connected) {
    return std::make_unique<Image>(itk::simple::GrayscaleGrindPeak(img, fully_connected));
}

std::unique_ptr<Image> filter_grayscale_morphological_closing(const Image& img, rust::Slice<const uint32_t> kernel_radius, int32_t kernel_type, bool safe_border) {
    std::vector<unsigned int> r(kernel_radius.begin(), kernel_radius.end());
    return std::make_unique<Image>(itk::simple::GrayscaleMorphologicalClosing(img, r, static_cast<itk::simple::KernelEnum>(kernel_type), safe_border));
}

std::unique_ptr<Image> filter_grayscale_morphological_opening(const Image& img, rust::Slice<const uint32_t> kernel_radius, int32_t kernel_type, bool safe_border) {
    std::vector<unsigned int> r(kernel_radius.begin(), kernel_radius.end());
    return std::make_unique<Image>(itk::simple::GrayscaleMorphologicalOpening(img, r, static_cast<itk::simple::KernelEnum>(kernel_type), safe_border));
}

std::unique_ptr<Image> filter_h_concave(const Image& img, double height, bool fully_connected) {
    return std::make_unique<Image>(itk::simple::HConcave(img, height, fully_connected));
}

std::unique_ptr<Image> filter_h_convex(const Image& img, double height, bool fully_connected) {
    return std::make_unique<Image>(itk::simple::HConvex(img, height, fully_connected));
}

std::unique_ptr<Image> filter_h_maxima(const Image& img, double height, bool /*fully_connected*/) {
    return std::make_unique<Image>(itk::simple::HMaxima(img, height));
}

std::unique_ptr<Image> filter_h_minima(const Image& img, double height, bool fully_connected) {
    return std::make_unique<Image>(itk::simple::HMinima(img, height, fully_connected));
}

std::unique_ptr<Image> filter_huang_threshold(const Image& img, uint8_t inside_value, uint8_t outside_value, uint32_t number_of_histogram_bins) {
    return std::make_unique<Image>(itk::simple::HuangThreshold(img, inside_value, outside_value, number_of_histogram_bins));
}

std::unique_ptr<Image> filter_intensity_windowing(const Image& img, double window_minimum, double window_maximum, double output_minimum, double output_maximum) {
    return std::make_unique<Image>(itk::simple::IntensityWindowing(img, window_minimum, window_maximum, output_minimum, output_maximum));
}

std::unique_ptr<Image> filter_intermodes_threshold(const Image& img, uint8_t inside_value, uint8_t outside_value, uint32_t number_of_histogram_bins) {
    return std::make_unique<Image>(itk::simple::IntermodesThreshold(img, inside_value, outside_value, number_of_histogram_bins));
}

std::unique_ptr<Image> filter_invert_intensity(const Image& img, double maximum) {
    return std::make_unique<Image>(itk::simple::InvertIntensity(img, maximum));
}

std::unique_ptr<Image> filter_iso_contour_distance(const Image& img, double level_set_value, double far_value) {
    return std::make_unique<Image>(itk::simple::IsoContourDistance(img, level_set_value, far_value));
}

std::unique_ptr<Image> filter_iso_data_threshold(const Image& img, uint8_t inside_value, uint8_t outside_value, uint32_t number_of_histogram_bins) {
    return std::make_unique<Image>(itk::simple::IsoDataThreshold(img, inside_value, outside_value, number_of_histogram_bins));
}

std::unique_ptr<Image> filter_kittler_illingworth_threshold(const Image& img, uint8_t inside_value, uint8_t outside_value, uint32_t number_of_histogram_bins) {
    return std::make_unique<Image>(itk::simple::KittlerIllingworthThreshold(img, inside_value, outside_value, number_of_histogram_bins));
}

std::unique_ptr<Image> filter_label_contour(const Image& img, bool fully_connected, double background_value) {
    return std::make_unique<Image>(itk::simple::LabelContour(img, fully_connected, background_value));
}

std::unique_ptr<Image> filter_laplacian_recursive_gaussian(const Image& img, double sigma, bool normalize_across_scale) {
    return std::make_unique<Image>(itk::simple::LaplacianRecursiveGaussian(img, sigma, normalize_across_scale));
}

std::unique_ptr<Image> filter_laplacian_sharpening(const Image& img, bool use_image_spacing) {
    return std::make_unique<Image>(itk::simple::LaplacianSharpening(img, use_image_spacing));
}

std::unique_ptr<Image> filter_li_threshold(const Image& img, uint8_t inside_value, uint8_t outside_value, uint32_t number_of_histogram_bins) {
    return std::make_unique<Image>(itk::simple::LiThreshold(img, inside_value, outside_value, number_of_histogram_bins));
}

std::unique_ptr<Image> filter_log(const Image& img) {
    return std::make_unique<Image>(itk::simple::Log(img));
}

std::unique_ptr<Image> filter_log10(const Image& img) {
    return std::make_unique<Image>(itk::simple::Log10(img));
}

std::unique_ptr<Image> filter_maximum_entropy_threshold(const Image& img, uint8_t inside_value, uint8_t outside_value, uint32_t number_of_histogram_bins) {
    return std::make_unique<Image>(itk::simple::MaximumEntropyThreshold(img, inside_value, outside_value, number_of_histogram_bins));
}

std::unique_ptr<Image> filter_maximum_projection(const Image& img, uint32_t projection_dimension) {
    return std::make_unique<Image>(itk::simple::MaximumProjection(img, projection_dimension));
}

std::unique_ptr<Image> filter_mean(const Image& img, rust::Slice<const uint32_t> radius) {
    std::vector<unsigned int> r(radius.begin(), radius.end());
    return std::make_unique<Image>(itk::simple::Mean(img, r));
}

std::unique_ptr<Image> filter_mean_projection(const Image& img, uint32_t projection_dimension) {
    return std::make_unique<Image>(itk::simple::MeanProjection(img, projection_dimension));
}

std::unique_ptr<Image> filter_median(const Image& img, rust::Slice<const uint32_t> radius) {
    std::vector<unsigned int> r(radius.begin(), radius.end());
    return std::make_unique<Image>(itk::simple::Median(img, r));
}

std::unique_ptr<Image> filter_median_projection(const Image& img, uint32_t projection_dimension) {
    return std::make_unique<Image>(itk::simple::MedianProjection(img, projection_dimension));
}

std::unique_ptr<Image> filter_minimum_projection(const Image& img, uint32_t projection_dimension) {
    return std::make_unique<Image>(itk::simple::MinimumProjection(img, projection_dimension));
}

std::unique_ptr<Image> filter_mirror_pad(const Image& img, rust::Slice<const uint32_t> pad_lower_bound, rust::Slice<const uint32_t> pad_upper_bound) {
    std::vector<unsigned int> lb(pad_lower_bound.begin(), pad_lower_bound.end());
    std::vector<unsigned int> ub(pad_upper_bound.begin(), pad_upper_bound.end());
    return std::make_unique<Image>(itk::simple::MirrorPad(img, lb, ub));
}

std::unique_ptr<Image> filter_moments_threshold(const Image& img, uint8_t inside_value, uint8_t outside_value, uint32_t number_of_histogram_bins) {
    return std::make_unique<Image>(itk::simple::MomentsThreshold(img, inside_value, outside_value, number_of_histogram_bins));
}

std::unique_ptr<Image> filter_morphological_gradient(const Image& img, rust::Slice<const uint32_t> kernel_radius, int32_t kernel_type) {
    std::vector<unsigned int> r(kernel_radius.begin(), kernel_radius.end());
    return std::make_unique<Image>(itk::simple::MorphologicalGradient(img, r, static_cast<itk::simple::KernelEnum>(kernel_type)));
}

std::unique_ptr<Image> filter_morphological_watershed(const Image& img, double level, bool mark_watershed_line, bool fully_connected) {
    return std::make_unique<Image>(itk::simple::MorphologicalWatershed(img, level, mark_watershed_line, fully_connected));
}

std::unique_ptr<Image> filter_noise_image_filter(const Image& img) {
    std::vector<unsigned int> r(3, 1);
    return std::make_unique<Image>(itk::simple::Noise(img, r));
}

std::unique_ptr<Image> filter_normalize(const Image& img) {
    return std::make_unique<Image>(itk::simple::Normalize(img));
}

std::unique_ptr<Image> filter_normalize_to_constant(const Image& img, double constant) {
    return std::make_unique<Image>(itk::simple::NormalizeToConstant(img, constant));
}

std::unique_ptr<Image> filter_not(const Image& img) {
    return std::make_unique<Image>(itk::simple::Not(img));
}

std::unique_ptr<Image> filter_opening_by_reconstruction(const Image& img, rust::Slice<const uint32_t> kernel_radius, int32_t kernel_type, bool fully_connected, bool preserve_intensities) {
    std::vector<unsigned int> r(kernel_radius.begin(), kernel_radius.end());
    return std::make_unique<Image>(itk::simple::OpeningByReconstruction(img, r, static_cast<itk::simple::KernelEnum>(kernel_type), fully_connected, preserve_intensities));
}

std::unique_ptr<Image> filter_otsu_multiple_thresholds(const Image& img, uint32_t number_of_thresholds, uint8_t label_offset, uint32_t number_of_histogram_bins, bool valley_emphasis) {
    return std::make_unique<Image>(itk::simple::OtsuMultipleThresholds(img, number_of_thresholds, label_offset, number_of_histogram_bins, valley_emphasis));
}

std::unique_ptr<Image> filter_otsu_threshold(const Image& img, uint8_t inside_value, uint8_t outside_value, uint32_t number_of_histogram_bins) {
    return std::make_unique<Image>(itk::simple::OtsuThreshold(img, inside_value, outside_value, number_of_histogram_bins));
}

std::unique_ptr<Image> filter_rank(const Image& img, rust::Slice<const uint32_t> radius, double rank) {
    std::vector<unsigned int> r(radius.begin(), radius.end());
    return std::make_unique<Image>(itk::simple::Rank(img, rank, r));
}

std::unique_ptr<Image> filter_recursive_gaussian(const Image& img, double sigma, bool normalize_across_scale, uint32_t order, uint32_t direction) {
    return std::make_unique<Image>(itk::simple::RecursiveGaussian(img, sigma, normalize_across_scale,
        static_cast<itk::simple::RecursiveGaussianImageFilter::OrderType>(order), direction));
}

std::unique_ptr<Image> filter_regional_maxima(const Image& img, double background_value, bool fully_connected, bool flat_is_maxima) {
    return std::make_unique<Image>(itk::simple::RegionalMaxima(img, background_value, fully_connected, flat_is_maxima));
}

std::unique_ptr<Image> filter_regional_minima(const Image& img, double background_value, bool fully_connected, bool flat_is_minima) {
    return std::make_unique<Image>(itk::simple::RegionalMinima(img, background_value, fully_connected, flat_is_minima));
}

std::unique_ptr<Image> filter_relabel_component(const Image& img, uint64_t minimum_object_size, bool sort_by_object_size) {
    return std::make_unique<Image>(itk::simple::RelabelComponent(img, minimum_object_size, sort_by_object_size));
}

std::unique_ptr<Image> filter_renyi_entropy_threshold(const Image& img, uint8_t inside_value, uint8_t outside_value, uint32_t number_of_histogram_bins) {
    return std::make_unique<Image>(itk::simple::RenyiEntropyThreshold(img, inside_value, outside_value, number_of_histogram_bins));
}

std::unique_ptr<Image> filter_rescale_intensity(const Image& img, double output_minimum, double output_maximum) {
    return std::make_unique<Image>(itk::simple::RescaleIntensity(img, output_minimum, output_maximum));
}

std::unique_ptr<Image> filter_round(const Image& img) {
    return std::make_unique<Image>(itk::simple::Round(img));
}

std::unique_ptr<Image> filter_salt_and_pepper_noise(const Image& img, double probability, uint32_t seed) {
    return std::make_unique<Image>(itk::simple::SaltAndPepperNoise(img, probability, seed));
}

std::unique_ptr<Image> filter_shanbhag_threshold(const Image& img, uint8_t inside_value, uint8_t outside_value, uint32_t number_of_histogram_bins) {
    return std::make_unique<Image>(itk::simple::ShanbhagThreshold(img, inside_value, outside_value, number_of_histogram_bins));
}

std::unique_ptr<Image> filter_shift_scale(const Image& img, double shift, double scale) {
    return std::make_unique<Image>(itk::simple::ShiftScale(img, shift, scale));
}

std::unique_ptr<Image> filter_shot_noise(const Image& img, double scale, uint32_t seed) {
    return std::make_unique<Image>(itk::simple::ShotNoise(img, scale, seed));
}

std::unique_ptr<Image> filter_shrink(const Image& img, rust::Slice<const uint32_t> shrink_factors) {
    std::vector<unsigned int> sf(shrink_factors.begin(), shrink_factors.end());
    return std::make_unique<Image>(itk::simple::Shrink(img, sf));
}

std::unique_ptr<Image> filter_sigmoid(const Image& img, double alpha, double beta, double output_maximum, double output_minimum) {
    return std::make_unique<Image>(itk::simple::Sigmoid(img, alpha, beta, output_maximum, output_minimum));
}

std::unique_ptr<Image> filter_signed_danielsson_distance_map(const Image& img, bool inside_is_positive, bool squared_distance, bool use_image_spacing) {
    return std::make_unique<Image>(itk::simple::SignedDanielssonDistanceMap(img, inside_is_positive, squared_distance, use_image_spacing));
}

std::unique_ptr<Image> filter_signed_maurer_distance_map(const Image& img, bool inside_is_positive, bool squared_distance, bool use_image_spacing, double background_value) {
    return std::make_unique<Image>(itk::simple::SignedMaurerDistanceMap(img, inside_is_positive, squared_distance, use_image_spacing, background_value));
}

std::unique_ptr<Image> filter_sin(const Image& img) {
    return std::make_unique<Image>(itk::simple::Sin(img));
}

std::unique_ptr<Image> filter_smoothing_recursive_gaussian(const Image& img, rust::Slice<const double> sigma, bool normalize_across_scale) {
    std::vector<double> s(sigma.begin(), sigma.end());
    return std::make_unique<Image>(itk::simple::SmoothingRecursiveGaussian(img, s, normalize_across_scale));
}

std::unique_ptr<Image> filter_speckle_noise(const Image& img, double standard_deviation, uint32_t seed) {
    return std::make_unique<Image>(itk::simple::SpeckleNoise(img, standard_deviation, seed));
}

std::unique_ptr<Image> filter_sqrt(const Image& img) {
    return std::make_unique<Image>(itk::simple::Sqrt(img));
}

std::unique_ptr<Image> filter_square(const Image& img) {
    return std::make_unique<Image>(itk::simple::Square(img));
}

std::unique_ptr<Image> filter_standard_deviation_projection(const Image& img, uint32_t projection_dimension) {
    return std::make_unique<Image>(itk::simple::StandardDeviationProjection(img, projection_dimension));
}

std::unique_ptr<Image> filter_sum_projection(const Image& img, uint32_t projection_dimension) {
    return std::make_unique<Image>(itk::simple::SumProjection(img, projection_dimension));
}

std::unique_ptr<Image> filter_tan(const Image& img) {
    return std::make_unique<Image>(itk::simple::Tan(img));
}

std::unique_ptr<Image> filter_threshold(const Image& img, double lower_threshold, double upper_threshold, double outside_value) {
    return std::make_unique<Image>(itk::simple::Threshold(img, lower_threshold, upper_threshold, outside_value));
}

std::unique_ptr<Image> filter_triangle_threshold(const Image& img, uint8_t inside_value, uint8_t outside_value, uint32_t number_of_histogram_bins) {
    return std::make_unique<Image>(itk::simple::TriangleThreshold(img, inside_value, outside_value, number_of_histogram_bins));
}

std::unique_ptr<Image> filter_unary_minus(const Image& img) {
    return std::make_unique<Image>(itk::simple::UnaryMinus(img));
}

std::unique_ptr<Image> filter_unsharp_mask(const Image& img, rust::Slice<const double> sigmas, double amount, double threshold) {
    std::vector<double> s(sigmas.begin(), sigmas.end());
    return std::make_unique<Image>(itk::simple::UnsharpMask(img, s, amount, threshold));
}

std::unique_ptr<Image> filter_valued_regional_maxima(const Image& img, bool fully_connected) {
    return std::make_unique<Image>(itk::simple::ValuedRegionalMaxima(img, fully_connected));
}

std::unique_ptr<Image> filter_valued_regional_minima(const Image& img, bool fully_connected) {
    return std::make_unique<Image>(itk::simple::ValuedRegionalMinima(img, fully_connected));
}

std::unique_ptr<Image> filter_vector_index_selection_cast(const Image& img, uint32_t index, int32_t output_pixel_type) {
    return std::make_unique<Image>(itk::simple::VectorIndexSelectionCast(img, index, static_cast<itk::simple::PixelIDValueEnum>(output_pixel_type)));
}

std::unique_ptr<Image> filter_vector_magnitude(const Image& img) {
    return std::make_unique<Image>(itk::simple::VectorMagnitude(img));
}

std::unique_ptr<Image> filter_voting_binary(const Image& img, rust::Slice<const uint32_t> radius, uint32_t birth_threshold, uint32_t survival_threshold, double foreground_value, double background_value) {
    std::vector<unsigned int> r(radius.begin(), radius.end());
    return std::make_unique<Image>(itk::simple::VotingBinary(img, r, birth_threshold, survival_threshold, foreground_value, background_value));
}

std::unique_ptr<Image> filter_voting_binary_hole_filling(const Image& img, rust::Slice<const uint32_t> radius, uint32_t majority_threshold, double foreground_value, double background_value, uint32_t /*maximum_number_of_iterations*/) {
    std::vector<unsigned int> r(radius.begin(), radius.end());
    return std::make_unique<Image>(itk::simple::VotingBinaryHoleFilling(img, r, majority_threshold, foreground_value, background_value));
}

std::unique_ptr<Image> filter_voting_binary_iterative_hole_filling(const Image& img, rust::Slice<const uint32_t> radius, uint32_t majority_threshold, double foreground_value, double background_value, uint32_t maximum_number_of_iterations) {
    std::vector<unsigned int> r(radius.begin(), radius.end());
    return std::make_unique<Image>(itk::simple::VotingBinaryIterativeHoleFilling(img, r, maximum_number_of_iterations, majority_threshold, foreground_value, background_value));
}

std::unique_ptr<Image> filter_white_top_hat(const Image& img, rust::Slice<const uint32_t> kernel_radius, int32_t kernel_type, bool safe_border) {
    std::vector<unsigned int> r(kernel_radius.begin(), kernel_radius.end());
    return std::make_unique<Image>(itk::simple::WhiteTopHat(img, r, static_cast<itk::simple::KernelEnum>(kernel_type), safe_border));
}

std::unique_ptr<Image> filter_yen_threshold(const Image& img, uint8_t inside_value, uint8_t outside_value, uint32_t number_of_histogram_bins) {
    return std::make_unique<Image>(itk::simple::YenThreshold(img, inside_value, outside_value, number_of_histogram_bins));
}

std::unique_ptr<Image> filter_zero_crossing(const Image& img, uint8_t foreground_value, uint8_t background_value) {
    return std::make_unique<Image>(itk::simple::ZeroCrossing(img, foreground_value, background_value));
}

std::unique_ptr<Image> filter_zero_crossing_based_edge_detection(const Image& img, double variance, double maximum_error) {
    return std::make_unique<Image>(itk::simple::ZeroCrossingBasedEdgeDetection(img, variance, maximum_error));
}

std::unique_ptr<Image> filter_zero_flux_neumann_pad(const Image& img, rust::Slice<const uint32_t> pad_lower_bound, rust::Slice<const uint32_t> pad_upper_bound) {
    std::vector<unsigned int> lb(pad_lower_bound.begin(), pad_lower_bound.end());
    std::vector<unsigned int> ub(pad_upper_bound.begin(), pad_upper_bound.end());
    return std::make_unique<Image>(itk::simple::ZeroFluxNeumannPad(img, lb, ub));
}

std::unique_ptr<Image> filter_constant_pad(const Image& img, rust::Slice<const uint32_t> pad_lower_bound, rust::Slice<const uint32_t> pad_upper_bound, double constant) {
    std::vector<unsigned int> lb(pad_lower_bound.begin(), pad_lower_bound.end());
    std::vector<unsigned int> ub(pad_upper_bound.begin(), pad_upper_bound.end());
    return std::make_unique<Image>(itk::simple::ConstantPad(img, lb, ub, constant));
}

std::unique_ptr<Image> filter_wrap_pad(const Image& img, rust::Slice<const uint32_t> pad_lower_bound, rust::Slice<const uint32_t> pad_upper_bound) {
    std::vector<unsigned int> lb(pad_lower_bound.begin(), pad_lower_bound.end());
    std::vector<unsigned int> ub(pad_upper_bound.begin(), pad_upper_bound.end());
    return std::make_unique<Image>(itk::simple::WrapPad(img, lb, ub));
}

// ── Binary filters ─────────────────────────────────────────────────────────

std::unique_ptr<Image> filter_absolute_value_difference(const Image& img1, const Image& img2) {
    return std::make_unique<Image>(itk::simple::AbsoluteValueDifference(img1, img2));
}

std::unique_ptr<Image> filter_add(const Image& img1, const Image& img2) {
    return std::make_unique<Image>(itk::simple::Add(img1, img2));
}

std::unique_ptr<Image> filter_and(const Image& img1, const Image& img2) {
    return std::make_unique<Image>(itk::simple::And(img1, img2));
}

std::unique_ptr<Image> filter_atan2(const Image& img1, const Image& img2) {
    return std::make_unique<Image>(itk::simple::Atan2(img1, img2));
}

std::unique_ptr<Image> filter_binary_magnitude(const Image& img1, const Image& img2) {
    return std::make_unique<Image>(itk::simple::BinaryMagnitude(img1, img2));
}

std::unique_ptr<Image> filter_binary_reconstruction_by_dilation(const Image& img1, const Image& img2, double background_value, double foreground_value, bool fully_connected) {
    return std::make_unique<Image>(itk::simple::BinaryReconstructionByDilation(img1, img2, background_value, foreground_value, fully_connected));
}

std::unique_ptr<Image> filter_binary_reconstruction_by_erosion(const Image& img1, const Image& img2, double background_value, double foreground_value, bool fully_connected) {
    return std::make_unique<Image>(itk::simple::BinaryReconstructionByErosion(img1, img2, background_value, foreground_value, fully_connected));
}

std::unique_ptr<Image> filter_checker_board(const Image& img1, const Image& img2, rust::Slice<const uint32_t> checker_pattern) {
    std::vector<unsigned int> cp(checker_pattern.begin(), checker_pattern.end());
    return std::make_unique<Image>(itk::simple::CheckerBoard(img1, img2, cp));
}

std::unique_ptr<Image> filter_divide(const Image& img1, const Image& img2) {
    return std::make_unique<Image>(itk::simple::Divide(img1, img2));
}

std::unique_ptr<Image> filter_divide_floor(const Image& img1, const Image& img2) {
    return std::make_unique<Image>(itk::simple::DivideFloor(img1, img2));
}

std::unique_ptr<Image> filter_divide_real(const Image& img1, const Image& img2) {
    return std::make_unique<Image>(itk::simple::DivideReal(img1, img2));
}

std::unique_ptr<Image> filter_equal(const Image& img1, const Image& img2) {
    return std::make_unique<Image>(itk::simple::Equal(img1, img2));
}

std::unique_ptr<Image> filter_grayscale_geodesic_dilate(const Image& marker, const Image& mask, bool run_one_iteration, bool fully_connected) {
    return std::make_unique<Image>(itk::simple::GrayscaleGeodesicDilate(marker, mask, run_one_iteration, fully_connected));
}

std::unique_ptr<Image> filter_grayscale_geodesic_erode(const Image& marker, const Image& mask, bool run_one_iteration, bool fully_connected) {
    return std::make_unique<Image>(itk::simple::GrayscaleGeodesicErode(marker, mask, run_one_iteration, fully_connected));
}

std::unique_ptr<Image> filter_greater(const Image& img1, const Image& img2) {
    return std::make_unique<Image>(itk::simple::Greater(img1, img2));
}

std::unique_ptr<Image> filter_greater_equal(const Image& img1, const Image& img2) {
    return std::make_unique<Image>(itk::simple::GreaterEqual(img1, img2));
}

std::unique_ptr<Image> filter_histogram_matching(const Image& img, const Image& reference, uint32_t number_of_histogram_levels, uint32_t number_of_match_points, bool threshold_at_mean_intensity) {
    return std::make_unique<Image>(itk::simple::HistogramMatching(img, reference, number_of_histogram_levels, number_of_match_points, threshold_at_mean_intensity));
}

std::unique_ptr<Image> filter_less(const Image& img1, const Image& img2) {
    return std::make_unique<Image>(itk::simple::Less(img1, img2));
}

std::unique_ptr<Image> filter_less_equal(const Image& img1, const Image& img2) {
    return std::make_unique<Image>(itk::simple::LessEqual(img1, img2));
}

std::unique_ptr<Image> filter_magnitude_and_phase_to_complex(const Image& img1, const Image& img2) {
    return std::make_unique<Image>(itk::simple::MagnitudeAndPhaseToComplex(img1, img2));
}

std::unique_ptr<Image> filter_mask_image(const Image& img, const Image& mask, double outside_value, double masking_value) {
    return std::make_unique<Image>(itk::simple::Mask(img, mask, outside_value, masking_value));
}

std::unique_ptr<Image> filter_mask_negated_image(const Image& img, const Image& mask, double outside_value, double masking_value) {
    return std::make_unique<Image>(itk::simple::MaskNegated(img, mask, outside_value, masking_value));
}

std::unique_ptr<Image> filter_morphological_watershed_from_markers(const Image& img, const Image& marker, bool mark_watershed_line, bool fully_connected) {
    return std::make_unique<Image>(itk::simple::MorphologicalWatershedFromMarkers(img, marker, mark_watershed_line, fully_connected));
}

std::unique_ptr<Image> filter_multiply(const Image& img1, const Image& img2) {
    return std::make_unique<Image>(itk::simple::Multiply(img1, img2));
}

std::unique_ptr<Image> filter_not_equal(const Image& img1, const Image& img2) {
    return std::make_unique<Image>(itk::simple::NotEqual(img1, img2));
}

std::unique_ptr<Image> filter_or(const Image& img1, const Image& img2) {
    return std::make_unique<Image>(itk::simple::Or(img1, img2));
}

std::unique_ptr<Image> filter_pow(const Image& img1, const Image& img2) {
    return std::make_unique<Image>(itk::simple::Pow(img1, img2));
}

std::unique_ptr<Image> filter_real_and_imaginary_to_complex(const Image& img1, const Image& img2) {
    return std::make_unique<Image>(itk::simple::RealAndImaginaryToComplex(img1, img2));
}

std::unique_ptr<Image> filter_reconstruction_by_dilation(const Image& img1, const Image& img2, bool fully_connected) {
    return std::make_unique<Image>(itk::simple::ReconstructionByDilation(img1, img2, fully_connected));
}

std::unique_ptr<Image> filter_reconstruction_by_erosion(const Image& img1, const Image& img2, bool fully_connected) {
    return std::make_unique<Image>(itk::simple::ReconstructionByErosion(img1, img2, fully_connected));
}

std::unique_ptr<Image> filter_squared_difference(const Image& img1, const Image& img2) {
    return std::make_unique<Image>(itk::simple::SquaredDifference(img1, img2));
}

std::unique_ptr<Image> filter_subtract(const Image& img1, const Image& img2) {
    return std::make_unique<Image>(itk::simple::Subtract(img1, img2));
}

std::unique_ptr<Image> filter_xor(const Image& img1, const Image& img2) {
    return std::make_unique<Image>(itk::simple::Xor(img1, img2));
}

// ── New filters ────────────────────────────────────────────────────────────

// Unary (no params)
std::unique_ptr<Image> filter_sobel_edge_detection(const Image& img) {
    return std::make_unique<Image>(itk::simple::SobelEdgeDetection(img));
}

std::unique_ptr<Image> filter_forward_fft(const Image& img) {
    return std::make_unique<Image>(itk::simple::ForwardFFT(img));
}

std::unique_ptr<Image> filter_inverse_fft(const Image& img) {
    return std::make_unique<Image>(itk::simple::InverseFFT(img));
}

std::unique_ptr<Image> filter_real_to_half_hermitian_forward_fft(const Image& img) {
    return std::make_unique<Image>(itk::simple::RealToHalfHermitianForwardFFT(img));
}

std::unique_ptr<Image> filter_toboggan(const Image& img) {
    return std::make_unique<Image>(itk::simple::Toboggan(img));
}

std::unique_ptr<Image> filter_label_map_to_label(const Image& img) {
    return std::make_unique<Image>(itk::simple::LabelMapToLabel(img));
}

// Unary with simple params
std::unique_ptr<Image> filter_derivative(const Image& img, uint32_t direction, uint32_t order, bool use_image_spacing) {
    return std::make_unique<Image>(itk::simple::Derivative(img, direction, order, use_image_spacing));
}

std::unique_ptr<Image> filter_half_hermitian_to_real_inverse_fft(const Image& img, bool actual_x_dimension_is_odd) {
    return std::make_unique<Image>(itk::simple::HalfHermitianToRealInverseFFT(img, actual_x_dimension_is_odd));
}

std::unique_ptr<Image> filter_fft_pad(const Image& img, int32_t boundary_condition, int32_t size_greatest_prime_factor) {
    return std::make_unique<Image>(itk::simple::FFTPad(img,
        static_cast<itk::simple::FFTPadImageFilter::BoundaryConditionType>(boundary_condition),
        size_greatest_prime_factor));
}

std::unique_ptr<Image> filter_vector_connected_component(const Image& img, double distance_threshold, bool fully_connected) {
    return std::make_unique<Image>(itk::simple::VectorConnectedComponent(img, distance_threshold, fully_connected));
}

std::unique_ptr<Image> filter_label_map_to_binary(const Image& img, double background_value, double foreground_value) {
    return std::make_unique<Image>(itk::simple::LabelMapToBinary(img, background_value, foreground_value));
}

std::unique_ptr<Image> filter_objectness_measure(const Image& img, double alpha, double beta, double gamma, bool scale_objectness_measure, uint32_t object_dimension, bool bright_object) {
    return std::make_unique<Image>(itk::simple::ObjectnessMeasure(img, alpha, beta, gamma, scale_objectness_measure, object_dimension, bright_object));
}

std::unique_ptr<Image> filter_threshold_maximum_connected_components(const Image& img, uint32_t minimum_object_size_in_pixels, double upper_boundary, uint8_t inside_value, uint8_t outside_value) {
    return std::make_unique<Image>(itk::simple::ThresholdMaximumConnectedComponents(img, minimum_object_size_in_pixels, upper_boundary, inside_value, outside_value));
}

std::unique_ptr<Image> filter_gradient(const Image& img, bool use_image_spacing, bool use_image_direction) {
    return std::make_unique<Image>(itk::simple::Gradient(img, use_image_spacing, use_image_direction));
}

std::unique_ptr<Image> filter_gradient_recursive_gaussian(const Image& img, double sigma, bool normalize_across_scale, bool use_image_direction) {
    return std::make_unique<Image>(itk::simple::GradientRecursiveGaussian(img, sigma, normalize_across_scale, use_image_direction));
}

std::unique_ptr<Image> filter_dicom_orient(const Image& img, rust::Str orientation) {
    return std::make_unique<Image>(itk::simple::DICOMOrient(img, std::string(orientation.data(), orientation.size())));
}

std::unique_ptr<Image> filter_scalar_connected_component(const Image& img, double distance_threshold, bool fully_connected) {
    return std::make_unique<Image>(itk::simple::ScalarConnectedComponent(img, distance_threshold, fully_connected));
}

std::unique_ptr<Image> filter_scalar_to_rgb_colormap(const Image& img, int32_t colormap, bool use_input_image_extrema_for_scaling) {
    return std::make_unique<Image>(itk::simple::ScalarToRGBColormap(img,
        static_cast<itk::simple::ScalarToRGBColormapImageFilter::ColormapType>(colormap),
        use_input_image_extrema_for_scaling));
}

std::unique_ptr<Image> filter_staple(const Image& img, double confidence_weight, double foreground_value, uint32_t maximum_iterations) {
    return std::make_unique<Image>(itk::simple::STAPLE(img, confidence_weight, foreground_value, maximum_iterations));
}

std::unique_ptr<Image> filter_multi_label_staple(const Image& img, float termination_threshold, uint32_t max_iterations) {
    return std::make_unique<Image>(itk::simple::MultiLabelSTAPLE(img,
        std::numeric_limits<uint64_t>::max(), termination_threshold, max_iterations,
        std::vector<float>()));
}

// Unary with vector params
std::unique_ptr<Image> filter_grayscale_connected_closing(const Image& img, rust::Slice<const uint32_t> seed, bool fully_connected) {
    std::vector<unsigned int> s(seed.begin(), seed.end());
    return std::make_unique<Image>(itk::simple::GrayscaleConnectedClosing(img, s, fully_connected));
}

std::unique_ptr<Image> filter_grayscale_connected_opening(const Image& img, rust::Slice<const uint32_t> seed, bool fully_connected) {
    std::vector<unsigned int> s(seed.begin(), seed.end());
    return std::make_unique<Image>(itk::simple::GrayscaleConnectedOpening(img, s, fully_connected));
}

std::unique_ptr<Image> filter_permute_axes(const Image& img, rust::Slice<const uint32_t> order) {
    std::vector<unsigned int> o(order.begin(), order.end());
    return std::make_unique<Image>(itk::simple::PermuteAxes(img, o));
}

std::unique_ptr<Image> filter_slic(const Image& img, rust::Slice<const uint32_t> super_grid_size, double spatial_proximity_weight, uint32_t maximum_number_of_iterations, bool enforce_connectivity, bool initialization_perturbation) {
    std::vector<unsigned int> sgs(super_grid_size.begin(), super_grid_size.end());
    return std::make_unique<Image>(itk::simple::SLIC(img, sgs, spatial_proximity_weight, maximum_number_of_iterations, enforce_connectivity, initialization_perturbation));
}

std::unique_ptr<Image> filter_region_of_interest(const Image& img, rust::Slice<const uint32_t> size, rust::Slice<const int32_t> index) {
    std::vector<unsigned int> sz(size.begin(), size.end());
    std::vector<int> idx(index.begin(), index.end());
    return std::make_unique<Image>(itk::simple::RegionOfInterest(img, sz, idx));
}

std::unique_ptr<Image> filter_simple_contour_extractor(const Image& img, double input_foreground_value, double input_background_value, rust::Slice<const uint32_t> radius, double output_foreground_value, double output_background_value) {
    std::vector<unsigned int> r(radius.begin(), radius.end());
    return std::make_unique<Image>(itk::simple::SimpleContourExtractor(img, input_foreground_value, input_background_value, r, output_foreground_value, output_background_value));
}

std::unique_ptr<Image> filter_slice(const Image& img, rust::Slice<const int32_t> start, rust::Slice<const int32_t> stop, rust::Slice<const int32_t> step) {
    std::vector<int> sta(start.begin(), start.end());
    std::vector<int> sto(stop.begin(), stop.end());
    std::vector<int> ste(step.begin(), step.end());
    return std::make_unique<Image>(itk::simple::Slice(img, sta, sto, ste));
}

std::unique_ptr<Image> filter_expand(const Image& img, rust::Slice<const uint32_t> expand_factors, int32_t interpolator) {
    std::vector<unsigned int> ef(expand_factors.begin(), expand_factors.end());
    return std::make_unique<Image>(itk::simple::Expand(img, ef,
        static_cast<itk::simple::InterpolatorEnum>(interpolator)));
}

std::unique_ptr<Image> filter_stochastic_fractal_dimension(const Image& img, rust::Slice<const uint32_t> neighborhood_radius) {
    std::vector<unsigned int> r(neighborhood_radius.begin(), neighborhood_radius.end());
    return std::make_unique<Image>(itk::simple::StochasticFractalDimension(img, r));
}

std::unique_ptr<Image> filter_scalar_image_kmeans(const Image& img, rust::Slice<const double> class_means, bool use_non_contiguous_labels) {
    std::vector<double> means(class_means.begin(), class_means.end());
    return std::make_unique<Image>(itk::simple::ScalarImageKmeans(img, means, use_non_contiguous_labels));
}

std::unique_ptr<Image> filter_n4_bias_field_correction(const Image& img, double convergence_threshold, rust::Slice<const uint32_t> max_iterations, uint32_t num_histogram_bins, rust::Slice<const uint32_t> num_control_points, uint32_t spline_order) {
    std::vector<uint32_t> mi(max_iterations.begin(), max_iterations.end());
    std::vector<uint32_t> ncp(num_control_points.begin(), num_control_points.end());
    return std::make_unique<Image>(itk::simple::N4BiasFieldCorrection(img, convergence_threshold, mi, 0.15, 0.01, num_histogram_bins, ncp, spline_order));
}

// Binary filters (two-image)
std::unique_ptr<Image> filter_convolution(const Image& img, const Image& kernel, bool normalize, int32_t boundary_condition, int32_t output_region_mode) {
    return std::make_unique<Image>(itk::simple::Convolution(img, kernel, normalize,
        static_cast<itk::simple::ConvolutionImageFilter::BoundaryConditionType>(boundary_condition),
        static_cast<itk::simple::ConvolutionImageFilter::OutputRegionModeType>(output_region_mode)));
}

std::unique_ptr<Image> filter_fft_convolution(const Image& img, const Image& kernel, bool normalize, int32_t boundary_condition, int32_t output_region_mode) {
    return std::make_unique<Image>(itk::simple::FFTConvolution(img, kernel, normalize,
        static_cast<itk::simple::FFTConvolutionImageFilter::BoundaryConditionType>(boundary_condition),
        static_cast<itk::simple::FFTConvolutionImageFilter::OutputRegionModeType>(output_region_mode)));
}

std::unique_ptr<Image> filter_label_overlay(const Image& img, const Image& label, double opacity, double background_value) {
    return std::make_unique<Image>(itk::simple::LabelOverlay(img, label, opacity, background_value, std::vector<uint8_t>()));
}

std::unique_ptr<Image> filter_inverse_deconvolution(const Image& img, const Image& kernel, double kernel_zero_magnitude_threshold, bool normalize, int32_t boundary_condition, int32_t output_region_mode) {
    return std::make_unique<Image>(itk::simple::InverseDeconvolution(img, kernel, kernel_zero_magnitude_threshold, normalize,
        static_cast<itk::simple::InverseDeconvolutionImageFilter::BoundaryConditionType>(boundary_condition),
        static_cast<itk::simple::InverseDeconvolutionImageFilter::OutputRegionModeType>(output_region_mode)));
}

std::unique_ptr<Image> filter_tikhonov_deconvolution(const Image& img, const Image& kernel, double regularization_constant, bool normalize, int32_t boundary_condition, int32_t output_region_mode) {
    return std::make_unique<Image>(itk::simple::TikhonovDeconvolution(img, kernel, regularization_constant, normalize,
        static_cast<itk::simple::TikhonovDeconvolutionImageFilter::BoundaryConditionType>(boundary_condition),
        static_cast<itk::simple::TikhonovDeconvolutionImageFilter::OutputRegionModeType>(output_region_mode)));
}

std::unique_ptr<Image> filter_wiener_deconvolution(const Image& img, const Image& kernel, double noise_variance, bool normalize, int32_t boundary_condition, int32_t output_region_mode) {
    return std::make_unique<Image>(itk::simple::WienerDeconvolution(img, kernel, noise_variance, normalize,
        static_cast<itk::simple::WienerDeconvolutionImageFilter::BoundaryConditionType>(boundary_condition),
        static_cast<itk::simple::WienerDeconvolutionImageFilter::OutputRegionModeType>(output_region_mode)));
}

std::unique_ptr<Image> filter_richardson_lucy(const Image& img, const Image& kernel, int32_t number_of_iterations, bool normalize, int32_t boundary_condition, int32_t output_region_mode) {
    return std::make_unique<Image>(itk::simple::RichardsonLucyDeconvolution(img, kernel, number_of_iterations, normalize,
        static_cast<itk::simple::RichardsonLucyDeconvolutionImageFilter::BoundaryConditionType>(boundary_condition),
        static_cast<itk::simple::RichardsonLucyDeconvolutionImageFilter::OutputRegionModeType>(output_region_mode)));
}

std::unique_ptr<Image> filter_landweber_deconvolution(const Image& img, const Image& kernel, double alpha, int32_t number_of_iterations, bool normalize, int32_t boundary_condition, int32_t output_region_mode) {
    return std::make_unique<Image>(itk::simple::LandweberDeconvolution(img, kernel, alpha, number_of_iterations, normalize,
        static_cast<itk::simple::LandweberDeconvolutionImageFilter::BoundaryConditionType>(boundary_condition),
        static_cast<itk::simple::LandweberDeconvolutionImageFilter::OutputRegionModeType>(output_region_mode)));
}

std::unique_ptr<Image> filter_projected_landweber_deconvolution(const Image& img, const Image& kernel, double alpha, int32_t number_of_iterations, bool normalize, int32_t boundary_condition, int32_t output_region_mode) {
    return std::make_unique<Image>(itk::simple::ProjectedLandweberDeconvolution(img, kernel, alpha, number_of_iterations, normalize,
        static_cast<itk::simple::ProjectedLandweberDeconvolutionImageFilter::BoundaryConditionType>(boundary_condition),
        static_cast<itk::simple::ProjectedLandweberDeconvolutionImageFilter::OutputRegionModeType>(output_region_mode)));
}

std::unique_ptr<Image> filter_fft_normalized_correlation(const Image& fixed_image, const Image& moving_image, uint64_t required_number_of_overlapping_pixels, double required_fraction_of_overlapping_pixels) {
    return std::make_unique<Image>(itk::simple::FFTNormalizedCorrelation(fixed_image, moving_image, required_number_of_overlapping_pixels, required_fraction_of_overlapping_pixels));
}

// Three-image filters
std::unique_ptr<Image> filter_ternary_add(const Image& img1, const Image& img2, const Image& img3) {
    return std::make_unique<Image>(itk::simple::TernaryAdd(img1, img2, img3));
}

std::unique_ptr<Image> filter_ternary_magnitude(const Image& img1, const Image& img2, const Image& img3) {
    return std::make_unique<Image>(itk::simple::TernaryMagnitude(img1, img2, img3));
}

std::unique_ptr<Image> filter_ternary_magnitude_squared(const Image& img1, const Image& img2, const Image& img3) {
    return std::make_unique<Image>(itk::simple::TernaryMagnitudeSquared(img1, img2, img3));
}

std::unique_ptr<Image> filter_masked_fft_normalized_correlation(const Image& fixed_image, const Image& moving_image, const Image& fixed_mask, const Image& moving_mask, uint64_t required_overlapping_pixels, float required_fraction) {
    return std::make_unique<Image>(itk::simple::MaskedFFTNormalizedCorrelation(fixed_image, moving_image, fixed_mask, moving_mask, required_overlapping_pixels, required_fraction));
}

// NeighborhoodConnected (class API, no seeds)
std::unique_ptr<Image> filter_neighborhood_connected(const Image& img, double lower, double upper, rust::Slice<const uint32_t> radius, double replace_value) {
    auto f = itk::simple::NeighborhoodConnectedImageFilter();
    f.SetLower(lower);
    f.SetUpper(upper);
    std::vector<uint32_t> r(radius.begin(), radius.end());
    f.SetRadius(r);
    f.SetReplaceValue(replace_value);
    return std::make_unique<Image>(f.Execute(img));
}

// ── Group 1: Binary image+image ────────────────────────────────────────────

std::unique_ptr<Image> filter_maximum(const Image& img1, const Image& img2) {
    return std::make_unique<Image>(itk::simple::Maximum(img1, img2));
}

std::unique_ptr<Image> filter_minimum(const Image& img1, const Image& img2) {
    return std::make_unique<Image>(itk::simple::Minimum(img1, img2));
}

std::unique_ptr<Image> filter_modulus(const Image& img1, const Image& img2) {
    return std::make_unique<Image>(itk::simple::Modulus(img1, img2));
}

// ── Group 2: Unary with simple scalar/bool params ──────────────────────────

std::unique_ptr<Image> filter_laplacian(const Image& img, bool use_image_spacing) {
    return std::make_unique<Image>(itk::simple::Laplacian(img, use_image_spacing));
}

std::unique_ptr<Image> filter_bspline_decomposition(const Image& img, uint32_t spline_order) {
    return std::make_unique<Image>(itk::simple::BSplineDecomposition(img, spline_order));
}

std::unique_ptr<Image> filter_relabel_label_map(const Image& img, bool reverse_ordering) {
    return std::make_unique<Image>(itk::simple::RelabelLabelMap(img, reverse_ordering));
}

std::unique_ptr<Image> filter_label_unique_label_map(const Image& img, bool reverse_ordering) {
    return std::make_unique<Image>(itk::simple::LabelUniqueLabelMap(img, reverse_ordering));
}

std::unique_ptr<Image> filter_label_to_rgb(const Image& img, double background_value) {
    return std::make_unique<Image>(itk::simple::LabelToRGB(img, background_value, std::vector<uint8_t>()));
}

std::unique_ptr<Image> filter_label_voting(const Image& img) {
    return std::make_unique<Image>(itk::simple::LabelVoting(img, std::numeric_limits<uint64_t>::max()));
}

std::unique_ptr<Image> filter_binary_image_to_label_map(const Image& img, bool fully_connected, double input_foreground_value, double output_background_value) {
    return std::make_unique<Image>(itk::simple::BinaryImageToLabelMap(img, fully_connected, input_foreground_value, output_background_value));
}

std::unique_ptr<Image> filter_label_image_to_label_map(const Image& img, double background_value) {
    return std::make_unique<Image>(itk::simple::LabelImageToLabelMap(img, background_value));
}

std::unique_ptr<Image> filter_min_max_curvature_flow(const Image& img, double time_step, uint32_t number_of_iterations, int32_t stencil_radius) {
    return std::make_unique<Image>(itk::simple::MinMaxCurvatureFlow(img, time_step, number_of_iterations, stencil_radius));
}

std::unique_ptr<Image> filter_fast_marching(const Image& img, double normalization_factor, double stopping_value) {
    auto f = itk::simple::FastMarchingImageFilter();
    f.SetNormalizationFactor(normalization_factor);
    f.SetStoppingValue(stopping_value);
    return std::make_unique<Image>(f.Execute(img));
}

std::unique_ptr<Image> filter_connected_threshold(const Image& img, double lower, double upper, uint8_t replace_value, int32_t connectivity) {
    auto f = itk::simple::ConnectedThresholdImageFilter();
    f.SetLower(lower);
    f.SetUpper(upper);
    f.SetReplaceValue(replace_value);
    f.SetConnectivity(static_cast<itk::simple::ConnectedThresholdImageFilter::ConnectivityType>(connectivity));
    return std::make_unique<Image>(f.Execute(img));
}

// ── Group 3: Unary with vector params ──────────────────────────────────────

std::unique_ptr<Image> filter_crop(const Image& img, rust::Slice<const uint32_t> lower_size, rust::Slice<const uint32_t> upper_size) {
    std::vector<unsigned int> lo(lower_size.begin(), lower_size.end());
    std::vector<unsigned int> hi(upper_size.begin(), upper_size.end());
    return std::make_unique<Image>(itk::simple::Crop(img, lo, hi));
}

std::unique_ptr<Image> filter_tile(const Image& img, rust::Slice<const uint32_t> layout, double default_pixel_value) {
    std::vector<unsigned int> lay(layout.begin(), layout.end());
    return std::make_unique<Image>(itk::simple::Tile(img, lay, default_pixel_value));
}

std::unique_ptr<Image> filter_compose(const Image& img) {
    return std::make_unique<Image>(itk::simple::Compose(img));
}

// ── Group 4: Two-image with params ─────────────────────────────────────────

std::unique_ptr<Image> filter_masked_assign(const Image& img, const Image& mask, double assign_constant) {
    return std::make_unique<Image>(itk::simple::MaskedAssign(img, mask, assign_constant));
}

std::unique_ptr<Image> filter_label_map_overlay(const Image& label_map, const Image& feature_image, double opacity) {
    return std::make_unique<Image>(itk::simple::LabelMapOverlay(label_map, feature_image, opacity, std::vector<uint8_t>()));
}

std::unique_ptr<Image> filter_label_map_mask(const Image& label_map, const Image& feature_image, uint64_t label, double background_value, bool negated, bool crop) {
    return std::make_unique<Image>(itk::simple::LabelMapMask(label_map, feature_image, label, background_value, negated, crop, std::vector<uint32_t>()));
}

std::unique_ptr<Image> filter_join_series_two(const Image& img1, const Image& img2, double origin, double spacing) {
    std::vector<itk::simple::Image> images = {img1, img2};
    return std::make_unique<Image>(itk::simple::JoinSeries(images, origin, spacing));
}

std::unique_ptr<Image> filter_isolated_connected(const Image& img, rust::Slice<const uint32_t> seed1, rust::Slice<const uint32_t> seed2, double lower, double upper, uint8_t replace_value, double tolerance, bool find_upper) {
    std::vector<unsigned int> s1(seed1.begin(), seed1.end());
    std::vector<unsigned int> s2(seed2.begin(), seed2.end());
    return std::make_unique<Image>(itk::simple::IsolatedConnected(img, s1, s2, lower, upper, replace_value, tolerance, find_upper));
}

std::unique_ptr<Image> filter_isolated_watershed(const Image& img, rust::Slice<const uint32_t> seed1, rust::Slice<const uint32_t> seed2, double threshold, double upper_value_limit, double tolerance, uint8_t replace1, uint8_t replace2) {
    std::vector<unsigned int> s1(seed1.begin(), seed1.end());
    std::vector<unsigned int> s2(seed2.begin(), seed2.end());
    return std::make_unique<Image>(itk::simple::IsolatedWatershed(img, s1, s2, threshold, upper_value_limit, tolerance, replace1, replace2));
}

// ── Group 5: Three-image ───────────────────────────────────────────────────

std::unique_ptr<Image> filter_normalized_correlation(const Image& image, const Image& mask_image, const Image& template_image) {
    return std::make_unique<Image>(itk::simple::NormalizedCorrelation(image, mask_image, template_image));
}

std::unique_ptr<Image> filter_label_map_contour_overlay(const Image& label_map, const Image& feature_image, double opacity, rust::Slice<const uint32_t> dilation_radius, rust::Slice<const uint32_t> contour_thickness, uint32_t slice_dimension, int32_t contour_type, int32_t priority) {
    std::vector<unsigned int> dr(dilation_radius.begin(), dilation_radius.end());
    std::vector<unsigned int> ct(contour_thickness.begin(), contour_thickness.end());
    return std::make_unique<Image>(itk::simple::LabelMapContourOverlay(label_map, feature_image, opacity, dr, ct, slice_dimension,
        static_cast<itk::simple::LabelMapContourOverlayImageFilter::ContourTypeType>(contour_type),
        static_cast<itk::simple::LabelMapContourOverlayImageFilter::PriorityType>(priority),
        std::vector<uint8_t>()));
}

std::unique_ptr<Image> filter_merge_label_map_two(const Image& img1, const Image& img2, int32_t method) {
    std::vector<itk::simple::Image> images = {img1, img2};
    return std::make_unique<Image>(itk::simple::MergeLabelMap(images,
        static_cast<itk::simple::MergeLabelMapFilter::MethodType>(method)));
}

// ── Group 6: Image sources (no Image input) ────────────────────────────────

std::unique_ptr<Image> source_gaussian(int32_t pixel_type, rust::Slice<const uint32_t> size, rust::Slice<const double> sigma, rust::Slice<const double> mean, double scale) {
    std::vector<uint32_t> sz(size.begin(), size.end());
    std::vector<double> sg(sigma.begin(), sigma.end());
    std::vector<double> mn(mean.begin(), mean.end());
    return std::make_unique<Image>(itk::simple::GaussianSource(
        static_cast<itk::simple::PixelIDValueEnum>(pixel_type),
        sz, sg, mn, scale));
}

std::unique_ptr<Image> source_gabor(int32_t pixel_type, rust::Slice<const uint32_t> size, rust::Slice<const double> sigma, rust::Slice<const double> mean, double frequency) {
    std::vector<uint32_t> sz(size.begin(), size.end());
    std::vector<double> sg(sigma.begin(), sigma.end());
    std::vector<double> mn(mean.begin(), mean.end());
    return std::make_unique<Image>(itk::simple::GaborSource(
        static_cast<itk::simple::PixelIDValueEnum>(pixel_type),
        sz, sg, mn, frequency));
}

std::unique_ptr<Image> source_grid(int32_t pixel_type, rust::Slice<const uint32_t> size, rust::Slice<const double> sigma, rust::Slice<const double> grid_spacing, rust::Slice<const double> grid_offset, double scale) {
    std::vector<uint32_t> sz(size.begin(), size.end());
    std::vector<double> sg(sigma.begin(), sigma.end());
    std::vector<double> gs(grid_spacing.begin(), grid_spacing.end());
    std::vector<double> go(grid_offset.begin(), grid_offset.end());
    return std::make_unique<Image>(itk::simple::GridSource(
        static_cast<itk::simple::PixelIDValueEnum>(pixel_type),
        sz, sg, gs, go, scale));
}

std::unique_ptr<Image> source_physical_point(int32_t pixel_type, rust::Slice<const uint32_t> size) {
    std::vector<uint32_t> sz(size.begin(), size.end());
    return std::make_unique<Image>(itk::simple::PhysicalPointSource(
        static_cast<itk::simple::PixelIDValueEnum>(pixel_type), sz));
}

// ── Final batch ────────────────────────────────────────────────────────────
std::unique_ptr<Image> filter_aggregate_label_map(const Image& img) {
    return std::make_unique<Image>(itk::simple::AggregateLabelMap(img));
}

std::unique_ptr<Image> filter_label_map_to_rgb(const Image& img) {
    return std::make_unique<Image>(itk::simple::LabelMapToRGB(img, std::vector<uint8_t>()));
}

std::unique_ptr<Image> filter_extract(const Image& img, rust::Slice<const uint32_t> size, rust::Slice<const int32_t> index) {
    auto f = itk::simple::ExtractImageFilter();
    f.SetSize(std::vector<unsigned int>(size.begin(), size.end()));
    f.SetIndex(std::vector<int>(index.begin(), index.end()));
    return std::make_unique<Image>(f.Execute(img));
}

std::unique_ptr<Image> filter_nary_add_two(const Image& img1, const Image& img2) {
    return std::make_unique<Image>(itk::simple::NaryAdd(std::vector<itk::simple::Image>{img1, img2}));
}

std::unique_ptr<Image> filter_nary_maximum_two(const Image& img1, const Image& img2) {
    return std::make_unique<Image>(itk::simple::NaryMaximum(std::vector<itk::simple::Image>{img1, img2}));
}

} // namespace sitk_rs

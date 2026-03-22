use simpleitk::{io, filters};

fn main() {
    let image = io::read_image("henlab.jpg").expect("failed to read image");
    println!("Input: {image:?}  pixel_id={}", image.pixel_id());

    // Gaussian smoothing (TIFF supports float)
    let smoothed = filters::smoothing_recursive_gaussian(&image, &[3.0, 3.0], false)
        .expect("smoothing failed");
    println!("After SmoothingRecursiveGaussian(sigma=3): {smoothed:?}  pixel_id={}", smoothed.pixel_id());
    io::write_image(&smoothed, "/tmp/henlab_smooth.tiff").expect("write smooth failed");

    // Binary threshold — input must be scalar; extract channel 0 first via VectorIndexSelectionCast
    let gray = filters::vector_index_selection_cast(&image, 0, -1)
        .expect("channel extract failed");
    println!("Grayscale (channel 0): {gray:?}  pixel_id={}", gray.pixel_id());

    let thresholded = filters::binary_threshold(&gray, 100.0, 200.0, 255, 0)
        .expect("threshold failed");
    println!("After BinaryThreshold(100..200): {thresholded:?}");
    io::write_image(&thresholded, "/tmp/henlab_thresh.png").expect("write thresh failed");

    // Canny edge detection on grayscale float
    let gray_f = filters::cast(&gray, 9).expect("cast to float32 failed"); // 9 = sitkFloat32
    // args: lower_threshold, upper_threshold, variance, maximum_error
    let edges = filters::canny_edge_detection(&gray_f, 50.0, 150.0, 2.0, 0.01)
        .expect("canny failed");
    println!("After CannyEdgeDetection: {edges:?}  pixel_id={}", edges.pixel_id());
    let edges_u8 = filters::cast(&edges, 1).expect("cast edges failed"); // 1 = sitkUInt8
    io::write_image(&edges_u8, "/tmp/henlab_edges.png").expect("write edges failed");

    println!("\nOK — written to /tmp/henlab_smooth.tiff, /tmp/henlab_thresh.png, /tmp/henlab_edges.png");
}

# simpleitk-rs

Rust bindings for [SimpleITK](https://simpleitk.org/) — a high-level image processing library built on ITK, widely used in medical imaging.

## Overview

The workspace contains two crates:

| Crate | Purpose |
|-------|---------|
| `simpleitk-sys` | Low-level CXX bridge to the `itk::simple` C++ API |
| `simpleitk` | Safe, idiomatic Rust wrapper (public API) |

Bindings are generated at build time using the [`cxx`](https://cxx.rs) crate, which provides safe interop with C++ without requiring a hand-written C API layer.

## Requirements

- Rust 1.75+
- CMake 3.20+
- A C++17-capable compiler (AppleClang, GCC, or MSVC)
- Internet access on first build (downloads SimpleITK 2.4.0 ~300 MB)

On macOS, Xcode Command Line Tools are sufficient.

## Usage

Add to your `Cargo.toml`:

```toml
[dependencies]
simpleitk = { path = "path/to/simpleitk-rs/simpleitk" }
```

```rust
use simpleitk::io;

fn main() {
    let image = io::read_image("input.nii").expect("failed to read");
    println!("{}x{}x{} ({}D)", image.width(), image.height(), image.depth(), image.dimension());
    io::write_image(&image, "output.nii.gz").expect("failed to write");
}
```

## Building

### Automatic (vendored build)

The first build downloads and compiles SimpleITK 2.4.0 from source. This takes 30–60 minutes but is fully automatic:

```sh
cargo build
```

### Using a pre-installed SimpleITK

Point `SIMPLEITK_DIR` at an existing install prefix to skip the vendored build:

```sh
SIMPLEITK_DIR=/path/to/simpleitk/install cargo build
```

The install prefix must contain `lib/libSimpleITK*.a` and `include/SimpleITK*/SimpleITK.h`. Both the SimpleITK and ITK static libraries must be present under `lib/`.

### Running the example

```sh
cargo run --example read_write -- input.jpg output.png
```

## API

### `simpleitk::io`

```rust
// Read any format SimpleITK supports (DICOM, NIfTI, MetaImage, PNG, TIFF, JPEG, ...)
pub fn read_image(path: &str) -> Result<Image, String>

// Write — format inferred from file extension
pub fn write_image(image: &Image, path: &str) -> Result<(), String>
```

### `simpleitk::Image`

```rust
pub fn width(&self)      -> u32
pub fn height(&self)     -> u32
pub fn depth(&self)      -> u32   // 0 for 2D images
pub fn dimension(&self)  -> u32   // 2 or 3
pub fn num_pixels(&self) -> u64
pub fn pixel_id(&self)   -> i32   // itk::simple::PixelIDValueEnum
```

`Image` implements `Debug`, `Send`, and drops the underlying C++ object automatically.

## Architecture

```
simpleitk-sys/
├── src/
│   ├── lib.rs            # #[cxx::bridge] — declares the FFI interface
│   ├── sitk_bridge.h     # C++ glue declarations (sitk_rs namespace)
│   └── sitk_bridge.cpp   # C++ glue implementations
├── build.rs              # Locates/builds SimpleITK; drives cxx_build
└── CMakeLists.txt        # Vendored SimpleITK 2.4.0 ExternalProject

simpleitk/
├── src/
│   ├── image.rs          # Image struct wrapping cxx::UniquePtr<ffi::Image>
│   ├── io.rs             # read_image / write_image
│   └── lib.rs
└── examples/
    └── read_write.rs
```

The `cxx` bridge wraps `itk::simple::Image` as an opaque type accessed through `UniquePtr<Image>`. Drop, move semantics, and exception propagation are all handled by the bridge automatically.

## Expanding Coverage

After a successful build, the installed SimpleITK headers are at:

```
target/*/build/simpleitk-sys-*/out/simpleitk_cmake_build/install/include/SimpleITK-2.4/
```

To add a new function:

1. Declare it in `simpleitk-sys/src/sitk_bridge.h` (in the `sitk_rs` namespace)
2. Implement it in `simpleitk-sys/src/sitk_bridge.cpp`
3. Add the signature to the `#[cxx::bridge]` in `simpleitk-sys/src/lib.rs`
4. Expose a safe wrapper in `simpleitk/src/`

## Platform Notes

**macOS 26.x (Sequoia/Tahoe beta):** The Command Line Tools ship a split libc++ where `cxxabi.h` lives inside the SDK rather than the compiler's built-in include path. `build.rs` detects this automatically and passes `-nostdinc++ -isystem <SDK>/usr/include/c++/v1` to the C++ compiler.

## License

Apache 2.0 — same as SimpleITK and ITK.

---

## Feature Coverage

Status of SimpleITK 2.4.0 operations in this library.

### IO & Image

| Operation | Status |
|-----------|--------|
| `read_image` | ✅ Implemented |
| `write_image` | ✅ Implemented |
| `Image::width` | ✅ Implemented |
| `Image::height` | ✅ Implemented |
| `Image::depth` | ✅ Implemented |
| `Image::dimension` | ✅ Implemented |
| `Image::num_pixels` | ✅ Implemented |
| `Image::pixel_id` | ✅ Implemented |
| `ImageFileReader` (class) | ❌ Not implemented |
| `ImageFileWriter` (class) | ❌ Not implemented |
| `ImageSeriesReader` | ❌ Not implemented |
| `ImageSeriesWriter` | ❌ Not implemented |

### Filters

| Operation | Status |
|-----------|--------|
| `AbsImageFilter` | ✅ Implemented |
| `AbsoluteValueDifferenceImageFilter` | ✅ Implemented |
| `AcosImageFilter` | ✅ Implemented |
| `AdaptiveHistogramEqualizationImageFilter` | ✅ Implemented |
| `AddImageFilter` | ✅ Implemented |
| `AdditiveGaussianNoiseImageFilter` | ✅ Implemented |
| `AggregateLabelMapFilter` | ✅ Implemented |
| `AndImageFilter` | ✅ Implemented |
| `AntiAliasBinaryImageFilter` | ✅ Implemented |
| `ApproximateSignedDistanceMapImageFilter` | ✅ Implemented |
| `AreaClosingImageFilter` | ✅ Implemented |
| `AreaOpeningImageFilter` | ✅ Implemented |
| `AsinImageFilter` | ✅ Implemented |
| `AtanImageFilter` | ✅ Implemented |
| `Atan2ImageFilter` | ✅ Implemented |
| `BSplineDecompositionImageFilter` | ✅ Implemented |
| `BSplineTransformInitializerFilter` | ❌ Not implemented |
| `BilateralImageFilter` | ✅ Implemented |
| `BinShrinkImageFilter` | ✅ Implemented |
| `BinaryClosingByReconstructionImageFilter` | ✅ Implemented |
| `BinaryContourImageFilter` | ✅ Implemented |
| `BinaryDilateImageFilter` | ✅ Implemented |
| `BinaryErodeImageFilter` | ✅ Implemented |
| `BinaryFillholeImageFilter` | ✅ Implemented |
| `BinaryGrindPeakImageFilter` | ✅ Implemented |
| `BinaryImageToLabelMapFilter` | ✅ Implemented |
| `BinaryMagnitudeImageFilter` | ✅ Implemented |
| `BinaryMedianImageFilter` | ✅ Implemented |
| `BinaryMinMaxCurvatureFlowImageFilter` | ✅ Implemented |
| `BinaryMorphologicalClosingImageFilter` | ✅ Implemented |
| `BinaryMorphologicalOpeningImageFilter` | ✅ Implemented |
| `BinaryNotImageFilter` | ✅ Implemented |
| `BinaryOpeningByReconstructionImageFilter` | ✅ Implemented |
| `BinaryProjectionImageFilter` | ✅ Implemented |
| `BinaryPruningImageFilter` | ✅ Implemented |
| `BinaryReconstructionByDilationImageFilter` | ✅ Implemented |
| `BinaryReconstructionByErosionImageFilter` | ✅ Implemented |
| `BinaryThinningImageFilter` | ✅ Implemented |
| `BinaryThresholdImageFilter` | ✅ Implemented |
| `BinaryThresholdProjectionImageFilter` | ✅ Implemented |
| `BinomialBlurImageFilter` | ✅ Implemented |
| `BitwiseNotImageFilter` | ✅ Implemented |
| `BlackTopHatImageFilter` | ✅ Implemented |
| `BoundedReciprocalImageFilter` | ✅ Implemented |
| `BoxMeanImageFilter` | ✅ Implemented |
| `BoxSigmaImageFilter` | ✅ Implemented |
| `CannyEdgeDetectionImageFilter` | ✅ Implemented |
| `CannySegmentationLevelSetImageFilter` | ✅ Implemented (`filters::canny_segmentation_level_set`) |
| `CastImageFilter` | ✅ Implemented |
| `CenteredTransformInitializerFilter` | ❌ Not implemented |
| `CenteredVersorTransformInitializerFilter` | ❌ Not implemented |
| `ChangeLabelImageFilter` | ❌ Not implemented (no simple C API for label map conversion) |
| `ChangeLabelLabelMapFilter` | ✅ Implemented (`filters::change_label_label_map`) |
| `CheckerBoardImageFilter` | ✅ Implemented |
| `ClampImageFilter` | ✅ Implemented |
| `ClosingByReconstructionImageFilter` | ✅ Implemented |
| `CollidingFrontsImageFilter` | ✅ Implemented (`filters::colliding_fronts`) |
| `ComplexToImaginaryImageFilter` | ✅ Implemented |
| `ComplexToModulusImageFilter` | ✅ Implemented |
| `ComplexToPhaseImageFilter` | ✅ Implemented |
| `ComplexToRealImageFilter` | ✅ Implemented |
| `ComposeImageFilter` | ✅ Implemented |
| `ConfidenceConnectedImageFilter` | ✅ Implemented (`filters::confidence_connected`) |
| `ConnectedComponentImageFilter` | ✅ Implemented |
| `ConnectedThresholdImageFilter` | ✅ Implemented |
| `ConstantPadImageFilter` | ✅ Implemented |
| `ConvolutionImageFilter` | ✅ Implemented |
| `CosImageFilter` | ✅ Implemented |
| `CropImageFilter` | ✅ Implemented |
| `CurvatureAnisotropicDiffusionImageFilter` | ✅ Implemented |
| `CurvatureFlowImageFilter` | ✅ Implemented |
| `CyclicShiftImageFilter` | ✅ Implemented |
| `DICOMOrientImageFilter` | ✅ Implemented |
| `DanielssonDistanceMapImageFilter` | ✅ Implemented |
| `DemonsRegistrationFilter` | ❌ Not implemented |
| `DerivativeImageFilter` | ✅ Implemented |
| `DiffeomorphicDemonsRegistrationFilter` | ❌ Not implemented |
| `DilateObjectMorphologyImageFilter` | ✅ Implemented |
| `DiscreteGaussianImageFilter` | ✅ Implemented |
| `DiscreteGaussianDerivativeImageFilter` | ✅ Implemented |
| `DisplacementFieldJacobianDeterminantFilter` | ✅ Implemented |
| `DivideImageFilter` | ✅ Implemented |
| `DivideFloorImageFilter` | ✅ Implemented |
| `DivideRealImageFilter` | ✅ Implemented |
| `DoubleThresholdImageFilter` | ✅ Implemented |
| `EdgePotentialImageFilter` | ✅ Implemented |
| `EqualImageFilter` | ✅ Implemented |
| `ErodeObjectMorphologyImageFilter` | ✅ Implemented |
| `ExpImageFilter` | ✅ Implemented |
| `ExpNegativeImageFilter` | ✅ Implemented |
| `ExpandImageFilter` | ✅ Implemented |
| `ExtractImageFilter` | ✅ Implemented |
| `FFTConvolutionImageFilter` | ✅ Implemented |
| `FFTNormalizedCorrelationImageFilter` | ✅ Implemented |
| `FFTPadImageFilter` | ✅ Implemented |
| `FFTShiftImageFilter` | ✅ Implemented |
| `FastApproximateRankImageFilter` | ✅ Implemented |
| `FastMarchingImageFilter` | ✅ Implemented |
| `FastMarchingUpwindGradientImageFilter` | ✅ Implemented (`filters::fast_marching_upwind_gradient`) |
| `FastSymmetricForcesDemonsRegistrationFilter` | ❌ Not implemented |
| `FlipImageFilter` | ✅ Implemented |
| `ForwardFFTImageFilter` | ✅ Implemented |
| `GeodesicActiveContourLevelSetImageFilter` | ✅ Implemented (`filters::geodesic_active_contour_level_set`) |
| `GradientImageFilter` | ✅ Implemented |
| `GradientAnisotropicDiffusionImageFilter` | ✅ Implemented |
| `GradientMagnitudeImageFilter` | ✅ Implemented |
| `GradientMagnitudeRecursiveGaussianImageFilter` | ✅ Implemented |
| `GradientRecursiveGaussianImageFilter` | ✅ Implemented |
| `GrayscaleConnectedClosingImageFilter` | ✅ Implemented |
| `GrayscaleConnectedOpeningImageFilter` | ✅ Implemented |
| `GrayscaleDilateImageFilter` | ✅ Implemented |
| `GrayscaleErodeImageFilter` | ✅ Implemented |
| `GrayscaleFillholeImageFilter` | ✅ Implemented |
| `GrayscaleGeodesicDilateImageFilter` | ✅ Implemented |
| `GrayscaleGeodesicErodeImageFilter` | ✅ Implemented |
| `GrayscaleGrindPeakImageFilter` | ✅ Implemented |
| `GrayscaleMorphologicalClosingImageFilter` | ✅ Implemented |
| `GrayscaleMorphologicalOpeningImageFilter` | ✅ Implemented |
| `GreaterImageFilter` | ✅ Implemented |
| `GreaterEqualImageFilter` | ✅ Implemented |
| `HConcaveImageFilter` | ✅ Implemented |
| `HConvexImageFilter` | ✅ Implemented |
| `HMaximaImageFilter` | ✅ Implemented |
| `HMinimaImageFilter` | ✅ Implemented |
| `HalfHermitianToRealInverseFFTImageFilter` | ✅ Implemented |
| `HashImageFilter` | ✅ Implemented (`measurements::hash`) |
| `HausdorffDistanceImageFilter` | ✅ Implemented (`measurements::hausdorff_distance`) |
| `HistogramMatchingImageFilter` | ✅ Implemented |
| `HuangThresholdImageFilter` | ✅ Implemented |
| `ImportImageFilter` | ❌ Not implemented |
| `IntensityWindowingImageFilter` | ✅ Implemented |
| `IntermodesThresholdImageFilter` | ✅ Implemented |
| `InverseDeconvolutionImageFilter` | ✅ Implemented |
| `InverseDisplacementFieldImageFilter` | ✅ Implemented (`filters::inverse_displacement_field`) |
| `InverseFFTImageFilter` | ✅ Implemented |
| `InvertDisplacementFieldImageFilter` | ✅ Implemented (`filters::invert_displacement_field`) |
| `InvertIntensityImageFilter` | ✅ Implemented |
| `IsoContourDistanceImageFilter` | ✅ Implemented |
| `IsoDataThresholdImageFilter` | ✅ Implemented |
| `IsolatedConnectedImageFilter` | ✅ Implemented |
| `IsolatedWatershedImageFilter` | ✅ Implemented |
| `IterativeInverseDisplacementFieldImageFilter` | ✅ Implemented (`filters::iterative_inverse_displacement_field`) |
| `JoinSeriesImageFilter` | ✅ Implemented |
| `KittlerIllingworthThresholdImageFilter` | ✅ Implemented |
| `LabelContourImageFilter` | ✅ Implemented |
| `LabelImageToLabelMapFilter` | ✅ Implemented |
| `LabelIntensityStatisticsImageFilter` | ✅ Implemented (`measurements::label_intensity_for_label`) |
| `LabelMapContourOverlayImageFilter` | ✅ Implemented |
| `LabelMapMaskImageFilter` | ✅ Implemented |
| `LabelMapOverlayImageFilter` | ✅ Implemented |
| `LabelMapToBinaryImageFilter` | ✅ Implemented |
| `LabelMapToLabelImageFilter` | ✅ Implemented |
| `LabelMapToRGBImageFilter` | ✅ Implemented |
| `LabelOverlapMeasuresImageFilter` | ✅ Implemented (`measurements::label_overlap`) |
| `LabelOverlayImageFilter` | ✅ Implemented |
| `LabelShapeStatisticsImageFilter` | ✅ Implemented (`measurements::label_shape_for_label`) |
| `LabelStatisticsImageFilter` | ✅ Implemented (`measurements::label_stats_for_label`) |
| `LabelToRGBImageFilter` | ✅ Implemented |
| `LabelUniqueLabelMapFilter` | ✅ Implemented |
| `LabelVotingImageFilter` | ✅ Implemented |
| `LandmarkBasedTransformInitializerFilter` | ❌ Not implemented |
| `LandweberDeconvolutionImageFilter` | ✅ Implemented |
| `LaplacianImageFilter` | ✅ Implemented |
| `LaplacianRecursiveGaussianImageFilter` | ✅ Implemented |
| `LaplacianSegmentationLevelSetImageFilter` | ✅ Implemented (`filters::laplacian_segmentation_level_set`) |
| `LaplacianSharpeningImageFilter` | ✅ Implemented |
| `LessImageFilter` | ✅ Implemented |
| `LessEqualImageFilter` | ✅ Implemented |
| `LevelSetMotionRegistrationFilter` | ❌ Not implemented |
| `LiThresholdImageFilter` | ✅ Implemented |
| `LogImageFilter` | ✅ Implemented |
| `Log10ImageFilter` | ✅ Implemented |
| `MagnitudeAndPhaseToComplexImageFilter` | ✅ Implemented |
| `MaskImageFilter` | ✅ Implemented |
| `MaskNegatedImageFilter` | ✅ Implemented |
| `MaskedAssignImageFilter` | ✅ Implemented |
| `MaskedFFTNormalizedCorrelationImageFilter` | ✅ Implemented |
| `MaximumImageFilter` | ✅ Implemented |
| `MaximumEntropyThresholdImageFilter` | ✅ Implemented |
| `MaximumProjectionImageFilter` | ✅ Implemented |
| `MeanImageFilter` | ✅ Implemented |
| `MeanProjectionImageFilter` | ✅ Implemented |
| `MedianImageFilter` | ✅ Implemented |
| `MedianProjectionImageFilter` | ✅ Implemented |
| `MergeLabelMapFilter` | ✅ Implemented |
| `MinMaxCurvatureFlowImageFilter` | ✅ Implemented |
| `MinimumImageFilter` | ✅ Implemented |
| `MinimumMaximumImageFilter` | ✅ Implemented (`measurements::min_max`) |
| `MinimumProjectionImageFilter` | ✅ Implemented |
| `MirrorPadImageFilter` | ✅ Implemented |
| `ModulusImageFilter` | ✅ Implemented |
| `MomentsThresholdImageFilter` | ✅ Implemented |
| `MorphologicalGradientImageFilter` | ✅ Implemented |
| `MorphologicalWatershedImageFilter` | ✅ Implemented |
| `MorphologicalWatershedFromMarkersImageFilter` | ✅ Implemented |
| `MultiLabelSTAPLEImageFilter` | ✅ Implemented |
| `MultiplyImageFilter` | ✅ Implemented |
| `N4BiasFieldCorrectionImageFilter` | ✅ Implemented |
| `NaryAddImageFilter` | ✅ Implemented (two-image) |
| `NaryMaximumImageFilter` | ✅ Implemented (two-image) |
| `NeighborhoodConnectedImageFilter` | ✅ Implemented |
| `NoiseImageFilter` | ✅ Implemented |
| `NormalizeImageFilter` | ✅ Implemented |
| `NormalizeToConstantImageFilter` | ✅ Implemented |
| `NormalizedCorrelationImageFilter` | ✅ Implemented |
| `NotImageFilter` | ✅ Implemented |
| `NotEqualImageFilter` | ✅ Implemented |
| `ObjectnessMeasureImageFilter` | ✅ Implemented |
| `OpeningByReconstructionImageFilter` | ✅ Implemented |
| `OrImageFilter` | ✅ Implemented |
| `OtsuMultipleThresholdsImageFilter` | ✅ Implemented |
| `OtsuThresholdImageFilter` | ✅ Implemented |
| `PasteImageFilter` | ✅ Implemented (`filters::paste`) |
| `PatchBasedDenoisingImageFilter` | ✅ Implemented (`filters::patch_based_denoising`) |
| `PermuteAxesImageFilter` | ✅ Implemented |
| `PowImageFilter` | ✅ Implemented |
| `ProjectedLandweberDeconvolutionImageFilter` | ✅ Implemented |
| `RankImageFilter` | ✅ Implemented |
| `RealAndImaginaryToComplexImageFilter` | ✅ Implemented |
| `RealToHalfHermitianForwardFFTImageFilter` | ✅ Implemented |
| `ReconstructionByDilationImageFilter` | ✅ Implemented |
| `ReconstructionByErosionImageFilter` | ✅ Implemented |
| `RecursiveGaussianImageFilter` | ✅ Implemented |
| `RegionOfInterestImageFilter` | ✅ Implemented |
| `RegionalMaximaImageFilter` | ✅ Implemented |
| `RegionalMinimaImageFilter` | ✅ Implemented |
| `RelabelComponentImageFilter` | ✅ Implemented |
| `RelabelLabelMapFilter` | ✅ Implemented |
| `RenyiEntropyThresholdImageFilter` | ✅ Implemented |
| `ResampleImageFilter` | ✅ Implemented (`transform::resample`, `transform::resample_to_ref`) |
| `RescaleIntensityImageFilter` | ✅ Implemented |
| `RichardsonLucyDeconvolutionImageFilter` | ✅ Implemented |
| `RoundImageFilter` | ✅ Implemented |
| `SLICImageFilter` | ✅ Implemented |
| `STAPLEImageFilter` | ✅ Implemented |
| `SaltAndPepperNoiseImageFilter` | ✅ Implemented |
| `ScalarChanAndVeseDenseLevelSetImageFilter` | ✅ Implemented (`filters::scalar_chan_vese_level_set`) |
| `ScalarConnectedComponentImageFilter` | ✅ Implemented |
| `ScalarImageKmeansImageFilter` | ✅ Implemented |
| `ScalarToRGBColormapImageFilter` | ✅ Implemented |
| `ShanbhagThresholdImageFilter` | ✅ Implemented |
| `ShapeDetectionLevelSetImageFilter` | ✅ Implemented (`filters::shape_detection_level_set`) |
| `ShiftScaleImageFilter` | ✅ Implemented |
| `ShotNoiseImageFilter` | ✅ Implemented |
| `ShrinkImageFilter` | ✅ Implemented |
| `SigmoidImageFilter` | ✅ Implemented |
| `SignedDanielssonDistanceMapImageFilter` | ✅ Implemented |
| `SignedMaurerDistanceMapImageFilter` | ✅ Implemented |
| `SimilarityIndexImageFilter` | ✅ Implemented (`measurements::similarity_index`) |
| `SimpleContourExtractorImageFilter` | ✅ Implemented |
| `SinImageFilter` | ✅ Implemented |
| `SliceImageFilter` | ✅ Implemented |
| `SmoothingRecursiveGaussianImageFilter` | ✅ Implemented |
| `SobelEdgeDetectionImageFilter` | ✅ Implemented |
| `SpeckleNoiseImageFilter` | ✅ Implemented |
| `SqrtImageFilter` | ✅ Implemented |
| `SquareImageFilter` | ✅ Implemented |
| `SquaredDifferenceImageFilter` | ✅ Implemented |
| `StandardDeviationProjectionImageFilter` | ✅ Implemented |
| `StatisticsImageFilter` | ✅ Implemented (`measurements::statistics`) |
| `StochasticFractalDimensionImageFilter` | ✅ Implemented |
| `SubtractImageFilter` | ✅ Implemented |
| `SumProjectionImageFilter` | ✅ Implemented |
| `SymmetricForcesDemonsRegistrationFilter` | ❌ Not implemented |
| `TanImageFilter` | ✅ Implemented |
| `TernaryAddImageFilter` | ✅ Implemented |
| `TernaryMagnitudeImageFilter` | ✅ Implemented |
| `TernaryMagnitudeSquaredImageFilter` | ✅ Implemented |
| `ThresholdImageFilter` | ✅ Implemented |
| `ThresholdMaximumConnectedComponentsImageFilter` | ✅ Implemented |
| `ThresholdSegmentationLevelSetImageFilter` | ✅ Implemented (`filters::threshold_segmentation_level_set`) |
| `TikhonovDeconvolutionImageFilter` | ✅ Implemented |
| `TileImageFilter` | ✅ Implemented |
| `TobogganImageFilter` | ✅ Implemented |
| `TransformGeometryImageFilter` | ✅ Implemented (`filters::transform_geometry`) |
| `TransformToDisplacementFieldFilter` | ✅ Implemented (`transform::transform_to_displacement_field`) |
| `TriangleThresholdImageFilter` | ✅ Implemented |
| `UnaryMinusImageFilter` | ✅ Implemented |
| `UnsharpMaskImageFilter` | ✅ Implemented |
| `ValuedRegionalMaximaImageFilter` | ✅ Implemented |
| `ValuedRegionalMinimaImageFilter` | ✅ Implemented |
| `VectorConfidenceConnectedImageFilter` | ✅ Implemented (`filters::vector_confidence_connected`) |
| `VectorConnectedComponentImageFilter` | ✅ Implemented |
| `VectorIndexSelectionCastImageFilter` | ✅ Implemented |
| `VectorMagnitudeImageFilter` | ✅ Implemented |
| `VotingBinaryImageFilter` | ✅ Implemented |
| `VotingBinaryHoleFillingImageFilter` | ✅ Implemented |
| `VotingBinaryIterativeHoleFillingImageFilter` | ✅ Implemented |
| `WarpImageFilter` | ✅ Implemented (`filters::warp_image`) |
| `WhiteTopHatImageFilter` | ✅ Implemented |
| `WienerDeconvolutionImageFilter` | ✅ Implemented |
| `WrapPadImageFilter` | ✅ Implemented |
| `XorImageFilter` | ✅ Implemented |
| `YenThresholdImageFilter` | ✅ Implemented |
| `ZeroCrossingImageFilter` | ✅ Implemented |
| `ZeroCrossingBasedEdgeDetectionImageFilter` | ✅ Implemented |
| `ZeroFluxNeumannPadImageFilter` | ✅ Implemented |

### Transforms

| Operation | Status |
|-----------|--------|
| `AffineTransform` | ✅ Implemented (`Transform::affine`) |
| `BSplineTransform` | ❌ Not implemented (coefficient image API not bridgeable simply) |
| `CompositeTransform` | ✅ Implemented (`Transform::composite`) |
| `DisplacementFieldTransform` | ✅ Implemented (`Transform::displacement_field`) |
| `Euler2DTransform` | ✅ Implemented (`Transform::euler2d`) |
| `Euler3DTransform` | ✅ Implemented (`Transform::euler3d`) |
| `ScaleTransform` | ✅ Implemented (`Transform::scale`) |
| `ScaleSkewVersor3DTransform` | ✅ Implemented (`Transform::scale_skew_versor3d`) |
| `ScaleVersor3DTransform` | ✅ Implemented (`Transform::scale_versor3d`) |
| `Similarity2DTransform` | ✅ Implemented (`Transform::similarity2d`) |
| `Similarity3DTransform` | ✅ Implemented (`Transform::similarity3d`) |
| `TranslationTransform` | ✅ Implemented (`Transform::translation`) |
| `VersorRigid3DTransform` | ✅ Implemented (`Transform::versor_rigid3d`) |
| `VersorTransform` | ✅ Implemented (`Transform::versor`) |
| `ComposeScaleSkewVersor3DTransform` | ✅ Implemented (`Transform::scale_skew_versor3d`) |

### Image Sources

| Operation | Status |
|-----------|--------|
| `GaussianImageSource` | ✅ Implemented |
| `GaborImageSource` | ✅ Implemented |
| `GridImageSource` | ✅ Implemented |
| `PhysicalPointImageSource` | ✅ Implemented |

### Registration

| Operation | Status |
|-----------|--------|
| `ImageRegistrationMethod` | ❌ Not implemented |

# Feature Coverage

Status of SimpleITK 2.4.0 operations in this library.

## IO & Image

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

## Filters

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
| `CannySegmentationLevelSetImageFilter` | ❌ Not implemented |
| `CastImageFilter` | ✅ Implemented |
| `CenteredTransformInitializerFilter` | ❌ Not implemented |
| `CenteredVersorTransformInitializerFilter` | ❌ Not implemented |
| `ChangeLabelImageFilter` | ❌ Not implemented |
| `ChangeLabelLabelMapFilter` | ❌ Not implemented |
| `CheckerBoardImageFilter` | ✅ Implemented |
| `ClampImageFilter` | ✅ Implemented |
| `ClosingByReconstructionImageFilter` | ✅ Implemented |
| `CollidingFrontsImageFilter` | ❌ Not implemented |
| `ComplexToImaginaryImageFilter` | ✅ Implemented |
| `ComplexToModulusImageFilter` | ✅ Implemented |
| `ComplexToPhaseImageFilter` | ✅ Implemented |
| `ComplexToRealImageFilter` | ✅ Implemented |
| `ComposeImageFilter` | ✅ Implemented |
| `ConfidenceConnectedImageFilter` | ❌ Not implemented |
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
| `FastMarchingUpwindGradientImageFilter` | ❌ Not implemented |
| `FastSymmetricForcesDemonsRegistrationFilter` | ❌ Not implemented |
| `FlipImageFilter` | ✅ Implemented |
| `ForwardFFTImageFilter` | ✅ Implemented |
| `GeodesicActiveContourLevelSetImageFilter` | ❌ Not implemented |
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
| `HashImageFilter` | ❌ Not implemented |
| `HausdorffDistanceImageFilter` | ❌ Not implemented |
| `HistogramMatchingImageFilter` | ✅ Implemented |
| `HuangThresholdImageFilter` | ✅ Implemented |
| `ImportImageFilter` | ❌ Not implemented |
| `IntensityWindowingImageFilter` | ✅ Implemented |
| `IntermodesThresholdImageFilter` | ✅ Implemented |
| `InverseDeconvolutionImageFilter` | ✅ Implemented |
| `InverseDisplacementFieldImageFilter` | ❌ Not implemented |
| `InverseFFTImageFilter` | ✅ Implemented |
| `InvertDisplacementFieldImageFilter` | ❌ Not implemented |
| `InvertIntensityImageFilter` | ✅ Implemented |
| `IsoContourDistanceImageFilter` | ✅ Implemented |
| `IsoDataThresholdImageFilter` | ✅ Implemented |
| `IsolatedConnectedImageFilter` | ✅ Implemented |
| `IsolatedWatershedImageFilter` | ✅ Implemented |
| `IterativeInverseDisplacementFieldImageFilter` | ❌ Not implemented |
| `JoinSeriesImageFilter` | ✅ Implemented |
| `KittlerIllingworthThresholdImageFilter` | ✅ Implemented |
| `LabelContourImageFilter` | ✅ Implemented |
| `LabelImageToLabelMapFilter` | ✅ Implemented |
| `LabelIntensityStatisticsImageFilter` | ❌ Not implemented |
| `LabelMapContourOverlayImageFilter` | ✅ Implemented |
| `LabelMapMaskImageFilter` | ✅ Implemented |
| `LabelMapOverlayImageFilter` | ✅ Implemented |
| `LabelMapToBinaryImageFilter` | ✅ Implemented |
| `LabelMapToLabelImageFilter` | ✅ Implemented |
| `LabelMapToRGBImageFilter` | ✅ Implemented |
| `LabelOverlapMeasuresImageFilter` | ❌ Not implemented |
| `LabelOverlayImageFilter` | ✅ Implemented |
| `LabelShapeStatisticsImageFilter` | ❌ Not implemented |
| `LabelStatisticsImageFilter` | ❌ Not implemented |
| `LabelToRGBImageFilter` | ✅ Implemented |
| `LabelUniqueLabelMapFilter` | ✅ Implemented |
| `LabelVotingImageFilter` | ✅ Implemented |
| `LandmarkBasedTransformInitializerFilter` | ❌ Not implemented |
| `LandweberDeconvolutionImageFilter` | ✅ Implemented |
| `LaplacianImageFilter` | ✅ Implemented |
| `LaplacianRecursiveGaussianImageFilter` | ✅ Implemented |
| `LaplacianSegmentationLevelSetImageFilter` | ❌ Not implemented |
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
| `MinimumMaximumImageFilter` | ❌ Not implemented |
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
| `PasteImageFilter` | ❌ Not implemented |
| `PatchBasedDenoisingImageFilter` | ❌ Not implemented |
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
| `ResampleImageFilter` | ❌ Not implemented |
| `RescaleIntensityImageFilter` | ✅ Implemented |
| `RichardsonLucyDeconvolutionImageFilter` | ✅ Implemented |
| `RoundImageFilter` | ✅ Implemented |
| `SLICImageFilter` | ✅ Implemented |
| `STAPLEImageFilter` | ✅ Implemented |
| `SaltAndPepperNoiseImageFilter` | ✅ Implemented |
| `ScalarChanAndVeseDenseLevelSetImageFilter` | ❌ Not implemented |
| `ScalarConnectedComponentImageFilter` | ✅ Implemented |
| `ScalarImageKmeansImageFilter` | ✅ Implemented |
| `ScalarToRGBColormapImageFilter` | ✅ Implemented |
| `ShanbhagThresholdImageFilter` | ✅ Implemented |
| `ShapeDetectionLevelSetImageFilter` | ❌ Not implemented |
| `ShiftScaleImageFilter` | ✅ Implemented |
| `ShotNoiseImageFilter` | ✅ Implemented |
| `ShrinkImageFilter` | ✅ Implemented |
| `SigmoidImageFilter` | ✅ Implemented |
| `SignedDanielssonDistanceMapImageFilter` | ✅ Implemented |
| `SignedMaurerDistanceMapImageFilter` | ✅ Implemented |
| `SimilarityIndexImageFilter` | ❌ Not implemented |
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
| `StatisticsImageFilter` | ❌ Not implemented |
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
| `ThresholdSegmentationLevelSetImageFilter` | ❌ Not implemented |
| `TikhonovDeconvolutionImageFilter` | ✅ Implemented |
| `TileImageFilter` | ✅ Implemented |
| `TobogganImageFilter` | ✅ Implemented |
| `TransformGeometryImageFilter` | ❌ Not implemented |
| `TransformToDisplacementFieldFilter` | ❌ Not implemented |
| `TriangleThresholdImageFilter` | ✅ Implemented |
| `UnaryMinusImageFilter` | ✅ Implemented |
| `UnsharpMaskImageFilter` | ✅ Implemented |
| `ValuedRegionalMaximaImageFilter` | ✅ Implemented |
| `ValuedRegionalMinimaImageFilter` | ✅ Implemented |
| `VectorConfidenceConnectedImageFilter` | ❌ Not implemented |
| `VectorConnectedComponentImageFilter` | ✅ Implemented |
| `VectorIndexSelectionCastImageFilter` | ✅ Implemented |
| `VectorMagnitudeImageFilter` | ✅ Implemented |
| `VotingBinaryImageFilter` | ✅ Implemented |
| `VotingBinaryHoleFillingImageFilter` | ✅ Implemented |
| `VotingBinaryIterativeHoleFillingImageFilter` | ✅ Implemented |
| `WarpImageFilter` | ❌ Not implemented |
| `WhiteTopHatImageFilter` | ✅ Implemented |
| `WienerDeconvolutionImageFilter` | ✅ Implemented |
| `WrapPadImageFilter` | ✅ Implemented |
| `XorImageFilter` | ✅ Implemented |
| `YenThresholdImageFilter` | ✅ Implemented |
| `ZeroCrossingImageFilter` | ✅ Implemented |
| `ZeroCrossingBasedEdgeDetectionImageFilter` | ✅ Implemented |
| `ZeroFluxNeumannPadImageFilter` | ✅ Implemented |

## Transforms

| Operation | Status |
|-----------|--------|
| `AffineTransform` | ❌ Not implemented |
| `BSplineTransform` | ❌ Not implemented |
| `CompositeTransform` | ❌ Not implemented |
| `DisplacementFieldTransform` | ❌ Not implemented |
| `Euler2DTransform` | ❌ Not implemented |
| `Euler3DTransform` | ❌ Not implemented |
| `ScaleTransform` | ❌ Not implemented |
| `ScaleSkewVersor3DTransform` | ❌ Not implemented |
| `ScaleVersor3DTransform` | ❌ Not implemented |
| `Similarity2DTransform` | ❌ Not implemented |
| `Similarity3DTransform` | ❌ Not implemented |
| `TranslationTransform` | ❌ Not implemented |
| `VersorRigid3DTransform` | ❌ Not implemented |
| `VersorTransform` | ❌ Not implemented |
| `ComposeScaleSkewVersor3DTransform` | ❌ Not implemented |

## Image Sources

| Operation | Status |
|-----------|--------|
| `GaussianImageSource` | ✅ Implemented |
| `GaborImageSource` | ✅ Implemented |
| `GridImageSource` | ✅ Implemented |
| `PhysicalPointImageSource` | ✅ Implemented |

## Registration

| Operation | Status |
|-----------|--------|
| `ImageRegistrationMethod` | ❌ Not implemented |

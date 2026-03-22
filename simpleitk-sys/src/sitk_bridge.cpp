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

} // namespace sitk_rs

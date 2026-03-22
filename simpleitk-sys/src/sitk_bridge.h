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

} // namespace sitk_rs

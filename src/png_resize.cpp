#include <opencv2/opencv.hpp>

size_t allocated_size = 0;

/**
 * Resizes an image-buffer with INTER_AREA interpolation
 * @param  width   width of image in buffer
 * @param  height  height of image in buffer
 * @param  nwidth  desired image width
 * @param  nheight desired image height
 * @param  data    pointer to array of pixel values with 3 channels and u8 values
 * @return         pointer to resized image
 */
extern "C"
uint8_t* resize_image(uint32_t width, uint32_t height, uint32_t nwidth, uint32_t nheight, uint8_t* data) {
    // Convert buffer to OpenCV Image on stack
    auto image = cv::Mat(cv::Size(width, height), CV_8UC3, data);
    // Resized image will also be on the stack
    cv::Mat resized_tile;
    // Resizing
    cv::resize(image, resized_tile, cv::Size(nwidth, nheight), 0, 0, cv::INTER_AREA);
    // Calculating size of new buffer
    auto size = resized_tile.total() * resized_tile.elemSize();
    // Allocating memory on heap
    auto* bytes = static_cast<uint8_t*>(malloc(sizeof(uint8_t) * size));
    // Copying data from image to buffer
    std::memcpy(bytes, resized_tile.data, size * sizeof(uint8_t));
    // Chaning allocated_size
    allocated_size = size;
    // Return buffer
    return bytes;
}

/**
 * Frees memory that was allocated with resize_image and resets size.
 * @param ptr pointer to resized image
 */
extern "C"
void free_image(uint8_t* ptr) {
    if (allocated_size < 1) return;
    free(ptr);
    allocated_size = 0;
}
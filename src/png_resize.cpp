#include <opencv2/opencv.hpp>

size_t allocated_size = 0;

extern "C"
uint8_t* resize_image(uint32_t width, uint32_t height, uint32_t nwidth, uint32_t nheight, uint8_t* data) {
    auto image = cv::Mat(cv::Size(width, height), CV_8UC3, data);
    cv::Mat resized_tile;
    cv::resize(image, resized_tile, cv::Size(nwidth, nheight), 0, 0, cv::INTER_AREA);
    auto size = resized_tile.total() * resized_tile.elemSize();
    auto* bytes = static_cast<uint8_t*>(malloc(sizeof(uint8_t) * size));
    std::memcpy(bytes,image.data,size * sizeof(uint8_t));
    allocated_size = size;
    return bytes;
}

extern "C"
void free_image(uint8_t* ptr) {
    free(ptr);
    allocated_size = 0;
}
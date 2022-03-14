fn main() {
    cc::Build::new()
        .file("src/png_resize.cpp")
        .include("/usr/local/Cellar/opencv/4.5.5/include/opencv4/opencv")
        .include("/usr/local/Cellar/opencv/4.5.5/include/opencv4")
        .flag("-std=c++11")
        .cpp(true)
        .compile("png_resize.a");
}

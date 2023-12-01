cmake_minimum_required(VERSION 3.25)

set(CMAKE_CUDA_ARCHITECTURES native)
set(CMAKE_CUDA_COMPILER /usr/local/cuda-12.1/bin/nvcc)
project(aoc_cuda LANGUAGES CUDA CXX)

set(CMAKE_CUDA_STANDARD 20)

add_compile_options(-g)
add_compile_options(-G)

add_subdirectory(bin)

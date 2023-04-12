cmake_minimum_required(VERSION 3.16)
project("C snippets")
set(CMAKE_CXX_FLAGS "-g -Wall")

# set up source files to be tested and build library
set(c_srcs 
  c_src/main.c
  c_src/measure.c
)
include_directories(c_src)
add_executable(c ${c_srcs})


cmake_minimum_required(VERSION 3.10)
project(googleplexer C)

set(CMAKE_C_STANDARD 11)

find_package(PkgConfig REQUIRED)

file(GLOB_RECURSE SOURCES "src/*.c")

add_executable(googleplexer ${SOURCES})


target_include_directories(googleplexer PRIVATE 
    ${CMAKE_CURRENT_SOURCE_DIR}/src
)
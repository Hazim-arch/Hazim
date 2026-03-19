cmake_minimum_required(VERSION 3.17)

# Force the compiler path (Standard Arch location)
set(CMAKE_C_COMPILER "/usr/bin/gcc")
set(CMAKE_TRY_COMPILE_TARGET_TYPE STATIC_LIBRARY)

# Initialize project WITHOUT languages first
project(googleplexer NONE) 

# Now enable C manually
enable_language(C)

set(CMAKE_SYSTEM_NAME Generic)
set(CMAKE_SYSTEM_PROCESSOR x86_64)

add_library(googleplexer STATIC 
    src/display/screen/screen.c
    src/print_driver/print_driver.c 
    )

target_compile_options(googleplexer PRIVATE 
    -ffreestanding 
    -nostdlib 
    -fno-stack-protector 
    -mno-red-zone
)
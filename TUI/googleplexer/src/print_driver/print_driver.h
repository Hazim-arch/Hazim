#ifndef PRINT_DRIVER
#define PRINT_DRIVER

#include <stdint.h>

#define VGA xb8000
#define WIDTH 80
#define HEIGHT 25

typedef struct {
    uint8_t ascii;
    uint8_t color;
} __attribute__((packed)) ScreenChar;

typedef struct {
    ScreenChar cells[WIDTH * HEIGHT];
} Screen;

void printc (int x, int y, char c, uint8_t color);

#endif
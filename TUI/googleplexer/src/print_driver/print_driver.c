#include "print_driver.h"

void printc(int x, int y, char c, uint8_t color) {
    volatile uint16_t* vga_buffer = (volatile uint16_t*)0xb8000;
    int index = (y * 80) + x;
    vga_buffer[index] = (uint16_t)c | (uint16_t)color << 8;
}
#include "screen.h"

void present(Screen* virtual_screen) {
    volatile ScreenChar* vga_hardware = (ScreenChar*)0xb8000;
    
    for (int i = 0; i < WIDTH * HEIGHT; i++) {
        vga_hardware[i] = virtual_screen->cells[i];
    }
}
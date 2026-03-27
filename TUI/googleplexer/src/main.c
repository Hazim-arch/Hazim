#include <stdio.h>
#include <stdbool.h>
#include <stdlib.h>
#include <string.h>
#include <stdint.h>

static void init(int WIGTH, int HIGHT, int X, int Y, const char *TITLE) {
    int SizeOfTitle = (int)strlen(TITLE);
    const int OFFSET_WIGTH_1 = 7; 
    const int OFFSET_WIGHT_2 = 8;
    int draw = WIGTH - OFFSET_WIGTH_1 - SizeOfTitle;
    int draw2 = WIGTH - OFFSET_WIGHT_2 + SizeOfTitle;
    printf("┌┤ %s ├", TITLE);
    for (int i = 0; i <= draw; i++) {
        printf("─");
    }
    printf("┐");

    for (int i = 0; i <= HIGHT; i++) {
        printf("\n│");
        for (int x = 0; x < draw + SizeOfTitle + 5; x++) {
            printf(" ");
        }
        printf("│");
    }

    printf("\n└");
    for (int i = 0; i <= draw2; i++) {
        printf("─");
    }
    printf("┘");
}

int main(int argc, char argv) {
    init(21, 10, 10, 0, "hello");
    char *string = "└─────────────────┘";
    size_t String = strlen(string);
    printf("\n%zu", String);
}
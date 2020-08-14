#include <stdio.h>
#include <stdlib.h>
#include <stdint.h>
#include <assert.h>
#include "../lvgl.h"

/* For 2x2 Image, 3 bytes per pixel (R, G, B):
{
    255, 0, 0,  // Red
    0, 255, 0,  // Green
    0, 0, 255,  // Blue
    255, 255, 0 // Yellow
}; */
#define BYTES_PER_PIXEL 3
uint8_t pixels[LV_HOR_RES_MAX * LV_VER_RES_MAX * BYTES_PER_PIXEL];

void put_px(uint16_t x, uint16_t y, uint8_t r, uint8_t g, uint8_t b, uint8_t a) {
    assert(x >= 0); assert(x < LV_HOR_RES_MAX);
    assert(y >= 0); assert(y < LV_VER_RES_MAX);
    int i = (y * LV_HOR_RES_MAX * BYTES_PER_PIXEL) + (x * BYTES_PER_PIXEL);
    pixels[i++] = r;  //  Red
    pixels[i++] = g;  //  Green
    pixels[i++] = b;  //  Blue
}

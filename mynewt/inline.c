//  Generate the static inline functions that are needed for the Rust Bindings in pinetime-lvgl (like lv_obj_set_style_local_text_font)
//  TODO: Sync with firmware: https://gitlab.com/lupyuen/pinetime_lvgl_mynewt/blob/master/src/pinetime/inline.c

//  Override the keywords "static" and "inline" so that the static inline functions will be defined here
#define static
#define inline

//  Include the definitions for static inline functions in lv_obj_style_dec.h
#include "../src/lv_core/lv_style.h"
#include "../src/lv_core/lv_obj.h"
#include "../src/lv_core/lv_obj_style_dec.h"

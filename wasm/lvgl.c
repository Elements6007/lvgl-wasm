//  Simple port of LVGL to WebAssembly.
//  ??? Renders UI controls but touch input not handled yet.
//  To build see lvgl.sh.
//  Sample log: logs/lvgl.log 
#include <stdio.h>
#include <stdlib.h>
#include <string.h>
#include <assert.h>
#include "../lvgl.h"
#include "../demo/lv_demo_widgets.h"
#include "lv_port_disp.h"

////////////////////////////////////////////////////////////////////
//  Render LVGL

/// Render a Button Widget and a Label Widget
static void render_widgets(void) {
    puts("Rendering widgets...");
    lv_obj_t * btn = lv_btn_create(lv_scr_act(), NULL);     //  Add a button the current screen
    lv_obj_set_pos(btn, 10, 10);                            //  Set its position
    lv_obj_set_size(btn, 120, 50);                          //  Set its size

    lv_obj_t * label = lv_label_create(btn, NULL);          //  Add a label to the button
    lv_label_set_text(label, "Button");                     //  Set the labels text
}

/// Render the LVGL display
static void render_display() {
    puts("Rendering display...");

    //  Init the LVGL display
    lv_init();
    lv_port_disp_init();

    //  Create the LVGL widgets
    render_widgets();  //  For button and label
    //  lv_demo_widgets();  //  For all kinds of demo widgets

    //  Render the LVGL widgets
    puts("Handle task...");
    lv_task_handler();
}

////////////////////////////////////////////////////////////////////
//  Main

int main(int argc, char **argv) {
    render_display();
    return 0;
}

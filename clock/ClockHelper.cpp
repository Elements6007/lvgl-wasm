////  WebAssembly Helper
#include <stdio.h>
#include <assert.h>
#include "Clock.h"
#include "ClockHelper.h"

static Pinetime::Applications::Screens::Clock *clock0 = nullptr;
static DisplayApp *app0 = nullptr;
static Pinetime::Controllers::DateTime *dateTimeController0 = nullptr;
static Pinetime::Controllers::Battery *batteryController0 = nullptr;
static Pinetime::Controllers::Ble *bleController0 = nullptr;

//  TODO: Check LabelBigStyle
//  extern lv_style_t* LabelBigStyle;
//  assert(LabelBigStyle != nullptr);

/// Create an instance of the clock
int create_clock(void) {
    puts("In C++: Creating clock...");
    app0 = new DisplayApp();
    dateTimeController0 = new Pinetime::Controllers::DateTime();
    batteryController0 = new Pinetime::Controllers::Battery();
    bleController0 = new Pinetime::Controllers::Ble();
    clock0 = new Pinetime::Applications::Screens::Clock(
        app0,
        *dateTimeController0,
        *batteryController0,
        *bleController0
    );
    return 0;
}

/// Redraw the clock
int refresh_clock(void) {
    puts("In C++: Refreshing clock...");
    assert(clock0 != 0);
    clock0->Refresh();
    return 0;
}

/// Update the clock time
int update_clock(void) {
    puts("In C++: Updating clock...");
    assert(clock0 != 0);
    //  TODO
    return 0;
}

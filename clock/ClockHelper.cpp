////  WebAssembly Helper
#include <stdio.h>
#include <assert.h>
#include "LittleVgl.h"
#include "WatchFaceDigital.h"
#include "ClockHelper.h"

static Pinetime::Components::LittleVgl *littleVgl0 = nullptr;
static Pinetime::Drivers::St7789 *lcd0 = nullptr;
static Pinetime::Drivers::Cst816S *touchPanel0 = nullptr;
static Pinetime::Applications::Screens::Clock *clock0 = nullptr;
static DisplayApp *app0 = nullptr;
static Pinetime::Controllers::DateTime *dateTimeController0 = nullptr;
static Pinetime::Controllers::Battery *batteryController0 = nullptr;
static Pinetime::Controllers::Ble *bleController0 = nullptr;
extern lv_style_t* LabelBigStyle;

/// Create an instance of the clock
int create_clock(void) {
    puts("In C++: Creating clock...");

    //  Init LVGL styles
    lcd0 = new Pinetime::Drivers::St7789();
    touchPanel0 = new Pinetime::Drivers::Cst816S();
    littleVgl0 = new Pinetime::Components::LittleVgl(
        *lcd0,
        *touchPanel0
    );
    assert(LabelBigStyle != nullptr);

    //  Create clock
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
    assert(clock0 != nullptr);
    clock0->Refresh();
    return 0;
}

/// Update the clock time. Use generic "int" type to prevent JavaScript-WebAssembly interoperability problems.
int update_clock(int year, int month, int day, 
    int hour, int minute, int second) {
    puts("In C++: Updating clock...");
    assert(dateTimeController0 != nullptr);
    dateTimeController0->SetTime(
        year, month, day, 
        0,  //  Auto compute day of week 
        hour, minute, second,
        0   //  Offset for systick counter
    );
    return 0;
}

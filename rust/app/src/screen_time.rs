//! Moved to watch_face.rs
/*
use core::{
    fmt::Write,
    ptr,
};
use lvgl::{
    result::*,
    core::obj,
    objx::label,
    Strn,
};
use lvgl_macros::strn;

/// Create the widgets for the Watch Face. Called by create_watch_face() below.
fn create_widgets(widgets: &mut WatchFaceWidgets) -> LvglResult<()> {
    let scr = widgets.screen;
    assert!(!scr.is_null(), "null screen");

    //  Create a label for time (00:00)
    let label1 = label::create(scr, ptr::null()) ? ;  //  `?` will terminate the function in case of error
    label::set_long_mode(label1, label::LV_LABEL_LONG_BREAK) ? ;
    label::set_text(label1, strn!("00:00")) ? ;  //  strn creates a null-terminated string
    obj::set_width(label1, 240) ? ;
    obj::set_height(label1, 200) ? ;
    label::set_align(label1, label::LV_LABEL_ALIGN_CENTER) ? ;
    obj::align(label1, scr, obj::LV_ALIGN_CENTER, 0, -30) ? ;
    obj::set_style(label1, unsafe { &style_time }) ? ;  //  Previously: label::set_style
    widgets.time_label = label1;

    //  Create a label for Bluetooth state
    let l_state = label::create(scr, ptr::null()) ? ;
    obj::set_width(l_state, 50) ? ;
    obj::set_height(l_state, 80) ? ;
    label::set_text(l_state, strn!("")) ? ;  //  strn creates a null-terminated string
    label::set_recolor(l_state, true) ? ;
    label::set_align(l_state, label::LV_LABEL_ALIGN_LEFT) ? ;
    obj::align(l_state, scr, obj::LV_ALIGN_IN_TOP_LEFT, 0, 0) ? ;
    widgets.ble_label = l_state;

    //  Create a label for Power indicator
    let l_power = label::create(scr, ptr::null()) ? ;
    obj::set_width(l_power, 80) ? ;
    obj::set_height(l_power, 20) ? ;
    label::set_text(l_power, strn!("")) ? ;  //  strn creates a null-terminated string
    label::set_recolor(l_power, true) ? ;
    label::set_align(l_power, label::LV_LABEL_ALIGN_RIGHT) ? ;
    obj::align(l_power, scr, obj::LV_ALIGN_IN_TOP_RIGHT, 0, 0) ? ;
    widgets.power_label = l_power;

    //  Create a label for Date
    let label_date = label::create(scr, ptr::null()) ? ;
    label::set_long_mode(label_date, label::LV_LABEL_LONG_BREAK) ? ;
    obj::set_width(label_date, 200) ? ;
    obj::set_height(label_date, 200) ? ;
    label::set_text(label_date, strn!("")) ? ;  //  strn creates a null-terminated string
    label::set_align(label_date, label::LV_LABEL_ALIGN_CENTER) ? ;
    obj::align(label_date, scr, obj::LV_ALIGN_CENTER, 0, 40) ? ;
    widgets.date_label = label_date;

    //  Allow touch events
    obj::set_click(scr, true) ? ;
    Ok(())
}

/// Update the widgets in the Watch Face with the current state. Called by update_watch_face() below.
fn update_widgets(widgets: &WatchFaceWidgets, state: &WatchFaceState) -> LvglResult<()> {
    set_time_label(widgets, state) ? ;
    set_bt_label(widgets, state) ? ;
    set_power_label(widgets, state) ? ;
    Ok(())
}

/// Populate the Bluetooth Label with the Bluetooth status. Called by screen_time_update_screen() above.
fn set_bt_label(widgets: &WatchFaceWidgets, state: &WatchFaceState) -> LvglResult<()> {
    if state.ble_state == BleState::BLEMAN_BLE_STATE_DISCONNECTED {
        label::set_text(widgets.ble_label, strn!("")) ? ;
    } else {
        //  Get the color of the Bluetooth icon
        let color = 
            match &state.ble_state {
                BleState::BLEMAN_BLE_STATE_INACTIVE     => "#000000",  //  Black
                BleState::BLEMAN_BLE_STATE_DISCONNECTED => "#f2495c",  //  GUI_COLOR_LBL_BASIC_RED
                BleState::BLEMAN_BLE_STATE_ADVERTISING  => "#5794f2",  //  GUI_COLOR_LBL_BASIC_BLUE
                BleState::BLEMAN_BLE_STATE_CONNECTED    => "#37872d",  //  GUI_COLOR_LBL_DARK_GREEN
            };
        //  Create a string buffer with max size 16 to format the Bluetooth status
        static mut BLUETOOTH_STATUS: heapless::String::<heapless::consts::U16> = heapless::String(heapless::i::String::new());
        //  Format the Bluetooth status and set the label
        unsafe {
            BLUETOOTH_STATUS.clear();
            write!(&mut BLUETOOTH_STATUS, 
                "{} \u{F293}#\0",  //  LV_SYMBOL_BLUETOOTH. Must terminate Rust strings with null.
                color)
                .expect("bt fail");
            label::set_text(widgets.ble_label, &Strn::new(BLUETOOTH_STATUS.as_bytes())) ? ;  //  TODO: Simplify    
        }
    }
    Ok(())
}

/// Populate the Power Label with the battery status. Called by screen_time_update_screen() above.
fn set_power_label(widgets: &WatchFaceWidgets, state: &WatchFaceState) -> LvglResult<()> {
    let percentage = unsafe { hal_battery_get_percentage(state.millivolts) };
    let color =   //  Charging color
        if percentage <= 20  //  battery_low 
            { "#f2495c" }    //  battery_low_color
        else if state.powered && !(state.charging) 
            { "#73bf69" }    //  battery_full_color: Battery charge cycle finished
        else 
            { "#fade2a" };   //  battery_mid_color
    let symbol =  //  Charging symbol
        if state.powered { "\u{F0E7}" }  //  LV_SYMBOL_CHARGE
        else { " " };
    //  Create a string buffer with max size 50 and format the battery status
    static mut BATTERY_STATUS: heapless::String::<heapless::consts::U50> = heapless::String(heapless::i::String::new());
    //  Format the battery status and set the label
    unsafe {
        BATTERY_STATUS.clear();
        write!(&mut BATTERY_STATUS, 
            "{} {}%{}#\nRUST ({}mV)\0",  //  Must terminate Rust strings with null
            color,
            percentage,
            symbol,
            state.millivolts)
            .expect("batt fail");
        label::set_text(widgets.power_label, &Strn::new(BATTERY_STATUS.as_bytes())) ? ;  //  TODO: Simplify    
    }
    obj::align(widgets.power_label, widgets.screen, obj::LV_ALIGN_IN_TOP_RIGHT, 0, 0) ? ;
    Ok(())
}

/// Populate the Time and Date Labels with the time and date. Called by screen_time_update_screen() above.
fn set_time_label(widgets: &WatchFaceWidgets, state: &WatchFaceState) -> LvglResult<()> {
    //  Create a string buffer with max size 6 to format the time
    static mut TIME_BUF: heapless::String::<heapless::consts::U6> = heapless::String(heapless::i::String::new());
    //  Format the time and set the label
    unsafe {
        TIME_BUF.clear();
        write!(&mut TIME_BUF, "{:02}:{:02}\0",  //  Must terminate Rust strings with null
            state.time.hour,
            state.time.minute)
            .expect("time fail");
        label::set_text(widgets.time_label, &Strn::new(TIME_BUF.as_bytes())) ? ;  //  TODO: Simplify
    }

    //  Get the short month name
    let month_cstr = unsafe { controller_time_month_get_short_name(&state.time) };  //  Returns null-terminated C string
    assert!(!month_cstr.is_null(), "month null");
    let month_str = unsafe { cstr_core::CStr::from_ptr(month_cstr).to_str() }       //  Convert C string to Rust string
        .expect("month fail");

    //  Create a string buffer with max size 15 to format the date
    static mut DATE_BUF: heapless::String::<heapless::consts::U15> = heapless::String(heapless::i::String::new());
    //  Format the date and set the label
    unsafe {
        DATE_BUF.clear();
        write!(&mut DATE_BUF, "{} {} {}\n\0",  //  Must terminate Rust strings with null
            state.time.dayofmonth,
            month_str,
            state.time.year)
        .expect("date fail");
        label::set_text(widgets.date_label, &Strn::new(DATE_BUF.as_bytes())) ? ;  //  TODO: Simplify
    }
    Ok(())
}

/// Create the Watch Face, populated with widgets. Called by _screen_time_create() in screen_time.c.
#[no_mangle]  //  Don't mangle the function name
extern "C" fn create_watch_face(widgets: *mut WatchFaceWidgets) -> i32 {  //  Declare extern "C" because it will be called by RIOT OS firmware
    assert!(!widgets.is_null(), "widgets null");
    unsafe { create_widgets(&mut *widgets) }
        .expect("create_screen fail");
    0  //  Return OK
}

/// Populate the Watch Face with the current status. Called by _screen_time_update_screen() in screen_time.c.
#[no_mangle]  //  Don't mangle the function name
extern "C" fn update_watch_face(widgets: *const WatchFaceWidgets, state: *const WatchFaceState) -> i32 {
    assert!(!widgets.is_null(), "widgets null");
    unsafe { update_widgets(&*widgets, &*state) }
        .expect("update_widgets fail");
    0  //  Return OK
}

/// State for the Watch Face, shared between GUI and control. TODO: Sync with widgets/home_time/include/home_time.h
#[repr(C)]
struct WatchFaceState {
    ble_state:  BleState,  //  bleman_ble_state_t
    time:       controller_time_spec_t,
    millivolts: u32,
    charging:   bool,
    powered:    bool,
}

/// Widgets for the Watch Face, private to Rust. TODO: Sync with widgets/home_time/include/home_time.h
#[repr(C)]
#[allow(non_camel_case_types)]
struct WatchFaceWidgets {
    screen:      *mut obj::lv_obj_t,  //  TODO: Shared with home_time_widget_t
    time_label:  *mut obj::lv_obj_t,  //  TODO: Should be private to Rust
    date_label:  *mut obj::lv_obj_t,  //  TODO: Should be private to Rust
    ble_label:   *mut obj::lv_obj_t,  //  TODO: Should be private to Rust
    power_label: *mut obj::lv_obj_t,  //  TODO: Should be private to Rust
}

//  TODO: Sync with modules/bleman/include/bleman.h
#[repr(u8)]
#[derive(PartialEq)]
#[allow(non_camel_case_types)]
#[allow(dead_code)]
enum BleState {  //  bleman_ble_state_t
    BLEMAN_BLE_STATE_INACTIVE = 0,
    BLEMAN_BLE_STATE_ADVERTISING = 1,
    BLEMAN_BLE_STATE_DISCONNECTED = 2,
    BLEMAN_BLE_STATE_CONNECTED = 3,
}

//  TODO: Sync with modules/controller/include/controller/time.h
#[repr(C)]
#[allow(non_camel_case_types)]
struct controller_time_spec_t {
    year:       u16,
    month:      u8,
    dayofmonth: u8,
    hour:       u8,
    minute:     u8,
    second:     u8,
    fracs:      u8,
}

/// Import C APIs
extern {
    //  TODO: Sync with modules/hal/include/hal.h
    fn hal_battery_get_percentage(voltage: u32) -> i32;
    //  TODO: Sync with modules/controller/include/controller/time.h
    fn controller_time_month_get_short_name(time: *const controller_time_spec_t) -> *const ::cty::c_char;
    /// Style for the Time Label. TODO: Sync with widgets/home_time/screen_time.c
    static style_time: obj::lv_style_t;
}
*/
/* Stack Trace for screen_time_create:
#0  screen_time_create (ht=ht@entry=0x200008dc <home_time_widget>) at /Users/Luppy/PineTime/PineTime-apps/widgets/home_time/screen_time.c:68
#1  0x0001b36c in home_time_draw (widget=0x200008dc <home_time_widget>, parent=<optimized out>) at /Users/Luppy/PineTime/PineTime-apps/widgets/home_time/screen_time.c:222
#2  0x00003b32 in _switch_widget_draw (type=<optimized out>, widget=0x200008dc <home_time_widget>, gui=<optimized out>) at /Users/Luppy/PineTime/PineTime-apps/modules/gui/gui.c:159
#3  _gui_handle_msg (msg=0x20004c90 <_stack+1944>, gui=<optimized out>) at /Users/Luppy/PineTime/PineTime-apps/modules/gui/gui.c:299
#4  _lvgl_thread (arg=0x20004408 <_gui>) at /Users/Luppy/PineTime/PineTime-apps/modules/gui/gui.c:351
#5  0x000002f0 in sched_switch (other_prio=<optimized out>) at /Users/Luppy/PineTime/PineTime-apps/RIOT/core/sched.c:179
*/
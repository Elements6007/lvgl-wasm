`fn create_widgets(widgets: &mut WatchFaceWidgets) -> ` <br>
<br>
&nbsp;&nbsp;`    LvglResult<()> {` <br>
&nbsp;&nbsp;`    //  Create a label for time (00:00)` <br>
&nbsp;&nbsp;`    let scr = widgets.screen;` <br>
&nbsp;&nbsp;`    let label1 = label::create(scr, ptr::null()) ? ;` <br>
<br>
&nbsp;&nbsp;`    label::set_text(label1, strn!("00:00")) ? ;` <br>
&nbsp;&nbsp;`    obj::set_width(label1, 240) ? ;` <br>
&nbsp;&nbsp;`    obj::set_height(label1, 200) ? ;` <br>
&nbsp;&nbsp;`    widgets.time_label = label1;` <br>
&nbsp;&nbsp;`    ...` <br>
&nbsp;&nbsp;`    Ok(())` <br>
`}` <br>

unsafe { lv_label_set_text(
    b"abc\0".as_ptr(), 
    [b'a', b'b', b'c', 0].as_ptr()) }; ////
}

extern "C" {
fn lv_label_set_text(label: *const u8, text: *const u8); ////
}

////
fn zzz_screen_time_create(ht: *mut home_time_widget_t) -> *mut obj::lv_obj_t {  //  Declare extern "C" because it will be called by RIOT OS firmware
    let scr = lv_obj_create(ptr::null_mut(), ptr::null());
    let label1 = lv_label_create(scr, ptr::null());
    (*ht).lv_time = label1;
    scr
}
struct home_time_widget_t {
    lv_time: *mut obj::lv_obj_t,
}
extern "C" {
    fn lv_obj_create(parent: *mut obj::lv_obj_t, copy: *const obj::lv_obj_t) -> *mut obj::lv_obj_t;
    fn lv_label_create(par: *mut obj::lv_obj_t, copy: *const obj::lv_obj_t) -> *mut obj::lv_obj_t;
    fn lv_label_set_text(label: *mut obj::lv_obj_t, text: *const u8);
    fn lv_obj_set_width(obj: *mut obj::lv_obj_t, w: i16);
    fn lv_obj_set_height(obj: *mut obj::lv_obj_t, h: i16);
}
////

extern "C" {
    pub fn lv_obj_create(
        parent: *mut obj::lv_obj_t, 
        copy:   *const obj::lv_obj_t
    ) -> *mut obj::lv_obj_t;
}

pub fn create(
    parent: *mut obj::lv_obj_t, 
    copy:   *const obj::lv_obj_t
) -> LvglResult<*mut obj::lv_obj_t> {
    unsafe {
        let result = lv_obj_create(
            parent as *mut obj::lv_obj_t,
            copy as *const obj::lv_obj_t
        );
        if result.is_null() { Err(LvglError::SYS_EUNKNOWN) }
        else { Ok(result) }
    }
}

fn test() {
    unsafe {
        let screen = create(ptr::null_mut(), ptr::null())
            .expect("no screen");
        //  Get the coordinates of the object
        let coords = &(*screen).coords;
    }
}

fn test() {
    let result = create(ptr::null_mut(), ptr::null());
    if result.is_err() {
        //  Handle error
    }
    //  Unwrap the object inside the result
    let screen = result.unwrap();
    //  Get the coordinates of the object
    let coords = &(*screen).coords;
}

////

/// Populate the Time and Date Labels with the time and date. Called by screen_time_update_screen() above.
fn zzz_set_time_label(widgets: &WatchFaceWidgets, state: &WatchFaceState) -> LvglResult<()> {
    //  Create a string buffer with max size 6 to format the time
    type TimeBufSize = heapless::consts::U6;  //  Size of the string buffer
    let mut time_buf: heapless::String::<TimeBufSize> = 
        heapless::String::new();
    //  Format the time and set the label
    time_buf.clear();
    write!(&mut time_buf, "{:02}:{:02}\0",  //  Must terminate Rust strings with null
        state.time.hour,
        state.time.minute)
        .expect("time fail");
    label::set_text(widgets.time_label, &Strn::new(time_buf.as_bytes())) ? ;  //  TODO: Simplify
    Ok(())
}


//! Rhai Scripting for LVGL in WebAssembly (looks like Rust)
//! https://schungx.github.io/rhai/about/index.html
use core::{
    ptr,
    ffi::c_void,
};
use rhai::{
    Dynamic,
    Engine,
    EvalAltResult,
    RegisterFn,
    RegisterResultFn,
    // packages::{
    //     BasicStringPackage,
    //     Package,
    // },
};
use barebones_watchface::{
    watchface::{
        self,
        lvgl::{
            self,
            core::obj,
            draw::draw,
            widgets::{
                canvas,
                img,
                label,
            },
            mynewt::{
                // self,
                Strn,
            }
        },
    },
};

/// Global instance of the Rhai script engine
static mut ENGINE: Option<Engine> = None;

/// Init the Rhai script engine for LVGL
pub fn init() {
    //  Create the script engine
    let mut engine = Engine::new();

    //  Add callbacks to support `print()` and `debug()` in script
    engine.on_print(|x| println!("print: {}", x));
    engine.on_debug(|x| println!("DEBUG: {}", x));

    //  Load the Basic String Package into the script engine
    //  let package = BasicStringPackage::new();
    //  engine.load_package(package.get());

    //  Register the LVGL type
    //  engine.register_type::<draw::lv_draw_rect_dsc_t>();

    //  Register the LVGL functions
    engine.register_fn(
        "watchface_get_active_screen",  //  Name of Rhai function
        watchface::get_active_screen    //  LVGL function
    );
    engine.register_fn("ptr_null", ptr_null);  //  TODO: Rewrite as ptr::null
    engine.register_fn("new_rect", new_rect);
    engine.register_fn("get_canvas", get_canvas);
    engine.register_fn("canvas_draw_rect", canvas_draw_rect);  //  TODO: Rewrite as canvas::draw_rect
    engine.register_result_fn("label_create", label_create);  //  TODO: Rewrite as label::create
    engine.register_result_fn("label_set_text", label_set_text);  //  TODO: Rewrite as label::set_text
    engine.register_result_fn("obj_set_width", obj_set_width);  //  TODO: Rewrite as obj_set_width
    engine.register_result_fn("obj_set_height", obj_set_height);  //  TODO: Rewrite obj_set_height
    engine.register_get_set("radius", rect_get_radius, rect_set_radius);

    //  Create the canvas
    create_canvas();    

    //  Set the global script engine
    unsafe { ENGINE = Some(engine) };
}

/// Run a Rhai script that calls LVGL functions in WebAssembly
pub fn run_script(script: &str) -> Result<(), Box<EvalAltResult>> {
    if let Some(engine) = ENGINE {
        //  Execute the Rhai script
        //  let engine = unsafe { ENGINE.unwrap() };
        let result = engine.eval::<i64>(script) ? ;
        println!("Result: {}", result);        
    }
    Ok(())
}

/// Create the canvas for rendering graphics
fn create_canvas() {
    //  Create a static buffer for the canvas
    const CANVAS_WIDTH: i16  = 240;
    const CANVAS_HEIGHT: i16 = 240;
    const BYTES_PER_PIXEL: usize = 2;  //  2 bytes for RGB565 colour
    const CANVAS_SIZE: usize = CANVAS_WIDTH as usize * CANVAS_HEIGHT as usize * BYTES_PER_PIXEL;
    static mut BUF: [obj::lv_color_t ; CANVAS_SIZE] = 
        [ obj::lv_color_t{ full: 0 } ; CANVAS_SIZE];  //  Init canvas to black

    //  Create the canvas
    let screen = watchface::get_active_screen();
    unsafe {
        CANVAS = canvas::create(screen, ptr::null())
            .expect("create canvas fail");
    }

    //  Set the buffer for the canvas
    let buf: *mut [obj::lv_color_t] = unsafe { &mut BUF };
    canvas::set_buffer(unsafe { CANVAS }, buf as *mut c_void, CANVAS_WIDTH, CANVAS_HEIGHT, 
        img::LV_IMG_CF_TRUE_COLOR as u8
    ).expect("canvas set buffer fail");
}

/// Global instance of canvas for rendering graphics
static mut CANVAS: *mut obj::lv_obj_t = ptr::null_mut();

fn ptr_null() -> *const obj::lv_obj_t {
    ptr::null()
}

//  LVGL Functions mapped to Rhai calling convention
//  TODO: Generate automatically with the `safe_wrap` proc macro

/// Get the canvas
fn get_canvas() -> *mut obj::lv_obj_t {
    unsafe { CANVAS }
}

/// Create a rectangle
fn new_rect() -> draw::lv_draw_rect_dsc_t {
    let mut rect = draw::lv_draw_rect_dsc_t::default();
    draw::rect_dsc_init(&mut rect)
        .expect("rect init fail");
    rect
}

//// Get rectangle radius
fn rect_get_radius(rect: &mut draw::lv_draw_rect_dsc_t) -> i64 {
    rect.radius.into()
}

//// Set rectangle radius
fn rect_set_radius(rect: &mut draw::lv_draw_rect_dsc_t, new_val: i64) {
    rect.radius = new_val as obj::lv_style_int_t;
}

/// Draw the rectangle on the canvas
fn canvas_draw_rect(
    canvas: *mut obj::lv_obj_t, 
    x: i64,  //  TODO: canvas::lv_coord_t, 
    y: i64,  //  TODO: canvas::lv_coord_t, 
    width: i64,  //  TODO: canvas::lv_coord_t, 
    height: i64,  //  TODO: canvas::lv_coord_t, 
    rect_dsc: draw::lv_draw_rect_dsc_t
    //  TODO: Change to rect_dsc: canvas::lv_draw_rect_dsc_t
) {
    let rect2: *const canvas::lv_draw_rect_dsc_t = 
        unsafe { core::mem::transmute(&rect_dsc) };  //  TODO: Move draw::lv_draw_rect_dsc_t to canvas::lv_draw_rect_dsc_t
    canvas::draw_rect(
        canvas, 
        x as canvas::lv_coord_t, 
        y as canvas::lv_coord_t, 
        width as canvas::lv_coord_t, 
        height as canvas::lv_coord_t, 
        rect2
    ).expect("canvas draw rect fail");
}

/// Create a label
fn label_create(
    par: *mut obj::lv_obj_t, 
    copy: *const obj::lv_obj_t
) -> Result<Dynamic, Box<EvalAltResult>> {
    let result = label::create(par, copy)
        .expect("label_create fail");
    Ok(Dynamic::from(result))
}

/// Set label text
fn label_set_text(lbl: lvgl::Ptr, _s: &str) -> Result<Dynamic, Box<EvalAltResult>> {    
    let result = label::set_text(lbl, macros::strn!("TODO"))
        .expect("label_set_text fail");
    Ok(result.into())
}

/// Set widget width
pub fn obj_set_width(obj: *mut obj::lv_obj_t, w: obj::lv_coord_t) -> Result<Dynamic, Box<EvalAltResult>> {
    let result = obj::set_width(obj, w)
        .expect("obj_set_width fail");
    Ok(result.into())
}

/// Set widget height
pub fn obj_set_height(obj: *mut obj::lv_obj_t, h: obj::lv_coord_t) -> Result<Dynamic, Box<EvalAltResult>> {
    let result = obj::set_height(obj, h)
        .expect("obj_set_height fail");
    Ok(result.into())
}

/*
let screen = watchface::get_active_screen();
let lbl = label::create(screen, ptr::null()) ? ;  //  `?` will terminate the function in case of error
label::set_text(     lbl, strn!("00:00")) ? ;     //  strn creates a null-terminated string
obj::set_width(      lbl, 240) ? ;
obj::set_height(     lbl, 200) ? ;
*/
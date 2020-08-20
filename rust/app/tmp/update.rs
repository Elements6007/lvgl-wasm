`fn set_time_label(` <br>
<br>
&nbsp;&nbsp;`    widgets: &WatchFaceWidgets, ` <br>
&nbsp;&nbsp;`    state: &WatchFaceState) -> ` <br>
&nbsp;&nbsp;`    LvglResult<()> {` <br>
&nbsp;&nbsp;`    //  Create a static string buffer` <br>
&nbsp;&nbsp;`    static mut TIME_BUF: HString::<U6> = HString(IString::new());` <br>
<br>
&nbsp;&nbsp;`    unsafe {` <br>
&nbsp;&nbsp;&nbsp;&nbsp;`        //  Format the time` <br>
&nbsp;&nbsp;&nbsp;&nbsp;`        TIME_BUF.clear();` <br>
&nbsp;&nbsp;&nbsp;&nbsp;`        write!(&mut TIME_BUF, ` <br>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`            "{:02}:{:02}\0",` <br>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`            state.time.hour,` <br>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`            state.time.minute)` <br>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`            .expect("overflow");` <br>
<br>
&nbsp;&nbsp;&nbsp;&nbsp;`        //  Set the label` <br>
&nbsp;&nbsp;&nbsp;&nbsp;`        label::set_text(widgets.time_label, ` <br>
&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;&nbsp;`            &Strn::from_str(&TIME_BUF) ? ;` <br>
&nbsp;&nbsp;`    }` <br>
<br>
&nbsp;&nbsp;`    //  Return OK` <br>
&nbsp;&nbsp;`    Ok(())` <br>
`}` <br>
`int set_time_label(home_time_widget_t *ht) {` <br>
<br>
&nbsp;&nbsp;`    //  Create a string buffer on stack` <br>
&nbsp;&nbsp;`    char time[6];` <br>
<br>
&nbsp;&nbsp;`    //  Format the time` <br>
&nbsp;&nbsp;`    int res = snprintf(time, ` <br>
&nbsp;&nbsp;&nbsp;&nbsp;`        sizeof(time), ` <br>
&nbsp;&nbsp;&nbsp;&nbsp;`        "%02u:%02u", ` <br>
&nbsp;&nbsp;&nbsp;&nbsp;`        ht->time.hour,` <br>
&nbsp;&nbsp;&nbsp;&nbsp;`        ht->time.minute);` <br>
<br>
&nbsp;&nbsp;`if (res != sizeof(time) - 1) {` <br>
&nbsp;&nbsp;&nbsp;&nbsp;`LOG_ERROR("overflow");` <br>
&nbsp;&nbsp;&nbsp;&nbsp;`return -1;` <br>
&nbsp;&nbsp;`}` <br>
<br>
&nbsp;&nbsp;`//  Set the label` <br>
&nbsp;&nbsp;`lv_label_set_text(ht->lv_time, time);` <br>
<br>
&nbsp;&nbsp;`//  Return OK` <br>
&nbsp;&nbsp;`return 0;` <br>
`}` <br>
`lv_obj_t *screen_time_create(home_time_widget_t *ht) {` <br>
<br>
&nbsp;&nbsp;`    //  Create a label for time (00:00)` <br>
&nbsp;&nbsp;`    lv_obj_t *scr = lv_obj_create(NULL, NULL);` <br>
&nbsp;&nbsp;`    lv_obj_t *label1 = lv_label_create(scr, NULL);` <br>
<br>
&nbsp;&nbsp;`    lv_label_set_text(label1, "00:00");` <br>
&nbsp;&nbsp;`    lv_obj_set_width(label1, 240);` <br>
&nbsp;&nbsp;`    lv_obj_set_height(label1, 200);` <br>
&nbsp;&nbsp;`    ht->lv_time = label1;` <br>
&nbsp;&nbsp;`    ...` <br>
&nbsp;&nbsp;`    return scr;` <br>
`}` <br>
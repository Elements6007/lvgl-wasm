////  Mock Class
#ifndef DISPLAYAPP_H
#define DISPLAYAPP_H

/// Change LVGL v6 lv_label_set_style() to LVGL v7 lv_obj_reset_style_list() and lv_obj_add_style()
#define lv_label_set_style(label, style_type, style) \
{ \
    lv_obj_reset_style_list(label, LV_LABEL_PART_MAIN); \
    lv_obj_add_style(label, LV_LABEL_PART_MAIN, style); \
}

/// Used by Clock.cpp
namespace Pinetime { 
    namespace Applications { 
        namespace Screens {
            class Clock;
            static Clock *backgroundLabel_user_data = nullptr;
        } 
    } 
}

class DisplayApp {};

#endif  //  DISPLAYAPP_H

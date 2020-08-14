////  Mock Class

#ifndef BATTERYICON_H
#define BATTERYICON_H

class BatteryIcon {
  public:
    static const char *GetBatteryIcon(uint8_t batteryPercent) { return "%"; }
    static const char *GetPlugIcon(bool isCharging)  { return "C"; }
};

#endif  //  BATTERYICON_H

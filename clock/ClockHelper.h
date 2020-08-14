////  WebAssembly Helper
#ifndef CLOCKHELPER_H
#define CLOCKHELPER_H

#ifdef __cplusplus
extern "C" {
#endif

/// Create an instance of the clock
int create_clock(void);

/// Redraw the clock
int refresh_clock(void);

/// Update the clock time
int update_clock(void);

#ifdef __cplusplus
}  //  extern "C"
#endif
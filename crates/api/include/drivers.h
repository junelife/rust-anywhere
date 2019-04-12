#ifndef DRIVERS_H
#define DRIVERS_H

#include <stdbool.h>

/// Enumerate the available LEDs.
typedef enum LEDs {
    GREEN,
    RED
} LEDs;

/// Called by system at start.
extern void drivers_init(void);

/// Turn an LED on or off.
extern void drivers_set_led(LEDs which, bool on);

/// Check if the button is pressed or not.
extern bool drivers_is_button_pressed();

#endif // DRIVERS_H

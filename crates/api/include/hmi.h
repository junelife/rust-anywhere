#ifndef HMI_H
#define HMI_H

/// Called by system at start after the lvgl library has been initialized.
extern void hmi_init(void);

/// Called by the application loop before lvgl is updated, so it is safe to
/// update the contents of the UI.
extern void hmi_update(void);

#endif // HMI_H

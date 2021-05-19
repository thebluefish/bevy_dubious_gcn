# gcn_wup28

---

This library aims to be compatible with the Wii U Gamecube Controller adapter (WUP-028) and compatible 3rd-party adapters.

If your 3rd-party adapter has an optional PC mode, ensure the adapter is set to Wii U mode.

Supported on Windows and Linux.

---
##Windows

Your users must install the WinUSB.sys (or compatible) driver for their adapter. [Zadig](https://zadig.akeo.ie/) is the preferred choice.

- Open Zadig.
- click `Options` menu -> `List All Devices`.
- Select `WUP-028` in the drop-down.
- Ensure the USB ID matches `057E 0337`.
- Ensure `WinUSB` is selected as the driver.

###Warning: It is possible to install this driver over another USB device connected to your PC, rendering it unusable. This could be your mouse, keyboard, anything. Ensure **all** of the above steps are checked before installing the driver. I am not responsible for any problems you cause using this utility.

- Click `Install Driver`.

---

##Linux

The library should work out of the box in common Linux distros, but may need root to access your adapter.

To use your device without root:

- Create a USB rule file `/etc/udev/rules.d/75-wup-adapter.rules`
- Insert the line `SUBSYSTEM=="usb", ATTRS{idVendor}=="057e", ATTRS{idProduct}=="0337", TAG+="uaccess"`
- Save & reboot

---

###About WUP-28

The WUP-028 Gamecube (and compatible) adapter presents an HID interface, but gives an improper HID report. You can pull this from the `get_report` example and parse it with a utility such as [this](https://eleccelerator.com/usbdescreqparser/). Due to this, there are some necessary considerations:

- On MacOS, we cannot access the device without adding an exception to a protected system file that requires disabling SIP. I recommend **not** doing that and this library will make no attempt to support it.
- On Linux,

---

###FAQ

Q: Why not macOS?  
A: Since the adapter behaves as an improper HID device, macOS takes exclusive control of the device but cannot use it. [It's possible](https://github.com/libusb/libusb/wiki/FAQ#How_can_I_run_libusb_applications_under_Mac_OS_X_if_there_is_already_a_kernel_extension_installed_for_the_device_and_claim_exclusive_access) to work around this, but requires disabling SIP on later versions. **Do not** disable SIP.

Q: Help, I bricked a device in Windows because I failed to read the instructions!  
A: If you accidentally install this driver over an unintended device, [uninstall the driver](https://docs.microsoft.com/en-us/windows-hardware/drivers/install/using-device-manager-to-uninstall-devices-and-driver-packages) and then reinstall the device's original driver.

Q: Will other platforms be supported?  
A: Possible, but not currently planned! Web platforms will never be supported. Mobile devices may be supported, but will require an alternative to `rusb`.

---

###Other Notes

`usb_shenanigans` is derived from my USB playground stuff, and contains a stripped-down set of features to cover only what we need. I would advise against building your own project off it, and instead encourage you to leverage `rusb` and the usb/hid specs directly.

I originally wanted to add support for multiple controller types, including Switch Pro, Switch Joycons, and Dualshock; however working with HID devices directly is difficult as the OS prefers to handle them. Therefore, this library is unsuited for supporting other HID controllers.

---

###License: [The Unlicense](LICENSE)
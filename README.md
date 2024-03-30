# ESP 32 Rust test

## Export env variables

```bash
. $HOME/esp32/esp-idf/export.sh
```

## List USB connected, Windows

```bash
usbipd list
```

## Attach USB

```bash
usbipd attach --wsl=Ubuntu --busid=1-4
```

## See USB devices connected, Linux

```bash
lsusb
```

### link

[Tutorial de Rust para ESP32-C3](https://github.com/shanemmattner/ESP32-C3_Rust_Tutorials/tree/main/Tutorials)

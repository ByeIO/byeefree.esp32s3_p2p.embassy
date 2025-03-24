# micropython快速开发waveshare-esp32s3-geek开发板
- ~~其实是因为esp-rs编译失败😭~~

## 烧录micropython固件
```sh
python3 /opt/homebrew/bin/esptool.py -b 460800 --before default_reset --after hard_reset --chip esp32s3 write_flash --flash_mode dio --flash_size detect --flash_freq 80m 0x0 "/Users/workspace/Desktop/projects/ByeIO/software/exp210-esp32s3-p2p-embassy/micropython/waveshare-esp32s3-geek-micropython.bin"
```
输出:
```sh
esptool.py v4.7.0
Found 2 serial ports
Serial port /dev/cu.usbmodem1301
Connecting...
Chip is ESP32-S3 (QFN56) (revision v0.2)
Features: WiFi, BLE, Embedded PSRAM 2MB (AP_3v3)
Crystal is 40MHz
MAC: 18:8b:0e:cc:8a:24
Uploading stub...
Running stub...
Stub running...
Changing baud rate to 460800
Changed.
Configuring flash size...
Auto-detected Flash size: 16MB
Flash will be erased from 0x00000000 to 0x0018dfff...
Warning: Image file at 0x0 is protected with a hash checksum, so not changing the flash size setting. Use the --flash_size=keep option instead of --flash_size=16MB in order to remove this warning, or use the --dont-append-digest option for the elf2image command in order to generate an image file without a hash checksum
Compressed 1628240 bytes to 1066973...
Wrote 1628240 bytes (1066973 compressed) at 0x00000000 in 13.8 seconds (effective 943.3 kbit/s)...
Hash of data verified.

Leaving...
Hard resetting via RTS pin...
```
输出:
```sh
esptool.py v4.7.0
Found 2 serial ports
Serial port /dev/cu.usbmodem1401
Connecting...
Chip is ESP32-S3 (QFN56) (revision v0.2)
Features: WiFi, BLE, Embedded PSRAM 2MB (AP_3v3)
Crystal is 40MHz
MAC: 18:8b:0e:cc:94:30
Uploading stub...
Running stub...
Stub running...
Changing baud rate to 460800
Changed.
Configuring flash size...
Auto-detected Flash size: 16MB
Flash will be erased from 0x00000000 to 0x0018dfff...
Warning: Image file at 0x0 is protected with a hash checksum, so not changing the flash size setting. Use the --flash_size=keep option instead of --flash_size=16MB in order to remove this warning, or use the --dont-append-digest option for the elf2image command in order to generate an image file without a hash checksum
Compressed 1628240 bytes to 1066973...
Wrote 1628240 bytes (1066973 compressed) at 0x00000000 in 14.0 seconds (effective 927.3 kbit/s)...
Hash of data verified.

Leaving...
Hard resetting via RTS pin...
```

## 开发
- A(连接上位机)的MAC: 18:8b:0e:cc:8a:24 
- B(连接计算板)的MAC: 18:8b:0e:cc:94:30
使用Thonny IDE.
```sh
lsusb
303a:4001 (bus 0, device 5) path: 1.4
303a:4001 (bus 0, device 6) path: 1.2.3
```
最大包:250 Bytes

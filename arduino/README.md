# 基于esp-now的点对点通信(arduino-ide版)

## 编译命令
```sh
arduino-cli help core
# 创建配置文件
arduino-cli config init
arduino-cli config dump
arduino-cli config dump --verbose
# 编译命令
arduino-cli compile --fqbn "esp32:esp32:esp32s3:CDCOnBoot=cdc,FlashSize=16M,UploadSpeed=921600" --export-binaries
arduino-cli compile --upload --port /dev/cu.usbmodem1401 --fqbn "esp32:esp32:esp32s3:CDCOnBoot=cdc,FlashSize=16M,UploadSpeed=921600" --export-binaries
```

## 开发
- A(连接上位机)的MAC: 18:8b:0e:cc:8a:24 
- B(连接计算板)的MAC: 18:8b:0e:cc:94:30
```c
// 配置对端MAC地址(节点B的MAC地址), B节点MAC:18:8b:0e:cc:94:30
uint8_t PEER_MAC[] = {0x18, 0x8b, 0x0e, 0xcc, 0x94, 0x30};
// 配置对端MAC地址(节点A的MAC地址), A节点MAC:18:8b:0e:cc:8a:24 
uint8_t PEER_MAC[] = {0x18, 0x8b, 0x0e, 0xcc, 0x8a, 0x24}; 
```
使用Arduino IDE.
```sh
lsusb
303a:4001 (bus 0, device 5) path: 1.4
303a:4001 (bus 0, device 6) path: 1.2.3
```
最大包:250 Bytes

### 引脚定义
- USB接口默认为usb_cdc
- LCD_BL = 7
- LCD_DC = 8
- LCD_CS = 10
- LCD_SCK = 12
- LCD_MOSI = 11
- LCD_RST = 9

### 详细配置
```sh
(penv) (base) workspace@qsbyemabook2022 node_a % arduino-cli board details --fqbn esp32:esp32:esp32s3
Board name:            ESP32S3 Dev Module
FQBN:                  esp32:esp32:esp32s3
Board version:         2.0.11

Package name:          esp32
Package maintainer:    Espressif Systems
Package URL:           https://raw.kgithub.com/espressif/arduino-esp32/gh-pages/package_esp32_index.json
Package website:       https://github.com/espressif/arduino-esp32
Package online help:   http://esp32.com

Platform name:         esp32
Platform category:     ESP32
Platform architecture: esp32
Platform URL:          https://github.com/espressif/arduino-esp32/releases/download/2.0.11/esp32-2.0.11.zip
Platform file name:    esp32-2.0.11.zip
Platform size (bytes): 250401265
Platform checksum:     SHA-256:d15386308dc72f94816ce80b5508af999f2fd0d88eb5e1ffba48316ab0b9c5d6

Required tool: arduino:dfu-util                              0.11.0-arduino5
Required tool: esp32:esptool_py                              4.5.1
Required tool: esp32:mklittlefs                              3.0.0-gnu12-dc7f933
Required tool: esp32:mkspiffs                                0.2.3
Required tool: esp32:openocd-esp32                           v0.11.0-esp32-20221026
Required tool: esp32:riscv32-esp-elf-gcc                     esp-2021r2-patch5-8.4.0
Required tool: esp32:riscv32-esp-elf-gdb                     11.2_20220823
Required tool: esp32:xtensa-esp-elf-gdb                      11.2_20220823
Required tool: esp32:xtensa-esp32-elf-gcc                    esp-2021r2-patch5-8.4.0
Required tool: esp32:xtensa-esp32s2-elf-gcc                  esp-2021r2-patch5-8.4.0
Required tool: esp32:xtensa-esp32s3-elf-gcc                  esp-2021r2-patch5-8.4.0

Option:        Upload Speed                                                          UploadSpeed
               921600                                        ✔                       UploadSpeed=921600
               115200                                                                UploadSpeed=115200
               230400                                                                UploadSpeed=230400
               460800                                                                UploadSpeed=460800
Option:        USB Mode                                                              USBMode
               Hardware CDC and JTAG                         ✔                       USBMode=hwcdc
               USB-OTG (TinyUSB)                                                     USBMode=default
Option:        USB CDC On Boot                                                       CDCOnBoot
               Disabled                                      ✔                       CDCOnBoot=default
               Enabled                                                               CDCOnBoot=cdc
Option:        USB Firmware MSC On Boot                                              MSCOnBoot
               Disabled                                      ✔                       MSCOnBoot=default
               Enabled (Requires USB-OTG Mode)                                       MSCOnBoot=msc
Option:        USB DFU On Boot                                                       DFUOnBoot
               Disabled                                      ✔                       DFUOnBoot=default
               Enabled (Requires USB-OTG Mode)                                       DFUOnBoot=dfu
Option:        Upload Mode                                                           UploadMode
               UART0 / Hardware CDC                          ✔                       UploadMode=default
               USB-OTG CDC (TinyUSB)                                                 UploadMode=cdc
Option:        CPU Frequency                                                         CPUFreq
               240MHz (WiFi)                                 ✔                       CPUFreq=240
               160MHz (WiFi)                                                         CPUFreq=160
               80MHz (WiFi)                                                          CPUFreq=80
               40MHz                                                                 CPUFreq=40
               20MHz                                                                 CPUFreq=20
               10MHz                                                                 CPUFreq=10
Option:        Flash Mode                                                            FlashMode
               QIO 80MHz                                     ✔                       FlashMode=qio
               QIO 120MHz                                                            FlashMode=qio120
               DIO 80MHz                                                             FlashMode=dio
               OPI 80MHz                                                             FlashMode=opi
Option:        Flash Size                                                            FlashSize
               4MB (32Mb)                                    ✔                       FlashSize=4M
               8MB (64Mb)                                                            FlashSize=8M
               16MB (128Mb)                                                          FlashSize=16M
Option:        Partition Scheme                                                      PartitionScheme
               Default 4MB with spiffs (1.2MB APP/1.5MB SPIFFS) ✔                       PartitionScheme=default
               Default 4MB with ffat (1.2MB APP/1.5MB FATFS)                         PartitionScheme=defaultffat
               8M with spiffs (3MB APP/1.5MB SPIFFS)                                 PartitionScheme=default_8MB
               Minimal (1.3MB APP/700KB SPIFFS)                                      PartitionScheme=minimal
               No OTA (2MB APP/2MB SPIFFS)                                           PartitionScheme=no_ota
               No OTA (1MB APP/3MB SPIFFS)                                           PartitionScheme=noota_3g
               No OTA (2MB APP/2MB FATFS)                                            PartitionScheme=noota_ffat
               No OTA (1MB APP/3MB FATFS)                                            PartitionScheme=noota_3gffat
               Huge APP (3MB No OTA/1MB SPIFFS)                                      PartitionScheme=huge_app
               Minimal SPIFFS (1.9MB APP with OTA/190KB SPIFFS)                         PartitionScheme=min_spiffs
               16M Flash (2MB APP/12.5MB FATFS)                                      PartitionScheme=fatflash
               16M Flash (3MB APP/9.9MB FATFS)                                       PartitionScheme=app3M_fat9M_16MB
               RainMaker                                                             PartitionScheme=rainmaker
Option:        Core Debug Level                                                      DebugLevel
               None                                          ✔                       DebugLevel=none
               Error                                                                 DebugLevel=error
               Warn                                                                  DebugLevel=warn
               Info                                                                  DebugLevel=info
               Debug                                                                 DebugLevel=debug
               Verbose                                                               DebugLevel=verbose
Option:        PSRAM                                                                 PSRAM
               Disabled                                      ✔                       PSRAM=disabled
               QSPI PSRAM                                                            PSRAM=enabled
               OPI PSRAM                                                             PSRAM=opi
Option:        Arduino Runs On                                                       LoopCore
               Core 1                                        ✔                       LoopCore=1
               Core 0                                                                LoopCore=0
Option:        Events Run On                                                         EventsCore
               Core 1                                        ✔                       EventsCore=1
               Core 0                                                                EventsCore=0
Option:        Erase All Flash Before Sketch Upload                                  EraseFlash
               Disabled                                      ✔                       EraseFlash=none
               Enabled                                                               EraseFlash=all
Option:        JTAG Adapter                                                          JTAGAdapter
               Disabled                                      ✔                       JTAGAdapter=default
               Integrated USB JTAG                                                   JTAGAdapter=builtin
               FTDI Adapter                                                          JTAGAdapter=external
               ESP USB Bridge                                                        JTAGAdapter=bridge
Programmers:   ID                                            Name
               esptool                                       Esptool
```

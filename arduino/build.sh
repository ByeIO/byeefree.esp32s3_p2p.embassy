# 清空固件, 防止上传失败, 按住boot键插入usb
python3 /opt/homebrew/bin/esptool.py -b 460800 --before default_reset --after hard_reset --chip esp32s3 erase_flash
# arduino-cli命令
arduino-cli compile node_a
arduino-cli board listall esp32s3
# 查询编译参数
arduino-cli board details --fqbn esp32:esp32:esp32s3
arduino-cli compile --fqbn "esp32:esp32:esp32s3:CDCOnBoot=cdc,FlashSize=16M,UploadSpeed=921600" --export-binaries
arduino-cli compile --upload --port /dev/cu.usbmodem1234561 --fqbn "esp32:esp32:esp32s3:CDCOnBoot=cdc,FlashSize=16M,UploadSpeed=921600" --export-binaries
arduino-cli compile --profile sketch
# default_fqbn: esp32:esp32:esp32s3
ARDUINO_SKETCH_ALWAYS_EXPORT_BINARIES=true arduino-cli compile
# 编译并保存二进制文件
arduino-cli compile --export_binaries

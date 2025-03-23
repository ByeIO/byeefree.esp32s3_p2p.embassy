# 编译命令
# rustup add rustc 1.76
rustup toolchain add stable --profile minimal
cargo install espup
brew install libuv
cargo install espflash
cargo install cargo-espflash
curl -L https://github.com/esp-rs/espup/releases/latest/download/espup-aarch64-apple-darwin -o espup
curl -L https://github.com/esp-rs/espup/releases/latest/download/espup-x86_64-unknown-linux-gnu -o espup
chmod a+x espup
./espup install
# Source the following file in every terminal before building a project
. $HOME/export-esp.sh
rustup dafault stable
locate linkall.x
locate rom_functions.x

# 编译
# /Users/workspace/Desktop/projects/ByeIO/software/exp210-esp32s3-p2p-embassy/tools/esp/bin/cargo build
cargo espflash flash
export PATH=$PATH:/Users/workspace/Desktop/projects/ByeIO/software/exp210-esp32s3-p2p-embassy/tools/esp/xtensa-esp-elf/esp-14.2.0_20240906/xtensa-esp-elf/bin
xtensa-esp-elf-gcc

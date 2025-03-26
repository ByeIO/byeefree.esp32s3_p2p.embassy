cargo run --example embed_file
cargo run --example usbd_serial
cargo run --example usb_descriptor
sudo cargo run --example create_user_group
RUST_BACKTRACE=1 cargo run --example log_file_service && cat /tmp/log_file_service
cargo build --example service_installer
sudo cargo run --example service_installer
rustup target list | grep musl
rustup target list | grep aarch64
# RUSTFLAGS='-C target-feature=+crt-static' cargo build --example service_installer --target aarch64-unknown-linux-musl
# cargo build --example service_installer --target aarch64-unknown-none
# cargo build --example service_installer --target aarch64-linux-android
cargo zigbuild --release --example service_installer --target aarch64-unknown-linux-gnu
cargo zigbuild --release --example service_installer
cargo zigbuild --release --example log_file_service --target aarch64-unknown-linux-gnu
cargo-zigbuild run --example udp_base64
cargo-zigbuild run --release --example udp_tcp_multi_package > ./result/udp_tcp_multi_package.log
sudo cargo-zigbuild run --release --example tap_interface
cargo-zigbuild run --release --example tap_interface
cargo-zigbuild run --release --example nusb_attach
cargo-zigbuild run --release --example nusb_control
cargo-zigbuild run --release --example read_write_usb_cdc
cargo-zigbuild run --example wasmer_ubuntu
cargo-zigbuild run --release --example clap_byeefree

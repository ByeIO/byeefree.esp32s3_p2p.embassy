// 正式编译前添加编译器标志
use std::println;

fn main() {
    println!("即将开始编译, 添加编译器标志中...");
    println!("cargo:rustc-link-arg-bins=-Tlinkall.x");
    println!("cargo:rustc-link-arg-bins=-Trom_functions.x");
}

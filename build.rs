// 正式编译前添加编译器标志
use std::println;
use std::env;
use std::path::PathBuf;

fn main() {
    println!("即将开始编译, 添加编译器标志中...");
    
    // 获取项目根目录绝对路径
    let manifest_dir = PathBuf::from(env::var("CARGO_MANIFEST_DIR").unwrap());
    
    // 构建链接脚本的完整路径
    let linkall_path = manifest_dir.join("linkall.x");
    let rom_functions_path = manifest_dir.join("rom_functions.x");
    
    // 输出链接参数（带完整路径）
    println!("cargo:rustc-link-arg-bins=-T{}", linkall_path.display());
    println!("cargo:rustc-link-arg-bins=-T{}", rom_functions_path.display());
}

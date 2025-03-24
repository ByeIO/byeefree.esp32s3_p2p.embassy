#![allow(unused_variables)]

//! 嵌入文件并测试解包文件

// 标准库
use std::{fs, path::Path};
use std::error::Error;

// 嵌入文件
use embed_file::embed_bytes;

fn main() -> Result<(), Box<dyn Error>> {
    // 嵌入二进制文件到编译产物中 
    let service_file = embed_bytes!("../assets/byeefree_esp32_p2p.service");
    
    // 创建目标目录（如果不存在）
    let output_dir = Path::new("./result");
    // 递归创建目录
    fs::create_dir_all(output_dir)?;  
    
    // 构建输出文件路径 
    let output_path = output_dir.join("byeefree_esp32_p2p.service");
    
    // 将嵌入的二进制数据写入文件 
    fs::write(&output_path, service_file)?;
    
    println!("文件已保存至：{}", output_path.display());
    Ok(())
}

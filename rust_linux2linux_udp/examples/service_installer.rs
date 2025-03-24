#![allow(unused)]

//! byeefree_log_time服务安装器

// 标准库
use std::{fs, path::Path};
use std::error::Error;
use std::process::{Command, Stdio};
use std::io;
use std::os::unix::fs::PermissionsExt;
use std::env::consts::OS;

// 嵌入文件
use embed_file::embed_bytes;

fn main()-> Result<(), Box<dyn Error>>{
    // 新建用户组_ByeIO_和用户_Byeefree_
    let os_name = std::env::consts::OS;
    match os_name {
        "linux" => linux_create_user_group()?,
        "macos" => macos_create_user_group()?,
        _ => return Err(Box::new(io::Error::new(
            io::ErrorKind::Unsupported,
            format!("Unsupported OS: {}", os_name)
        )))
    };
    
    // 复制可执行文件和.service配置文件到对应目录
    /// 嵌入二进制文件到编译产物中 
    let service_file = embed_bytes!("../assets/byeefree_log_time.service");
    let plist_file = embed_bytes!("../assets/byeefree_log_time.plist");
    let exec_file = embed_bytes!("../assets/byeefree_log_time_service.exec");
    let elf_file = embed_bytes!("../assets/byeefree_log_time_service.linux.aarch64");
    
    /// 复制文件
    println!("复制文件中...");
    // 创建目标目录（如果不存在）
    let output_dir_1 = Path::new("/tmp/_Byeefree_");
    let output_dir_2 = Path::new("/etc/systemd/system");
    let output_dir_3 = Path::new("/Library/LaunchDaemons");
    
    // 递归创建目录
    fs::create_dir_all(output_dir_1)?;

    // 构建输出文件路径 
    let output_service_path = output_dir_2.join("byeefree_log_time.service");
    let output_plist_path = output_dir_3.join("byeefree_log_time.plist");
    let output_exec_path = output_dir_1.join("byeefree_log_time_service.exec");
    
    // 将嵌入的二进制数据写入文件 
    match os_name {
        "linux" => {
            // 递归创建目录
            fs::create_dir_all(output_dir_2)?;
            fs::write(&output_service_path, service_file)?;
            println!("文件已保存至：{}", output_service_path.display());
            fs::write(&output_exec_path, elf_file)?;
            println!("文件已保存至：{}", output_exec_path.display());
        },
        "macos" => {
            // 递归创建目录
            fs::create_dir_all(output_dir_3)?;
            fs::write(&output_plist_path, plist_file)?;
            println!("文件已保存至：{}", output_plist_path.display());
            fs::write(&output_exec_path, exec_file)?;
            println!("文件已保存至：{}", output_exec_path.display());
        },
        _ => {},
    }
    
    // 配置可执行文件权限及所有权
    fs::set_permissions(output_exec_path.clone(), fs::Permissions::from_mode(0o755))?;
    
    let chown_status = Command::new("chown")
        .arg("_Byeefree_:_ByeIO_")
        .arg(output_exec_path)
        .status()?;
    if !chown_status.success() {
        return Err("Failed to set executable ownership".into());
    }else{
        println!("文件添加可执行权限成功.");
    }

    // 设置服务文件权限（仅限Linux）
    match os_name {
        "linux" => {
            fs::set_permissions(output_service_path.clone(), fs::Permissions::from_mode(0o644))?;
        },
        "macos" => {
            fs::set_permissions(output_plist_path.clone(), fs::Permissions::from_mode(0o644))?;
        },
        _ => {},
    }
    
    println!("服务配置文件添加权限成功.");
    
    // 启动服务并设置开机自启
    match os_name {
        "linux" => {
            Command::new("systemctl")
                .args(&["daemon-reload"])
                .status()?;
            println!("linux服务配置重载成功.");
            Command::new("systemctl")
                .args(&["start", "byeefree_log_time.service"])
                .status()?;
            println!("linux服务启动成功.");
            Command::new("systemctl")
                .args(&["enable", "byeefree_log_time.service"])
                .status()?;
            println!("linux服务设置开机自启成功.");
        },
        "macos" => {
            Command::new("launchctl")
                .args(&["load", "-w", output_plist_path.clone().to_str().unwrap()])
                .status()?;
            println!("macOS服务设置开机自启成功.");
        },
        _ => {}
    }

    // 全部任务完成
    println!("安装服务完成, 输入`cat /tmp/log_file_service`查看效果");
    Ok(())
}

fn linux_create_user_group() -> io::Result<()> {
    use std::io::{self, ErrorKind, Error};

    // 检查用户组是否存在
    let group_exists = Command::new("getent")
        .arg("group")
        .arg("_ByeIO_")
        .status()?
        .success();

    if !group_exists {
        // 创建用户组
        let group_output = Command::new("groupadd")
            .arg("_ByeIO_")
            .output()?;

        if !group_output.status.success() {
            return Err(Error::new(
                ErrorKind::Other,
                format!("用户组创建失败: {}", String::from_utf8_lossy(&group_output.stderr))
            ));
        }
    }

    // 检查用户是否存在
    let user_exists = Command::new("id")
        .arg("-u")
        .arg("_Byeefree_")
        .status()?
        .success();

    if !user_exists {
        // 创建用户
        let user_output = Command::new("useradd")
            .args(&["-g", "_ByeIO_", "_Byeefree_"])
            .output()?;

        if !user_output.status.success() {
            return Err(Error::new(
                ErrorKind::Other,
                format!("用户创建失败: {}", String::from_utf8_lossy(&user_output.stderr))
            ));
        }
    }

    Ok(())
}

fn macos_create_user_group() -> io::Result<()> {
    use std::io::{self, ErrorKind, Error};

    // 检查用户组是否存在
    let group_exists = Command::new("sudo")
        .args(&["dscl", ".", "-read", "/Groups/_ByeIO_"])
        .status()
        .map(|s| s.success())?;

    if !group_exists {
        // 创建用户组
        let group_create = Command::new("sudo")
            .args(&["dscl", ".", "-create", "/Groups/_ByeIO_"])
            .status()?;

        if !group_create.success() {
            return Err(Error::new(ErrorKind::Other, "macOS用户组创建失败"));
        }
    }

    // 检查用户是否存在
    let user_exists = Command::new("sudo")
        .args(&["dscl", ".", "-read", "/Users/_Byeefree_"])
        .status()
        .map(|s| s.success())?;

    if !user_exists {
        // 创建用户属性
        let user_commands = [
            ("/Users/_Byeefree_", "UserShell", "/bin/bash"),
            ("/Users/_Byeefree_", "RealName", "ByeIO Service Account"),
            ("/Users/_Byeefree_", "PrimaryGroupID", "2000"),
        ];

        for (path, key, value) in &user_commands {
            let status = Command::new("sudo")
                .args(&["dscl", ".", "-create", path, key, value])
                .status()?;
            if !status.success() {
                return Err(Error::new(
                    ErrorKind::Other,
                    format!("用户属性创建失败: {}/{}/{}", path, key, value)
                ));
            }
        }

        // 将用户加入组
        let append_status = Command::new("sudo")
            .args(&["dscl", ".", "-append", "/Groups/_ByeIO_", "GroupMembership", "_Byeefree_"])
            .status()?;
        if !append_status.success() {
            return Err(Error::new(ErrorKind::Other, "无法将用户添加到组"));
        }
    }

    Ok(())
}

//! 创建系统用户和用户组

use std::process::Command;
use std::io::{self, Error, ErrorKind};
use std::process::Stdio;

fn main() -> io::Result<()> {
    let os_name = std::env::consts::OS;
    
    match os_name {
        "linux" => linux_create_user_group(),
        "macos" => macos_create_user_group(),
        _ => Err(Error::new(ErrorKind::Unsupported,
            format!("Unsupported OS: {}", os_name)))
    }
}

fn linux_create_user_group() -> io::Result<()> {
    // 创建用户组 _ByeIO_
    let group_output = Command::new("groupadd")
        .arg("_ByeIO_") 
        .output()?;     
    
    println!("创建用户组命令输出：");
    println!("stdout: {}", String::from_utf8_lossy(&group_output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&group_output.stderr));

    if !group_output.status.success() {
        return Err(Error::new(ErrorKind::Other, 
            format!("创建用户组失败，退出码: {}", group_output.status)));
    }

    // 创建用户 _ByeIO_ 并指定主组
    let user_output = Command::new("useradd")
        .arg("-g")          
        .arg("_ByeIO_")     
        .arg("_ByeIO_")     
        .output()?;

    println!("\n创建用户命令输出：");
    println!("stdout: {}", String::from_utf8_lossy(&user_output.stdout));
    println!("stderr: {}", String::from_utf8_lossy(&user_output.stderr));

    if !user_output.status.success() {
        return Err(Error::new(ErrorKind::Other,
            format!("创建用户失败，退出码: {}", user_output.status)));
    }

    Ok(())
}

fn macos_create_user_group() -> io::Result<()> {
    let group_name = "_ByeIO_";
    let user_name = "_Byeefree_";

    // 创建用户组
    let group_output = Command::new("sudo")
        .args(&["dscl", ".", "-create", &format!("/Groups/{}", group_name)])
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()?
        .wait_with_output()?;

    if !group_output.status.success() {
        return Err(Error::new(
            ErrorKind::Other,
            format!(
                "macOS group creation failed: {}",
                String::from_utf8_lossy(&group_output.stderr)
            ),
        ));
    }

    // 提前创建路径字符串
    let user_path = format!("/Users/{}", user_name);
    let group_path = format!("/Groups/{}", group_name);

    let user_commands = vec![
        vec!["-create", &user_path],
        vec!["-create", &user_path, "UserShell", "/bin/bash"],
        vec!["-create", &user_path, "RealName", "ByeIO Service Account"],
        vec!["-create", &user_path, "PrimaryGroupID", "2000"],
        vec!["-append", &group_path, "GroupMembership", user_name],
    ];

    for cmd in &user_commands {
        let output = Command::new("sudo")
            .args(["dscl", "."].iter().chain(cmd.iter()))
            .output()?;

        if !output.status.success() {
            return Err(Error::new(
                ErrorKind::Other,
                format!(
                    "macOS user setup failed at step {:?}: {}",
                    cmd,
                    String::from_utf8_lossy(&output.stderr)
                ),
            ));
        }
    }

    println!("macOS 用户和组创建成功");
    Ok(())
}

#[cfg(not(unix))]
fn main() {
    eprintln!("错误：当前操作系统 {} 不支持此操作", std::env::consts::OS);
    std::process::exit(1);
}

#![allow(unused)]
#![allow(deprecated)]

//! 将`hello`打包为udp包并编码为base64然后打印

// 网络协议相关
use edge_raw::udp;
use core::net::{Ipv4Addr, SocketAddrV4};

// base64编码
use base64;

fn main() {
    // 创建源地址和目标地址（示例使用本地回环地址和随机端口）
    // 源端口1234
    let src_addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 1234); 
    // 目标端口5678
    let dst_addr = SocketAddrV4::new(Ipv4Addr::new(127, 0, 0, 1), 5678); 

    // 创建足够大的缓冲区（UDP头8字节 + 数据长度）
    let mut buffer = [0u8; 233];

    // 使用UDP编码函数打包数据
    let udp_packet = udp::encode(&mut buffer, src_addr, dst_addr, |payload_buf| {
        // 要发送的数据
        let data = b"hello"; 
        
        // 检查缓冲区长度是否足够
        if payload_buf.len() < data.len() {
            return Err(edge_raw::Error::BufferOverflow);
        }
        
        // 将数据拷贝到缓冲区
        payload_buf[..data.len()].copy_from_slice(data);
        // 返回实际写入长度
        Ok(data.len()) 
    })
    // 处理编码错误
    .expect("UDP数据包编码失败"); 

    // 进行Base64编码
    let base64_str = base64::encode(udp_packet);
    
    // 打印编码结果
    println!("Base64编码的UDP包: {}", base64_str);
}

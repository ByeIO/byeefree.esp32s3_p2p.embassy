#![allow(unused)]
#![allow(deprecated)]

//! 大载荷拆为多个包传输

// 标准库
use std::{fs, path::Path};
use std::error::Error;

// 嵌入文件
use embed_file::embed_bytes;

// 网络协议相关
use edge_nal;
use core::net::{Ipv4Addr, SocketAddrV4};

// base64编码
use base64;

// 网络传输参数
const SRC_ADDR : [u8; 4] = [10, 168, 233, 233];
const SRC_PORT : u16 = 8888;
const DEST_ADDR : [u8; 4] = [10, 168, 233, 234];
const DEST_PORT : u16 = 8888;

fn main() {
    // 从文件中获取数据
    let payload_from_file = embed_a_file().expect("文件读取失败");
    let payloads_from_file_base64 = base64_encode(&payload_from_file[..]).expect("转为base64失败");
    println!("原始数据: {}", String::from_utf8_lossy(&payloads_from_file_base64));
    
    // 分割载荷数据以适合MTU
    let payloads = split_payload_to_fit_mtu(&payload_from_file).expect("数据分割失败");
    
    // 遍历每个分割后的数据包
    for payload in payloads {
        // 打包为UDP包并打印
        let udp_packet = pack_udp_with_ip(SRC_ADDR, SRC_PORT, DEST_ADDR, DEST_PORT, &payload[..]).expect("UDP打包失败");
        let udp_packet_base64 = base64_encode(&udp_packet[..]).expect("转为base64失败");
        println!("UDP数据包: {}", String::from_utf8_lossy(&udp_packet_base64));
        
        // 打包为TCP包并打印
        let tcp_packet = pack_tcp_with_ip(SRC_ADDR, SRC_PORT, DEST_ADDR, DEST_PORT, &payload[..]).expect("TCP打包失败");
        let tcp_packet_base64 = base64_encode(&tcp_packet[..]).expect("转为base64失败");
        println!("TCP数据包: {}", String::from_utf8_lossy(&tcp_packet_base64));
    }
}

/// 读取文件数据
fn embed_a_file() -> Result<Vec<u8>, std::io::Error> {
    // 嵌入二进制文件到编译产物中 
    let service_file = embed_bytes!("../assets/byeefree_log_time_service.exec");
    
    Ok(service_file.to_vec())
}

/// 数据拆分为符合MTU要求(单个数据载荷最大208或156(base64)Bytes)
fn split_payload_to_fit_mtu(payload: &[u8]) -> Result<Vec<Vec<u8>>, std::io::Error> {
    const MTU_SIZE: usize = 250;
    // const PAYLOAD_SIZE: usize = 208;
    
    // base64编码导致长度增加33.3%, 所以需要预留空间
    const PAYLOAD_SIZE : usize = 156; 
    let mut result = Vec::new();
    let mut remaining = payload;

    // 不足208 Bytes直接装入Vec
    while !remaining.is_empty() {
        let chunk_size = std::cmp::min(PAYLOAD_SIZE, remaining.len());
        let chunk = remaining[..chunk_size].to_vec();
        result.push(chunk);
        remaining = &remaining[chunk_size..];
    }

    Ok(result)
}

/// 封装udp数据包, 返回Vec<u8>防止悬垂引用
fn pack_udp_with_ip(src_addr: [u8; 4], src_port: u16, dst_addr: [u8; 4], dst_port: u16, payload: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    // IP包头结构
    // 版本: 4位
    // 头长度: 4位
    // 服务类型: 8位
    // 总长度: 16位
    // 标识: 16位
    // 标志: 3位
    // 片偏移: 13位
    // 生存时间: 8位
    // 协议: 8位
    // 头校验和: 16位
    // 源IP地址: 32位
    // 目的IP地址: 32位
    // 选项: 可变长度(可选)
    // IP包头的最小大小为20字节
    
    // UDP包头结构
    // 源端口: 16位
    // 目的端口: 16位
    // 长度: 16位
    // 校验和: 16位
    // UDP包头的最小大小为8字节
    
    use edge_raw::udp;
    use core::net::{Ipv4Addr, SocketAddrV4};

    // 创建源地址和目标地址（使用本地回环地址）
    let src_addr = SocketAddrV4::new(Ipv4Addr::new(src_addr[0], src_addr[1], src_addr[2], src_addr[3]), src_port);
    let dst_addr = SocketAddrV4::new(Ipv4Addr::new(dst_addr[0], dst_addr[1], dst_addr[2], dst_addr[3]), dst_port);

    // 创建足够大的缓冲区（IP头20字节 + UDP头8字节 + 数据长度）, MTU=250 Bytes
    let mut buffer = [0u8; 250];

    // 使用UDP编码函数打包数据
    let udp_packet = udp::encode(&mut buffer, src_addr, dst_addr, |payload_buf| {
        // 检查缓冲区长度是否足够
        if payload_buf.len() < payload.len() {
            return Err(edge_raw::Error::BufferOverflow);
        }
        
        // 将数据拷贝到缓冲区
        payload_buf[..payload.len()].copy_from_slice(payload);
        // 返回实际写入长度
        Ok(payload.len())
    }).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    // 返回Vec<u8>防止悬垂引用
    Ok(udp_packet.to_vec())
}

/// 封装tcp数据包, 返回Vec<u8>防止悬垂引用
fn pack_tcp_with_ip(src_addr: [u8; 4], src_port: u16, dst_addr: [u8; 4], dst_port: u16, payload: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    // IP包头结构
    // 版本: 4位
    // 头长度: 4位
    // 服务类型: 8位
    // 总长度: 16位
    // 标识: 16位
    // 标志: 3位
    // 片偏移: 13位
    // 生存时间: 8位
    // 协议: 8位
    // 头校验和: 16位
    // 源IP地址: 32位
    // 目的IP地址: 32位
    // 选项: 可变长度(可选)
    // IP包头的最小大小为20字节
    
    // TCP包头结构
    // 源端口: 16位
    // 目标端口: 16位
    // 序列号: 32位
    // 回应序号: 32位
    // TCP头长度: 4位
    // reserved: 6位
    // 控制代码: 6位
    // 窗口大小: 16位
    // 偏移量: 16位
    // 校验和: 16位
    // 选项: 32位(可选)
    // TCP包头的最小大小为20字节
    
    use edge_raw::tcp;
    use core::net::{Ipv4Addr, SocketAddrV4};
    
    // 计算总长度（IP头20字节 + TCP头20字节 + 数据长度）
    let total_length = 20 + 20 + payload.len();

    // 检查是否超过MTU限制
    if total_length > 250 {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            "Packet size exceeds MTU limit",
        ));
    }

    // 创建源地址和目标地址（使用本地回环地址）
    let src_addr = SocketAddrV4::new(Ipv4Addr::new(src_addr[0], src_addr[1], src_addr[2], src_addr[3]), src_port);
    let dst_addr = SocketAddrV4::new(Ipv4Addr::new(dst_addr[0], dst_addr[1], dst_addr[2], dst_addr[3]), dst_port);

    // 创建足够大的缓冲区（IP头20字节 + TCP头20字节 + 数据长度）, MTU=250 Bytes
    // let mut buffer = [0u8; 250];
    let mut buffer = vec![0u8; total_length];

    // 使用TCP编码函数打包数据
    let tcp_packet = tcp::encode(&mut buffer, src_addr, dst_addr, |payload_buf| {
        // 检查缓冲区长度是否足够
        if payload_buf.len() < payload.len() {
            return Err(edge_raw::Error::BufferOverflow);
        }
        
        // 将数据拷贝到缓冲区
        payload_buf[..payload.len()].copy_from_slice(payload);
        
        println!("实际长度:{}", payload.len());
        
        // 返回实际写入长度
        Ok(payload.len())
    }).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    // 返回Vec<u8>防止悬垂引用
    Ok(tcp_packet.to_vec())
    
}

/// 封装ip数据包, 返回Vec<u8>防止悬垂引用
fn pack_ip(src_addr: [u8; 4], dst_addr: [u8; 4], protocol: u8, payload: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    // IP包头结构
    // 版本: 4位
    // 头长度: 4位
    // 服务类型: 8位
    // 总长度: 16位
    // 标识: 16位
    // 标志: 3位
    // 片偏移: 13位
    // 生存时间: 8位
    // 协议: 8位
    // 头校验和: 16位
    // 源IP地址: 32位
    // 目的IP地址: 32位
    // 选项: 可变长度(可选)
    // IP包头的最小大小为20字节
    
    use core::net::Ipv4Addr;
    use edge_raw::ip::encode;

    // 将u8数组转换为Ipv4Addr
    let src_ip = Ipv4Addr::new(src_addr[0], src_addr[1], src_addr[2], src_addr[3]);
    let dst_ip = Ipv4Addr::new(dst_addr[0], dst_addr[1], dst_addr[2], dst_addr[3]);

    // 创建足够大的缓冲区（IP头20字节 + 数据长度）
    let mut buffer = [0u8; 250];

    // 使用IP编码函数打包数据
    let ip_packet = encode(&mut buffer, src_ip, dst_ip, protocol, |payload_buf| {
        // 检查缓冲区长度是否足够
        if payload_buf.len() < payload.len() {
            return Err(edge_raw::Error::BufferOverflow);
        }
        
        // 将数据拷贝到缓冲区
        payload_buf[..payload.len()].copy_from_slice(payload);
        // 返回实际写入长度
        Ok(payload.len())
    }).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    // 返回Vec<u8>防止悬垂引用
    Ok(ip_packet.to_vec())
}

/// base64编码, 返回Vec<u8>防止悬垂引用
fn base64_encode(data: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    use base64;
    let output = base64::encode(data);
    Ok(output.into_bytes())
}

/// base64解码, 返回Vec<u8>防止悬垂引用
fn base64_decode(data: &[u8]) -> Result<Vec<u8>, std::io::Error> {
    use base64;
    let output = base64::decode(data).map_err(|e| std::io::Error::new(std::io::ErrorKind::InvalidData, e))?;
    Ok(output)
}

/// 模拟esp_now的半双工通信, 返回Vec<u8>防止悬垂引用
fn esp_now_half_duplex_pipe(data: &[u8])->Result<&[u8], std::io::Error>{
    const MAX_PACKAGE_SIZE: usize = 250;
    
    // 超过大小
    if data.len() > MAX_PACKAGE_SIZE {
        return Err(std::io::Error::new(
            std::io::ErrorKind::InvalidInput,
            format!("Data package exceeds maximum size of {} bytes", MAX_PACKAGE_SIZE)
        ));
    }
    
    // 没有超过大小
    Ok(data)
}

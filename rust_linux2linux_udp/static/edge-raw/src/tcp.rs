#![allow(unused_imports)]
use log::trace;
use core::net::{Ipv4Addr, SocketAddrV4};
use super::bytes::{BytesIn, BytesOut};
use super::{checksum_accumulate, checksum_finish, Error};

/// 表示一个TCP数据包头部
#[derive(Clone, Debug)]
pub struct TcpPacketHeader {
    pub src_port: u16,      // 源端口
    pub dst_port: u16,      // 目标端口
    pub seq_num: u32,       // 序列号
    pub ack_num: u32,       // 确认号
    pub data_offset: u8,    // 数据偏移（头部长度）
    pub flags: u8,          // 控制标志位
    pub window_size: u16,   // 窗口大小
    pub checksum: u16,      // 校验和
    pub urgent_ptr: u16,    // 紧急指针
}

impl TcpPacketHeader {
    pub const PROTO: u8 = 6;  // TCP协议号
    pub const MIN_SIZE: usize = 20;  // TCP头部最小长度

    /// 创建一个新的TCP头部实例
    pub fn new(src_port: u16, dst_port: u16) -> Self {
        Self {
            src_port,
            dst_port,
            seq_num: 0,
            ack_num: 0,
            data_offset: ((Self::MIN_SIZE / 4) as u8) << 4, // 左移4位
            flags: 0,
            window_size: 0,
            checksum: 0,
            urgent_ptr: 0,
        }
    }

    /// 从字节切片解码TCP头部
    pub fn decode(data: &[u8]) -> Result<Self, Error> {
        let mut bytes = BytesIn::new(data);

        Ok(Self {
            src_port: u16::from_be_bytes(bytes.arr()?),
            dst_port: u16::from_be_bytes(bytes.arr()?),
            seq_num: u32::from_be_bytes(bytes.arr()?),
            ack_num: u32::from_be_bytes(bytes.arr()?),
            data_offset: bytes.byte()?,
            flags: bytes.byte()?,
            window_size: u16::from_be_bytes(bytes.arr()?),
            checksum: u16::from_be_bytes(bytes.arr()?),
            urgent_ptr: u16::from_be_bytes(bytes.arr()?),
        })
    }

    /// 将TCP头部编码到提供的缓冲区
    pub fn encode<'o>(&self, buf: &'o mut [u8]) -> Result<&'o [u8], Error> {
        let mut bytes = BytesOut::new(buf);

        bytes
            .push(&u16::to_be_bytes(self.src_port))?
            .push(&u16::to_be_bytes(self.dst_port))?
            .push(&u32::to_be_bytes(self.seq_num))?
            .push(&u32::to_be_bytes(self.ack_num))?
            .byte(self.data_offset)?
            .byte(self.flags)?
            .push(&u16::to_be_bytes(self.window_size))?
            .push(&u16::to_be_bytes(self.checksum))?
            .push(&u16::to_be_bytes(self.urgent_ptr))?;

        let len = bytes.len();
        Ok(&buf[..len])
    }

    /// 计算TCP数据包的校验和
    pub fn checksum(packet: &[u8], src: Ipv4Addr, dst: Ipv4Addr) -> u16 {
        let mut buf = [0; 12];

        // 伪IP头部用于TCP校验和计算
        let len = BytesOut::new(&mut buf)
            .push(&u32::to_be_bytes(src.into()))
            .unwrap()
            .push(&u32::to_be_bytes(dst.into()))
            .unwrap()
            .byte(0)
            .unwrap()
            .byte(Self::PROTO)
            .unwrap()
            .push(&u16::to_be_bytes(packet.len() as u16))
            .unwrap()
            .len();

        let sum = checksum_accumulate(&buf[..len], usize::MAX)
            + checksum_accumulate(packet, 0);

        checksum_finish(sum)
    }
    
    /// 解码TCP头部和有效载荷
    pub fn decode_with_payload(
        packet: &[u8],
        src: Ipv4Addr,
        dst: Ipv4Addr,
        filter_src: Option<u16>,
        filter_dst: Option<u16>,
    ) -> Result<Option<(Self, &[u8])>, Error> {
        let hdr = Self::decode(packet)?;

        if let Some(filter_src) = filter_src {
            if filter_src != hdr.src_port {
                return Ok(None);
            }
        }

        if let Some(filter_dst) = filter_dst {
            if filter_dst != hdr.dst_port {
                return Ok(None);
            }
        }

        let data_offset = (hdr.data_offset >> 4) as usize * 4;
        if packet.len() < data_offset {
            return Err(Error::DataUnderflow);
        }

        let checksum = Self::checksum(packet, src, dst);
        if checksum != hdr.checksum {
            return Err(Error::InvalidChecksum);
        }

        let payload = &packet[data_offset..];
        Ok(Some((hdr, payload)))
    }

    /// 编码TCP头部和有效载荷
    pub fn encode_with_payload<'o, F>(
        &mut self,
        buf: &'o mut [u8],
        src: Ipv4Addr,
        dst: Ipv4Addr,
        encoder: F,
    ) -> Result<&'o [u8], Error>
    where
        F: FnOnce(&mut [u8]) -> Result<usize, Error>,
    {
        let data_offset = (self.data_offset >> 4) as usize * 4;
        if buf.len() < data_offset {
            return Err(Error::BufferOverflow);
        }
        
        // 提前克隆以防止引用借用问题
        let buf_vec = buf.to_vec();
        let (hdr_buf, payload_buf) = buf.split_at_mut(data_offset);
        let payload_len = encoder(payload_buf)?;
    
        let total_len = data_offset + payload_len;
        
        // 克隆需要计算校验和的部分数据
        let checksum_data = buf_vec[..total_len].to_vec();
        self.checksum = Self::checksum(&checksum_data, src, dst);
    
        self.encode(hdr_buf)?;
        Ok(&buf[..total_len])
    }
    
}

/// 解码TCP数据包
pub fn decode(
    src: Ipv4Addr,
    dst: Ipv4Addr,
    packet: &[u8],
    filter_src: Option<u16>,
    filter_dst: Option<u16>,
) -> Result<Option<(SocketAddrV4, SocketAddrV4, &[u8])>, Error> {
    let data = TcpPacketHeader::decode_with_payload(packet, src, dst, filter_src, filter_dst)?.map(
        |(hdr, payload)| {
            (
                SocketAddrV4::new(src, hdr.src_port),
                SocketAddrV4::new(dst, hdr.dst_port),
                payload,
            )
        },
    );

    Ok(data)
}

/// 编码TCP数据包
pub fn encode<F>(
    buf: &mut [u8],
    src: SocketAddrV4,
    dst: SocketAddrV4,
    payload: F,
) -> Result<&[u8], Error>
where
    F: FnOnce(&mut [u8]) -> Result<usize, Error>,
{
    let mut hdr = TcpPacketHeader::new(src.port(), dst.port());

    hdr.encode_with_payload(buf, *src.ip(), *dst.ip(), |buf| payload(buf))
}

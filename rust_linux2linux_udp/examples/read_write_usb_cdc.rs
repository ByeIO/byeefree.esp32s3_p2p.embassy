#![allow(unused_imports)]
#![allow(dead_code)]
#![allow(unused_doc_comments)]
#![allow(unused)]

use nusb::{
    /* Interface, */
    transfer::{ControlIn, ControlOut, ControlType, Recipient::{Interface, Endpoint}, RequestBuffer },
    Error as NusbError, MaybeFuture,
};
use std::time::Duration;
use std::sync::Arc;

// CDC类特定请求类型（参考USB CDC规范）
const SET_LINE_CODING: u8 = 0x20;
const GET_LINE_CODING: u8 = 0x21;
const SET_CONTROL_LINE_STATE: u8 = 0x22;

// esp32s3 JTAG/serial多用途usb设备
const ESP32S3_VID : u16 = 0x303a;
const ESP32S3_PID : u16 = 0x1001;

fn main(){
    // 获取usb-cdc设备实例
    // let serial_port = SerialPort::new(ESP32S3_VID, ESP32S3_PID).unwrap();
}

/// 自定义USB错误类型
// Error::new(ErrorKind::Other, "oh no!");

/// USB-CDC 串口设备结构体
pub struct SerialPort {
    // 接口
    // interface: Interface,
    // 大量数据写入 device->host
    // bulk_in: Endpoint,
    // 大量数据读出 host->device
    // bulk_out: Endpoint,
}

impl SerialPort {
    /// 创建并初始化USB-CDC设备
    // pub fn new(vid: u16, pid: u16) -> Result<Self, std::io::Error> {
    pub fn new(vid: u16, pid: u16){
        // 查找指定VID/PID的设备
        let di = nusb::list_devices()
            .wait()
            .unwrap()
            .find(|d| d.vendor_id() == vid && d.product_id() == pid)
            .expect("设备未连接");
        println!("设备已连接");
 
        // 打开设备并声明接口
        // 通常使用接口0
        // let device = di.open().wait().unwrap();
        // let interface = device.claim_interface(0).wait().unwrap();

        // 配置串口参数
        // configure_serial_port(&interface)?;
        
        /// A buffer for requesting an IN transfer.
        ///
        /// A `RequestBuffer` is passed when submitting an `IN` transfer to define the
        /// requested length and provide a buffer to receive data into. The buffer is
        /// returned in the [`Completion`][`crate::transfer::Completion`] as a `Vec<u8>`
        /// with the data read from the endpoint. The `Vec`'s allocation can turned back
        /// into a `RequestBuffer` to re-use it for another transfer.
        ///
        /// You can think of a `RequestBuffer` as a `Vec` with uninitialized contents.
        let mut buf = [0u8; 256];
        // let mut req_buf_in = RequestBuffer{buf: &mut buf, capacity: 16, requested: 16};
        // // let mut req_buf_out = RequestBuffer{buf: &mut buf, capacity: 16, requested: 16};
        // let req_buf_out = Vec::<u8>;
        // pub struct RequestBuffer {
        //     pub(crate) buf: *mut u8,
        //     pub(crate) capacity: usize,
        //     pub(crate) requested: usize,
        // }

        // 获取数据端点（假设端点号：输出0x01，输入0x81）
        // let bulk_out = interface.bulk_out(0x01, req_buf_out);
        // let bulk_in = interface.bulk_in(0x81, req_buf_in);

        // 设置端点超时时间为0（非阻塞模式）
        // bulk_in.with_timeout(Duration::from_secs(0));
        // bulk_out.set_timeout(Duration::from_secs(0));

        // 返回值
        // Ok(Self {
        //     interface,
        //     bulk_in,
        //     bulk_out,
        // })
        
    }// end new
    
}

//     /// 从设备读取数据（非阻塞）
//     pub fn read(&mut self, buf: &mut [u8]) -> Result<usize, UsbError> {
//         self.bulk_in.read(buf).wait().map_err(UsbError::from)
//     }

//     /// 向设备写入数据（非阻塞）
//     pub fn write(&mut self, data: &[u8]) -> Result<usize, UsbError> {
//         self.bulk_out.write(data).wait().map_err(UsbError::from)
//     }
// }

// /// 配置串口参数（波特率、数据位、停止位等）
// fn configure_serial_port(interface: &Interface) -> Result<(), UsbError> {
//     // 设置线路编码（115200 8N1）
//     let line_coding = [
//         0x00, 0xC2, 0x01, 0x00, // 115200 小端格式（0x0001C200）
//         0x00,                   // 1位停止位
//         0x00,                   // 无校验
//         0x08,                   // 8位数据位
//     ];

//     // 发送SET_LINE_CODING控制请求
//     interface
//         .control_out(ControlOut {
//             control_type: ControlType::Class,
//             recipient: Recipient::Interface,
//             request: SET_LINE_CODING,
//             value: 0x0000,
//             index: 0x00,
//             data: &line_coding,
//         })
//         .wait()
//         .map_err(UsbError::from)?;

//     // 启用DTR（数据终端就绪）
//     interface
//         .control_out(ControlOut {
//             control_type: ControlType::Class,
//             recipient: Recipient::Interface,
//             request: SET_CONTROL_LINE_STATE,
//             value: 0x01,
//             index: 0x00,
//             data: &[],
//         })
//         .wait()
//         .map_err(UsbError::from)?;

//     Ok(())
// }

// fn main() {
//     env_logger::init();

//     // 初始化USB-CDC设备
//     let mut serial = SerialPort::new().expect("无法打开串口设备");

//     loop {
//         // 读取缓冲区（64字节）
//         let mut buf = [0u8; 64];
        
//         // 非阻塞读取
//         match serial.read(&mut buf) {
//             Ok(count) if count > 0 => {
//                 println!("收到 {} 字节: {:?}", count, &buf[..count]);
//             }
//             Err(UsbError::WouldBlock) => {} // 无数据不处理
//             Err(e) => eprintln!("读取错误: {:?}", e),
//         }

//         // 发送笑脸符号 ":)"
//         match serial.write(&[0x3a, 0x29]) {
//             Ok(count) => println!("发送 {} 字节", count),
//             Err(UsbError::WouldBlock) => {} // 缓冲区满不处理
//             Err(e) => eprintln!("写入错误: {:?}", e),
//         }

//         // 降低CPU占用（100ms间隔）
//         std::thread::sleep(Duration::from_millis(100));
//     }
// }

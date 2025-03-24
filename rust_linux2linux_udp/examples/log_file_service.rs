// 导入需要的标准库模块
use std::{
    fs::OpenOptions,
    io::Write,
    thread,
    time::{SystemTime, UNIX_EPOCH}
};

fn main() {
    // 循环写入时间到文件
    loop {
        // 获取当前UNIX时间戳（秒）
        let timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("获取时间失败")
            .as_secs();
        
        // 以覆写模式打开文件（不存在则创建，存在则清空）
        let mut file = OpenOptions::new()
            .create(true)
            .write(true)
            .truncate(true)
            .open("/tmp/log_file_service")
            .expect("无法打开文件");
        
        // 写入时间戳并换行
        writeln!(file, "{}", timestamp)
            .expect("写入文件失败");
        
        // 每秒执行一次 
        thread::sleep(std::time::Duration::from_secs(1));
    }
}

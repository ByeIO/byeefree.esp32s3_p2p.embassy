// esp32专用
#include <WiFi.h>
#include <esp_now.h>

// 屏幕相关
#include <SPI.h>
#include "./LCD_Driver.h"
#include "./GUI_Paint.h"

// 打印调试信息开关
#define DEBUG 0

// 配置对端MAC地址(节点B的MAC地址), B节点MAC:18:8b:0e:cc:94:30
uint8_t PEER_MAC[] = {0x18, 0x8b, 0x0e, 0xcc, 0x94, 0x30};

// 这个必须配置为全局变量, 生命周期错误会导致`espnow-peer-interface-is-invalid`
esp_now_peer_info_t peerInfo;

// esp-now连接成功
bool esp_now_connected = false;


// 发送回调函数
void OnDataSent(const uint8_t *mac_addr, esp_now_send_status_t status) {
  #if DEBUG
  Serial.print("\r\n发送状态:");
  Serial.println(status == ESP_NOW_SEND_SUCCESS ? "成功" : "失败");
  #endif
}

// 接收回调函数
void OnDataRecv(const uint8_t *mac, const uint8_t *incomingData, int len) {
  #if DEBUG
  Serial.printf("\r\n来自 %02X:%02X:%02X:%02X:%02X:%02X 的消息：",
                mac[0], mac[1], mac[2], mac[3], mac[4], mac[5]);
  #endif

  for (int i=0; i<len; i++) {
    Serial.print((char)incomingData[i]);
  }

  // 换行
  Serial.println();
}

void setup() {
  // 设置WiFi为STA模式
  Serial.begin(115200);
  WiFi.mode(WIFI_STA); 
  
  // 初始化屏幕
  Config_Init();
  LCD_Init();
  LCD_SetBacklight(100);
  Paint_SetRotate(90);
  LCD_Clear(WHITE);
  delay(1000);
  
  // 初始化ESP-NOW
  if (esp_now_init() != ESP_OK) {
    #if DEBUG
    Serial.println("ESP-NOW初始化失败");
    #endif
    // 重启
    ESP.restart();
  }
  
  // 配置对端设备
  memcpy(peerInfo.peer_addr, PEER_MAC, 6);
  peerInfo.channel = 0;  
  peerInfo.encrypt = false;
  
  // 注册回调函数
  esp_now_register_send_cb(OnDataSent);
  esp_now_register_recv_cb(OnDataRecv);
  
  if (esp_now_add_peer(&peerInfo) != ESP_OK){
    #if DEBUG
    Serial.println("添加对端设备失败");
    #endif
    // 重启
    ESP.restart();
  }

  // 连接成功
  esp_now_connected = true;
  
  #if DEBUG
  // 打印本机MAC地址
  Serial.print("当前节点MAC:");
  Serial.println(WiFi.macAddress());
  Serial.println("输入消息通过串口发送");
  #endif
}

void loop() {
  // 从串口读取输入
  if (Serial.available() > 0) {
    String input = Serial.readStringUntil('\n');
    
    // 发送ESP-NOW消息
    esp_err_t result = esp_now_send(PEER_MAC, 
                                 (uint8_t *)input.c_str(), 
                                 input.length());
    
    // 处理发送结果
    if (result == ESP_OK) {

      #if DEBUG
      Serial.print("已发送：");
      Serial.println(input);
      #endif

      LCD_Clear(GREEN);
      Paint_DrawString_EN(20, 50, "sent!", &Font20, BLACK, RED);

    } else {

      #if DEBUG
      Serial.println("发送失败");
      #endif
      LCD_Clear(RED);

    }
  }
}

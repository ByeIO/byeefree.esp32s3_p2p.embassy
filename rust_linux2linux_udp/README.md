# 虚拟网卡
将esp32点对点透传的数据封装为udp/tcp并通过linux TAP虚拟网卡模拟为网卡, 然后就可以使用:
```sh
ssh qsbye@[ip地址]
```
来连接开发板了.

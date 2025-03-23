import network
import espnow

# A WLAN interface must be active to send()/recv()
sta = network.WLAN(network.STA_IF)  # Or network.WLAN.IF_AP
sta.active(True)

e = espnow.ESPNow()
e.active(True)
# B: 18:8b:0e:cc:94:30
peer = b'\x18\x8b\x0e\xcc\x94\x30'   # MAC address of peer's wifi interface
e.add_peer(peer)      # Must add_peer() before send()

e.send(peer, "Starting...")
while(True):
    for i in range(100):
        e.send(peer, str(i)*20, True)
    e.send(peer, b'end')

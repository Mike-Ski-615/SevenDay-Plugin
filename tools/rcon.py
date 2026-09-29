import os
import socket, struct, sys

HOST = os.environ.get("RCON_HOST", "127.0.0.1")
PORT = int(os.environ.get("RCON_PORT", "25575"))
PW = os.environ.get("RCON_PASSWORD", "")
if not PW:
    print("请先设置环境变量 RCON_PASSWORD（不要写死在代码里）")
    sys.exit(1)

def pack(rid, ptype, body):
    data = struct.pack("<ii", rid, ptype) + body.encode("utf-8") + b"\x00\x00"
    return struct.pack("<i", len(data)) + data

def recv_packet(s):
    hdr = s.recv(4)
    if len(hdr) < 4:
        return None
    (length,) = struct.unpack("<i", hdr)
    data = b""
    while len(data) < length:
        chunk = s.recv(length - len(data))
        if not chunk:
            break
        data += chunk
    rid, ptype = struct.unpack("<ii", data[:8])
    return rid, ptype, data[8:-2].decode("utf-8", "replace")

s = socket.create_connection((HOST, PORT), timeout=8)
s.sendall(pack(1, 3, PW))
auth = recv_packet(s)
if auth is None or auth[0] == -1:
    print("LOGIN FAILED:", auth)
    sys.exit(1)
s.settimeout(0.8)

for c in sys.argv[1:]:
    s.sendall(pack(2, 2, c))
    parts = []
    while True:
        try:
            r = recv_packet(s)
        except socket.timeout:
            break
        if r is None:
            break
        if r[0] == -1:
            break
        if r[2].strip():
            parts.append(r[2].strip())
    print("> " + c)
    print("  " + ("\n  ".join(parts) if parts else "(无响应)"))
    print()
s.close()

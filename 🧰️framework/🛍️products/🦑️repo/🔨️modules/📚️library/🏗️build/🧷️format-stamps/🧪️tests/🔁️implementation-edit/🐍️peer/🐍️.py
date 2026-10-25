"""🐍️ A peer that edits a file while another process runs: waits for the ready file, appends bytes (or replaces the file atomically) and creates the edited file. usage: peer.py <ready> <edited> <target> <mode>"""
import os
import sys
import time

ready, edited, target, mode = sys.argv[1:5]
deadline = time.time() + 60
while not os.path.exists(ready) and time.time() < deadline:
    time.sleep(0.01)
with open(target, "rb") as handle:
    data = handle.read()
if mode == "append":
    with open(target, "ab") as handle:
        handle.write(b"\n// edited by a peer while the command ran\n")
else:
    temporary = target + ".peer"
    with open(temporary, "wb") as handle:
        handle.write(data + b"\n// replaced atomically by a peer\n")
    os.replace(temporary, target)
with open(edited, "w") as handle:
    handle.write("edited")

"""🧱️ Language-agnostic allocator child: touches <megabytes> in steps of <step> megabytes, holds them for <hold> milliseconds, prints done."""
import sys
import time

megabytes, step, hold = int(sys.argv[1]), int(sys.argv[2]), int(sys.argv[3])
blocks = []
for _ in range(0, megabytes, step):
    block = bytearray(step * 1048576)
    for index in range(0, len(block), 4096):
        block[index] = 1
    blocks.append(block)
    time.sleep(0.01)
time.sleep(hold / 1000)
print("done", flush=True)

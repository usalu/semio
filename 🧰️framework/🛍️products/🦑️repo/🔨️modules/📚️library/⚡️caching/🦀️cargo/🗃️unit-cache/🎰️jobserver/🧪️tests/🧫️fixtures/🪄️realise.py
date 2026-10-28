"""🎰️ usage: realise.py <probe> <root>; stdin: one fixture case. Makes the case real: every `fifo:` path of the environment moves under <root> and is created as an empty named pipe when the case lists it, and the descriptors of the census pair (SEMIO_UNIT_CACHE_JOBSERVER_FDS) are really open pipe ends under exactly those numbers. Then runs the probe with only that environment and prints what it printed."""
import fcntl
import json
import os
import subprocess
import sys

probe, root = sys.argv[1], sys.argv[2]
case = json.load(sys.stdin)
environment = {name: value.replace("fifo:/", "fifo:" + root + "/") for name, value in case["environment"].items()}
for path in case["fifos"]:
    target = root + path
    os.makedirs(os.path.dirname(target), exist_ok=True)
    os.mkfifo(target, 0o600)
pass_fds = []
census = environment.get("SEMIO_UNIT_CACHE_JOBSERVER_FDS", "")
if census:
    first, second = [int(part) for part in census.split(",")]
    read, write = os.pipe()
    high_read, high_write = fcntl.fcntl(read, fcntl.F_DUPFD, 100), fcntl.fcntl(write, fcntl.F_DUPFD, 100)
    os.close(read)
    os.close(write)
    os.dup2(high_read, first, inheritable=True)
    os.dup2(high_write, second, inheritable=True)
    os.close(high_read)
    os.close(high_write)
    pass_fds = [first, second]
environment.pop("SEMIO_UNIT_CACHE_JOBSERVER_FDS", None)
run = subprocess.run([probe], env=environment, pass_fds=pass_fds, capture_output=True, text=True)
sys.stdout.write(run.stdout)
sys.exit(run.returncode)

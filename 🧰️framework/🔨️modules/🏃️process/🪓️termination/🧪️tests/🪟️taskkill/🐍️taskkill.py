"""🪓️ A POSIX stand-in for `taskkill.exe` that follows its documented grammar and exit codes, used to run the Windows tree-kill
branches on any host. Switches are case-insensitive; `/PID n` names the process; `/T` ends the process and every descendant;
without `/F` a console process cannot be closed ("can only be terminated forcefully", exit 1); an unknown pid exits 128;
invalid syntax exits 1. `SEMIO_TASKKILL_LOG` receives one JSON line with the received arguments; `SEMIO_TASKKILL_FAIL=<code>`
makes the call fail with that exit code before touching anything (access denied, for instance).
See https://learn.microsoft.com/windows-server/administration/windows-commands/taskkill"""
import json, os, signal, subprocess, sys


def tree(root):
    table = {}
    for line in subprocess.run(["ps", "-axo", "pid=,ppid="], capture_output=True, text=True).stdout.splitlines():
        fields = line.split()
        if len(fields) == 2:
            table.setdefault(int(fields[1]), []).append(int(fields[0]))
    order, frontier = [], [root]
    while frontier:
        pid = frontier.pop()
        order.append(pid)
        frontier.extend(table.get(pid, []))
    return order


arguments = sys.argv[1:]
if os.environ.get("SEMIO_TASKKILL_LOG"):
    with open(os.environ["SEMIO_TASKKILL_LOG"], "a") as log:
        log.write(json.dumps(arguments) + "\n")
if os.environ.get("SEMIO_TASKKILL_FAIL"):
    print("ERROR: Access is denied.", file=sys.stderr)
    sys.exit(int(os.environ["SEMIO_TASKKILL_FAIL"]))
switches, pid, index = set(), None, 0
while index < len(arguments):
    name = arguments[index].lower()
    if name == "/pid" and index + 1 < len(arguments) and arguments[index + 1].isdigit():
        pid = int(arguments[index + 1])
        index += 1
    elif name in ("/t", "/f"):
        switches.add(name)
    else:
        print("ERROR: Invalid syntax.", file=sys.stderr)
        sys.exit(1)
    index += 1
if pid is None:
    print("ERROR: Invalid syntax.", file=sys.stderr)
    sys.exit(1)
try:
    os.kill(pid, 0)
except ProcessLookupError:
    print('ERROR: The process "%d" not found.' % pid, file=sys.stderr)
    sys.exit(128)
if "/f" not in switches:
    print("ERROR: The process with PID %d could not be terminated.\nReason: This process can only be terminated forcefully (with /F option)." % pid, file=sys.stderr)
    sys.exit(1)
for victim in (reversed(tree(pid)) if "/t" in switches else [pid]):
    try:
        os.kill(victim, signal.SIGKILL)
    except ProcessLookupError:
        pass
print("SUCCESS: The process with PID %d has been terminated." % pid)

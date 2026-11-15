"""Independent model of the responsible-process watch: replays 🧫️cases.json and prints the verdicts as JSON; with --system PID it asks the operating system itself (ctypes, no repo code) who is responsible for PID."""
import ctypes, json, sys

def verdicts(spec, case):
    pid, first, out = spec["pid"], None, []
    for answer in case["answers"]:
        if first is None:
            first = 0 if answer is None else answer
            out.append("own" if first in (0, pid) else "watch")
        elif first in (0, pid):
            out.append("own")
        else:
            out.append("watch" if answer == first else "lost")
    return out

if len(sys.argv) > 2 and sys.argv[1] == "--system":
    call = ctypes.CDLL(None).responsibility_get_pid_responsible_for_pid
    call.argtypes, call.restype = [ctypes.c_int], ctypes.c_int
    print(call(int(sys.argv[2])))
else:
    spec = json.load(open(sys.argv[1]))
    print(json.dumps({case["name"]: verdicts(spec, case) for case in spec["cases"]}))

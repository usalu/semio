"""🐍️ Independent oracle of the journal lock decision: a lock is held by its owner when the owner is a real process id and alive and either its heartbeat is fresh or the lock is younger than the start timeout; an owner without a real process id holds the lock only while the lock is younger than the start timeout. usage: python3 -I oracle.py < {vectors:[...], heartbeatStaleMilliseconds, startTimeoutMilliseconds} -> {held:[bool,...]}"""
import json, sys
data = json.load(sys.stdin)
stale, start = data["heartbeatStaleMilliseconds"], data["startTimeoutMilliseconds"]
def real(pid): return isinstance(pid, int) and not isinstance(pid, bool) and pid > 0
def held(v):
    if not real(v["pid"]): return v["ageMilliseconds"] < start
    return bool(v["alive"]) and (v["heartbeatAgeMilliseconds"] < stale or v["ageMilliseconds"] < start)
json.dump({"held": [held(v) for v in data["vectors"]]}, sys.stdout)

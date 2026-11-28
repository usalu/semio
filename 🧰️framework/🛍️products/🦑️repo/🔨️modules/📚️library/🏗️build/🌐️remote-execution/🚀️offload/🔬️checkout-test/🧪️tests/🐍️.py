import json, re, sys

def covers(need, have):
    behind = []
    for origin in sorted(need):
        want = need[origin]
        held = have.get(origin)
        if held is None or (held["segment"], held["bytes"]) < (want["segment"], want["bytes"]):
            behind.append(origin)
    return {"covered": not behind, "behind": behind}

def valid_path(path):
    if not path or len(path) > 1024 or path.startswith("/") or "\\" in path or re.match(r"^[A-Za-z]:", path):
        return False
    if any(ord(ch) < 0x20 or ord(ch) == 0x7F for ch in path):
        return False
    return all(part not in ("", ".", "..") for part in path.split("/"))

def gate(row):
    if row["mode"] == "never":
        return {"offload": False, "reason": "never", "asked": []}
    asked = []
    reason = "always"
    if row["mode"] == "auto":
        asked.append("host")
        if row["host"] != "wait":
            if row.get("load") is None:
                return {"offload": False, "reason": "local-slot", "asked": asked}
            asked.append("load")
            if row["load"] != "offload":
                return {"offload": False, "reason": "local-slot", "asked": asked}
            reason = "host-load"
        else:
            reason = "host-wait"
    asked.append("idle")
    idle = row["idle"]
    if idle is not None and idle > 0:
        return {"offload": True, "reason": reason, "asked": asked}
    return {"offload": False, "reason": "no-idle-machine", "asked": asked}

def load_verdict(row, policy):
    load = row["loadPerCore"]
    if load < policy["perCoreMeasured"]:
        return "keep"
    def fresh(timing):
        return timing is not None and timing["ageSeconds"] <= policy["maximumAgeSeconds"]
    local, remote = row["local"], row["remote"]
    if fresh(local) and fresh(remote) and remote["seconds"] >= local["seconds"] * policy["remoteWorthRatio"]:
        return "keep"
    return "offload"

request = json.load(sys.stdin)
print(json.dumps({
    "cover": [covers(row["need"], row["have"]) for row in request.get("cover", [])],
    "path": [valid_path(row["path"]) for row in request.get("path", [])],
    "gate": [gate(row) for row in request.get("gate", [])],
    "load": [load_verdict(row, request["policy"]) for row in request.get("load", [])],
}))

"""🐍️ Independent oracle of the fill rates: argv[1] is the policy schema, stdin a list of {nowMs, samples}; prints the rates of each."""
import json
import sys

policy = json.load(open(sys.argv[1], encoding="utf8"))["definitions"]["Policy"]["const"]
HOUR = 3600000


def rates(state):
    now = state["nowMs"]
    rows = sorted((s for s in state["samples"] if s["atMs"] <= now and now - s["atMs"] <= policy["windowMilliseconds"]), key=lambda s: s["atMs"])
    if not rows:
        return {"spanMs": 0, "freeBytesPerHour": 0, "profileBytesPerHour": {}}
    span = rows[-1]["atMs"] - rows[0]["atMs"]
    if span < policy["minimumSpanMilliseconds"]:
        return {"spanMs": span, "freeBytesPerHour": 0, "profileBytesPerHour": {}}
    free = round((rows[0]["freeBytes"] - rows[-1]["freeBytes"]) * HOUR / span)
    scans = [s for s in rows if "profiles" in s]
    profiles = {}
    if len(scans) >= 1 and scans[-1]["atMs"] - scans[0]["atMs"] >= policy["minimumSpanMilliseconds"]:
        scan_span = scans[-1]["atMs"] - scans[0]["atMs"]
        for name in sorted(scans[-1]["profiles"]):
            profiles[name] = round((scans[-1]["profiles"][name] - scans[0]["profiles"].get(name, 0)) * HOUR / scan_span)
    return {"spanMs": span, "freeBytesPerHour": free, "profileBytesPerHour": profiles}


def js_round(value):
    return value


print(json.dumps([rates(state) for state in json.load(sys.stdin)]))

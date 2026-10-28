#!/usr/bin/env python3
"""Independent model of the Nx daemon admission tables: limits (memory share and overrides), the admission decision and the idle rule. usage: 🐍️.py FIXTURE"""
import json, re, sys

fixture = json.load(open(sys.argv[1]))
POLICY = {"finite": 2, "watch": 6, "maximum": 16, "idle": 600000, "drain": 60000, "mb": 2048, "share": 4, "resident": 2048, "stale": 120000, "lean": {"bytes": 25769803776, "idle": 120000, "resident": 1280}}


def natural(text):
    return int(text) if text is not None and re.fullmatch(r"\d{1,9}", text) else None


def limits(environment, memory_gib):
    budget = (memory_gib * 1024) // POLICY["share"] // POLICY["mb"]
    parts = (environment.get("SEMIO_NX_DAEMONS", "") + ",").split(",")[:2]
    finite, watch = natural(parts[0]), natural(parts[1])
    idle = natural(environment.get("SEMIO_NX_DAEMON_IDLE_MS"))
    drain = natural(environment.get("SEMIO_NX_DAEMON_DRAIN_MS"))
    lean = memory_gib * fixture["GiB"] <= POLICY["lean"]["bytes"]
    resident = natural(environment.get("SEMIO_NX_DAEMON_RESIDENT_MB"))
    return {"residentMegabytes": resident if resident is not None else (POLICY["lean"]["resident"] if lean else POLICY["resident"]), "finite": min(POLICY["maximum"], finite if finite is not None else min(POLICY["finite"], budget)), "watch": min(POLICY["maximum"], watch if watch is not None else POLICY["watch"]), "idleMilliseconds": idle if idle is not None else (POLICY["lean"]["idle"] if lean else POLICY["idle"]), "drainMilliseconds": drain if drain is not None else POLICY["drain"]}


def decide(row):
    if row.get("explicit") is not None:
        return {"enabled": row["explicit"] != "false", "reason": "explicit"}
    if row["namespaceLive"]:
        return {"enabled": True, "reason": "reuse"}
    if row["limit"] <= 0:
        return {"enabled": False, "reason": "disabled"}
    return {"enabled": row["live"] < row["limit"], "reason": "admitted" if row["live"] < row["limit"] else "saturated"}


def idle(state):
    return state["kind"] == "finite" and state["openRequests"] == 0 and state["idleMilliseconds"] > 0 and state["now"] - state["lastActivity"] >= state["idleMilliseconds"]


def retire(state):
    return state["kind"] == "finite" and state["openRequests"] == 0 and state["limitBytes"] > 0 and state["residentBytes"] > state["limitBytes"]


def resident(section):
    rows = [r for r in section["records"] if r["file"] == "%d.json" % r["pid"] and r["kind"] in ("finite", "watch") and section["now"] - r["atMs"] <= POLICY["stale"] and r["alive"]]
    return {"pids": sorted(r["pid"] for r in rows), "totalBytes": sum(r["rssBytes"] for r in rows)}


def yields(state):
    return state["kind"] == "finite" and state["openRequests"] == 0 and state["requested"]


print(json.dumps({"retire": [retire(row["state"]) for row in fixture["retire"]], "resident": resident(fixture["resident"]), "yield": [yields(row["state"]) for row in fixture["yield"]], "limits": [limits(row["environment"], row["memoryGiB"]) for row in fixture["limits"]], "decisions": [decide(row["input"]) for row in fixture["decisions"]], "idle": [idle(row["state"]) for row in fixture["idle"]]}))

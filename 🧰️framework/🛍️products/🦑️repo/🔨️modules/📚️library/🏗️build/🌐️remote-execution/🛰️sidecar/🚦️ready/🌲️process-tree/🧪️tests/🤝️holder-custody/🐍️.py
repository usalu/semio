#!/usr/bin/env python3
"""Independent oracle of the holder custody rules (R183): the parse of the wait lines (own regular expressions, not the policy ones), the judgement of a holder, the background reader plan (a fixpoint over the table) and the foreground test. Prints one JSON document."""
import json,re,sys
schema=json.load(open(sys.argv[1],encoding="utf8"));fixture=json.load(open(sys.argv[2],encoding="utf8"))
P=schema["definitions"]["Policy"]["const"];TOL=P["toleranceMilliseconds"]
EN=re.compile(r"^Process (\d+) \((.*)\) holds the dependency store since (\S+?)(?: and has not renewed its registration for (\d+) s, so it may be blocked)?; waiting for it \((\d+) s of at most (\d+) s\)$")
DE=re.compile(r"^Der Prozess (\d+) \((.*)\) hält den Abhängigkeitsspeicher seit (\S+?)(?: und hat seine Anmeldung seit (\d+) s nicht erneuert, er ist womöglich blockiert)?; es wird darauf gewartet \((\d+) s von höchstens (\d+) s\)$")
def parse(lines):
    for line in reversed(lines):
        line=line.rstrip("\r")
        for rx in (EN,DE):
            m=rx.match(line)
            if m:
                g=m.groups();return {"pid":int(g[0]),"command":g[1],"since":g[2],"silentSeconds":None if g[3] is None else int(g[3]),"seconds":int(g[4]),"limitSeconds":int(g[5])}
    return None
def custody(case):
    w=case["wait"];table={r["pid"]:r for r in case["table"]};e=case["entry"]
    if w["seconds"]>=w["limitSeconds"]:return "bound"
    if not table:return "unknown"
    row=table.get(w["pid"])
    if row is None:return "gone"
    if e is not None and abs(row["startMs"]-e["startedAtMs"])>TOL:return "gone"
    if e is not None and e.get("updatedAtMs") is not None:silent=max(0,case["nowMs"]-e["updatedAtMs"])
    else:silent=(w["silentSeconds"] or 0)*1000
    return "silent" if silent>P["holderSilentMilliseconds"] else "progressing"
def reader(case):
    r=case["record"]
    if r is None or r["status"] is not None or r["endedMs"] is not None:return None
    table={x["pid"]:x for x in case["table"]};run=table.get(r["runner"])
    if run is None or abs(run["startMs"]-r["startedMs"])>TOL:return None
    members={run["pid"]};grew=True
    while grew:
        grew=False
        for x in case["table"]:
            par=table.get(x["ppid"])
            if x["pid"] not in members and x["pid"]!=x["ppid"] and par is not None and x["ppid"] in members and x["startMs"]>=par["startMs"]-TOL:members.add(x["pid"]);grew=True
    if case["holder"] not in members or case["self"] in members:return None
    return {"root":run["pid"],"plan":sorted(members),"reason":r["reason"]}
def foreground(case):
    s=case["status"]
    if not isinstance(s,dict):return False
    u=s.get("updatedMs")
    return s.get("ready") is False and s.get("state") in ("running","waiting","backoff") and isinstance(u,(int,float)) and case["nowMs"]-u<=P["foregroundFreshMilliseconds"]
print(json.dumps({"parse":[parse(c["lines"]) for c in fixture["parse"]],"custody":[custody(c) for c in fixture["custody"]],"reader":[reader(c) for c in fixture["reader"]],"foreground":[foreground(c) for c in fixture["foreground"]]},ensure_ascii=False))

#!/usr/bin/env python3
"""🐍️ Independent oracle of the shared worktree journal rules. It shares no code with `../🟦️.ts`: it reads only the language-agnostic fixture and recomputes the hybrid logical clock, the total order, event admission and the fold.
usage: 🐍️.py [fixture.json]    check every vector of the fixture (default: the committed one); exit 1 on any mismatch
Python 3.9 compatible, standard library only."""
import json,os,re,sys
HERE=os.path.dirname(os.path.abspath(__file__))
DEFAULT=os.path.join(HERE,"🧫️fixtures","🔣️.json")
HLC=re.compile(r"\A([0-9]{13})-([0-9]{6})-([a-z0-9][a-z0-9\-]{0,39})\Z")
ORIGIN=re.compile(r"\A[a-z0-9][a-z0-9\-]{0,39}\Z")
HEX=re.compile(r"\A[0-9a-f]{64}\Z")
FUTURE=60000
MAX_COUNTER=999999
MAX_MS=9999999999999
MAX_COUNT=9007199254740991

def parse(hlc):
    m=HLC.match(hlc) if isinstance(hlc,str) else None
    if not m: return None
    return {"ms":int(m.group(1)),"counter":int(m.group(2)),"originId":m.group(3)}
def key(hlc):
    p=parse(hlc);return (p["ms"],p["counter"],p["originId"])
def tick(last,wall):
    ms=max(wall,last["ms"]);counter=0 if ms>last["ms"] else last["counter"]+1
    if ms>MAX_MS or counter>MAX_COUNTER: return None
    return {"ms":ms,"counter":counter}
def merge(last,received,wall):
    p=parse(received)
    if p is None or p["ms"]>wall+FUTURE: return None
    if (p["ms"],p["counter"])>(last["ms"],last["counter"]): return {"ms":p["ms"],"counter":p["counter"]}
    return {"ms":last["ms"],"counter":last["counter"]}
def fmt(clock,origin): return "%013d-%06d-%s"%(clock["ms"],clock["counter"],origin)
def wins(candidate,current):
    return current is None or key(candidate)>key(current)

def count_ok(v): return isinstance(v,int) and not isinstance(v,bool) and 0<=v<=MAX_COUNT
def path_ok(p):
    if not isinstance(p,str) or p=="" or len(p)>1024 or "\\" in p or "\x00" in p: return False
    return all(part not in ("","." ,"..") for part in p.split("/"))
def base_ok(v): return v is None or (isinstance(v,str) and HEX.match(v) is not None)
def admitted(e):
    if not isinstance(e,dict) or e.get("version")!=1 or e.get("version") is True: return False
    if parse(e.get("hlc")) is None or not path_ok(e.get("path")) or "base" not in e or not base_ok(e["base"]): return False
    op=e.get("operation");keys=set(e.keys())
    common={"version","hlc","operation","path","base"}
    if op=="put":
        return keys==common|{"sha256","bytes","executable"} and isinstance(e["sha256"],str) and HEX.match(e["sha256"]) is not None and count_ok(e["bytes"]) and isinstance(e["executable"],bool)
    if op=="delete": return keys==common
    if op=="link": return keys==common|{"target"} and isinstance(e["target"],str) and 1<=len(e["target"])<=4096
    return False
def entry(e):
    if e["operation"]=="put": return {"hlc":e["hlc"],"operation":"put","sha256":e["sha256"],"bytes":e["bytes"],"executable":e["executable"]}
    if e["operation"]=="link": return {"hlc":e["hlc"],"operation":"link","target":e["target"]}
    return {"hlc":e["hlc"],"operation":"delete"}
def fold(initial,events):
    manifest=dict(initial);won={}
    for index,e in enumerate(events):
        current=manifest.get(e["path"])
        if wins(e["hlc"],None if current is None else current["hlc"]):
            manifest[e["path"]]=entry(e);won[e["path"]]=index
    order=sorted(won.values(),key=lambda i:(key(events[i]["hlc"]),events[i]["path"]))
    return manifest,order

def main(argv):
    fixture=json.load(open(argv[1] if len(argv)>1 else DEFAULT,encoding="utf8"))
    failures=[];checks=[0]
    def expect(name,actual,wanted):
        checks[0]+=1
        if actual!=wanted: failures.append("%s: expected %r, got %r"%(name,wanted,actual))
    for r in fixture["tick"]: expect("tick "+r["id"],tick(r["last"],r["wallMs"]),r["expect"])
    for r in fixture["merge"]: expect("merge "+r["id"],merge(r["last"],r["received"],r["wallMs"]),r["expect"])
    for r in fixture["format"]:
        expect("format "+r["id"],fmt(r["clock"],r["originId"]),r["hlc"])
        p=parse(r["hlc"]);expect("format-parse "+r["id"],None if p is None else {"ms":p["ms"],"counter":p["counter"]},r["clock"])
        expect("format-origin "+r["id"],None if p is None else p["originId"],r["originId"])
    for r in fixture["parse"]: expect("parse "+r["id"],parse(r["hlc"]),r["expect"])
    for r in fixture["wins"]:
        expect("wins "+r["id"],wins(r["candidate"],r["current"]),r["expect"])
        if r["current"] is not None:
            expect("wins-string "+r["id"],r["candidate"]>r["current"],r["expect"])
    for o in fixture["origins"]["valid"]: expect("origin "+o,ORIGIN.match(o) is not None,True)
    for o in fixture["origins"]["invalid"]: expect("origin "+repr(o),ORIGIN.match(o) is not None,False)
    for r in fixture["valid"]: expect("valid "+r["id"],admitted(r["value"]),True)
    for r in fixture["invalid"]: expect("invalid "+r["id"],admitted(r["value"]),False)
    for r in fixture["fold"]:
        events=r["events"]
        for index,e in enumerate(events): expect("fold %s event %d admitted"%(r["id"],index),admitted(e),True)
        manifest,order=fold(r["initial"],events)
        expect("fold "+r["id"]+" manifest",manifest,r["manifest"])
        expect("fold "+r["id"]+" winners",order,r["winners"])
        back,_=fold(r["initial"],list(reversed(events)))
        expect("fold "+r["id"]+" order independence",back,manifest)
    if failures:
        for line in failures: sys.stderr.write("FAIL "+line+"\n")
        sys.stderr.write("%d of %d checks failed\n"%(len(failures),checks[0]))
        return 1
    print("ok %d checks"%checks[0])
    return 0

if __name__=="__main__":
    sys.exit(main(sys.argv))

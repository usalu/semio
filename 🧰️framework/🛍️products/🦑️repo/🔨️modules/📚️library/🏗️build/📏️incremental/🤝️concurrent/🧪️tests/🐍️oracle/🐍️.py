#!/usr/bin/env python3
"""Independent Python oracle of the concurrent-session harness: derives lock waits from stderr lines, attributes them, judges pass criteria, renders edits, digests files and admits workload data from the same schema and fixture, using only the standard library. Reads one JSON request on stdin, writes one JSON answer on stdout."""
import hashlib,json,re,sys

def key(text): return text.encode("utf-16-be","surrogatepass")
def usorted(values): return sorted(set(values),key=key)

request=json.load(sys.stdin)
policy=json.load(open(request["schemaPath"],encoding="utf8"))["definitions"]["Policy"]["const"]

def classify(classes,resource):
    for row in classes:
        if any(re.search(pattern,resource) for pattern in row["patterns"]): return row["id"]
    return "other"

def activity(line):
    m=re.search(policy["scriptRow"],line)
    if not m: return None
    try: row=json.loads(m.group(1))
    except ValueError: return None
    a=row.get("activity") if isinstance(row,dict) else None
    if not isinstance(a,dict): return None
    resource,mode,elapsed=a.get("resource"),a.get("mode"),a.get("elapsedMs")
    if not isinstance(resource,str) or resource=="" or mode not in ("shared","exclusive") or isinstance(elapsed,bool) or not isinstance(elapsed,(int,float)) or elapsed<0: return None
    return resource,mode,elapsed

def waits(classes,end,lines):
    gap=policy["episodeGapMilliseconds"];found=[];lease={};timed={}
    def lease_done(e):
        found.append({"class":classify(classes,e["resource"]),"resource":e["resource"],"mode":e["mode"],"source":"lease","precise":False,"startMs":e["first_at"]-e["first_elapsed"],"endMs":e["last_at"],"waitMs":e["last_elapsed"],"holderPid":e["pid"],"position":e["position"]})
    def timed_done(e):
        found.append({"class":classify(classes,e["resource"]),"resource":e["resource"],"mode":"exclusive","source":e["source"],"precise":True,"startMs":e["start"],"endMs":e["end"],"waitMs":e["end"]-e["start"],"holderPid":e["pid"],"position":None})
    def close_timed(at):
        for k in list(timed):
            e=timed.pop(k);e["end"]=max(at,e["start"]);timed_done(e)
    for item in sorted(lines,key=lambda x:x["atMs"]):
        at,line=item["atMs"],item["line"]
        b=re.search(policy["bazelWait"],line);c=re.search(policy["cargoWait"],line)
        if b:
            k=("bazel",policy["bazelResource"])
            if k not in timed: timed[k]={"source":"bazel","resource":policy["bazelResource"],"start":at,"end":at,"pid":int(b.group(1))}
            continue
        if c:
            resource=policy["cargoResourcePrefix"]+c.group(1);k=("cargo",resource)
            for other in [o for o in timed if o[0]=="cargo" and o[1]!=resource]:
                e=timed.pop(other);e["end"]=max(at,e["start"]);timed_done(e)
            if k not in timed: timed[k]={"source":"cargo","resource":resource,"start":at,"end":at,"pid":None}
            continue
        if re.search(policy["bazelContinuation"],line) and any(k[0]=="bazel" for k in timed): continue
        close_timed(at)
        row=activity(line)
        if row is None: continue
        resource,mode,elapsed=row
        if resource.startswith(policy["queuePrefix"]):
            try: info=json.loads(resource[len(policy["queuePrefix"]):])
            except ValueError: continue
            if not isinstance(info,dict) or not isinstance(info.get("resource"),str): continue
            pid,pos=info.get("holderPid"),info.get("position")
            if isinstance(pid,bool) or not isinstance(pid,int) or isinstance(pos,bool) or not isinstance(pos,int): continue
            k=(info["resource"],mode);cur=lease.get(k)
            if cur is not None and at-cur["last_at"]<=gap: cur["pid"]=pid;cur["position"]=pos
            else:
                if cur is not None: lease_done(lease.pop(k))
                lease[k]={"resource":info["resource"],"mode":mode,"first_at":at,"first_elapsed":0,"last_at":at,"last_elapsed":0,"pid":pid,"position":pos}
            continue
        if any(resource.startswith(p) for p in policy["ignoredPrefixes"]): continue
        k=(resource,mode);cur=lease.get(k)
        if cur is not None and elapsed>=cur["last_elapsed"] and at-cur["last_at"]<=gap:
            cur["last_at"]=at;cur["last_elapsed"]=elapsed
        else:
            if cur is not None: lease_done(lease.pop(k))
            lease[k]={"resource":resource,"mode":mode,"first_at":at,"first_elapsed":elapsed,"last_at":at,"last_elapsed":elapsed,"pid":None,"position":None}
    close_timed(end)
    for k in list(lease): lease_done(lease.pop(k))
    return sorted(found,key=lambda w:(w["startMs"],key(w["resource"]),key(w["source"])))

def relation(wait,self_,others):
    holders=[o for o in others if o["id"]!=self_["id"] and any(s<=wait["endMs"] and e>=wait["startMs"] for s,e in o["intervals"])]
    if not holders: rel="external"
    elif all(o["group"]!=self_["group"] for o in holders): rel="unrelated"
    else: rel="related"
    return {"relation":rel,"holders":usorted(o["id"] for o in holders)}

def trace(text,origin,classes):
    rows=[]
    for line in text.split("\n"):
        if not line.strip(): continue
        try: row=json.loads(line)
        except ValueError: continue
        if not isinstance(row,dict) or row.get("version")!=1: continue
        r,m,s,a,p=row.get("resource"),row.get("mode"),row.get("startedMs"),row.get("acquiredMs"),row.get("pid")
        h=row.get("holderPid")
        ints=lambda v:isinstance(v,int) and not isinstance(v,bool)
        if not isinstance(r,str) or r=="" or m not in ("shared","exclusive") or not ints(s) or s<0 or not ints(a) or a<0 or not ints(p) or p<1: continue
        if "holderPid" in row and (not ints(h) or h<1): continue
        if set(row)-{"version","resource","mode","startedMs","acquiredMs","pid","holderPid"}: continue
        if a<s: continue
        rows.append({"resource":r,"mode":m,"startedMs":s,"acquiredMs":a,"pid":p,"holderPid":h if "holderPid" in row else None})
    merged=[]
    for row in sorted(rows,key=lambda x:(key(x["resource"]),key(x["mode"]),x["pid"],x["startedMs"])):
        last=merged[-1] if merged else None
        if last and last["resource"]==row["resource"] and last["mode"]==row["mode"] and last["pid"]==row["pid"] and row["startedMs"]<=last["acquiredMs"]:
            last["acquiredMs"]=max(last["acquiredMs"],row["acquiredMs"])
            if last["holderPid"] is None and row["holderPid"] is not None: last["holderPid"]=row["holderPid"]
        else: merged.append(row)
    found=[{"class":classify(classes,x["resource"]),"resource":x["resource"],"mode":x["mode"],"source":"trace","precise":True,"startMs":x["startedMs"]-origin,"endMs":x["acquiredMs"]-origin,"waitMs":x["acquiredMs"]-x["startedMs"],"holderPid":x["holderPid"],"position":None} for x in merged]
    return sorted(found,key=lambda w:(w["startMs"],key(w["resource"])))

def merge(derived,traced):
    kept=[w for w in derived if w["source"]!="lease" or not any(o["resource"]==w["resource"] and o["mode"]==w["mode"] and o["startMs"]<=w["endMs"] and o["endMs"]>=w["startMs"] for o in traced)]
    return sorted(kept+traced,key=lambda w:(w["startMs"],key(w["resource"]),key(w["source"])))

def usable(step): return step["observation"] is not None and step["observed"] in ("exact","window")

def evaluate(criteria,measurement):
    uncached=set(measurement["uncached"]);out=[]
    def verdict(rule,status,value,limit,detail): return {"rule":rule["id"],"kind":rule["kind"],"owners":rule["owners"],"status":status,"value":value,"limit":limit,"detail":detail}
    def chosen(ids,i): return "*" in ids or i in ids
    for rule in criteria["rules"]:
        kind=rule["kind"];sessions=measurement["sessions"]
        if kind=="no-failure":
            bad=[f'{s["id"]}#{st["index"]} exit {js(st["exit"])}' for s in sessions for st in s["steps"] if st["exit"]!=0]
            out.append(verdict(rule,"fail" if bad else "pass",len(bad),0,bad))
        elif kind=="serial-equal":
            if measurement["serial"] is None: out.append(verdict(rule,"skip",None,0,[]));continue
            bad=[]
            for s in sessions:
                other=next((r for r in measurement["serial"] if r["id"]==s["id"]),None)
                if other is None: bad.append(f'{s["id"]}: no serial session');continue
                for st in s["steps"]:
                    twin=next((r for r in other["steps"] if r["index"]==st["index"]),None)
                    if twin is None: bad.append(f'{s["id"]}#{st["index"]}: no serial step');continue
                    for k in rule["compare"]:
                        if st[k]!=twin[k]: bad.append(f'{s["id"]}#{st["index"]} {k}: {js(st[k])} != {js(twin[k])}')
            out.append(verdict(rule,"fail" if bad else "pass",len(bad),0,bad))
        elif kind=="max-wait":
            hits=[]
            for s in sessions:
                for st in s["steps"]:
                    for w in st["waits"]:
                        if not ("*" in rule["classes"] or w["class"] in rule["classes"]): continue
                        rel=w.get("relation")
                        ok=rule["relation"]=="any" or (rel=="related" if rule["relation"]=="related" else rel in ("unrelated","external"))
                        if ok: hits.append((s,st,w))
            bad=[];unproven=[]
            for s,st,w in hits:
                holders=w.get("holders") or []
                tail=f'({w.get("relation","unknown")}{": "+",".join(holders) if holders else ""})'
                if w["waitMs"]>rule["maximumMilliseconds"]: bad.append(f'{s["id"]}#{st["index"]} {w["class"]} {w["resource"]} {js(w["waitMs"])} ms {tail}')
                elif not w["precise"]: unproven.append(f'{s["id"]}#{st["index"]} {w["class"]} {w["resource"]} unproven, at least {js(w["waitMs"])} ms {tail}')
            most=0
            for s,st,w in hits: most=max(most,w["waitMs"])
            out.append(verdict(rule,"fail" if bad else ("skip" if unproven else "pass"),most,rule["maximumMilliseconds"],bad+unproven))
        else:
            runs=[(s,st) for s in sessions if chosen(rule["sessions"],s["id"]) for st in s["steps"] if st["kind"]=="run"]
            seen=[(s,st) for s,st in runs if usable(st)];missing=[f'unobserved {s["id"]}#{st["index"]}' for s,st in runs if not usable(st)]
            skip=(not seen) or bool(missing)
            if kind=="executes-once":
                owners={}
                for s,st in seen:
                    for task in st["observation"]["executed"]:
                        if rule["excludeUncached"] and task in uncached: continue
                        owners.setdefault(task,set()).add(s["id"])
                bad=sorted((f'{t}: {", ".join(usorted(w))}' for t,w in owners.items() if len(w)>1),key=key)
                out.append(verdict(rule,"fail" if bad else ("skip" if skip else "pass"),len(bad),0,bad+missing))
            else:
                counts=[(f'{s["id"]}#{st["index"]}',len([t for t in st["observation"]["executed"] if not (rule["excludeUncached"] and t in uncached)])) for s,st in seen]
                bad=[f"{n}: {c} executed" for n,c in counts if c>rule["maximum"]]
                out.append(verdict(rule,"fail" if bad else ("skip" if skip else "pass"),max([c for _,c in counts],default=0),rule["maximum"],bad+missing))
    return out

def js(value):
    if value is None: return "null"
    if isinstance(value,bool): return "true" if value else "false"
    if isinstance(value,float) and value==int(value): return str(int(value))
    return str(value)

def tally(runs):
    p=f=s=0;owners={}
    for run in runs:
        for rule in run["rules"]:
            name={"pass":"passed","fail":"failed","skip":"skipped"}[rule["status"]]
            if name=="passed": p+=1
            elif name=="failed": f+=1
            else: s+=1
            for owner in rule["owners"]:
                owners.setdefault(owner,{"passed":0,"failed":0,"skipped":0})[name]+=1
    return {"passed":p,"failed":f,"skipped":s,"owners":[{"owner":o,**owners[o]} for o in sorted(owners,key=key)]}

def number(nonce): return int(nonce[:6],36)%1000003
TEMPLATES={
 "ts":{"comment":lambda n:f"// [incremental] {n}\n","private-body":lambda n:f"void {number(n)};\n","public-api":lambda n:f"export const __incremental{n}={number(n)} as const;\n"},
 "rs":{"comment":lambda n:f"// [incremental] {n}\n","private-body":lambda n:f"const _: u32 = {number(n)};\n","public-api":lambda n:f"/// [incremental] public probe {n}\npub const __INCREMENTAL_{n.upper()}: u32 = {number(n)};\n"},
 "py":{"comment":lambda n:f"# [incremental] {n}\n","private-body":lambda n:f"_incremental_{n} = {number(n)}\n","public-api":lambda n:f"INCREMENTAL_{n.upper()} = {number(n)}\n"},
 "text":{"comment":lambda n:f"# [incremental] {n}\n"},
 "go":{"comment":lambda n:f"// [concurrent] {n}\n","private-body":lambda n:f"var _ = {number(n)}\n","public-api":lambda n:f"const Concurrent{n.upper()} = {number(n)}\n"},
}
def mutate(text,op,language,nonce):
    if not re.fullmatch(policy["nonce"],nonce): raise ValueError("nonce")
    if op=="whitespace": return text+"\n"
    render=TEMPLATES.get(language,{}).get(op)
    if render is None: raise ValueError("unsupported")
    return (text if text.endswith("\n") or text=="" else text+"\n")+render(nonce)

def digest(parts):
    h=hashlib.sha256()
    for part in sorted(parts,key=lambda p:key(p["path"])):
        data=part["text"].encode("utf8")
        h.update(f'{part["path"]}\0{len(data)}\0'.encode("utf8"));h.update(data);h.update(b"\0")
    return h.hexdigest()

def independence(rows):
    found=[]
    for a in rows:
        for b in rows:
            if a["id"]==b["id"] or a["group"]==b["group"]: continue
            for f in a["edits"]:
                if f in b["touches"]: found.append(f'{a["id"]} edits {f}, an input of the targets of {b["id"]}, but {a["group"]} and {b["group"]} are declared unrelated')
    return sorted(found,key=key)

def workload_issues(value):
    found=[]
    def unique(kind,ids):
        seen=set()
        for i in ids:
            if i in seen: found.append(f"duplicate {kind} {i}")
            seen.add(i)
    classes={c["id"] for c in value["classes"]};criteria={c["id"]:c for c in value["criteria"]}
    unique("class",[c["id"] for c in value["classes"]]);unique("criteria",[c["id"] for c in value["criteria"]]);unique("workload",[w["id"] for w in value["workloads"]])
    for c in value["classes"]:
        for p in c["patterns"]:
            try: re.compile(p)
            except re.error: found.append(f'{c["id"]}: invalid pattern {p}')
    for c in value["criteria"]:
        unique(f'rule of {c["id"]}',[r["id"] for r in c["rules"]])
        for r in c["rules"]:
            if r["kind"]=="max-wait":
                for n in r["classes"]:
                    if n!="*" and n not in classes: found.append(f'{c["id"]}/{r["id"]}: unknown lock class {n}')
    for w in value["workloads"]:
        sessions={s["id"] for s in w["sessions"]};owners={}
        unique(f'session of {w["id"]}',[s["id"] for s in w["sessions"]])
        rules=criteria.get(w["criteria"])
        if rules is None: found.append(f'{w["id"]}: unknown criteria {w["criteria"]}')
        else:
            for r in rules["rules"]:
                for n in r.get("sessions",[]):
                    if n!="*" and n not in sessions: found.append(f'{w["id"]}/{r["id"]}: unknown session {n}')
        for owner in [{"id":"setup","steps":w["setup"]}]+w["sessions"]:
            barriers=[s["id"] for s in owner["steps"] if s["kind"]=="barrier"]
            unique(f'barrier in {w["id"]}/{owner["id"]}',barriers)
            for s in owner["steps"]:
                if s["kind"]=="edit" and s["op"]!="whitespace" and TEMPLATES.get(s["language"],{}).get(s["op"]) is None:
                    found.append(f'{w["id"]}/{owner["id"]}: Concurrent mutation {s["op"]} is unsupported for {s["language"]}' if s["language"]=="go" else f'{w["id"]}/{owner["id"]}: Incremental mutation {s["op"]} is unsupported for {s["language"]}')
            for s in owner["steps"]:
                if s["kind"]!="edit": continue
                holder=owners.get(s["file"])
                if holder is not None and holder!=owner["id"]: found.append(f'{w["id"]}: {s["file"]} is edited by {holder} and {owner["id"]}')
                owners[s["file"]]=owner["id"]
    return found

def nx_fit(record,ids,window,concurrent):
    tasks=record.get("tasks") if isinstance(record,dict) else None
    if not isinstance(tasks,list): return {"observation":None,"observed":"ambiguous"}
    known=set(ids);rows=[t for t in tasks if isinstance(t,dict) and isinstance(t.get("taskId"),str) and isinstance(t.get("cacheStatus"),str) and t["taskId"] in known]
    if not rows: return {"observation":None,"observed":"ambiguous"}
    def stamp(v):
        if isinstance(v,bool): return None
        if isinstance(v,(int,float)): return v
        if isinstance(v,str):
            m=re.fullmatch(r"(\d{4})-(\d\d)-(\d\d)T(\d\d):(\d\d):(\d\d)(?:\.(\d+))?Z",v)
            if not m: return None
            import calendar
            y,mo,d,h,mi,s=(int(x) for x in m.groups()[:6]);ms=int((m.group(7) or "0").ljust(3,"0")[:3])
            return calendar.timegm((y,mo,d,h,mi,s))*1000+ms
        return None
    run=record.get("run") if isinstance(record.get("run"),dict) else {}
    start,end=stamp(run.get("startTime")),stamp(run.get("endTime"));skew=policy["nxSkewMilliseconds"]
    if start is not None and end is not None:
        if start>=window["startMs"]-skew and end<=window["endMs"]+skew: observed="window" if concurrent else "exact"
        else: observed="ambiguous"
    else: observed="ambiguous" if concurrent else "exact"
    if observed=="ambiguous": return {"observation":None,"observed":observed}
    return {"observation":{"executed":usorted(t["taskId"] for t in rows if t["cacheStatus"]=="cache-miss"),"restored":usorted(t["taskId"] for t in rows if t["cacheStatus"]!="cache-miss"),"cached":[]},"observed":observed}

op=request["op"]
if op=="waits": answer=[waits(c["classes"],c["endMs"],c["lines"]) for c in request["cases"]]
elif op=="relation": answer=[relation(c["wait"],c["self"],c["others"]) for c in request["cases"]]
elif op=="evaluate": answer=[evaluate(c["criteria"],c["measurement"]) for c in request["cases"]]
elif op=="mutate": answer=[mutate(c["text"],c["op"],c["language"],c["nonce"]) for c in request["cases"]]
elif op=="independence": answer=[independence(c["rows"]) for c in request["cases"]]
elif op=="workloads": answer=[workload_issues(c["set"]) for c in request["cases"]]
elif op=="nx": answer=[nx_fit(c["record"],c["ids"],{"startMs":c["windowStart"],"endMs":c["windowEnd"]},c["concurrent"]) for c in request["cases"]]
elif op=="digest": answer=[digest(c["parts"]) for c in request["cases"]]
elif op=="traces": answer=[trace(c["text"],c["origin"],c["classes"]) for c in request["cases"]]
elif op=="merges": answer=[merge(c["derived"],c["traced"]) for c in request["cases"]]
elif op=="tally": answer=[tally(c["runs"]) for c in request["cases"]]
else: raise SystemExit("unknown op")
json.dump(answer,sys.stdout,ensure_ascii=False)

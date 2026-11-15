#!/usr/bin/env python3
"""🐍️ Independent oracle of the shared worktree compaction. It shares no code with `../🟦️.ts`: it folds a store from its files.
usage: 🐍️.py fold <store root> [--no-snapshot]   fold the newest snapshot plus every journal event after its cursors (with --no-snapshot: every event from the first segment, which fails when a segment was retired) and print {"entries":N,"digest":sha256,"cursors":{...}} as JSON
       🐍️.py retire <store root> <now-ms> <safety-hours> <stale-hours> <snapshot-origin> <snapshot-hlc>   print, as JSON, the segments {origin:[numbers]} that the retirement rule allows for that snapshot
Python 3.9 compatible, standard library only; the digest is sha256 over the lines `path TAB json(entry, sorted keys, no spaces)` in code point order."""
import hashlib,json,os,re,sys
HLC=re.compile(r"\A([0-9]{13})-([0-9]{6})-([a-z0-9][a-z0-9\-]{0,39})\Z")
SEGMENT=re.compile(r"\A([0-9]{12})\.jsonl\Z")
SNAPSHOT=re.compile(r"\A([0-9]{13}-[0-9]{6}-[a-z0-9][a-z0-9\-]{0,39})\.json\Z")

def key(hlc):
    m=HLC.match(hlc)
    return (int(m.group(1)),int(m.group(2)),m.group(3))

def read_json(path):
    with open(path,encoding="utf8") as handle: return json.load(handle)

def snapshots(v1):
    found=[]
    base=os.path.join(v1,"snapshots")
    if not os.path.isdir(base): return found
    for origin in sorted(os.listdir(base)):
        folder=os.path.join(base,origin)
        if not os.path.isdir(folder): continue
        for name in os.listdir(folder):
            m=SNAPSHOT.match(name)
            if m: found.append((key(m.group(1)),origin,m.group(1),os.path.join(folder,name)))
    found.sort(key=lambda item:(item[0],),reverse=True)
    return found

def fold(root,use_snapshot):
    v1=os.path.join(root,"v1")
    manifest={}
    cursors={}
    if use_snapshot:
        for _,origin,hlc,path in snapshots(v1):
            try: document=read_json(path)
            except ValueError: continue
            if document.get("hlc")!=hlc: continue
            manifest={p:e for p,e in document["manifest"].items()}
            cursors=document["cursors"]
            break
    origins=os.path.join(v1,"origins")
    for origin in sorted(os.listdir(origins)):
        head_file=os.path.join(origins,origin,"head.json")
        if not os.path.isfile(head_file): continue
        head=read_json(head_file)
        start=cursors.get(origin,{"segment":0,"bytes":0})
        segments=os.path.join(origins,origin,"segments")
        for number in range(start["segment"],head["segment"]+1):
            path=os.path.join(segments,"%012d.jsonl"%number)
            with open(path,"rb") as handle:
                data=handle.read()
            low=start["bytes"] if number==start["segment"] else 0
            high=head["bytes"] if number==head["segment"] else len(data)
            for line in data[low:high].split(b"\n"):
                if not line: continue
                event=json.loads(line.decode("utf8"))
                current=manifest.get(event["path"])
                if current is not None and key(event["hlc"])<=key(current["hlc"]): continue
                if event["operation"]=="put": entry={"hlc":event["hlc"],"operation":"put","sha256":event["sha256"],"bytes":event["bytes"],"executable":event["executable"]}
                elif event["operation"]=="link": entry={"hlc":event["hlc"],"operation":"link","target":event["target"]}
                else: entry={"hlc":event["hlc"],"operation":"delete"}
                manifest[event["path"]]=entry
    digest=hashlib.sha256()
    for path in sorted(manifest):
        digest.update((path+"\t"+json.dumps(manifest[path],sort_keys=True,separators=(",",":"),ensure_ascii=False)+"\n").encode("utf8"))
    return {"entries":len(manifest),"digest":digest.hexdigest(),"cursors":cursors}

def retire(root,now,safety_hours,stale_hours,snapshot_origin,snapshot_hlc):
    v1=os.path.join(root,"v1")
    document=read_json(os.path.join(v1,"snapshots",snapshot_origin,snapshot_hlc+".json"))
    cursors=document["cursors"]
    origins=os.path.join(v1,"origins")
    holders={}
    for origin in sorted(os.listdir(origins)):
        retired=os.path.join(origins,origin,"retired.json")
        if os.path.isfile(retired) and read_json(retired).get("retired") is True: continue
        attest=os.path.join(origins,origin,"attest.json")
        if not os.path.isfile(attest): continue
        wall=os.stat(attest).st_mtime*1000
        sync=True
        attested={}
        try:
            value=read_json(attest)
            if type(value.get("wallMs")) is int and type(value.get("sync")) is bool and isinstance(value.get("cursors"),dict):
                wall=value["wallMs"];sync=value["sync"];attested=value["cursors"]
        except ValueError: pass
        if now-wall>stale_hours*3600000 or not sync: continue
        holders[origin]=attested
    result={}
    for origin in sorted(os.listdir(origins)):
        head_file=os.path.join(origins,origin,"head.json")
        if not os.path.isfile(head_file): continue
        head=read_json(head_file)
        limit=min(cursors.get(origin,{"segment":0})["segment"],head["segment"])
        for cursor in holders.values(): limit=min(limit,cursor.get(origin,{"segment":0})["segment"])
        folder=os.path.join(origins,origin,"segments")
        chosen=[]
        for number in sorted(int(SEGMENT.match(name).group(1)) for name in os.listdir(folder) if SEGMENT.match(name)):
            if number>=limit: break
            if now-os.stat(os.path.join(folder,"%012d.jsonl"%number)).st_mtime*1000<safety_hours*3600000: break
            chosen.append(number)
        result[origin]=chosen
    return result

if __name__=="__main__":
    command=sys.argv[1]
    if command=="fold": print(json.dumps(fold(sys.argv[2],"--no-snapshot" not in sys.argv[3:])))
    elif command=="retire": print(json.dumps(retire(sys.argv[2],float(sys.argv[3]),float(sys.argv[4]),float(sys.argv[5]),sys.argv[6],sys.argv[7])))
    else: sys.exit(2)

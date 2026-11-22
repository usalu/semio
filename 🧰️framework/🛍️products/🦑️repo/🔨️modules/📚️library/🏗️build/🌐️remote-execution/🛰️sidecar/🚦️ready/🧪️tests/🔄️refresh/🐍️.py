import hashlib,json,sys
ZERO="0"*64
def digests(platform,stages,hashes):
    out=[]
    for name,paths in stages:
        text=f"{name}\n{platform}\n{ZERO}\n"+"".join(f"{p}\0{hashes.get(p,'absent')}\n" for p in paths)
        out.append((name,hashlib.sha256(text.encode()).hexdigest()))
    return out
def refresh(stages,done,done_ms,now,maximum,marker,repo,ready_ms,failure):
    if not marker or not repo or ready_ms is None or failure is not None:return None
    stale=[n for n,d in stages if done.get(n)!=d]
    if not stale:return None
    for n in stale:
        if n not in done or now-done_ms.get(n,float('-inf'))>maximum:return None
    return stale
if sys.argv[1]=="digests":
    a=json.loads(sys.argv[2])
    print(json.dumps(digests(a["platform"],a["stages"],a["hashes"])))
else:
    a=json.loads(sys.argv[2])
    print(json.dumps([refresh([tuple(s) for s in r["stages"]],r["done"],r["doneMs"],r["now"],r["maximum"],r["marker"],r["repo"],r["readyMs"],r["failure"]) for r in a]))

import json,sys,re
rows=json.load(open(sys.argv[1],encoding="utf8"))["rows"]
out=[]
for r in rows:
    c=r["context"];parts=["status 0x%x"%(r["code"]&0xffffffff)]
    if r["name"]:parts.append(r["name"])
    parts.append(r["origin"])
    for k in ("root","name","role"):
        if k in c:parts.append(k+" "+re.sub(r"[\x00-\x1f\x7f]","?",c[k])[:120])
    if "access" in c:parts.append("access 0x%x"%(c["access"]&0xffffffff))
    out.append("Windows relative %s failed (%s)"%(r["operation"],"; ".join(parts)))
print(json.dumps(out,ensure_ascii=False))

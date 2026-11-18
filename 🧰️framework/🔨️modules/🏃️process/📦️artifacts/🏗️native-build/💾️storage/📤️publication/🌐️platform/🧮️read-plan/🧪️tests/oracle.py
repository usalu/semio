import json,sys
doc=json.load(open(sys.argv[1]))
settle=5_000_000_000
out=[]
for row in doc["decisions"]:
    rec=row.get("recordedAtNs")
    if row["mode"]=="all" or (row["mode"]=="root" and row["fileRoot"]==row.get("rootIndex")) or rec is None:
        out.append(True);continue
    newest=max(int(row["mtimeNs"]),int(row["ctimeNs"]))
    out.append(newest+settle>int(rec))
def allowance(case):
    ops=16
    for root in case["roots"]:ops+=8+len(root.split("/"))
    for file in case["files"]:
        chunks=max(1,-(-(-(-file["bytes"]//3))*4//case["chunkBytes"]))
        ops+=8+len(file["path"].split("/"))+6*chunks
    return ops*512
print(json.dumps(out if not (len(sys.argv)>2 and sys.argv[2]=="allowance") else [allowance(case) for case in doc["allowances"]]))

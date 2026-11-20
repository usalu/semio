import json,stat,sys
from pathlib import Path
doc=json.load(open(sys.argv[1]))
if len(sys.argv) == 4 and sys.argv[2] == "directories":
    root = Path(sys.argv[3])
    def refusal(path):
        entry = root
        for part in Path(path).parts:
            entry = entry / part
            try:
                info = entry.lstat()
            except FileNotFoundError:
                observed = "missing"
            else:
                if stat.S_ISDIR(info.st_mode) and info.st_dev == root.stat().st_dev:
                    continue
                observed = "link" if stat.S_ISLNK(info.st_mode) else "file" if stat.S_ISREG(info.st_mode) else "different device"
            return {"path": path, "component": entry.relative_to(root).as_posix(), "observed": observed}
    print(json.dumps([refusal(row["path"]) for row in doc["refusedDirectories"]]))
    sys.exit(0)
if len(sys.argv) == 4 and sys.argv[2] == "ordinary":
    entries = list(Path(sys.argv[3]).rglob("*"))
    print(json.dumps({"directories": sum(entry.is_dir() for entry in entries), "files": sum(entry.is_file() for entry in entries)}))
    sys.exit(0)
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

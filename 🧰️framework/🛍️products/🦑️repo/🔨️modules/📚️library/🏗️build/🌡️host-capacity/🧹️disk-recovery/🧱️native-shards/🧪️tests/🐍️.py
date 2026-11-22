"""Independent model of the native shard relief: a shard with a regenerable file map is short when its free bytes do not cover the map plus the margin or when it is at least 90 percent full; every file map of a short shard is removed, the fullest short shard is the named consumer."""
import json,sys
MARGIN=4194304
NEAR=90
def short(f):
    maps=[x["bytes"] for x in f["files"] if x["name"]=="nx_files.nxt"]
    if not maps:
        return False
    used=f["totalBytes"]-f["freeBytes"]
    return f["freeBytes"]<sum(maps)+MARGIN or (f["totalBytes"]>0 and used*100>=f["totalBytes"]*NEAR)
def model(facts):
    ordered=sorted(facts,key=lambda f:f["id"])
    removals=[{"id":f["id"],"name":x["name"],"bytes":x["bytes"]} for f in ordered if short(f) for x in f["files"] if x["name"]=="nx_files.nxt"]
    shorts=[f for f in facts if short(f)]
    named=None
    if shorts:
        top=sorted(shorts,key=lambda f:(-(f["totalBytes"]-f["freeBytes"]),f["id"]))[0]
        named=top["id"]
    return {"removals":removals,"consumer":named}
if __name__=="__main__":
    data=json.load(open(sys.argv[1],encoding="utf8"))
    print(json.dumps({c["id"]:model(c["facts"]) for c in data["cases"]}))

import json,sqlite3,sys

policy=json.load(open(sys.argv[1],encoding="utf8"))["definitions"]["Policy"]["const"]

def plan(state):
    database=sqlite3.connect(":memory:")
    database.execute("create table file(lane text,path text,bytes integer,mtime integer,cutoff real)")
    for lane in state["lanes"]:
        starts=[row["startedMs"] for row in lane["holders"]]
        cutoff=state["nowMs"]-policy["idleMilliseconds"] if not starts else float("-inf") if any(start is None for start in starts) else min(starts)-policy["graceMilliseconds"]
        database.executemany("insert into file values(?,?,?,?,?)",[(lane["id"],row["path"],row["bytes"],row["mtimeMs"],cutoff) for row in lane["files"]])
    return [{"lane":lane,"path":path,"bytes":size} for lane,path,size in database.execute("select lane,path,bytes from file where mtime<cutoff order by lane,path")]

print(json.dumps([plan(state) for state in json.load(sys.stdin)]))

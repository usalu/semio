import json,sqlite3,sys

policy=json.load(open(sys.argv[1],encoding="utf8"))["definitions"]["Policy"]["const"]

def plan(state):
    database=sqlite3.connect(":memory:")
    database.execute("create table session(path text primary key,bytes integer,used integer,held integer,newest integer)")
    database.executemany("insert into session values(?,?,?,?,?)",[(row["path"],row["bytes"],row["lastUsedMs"],int(row["held"]),int(row["newest"])) for row in state["sessions"]])
    now=state["nowMs"]
    ordered=database.execute("select path,bytes,used,held,newest from session order by used,path").fetchall()
    removed={}
    def eligible(row):
        return not row[3] and now-row[2]>=policy["minimumAgeMilliseconds"]
    for row in ordered:
        if eligible(row) and not row[4] and now-row[2]>policy["idleMilliseconds"]:
            removed[row[0]]="idle"
    total=database.execute("select coalesce(sum(bytes),0) from session where path not in (%s)"%",".join("?"*len(removed)) if removed else "select coalesce(sum(bytes),0) from session",list(removed)).fetchone()[0]
    budget=int(state["volumeBytes"]*policy["budgetShare"])
    for row in ordered:
        if total<=budget:break
        if eligible(row) and row[0] not in removed:
            removed[row[0]]="budget";total-=row[1]
    freed=sum(row[1] for row in ordered if row[0] in removed)
    for row in ordered:
        if freed>=state["needBytes"]:break
        if eligible(row) and row[0] not in removed:
            removed[row[0]]="pressure";freed+=row[1]
    return [{"path":row[0],"bytes":row[1],"reason":removed[row[0]]} for row in ordered if row[0] in removed]

print(json.dumps([plan(state) for state in json.load(sys.stdin)]))

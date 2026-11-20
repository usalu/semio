import json,struct,sys
request=json.load(sys.stdin)
STATUS="<IIQQQQQQQ"
ENTRY="@IIIQIIIiI520s0Q"
COUNTERS="@IIQQQQQQQQQ"
def status(row):
    return struct.pack(STATUS,struct.calcsize(STATUS),row["load"],row["total"],row["available"],row["commit"],row["commitAvailable"],0,0,0).hex()
def entry(row):
    name=(row["name"]+"\0").encode("utf-16le").ljust(520,b"\0")
    return struct.pack(ENTRY,struct.calcsize(ENTRY),0,row["pid"],0,0,row["threads"],row["parent"],8,0,name).hex()
def counters(row):
    return struct.pack(COUNTERS,struct.calcsize(COUNTERS),0,row["peak"],row["workingSet"],0,0,0,0,row["pagefile"],row["pagefile"],row["private"]).hex()
out={"sizes":{"status":struct.calcsize(STATUS),"entry":struct.calcsize(ENTRY),"counters":struct.calcsize(COUNTERS)},
 "status":[status(r) for r in request.get("status",[])],
 "entries":[entry(r) for r in request.get("entries",[])],
 "counters":[counters(r) for r in request.get("counters",[])]}
json.dump(out,sys.stdout)

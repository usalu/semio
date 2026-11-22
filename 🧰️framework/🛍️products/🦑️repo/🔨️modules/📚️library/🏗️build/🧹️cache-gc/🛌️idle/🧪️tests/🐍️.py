"""🐍️ Independent oracle for the plan of compaction on idle."""
import json,sys
rows=json.load(sys.stdin)
out=[]
for r in rows["rows"]:
    if not r["due"]:out.append("not-due")
    elif r.get("lastAt") is not None and rows["now"]-r["lastAt"]<rows["minimum"]:out.append("too-soon")
    else:out.append("try")
print(json.dumps(out))

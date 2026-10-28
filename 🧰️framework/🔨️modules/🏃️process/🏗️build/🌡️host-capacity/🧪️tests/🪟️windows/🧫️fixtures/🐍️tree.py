"""🪟️ Fixture reading a process table and a pid (Windows host capacity)."""
import json,sys
table,pid=sys.argv[1],int(sys.argv[2])
rows={}
for line in table.splitlines():
    parts=line.split()
    if len(parts)==3 and all(p.isdigit() for p in parts):
        rows[int(parts[0])]=(int(parts[1]),int(parts[2]))
if pid not in rows:
    print(json.dumps(None)); sys.exit(0)
seen,stack,total=set(),[pid],0
while stack:
    cur=stack.pop()
    if cur in seen: continue
    seen.add(cur); total+=rows[cur][1]
    stack.extend(p for p,(parent,_) in rows.items() if parent==cur)
print(json.dumps(total//1024))

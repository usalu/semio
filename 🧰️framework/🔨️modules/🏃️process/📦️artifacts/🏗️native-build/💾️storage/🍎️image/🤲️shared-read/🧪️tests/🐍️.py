import json,sys
def simulate(ops,maximum,idle):
    acquires=[];since=None;last_end=None
    for start,duration in ops:
        if since is not None and last_end is not None and start-last_end>=idle:since=None
        if since is not None and start-since>=maximum:since=None
        if since is None:since=start;acquires.append(start)
        last_end=start+duration
    return acquires
if sys.argv[1]=="acquires":
    rows=json.load(open(sys.argv[2]))
    print(json.dumps([simulate(row["ops"],row["maximum"],row["idle"]) for row in rows]))

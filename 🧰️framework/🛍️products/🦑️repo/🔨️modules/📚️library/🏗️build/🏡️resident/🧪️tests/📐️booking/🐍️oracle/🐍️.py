"""Independent oracle of the slice booking: what a load asks the gate for on this machine."""
import json,sqlite3,sys
def book(c):
    asked=c["declared"] if c.get("declared") is not None else max(min(c["maximum"] or float("inf"),c["learned"]),c["unlearned"])
    cap=c["host"]*c["capBasisPoints"]//10000 if c["host"]>0 else float("inf")
    wanted=max(0,min(asked,max(cap,c["minimum"])))
    return [wanted,min(c["minimum"],wanted)]
if sys.argv[1:] == ["lean"]:
    print(json.dumps([min(c["booked"],max(c["floor"],-(-c["booked"]*c["lean"]//10000))) for c in json.load(sys.stdin)]))
elif sys.argv[1:] == ["worker-budget"]:
    rows=json.load(sys.stdin)
    db=sqlite3.connect(":memory:")
    query="""select case when ?1 is null then ?2
      else min(coalesce(?2,?1),max(?3,cast(?1*?4 as integer))) end"""
    print(json.dumps([db.execute(query,(None if row.get("capacityOff") else row["grant"],row["configured"],row["minimum"],row["share"])).fetchone()[0] for row in rows]))
else:
    print(json.dumps([book(c) for c in json.load(sys.stdin)]))

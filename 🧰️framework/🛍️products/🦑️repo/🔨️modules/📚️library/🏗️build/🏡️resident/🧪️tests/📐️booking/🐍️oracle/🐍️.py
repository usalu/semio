"""Independent oracle of the slice booking: what a load asks the gate for on this machine."""
import json,sys
def book(c):
    asked=c["declared"] if c.get("declared") is not None else max(min(c["maximum"] or float("inf"),c["learned"]),c["unlearned"])
    cap=c["host"]*c["capBasisPoints"]//10000 if c["host"]>0 else float("inf")
    wanted=max(0,min(asked,max(cap,c["minimum"])))
    return [wanted,min(c["minimum"],wanted)]
print(json.dumps([book(c) for c in json.load(sys.stdin)]))

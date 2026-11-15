import json,sys
# Independent oracle of the sidecar rules: the restart back-off and which children a machine wants. Standard library only.
cases=json.load(sys.stdin)
out=[]
for case in cases:
    if case["kind"]=="backoff":
        r=case["restart"];n=case["failures"]
        d=r["initialMilliseconds"]
        for _ in range(max(0,n-1)):
            d=min(d*2,r["maximumMilliseconds"])
        out.append(min(d,r["maximumMilliseconds"]))
    else:
        s=case["settings"];names=None
        o=case.get("override")
        if o not in (None,""):
            names=set(x.strip() for x in o.split(","))
        want=[]
        if s["enabled"] and o!="off":
            for c in s["children"]:
                role="server" if case["server"] else "member"
                if c["enabled"] and role in c["roles"] and (c["requires"]=="none" or case["peers"]>0) and (names is None or c["name"] in names):
                    want.append(c["name"])
        out.append(want)
json.dump(out,sys.stdout)

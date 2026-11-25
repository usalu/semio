import json,math,sys
data=json.load(sys.stdin)
p=data["policy"]
def weigh(before,observed,weight):
    return math.floor(before+weight*(observed-before)+0.5)
for row in data["cases"]:
    prov=row["provenance"]
    observed=row["ranMilliseconds"] if prov is None else prov["wallMilliseconds"]
    contended=prov is not None and observed>0 and (prov["userMilliseconds"]+prov["systemMilliseconds"])/observed<p["minimumUtilisation"]
    before=row["before"]
    if before is None:
        got=None if contended and not p["learnContendedFirst"] else observed
    elif not contended:
        got=weigh(before,observed,p["calmWeight"])
    else:
        got=weigh(before,observed,p["contendedRaiseWeight"] if observed>=before else p["contendedLowerWeight"])
    if got!=row["expected"]:
        print("mismatch",row["name"],got);sys.exit(1)
print("ok")

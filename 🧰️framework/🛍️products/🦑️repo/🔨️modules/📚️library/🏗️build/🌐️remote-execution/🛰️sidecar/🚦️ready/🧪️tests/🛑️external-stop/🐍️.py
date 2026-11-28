import json,re,sys
def stopped(r,signals,exits,patterns):
    if r.get("signal") in signals or r.get("exit") in exits:return True
    plain=[re.sub(r"\x1b\[[0-9;]*m","",l) for l in r.get("failure",[])+r["tail"]]
    return any(re.search(p,l) for p in patterns for l in plain)
a=json.loads(sys.argv[2])
print(json.dumps([stopped(r["result"],r["signals"],r["exits"],r["patterns"]) for r in a]))

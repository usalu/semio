import json,sys
corpus=json.load(open(sys.argv[1],encoding="utf8"))["distributions"]
schema=json.load(open(sys.argv[2],encoding="utf8"))["definitions"]["Protocol"]["const"]
def resolve(row):
    if row["platform"]=="darwin": names=[schema["darwin"]]
    elif row["platform"]!="linux": return "unsupported-platform"
    elif row["arch"] not in schema["linux"]: return "unsupported-architecture"
    else: names=schema["linux"][row["arch"]]
    for n in names:
        if n in row["loadable"]: return n
    return "missing"
print(json.dumps({r["id"]:resolve(r) for r in corpus}))

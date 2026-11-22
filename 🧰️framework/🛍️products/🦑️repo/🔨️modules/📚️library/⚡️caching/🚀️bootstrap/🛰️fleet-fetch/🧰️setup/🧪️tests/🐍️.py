import json,hashlib,sys
fixture=json.load(open(sys.argv[1],encoding="utf8"));schema=json.load(open(sys.argv[2],encoding="utf8"))
classes=json.load(open(sys.argv[3],encoding="utf8"))["definitions"]["Policy"]["const"]["classes"]
import os
index_version=json.load(open(os.path.join(os.path.dirname(os.path.abspath(sys.argv[2])),"..","..","🧬️schema","🔣️.json"),encoding="utf8"))["definitions"]["InstallIndex"]["properties"]["version"]["const"]
def klass(kind,host):
    if kind=="neutral":return ("any","any","any")
    exact=host["platform"]+"-"+host["arch"]
    if kind=="posix":
        for name,members in classes.items():
            if name.startswith("posix-") and exact in members:return (name,"posix",name.split("-",1)[1])
        return (exact,host["platform"],host["arch"]) if host["platform"] in("darwin","linux","win32") else None
    if host["platform"] not in("darwin","linux","win32"):return None
    arch=host["arch"]+("-"+host["libc"] if host.get("libc") else "")
    return (host["platform"]+"-"+arch,host["platform"],arch)
def sha(text):return hashlib.sha256(text.encode("utf8")).hexdigest()
def dumps(v):return json.dumps(v,separators=(",",":"),ensure_ascii=False)
out={"classes":[],"keys":[]}
for c in fixture["classes"]:
    k=klass(c["kind"],c["host"]);out["classes"].append(None if k is None else k[0])
for row in fixture["keys"]:
    k=klass(row["kind"],row["host"])
    projection=sha(dumps([row["task"],sorted([[a,b] for a,b in row["inputs"].items()])]))
    recipe=sha(dumps([1,row["task"],row["kind"],sorted(row["roots"])]))
    runtime=dumps([index_version,projection,row["task"][:64],k[1],k[2],recipe])
    out["keys"].append("inst-"+sha(runtime))
print(json.dumps(out))

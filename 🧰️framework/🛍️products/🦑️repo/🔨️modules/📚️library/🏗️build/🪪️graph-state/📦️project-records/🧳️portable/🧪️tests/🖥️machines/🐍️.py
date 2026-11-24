#!/usr/bin/env python3
"""Independent oracle of the machine-independence law of owner records: `fixture` writes the raw records two synthetic machines record for the same owner (other checkout roots, other cache roots, other sealed tool snapshot locations, other dependency generations, other analysis provider hashes, other file stamps), `neutral` derives the neutral canonical form of one raw record from the schema alone."""
import copy,hashlib,json,re,sys

HEX="0123456789abcdef"
def digest(text):return hashlib.sha256(text.encode("utf8")).hexdigest()
def canonical(value):
    if isinstance(value,list):return [canonical(item) for item in value]
    if isinstance(value,dict):return {key:canonical(value[key]) for key in sorted(value)}
    return value
def dump(value):return json.dumps(value,ensure_ascii=False,separators=(",",":"))

MACHINES=[
    {"name":"a","root":"/srv/ci-a/semio","cache":"/srv/ci-a/semio/.cache/b","snapshot":"/srv/ci-a/semio/.cache/b/t/sealed/"+"0f3a"*4,"generation":"a3f9"*8,"provider":"9d2b"*16,"stamp":[1700000000111.5,1700000000222.5,1001]},
    {"name":"b","root":"/home/dev-b/work/semio","cache":"/var/lib/semio-b/.cache","snapshot":"/opt/semio-tools/sealed/"+"b7d2"*4,"generation":"7c1e"*8,"provider":"e41c"*16,"stamp":[1800000000333.25,1800000000444.25,2002]},
]
SHA={"project":"1b"*32,"analysis":"2c"*32,"nested":"3d"*32,"result":"4e"*32}
REVISION="5f"*32

def recipe(root):return {"fn":"closure-of","entries":["tools/a.ts"],"workspaceRoot":root}
def key(value):return digest(dump(canonical(value)))

def stamped(machine,size,sha,offset=0):
    s=machine["stamp"];return {"size":size,"mtimeMs":s[0]+offset,"ctimeMs":s[1]+offset,"ino":s[2]+offset,"sha256":sha}

def raw(machine):
    root=machine["root"];parts=[p for p in root.split("/") if p];above=["/"]+["/"+"/".join(parts[:at]) for at in range(1,len(parts))]
    rows=[{"kind":"lstat","path":path,"fact":"directory"} for path in above]
    rows.append({"kind":"file","path":root+"/libs/x/project.json","fact":stamped(machine,12,SHA["project"])})
    rows.append({"kind":"exists","path":root+"/libs/x/Cargo.toml","fact":True})
    provider="build:command-imports:v1:"+machine["provider"]
    rows.append({"kind":"analysis","path":root+"/tools/a.ts","fact":{"provider":provider,"revision":REVISION,"source":stamped(machine,30,SHA["analysis"],1),"resultSha256":SHA["result"],"nestedObservations":[{"kind":"file","path":root+"/tools/b.ts","fact":stamped(machine,7,SHA["nested"],2)}]}})
    k=key(recipe(root))
    rows.append({"kind":"closure","path":dump([root,k,["generated"]]),"fact":"[]"})
    image=dump({"name":"x","root":"libs/x","closureRecipes":{k:recipe(root)},"command":"bun "+root+"/tools/a.ts"})
    local=[{"kind":"file","path":machine["snapshot"]+"/mod.ts","fact":stamped(machine,5,"6a"*32)},{"kind":"exists","path":machine["cache"]+"/f/x","fact":True}]
    return {"digest":"d"*64,"authority":[],"observations":rows,"image":image,"machineLocal":local}

def neutral(record,machine,schema):
    P=schema["definitions"]["Policy"]["const"];TOKEN=P["rootToken"];OWNER=P["ownerToken"];root=machine["root"]
    parts=[p for p in root.split("/") if p];above={"/"}|{"/"+"/".join(parts[:at]) for at in range(1,len(parts))}
    def path(text):return TOKEN+text[len(root):] if text==root or text.startswith(root+"/") else text
    def words(text):return re.sub("(?<![\\w.-])"+re.escape(root)+"(?![\\w.-])",TOKEN,text)
    def zero(fact):
        fact=dict(fact)
        for name in P["stampFields"]:
            if name in fact and isinstance(fact[name],(int,float)):fact[name]=P["stampValue"]
        return fact
    def row(item):
        kind=item["kind"];out=dict(item)
        if kind=="closure":
            p=json.loads(item["path"]);p[0]=path(p[0]);out["path"]=dump(p)
        elif kind=="analysis":
            fact=zero(item["fact"]);fact["source"]=zero(fact["source"])
            match=re.match(P["ownerPattern"],fact["provider"]);fact["provider"]=match.group(1)+OWNER if match else fact["provider"]
            fact["nestedObservations"]=[row(n) for n in fact["nestedObservations"]];out["path"]=path(item["path"]);out["fact"]=fact
        else:
            out["path"]=path(item["path"])
            if kind=="file":out["fact"]=zero(item["fact"])
        return out
    kept=[item for item in record["observations"] if not (item["kind"] in P["locationKinds"] and item["fact"]=="directory" and item["path"] in above)]
    rows=[row(item) for item in kept]
    body=json.loads(record["image"]);table=body.pop("closureRecipes");moved={}
    for old,value in table.items():
        text=words(dump(value));new=json.loads(text);moved[key(new)]=new;
        for item in rows:
            if item["kind"]=="closure":
                p=json.loads(item["path"])
                if p[1]==old:p[1]=key(new);item["path"]=dump(p)
    body["closureRecipes"]=moved;body["command"]=words(body["command"])
    return {"digest":record["digest"],"authority":[],"observations":rows,"image":dump(canonical(body))}

if __name__=="__main__":
    if sys.argv[1]=="fixture":
        print(json.dumps({"machines":[{**m,"record":raw(m)} for m in MACHINES]},ensure_ascii=False,indent=1))
    else:
        schema=json.load(open(sys.argv[2],encoding="utf8"));data=json.load(open(sys.argv[3],encoding="utf8"))
        print(json.dumps([neutral(m["record"],m,schema) for m in data["machines"]],ensure_ascii=False))

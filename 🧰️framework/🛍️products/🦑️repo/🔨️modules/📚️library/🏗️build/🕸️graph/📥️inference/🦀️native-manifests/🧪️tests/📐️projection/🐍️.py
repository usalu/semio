import json,sys
def table(t):return t if isinstance(t,dict) else {}
def deps(t,kinds):
    out={}
    for kind in kinds:
        red={}
        for alias,d in table(t.get(kind)).items():
            if not isinstance(d,dict):continue
            if d.get("workspace") is True:red[alias]={"workspace":True}
            elif d.get("path"):
                e={"path":d["path"]}
                if "package" in d:e["package"]=d["package"]
                red[alias]=e
        if red:out[kind]=red
    return out
def graph(m,tests):
    kinds=["dependencies","build-dependencies"]+(["dev-dependencies"] if tests else [])
    out=dict(deps(m,kinds))
    p=m.get("package")
    if isinstance(p,dict) and "workspace" in p:out["package"]={"workspace":p["workspace"]}
    tg={}
    for name,owner in table(m.get("target")).items():
        r=deps(owner,kinds) if isinstance(owner,dict) else {}
        if r:tg[name]=r
    if tg:out["target"]=tg
    w=m.get("workspace")
    if w:
        out["workspace"]={"dependencies":{a:({"path":d["path"],**({"package":d["package"]} if "package" in d else {})}) for a,d in table(w.get("dependencies") if isinstance(w,dict) else None).items() if isinstance(d,dict) and d.get("path")}} if isinstance(w,dict) else {}
    return out
def paths(l):
    if not isinstance(l,list):return None
    return [({"path":i["path"]} if isinstance(i,dict) and "path" in i else {}) for i in l]
def sources(m,tests):
    out={}
    p=m.get("package")
    if isinstance(p,dict):
        pk={}
        if "build" in p:pk["build"]=p["build"]
        a=(((p.get("metadata") or {}).get("semio") or {}) if isinstance(p.get("metadata"),dict) else {})
        if isinstance(a,dict) and "build-script" in a:pk["metadata"]={"semio":{"build-script":a["build-script"]}}
        if pk:out["package"]=pk
    l=m.get("lib")
    if isinstance(l,dict) and "path" in l:out["lib"]={"path":l["path"]}
    for k in (["bin","test","bench","example"] if tests else ["bin"]):
        v=paths(m.get(k))
        if v is not None:out[k]=v
    b=m.get("build-dependencies")
    if isinstance(b,dict) and b:out["build-dependencies"]=b
    tg={}
    for name,owner in table(m.get("target")).items():
        b=owner.get("build-dependencies") if isinstance(owner,dict) else None
        if isinstance(b,dict) and b:tg[name]={"build-dependencies":b}
    if tg:out["target"]=tg
    return out
def project(m,role):
    return {"graph":lambda:graph(m,False),"graph-tests":lambda:graph(m,True),"sources":lambda:sources(m,False),"sources-tests":lambda:sources(m,True)}[role]()
req=json.load(sys.stdin)
print(json.dumps([{r:project(m,r) for r in ["graph","graph-tests","sources","sources-tests"]} for m in req["manifests"]],sort_keys=True))

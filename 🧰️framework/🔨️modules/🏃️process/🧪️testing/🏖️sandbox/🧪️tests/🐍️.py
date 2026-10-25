#!/usr/bin/env python3
"""Independent Python replay of the test sandbox law: scanner rules, path policy, owner rules and guard decisions. usage: 🐍️.py PROTOCOL CORPUS (exit 1 and a mismatch list on any difference). The TypeScript scanner and this oracle share only the declared protocol and corpus."""
import json,posixpath,re,sys

def compile_law(p):
    rules=[]
    for r in p["rules"]:
        fl=re.I if r.get("flags")=="i" else 0
        c=dict(r)
        c["_p"]=re.compile(r["pattern"],fl) if "pattern" in r else None
        c["_also"]=[re.compile(x,fl) for x in r.get("also",[])]
        c["_ex"]=[re.compile(x,fl) for x in r.get("exempt",[])]
        c["_all"]=[re.compile(x,fl) for x in r.get("all",[])]
        c["_none"]=[re.compile(x,fl) for x in r.get("none",[])]
        rules.append(c)
    sc=p["scan"]
    ext={e:l for l,es in sc["extensions"].items() for e in es}
    return {"p":p,"rules":rules,"comment":{k:re.compile(v) for k,v in sc["commentLine"].items()},"suite":{k:re.compile(v) for k,v in sc["suite"].items()},"ext":ext}

def language(law,path):
    base=path.split("/")[-1]
    return law["ext"].get(base.rsplit(".",1)[-1]) if "." in base else None

def is_test_path(law,path,declared=()):
    parts=path.split("/"); base=parts[-1]; sc=law["p"]["scan"]
    if any(x in sc["skipSegments"] for x in parts): return False
    return sc["testSegment"] in parts[:-1] or path in declared or base.endswith("_test.go") or re.search(r"\.(test|spec)\.[a-z]+$",base) is not None

def scan_source(law,file,lang,text):
    if not law["suite"][lang].search(text): return []
    out=[]; lines=text.split("\n"); cm=law["comment"][lang]
    code="\n".join("" if cm.match(x) else x for x in lines)
    for r in law["rules"]:
        if lang not in r["languages"]: continue
        if r["scope"]=="line":
            for i,line in enumerate(lines):
                if cm.match(line): continue
                if r["_p"].search(line) and all(a.search(line) for a in r["_also"]) and not any(e.search(line) for e in r["_ex"]):
                    out.append((r["id"],i+1))
        elif r["scope"]=="file":
            if all(a.search(code) for a in r["_all"]) and not any(n.search(code) for n in r["_none"]):
                first=next((i+1 for i,line in enumerate(lines) if r["_all"][0].search(line)),1)
                out.append((r["id"],first))
    return out

def scan_owner(law,sources):
    out=[]
    for r in law["rules"]:
        if r["scope"]!="owner": continue
        text="\n".join(s["text"] for s in sources if s["language"] in r["languages"])
        if text!="" and all(a.search(text) for a in r["_all"]) and not any(n.search(text) for n in r["_none"]): out.append((r["id"],1))
    return out

def inside(root,path,platform):
    fold=(lambda v:v.lower()) if platform in ("win32","darwin") else (lambda v:v)
    rel=posixpath.relpath(fold(posixpath.normpath(path)),fold(posixpath.normpath(root)))
    return rel=="." or not (rel==".." or rel.startswith("../"))

def decide_write(ctx,fn,target):
    absolute=posixpath.normpath(target if target.startswith("/") else posixpath.join(ctx["cwd"],target))
    co=ctx.get("checkout")
    if not co or not inside(co,absolute,ctx["platform"]): return None
    if any(inside(r,absolute,ctx["platform"]) for r in ctx["sandboxRoots"]): return None
    local=posixpath.relpath(absolute,posixpath.normpath(co))
    if any(local==x or local.startswith(x+"/") for x in ctx["allowedCheckoutPrefixes"]): return None
    return "protected-write" if local.split("/")[0] in ctx["protectedPrefixes"] else "checkout-write"

def decide_spawn(ctx,p,command,args):
    g=p["guard"]; program=re.sub(r"\.exe$","",command.split("/")[-1],flags=re.I)
    if program in g["killPrograms"]: return "process-kill"
    if program not in g["kernelPrograms"]: return None
    verb=next((a for a in args if not a.startswith("-")),None)
    if verb is None or verb not in g["kernelVerbs"]: return None
    for a in args:
        if a.startswith("-") or not re.search(r"[\\/]",a): continue
        absolute=posixpath.normpath(a if a.startswith("/") else posixpath.join(ctx["cwd"],a))
        if not any(inside(r,absolute,ctx["platform"]) for r in ctx["allowed"]): return "kernel-image"
    return None

def decide_listen(ctx,port,leased):
    if port==0 or port<1024: return None
    return None if port in leased else "fixed-port"

def main():
    p=json.load(open(sys.argv[1],encoding="utf8")); corpus=json.load(open(sys.argv[2],encoding="utf8"))
    law=compile_law(p); bad=[]
    for c in corpus["scans"]:
        got=sorted(scan_source(law,c["path"],c["language"],c["text"]))
        want=sorted((x["rule"],x["line"]) for x in c["expected"])
        if got!=want: bad.append(("scan",c["id"],got,want))
    for c in corpus["owners"]:
        got=sorted(scan_owner(law,c["sources"])); want=sorted((x["rule"],1) for x in c["expected"])
        if got!=want: bad.append(("owner",c["id"],got,want))
    for c in corpus["paths"]:
        got=bool(language(law,c["path"])) and is_test_path(law,c["path"],c.get("declared",[]))
        if got!=c["test"]: bad.append(("path",c["path"],got,c["test"]))
    for c in corpus["writes"]:
        got=decide_write(c["context"],c["function"],c["target"])
        if got!=c["expected"]: bad.append(("write",c["id"],got,c["expected"]))
    for c in corpus["spawns"]:
        got=decide_spawn(c["context"],p,c["command"],c["args"])
        if got!=c["expected"]: bad.append(("spawn",c["id"],got,c["expected"]))
    for c in corpus["listens"]:
        got=decide_listen(c["context"],c["port"],c["leased"])
        if got!=c["expected"]: bad.append(("listen",c["id"],got,c["expected"]))
    for b in bad: print("MISMATCH",b)
    print("oracle cases",len(corpus["scans"])+len(corpus["owners"])+len(corpus["paths"])+len(corpus["writes"])+len(corpus["spawns"])+len(corpus["listens"]),"mismatches",len(bad))
    sys.exit(1 if bad else 0)
if __name__=="__main__": main()

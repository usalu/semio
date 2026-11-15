"""🐍️ Independent oracle of the boundary law: reads the scheduling table on stdin and the checkout root as the first argument, and answers the status of every member of the build and caching modules and, per domain, the imports from keyed code into files that are not ports. Written without the TypeScript scanner: a plain regular expression over the import forms."""
import json,os,re,sys
LIB="🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/"
SPEC=re.compile(r'''(?:from\s*|import\s*\(\s*|import\s+|require\s*\(\s*)["'](\.{1,2}/[^"']+)["']''')
CODE=re.compile(r'\.(?:ts|mjs|js|cjs|mts|tsx)$')
def walk(root):
    out=[]
    for module in ("🏗️build","⚡️caching"):
        base=os.path.join(root,LIB+module)
        for directory,names,files in os.walk(base):
            names[:]=[n for n in names if n!="node_modules"]
            for f in files:out.append(os.path.relpath(os.path.join(directory,f),root))
    return out
def main():
    root=sys.argv[1];table=json.load(sys.stdin);files=walk(root);known=set(files)
    prefixes=[p for d in table["domains"] for p in d["prefixes"]]
    members={}
    for f in files:
        parts=f[len(LIB):].split("/")
        name=parts[0]+"/"+(parts[1] if len(parts)>2 else ".")
        covered=any(f.startswith(p) for p in prefixes)
        members.setdefault(name,[0,0]);members[name][0 if covered else 1]+=1
    status={n:("scheduling" if k==0 else "keyed" if c==0 else "mixed") for n,(c,k) in members.items()}
    graph={}
    for f in files:
        if not CODE.search(f) or "/🧪️tests/" in f:continue
        try:src=open(os.path.join(root,f),encoding="utf8").read()
        except OSError:continue
        found=set()
        for m in SPEC.finditer(src):
            p=os.path.normpath(os.path.join(os.path.dirname(f),m.group(1)))
            for c in (p,re.sub(r"\.js$",".ts",p),re.sub(r"\.mjs$",".d.mts",p)):
                if c in known:found.add(c);break
        graph[f]=found
    violations={}
    for d in table["domains"]:
        inside=lambda f:any(f.startswith(p) for p in d["prefixes"]);ports=set(d["ports"]);bad=set()
        for f,targets in graph.items():
            if inside(f):continue
            for t in targets:
                if inside(t) and t not in ports:bad.add(f+" -> "+t)
        violations[d["id"]]=sorted(bad)
    json.dump({"members":status,"violations":violations},sys.stdout,ensure_ascii=False)
main()

import json,sys,fnmatch
def main(path):
    data=json.load(open(path,encoding="utf8"));projects=data["projects"];out={}
    for case in data["cases"]:
        if "error" in case:continue
        tasks=set();projs=set();stack=[(t["project"],t["target"]) for t in case["scope"]["targets"]]
        def bridge(p,pat):
            seen=set();pending=list(projects[p]["dependencies"]);hit=[]
            while pending:
                o=pending.pop(0)
                if o in seen:continue
                seen.add(o)
                ts=[t for t in projects[o]["targets"] if fnmatch.fnmatchcase(t,pat)]
                if ts:hit+=[(o,t) for t in ts]
                else:pending+=projects[o]["dependencies"]
            return hit
        while stack:
            p,t=stack.pop()
            if (p,t) in tasks:continue
            tasks.add((p,t))
            for d in projects[p]["targets"][t].get("dependsOn",[]):
                if isinstance(d,str):
                    if d.startswith("^"):stack+=bridge(p,d[1:])
                    else:
                        o,pat=(d.split(":",1) if ":" in d and d.split(":",1)[0] in projects else (p,d))
                        stack+=[(o,x) for x in projects[o]["targets"] if fnmatch.fnmatchcase(x,pat)]
                elif d.get("dependencies"):stack+=bridge(p,d["target"])
                else:
                    owners=d.get("projects",[p])
                    stack+=[(o,x) for o in owners for x in projects[o]["targets"] if fnmatch.fnmatchcase(x,d["target"])]
        work=[p for p,_ in tasks]
        while work:
            n=work.pop()
            if n in projs:continue
            projs.add(n);work+=projects[n]["dependencies"]
        if case["scope"]["includeToolClosure"]:
            work=list(projs)
            while work:
                n=work.pop()
                for x in projects[n].get("tools",[])+projects[n]["dependencies"]:
                    if x not in projs:projs.add(x);work.append(x)
        out[case["name"]]={"tasks":sorted(a+":"+b for a,b in tasks),"projects":sorted(projs)}
    json.dump(out,sys.stdout)
main(sys.argv[1])

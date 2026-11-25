import json,re,sys
def transient(lines,root,patterns):
    plain=[re.sub(r"\x1b\[[0-9;]*m","",l) for l in lines]
    for p in patterns:
        if root=="" and "{root}" in p:continue
        e=re.compile(p.replace("{root}",re.escape(root)))
        if any(e.search(l) for l in plain):return True
    return False
def retry(failure,tree,debounce):
    if failure.get("code")!="source-transient" or failure.get("tree") is None or tree is None or tree["stamp"]==failure["tree"]:return failure["nextAttemptMs"]
    return min(failure["nextAttemptMs"],max(failure.get("atMs",0),tree["changedMs"])+debounce)
a=json.loads(sys.argv[2])
if sys.argv[1]=="classify":print(json.dumps([transient(r["lines"],r["root"],r["patterns"]) for r in a]))
else:print(json.dumps([retry(r["failure"],r["tree"],r["debounce"]) for r in a]))

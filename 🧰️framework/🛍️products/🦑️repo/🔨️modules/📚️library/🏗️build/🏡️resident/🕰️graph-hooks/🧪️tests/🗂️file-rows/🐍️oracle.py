import json,sys
def rel(root,path):
    p=path.replace("\\","/")
    r=root.replace("\\","/").rstrip("/")
    absolute=p.startswith("/") or (len(p)>2 and p[1]==":" and p[2]=="/" and p[0].isalpha())
    if absolute:
        if p==r:return ""
        return p[len(r)+1:] if p.startswith(r+"/") else None
    return None if p.startswith("..") else p
def level(case):
    path=rel(case["root"],case["path"])
    if path is None:return 1
    exact=set(case["exact"]);subtrees=set(case["subtrees"])
    at=path
    if at in exact:return 1
    while True:
        if at in subtrees:return 1
        if "/" not in at:break
        at=at[:at.rindex("/")]
    return 0
print(json.dumps([level(c) for c in json.load(open(sys.argv[1]))]))

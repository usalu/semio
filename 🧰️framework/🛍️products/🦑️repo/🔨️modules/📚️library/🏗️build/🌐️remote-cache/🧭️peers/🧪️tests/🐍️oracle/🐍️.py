#!/usr/bin/env python3
"""🐍️ Independent oracle of the fleet peers. It shares no code with `../../🟦️.ts`: with the standard library only it judges the relayed exchange vectors (path, request, answer), the SHA-256 of the answer bodies and the CIDR membership that the optional network policy uses.
usage: 🐍️.py check    judge every vector of ../🧫️fixtures/🔣️.json and ../🧫️fixtures/🌍️.json; exit 0 when all agree with their expectation
Python 3.9 compatible."""
import hashlib,ipaddress,json,os,re,sys
HERE=os.path.dirname(os.path.abspath(__file__))
GROUP=r"[a-z][a-z0-9]{0,15}"
PATH=r"/([a-z0-9][a-z0-9._-]{0,63}/)?(ac/[0-9a-f]{64}|cas/[0-9a-f]{64}|v1/cache/[a-z0-9_-]{1,128})"
ID=r"[0-9a-f]{24}"
def whole(pattern,value): return isinstance(value,str) and re.fullmatch(pattern,value) is not None
def integer(value,low,high=None): return type(value) is int and value>=low and (high is None or value<=high)
def exact(node,required,optional=()):
    return isinstance(node,dict) and set(required)<=set(node)<=set(required)|set(optional)
def path(value): return whole(PATH,value)
def request(v):
    return exact(v,["version","id","from","to","path","createdMs"]) and v["version"]==1 and whole(ID,v["id"]) and whole(GROUP,v["from"]) and whole(GROUP,v["to"]) and path(v["path"]) and integer(v["createdMs"],0)
def answer(v):
    if not exact(v,["version","id","status","atMs"],["size","sha256"]): return False
    return v["version"]==1 and whole(ID,v["id"]) and integer(v["status"],100,599) and integer(v["atMs"],0) and ("size" not in v or integer(v["size"],0)) and ("sha256" not in v or whole(r"[0-9a-f]{64}",v["sha256"]))
JUDGES={"Path":path,"Request":request,"Answer":answer}
def allowed(peer,network):
    if peer in ("::1",) or peer.startswith("127."): return True
    try: return ipaddress.ip_address(peer.replace("::ffff:","")) in ipaddress.ip_network(network)
    except ValueError: return False
def check():
    failures=[]
    vectors=json.load(open(os.path.join(HERE,"..","🧫️fixtures","🔣️.json"),encoding="utf8"))
    for name,rows in vectors.items():
        for expected,key in ((True,"valid"),(False,"invalid")):
            for value in rows[key]:
                if JUDGES[name](value)!=expected: failures.append("%s %s should be %s: %s"%(name,key,expected,json.dumps(value)[:80]))
    other=json.load(open(os.path.join(HERE,"..","🧫️fixtures","🌍️.json"),encoding="utf8"))
    for peer,network,expected in other["cidr"]:
        if allowed(peer,network)!=expected: failures.append("cidr %s in %s should be %s"%(peer,network,expected))
    for text,digest in other["sha256"]:
        if hashlib.sha256(text.encode("utf8")).hexdigest()!=digest: failures.append("sha256 of "+text)
    for failure in failures: print(failure)
    return 0 if not failures else 1
if __name__=="__main__":
    sys.exit(check() if sys.argv[1:]==["check"] else 2)

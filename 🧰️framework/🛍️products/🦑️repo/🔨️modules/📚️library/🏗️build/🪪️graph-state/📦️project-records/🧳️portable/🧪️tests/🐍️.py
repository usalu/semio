#!/usr/bin/env python3
"""Independent oracle of the portable project record: derives the neutral form of a record and its form for a taker from the schema and the fixture alone, with Python's own Windows path semantics (`PureWindowsPath`, which compares drive and parts without case and accepts both separators) and its own SHA-256 of the sorted recipe JSON. Usage: portable.oracle.py <schema.json> <fixture.json>  (prints the cases, paths, hosts and records as JSON)"""
import copy,hashlib,json,re,sys
from pathlib import PurePosixPath,PureWindowsPath

schema=json.load(open(sys.argv[1],encoding="utf8"));P=schema["definitions"]["Policy"]["const"]
TOKEN=P["rootToken"];OWNER=P["ownerToken"];GEN=P["token"];UNAVAILABLE=P["unavailable"]

class Unportable(Exception):pass

def is_windows(root):return re.match(r"^([A-Za-z]:([\\/]|$)|[\\/]{2}[^\\/])",root) is not None

def strip_extended(path):return re.sub(r"^[\\/]{2}\?[\\/]","",path)

def remainder(path,root):
    if is_windows(root):
        text=strip_extended(path);p,r=PureWindowsPath(text),PureWindowsPath(strip_extended(root))
        if p==r:return ""
        if p.is_relative_to(r):return "/"+"/".join(p.relative_to(r).parts)
        return None
    p,r=PurePosixPath(path),PurePosixPath(root)
    if p==r:return ""
    if r in p.parents:return "/"+"/".join(p.relative_to(r).parts)
    return None

def neutral_path(path,root):
    rest=remainder(path,root)
    return path if rest is None else TOKEN+rest

def local_path(path,root):
    if not path.startswith(TOKEN):return path
    rest=path[len(TOKEN):]
    if rest!="" and rest[0]!="/":return path
    if is_windows(root):return str(PureWindowsPath(root,*[x for x in rest.split("/") if x])) if rest else str(PureWindowsPath(root))
    return root.rstrip("/")+rest

LEAD=r"(?<![\w.\-])";BOUND=r"(?![\w.\-])"
def slash_root(root):return re.sub(r"/+$","",strip_extended(root).replace("\\","/"))

def neutral_text(text,root):
    if not is_windows(root):
        return re.sub(LEAD+re.escape(root.rstrip("/"))+BOUND,lambda m:TOKEN,text)
    moved=re.sub(LEAD+re.escape(slash_root(root))+BOUND,lambda m:TOKEN,text,flags=re.I)
    raw=strip_extended(root).replace("/","\\").rstrip("\\")
    for spelling in (raw,raw.replace("\\","\\\\")):
        if re.search(LEAD+re.escape(spelling)+BOUND,moved,flags=re.I):raise Unportable("backslash spelling")
    return moved

def local_text(text,root):
    moved=text.replace(TOKEN,slash_root(root) if is_windows(root) else root.rstrip("/"))
    if P["residue"] in moved:raise Unportable("residue")
    return moved

def sorted_json(value):
    if isinstance(value,list):return [sorted_json(x) for x in value]
    if isinstance(value,dict):return {k:sorted_json(value[k]) for k in sorted(value)}
    return value
def closure_key(recipe):return hashlib.sha256(json.dumps(sorted_json(recipe),separators=(",",":"),ensure_ascii=False).encode("utf8")).hexdigest()

def dumps(value):return json.dumps(value,separators=(",",":"),ensure_ascii=False)

def owner_neutral(text):
    m=re.match(P["ownerPattern"],text)
    return text if m is None else m.group(1)+OWNER
def owner_local(providers):
    def f(text):
        if not text.endswith(OWNER):return text
        prefix=text[:-len(OWNER)]
        for pid in providers:
            m=re.match(P["ownerPattern"],pid)
            if m and m.group(1)==prefix:return pid
        return text
    return f

def convert_row(row,way):
    kind=row["kind"];row=dict(row)
    if kind in P["jsonKinds"]:
        try:items=json.loads(row["path"])
        except Exception:items=None
        if isinstance(items,list):
            def deep(v):
                if isinstance(v,str):return way["text"](v)
                if isinstance(v,list):return [deep(x) for x in v]
                if isinstance(v,dict):return {k:deep(x) for k,x in v.items()}
                return v
            items=[way["path"](x) if i==0 and isinstance(x,str) else (way["keys"].get(x,x) if i==1 and kind=="closure" and isinstance(x,str) else (x if i==0 else deep(x))) for i,x in enumerate(items)]
            row["path"]=dumps(items)
        if kind=="closure" and isinstance(row.get("fact"),str):row["fact"]=way["text"](row["fact"])
        elif kind in P["factPathKinds"] and isinstance(row.get("fact"),str):row["fact"]=way["path"](row["fact"])
        return row
    if kind=="analysis" and isinstance(row.get("fact"),dict):
        fact=dict(row["fact"]);row["path"]=way["path"](row["path"])
        if isinstance(fact.get("provider"),str):fact["provider"]=way["owner"](fact["provider"])
        if isinstance(fact.get("nestedObservations"),list):fact["nestedObservations"]=[convert_row(x,way) for x in fact["nestedObservations"]]
        row["fact"]=fact;return row
    if kind in P["pathKinds"]:
        row["path"]=way["path"](row["path"])
        if kind in P["factPathKinds"] and isinstance(row.get("fact"),str):row["fact"]=way["path"](row["fact"])
        return row
    row["path"]=way["text"](row["path"])
    if isinstance(row.get("fact"),str):row["fact"]=way["text"](row["fact"])
    return row

def above(path,root):
    win=is_windows(root);f=(lambda x:x.lower()) if win else (lambda x:x)
    r=f(root.replace("\\","/").rstrip("/")+"/");p=f(path.replace("\\","/").rstrip("/")+"/")
    return r!=p and r.startswith(p)

def generation_tokens(rows):
    pat=re.compile(r"(\.cache/b/d/m/j[\\/])([0-9a-f]{16,64})(?=[\\/]|$)")
    ids=set()
    for row in rows:
        for m in pat.finditer(json.dumps([row["path"],row.get("fact")],ensure_ascii=False)):ids.add(m.group(2))
    if len(ids)!=1:return rows
    (ident,)=ids
    def sub(text):return re.sub(r"(\.cache/b/d/m/j[\\/])"+ident+r"(?=[\\/]|$)",lambda m:m.group(1)+GEN,text)
    def deep(v):
        if isinstance(v,str):return sub(v)
        if isinstance(v,list):return [deep(x) for x in v]
        if isinstance(v,dict):return {k:deep(x) for k,x in v.items()}
        return v
    out=[]
    for row in rows:
        row=dict(row);row["path"]=sub(row["path"])
        if "fact" in row:row["fact"]=deep(row["fact"])
        out.append(row)
    return out

def neutral_way(root,keys):return {"keys":keys,"owner":owner_neutral,"path":lambda x:neutral_path(x,root),"text":lambda x:neutral_text(x,root)}
def local_way(root,keys,providers):return {"keys":keys,"owner":owner_local(providers),"path":lambda x:local_path(x,root),"text":lambda x:local_text(x,root)}

def zero_stamps(fact):return {k:(P["stampValue"] if k in P["stampFields"] and isinstance(v,(int,float)) and not isinstance(v,bool) else v) for k,v in fact.items()}

def unstamp(row):
    fact=row.get("fact");row=dict(row)
    if not isinstance(fact,dict):return row
    if row["kind"]=="file" and "error" not in fact:row["fact"]=zero_stamps(fact);return row
    if row["kind"]!="analysis":return row
    fact=dict(fact)
    if isinstance(fact.get("source"),dict):fact["source"]=zero_stamps(fact["source"])
    if isinstance(fact.get("nestedObservations"),list):fact["nestedObservations"]=[unstamp(x) for x in fact["nestedObservations"]]
    row["fact"]=fact;return row

def neutral_rows(rows,root,keys):
    way=neutral_way(root,keys)
    kept=[r for r in rows if not (r["kind"] in P["locationKinds"] and r.get("fact")=="directory" and above(r["path"],root))]
    return generation_tokens([unstamp(convert_row(r,way)) for r in kept])

def local_rows(rows,root,keys,providers,generation):
    way=local_way(root,keys,providers);put=generation if generation is not None else UNAVAILABLE
    def deep(v):
        if isinstance(v,str):return v.replace(GEN,put)
        if isinstance(v,list):return [deep(x) for x in v]
        if isinstance(v,dict):return {k:deep(x) for k,x in v.items()}
        return v
    out=[]
    for row in rows:
        row=convert_row(row,way);row["path"]=row["path"].replace(GEN,put)
        if "fact" in row:row["fact"]=deep(row["fact"])
        out.append(row)
    return out

def markers(value,keys):
    if isinstance(value,list):return [markers(x,keys) for x in value]
    if isinstance(value,dict):
        if list(value)==["closureRef"] and isinstance(value["closureRef"],str):return {"closureRef":keys.get(value["closureRef"],value["closureRef"])}
        return {k:markers(x,keys) for k,x in value.items()}
    return value

def move_image(image,text,check):
    try:parsed=json.loads(image)
    except Exception:raise Unportable("no json")
    keys={};moved=[];table=parsed.get("closureRecipes")
    if isinstance(table,dict):
        for key,recipe in table.items():
            if check and closure_key(recipe)!=key:raise Unportable("key")
            nxt=json.loads(text(dumps(recipe)));nk=closure_key(nxt);keys[key]=nk;moved.append((nk,nxt))
    body=dict(parsed)
    if isinstance(table,dict):body["closureRecipes"]=dict(sorted(moved) if check else moved)
    return text(dumps(markers(body,keys))),keys

def neutral_record(rec,root):
    try:
        image,keys=move_image(rec["image"],lambda t:neutral_text(t,root),True)
        return {"digest":rec["digest"],"authority":neutral_rows(rec["authority"],root,keys),"observations":neutral_rows(rec["observations"],root,keys),"image":image},keys
    except Unportable:return None,{}

def local_record(rec,root,providers,generation):
    image,keys=move_image(rec["image"],lambda t:local_text(t,root),False)
    return {"digest":rec["digest"],"authority":local_rows(rec["authority"],root,keys,providers,generation),"observations":local_rows(rec["observations"],root,keys,providers,generation),"image":image}

def neutral_bundle(bundle,root,keys):
    rest={k:v for k,v in bundle.items() if k not in("rows","layers")};moved=json.loads(neutral_text(dumps(rest),root))
    moved["rows"]=neutral_rows(bundle["rows"],root,keys);moved["layers"]=[neutral_rows(l,root,keys) for l in bundle["layers"]]
    return closure_key(moved["recipe"]),moved

def local_bundle(bundle,root,keys,providers,generation):
    rest={k:v for k,v in bundle.items() if k not in("rows","layers")};moved=json.loads(local_text(dumps(rest),root))
    moved["rows"]=local_rows(bundle["rows"],root,keys,providers,generation);moved["layers"]=[local_rows(l,root,keys,providers,generation) for l in bundle["layers"]]
    return closure_key(moved["recipe"]),moved

def run(fixture):
    out={"paths":[],"hosts":[],"records":[]}
    for c in fixture["paths"]:
        n=neutral_path(c["path"],c["root"]);out["paths"].append({"name":c["name"],"neutral":n,"local":None if n==c["path"] else local_path(n,c["root"])})
    for c in fixture["hosts"]:out["hosts"].append({"name":c["name"],"local":local_path(c["neutral"],c["root"])})
    for c in fixture["records"]:
        neutral,keys=neutral_record(c["producer"],c["root"])
        row={"name":c["name"],"neutral":neutral,"takers":[]}
        if neutral is not None:
            for t in c["takers"]:
                try:local=local_record(neutral,t["root"],t["providers"],t["generation"])
                except Unportable:local=None
                row["takers"].append({"root":t["root"],"local":local})
        out["records"].append(row)
    return out

def generation_case(case):
    kept=[r for r in case["rows"] if not (r["kind"] in P["locationKinds"] and r.get("fact")=="directory" and above(r["path"],case["root"]))]
    portable=generation_tokens(kept)
    put=case["generation"] if case["generation"] is not None else UNAVAILABLE
    def deep(v):
        if isinstance(v,str):return v.replace(GEN,put)
        if isinstance(v,list):return [deep(x) for x in v]
        if isinstance(v,dict):return {k:deep(x) for k,x in v.items()}
        return v
    local=[dict(r,path=r["path"].replace(GEN,put),**({"fact":deep(r["fact"])} if "fact" in r else {})) for r in portable]
    return {"name":case["name"],"portable":portable,"local":local}

if __name__=="__main__":
    fixture=json.load(open(sys.argv[2],encoding="utf8"));out=run(fixture);out["cases"]=[generation_case(c) for c in fixture["cases"]]
    print(json.dumps(out,ensure_ascii=False,sort_keys=True))

#!/usr/bin/env python3
"""🐍️ Independent oracle of the fleet topology. It shares no code with `../🟦️.ts`: it validates a topology structurally with the standard library and recomputes the environment of a machine from the topology alone.
usage: 🐍️.py check                          validate the committed topology and the fixture (valid accepted, every invalid case refused, environments recomputed)
       🐍️.py env <topology.json|fixture.json> <group>    print the environment of one group as JSON
       🐍️.py plist <plist> <environment> <worktree> <language>    insert the fleet environment into the first EnvironmentVariables dict, the same way the macOS provision script does
       🐍️.py logs <plist>    print the unique StandardOutPath and StandardErrorPath values
       🐍️.py namespace <root> <link>    print keep or remove for a node_modules link
Python 3.9 compatible."""
from pathlib import Path
import copy,glob,json,os,re,sys
HERE=os.path.dirname(os.path.abspath(__file__))
HOST=r"[a-z0-9][a-z0-9.-]{0,251}[a-z0-9]"
GROUP=r"[a-z][a-z0-9]{0,15}"
UNIX=r"/[A-Za-z0-9._/-]{1,200}"
WINDOWS=r"[A-Za-z]:\\[A-Za-z0-9._\\-]{1,200}"
WINDOWS_ROOT=r"[A-Za-z]:\\[A-Za-z0-9._-]{1,12}"
SHARE=r"/[A-Za-z0-9._-]+"
DRIVE=r"[A-Za-z]:\\"
ENDPOINT=r"https?://"+HOST+r"(:[0-9]{1,5})?"
INSTANCE=r"[A-Za-z0-9._-]{1,64}"
PLATFORMS=("linux","macos","windows")
NAMES={"endpoint":"SEMIO_REMOTE_CACHE_ENDPOINT","access":"SEMIO_REMOTE_CACHE_ACCESS","instance":"SEMIO_REMOTE_CACHE_INSTANCE"}

def whole(pattern,value): return isinstance(value,str) and re.fullmatch(pattern,value) is not None
def integer(value,low,high): return type(value) is int and low<=value<=high
def cidr(value):
    if not isinstance(value,str) or value.count("/")!=1: return False
    address,prefix=value.split("/")
    octets=address.split(".")
    if len(octets)!=4 or not prefix.isascii() or not prefix.isdigit() or int(prefix)>32 or (len(prefix)>1 and prefix[0]=="0"): return False
    return all(o.isascii() and o.isdigit() and int(o)<=255 and (o=="0" or o[0]!="0") for o in octets)
def exact(node,keys,where,out):
    if not isinstance(node,dict):
        out.append(where+" is not an object");return False
    if sorted(node)!=sorted(keys):
        out.append(where+" must contain exactly "+",".join(sorted(keys)));return False
    return True

def problems(topology):
    out=[]
    if not exact(topology,["version","worktree","storeMount","groups","replica"],"topology",out): return out
    if type(topology["version"]) is not int or topology["version"]!=1: out.append("version must be 1")
    for name,pattern,drive in (("worktree",UNIX,WINDOWS_ROOT),("storeMount",UNIX,DRIVE)):
        node=topology[name]
        if exact(node,list(PLATFORMS),name,out):
            for platform in ("linux","macos"):
                if not whole(pattern,node[platform]): out.append(name+"."+platform+" invalid")
            if not whole(drive,node["windows"]): out.append(name+".windows invalid")
    replica=topology["replica"]
    if exact(replica,["pollMilliseconds","scanSeconds","debounceMilliseconds","retentionDays","bootstrapMinutes"],"replica",out):
        for key,low,high in (("pollMilliseconds",50,60000),("scanSeconds",1,3600),("debounceMilliseconds",10,10000),("retentionDays",1,3650),("bootstrapMinutes",1,10080)):
            if not integer(replica[key],low,high): out.append("replica."+key+" invalid")
    groups=topology["groups"]
    if not isinstance(groups,list) or not 1<=len(groups)<=64:
        out.append("groups must hold 1 to 64 entries");return out
    ids=[]
    for index,group in enumerate(groups):
        where="groups/"+str(index)
        if not exact(group,["id","nfs","server","cache","peers"],where,out): continue
        if not whole(GROUP,group["id"]): out.append(where+" id invalid")
        else: ids.append(group["id"])
        if exact(group["nfs"],["host","export"],where+" nfs",out):
            if not whole(HOST,group["nfs"]["host"]): out.append(where+" nfs.host invalid")
            if not whole(UNIX,group["nfs"]["export"]): out.append(where+" nfs.export invalid")
        server=group["server"]
        if isinstance(server,dict) and "directory" in server:
            if not whole(WINDOWS,server["directory"]): out.append(where+" server.directory invalid")
            server={k:v for k,v in server.items() if k!="directory"}
        if exact(server,["platform","protocol","network"],where+" server",out):
            if server["platform"] not in PLATFORMS: out.append(where+" server.platform invalid")
            if server["protocol"] not in ("nfs","smb"): out.append(where+" server.protocol invalid")
            if not cidr(server["network"]): out.append(where+" server.network invalid")
        if exact(group["cache"],["endpoint","instance"],where+" cache",out):
            if not whole(ENDPOINT,group["cache"]["endpoint"]): out.append(where+" cache.endpoint invalid")
            if not whole(INSTANCE,group["cache"]["instance"]): out.append(where+" cache.instance invalid")
        peers=group["peers"]
        if not isinstance(peers,dict): out.append(where+" peers is not an object")
        else:
            for peer,path in peers.items():
                if not whole(GROUP,peer) or not whole(UNIX,path): out.append(where+" peer "+str(peer)+" invalid")
    if len(set(ids))!=len(ids): out.append("group ids must be unique")
    for group in groups:
        server=group.get("server") if isinstance(group,dict) else None
        nfs=group.get("nfs") if isinstance(group,dict) else None
        if not isinstance(server,dict) or not isinstance(nfs,dict) or not isinstance(nfs.get("export"),str): continue
        name=str(group.get("id"))
        if server.get("protocol")=="smb" and server.get("platform")!="windows": out.append(name+" shares over smb off windows")
        if server.get("platform")=="windows":
            if "directory" not in server: out.append(name+" windows server needs a directory")
            if not whole(SHARE,nfs["export"]): out.append(name+" windows server needs a single-segment share name")
        elif "directory" in server: out.append(name+" directory only on windows")
        if server.get("platform")=="macos" and not (nfs["export"].startswith("/Users/") or nfs["export"].startswith("/Volumes/")): out.append(name+" macos export outside /Users and /Volumes")
    servers=[g.get("server") for g in groups if isinstance(g,dict) and isinstance(g.get("server"),dict)]
    if any(x.get("protocol")=="smb" for x in servers) and any(x.get("platform")=="macos" for x in servers): out.append("a macos server cannot read an smb store")
    for group in groups:
        if isinstance(group,dict) and isinstance(group.get("peers"),dict) and isinstance(group.get("id"),str):
            if sorted(group["peers"])!=sorted(i for i in ids if i!=group["id"]): out.append(group["id"]+" must mount exactly the other groups")
    return out

def environment(topology,group_id):
    found=[g for g in topology["groups"] if g["id"]==group_id]
    if len(found)!=1: raise SystemExit("unknown group "+group_id)
    cache=found[0]["cache"]
    return {NAMES["endpoint"]:cache["endpoint"],NAMES["access"]:"read-write",NAMES["instance"]:cache["instance"]}

def mutate(base,change):
    copied=copy.deepcopy(base);node=copied
    for key in change["path"][:-1]: node=node[key]
    last=change["path"][-1]
    if change.get("remove"): del node[last]
    else: node[last]=change["value"]
    return copied

def load(path):
    with open(path,encoding="utf8") as handle: return json.load(handle)

def check():
    failures=[]
    committed=load(glob.glob(os.path.join(HERE,"..","*.json"))[0])
    fixture=load(glob.glob(os.path.join(HERE,"*","*.json"))[0])
    for message in problems(committed): failures.append("committed: "+message)
    if [g["id"] for g in committed["groups"]]!=["g%d"%n for n in range(1,6)]: failures.append("committed: groups must be g1..g5")
    for group in committed["groups"]:
        n=group["id"]
        if n=="g1":
            if group["nfs"]!={"host":"nfs.g1.semio.internal","export":"/Users/Shared/semio-store"}: failures.append("committed: nfs of g1")
            if group["server"]!={"platform":"macos","protocol":"nfs","network":"10.0.1.0/24"}: failures.append("committed: server of g1")
            if group["cache"]!={"endpoint":"http://cache.g1.semio.internal:6090","instance":"semio"}: failures.append("committed: cache of g1")
            if environment(committed,n)!={NAMES["endpoint"]:"http://cache.g1.semio.internal:6090",NAMES["access"]:"read-write",NAMES["instance"]:"semio"}: failures.append("committed: environment of g1")
        else:
            if group["nfs"]!={"host":"nfs.%s.semio.internal"%n,"export":"/srv/semio-store"}: failures.append("committed: nfs of "+n)
            if group["server"]!={"platform":"linux","protocol":"nfs","network":"10.0.%s.0/24"%n[1:]}: failures.append("committed: server of "+n)
            if group["cache"]!={"endpoint":"http://cache.%s.semio.internal:6090"%n,"instance":"semio"}: failures.append("committed: cache of "+n)
            if environment(committed,n)!={NAMES["endpoint"]:"http://cache.%s.semio.internal:6090"%n,NAMES["access"]:"read-write",NAMES["instance"]:"semio"}: failures.append("committed: environment of "+n)
        if group["peers"]!={m["id"]:"/mnt/semio-peers/"+m["id"] for m in committed["groups"] if m["id"]!=n}: failures.append("committed: peers of "+n)
    if len(committed["worktree"]["windows"])>15: failures.append("committed: windows worktree root is not short")
    if committed["worktree"]!={"linux":"/srv/semio","macos":"/Users/Shared/semio","windows":"C:\\semio"}: failures.append("committed: worktree")
    if committed["storeMount"]!={"linux":"/mnt/semio-store","macos":"/Volumes/semio-store","windows":"S:\\"}: failures.append("committed: storeMount")
    if committed["replica"]!={"pollMilliseconds":250,"scanSeconds":900,"debounceMilliseconds":150,"retentionDays":14,"bootstrapMinutes":60}: failures.append("committed: replica")
    if fixture["variables"]!=NAMES: failures.append("fixture: variable names")
    for message in problems(fixture["topology"]): failures.append("fixture: "+message)
    for group_id,expected in fixture["environments"].items():
        if environment(fixture["topology"],group_id)!=expected: failures.append("fixture: environment of "+group_id)
    if sorted(fixture["environments"])!=sorted(g["id"] for g in fixture["topology"]["groups"]): failures.append("fixture: environments cover the groups")
    for change in fixture["invalid"]:
        if not problems(mutate(fixture["topology"],change)): failures.append("accepted: "+change["name"])
    return failures


def apply_address(topology, environment):
    def present(name):
        value=environment.get(name)
        return value if isinstance(value,str) and value!="" else None
    group_id,host,network,endpoint=present("SEMIO_FLEET_GROUP"),present("SEMIO_FLEET_NFS_HOST"),present("SEMIO_FLEET_NETWORK"),present("SEMIO_FLEET_CACHE_ENDPOINT")
    if group_id is None and host is None and network is None and endpoint is None: return topology
    if group_id is None or host is None or network is None: raise ValueError("SEMIO_FLEET_GROUP, SEMIO_FLEET_NFS_HOST and SEMIO_FLEET_NETWORK are set together")
    if not any(isinstance(group,dict) and group.get("id")==group_id for group in topology["groups"]): return topology
    def rewritten(current):
        match=re.fullmatch(r"(https?://)"+HOST+r"(:[0-9]{1,5})?", current)
        if match is None: raise ValueError("cache endpoint invalid")
        return match.group(1)+host+(match.group(2) or "")
    groups=[]
    for group in topology["groups"]:
        if group.get("id")!=group_id:
            groups.append(group); continue
        copied=copy.deepcopy(group)
        copied["nfs"]["host"]=host
        copied["server"]["network"]=network
        copied["cache"]["endpoint"]=endpoint or rewritten(group["cache"]["endpoint"])
        groups.append(copied)
    copied=copy.deepcopy(topology)
    copied["groups"]=groups
    return copied

def xml_escape(value):
    return value.replace("&","&amp;").replace("<","&lt;").replace(">","&gt;").replace('"',"&quot;")

def plist_environment(plist, environment_text, worktree, language):
    plist=plist.rstrip("\n")
    extra="<key>SEMIO_WORKSPACE_ROOT</key><string>"+xml_escape(worktree)+"</string><key>SEMIO_LANGUAGE</key><string>"+xml_escape(language)+"</string>"
    for line in environment_text.splitlines():
        if line=="": continue
        if line.startswith("export "): line=line[len("export "):]
        name,value=line.split("=",1)
        extra+="<key>"+xml_escape(name)+"</key><string>"+xml_escape(value)+"</string>"
    marker="<key>EnvironmentVariables</key><dict>"
    return plist.replace(marker, marker+extra, 1)


def daemon_logs(text):
    return sorted(set(re.findall(r"<key>Standard(?:Out|Error)Path</key><string>([^<]*)</string>", text)))


def foreign_namespace(root, link):
    import os
    if not os.path.islink(link):
        return "keep"
    target = os.readlink(link)
    if not target.startswith("/"):
        target = os.path.join(root, target)
    resolved = os.path.realpath(target) if os.path.isdir(target) else os.path.abspath(target)
    root = os.path.realpath(root)
    if resolved == root or resolved.startswith(root + os.sep):
        return "keep"
    return "remove"

def filter_exports(text, resolves):
    kept_lines=[]
    for line in text.splitlines():
        if line=="" or line.startswith("#"):
            kept_lines.append(line)
            continue
        tokens=line.split()
        kept=[tokens[0]]
        network=False
        hosts=False
        index=1
        while index<len(tokens):
            token=tokens[index]
            if token in ("-network","-mask"):
                kept.extend((token, tokens[index+1]))
                network=network or token=="-network"
                index+=2
                continue
            if token.startswith("-"):
                kept.append(token)
                index+=1
                continue
            if resolves(token):
                kept.append(token)
                hosts=True
            index+=1
        if network or hosts: kept_lines.append(" ".join(kept))
    return ("\n".join(kept_lines)+("\n" if kept_lines else ""))

if __name__=="__main__":
    if len(sys.argv)==3 and sys.argv[1]=="address":
        data=load(sys.argv[2])
        try: applied=apply_address(data.get("topology", data), os.environ)
        except ValueError as error:
            print(error, file=sys.stderr); sys.exit(1)
        group_id=os.environ.get("SEMIO_FLEET_GROUP","")
        found=[group for group in applied["groups"] if group["id"]==group_id]
        print(json.dumps({} if not found else {"endpoint":found[0]["cache"]["endpoint"],"host":found[0]["nfs"]["host"],"network":found[0]["server"]["network"]},sort_keys=True))
        sys.exit(0)
    if len(sys.argv)==6 and sys.argv[1]=="plist":
        print(plist_environment(Path(sys.argv[2]).read_text(), Path(sys.argv[3]).read_text(), sys.argv[4], sys.argv[5]), end="")
        sys.exit(0)
    if len(sys.argv)==3 and sys.argv[1]=="logs":
        for path in daemon_logs(Path(sys.argv[2]).read_text()): print(path)
        sys.exit(0)
    if len(sys.argv)==4 and sys.argv[1]=="namespace":
        print(foreign_namespace(sys.argv[2], sys.argv[3]))
        sys.exit(0)
    if len(sys.argv)==4 and sys.argv[1]=="exports":
        names=set(filter(None, sys.argv[3].split(",")))
        print(filter_exports(Path(sys.argv[2]).read_text(), lambda host: host in names), end="")
        sys.exit(0)
    if len(sys.argv)==2 and sys.argv[1]=="check":
        found=check()
        for line in found: print(line)
        sys.exit(1 if found else 0)
    if len(sys.argv)==4 and sys.argv[1]=="env":
        data=load(sys.argv[2]);print(json.dumps(environment(data.get("topology",data),sys.argv[3]),sort_keys=True));sys.exit(0)
    sys.exit(__doc__)

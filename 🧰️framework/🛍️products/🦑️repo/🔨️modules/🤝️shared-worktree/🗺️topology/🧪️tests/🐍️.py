#!/usr/bin/env python3
"""🐍️ Independent oracle of the fleet topology. It shares no code with `../🟦️.ts`: it validates a topology structurally with the standard library and recomputes the environment of a machine from the topology alone.
usage: 🐍️.py check                          validate the committed topology and the fixture (valid accepted, every invalid case refused)
       🐍️.py plist <plist> <environment> <worktree> <language>    insert the fleet environment into the first EnvironmentVariables dict, the same way the macOS provision script does
       🐍️.py worktree <topology.json|fixture.json> <platform> <home>    print the worktree of one platform below an account home
       🐍️.py fill <text> <home> <user> <xml|plain>    replace the home and user placeholders of a service definition, escaping for XML when asked
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
HOME_ROOT=r"~(/[A-Za-z0-9][A-Za-z0-9._-]{0,31}){1,4}"
SHARE=r"/[A-Za-z0-9._-]+"
DRIVE=r"[A-Za-z]:\\"
ENDPOINT=r"https?://"+HOST+r"(:[0-9]{1,5})?"
ORIGIN=r"[a-z][a-z0-9-]{0,63}"
INSTANCE=r"[A-Za-z0-9._-]{1,64}"
PLATFORMS=("linux","macos","windows")

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

def optional_problems(extra,group,where,out):
    if "ports" in extra:
        ports=extra["ports"]
        if not exact(ports,["cache","scheduler","worker"],where+" ports",out): pass
        elif not all(integer(ports[k],1,65535) for k in ports): out.append(where+" ports invalid")
    if "access" in extra:
        access=extra["access"]
        if isinstance(access,dict) and set(access)-{"enforceNetwork","transport"}: out.append(where+" access has unknown keys")
        elif not isinstance(access,dict) or "enforceNetwork" not in access: out.append(where+" access.enforceNetwork missing")
        else:
            if type(access["enforceNetwork"]) is not bool: out.append(where+" access.enforceNetwork invalid")
            transport=access.get("transport")
            if transport is not None:
                if not isinstance(transport,dict) or set(transport)-{"crossGroup","inGroup"}: out.append(where+" access.transport invalid")
                else:
                    if transport.get("crossGroup","sealed")!="sealed": out.append(where+" access.transport.crossGroup must be sealed")
                    if transport.get("inGroup","sealed") not in ("sealed","trusted-lan"): out.append(where+" access.transport.inGroup invalid")
    if "links" in extra:
        links=extra["links"]
        if not isinstance(links,dict): out.append(where+" links is not an object")
        else:
            for peer,link in links.items():
                if not whole(GROUP,peer) or not isinstance(link,dict) or link.get("transport") not in ("direct","relayed") or not set(link)<={"transport","endpoints","serverKey"}: out.append(where+" link "+str(peer)+" invalid");continue
                if "endpoints" in link and (not isinstance(link["endpoints"],list) or len(link["endpoints"])>8 or not all(whole(ENDPOINT,e) for e in link["endpoints"])): out.append(where+" link "+str(peer)+" endpoints invalid")
                if "serverKey" in link and not whole(r"[A-Za-z0-9_-]{40,200}",link["serverKey"]): out.append(where+" link "+str(peer)+" serverKey invalid")
                if link["transport"]=="relayed" and link.get("endpoints"): out.append(where+" relayed link lists endpoints")
                if isinstance(group.get("peers"),dict) and peer not in group["peers"]: out.append(where+" links a group that is not a peer")

def sidecar_problems(node,out):
    if not exact(node,["enabled","startAfterSeconds","graceSeconds","restart","children"],"sidecars",out): return
    if type(node["enabled"]) is not bool: out.append("sidecars.enabled invalid")
    if not integer(node["startAfterSeconds"],0,3600): out.append("sidecars.startAfterSeconds invalid")
    if not integer(node["graceSeconds"],1,300): out.append("sidecars.graceSeconds invalid")
    if exact(node["restart"],["initialMilliseconds","maximumMilliseconds","stableMilliseconds"],"sidecars.restart",out):
        for key,low,high in (("initialMilliseconds",10,600000),("maximumMilliseconds",10,3600000),("stableMilliseconds",10,86400000)):
            if not integer(node["restart"][key],low,high): out.append("sidecars.restart."+key+" invalid")
    children=node["children"]
    if not isinstance(children,list) or len(children)>8:
        out.append("sidecars.children must hold at most 8 entries");return
    names=[]
    for child in children:
        if not exact(child,["name","script","arguments","roles","requires","enabled"],"sidecars.child",out): continue
        if not whole(r"[a-z][a-z0-9-]{0,39}",child["name"]): out.append("sidecars.child.name invalid")
        names.append(child["name"])
        if not isinstance(child["script"],str) or not 1<=len(child["script"])<=240: out.append("sidecars.child.script invalid")
        if not isinstance(child["arguments"],list) or len(child["arguments"])>16 or not all(isinstance(a,str) and len(a)<=128 for a in child["arguments"]): out.append("sidecars.child.arguments invalid")
        roles=child["roles"]
        if not isinstance(roles,list) or not 1<=len(roles)<=2 or len(set(roles))!=len(roles) or not all(r in ("member","server") for r in roles): out.append("sidecars.child.roles invalid")
        if child["requires"] not in ("none","platform-peer"): out.append("sidecars.child.requires invalid")
        if type(child["enabled"]) is not bool: out.append("sidecars.child.enabled invalid")
    if len(set(names))!=len(names): out.append("sidecars.children names must be unique")

def problems(topology):
    out=[]
    if not exact(topology,["version","worktree","storeMount","groups","replica"]+(["sidecars"] if isinstance(topology,dict) and "sidecars" in topology else []),"topology",out): return out
    if "sidecars" in topology: sidecar_problems(topology["sidecars"],out)
    if type(topology["version"]) is not int or topology["version"]!=1: out.append("version must be 1")
    for name,pattern,drive in (("worktree",HOME_ROOT,WINDOWS_ROOT),("storeMount",UNIX,DRIVE)):
        node=topology[name]
        if exact(node,list(PLATFORMS),name,out):
            for platform in ("linux","macos"):
                if not whole(pattern,node[platform]): out.append(name+"."+platform+" invalid")
            if not whole(drive,node["windows"]): out.append(name+".windows invalid")
    replica=topology["replica"]
    if exact(replica,["pollMilliseconds","scanSeconds","debounceMilliseconds","retentionDays","bootstrapMinutes","compaction"],"replica",out):
        for key,low,high in (("pollMilliseconds",50,60000),("scanSeconds",1,3600),("debounceMilliseconds",10,10000),("retentionDays",1,3650),("bootstrapMinutes",1,10080)):
            if not integer(replica[key],low,high): out.append("replica."+key+" invalid")
        compaction=replica["compaction"]
        if exact(compaction,["automatic","snapshotEvents","safetyHours","staleHours","catchUpEvents"],"replica.compaction",out):
            if type(compaction["automatic"]) is not bool: out.append("replica.compaction.automatic invalid")
            for key,low,high in (("snapshotEvents",1000,100000000),("safetyHours",1,8760),("staleHours",1,8760),("catchUpEvents",1000,100000000)):
                if not integer(compaction[key],low,high): out.append("replica.compaction."+key+" invalid")
    groups=topology["groups"]
    if not isinstance(groups,list) or not 1<=len(groups)<=64:
        out.append("groups must hold 1 to 64 entries");return out
    ids=[]
    extras={}
    for index,group in enumerate(groups):
        where="groups/"+str(index)
        optional_keys=("ports","access","links")
        if isinstance(group,dict):
            for key in optional_keys:
                if key in group: extras.setdefault(index,{})[key]=group[key]
            group={k:v for k,v in group.items() if k not in optional_keys}
        if not exact(group,["id","roles","nfs","server","cache","peers"],where,out): continue
        optional_problems(extras.get(index,{}),group,where,out)
        if not whole(GROUP,group["id"]): out.append(where+" id invalid")
        else: ids.append(group["id"])
        if exact(group["roles"],["store","cache","scheduler","relay"],where+" roles",out):
            for role,origin in group["roles"].items():
                if not whole(ORIGIN,origin): out.append(where+" roles."+role+" invalid")
                elif not origin.startswith(str(group["id"])+"-"): out.append(where+" roles."+role+" names an origin of another group")
        if exact(group["nfs"],["export"],where+" nfs",out):
            if not whole(UNIX,group["nfs"]["export"]): out.append(where+" nfs.export invalid")
        server=group["server"]
        if isinstance(server,dict) and "directory" in server:
            if not whole(WINDOWS,server["directory"]): out.append(where+" server.directory invalid")
            server={k:v for k,v in server.items() if k!="directory"}
        network_given=isinstance(server,dict) and "network" in server
        if exact(server,["platform","protocol"]+(["network"] if network_given else []),where+" server",out):
            if server["platform"] not in PLATFORMS: out.append(where+" server.platform invalid")
            if server["protocol"] not in ("nfs","smb"): out.append(where+" server.protocol invalid")
            if network_given and not cidr(server["network"]): out.append(where+" server.network invalid")
        if exact(group["cache"],["instance"],where+" cache",out):
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
    fixture=load(os.path.join(HERE,"🧫️fixtures","🔣️.json"))
    for message in problems(committed): failures.append("committed: "+message)
    if [g["id"] for g in committed["groups"]]!=["g%d"%n for n in range(1,6)]: failures.append("committed: groups must be g1..g5")
    for group in committed["groups"]:
        n=group["id"]
        if group.get("ports")!={"cache":6094,"scheduler":8980,"worker":8981}: failures.append("committed: ports of "+n)
        if group["roles"]!={role:n+"-server" for role in ("store","cache","scheduler","relay")}: failures.append("committed: roles of "+n)
        if group.get("access")!={"enforceNetwork":False,"transport":{"crossGroup":"sealed","inGroup":"trusted-lan"}}: failures.append("committed: access of "+n)
        if n=="g1":
            if group["nfs"]!={"export":"/Users/Shared/semio-store"}: failures.append("committed: nfs of g1")
            if group["server"]!={"platform":"macos","protocol":"nfs"}: failures.append("committed: server of g1")
        else:
            if group["nfs"]!={"export":"/srv/semio-store"}: failures.append("committed: nfs of "+n)
            if group["server"]!={"platform":"linux","protocol":"nfs"}: failures.append("committed: server of "+n)
        if group["cache"]!={"instance":"main"}: failures.append("committed: cache of "+n)
        if group["peers"]!={m["id"]:"/mnt/semio-peers/"+m["id"] for m in committed["groups"] if m["id"]!=n}: failures.append("committed: peers of "+n)
    if len(committed["worktree"]["windows"])>15: failures.append("committed: windows worktree root is not short")
    if committed["worktree"]!={"linux":"~/semio","macos":"~/Documents/semio","windows":"C:\\semio"}: failures.append("committed: worktree")
    if committed["storeMount"]!={"linux":"/mnt/semio-store","macos":"/Volumes/semio-store","windows":"S:\\"}: failures.append("committed: storeMount")
    if committed["replica"]!={"pollMilliseconds":250,"scanSeconds":90,"debounceMilliseconds":150,"retentionDays":14,"bootstrapMinutes":60,"compaction":{"automatic":False,"snapshotEvents":100000,"safetyHours":3,"staleHours":12,"catchUpEvents":20000}}: failures.append("committed: replica")
    if [(c["name"],c["arguments"],c["roles"],c["requires"]) for c in committed["sidecars"]["children"]]!=[("build-agent",["agent"],["member","server"],"none"),("build-scheduler",["scheduler"],["server"],"none"),("build-cache",["cache"],["server"],"none")]: failures.append("committed: sidecars")
    for message in problems(fixture["topology"]): failures.append("fixture: "+message)
    for change in fixture["invalid"]:
        if not problems(mutate(fixture["topology"],change)): failures.append("accepted: "+change["name"])
    return failures


def apply_address(topology, environment):
    def present(name):
        value=environment.get(name)
        return value if isinstance(value,str) and value!="" else None
    group_id,address,network=present("SEMIO_FLEET_GROUP"),present("SEMIO_FLEET_STORE_ADDRESS"),present("SEMIO_FLEET_NETWORK")
    if group_id is None and address is None and network is None: return topology
    if group_id is None or not any(isinstance(group,dict) and group.get("id")==group_id for group in topology["groups"]): return topology
    groups=[]
    for group in topology["groups"]:
        if group.get("id")!=group_id:
            groups.append(group); continue
        copied=copy.deepcopy(group)
        if network is not None: copied["server"]["network"]=network
        if address is not None:
            copied["nfs"]["host"]=address
            copied["cache"]["endpoint"]="http://"+(("["+address+"]") if ":" in address else address)+":"+str(group["ports"]["cache"])
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


def worktree_path(topology, platform, home):
    template=topology["worktree"][platform]
    if platform=="windows": return template
    trimmed=home.rstrip("/")
    if not home.startswith("/") or "\0" in home or trimmed=="": raise ValueError("Absolute account home required: "+home)
    return trimmed+template[1:]


def fill(text, home, user, xml):
    def escaped(value): return value.replace("&","&amp;").replace("<","&lt;") if xml else value
    return text.replace("__WORKER_USER__",escaped(user)).replace("__WORKER_HOME__",escaped(home.rstrip("/")))


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
        print(json.dumps({} if not found else {k:v for k,v in {"endpoint":found[0]["cache"].get("endpoint"),"host":found[0]["nfs"].get("host"),"network":found[0]["server"].get("network")}.items() if v is not None},sort_keys=True))
        sys.exit(0)
    if len(sys.argv)==6 and sys.argv[1]=="plist":
        print(plist_environment(Path(sys.argv[2]).read_text(), Path(sys.argv[3]).read_text(), sys.argv[4], sys.argv[5]), end="")
        sys.exit(0)
    if len(sys.argv)==6 and sys.argv[1]=="fill":
        print(fill(Path(sys.argv[2]).read_text(), sys.argv[3], sys.argv[4], sys.argv[5]=="xml"), end="")
        sys.exit(0)
    if len(sys.argv)==5 and sys.argv[1]=="worktree":
        data=load(sys.argv[2])
        try: print(worktree_path(data.get("topology",data), sys.argv[3], sys.argv[4]))
        except ValueError as error:
            print(error, file=sys.stderr); sys.exit(1)
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
    sys.exit(__doc__)

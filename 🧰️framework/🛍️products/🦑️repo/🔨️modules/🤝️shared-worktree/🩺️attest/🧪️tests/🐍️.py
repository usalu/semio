#!/usr/bin/env python3
"""🐍️ Independent oracle of the shared worktree attestation. It shares no code with `../🟦️.ts`: it reads the same language-agnostic fixture, recomputes the manifest digests and the code digests with hashlib, re-reads the `git status --porcelain=v1 -z` vectors into counts, sorted lines and view digests, re-implements the fleet verdict and validates the fixture with the third-party `jsonschema` library when it is installed.
usage: 🐍️.py            exit 0 when every vector agrees, 1 on any mismatch
Python 3.9 compatible (no match statements)."""
import hashlib,importlib.util,json,os,sys,tempfile,unicodedata
HERE=os.path.dirname(os.path.abspath(__file__))
FIXTURE=json.load(open(os.path.join(HERE,"🧫️fixtures","🔣️.json"),encoding="utf8"))
SCHEMA=json.load(open(os.path.join(HERE,"..","🧬️schema","🔣️.json"),encoding="utf8"))
_spec=importlib.util.spec_from_file_location("agreement",os.path.join(os.path.dirname(os.path.abspath(__file__)),"🐍️agreement.py"))
agreement=importlib.util.module_from_spec(_spec);_spec.loader.exec_module(agreement)
bucket_of,manifest_buckets,view_buckets,mask_of,compare,oldest_unseen,recent=agreement.bucket_of,agreement.manifest_buckets,agreement.view_buckets,agreement.mask_of,agreement.compare,agreement.oldest_unseen,agreement.recent
FIELDS=["manifest","head.commit","indexTree","view.digest","guards.pins","guards.infoExclude","guards.caseIgnore","guards.caseMismatch","guards.trackedIgnored","guards.trackedRefused","guards.modeMismatch"]
ALWAYS=("manifest","head.commit","indexTree")

failures=[]

def check(ok,message):
    if not ok: failures.append(message)

def manifest(entries):
    digest=bytearray(32)
    for entry in entries:
        path=entry["path"]
        if entry["operation"]=="delete" or path==".cache/f/index-base" or path.startswith(".cache/f/index/"): continue
        term=hashlib.sha256((path+"\0"+entry["operation"]+"\0"+entry["value"]).encode("utf-8")).digest()
        for index in range(32): digest[index]^=term[index]
    return digest.hex()

def code_of(root):
    found=[]
    for directory,names,files in os.walk(root):
        names[:]=[name for name in names if name!="🧪️tests"]
        for name in files:
            if name in ("🟦️.ts","📜️script.ts"):
                full=os.path.join(directory,name)
                found.append(os.path.relpath(full,root).replace(os.sep,"/"))
    found.sort(key=lambda path:path.encode("utf-8"))
    hasher=hashlib.sha256()
    for path in found:
        data=open(os.path.join(root,*path.split("/")),"rb").read()
        hasher.update(path.encode("utf-8")+b"\0"+str(len(data)).encode()+b"\0"+data)
    return hasher.hexdigest()

def value_of(attestation,field):
    if field=="manifest": return attestation["manifest"]
    if field=="head.commit": return attestation["head"]["commit"] if attestation["head"] else None
    if field=="indexTree": return attestation["indexTree"]
    if field=="view.digest": return attestation["view"]["digest"] if attestation["view"] else None
    guards=attestation["guards"]
    if guards is None: return None
    value=guards[field[7:]]
    return None if value is None else str(value)

def majority(values):
    counts={}
    for value in values: counts[value]=counts.get(value,0)+1
    best=None;most=0
    for value in sorted(counts):
        if counts[value]>most: best=value;most=counts[value]
    return best

def capabilities(live,attestations):
    warnings=[]
    measured=[origin for origin in live if attestations[origin]["guards"] is not None]
    def volume(origin):
        found=attestations[origin]["guards"]["volume"]
        return "case=%s normalisation=%s"%("insensitive" if found["caseInsensitive"] else "sensitive","insensitive" if found["normalisationInsensitive"] else "sensitive")
    usual=majority([attestations[origin]["guards"]["version"] or "-" for origin in measured])
    common=majority([volume(origin) for origin in measured])
    for origin in measured:
        guards=attestations[origin]["guards"]
        if (guards["version"] or "-")!=usual: warnings.append({"originId":origin,"kind":"git-version","detail":guards["version"] or "-"})
        if volume(origin)!=common: warnings.append({"originId":origin,"kind":"volume","detail":volume(origin)})
        if guards["overBudget"]>0: warnings.append({"originId":origin,"kind":"over-budget","detail":str(guards["overBudget"])})
        if not guards["volume"]["symlinks"] and guards["links"]>0: warnings.append({"originId":origin,"kind":"symlinks","detail":str(guards["links"])})
    return warnings


def refused(path):
    """Independent statement of the path policy of the replica (a path it never carries): more than 256 UTF-16 units, a component of more than 255 UTF-8 bytes, a character Windows forbids or a control character, a component that ends with a dot or a space, a Windows device name."""
    if len(path.encode("utf-16-le"))//2>256: return True
    for part in path.split("/"):
        if len(part.encode("utf-8"))>255: return True
        if part and part[-1] in ". ": return True
        stem=part.split(".")[0].rstrip(" ").upper()
        if stem in ("CON","PRN","AUX","NUL","CONIN$","CONOUT$","CLOCK$") or (len(stem)==4 and stem[:3] in ("COM","LPT") and stem[3] in "123456789\u00b9\u00b2\u00b3"): return True
    return any(char in '<>:"|?*\\' or ord(char)<32 or 127<=ord(char)<=159 for char in path)


def view_of(porcelain):
    """Independent reader of `git status --porcelain=v1 -z` output: entries are `XY path`, a rename or copy is followed by one more field."""
    fields=porcelain.split("\0")
    staged=modified=deleted=untracked=0
    lines=[]
    index=0
    while index<len(fields):
        record=fields[index]
        index+=1
        if len(record)<4 or record[2]!=" ": continue
        x,y,path=record[0],record[1],unicodedata.normalize("NFC",record[3:])
        if x in "RC" or y in "RC": index+=1
        if x+y=="!!": continue
        if refused(path):
            if x not in " ?": staged+=1
            continue
        if x+y=="??":
            untracked+=1
            lines.append("?? "+path)
            continue
        if x!=" ": staged+=1
        if y==" ": continue
        if y=="D": deleted+=1
        else: modified+=1
        lines.append(x+y+" "+path)
    lines.sort(key=lambda line:line.encode("utf-8"))
    return {"staged":staged,"modified":modified,"deleted":deleted,"untracked":untracked,"lines":lines,"digest":hashlib.sha256("\n".join(lines).encode("utf-8")).hexdigest()}

def judge(case):
    now,heads,attestations=case["now"],case["heads"],case["attestations"]
    windows=case.get("windows",{})
    retired=set(origin for origin,record in case["retirements"].items() if record["retired"])
    origins=sorted(set(heads)|set(attestations)|retired)
    rows={}
    live=[]
    pointed=[]
    counted=[]
    for origin in origins:
        attestation=attestations.get(origin)
        if attestation is None:
            rows[origin]={"originId":origin,"platform":None,"sync":None,"ageSeconds":None,"behindOrigins":None,"incoming":None,"code":None,"fresh":False,"settled":False,"view":None,"latency":None,"verdict":"retired" if origin in retired else "missing","differs":[]}
            continue
        age=max(0,(now-attestation["wallMs"])//1000)
        behind=sum(1 for other,head in heads.items() if other not in retired and attestation["cursors"].get(other)!=head)
        fresh=(now-attestation["wallMs"])//1000<=60 and (now-attestation["wallMs"])//1000>=-30
        checkpoint="checkpoints" in attestation
        taken=attestation.get("checkpoints",{}).get("manifest")
        if checkpoint:
            unseen=None if taken is None else oldest_unseen(taken["cursors"],dict((other,head) for other,head in heads.items() if other not in retired),windows)
            settled=taken is not None and recent(now,taken["wallMs"]) and unseen!="undefined" and (unseen is None or now-unseen<=60000)
        else:
            settled=behind==0 and attestation["queue"]["incoming"]==0 and not (attestation["view"] or {}).get("stale",False)
        if attestation["sync"] and fresh and settled and origin not in retired:
            live.append(origin)
            (pointed if checkpoint else counted).append(origin)
        verdict="retired" if origin in retired else "off" if not attestation["sync"] else "stale" if not fresh else "lagging" if not settled else "ok"
        rows[origin]={"originId":origin,"platform":attestation["platform"],"sync":attestation["sync"],"ageSeconds":age,"behindOrigins":behind,"incoming":attestation["queue"]["incoming"],"code":attestation["code"][:12],"fresh":fresh,"settled":settled,"view":attestation["view"],"latency":attestation["latency"],"verdict":verdict,"differs":[]}
        if checkpoint:
            rows[origin]["basis"]="checkpoint"
            rows[origin]["checkpointSeconds"]=None if taken is None else max(0,(now-taken["wallMs"])//1000)
    differing=[]
    diverged={}
    for field in FIELDS:
        voters=[origin for origin in counted if field in ALWAYS or value_of(attestations[origin],field) is not None]
        counts={}
        for origin in voters:
            value=value_of(attestations[origin],field)
            counts[value]=counts.get(value,0)+1
        expected=None
        best=-1
        for value in sorted(counts,key=lambda item:"" if item is None else item):
            if counts[value]>best: best=counts[value];expected=value
        for origin in voters:
            value=value_of(attestations[origin],field)
            if value!=expected:
                differing.append({"originId":origin,"field":field,"value":value,"expected":expected})
                diverged.setdefault(origin,[]).append(field)
    known=dict((origin,set()) for origin in pointed)
    keys={}
    for index,one in enumerate(pointed):
        for two in pointed[index+1:]:
            found=compare(attestations[one]["checkpoints"],attestations[two]["checkpoints"],now,windows)
            for track in found["known"]:
                known[one].add(track);known[two].add(track)
            for item in found["items"]:
                rank=agreement.COMPARED.index(item["field"])*1000+item["bucket"]+1
                entry=keys.setdefault(rank,{"field":item["field"],"bucket":item["bucket"],"apart":[],"alike":[]})
                if item["same"]: entry["alike"].append((one,two))
                else: entry["apart"].append((one,two,agreement.spell(item["bucket"],item["left"]),agreement.spell(item["bucket"],item["right"])))
    flagged={}
    ambiguous=set()
    for rank in sorted(keys):
        entry=keys[rank]
        removed=set()
        apart=entry["apart"]
        while apart:
            score={}
            for one,two,_,_ in apart:
                score[one]=score.get(one,0)+1;score[two]=score.get(two,0)+1
            for one,two in entry["alike"]:
                if one in removed or two in removed: continue
                if one in score: score[one]-=1
                if two in score: score[two]-=1
            most=None;tied=[]
            for origin in sorted(score):
                if most is None or score[origin]>most: most=score[origin];tied=[origin]
                elif score[origin]==most: tied.append(origin)
            if len(tied)>1 and len(pointed)>=3:
                for origin in tied: ambiguous.add(origin)
                apart=[item for item in apart if not (item[0] in tied and item[1] in tied)]
                continue
            victim=tied[-1]
            one,two,left,right=[item for item in apart if victim in (item[0],item[1])][0]
            sides=flagged.setdefault(victim,{}).setdefault(entry["field"],{"value":[],"expected":[]})
            sides["value"].append(left if one==victim else right);sides["expected"].append(right if one==victim else left)
            removed.add(victim)
            apart=[item for item in apart if victim not in (item[0],item[1])]
    for origin in sorted(flagged):
        for field,sides in flagged[origin].items():
            differing.append({"originId":origin,"field":field,"value":",".join(sides["value"]),"expected":",".join(sides["expected"])})
            diverged.setdefault(origin,[]).append(field)
    def undecided(origin):
        others=[other for other in pointed if other!=origin]
        if not others: return False
        mine=attestations[origin]["checkpoints"].get("view") is not None and any(attestations[other]["checkpoints"].get("view") is not None for other in others)
        return not ("manifest" in known[origin] and "git" in known[origin] and (not mine or "view" in known[origin]))
    for origin in origins:
        row=rows[origin]
        if origin in diverged:
            row["verdict"]="diverged";row["differs"]=diverged[origin]
        elif row["verdict"]=="ok" and "basis" in row and (origin in ambiguous or undecided(origin)):
            row["verdict"]="lagging"
    ordered=[rows[origin] for origin in origins]
    ok=all(row["verdict"] in ("ok","off","retired") for row in ordered) and any(row["verdict"]=="ok" for row in ordered)
    warnings=capabilities(live,attestations)
    if pointed:
        for origin in origins:
            attestation=attestations.get(origin)
            if attestation is not None and "checkpoints" not in attestation and attestation["sync"] and origin not in retired:
                warnings.append({"originId":origin,"kind":"no-checkpoints","detail":attestation["code"][:12]})
    for origin in sorted(set(case.get("registered",[]))):
        if origin not in heads and origin not in attestations and origin not in retired:
            warnings.append({"originId":origin,"kind":"never-attested","detail":"fleet-auth.json"})
    return {"version":1,"ok":ok,"freshSeconds":60,"rows":ordered,"differing":differing,"warnings":warnings}

try:
    import jsonschema
    validator=jsonschema.Draft7Validator({"$schema":SCHEMA["$schema"],"definitions":SCHEMA["definitions"],"$ref":"#/definitions/Fixture"})
    for error in validator.iter_errors(FIXTURE): failures.append("schema: "+error.message)
    print("jsonschema",jsonschema.__version__ if hasattr(jsonschema,"__version__") else "")
except ImportError:
    print("jsonschema is not installed: schema validation skipped")

for case in FIXTURE["digests"]:
    check(manifest(case["entries"])==case["manifest"],"digest: "+case["name"])
for case in FIXTURE["code"]:
    with tempfile.TemporaryDirectory() as root:
        for file in case["files"]:
            target=os.path.join(root,*file["path"].split("/"))
            os.makedirs(os.path.dirname(target),exist_ok=True)
            open(target,"wb").write(file["text"].encode("utf-8"))
        check(code_of(root)==case["code"],"code: "+case["name"])
for case in FIXTURE["verdicts"]:
    check(judge(case)==case["expected"],"verdict: "+case["name"])

for case in FIXTURE["agreements"]:
    check(judge(case)==case["expected"],"agreement: "+case["name"])
for case in FIXTURE["bucketing"]["paths"]:
    check(bucket_of(case["path"])==case["bucket"],"bucket: "+case["path"])
for case in FIXTURE["bucketing"]["gitPaths"]:
    check(agreement.git_path(case["path"])==case["git"],"git path: "+case["path"])
for case in FIXTURE["bucketing"]["manifests"]:
    check(manifest_buckets(case["entries"])==case["buckets"],"manifest buckets: "+case["name"])
for case in FIXTURE["bucketing"]["views"]:
    check(view_buckets(case["lines"])==case["buckets"] and mask_of(case["pending"])==case["mask"],"view buckets: "+case["name"])

for case in FIXTURE["views"]:
    got=view_of(case["porcelain"])
    check(all(got[key]==case[key] for key in ("staged","modified","deleted","untracked","lines","digest")),"view: "+case["name"])

for message in failures: print("MISMATCH",message)
total=len(FIXTURE["digests"])+len(FIXTURE["code"])+len(FIXTURE["verdicts"])+len(FIXTURE["views"])+len(FIXTURE["agreements"])+len(FIXTURE["bucketing"]["paths"])+len(FIXTURE["bucketing"]["gitPaths"])+len(FIXTURE["bucketing"]["manifests"])+len(FIXTURE["bucketing"]["views"])
print("%d vectors, %d mismatches"%(total,len(failures)))
sys.exit(1 if failures else 0)

#!/usr/bin/env python3
"""🐍️ Independent oracle of the shared worktree resilience rules. It shares no code with `../🟦️.ts`: it reads the same language-agnostic fixture, recomputes the restart jitter with hashlib, the import closure with `re` and `os`, the text catalogue verdicts, the snapshot retention order and the log tail, and validates the fixture with the third-party `jsonschema` library when it is installed.
usage: 🐍️.py            exit 0 when every vector agrees, 1 on any mismatch
Python 3.9 compatible (no match statements)."""
import hashlib,json,os,re,sys,tempfile
HERE=os.path.dirname(os.path.abspath(__file__))
FIXTURE=json.load(open(os.path.join(HERE,"🧫️fixtures","🔣️.json"),encoding="utf8"))
SCHEMA=json.load(open(os.path.join(HERE,"..","🧬️schema","🔣️.json"),encoding="utf8"))
failures=[]

def check(ok,message):
    if not ok: failures.append(message)

def jitter(origin,digest,span):
    if span<=0: return 0
    return int.from_bytes(hashlib.sha256((origin+"\0"+digest).encode("utf-8")).digest()[:4],"big")%span

SPECIFIER=re.compile(r"\bfrom\s*[\"']([^\"']+)[\"']|\bimport\s*\(\s*[\"']([^\"']+)[\"']|\bimport\s*[\"']([^\"']+)[\"']")

def closure(root,entries):
    own=set(entries); seen=set(); extra=[]; queue=[os.path.normpath(os.path.join(root,*e.split("/"))) for e in entries]
    while queue:
        file=queue.pop(0)
        if file in seen: continue
        seen.add(file)
        rel=os.path.relpath(file,root).replace(os.sep,"/")
        if rel not in own: extra.append(rel)
        if not file.endswith((".ts",".js",".mjs")): continue
        text=open(file,encoding="utf8").read()
        for match in SPECIFIER.finditer(text):
            name=match.group(1) or match.group(2) or match.group(3)
            if not name.startswith("."): continue
            target=os.path.normpath(os.path.join(os.path.dirname(file),name))
            if target not in seen and os.path.isfile(target): queue.append(target)
    return sorted(extra,key=lambda p:p.encode("utf-8"))

def problem(catalogue):
    if not isinstance(catalogue,dict): return "catalogue must be an object"
    languages=catalogue.get("languages"); messages=catalogue.get("messages")
    if not isinstance(languages,list) or not languages or any((not isinstance(x,str)) or x=="" for x in languages): return "languages must be a non-empty list of names"
    if len(set(languages))!=len(languages): return "languages must be unique"
    if not isinstance(messages,dict): return "messages must be an object"
    for key,message in messages.items():
        if not isinstance(message,dict): return "message %s must be an object"%key
        if len(message)!=len(languages) or any(l not in message for l in languages): return "message %s must have exactly the languages %s"%(key,",".join(languages))
        reference=None
        for language in languages:
            text=message[language]
            if not isinstance(text,str) or text=="": return "message %s has no text in %s"%(key,language)
            names=",".join(sorted(set(re.findall(r"\{([a-z0-9]+)\}",text))))
            if reference is None: reference=names
            elif reference!=names: return "message %s differs in its placeholders between the languages"%key
    return None

for case in FIXTURE["jitter"]:
    check(jitter(case["originId"],case["digest"],case["span"])==case["expected"],"jitter "+case["originId"])
    check(case["span"]<=0 or 0<=case["expected"]<case["span"],"jitter bound "+case["originId"])

for case in FIXTURE["closure"]:
    with tempfile.TemporaryDirectory() as workspace:
        for file in case["files"]:
            full=os.path.join(workspace,*file["path"].split("/"))
            os.makedirs(os.path.dirname(full),exist_ok=True)
            open(full,"w",encoding="utf8").write(file["text"])
        check(closure(os.path.join(workspace,"mod"),case["entries"])==case["expected"],"closure "+case["name"])

for case in FIXTURE["texts"]:
    check(problem(case["catalogue"])==case["error"],"texts "+case["name"])

for case in FIXTURE["prune"]:
    order=sorted(case["created"].items(),key=lambda item:(-item[1],item[0]))
    check([key for key,_ in order[:case["keep"]]]==case["kept"],"prune "+case["name"])

for case in FIXTURE["rotate"]:
    data=case["text"].encode("utf-8"); rotated=len(data)>case["maxBytes"]
    check(rotated==case["rotated"],"rotate flag "+case["name"])
    check((data[-case["keepBytes"]:].decode("utf-8","replace") if rotated else "")==case["tail"],"rotate tail "+case["name"])

for case in FIXTURE["store"]:
    expected=[{"path":f["path"],"bytes":len(f["text"].encode("utf-8")),"sha256":hashlib.sha256(f["text"].encode("utf-8")).hexdigest(),"executable":f["executable"]} for f in sorted(case["files"],key=lambda item:item["path"].encode("utf-8"))]
    check(expected==case["expected"],"store "+case["name"])

try:
    import jsonschema
    validator=jsonschema.Draft7Validator({"$ref":"#/definitions/Fixture","definitions":SCHEMA["definitions"]})
    for error in validator.iter_errors(FIXTURE): check(False,"schema: "+error.message[:200])
except ImportError:
    pass

if failures:
    print("\n".join(failures)); sys.exit(1)
print("ok: %d jitter, %d closure, %d texts, %d prune, %d rotate, %d store vectors"%(len(FIXTURE["jitter"]),len(FIXTURE["closure"]),len(FIXTURE["texts"]),len(FIXTURE["prune"]),len(FIXTURE["rotate"]),len(FIXTURE["store"])))

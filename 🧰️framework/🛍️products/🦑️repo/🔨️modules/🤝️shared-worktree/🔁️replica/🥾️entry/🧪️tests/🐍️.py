#!/usr/bin/env python3
"""🐍️ Independent oracle of the shared worktree service entry rules. It shares no code with `../🟦️.ts` or `../🟨️.mjs`: it reads the same language-agnostic fixture, recomputes the install directory of every operating system, the sha256 vectors with hashlib, the update decision of every case from the rules (never a downgrade, a damaged installation is repaired, a failing self-test refuses), the static import rule of the trampoline guard, and the installed manifest of the boot program files with hashlib; it validates the fixture with the third-party `jsonschema` library when it is installed.
usage: 🐍️.py            exit 0 when every vector agrees, 1 on any mismatch
Python 3.9 compatible (no match statements)."""
import hashlib,json,os,re,sys
HERE=os.path.dirname(os.path.abspath(__file__))
ENTRY=os.path.dirname(HERE)
FIXTURE=json.load(open(os.path.join(HERE,"🧫️fixtures","🔣️.json"),encoding="utf8"))
SCHEMA=json.load(open(os.path.join(ENTRY,"🧬️schema","🔣️.json"),encoding="utf8"))
failures=[]

def check(ok,message):
    if not ok: failures.append(message)

def directory(platform,home,env):
    if "SEMIO_FLEET_ENTRY_DIR" in env: return env["SEMIO_FLEET_ENTRY_DIR"]
    if platform=="win32": return (env.get("LOCALAPPDATA") or home+"\\AppData\\Local")+"\\semio\\fleet\\entry"
    if platform=="darwin": return home+"/Library/Application Support/semio/fleet/entry"
    return (env.get("XDG_DATA_HOME") or home+"/.local/share")+"/semio/fleet/entry"

def decide(case):
    installed=case["installed"]
    if installed is not None and installed>case["source"]: return "refused-downgrade"
    if installed==case["source"] and not case["damaged"]:
        return "current" if case["identical"] else "refused-same-version"
    if case["selfTestBroken"]: return "refused-selftest"
    return "installed"

IMPORT=re.compile(r"\bfrom\s*[\"']([^\"']+)[\"']|\bimport\s*\(\s*[\"']([^\"']+)[\"']|\bimport\s*[\"']([^\"']+)[\"']|\brequire\s*\(\s*[\"']([^\"']+)[\"']")

def static_ok(source):
    if len(source.encode("utf8"))>16*1024: return False
    for match in IMPORT.finditer(source):
        name=match.group(1) or match.group(2) or match.group(3) or match.group(4)
        dynamic=match.group(2) is not None
        if not name.startswith("node:") and not (dynamic and name.startswith("./")): return False
    return True

for case in FIXTURE["directories"]:
    check(directory(case["platform"],case["home"],case["env"])==case["expected"],"directory "+case["expected"])
for case in FIXTURE["hashes"]:
    check(hashlib.sha256(case["text"].encode("utf8")).hexdigest()==case["sha256"],"hash "+case["text"])
for case in FIXTURE["decisions"]:
    check(decide(case)==case["expected"],"decision "+case["name"])
for case in FIXTURE["guards"]:
    if case["admitted"]: check(static_ok(case["source"]),"guard "+case["name"]+" admitted but breaks the static rule")

trampoline=open(os.path.join(ENTRY,"..","📜️script.ts"),encoding="utf8").read()
check(static_ok(trampoline),"the trampoline breaks the static import rule")
check("--trampoline-self-test" in trampoline,"the trampoline has no self-test")

names=sorted(os.listdir(ENTRY))
sources=[n for n in names if os.path.splitext(n)[1] in (".mjs",".sh",".ps1")]
check(len(sources)==3,"the entry directory must hold exactly one .mjs, .sh and .ps1")
manifest={"boot"+os.path.splitext(n)[1]:{"sha256":hashlib.sha256(open(os.path.join(ENTRY,n),"rb").read()).hexdigest(),"bytes":os.path.getsize(os.path.join(ENTRY,n))} for n in sources}
check(set(manifest)=={"boot.mjs","boot.sh","boot.ps1"},"installed names")
check(all(re.fullmatch("[0-9a-f]{64}",v["sha256"]) for v in manifest.values()),"manifest hashes")

try:
    import jsonschema
    resolver=jsonschema.RefResolver.from_schema(SCHEMA)
    jsonschema.Draft7Validator({"$ref":"#/definitions/Fixture"},resolver=resolver).validate(FIXTURE)
    jsonschema.Draft7Validator({"$ref":"#/definitions/EntryManifest"},resolver=resolver).validate({"version":1,"bootVersion":1,"files":manifest,"worktree":"/w","module":"m","store":None,"installedMs":1})
except ImportError:
    pass
except Exception as error:
    failures.append("schema: %s"%error)

if failures:
    for line in failures: print("FAIL "+line)
    sys.exit(1)
print("entry oracle: all vectors agree")

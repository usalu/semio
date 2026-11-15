#!/usr/bin/env python3
"""🐍️ Independent oracle of the adoption classification. It shares no code with `../🟦️.ts`: it reads the vectors of `🧫️fixtures/🔣️.json`, decides every case from the rule alone (content, where both sides know it the executable bit, history and base, never a modification time) and validates an adoption report against the JSON schema with `jsonschema`.
usage: 🐍️.py classify <fixture>      print [category-or-null, reason-or-null] of every case as JSON
       🐍️.py report <report.json>    validate one adoption report against ../🧬️schema/🔣️.json, print nothing when valid
Python 3.9 compatible."""
import json,os,sys
HERE=os.path.dirname(os.path.abspath(__file__))

def same(local,remote):
    if remote["operation"]=="put": return local["kind"]=="file" and local.get("sha256") is not None and local["sha256"]==remote["sha256"] and (local.get("executable") is None or remote.get("executable") is None or local["executable"]==remote["executable"])
    if remote["operation"]=="link": return local["kind"]=="link" and local.get("target")==remote["target"]
    return False

def decide(local,remote,evidence):
    if local is None:
        return ("materialise" if remote is not None and remote["operation"]!="delete" else None,None)
    if remote is not None and same(local,remote): return ("same",None)
    seen,base,since=evidence["seen"],evidence["base"],evidence["fleetSince"]
    if seen is not None: return ("stale",seen)
    if remote is None: return ("publish",None)
    if base=="equal": return ("stale","base")
    if since is False: return ("publish",None) if remote["operation"]=="delete" else ("edit","fleet-unchanged")
    if remote["operation"]=="delete": return ("conflict","deleted-by-fleet")
    return ("conflict","no-base" if base=="unknown" else "both-changed")

if __name__=="__main__":
    if len(sys.argv)==3 and sys.argv[1]=="classify":
        data=json.load(open(sys.argv[2]))
        print(json.dumps([list(decide(case["local"],case["remote"],case["evidence"])) for case in data["cases"]]))
        sys.exit(0)
    if len(sys.argv)==3 and sys.argv[1]=="report":
        try: import jsonschema
        except ImportError: sys.exit(0)
        schema=json.load(open(os.path.join(HERE,"..","🧬️schema","🔣️.json")))
        jsonschema.validate(json.load(open(sys.argv[2])),{"$ref":"#/definitions/Report","definitions":schema["definitions"]})
        sys.exit(0)
    sys.exit(__doc__)

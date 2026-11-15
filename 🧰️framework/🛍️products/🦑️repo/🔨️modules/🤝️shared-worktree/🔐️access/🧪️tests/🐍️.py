#!/usr/bin/env python3
"""🐍️ Independent oracle of the access record `origins/<origin>/access.json`. It shares no code with `../🟦️.ts`: it validates a record structurally with the standard library and, when it is installed, with `jsonschema` against `../🧬️schema/🔣️.json`.
usage: 🐍️.py record <record.json>    print nothing when the record is valid, one problem per line otherwise
Python 3.9 compatible."""
import json,os,re,sys
HERE=os.path.dirname(os.path.abspath(__file__))
KEYS=["version","originId","state","wallMs","worktree","platform","error"]

def problems(record):
    out=[]
    if not isinstance(record,dict): return ["record must be an object"]
    if sorted(record.keys())!=sorted(KEYS): out.append("record must contain exactly "+",".join(sorted(KEYS)))
    if type(record.get("version")) is not int or record.get("version")!=1: out.append("version must be 1")
    if not isinstance(record.get("originId"),str) or re.fullmatch(r"[a-z0-9][a-z0-9-]{0,39}",record["originId"]) is None: out.append("originId invalid")
    if record.get("state") not in ("denied","waiting-for-consent"): out.append("state invalid")
    if type(record.get("wallMs")) is not int or record["wallMs"]<0: out.append("wallMs invalid")
    for key in ("worktree","platform"):
        if not isinstance(record.get(key),str) or record[key]=="": out.append(key+" invalid")
    if not isinstance(record.get("error"),str): out.append("error invalid")
    return out

if __name__=="__main__":
    if len(sys.argv)==3 and sys.argv[1]=="record":
        record=json.load(open(sys.argv[2]))
        found=problems(record)
        if not found:
            try: import jsonschema
            except ImportError: jsonschema=None
            if jsonschema is not None:
                schema=json.load(open(os.path.join(HERE,"..","🧬️schema","🔣️.json")))
                try: jsonschema.validate(record,{"$ref":"#/definitions/Record","definitions":schema["definitions"]})
                except jsonschema.ValidationError as error: found.append("jsonschema: "+error.message)
        for line in found: print(line)
        sys.exit(1 if found else 0)
    sys.exit(__doc__)

#!/usr/bin/env python3
"""🐍️ Independent oracle of the blob wants record `origins/<origin>/wants.json`. It shares no code with `../🟦️.ts`: it validates a record structurally with the standard library.
usage: 🐍️.py <fixtures.json>    prints one line `<valid|invalid>:<id>:<true|false>` per vector, true when the oracle accepts the record
Python 3.9 compatible."""
import json,re,sys
LIMIT=64

def accepts(record):
    if not isinstance(record,dict) or sorted(record.keys())!=["originId","version","wallMs","wants"]: return False
    if type(record["version"]) is not int or record["version"]!=1: return False
    if not isinstance(record["originId"],str) or re.fullmatch(r"[a-z0-9][a-z0-9-]{0,39}",record["originId"]) is None: return False
    if type(record["wallMs"]) is not int or record["wallMs"]<0: return False
    wants=record["wants"]
    if not isinstance(wants,list) or len(wants)>LIMIT: return False
    for item in wants:
        if not isinstance(item,dict) or sorted(item.keys())!=["path","sha256"]: return False
        if not isinstance(item["sha256"],str) or re.fullmatch(r"[0-9a-f]{64}",item["sha256"]) is None: return False
        if not isinstance(item["path"],str) or not 1<=len(item["path"])<=4096: return False
    return True

if __name__=="__main__":
    data=json.load(open(sys.argv[1],encoding="utf8"))
    for kind in ("valid","invalid"):
        for vector in data[kind]: print("%s:%s:%s"%(kind,vector["id"],"true" if accepts(vector["value"]) else "false"))

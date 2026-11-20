#!/usr/bin/env python3
"""Independent oracle of the refusal classes: reads the schema and the fixture and prints the code and the three flags of every case as JSON. The matching is written again here (first row whose conditions hold) and shares nothing with the TypeScript beyond the authored rows. Usage: oracle.py <schema.json> <fixture.json>"""
import json,sys
schema=json.load(open(sys.argv[1],encoding="utf8"));fixture=json.load(open(sys.argv[2],encoding="utf8"))
policy=schema["definitions"]["Policy"]["const"]
LOCAL={"claim-lost","no-upload","leaf-missing","limit-document","limit-entry","limit-documents"}
def classify(answer,body):
    body=body[:policy["bodyCharacters"]]
    if isinstance(answer,int):kind="status"
    elif answer in LOCAL:kind="local"
    else:kind=answer
    for row in policy["rows"]:
        w=row["when"]
        if "answer" in w:
            if w["answer"]!=kind:continue
            if kind=="local" and w.get("body")!=answer:continue
        elif kind!="status":continue
        if "status" in w:
            if kind!="status":continue
            low=w["status"][0];high=w["status"][1] if len(w["status"])>1 else low
            if not low<=answer<=high:continue
        if "body" in w and kind=="status" and w["body"] not in body:continue
        return {"code":row["code"],"transient":row["transient"],"deterministic":row["deterministic"],"stored":row["stored"]}
    raise SystemExit("no row")
print(json.dumps([classify(c["answer"],c.get("body","")) for c in fixture["cases"]],sort_keys=True))

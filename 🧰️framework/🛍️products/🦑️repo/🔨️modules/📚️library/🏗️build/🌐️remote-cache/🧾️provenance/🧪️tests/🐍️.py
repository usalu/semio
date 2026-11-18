#!/usr/bin/env python3
"""Independent check of the provenance fingerprint: sha256 of the sorted, de-duplicated, newline-joined output digests without the volatile manifest names, read from the shared fixture; prints ok or the first mismatch. usage: oracle.py <fixture.json>"""
import hashlib,json,sys
data=json.load(open(sys.argv[1],encoding="utf8"))
for case in data["cases"]:
    stable=[row["hash"] for row in case["outputs"] if row["path"] not in data["volatile"]]
    got=hashlib.sha256("\n".join(sorted(set(stable))).encode()).hexdigest()
    manifest=any(row["path"] in data["volatile"] for row in case["outputs"])
    if got!=case["fingerprint"] or manifest!=case["manifest"]:
        print("mismatch:",case["name"]);sys.exit(1)
print("ok",len(data["cases"]))

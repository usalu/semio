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
for case in data["classes"]:
    names=data["advisory"]
    hit=next((names[path] for path in case["paths"] if path in names),None)
    sort=hit if hit else "unit" if case["manifest"] else "project-inference" if "record.br" in case["paths"] else "closure" if any(path.startswith("closure-") for path in case["paths"]) else "owner-pool" if any(path in ("head.br","units.br","session.br","pointer.br") for path in case["paths"]) else "other"
    if sort!=case["sort"] or (hit is not None)!=case["advisory"]:
        print("class mismatch:",case["name"]);sys.exit(1)
for case in data["differing"]:
    left={row["path"]:row["hash"] for row in case["kept"]};right={row["path"]:row["hash"] for row in case["refused"]}
    got=sorted(name for name in set(left)|set(right) if left.get(name)!=right.get(name))[:16]
    if got!=case["differing"]:
        print("differing mismatch:",case["name"]);sys.exit(1)
print("ok classes",len(data["classes"]),"differing",len(data["differing"]))

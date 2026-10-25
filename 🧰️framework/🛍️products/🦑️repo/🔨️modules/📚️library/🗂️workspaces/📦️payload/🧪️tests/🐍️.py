#!/usr/bin/env python3
"""🐍️ Independent model of the export-namespace ownership (stdlib only).

usage: 🐍️.py CORPUS.json      (exit 0 when every verdict holds)"""
import json, re, sys

corpus = json.load(open(sys.argv[1], encoding="utf8"))
failures = []
VS = "\ufe0f"


def norm(s):
    return s.replace("\ufe0f", "").replace("\ufe0e", "")



def targets(v, subpaths=True):
    if isinstance(v, str):
        return [v]
    if isinstance(v, list):
        return [t for e in v for t in targets(e, False)]
    if not isinstance(v, dict):
        return []
    items = list(v.items())
    if any(k.startswith(".") for k, _ in items):
        if not subpaths or any(k != "." and (not k.startswith(".") or "*" in k) for k, _ in items):
            return []
        return [t for _, e in items for t in targets(e, False)]
    if any(k == "" or re.match(r"^\d+$", k) for k, _ in items):
        return []
    out = []
    for cond, e in items:
        out += targets(e, False)
        if cond == "default":
            break
    return out


def owns(owner, payload):
    if not owner.get("name") or owner["name"] != payload.get("name"):
        return False
    owner_dir, payload_dir = owner["absDir"], payload["absDir"]
    if not payload_dir.startswith(owner_dir + "/"):
        prefix = "../" * 99 + "/"
    else:
        prefix = payload_dir[len(owner_dir) + 1:] + "/"
    for t in targets(owner.get("exports")):
        if not t.startswith("./") or re.search(r"[\\:*?%#\x00]", t):
            continue
        segs = t[2:].split("/")
        if any(s in ("", ".", "..", "node_modules") for s in segs) or not "/".join(segs).startswith(prefix):
            continue
        return True
    return False


for c in corpus["cases"]:
    if owns(c["owner"], c["payload"]) != c["expected"]:
        failures.append(c["name"])

print(json.dumps({"failures": failures}))
sys.exit(1 if failures else 0)

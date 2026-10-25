#!/usr/bin/env python3
"""🐍️ Independent model of the public runtime export selection (stdlib json/re only).

usage: 🐍️.py CORPUS.json      (exit 0 when every verdict holds)"""
import json, re, sys

corpus = json.load(open(sys.argv[1], encoding="utf8"))
failures = []
VS = "\ufe0f"


def norm(s):
    return s.replace("\ufe0f", "").replace("\ufe0e", "")



def targets(v):
    if isinstance(v, str):
        return [v]
    if isinstance(v, list):
        out = []
        for x in v:
            for t in targets(x):
                if t not in out:
                    out.append(t)
        return out
    if isinstance(v, dict):
        for k, t in v.items():
            if k in ("node", "import", "default"):
                return targets(t)
    return []


def select(m):
    if not isinstance(m, dict):
        raise ValueError("Package manifest required")
    name, surface = m.get("name"), m.get("exports")
    if not isinstance(name, str) or not re.match(r"^(?:@[^/]+/)?[^/]+$", name):
        raise ValueError("Explicit package name required")
    if surface is None:
        return []
    keys = list(surface) if isinstance(surface, dict) else []
    entries = list(surface.items()) if any(k.startswith(".") for k in keys) else [(".", surface)]
    out = []
    for key, value in entries:
        if key != "." and (not key.startswith("./") or any(p in ("", ".", "..") or re.search(r"[*\\:]", p) for p in key[2:].split("/"))):
            raise ValueError("Explicit portable public export key required")
        ts = targets(value)
        if not ts:
            continue
        t = ts[0]
        if not t.startswith("./") or any(p in ("", ".", "..", "node_modules") or re.search(r"[*\\:]", p) for p in t[2:].split("/")):
            raise ValueError("Portable package-relative runtime export required")
        out.append({"specifier": name if key == "." else name + key[1:], "target": t})
    return out


for c in corpus["ok"]:
    if select(c["manifest"]) != c["expected"]:
        failures.append("ok: " + c["name"])
for c in corpus["errors"]:
    try:
        select(c["manifest"])
        failures.append("accepted " + c["name"])
    except ValueError as e:
        if c["message"] not in str(e):
            failures.append("message " + c["name"])

print(json.dumps({"failures": failures}))
sys.exit(1 if failures else 0)

#!/usr/bin/env python3
"""🐍️ Independent model of the allowlist discovery: os.walk over the rebuilt tree with the same skip list (stdlib only).

usage: 🐍️.py CORPUS.json      (exit 0 when every verdict holds)"""
import json, re, sys

corpus = json.load(open(sys.argv[1], encoding="utf8"))
failures = []
VS = "\ufe0f"


def norm(s):
    return s.replace("\ufe0f", "").replace("\ufe0e", "")


import os, tempfile, shutil

root = tempfile.mkdtemp(prefix="allowlist-oracle-")
try:
    for p, t in corpus["files"].items():
        f = os.path.join(root, *p.split("/"))
        os.makedirs(os.path.dirname(f), exist_ok=True)
        open(f, "w", encoding="utf8").write(t)
    skip = set(corpus["skipped"])
    found = {}
    for dp, dn, fn in os.walk(root):
        dn[:] = [d for d in dn if d not in skip]
        if corpus["filename"] in fn:
            try:
                doc = json.load(open(os.path.join(dp, corpus["filename"]), encoding="utf8"))
            except ValueError:
                continue
            for k, v in doc.items():
                if isinstance(v, list):
                    found.setdefault(k, set()).update(x for x in v if isinstance(x, str))
    for k, want in corpus["expected"].items():
        if sorted(found.get(k, [])) != want:
            failures.append("allowlist " + k)
finally:
    shutil.rmtree(root, ignore_errors=True)

print(json.dumps({"failures": failures}))
sys.exit(1 if failures else 0)

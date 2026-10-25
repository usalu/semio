#!/usr/bin/env python3
"""🐍️ Independent model of the workspace discovery: os.walk with the same skip rules and the export-namespace ownership (stdlib only).

usage: 🐍️.py CORPUS.json      (exit 0 when every verdict holds)"""
import json, re, sys

corpus = json.load(open(sys.argv[1], encoding="utf8"))
failures = []
VS = "\ufe0f"


def norm(s):
    return s.replace("\ufe0f", "").replace("\ufe0e", "")


import os, tempfile, shutil

SKIP = {"🗑" + VS + "generated", "node_modules", "target", "dist", "build", "🤖" + VS + "generated", "storybook-static", "temp", "coverage", "🔌" + VS + "plugin-modules", ".🧬semio", "compose"}


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
    for c, e in items:
        out += targets(e, False)
        if c == "default":
            break
    return out


def owns(owner, payload):
    if not owner.get("name") or owner["name"] != payload.get("name"):
        return False
    prefix = os.path.relpath(payload["abs"], owner["abs"]).replace(os.sep, "/") + "/"
    for t in targets(owner.get("exports")):
        if not t.startswith("./") or re.search(r"[\\:*?%#\x00]", t):
            continue
        segs = t[2:].split("/")
        if any(s in ("", ".", "..", "node_modules") for s in segs) or not "/".join(segs).startswith(prefix):
            continue
        return True
    return False


root = tempfile.mkdtemp(prefix="workspaces-oracle-")
try:
    for p, t in corpus["tree"].items():
        f = os.path.join(root, *p.split("/"))
        os.makedirs(os.path.dirname(f), exist_ok=True)
        open(f, "w", encoding="utf8").write(t)
    cands = {}
    for dp, dn, fn in os.walk(root):
        dn[:] = sorted(d for d in dn if not d.startswith(".") and d not in SKIP)
        for d in dn:
            m = os.path.join(dp, d, "package.json")
            if os.path.isfile(m):
                doc = json.load(open(m, encoding="utf8"))
                cands[os.path.join(dp, d)] = {"abs": os.path.join(dp, d), "rel": os.path.relpath(os.path.join(dp, d), root).replace(os.sep, "/"), "name": doc.get("name") if isinstance(doc.get("name"), str) else None, "exports": doc.get("exports")}
    keep = []
    for c in cands.values():
        parent = os.path.dirname(c["abs"])
        ok = True
        while parent != root and parent != os.path.dirname(parent):
            if parent in cands:
                ok = not owns(cands[parent], c)
                break
            parent = os.path.dirname(parent)
        if ok:
            keep.append(c["rel"])
    if sorted(keep) != sorted(corpus["expected"]):
        failures.append("expected %r" % sorted(keep))
    for m in corpus["membership"]:
        cur = m["current"]
        missing = [e for e in corpus["expected"] if e not in set(cur)]
        stale = [e for e in cur if e not in set(corpus["expected"])]
        fresh = cur == corpus["expected"]
        if (missing, stale, fresh) != (m["missing"], m["stale"], m["fresh"]) or (not fresh and not missing and not stale) != m["orderChanged"]:
            failures.append("membership " + m["name"])
finally:
    shutil.rmtree(root, ignore_errors=True)

print(json.dumps({"failures": failures}))
sys.exit(1 if failures else 0)

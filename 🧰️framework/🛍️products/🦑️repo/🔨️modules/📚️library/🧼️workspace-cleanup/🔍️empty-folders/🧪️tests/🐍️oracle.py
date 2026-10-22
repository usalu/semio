#!/usr/bin/env python3
"""🐍️ Independent model of the empty-folder discovery: walks a rebuilt workspace with os.walk and lists directories with no entries (stdlib only).

usage: oracle.py CORPUS.json      (exit 0 when every verdict holds)"""
import json, re, sys

corpus = json.load(open(sys.argv[1], encoding="utf8"))
failures = []
VS = "\ufe0f"


def norm(s):
    return s.replace("\ufe0f", "").replace("\ufe0e", "")


import os, tempfile, shutil

root = tempfile.mkdtemp(prefix="folders-oracle-")
for n in corpus["nodes"]:
    t = os.path.join(root, *n["path"].split("/"))
    if n["kind"] == "directory":
        os.makedirs(t, exist_ok=True)
    else:
        os.makedirs(os.path.dirname(t), exist_ok=True)
        open(t, "w", encoding="utf8").write(n.get("text", ""))
SKIP = {"node_modules", ".git", "⚡️cache"}
skipped = lambda n: n in SKIP or n in ("target", "dist", "build", "out") or n.startswith("target-")
DIFF = "🔺️diff"


def walk(visit):
    stack = [root]
    while stack:
        d = stack.pop()
        for name in os.listdir(d):
            a = os.path.join(d, name)
            if not os.path.isdir(a) or os.path.islink(a):
                continue
            act = visit(a, name)
            if act == "enter":
                stack.append(a)


def under(a, prefixes):
    return any(a == p or a.startswith(p + os.sep) for p in prefixes)


def rel(a):
    return os.path.relpath(a, root).replace(os.sep, "/")


def ordered(paths):
    return sorted(paths, key=lambda p: (-len(p), p))

for c in corpus["empty"]["dirs"]:
    a = os.path.join(root, *c["path"].split("/"))
    if (os.path.isdir(a) and not os.path.islink(a) and os.listdir(a) == []) != c["expected"]:
        failures.append("dir " + c["path"])
for c in corpus["empty"]["collect"]:
    found = []
    prefixes = [os.path.join(root, *p.split("/")) for p in c["prefixes"]]

    def visit(a, name):
        if skipped(name) or under(a, prefixes):
            return "skip"
        if os.listdir(a) == []:
            found.append(rel(a))
        return "enter"
    walk(visit)
    if ordered(found) != c["expected"]:
        failures.append("collect %s" % c["prefixes"])

shutil.rmtree(root, ignore_errors=True)

print(json.dumps({"failures": failures}))
sys.exit(1 if failures else 0)

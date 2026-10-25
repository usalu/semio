#!/usr/bin/env python3
"""🐍️ Independent model of the marker-only folder discovery: derives the marker names from the taxonomy JSON and walks a rebuilt workspace with os.walk (stdlib only).

usage: 🐍️.py CORPUS.json      (exit 0 when every verdict holds)"""
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

tax = json.load(open(os.path.join(os.path.dirname(os.path.abspath(sys.argv[1])), *([".."] * 3), "🔣️taxonomy.json"), encoding="utf8"))
names = {k["emoji"] + k["extensionChains"][0] for k in tax["fileKinds"].values() if k["role"] == "marker"} | {"📝️.md"}
if sorted(names) != sorted(corpus["marker"]["expectedNames"]):
    failures.append("names")


def is_marker(a):
    return os.path.isfile(a) and not os.path.islink(a) and os.path.getsize(a) == 0 and os.path.basename(a) in names


def only_markers(d):
    if os.path.basename(d) == DIFF:
        return False
    saw = False
    for dp, dn, fn in os.walk(d):
        if DIFF in dn or any(os.path.islink(os.path.join(dp, x)) for x in dn + fn):
            return False
        for x in fn:
            saw = True
            if not is_marker(os.path.join(dp, x)):
                return False
    return saw


for c in corpus["marker"]["files"]:
    if is_marker(os.path.join(root, *c["path"].split("/"))) != c["expected"]:
        failures.append("file " + c["path"])
for c in corpus["marker"]["trees"]:
    if only_markers(os.path.join(root, *c["path"].split("/"))) != c["expected"]:
        failures.append("tree " + c["path"])
for c in corpus["marker"]["collect"]:
    found = set()
    prefixes = [os.path.join(root, *p.split("/")) for p in c["prefixes"]]

    def visit(a, name):
        if skipped(name) or under(a, prefixes):
            return "skip"
        if only_markers(a):
            found.add(a)
        return "enter"
    walk(visit)
    tops = [rel(a) for a in found if os.path.dirname(a) not in found]
    if ordered(tops) != c["expected"]:
        failures.append("collect %s" % c["prefixes"])

shutil.rmtree(root, ignore_errors=True)

print(json.dumps({"failures": failures}))
sys.exit(1 if failures else 0)

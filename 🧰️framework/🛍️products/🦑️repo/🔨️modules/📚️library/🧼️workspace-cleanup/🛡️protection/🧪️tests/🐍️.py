#!/usr/bin/env python3
"""🐍️ Independent model of the cleanup protection gate: names, ticket boundaries, manifests, removal protection, open-ticket generated output, incremental-cache selection and deduplication over the corpus node tree (stdlib only; storage and native-image rows depend on the checkout and are skipped).

usage: 🐍️.py CORPUS.json      (exit 0 when every verdict holds)"""
import json, re, sys

corpus = json.load(open(sys.argv[1], encoding="utf8"))
failures = []
VS = "\ufe0f"


def norm(s):
    return s.replace("\ufe0f", "").replace("\ufe0e", "")


import posixpath

NODES = {n["path"]: n for n in corpus["nodes"]}
C = corpus["constants"]
GEN_DIRS = set(C["generated"])
PROBES = tuple(C["probePrefixes"])


def kind(p):
    return "directory" if p in ("", ".") else NODES[p]["kind"] if p in NODES else "missing"


def children(p):
    p = "" if p == "." else p
    n = NODES.get(p)
    if p != "" and (n is None or n["kind"] != "directory" or n.get("childrenUnreadable")):
        return None
    return sorted(q[len(p) + 1:] if p else q for q in NODES if posixpath.dirname(q) == p and q != "")


def read(p):
    return NODES.get(p, {}).get("text")


def is_gen(name):
    return name in GEN_DIRS or name.startswith(PROBES)


def boundary(p):
    segs = [s for s in ("" if p == "." else p).split("/") if s]
    return any(s.endswith("tickets") and len(segs) - i - 1 == 4 for i, s in enumerate(segs))


def closed(p):
    m = (p + "/" if p else "") + C["manifest"]
    if kind(m) != "file":
        return False
    try:
        doc = json.loads(read(m))
    except (ValueError, TypeError):
        return False
    return isinstance(doc, dict) and doc.get("status") == "closed"


def ticket_for(p):
    a = posixpath.dirname(p)
    while True:
        if a == "" or a.startswith(".."):
            return None
        if boundary(a):
            return a
        parent = posixpath.dirname(a)
        if parent == a:
            return None
        a = parent


def protection(candidate, allowed=None):
    target = posixpath.normpath(candidate)
    if target == "." or target.startswith(".."):
        return [target]
    prot, closed_set = set(), set()
    chain = [("", "directory")]

    def inspect(d):
        if not boundary(d) and kind((d + "/" if d else "") + C["manifest"]) == "missing":
            return
        if closed(d):
            closed_set.add(d)
        elif d != allowed:
            prot.add(d)
    inspect("")
    anc = ""
    for seg in target.split("/"):
        anc = anc + "/" + seg if anc else seg
        k = kind(anc)
        if k in ("symlink", "missing", "unreadable") or (anc != target and k != "directory"):
            return [anc]
        chain.append((anc, k))
        if k == "directory":
            inspect(anc)
    if prot:
        return sorted(prot)
    stack = [target] if kind(target) == "directory" else []
    while stack:
        d = stack.pop()
        inspect(d)
        if d in prot:
            continue
        names = children(d)
        if names is None:
            prot.add(d)
            continue
        for name in names:
            child = d + "/" + name
            k = kind(child)
            if k == "directory":
                stack.append(child)
            elif k != "file":
                prot.add(child)
    for p, k in chain:
        if kind(p) != k:
            prot.add(p)
    for d in closed_set:
        if kind(d) != "directory" or not closed(d):
            prot.add(d)
    return sorted(prot)


def generated(candidate):
    target = posixpath.normpath(candidate)
    ticket = ticket_for(target)
    if ticket is not None and not closed(ticket):
        inside = posixpath.relpath(target, ticket).split("/")
        if any(is_gen(s) for s in inside):
            return [target]
    if kind(target) != "directory":
        return []
    stack = [target]
    while stack:
        d = stack.pop()
        names = children(d)
        if names is None:
            return [d]
        for name in names:
            child = d + "/" + name
            if kind(child) != "directory":
                continue
            if is_gen(name):
                owner = ticket_for(child)
                if owner is not None and not closed(owner):
                    return [child]
                continue
            stack.append(child)
    return []


for c in corpus["protection"]:
    if not c["environment"] and protection(c["candidate"], c["allowed"]) != sorted(c["expected"]):
        failures.append("protection: " + c["name"])
for c in corpus["generated"]:
    if generated(c["candidate"]) != sorted(c["expected"]):
        failures.append("generated: " + c["name"])
for c in corpus["boundary"]:
    if boundary(c["path"]) != c["expected"]:
        failures.append("boundary " + c["path"])
for c in corpus["folderFor"]:
    if ticket_for(posixpath.normpath(c["path"])) != c["expected"]:
        failures.append("folderFor " + c["path"])
for c in corpus["generatedRoot"]:
    p = posixpath.normpath(c["path"])
    got = ticket_for(p) if is_gen(posixpath.basename(p)) else None
    if got != c["expected"]:
        failures.append("generatedRoot " + c["path"])
for c in corpus["closed"]:
    if closed(c["path"]) != c["expected"]:
        failures.append("closed " + c["path"])
for c in corpus["dedupe"]:
    rows = sorted((r["path"] for r in c["rows"]), key=lambda p: (len(p), p))
    kept = []
    for p in rows:
        if not any(p == k or p.startswith(k + "/") for k in kept):
            kept.append(p)
    if sorted(kept) != sorted(c["expected"]):
        failures.append("dedupe")


def huge(d, limit):
    if limit < 0 or limit != int(limit):
        raise ValueError("limit")
    base = d["path"].replace("\\", "/").rstrip("/")
    if not base or any(s in ("", ".", "..") for s in base.split("/")):
        raise ValueError("path")
    for cr in d["crates"]:
        if not cr["name"] or "/" in cr["name"] or "\\" in cr["name"] or cr["name"] in (".", "..") or cr["bytes"] < 0:
            raise ValueError("crate")
    total = sum(cr["bytes"] for cr in d["crates"])
    if total > 9007199254740991:
        raise ValueError("sum")
    if total <= limit:
        return []
    idle = [cr for cr in d["crates"] if not cr["working"]]
    if not idle:
        return []
    if len(idle) == len(d["crates"]):
        return [base]
    return sorted(base + "/" + cr["name"] for cr in idle)


for c in corpus["huge"]:
    if huge(c["dir"], c["max"]) != c["expected"]:
        failures.append("huge: " + c["name"])
for c in corpus["hugeErrors"]:
    try:
        huge(c["dir"], c["max"])
        failures.append("huge error: " + c["name"])
    except ValueError:
        pass
for c in corpus["incrementalPaths"]:
    p = posixpath.normpath(c["path"])
    parts = p.split("/")
    ok = False
    if not (p == "." or p.startswith("..")) and "incremental" in parts:
        at = len(parts) - 1 - parts[::-1].index("incremental")
        ok = len(parts) in (at + 1, at + 2) and any(p == posixpath.normpath(r) or p.startswith(posixpath.normpath(r) + "/") for r in c["roots"])
    if ok != c["expected"]:
        failures.append("incremental path " + c["path"])
N = corpus["names"]
RESERVED = re.compile(r"^(con|prn|aux|nul|com[1-9¹²³]|lpt[1-9¹²³])(\..*)?$", re.I)


def illegal(n):
    if not n:
        return True
    if n in (".", ".."):
        return False
    return n.strip() == "" or n.endswith(" ") or n.endswith(".") or n.startswith(" ") or re.search(r'[<>:"|?*\x00-\x1f]', n) is not None or RESERVED.match(n) is not None


predicates = {
    "misplacedRepo": lambda n: n.endswith("repo") and n != C["repoDir"],
    "misplacedTickets": lambda n: n.endswith("tickets") and n != C["ticketsDir"],
    "buildArtifact": lambda n: n in C["buildNames"] or n == "target" or n.startswith("target-"),
    "cargoTargetName": lambda n: n == "target" or n.startswith("target-"),
    "semioRoot": lambda n: n == ".🧬semio" or (n.startswith(".🧬") and n.endswith("semio")),
    "rootTransient": lambda n: re.match(r"^\.generation3d-.+-link$", n) is not None or n == ".w-g3-ticket",
    "windowsIllegal": illegal,
    "generatedDir": is_gen,
}
for key, fn in predicates.items():
    for row in N[key]:
        if fn(norm(row["name"]) if key == "semioRoot" and False else row["name"]) != row["expected"]:
            failures.append("%s %r" % (key, row["name"]))
for row in N["endsWithAscii"]:
    if row["name"].endswith(row["suffix"]) != row["expected"]:
        failures.append("endsWith %r" % row["name"])

print(json.dumps({"failures": failures}))
sys.exit(1 if failures else 0)

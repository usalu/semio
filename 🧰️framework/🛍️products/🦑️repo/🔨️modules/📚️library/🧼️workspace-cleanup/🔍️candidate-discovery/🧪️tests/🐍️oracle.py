#!/usr/bin/env python3
"""🐍️ Independent model of the cleanup candidate discovery: rebuilds the corpus workspace (sparse files via truncate) in a scratch directory and recomputes the filesystem-only verdicts with os.walk (stdlib only).

usage: oracle.py CORPUS.json      (exit 0 when every verdict holds)"""
import json, re, sys

corpus = json.load(open(sys.argv[1], encoding="utf8"))
failures = []
VS = "\ufe0f"


def norm(s):
    return s.replace("\ufe0f", "").replace("\ufe0e", "")


import os, tempfile, shutil

root = tempfile.mkdtemp(prefix="discovery-oracle-")
MB = 1024 * 1024
M = next(n["path"] for n in corpus["nodes"] if n["path"].endswith("ticket.json")).rsplit("/", 1)[1]
for n in corpus["nodes"]:
    target = os.path.join(root, *n["path"].split("/"))
    if n["kind"] == "directory":
        os.makedirs(target, exist_ok=True)
        continue
    os.makedirs(os.path.dirname(target), exist_ok=True)
    with open(target, "wb") as fh:
        if "size" in n:
            fh.truncate(n["size"])
        else:
            fh.write(n.get("text", "").encode())


def rel(p):
    return os.path.relpath(p, root).replace(os.sep, "/")


def path_bytes(p):
    if os.path.islink(p) or os.path.isfile(p):
        return os.lstat(p).st_size
    return sum(path_bytes(os.path.join(p, c)) for c in os.listdir(p)) if os.path.isdir(p) else 0


def under(p, prefixes):
    return any(p == q or p.startswith(q + os.sep) for q in prefixes)


def intersects(p, prefixes):
    return any(p == q or p.startswith(q + os.sep) or q.startswith(p + os.sep) for q in prefixes)


def absl(lst):
    return [os.path.join(root, *p.split("/")) for p in lst]


def walk_dirs(top, visit):
    stack = [top]
    while stack:
        d = stack.pop()
        try:
            names = os.listdir(d)
        except OSError:
            continue
        for name in names:
            a = os.path.join(d, name)
            if not os.path.isdir(a) or os.path.islink(a):
                continue
            act = visit(a, name)
            if act == "stop":
                return
            if act == "enter":
                stack.append(a)


REPO = corpus["nodes"][0]["path"].split("/")[1]
SEMIO_ROOT = lambda n: n == ".🧬semio" or (n.startswith(".🧬") and n.endswith("semio"))
BUILD = lambda n: n in ("target", "dist", "build", "out") or n == "target" or n.startswith("target-")
SKIP = {"node_modules", ".git", "⚡️cache"}
TICKETS = "🎫️tickets"


def misplaced(prefixes):
    out = {}

    def push(a):
        if not intersects(a, prefixes):
            out[rel(a)] = {"kind": "misplaced", "path": rel(a), "bytes": path_bytes(a)}
    if os.path.exists(os.path.join(root, REPO)):
        push(os.path.join(root, REPO))

    def visit(a, name):
        if name in SKIP or BUILD(name):
            return "skip"
        if SEMIO_ROOT(name):
            for c in os.listdir(a):
                ca = os.path.join(a, c)
                if not os.path.isdir(ca):
                    continue
                if c.endswith("repo") and c != REPO:
                    push(ca)
                if c == REPO:
                    for t in os.listdir(ca):
                        if t.endswith("tickets") and t != TICKETS and os.path.isdir(os.path.join(ca, t)):
                            push(os.path.join(ca, t))
            return "skip"
        return "enter"
    walk_dirs(root, visit)
    return sorted(out.values(), key=lambda r: r["path"])


for key, c in corpus["misplaced"].items():
    if misplaced(absl(c["prefixes"])) != sorted(c["expected"], key=lambda r: r["path"]):
        failures.append("misplaced " + key)
t = corpus["transient"]
for prefixes, expected in ((t["prefixes"], t["expected"]), (t["protected"], t["expectedProtected"])):
    got = [{"kind": "root-transient", "path": n, "bytes": path_bytes(os.path.join(root, n))} for n in sorted(os.listdir(root)) if (re.match(r"^\.generation3d-.+-link$", n) or n == ".w-g3-ticket") and not intersects(os.path.join(root, n), absl(prefixes))]
    if got != sorted(expected, key=lambda r: r["path"]):
        failures.append("transient")
roots_found = []


def find_roots(a, name):
    if name in SKIP or BUILD(name):
        return "skip"
    if name == TICKETS:
        roots_found.append(rel(a))
        return "skip"
    return "enter"


walk_dirs(root, find_roots)
if sorted(roots_found) != sorted(corpus["ticketRoots"]):
    failures.append("ticket roots")
troot = os.path.join(root, *corpus["ticketRoots"][0].split("/"))
folders = []
for y in os.listdir(troot):
    ya = os.path.join(troot, y)
    for m_ in (os.listdir(ya) if os.path.isdir(ya) else []):
        ma = os.path.join(ya, m_)
        for dd in (os.listdir(ma) if os.path.isdir(ma) else []):
            da = os.path.join(ma, dd)
            for slug in (os.listdir(da) if os.path.isdir(da) else []):
                if os.path.isdir(os.path.join(da, slug)):
                    folders.append(rel(os.path.join(da, slug)))
if sorted(folders) != sorted(corpus["ticketFolders"]):
    failures.append("ticket folders")


def closed(folder):
    try:
        doc = json.load(open(os.path.join(folder, M), encoding="utf8"))
    except (OSError, ValueError):
        return False
    return isinstance(doc, dict) and doc.get("status") == "closed"


def size_removals(folder, prefixes):
    out = []

    def node(a):
        if under(a, prefixes):
            return None
        if os.path.islink(a) or os.path.isfile(a):
            return (a, False, os.lstat(a).st_size, [])
        kids = [k for k in (node(os.path.join(a, n)) for n in sorted(os.listdir(a))) if k]
        return (a, True, sum(k[2] for k in kids), kids)

    def visit(n):
        for k in n[3]:
            visit(k)
        if not n[1] and n[2] > corpus["thresholds"]["ticketFile"]:
            out.append({"kind": "ticket-file", "path": rel(n[0]), "bytes": n[2]})
        elif n[1] and n[2] > corpus["thresholds"]["ticketDir"]:
            out.append({"kind": "ticket-dir", "path": rel(n[0]), "bytes": n[2]})
    tree = node(folder)
    for k in (tree[3] if tree else []):
        visit(k)
    return sorted(out, key=lambda r: r["path"])


for c in corpus["sizeRemovals"]:
    if size_removals(os.path.join(root, *c["folder"].split("/")), absl(c["prefixes"])) != sorted(c["expected"], key=lambda r: r["path"]):
        failures.append("size removals " + c["folder"])


def gen_removals(folder, prefixes):
    out = []
    GEN = set(corpus_generated)

    def visit(a, name):
        if name not in GEN and not name.startswith(("🧪️purity-", "🧪️cli-", "🧪️inventory-")):
            return "enter"
        if intersects(a, [p for p in prefixes if p != folder]) or not closed(folder):
            return "skip"
        out.append({"kind": "ticket-generated", "path": rel(a), "bytes": path_bytes(a)})
        return "skip"
    walk_dirs(folder, visit)
    return sorted(out, key=lambda r: r["path"])


corpus_generated = ["🗑️generated", "🗑️temp", "🧾️runs", "🧪️runs", "🧾️taxonomy-transaction"]
for c in corpus["generatedRemovals"]:
    if gen_removals(os.path.join(root, *c["folder"].split("/")), absl(c["prefixes"])) != sorted(c["expected"], key=lambda r: r["path"]):
        failures.append("generated removals " + c["folder"])


def open_oversized(folder, prefixes):
    if closed(folder):
        return []
    out = []
    for dp, dn, fn in os.walk(folder):
        dn[:] = [d for d in dn if not intersects(os.path.join(dp, d), [p for p in prefixes if p != folder])]
        for name in fn:
            a = os.path.join(dp, name)
            if intersects(a, [p for p in prefixes if p != folder]) or os.path.islink(a):
                continue
            if os.lstat(a).st_size > corpus["thresholds"]["openTicketFile"]:
                out.append({"kind": "ticket-generated", "path": rel(a), "bytes": os.lstat(a).st_size})
    return sorted(out, key=lambda r: r["path"])


for c in corpus["openOversized"]:
    if open_oversized(os.path.join(root, *c["folder"].split("/")), absl(c["prefixes"])) != sorted(c["expected"], key=lambda r: r["path"]):
        failures.append("open oversized " + c["folder"])


def build(prefixes):
    out = []

    def visit(a, name):
        if name in SKIP or name == TICKETS or under(a, prefixes):
            return "skip"
        if BUILD(name):
            if intersects(a, prefixes):
                return "skip"
            b = path_bytes(a)
            if os.path.exists(os.path.join(a, "CACHEDIR.TAG")) and (name == "target" or name.startswith("target-")) or b > corpus["thresholds"]["buildArtifact"]:
                out.append({"kind": "build-artifact", "path": rel(a), "bytes": b})
            return "skip"
        return "enter"
    walk_dirs(root, visit)
    return sorted(out, key=lambda r: r["path"])


b = corpus["build"]
if build([]) != sorted(b["expected"], key=lambda r: r["path"]) or build(absl(b["protected"])) != sorted(b["expectedProtected"], key=lambda r: r["path"]):
    failures.append("build")
for c in corpus["dedupe"]:
    rows = sorted((r["path"] for r in c["rows"]), key=lambda p: (-len(p), p))
    kept = []
    for p in rows:
        if not any(k == p or k.startswith(p + "/") for k in kept):
            kept.append(p)
    if sorted(kept) != sorted(c["expected"]):
        failures.append("dedupe")
shutil.rmtree(root, ignore_errors=True)

print(json.dumps({"failures": failures}))
sys.exit(1 if failures else 0)

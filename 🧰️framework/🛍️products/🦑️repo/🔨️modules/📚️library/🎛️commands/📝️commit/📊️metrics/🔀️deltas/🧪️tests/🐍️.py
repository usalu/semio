#!/usr/bin/env python3
"""🐍️ Independent model of the commit metric deltas: replays the corpus commits in a scratch repository with the git CLI and recomputes numstat, line and byte deltas (stdlib + git).

usage: 🐍️.py CORPUS.json      (exit 0 when every verdict holds)"""
import json, re, sys

corpus = json.load(open(sys.argv[1], encoding="utf8"))
failures = []
VS = "\ufe0f"


def norm(s):
    return s.replace("\ufe0f", "").replace("\ufe0e", "")


import os, subprocess, tempfile, shutil

SKIP = (".🧬semio", "node_modules", "dist", "build", "target", ".git", ".nx", "coverage", ".cache", ".turbo", ".next", "out", "vendor", "third_party", "Carthage")
LANG = {".ts": "TypeScript", ".py": "Python", ".go": "Go", ".md": "Markdown"}


def split(a, r):
    a, r = max(0, a), max(0, r)
    e = min(a, r)
    return {"added": a - e, "removed": r - e, "edited": e}


for c in corpus["split"]:
    if split(c["added"], c["removed"]) != {"added": c["netAdded"], "removed": c["netRemoved"], "edited": c["edited"]}:
        failures.append("split %s/%s" % (c["added"], c["removed"]))


def under(path, prefixes):
    if not prefixes:
        return True
    n = path.replace("\\", "/")
    n = n[2:] if n.startswith("./") else n
    for raw in prefixes:
        p = raw.replace("\\", "/")
        p = p[2:] if p.startswith("./") else p
        p = p.rstrip("/")
        if p and (n == p or n.startswith(p + "/")):
            return True
    return False


for c in corpus["prefix"]:
    if under(c["path"], c["prefixes"]) != c["expected"]:
        failures.append("prefix %s" % c["path"])


def lang(path):
    return LANG.get(os.path.splitext(path)[1], "")


def skipped(path):
    return any(path == d or path.startswith(d + "/") for d in SKIP) or any(s.startswith(".") for s in path.split("/"))


def accumulate(rows, prefixes=None):
    out = {}
    for r in rows:
        if skipped(r["path"]) or (prefixes and not under(r["path"], prefixes)) or not lang(r["path"]):
            continue
        d = out.setdefault(lang(r["path"]), {"added": 0, "removed": 0, "edited": 0})
        for k, v in split(r["added"], r["removed"]).items():
            d[k] += v
    return out


acc = corpus["accumulate"]
if accumulate(acc["rows"]) != acc["expected"]:
    failures.append("accumulate")
if accumulate(acc["rows"], acc["prefix"]) != acc["expectedPrefixed"]:
    failures.append("accumulate prefixed")


def git(cwd, *args):
    env = dict(os.environ, GIT_CONFIG_GLOBAL="/dev/null", GIT_CONFIG_NOSYSTEM="1")
    return subprocess.run(["git", "-c", "commit.gpgsign=false", "-c", "core.hooksPath=/dev/null", "-c", "user.name=t", "-c", "user.email=t@example.com"] + list(args), cwd=cwd, env=env, check=True, capture_output=True, text=True).stdout


root = tempfile.mkdtemp(prefix="deltas-oracle-")
try:
    git(root, "init", "-q")
    revs = []
    for files in corpus["repo"]["commits"]:
        for path, text in files.items():
            full = os.path.join(root, path)
            if text is None:
                os.remove(full)
            else:
                os.makedirs(os.path.dirname(full), exist_ok=True)
                open(full, "w").write(text)
        git(root, "add", "-A")
        git(root, "commit", "-q", "-m", "c")
        revs.append(git(root, "rev-parse", "HEAD").strip())
    rows = []
    for line in git(root, "diff", "--numstat", revs[0] + ".." + revs[1]).splitlines():
        a, r, p = line.split("\t")
        rows.append({"path": p, "added": int(a), "removed": int(r)})
    if rows != corpus["repo"]["numstat"]:
        failures.append("numstat")
    if accumulate(rows) != corpus["repo"]["uloc"]:
        failures.append("uloc")
    if accumulate(rows, corpus["repo"]["prefixes"]) != corpus["repo"]["ulocPrefixed"]:
        failures.append("uloc prefixed")

    def blob(rev, path):
        r = subprocess.run(["git", "cat-file", "-s", rev + ":" + path], cwd=root, capture_output=True, text=True)
        return int(r.stdout) if r.returncode == 0 else 0

    sizes = {}
    for r in rows:
        if skipped(r["path"]) or not lang(r["path"]):
            continue
        d = sizes.setdefault(lang(r["path"]), {"added": 0, "removed": 0, "edited": 0})
        for k, v in split(blob(revs[1], r["path"]), blob(revs[0], r["path"])).items():
            d[k] += v
    if sizes != corpus["repo"]["size"]:
        failures.append("size")
finally:
    shutil.rmtree(root, ignore_errors=True)

print(json.dumps({"failures": failures}))
sys.exit(1 if failures else 0)

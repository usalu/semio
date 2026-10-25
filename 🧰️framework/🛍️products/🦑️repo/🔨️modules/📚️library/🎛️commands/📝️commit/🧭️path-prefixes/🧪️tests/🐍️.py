#!/usr/bin/env python3
"""🐍️ Independent model of the bundle path prefixes: replays the commits with the git CLI and assigns changed paths by label tokens, then derives the token-carrying parent prefixes (stdlib + git).

usage: 🐍️.py CORPUS.json      (exit 0 when every verdict holds)"""
import json, re, sys

corpus = json.load(open(sys.argv[1], encoding="utf8"))
failures = []
VS = "\ufe0f"


def norm(s):
    return s.replace("\ufe0f", "").replace("\ufe0e", "")


import os, subprocess, tempfile, shutil

EMOJI = lambda c: 0x1F000 <= ord(c) <= 0x1FAFF or 0x2600 <= ord(c) <= 0x27BF or 0x2B00 <= ord(c) <= 0x2BFF


def tokens(label):
    t = "".join(" " if EMOJI(c) else c for c in label.strip()).lower()
    return [x for x in re.split(r"[^a-z0-9]+", t) if len(x) >= 2 and x not in {"uloc", "repo", "the", "and"}]


def assign(path, bundles):
    best, score = -1, 0
    for i, b in enumerate(bundles):
        tk = tokens(b["label"])
        if tk and all(t in path.lower() for t in tk) and len(tk) > score:
            best, score = i, len(tk)
    return best


def common_prefix(paths):
    if not paths:
        return ""
    parts = [p.split("/") for p in paths]
    n = 0
    for i, seg in enumerate(parts[0]):
        if all(len(q) > i and q[i] == seg for q in parts):
            n = i + 1
        else:
            break
    return paths[0].split("/")[0] if n == 0 else "/".join(parts[0][:n])


def git(cwd, *a):
    env = dict(os.environ, GIT_CONFIG_GLOBAL="/dev/null", GIT_CONFIG_NOSYSTEM="1")
    return subprocess.run(["git", "-c", "commit.gpgsign=false", "-c", "core.hooksPath=/dev/null", "-c", "user.name=t", "-c", "user.email=t@e.x"] + list(a), cwd=cwd, env=env, check=True, capture_output=True, text=True).stdout.strip()


root = tempfile.mkdtemp(prefix="prefix-oracle-")
try:
    git(root, "init", "-q")

    def put(files):
        for p, t in files.items():
            f = os.path.join(root, *p.split("/"))
            os.makedirs(os.path.dirname(f), exist_ok=True)
            open(f, "w").write(t)
    put(corpus["repo"]["base"])
    git(root, "add", "-A")
    git(root, "commit", "-q", "-m", "base")
    base = git(root, "rev-parse", "HEAD")
    for c in corpus["repo"]["commits"]:
        put(c["files"])
        git(root, "add", "-A")
        git(root, "commit", "-q", "-m", c["subject"], "-m", c["body"])
    head = git(root, "rev-parse", "HEAD")
    changed = [p for p in git(root, "diff", "--name-only", base + ".." + head).split("\n") if p]
    bundles = corpus["bundles"]
    assigned = [[] for _ in bundles]
    for p in changed:
        i = assign(p, bundles)
        if i >= 0:
            assigned[i].append(p)
    if assigned != corpus["expected"]["assigned"]:
        failures.append("assigned")
    sets = []
    for i, b in enumerate(bundles):
        tk, s = tokens(b["label"]), []
        for p in assigned[i]:
            if p not in s:
                s.append(p)
            acc = ""
            for seg in p.split("/"):
                acc = acc + "/" + seg if acc else seg
                if tk and all(t in acc.lower() for t in tk) and acc not in s:
                    s.append(acc)
        sets.append(s)
    if sets != corpus["expected"]["prefixSets"]:
        failures.append("prefix sets")
    for c in corpus["expected"]["infer"]:
        tk = tokens(c["label"])
        pool = c["paths"] if c["paths"] is not None else changed
        matched = [p for p in pool if tk and all(t in p.lower() for t in tk)]
        got = [common_prefix(matched)] if matched and common_prefix(matched) else []
        if got != c["expected"]:
            failures.append("infer %s" % c["label"])
finally:
    shutil.rmtree(root, ignore_errors=True)


def owners(field, sets_, bundles_):
    out = set()
    paths = []
    for part in [p.strip() for p in field.split("\t") if p.strip()]:
        m = re.match(r"^(.*)\{(.+?)\s*=>\s*(.+?)\}(.*)$", part)
        if m:
            for side in (m.group(2).strip(), m.group(3).strip()):
                if side:
                    paths.append(m.group(1) + side + m.group(4))
        elif re.search(r"\s*=>\s*", part):
            paths += [x.strip() for x in re.split(r"\s*=>\s*", part) if x.strip()]
        else:
            paths.append(part)
    for p in paths:
        for i in range(len(bundles_)):
            pre = sets_[i] if i < len(sets_) else []
            if pre:
                hit = any(p == q.rstrip("/") or p.startswith(q.rstrip("/") + "/") for q in pre if q.rstrip("/"))
            else:
                hit = assign(p, bundles_) == i
            if hit:
                out.add(i)
    return sorted(out)


for c in corpus["rowMatch"]:
    if owners(c["field"], corpus["expected"]["prefixSets"], bundles) != c["expected"]:
        failures.append("row %r" % c["field"])
for c in corpus["pathMatch"]:
    pre = c["prefixSets"][c["index"]]
    hit = any(c["path"] == q or c["path"].startswith(q + "/") for q in pre) if pre else assign(c["path"], bundles) == c["index"]
    if hit != c["expected"]:
        failures.append("path match")

print(json.dumps({"failures": failures}))
sys.exit(1 if failures else 0)

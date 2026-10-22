#!/usr/bin/env python3
"""🐍️ Independent model of the bundle names and the newest-squash search, replaying the commits with the git CLI (stdlib + git).

usage: oracle.py CORPUS.json      (exit 0 when every verdict holds)"""
import json, re, sys

corpus = json.load(open(sys.argv[1], encoding="utf8"))
failures = []
VS = "\ufe0f"


def norm(s):
    return s.replace("\ufe0f", "").replace("\ufe0e", "")


import os, subprocess, tempfile, shutil

who = corpus["contributor"]
for c in corpus["names"]:
    y, m, d = c["now"]
    base = "%s%s🎆%s%02d🌙%s%02d☀%s%02d" % (who["emoji"], who["alias"], VS, y % 100, VS, m, VS, d)
    if base + "🚩" + VS != c["expectedTag"] or base + "🔀" + VS != c["expectedSubject"]:
        failures.append("names %r" % c["now"])


def git(cwd, *a):
    env = dict(os.environ, GIT_CONFIG_GLOBAL="/dev/null", GIT_CONFIG_NOSYSTEM="1")
    return subprocess.run(["git", "-c", "commit.gpgsign=false", "-c", "core.hooksPath=/dev/null", "-c", "user.name=t", "-c", "user.email=t@e.x"] + list(a), cwd=cwd, env=env, check=True, capture_output=True, text=True).stdout.strip()


root = tempfile.mkdtemp(prefix="tags-oracle-")
try:
    git(root, "init", "-q")
    for s in corpus["repo"]["commits"]:
        git(root, "commit", "-q", "--allow-empty", "-m", s)
    pattern = re.compile("^(.+🎆" + VS + r"\d{2}🌙" + VS + r"\d{2}☀" + VS + r"\d{2})🔀" + VS + "$")
    rows = git(root, "log", "--format=%H%x09%s").split("\n")
    first = next((r.split("\t") for r in rows if pattern.match(r.split("\t", 1)[1])), None)
    if first is None or first[1] != corpus["repo"]["commits"][corpus["repo"]["expectedIndex"]]:
        failures.append("newest squash")
finally:
    shutil.rmtree(root, ignore_errors=True)

print(json.dumps({"failures": failures}))
sys.exit(1 if failures else 0)

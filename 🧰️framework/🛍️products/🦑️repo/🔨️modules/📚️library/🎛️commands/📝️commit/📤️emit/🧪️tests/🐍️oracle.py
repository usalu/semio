#!/usr/bin/env python3
"""🐍️ Independent model of the commit history comparison: replays the commits with the git CLI and normalises the bodies (stdlib + git).

usage: oracle.py CORPUS.json      (exit 0 when every verdict holds)"""
import json, re, sys

corpus = json.load(open(sys.argv[1], encoding="utf8"))
failures = []
VS = "\ufe0f"


def norm(s):
    return s.replace("\ufe0f", "").replace("\ufe0e", "")


import os, subprocess, tempfile, shutil

DAY = "🎆" + VS + r"\d{2}🌙" + VS + r"\d{2}☀" + VS + r"\d{2}"
WIP = re.compile("^(.+" + DAY + ")🔀" + VS + "$")


def git(cwd, *a):
    env = dict(os.environ, GIT_CONFIG_GLOBAL="/dev/null", GIT_CONFIG_NOSYSTEM="1")
    return subprocess.run(["git", "-c", "commit.gpgsign=false", "-c", "core.hooksPath=/dev/null", "-c", "user.name=t", "-c", "user.email=t@e.x"] + list(a), cwd=cwd, env=env, check=True, capture_output=True, text=True).stdout


def norm(s):
    return re.sub(r"\s+", " ", s.strip()).lower()


root = tempfile.mkdtemp(prefix="emit-oracle-")
try:
    git(root, "init", "-q")
    shas = []
    for c in corpus["commits"]:
        a = ["commit", "-q", "--allow-empty", "-m", c["subject"]] + (["-m", c["body"]] if c["body"] else [])
        git(root, *a)
        shas.append(git(root, "rev-parse", "HEAD").strip())
    subjects = git(root, "log", "--format=%H%x09%s").strip().split("\n")
    wip = next(((h, s) for h, s in (x.split("\t", 1) for x in subjects) if WIP.match(s)), None)
    if wip is None or wip[0] != shas[corpus["expectedRange"]["index"]]:
        failures.append("range")
    lines = set()
    for body in git(root, "log", "--format=%B%x00", shas[0] + "..HEAD").split("\0"):
        for raw in body.split("\n"):
            l = raw.strip()
            if len(l) < 12 or WIP.match(l) or l.startswith("📊" + VS + "metric") or l.startswith("Signed-off-by:") or re.match("^" + DAY + "⏰" + VS, l):
                continue
            k = re.sub(r"^(" + DAY + ")", r"\1", l)
            if re.match("^" + DAY + "$", l.split("📊")[0].strip()):
                continue
            lines.add(norm(l))
    if sorted(lines) != sorted(corpus["history"]):
        failures.append("history")
    for c in corpus["bullets"]:
        if (norm(c["bullet"]) in set(corpus["history"])) != c["expected"]:
            failures.append("bullet %r" % c["bullet"])
    for kind, expect_copy in (("ok", False), ("copy", True)):
        hit = any(norm(b) in lines for bundle in corpus["fresh"][kind] for d in bundle["dates"] for b in d["bullets"])
        if hit != expect_copy:
            failures.append("fresh " + kind)
finally:
    shutil.rmtree(root, ignore_errors=True)

print(json.dumps({"failures": failures}))
sys.exit(1 if failures else 0)

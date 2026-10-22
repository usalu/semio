#!/usr/bin/env python3
"""🐍️ Shell oracle of the prepare commands: tokenises every rendered command with `shlex` and checks that the operands come back exactly (stdlib only; the Bun test passes the corpus, this model renders its own commands and parses both).

usage: oracle.py CORPUS.json      (exit 0 when every verdict holds)"""
import json, re, sys

corpus = json.load(open(sys.argv[1], encoding="utf8"))
failures = []
VS = "\ufe0f"


def norm(s):
    return s.replace("\ufe0f", "").replace("\ufe0e", "")


import shlex

CODE = chr(96) * 3


def q(s):
    return "'" + s.replace("'", "'\"'\"'") + "'"


def tag_command(name, head="HEAD"):
    return "git tag -s -m %s %s %s" % (q(name), q(name), head)


for c in corpus["cases"]:
    msg = c.get("messageFile", ".git/compose-commit-message")
    tag = tag_command(c["tagName"])
    squash = "git reset --soft %s && git commit -S -F %s" % (c["wipSha"], q(msg))
    allc = "%s && git reset --soft %s && git commit -S -F %s && git push --follow-tags" % (tag, c["wipSha"], q(msg))
    text = "\n\n".join("%s\n%s\n%s" % (CODE, x, CODE) for x in (tag, squash, "git push --follow-tags", allc)) + "\n"
    first = shlex.split(tag)
    if first != ["git", "tag", "-s", "-m", c["tagName"], c["tagName"], "HEAD"]:
        failures.append("shlex tag %r" % c["tagName"])
    if shlex.split(squash)[-1] != msg:
        failures.append("shlex message %r" % msg)
    if text != c["expected"]:
        failures.append("render %r" % c["tagName"])
    for line in c["expected"].split("\n"):
        if line.startswith("git tag") and shlex.split(line)[4] != c["tagName"]:
            failures.append("expected tag operand %r" % c["tagName"])
for t in corpus["tags"]:
    if shlex.split(tag_command(t))[4:6] != [t, t]:
        failures.append("roundtrip %r" % t)

print(json.dumps({"failures": failures}))
sys.exit(1 if failures else 0)

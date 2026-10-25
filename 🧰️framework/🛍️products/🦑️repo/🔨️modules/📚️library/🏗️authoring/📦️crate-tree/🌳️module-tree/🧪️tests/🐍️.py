#!/usr/bin/env python3
"""🐍️ Independent model of the cfg-implies-test predicate, the dangling-doc demotion, comment-start extension and path attribute reading (stdlib only).

usage: 🐍️.py CORPUS.json      (exit 0 when every verdict holds)"""
import json, re, sys

corpus = json.load(open(sys.argv[1], encoding="utf8"))
failures = []
VS = "\ufe0f"


def norm(s):
    return s.replace("\ufe0f", "").replace("\ufe0e", "")



def implies(p):
    p = p.strip()
    if p == "test":
        return True
    m = re.match(r"^(all|any)\s*\((.*)\)$", p, re.S)
    if not m:
        return False
    body, args, depth, start = m.group(2), [], 0, 0
    for i, ch in enumerate(body):
        if ch == "(":
            depth += 1
        elif ch == ")":
            depth -= 1
        elif ch == "," and depth == 0:
            args.append(body[start:i].strip())
            start = i + 1
    args.append(body[start:].strip())
    real = [a for a in args if a]
    return any(implies(a) for a in real) if m.group(1) == "all" else bool(real) and all(implies(a) for a in real)


def cfg(attr):
    m = re.match(r"^cfg\s*\((.*)\)$", attr.strip(), re.S)
    return m is not None and implies(m.group(1))


for c in corpus["cfg"]:
    if cfg(c["attribute"]) != c["expected"]:
        failures.append("cfg %r" % c["attribute"])
DOC = re.compile(r"^[ \t]*///(?!/)")


def demote(src):
    lines = src.split("\n")
    skippable = lambda l: l.strip() == "" or (re.match(r"^[ \t]*//", l) is not None and not DOC.match(l))
    i = 0
    while i < len(lines):
        if not DOC.match(lines[i]):
            i += 1
            continue
        end = i
        while end + 1 < len(lines) and DOC.match(lines[end + 1]):
            end += 1
        nxt = end + 1
        while nxt < len(lines) and skippable(lines[nxt]):
            nxt += 1
        if nxt >= len(lines) or re.match(r"^[ \t]*[})\]]", lines[nxt]):
            for k in range(i, end + 1):
                lines[k] = lines[k].replace("///", "//", 1)
        i = end + 1
    return "\n".join(lines)


for c in corpus["docs"]:
    if demote(c["source"]) != c["expected"]:
        failures.append("docs %r" % c["source"])
for c in corpus["pathValue"]:
    m = re.match(r'^path\s*=\s*"((?:[^"\\]|\\.)*)"$', c["attribute"])
    if (m.group(1) if m else None) != c["expected"]:
        failures.append("path %r" % c["attribute"])


def comment_start(src, start):
    cur = start
    while True:
        ls = src.rfind("\n", 0, max(cur - 1, 0)) + 1
        prev = src[ls:cur - 1]
        if cur <= 0 or not re.match(r"^\s*//(?!!)", prev) or prev.strip() == "":
            return cur
        cur = ls


for c in corpus["itemComments"]:
    at = comment_start(c["source"], c["source"].index(c["needle"]))
    if c["source"][at:at + len(c["expected"])] != c["expected"]:
        failures.append("item comments %r" % c["source"])

print(json.dumps({"failures": failures}))
sys.exit(1 if failures else 0)

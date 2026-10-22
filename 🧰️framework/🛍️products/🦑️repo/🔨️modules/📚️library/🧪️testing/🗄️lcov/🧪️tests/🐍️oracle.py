#!/usr/bin/env python3
"""🐍️ Independent model of LCOV parsing, merging, rendering and Go cover-profile expansion (stdlib only).

usage: oracle.py CORPUS.json      (exit 0 when every verdict holds)"""
import json, re, sys

corpus = json.load(open(sys.argv[1], encoding="utf8"))
failures = []
VS = "\ufe0f"


def norm(s):
    return s.replace("\ufe0f", "").replace("\ufe0e", "")



def parse(text):
    recs, cur = [], None
    for raw in re.split(r"\r?\n", text):
        line = raw.strip()
        if line.startswith("SF:"):
            cur = {"path": line[3:], "lines": {}}
            recs.append(cur)
        elif line.startswith("DA:") and cur is not None:
            a, _, b = line[3:].partition(",")
            n, h = int(a), int(b) if b.strip().lstrip("-").isdigit() else 0
            cur["lines"][n] = cur["lines"].get(n, 0) + h
        elif line == "end_of_record":
            cur = None
    return [{"path": r["path"], "lines": [[k, v] for k, v in r["lines"].items()]} for r in recs]


for c in corpus["parse"]:
    if parse(c["text"]) != c["expected"]:
        failures.append("parse: " + c["name"])
merged = {}
for s in corpus["merge"]["sets"]:
    for r in s:
        t = merged.setdefault(r["path"], {})
        for n, h in r["lines"]:
            t[n] = t.get(n, 0) + h
if [[p, [[n, h] for n, h in l.items()]] for p, l in merged.items()] != corpus["merge"]["expected"]:
    failures.append("merge")
out = []
for p, l in corpus["render"]["merged"]:
    out.append("SF:" + p)
    for n, h in sorted(l):
        out.append("DA:%d,%d" % (n, h))
    out += ["end_of_record", ""]
if "\n".join(out) != corpus["render"]["expected"]:
    failures.append("render")
for c in corpus["go"]:
    files = {}
    for raw in re.split(r"\r?\n", c["text"]):
        line = raw.strip()
        if not line or line.startswith("mode:"):
            continue
        m = re.match(r"^(.+):(\d+)\.\d+,(\d+)\.\d+ \d+ (\d+)$", line)
        if not m:
            continue
        f, a, b, h = m.group(1), int(m.group(2)), int(m.group(3)), int(m.group(4))
        d = files.setdefault(f, {})
        for n in range(a, b + 1):
            d[n] = d.get(n, 0) + h
    got = [{"path": f, "lines": [[n, h] for n, h in d.items()]} for f, d in files.items()]
    if got != c["expected"]:
        failures.append("go: " + c["name"])

print(json.dumps({"failures": failures}))
sys.exit(1 if failures else 0)

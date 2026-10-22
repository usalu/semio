#!/usr/bin/env python3
"""🐍️ Independent derivation of the cumulative level arguments from the ordered level list (stdlib only).

usage: oracle.py CORPUS.json      (exit 0 when every verdict holds)"""
import json, re, sys

corpus = json.load(open(sys.argv[1], encoding="utf8"))
failures = []
VS = "\ufe0f"


def norm(s):
    return s.replace("\ufe0f", "").replace("\ufe0e", "")



LV = ["fundamental", "quick", "long", "exhaustive"]
BUD = {"fundamental": 15000, "quick": 300000, "long": 900000, "exhaustive": 1800000}


def above(l):
    return LV[LV.index(l) + 1:]


def secs(ms):
    return -(-ms // 1000)


def derive(l, ms):
    ab = above(l)
    go = ["-timeout", "%ds" % secs(ms)] + (["-short"] if LV.index(l) <= LV.index("quick") else []) + (["-skip", "^Test(" + "|".join(x.capitalize() for x in ab) + ")"] if ab else [])
    py = (["-m", " and ".join("not " + x for x in ab)] if ab else []) + ["--timeout=%d" % secs(ms), "--timeout-method=thread"]
    dn = (["--filter", "&".join("Category!=" + x for x in ab)] if ab else []) + ["--blame-hang", "--blame-hang-timeout", str(ms)]
    vt = ["--testTimeout", str(ms), "--hookTimeout", str(ms), "--teardownTimeout", str(ms)]
    return {"go": go, "pytest": py, "dotnet": dn, "vitest": vt, "bun": ["--timeout", str(ms)], "playwright": ms}


for r in corpus["levels"]:
    d = derive(r["level"], BUD[r["level"]])
    if any(d[k] != r[k] for k in d):
        failures.append("level " + r["level"])
o = corpus["override"]
d = derive("fundamental", int(o["budgetMs"]))
if d["go"] != o["go"] or d["pytest"] != o["pytest"] or d["bun"] != o["bun"] or d["playwright"] != o["playwright"]:
    failures.append("override")

print(json.dumps({"failures": failures}))
sys.exit(1 if failures else 0)

#!/usr/bin/env python3
"""🐍️ Independent model of the micro-commit counter: parses on selector-stripped text and maps the match back to the original string (stdlib only).

usage: oracle.py CORPUS.json      (exit 0 when every verdict holds)"""
import json, re, sys

corpus = json.load(open(sys.argv[1], encoding="utf8"))
failures = []
VS = "\ufe0f"


def norm(s):
    return s.replace("\ufe0f", "").replace("\ufe0e", "")



SEL = "︎️"


def parse(subject):
    s = subject.strip()
    stripped = "".join(c for c in s if c not in SEL)
    m = re.match(r"^(.+🎆\d{2}🌙\d{2}☀\d{2})🚩(\d+)$", stripped)
    if not m:
        return None
    return {"nnn": int(m.group(2)), "stripped": m.group(1), "original": s}


def line1(subject):
    p = parse(subject)
    if not p:
        return None
    o = p["original"]
    cut = o.rfind("🚩")
    return o[:cut]


def numeric(subject):
    m = re.match(r"^(\d+)$", subject.strip())
    return int(m.group(1)) if m and int(m.group(1)) > 0 else None


def bump(subjects, who, now, wip):
    prefix = who["emoji"] + who["alias"]
    pn = "".join(c for c in prefix if c not in SEL)
    top, base = 0, None
    for s in subjects:
        p = parse(s)
        if p:
            if not p["stripped"].startswith(pn):
                continue
            top = max(top, p["nnn"])
            base = base or line1(s)
        elif numeric(s) is not None:
            top = max(top, numeric(s))
    yy, mm, dd = now
    fresh = "%s🎆️%02d🌙️%02d☀️%02d" % (prefix, yy % 100, mm, dd)
    epoch = base or wip or fresh
    if top > 0:
        return {"line1Base": epoch, "nnn": "%03d" % (top + 1)}
    for s in subjects:
        n = "".join(c for c in s.strip() if c not in SEL)
        if n.startswith(pn + "🎆") and re.search(r"🚩\d+$", n) and not parse(s):
            raise ValueError("refuse")
    return {"line1Base": epoch, "nnn": "001"}


for c in corpus["extract"]:
    p = parse(c["subject"])
    got = None if p is None else {"nnn": p["nnn"], "line1Base": line1(c["subject"])}
    if got != c["expected"]:
        failures.append("extract %r" % c["subject"])
for c in corpus["numeric"]:
    if numeric(c["subject"]) != c["expected"]:
        failures.append("numeric %r" % c["subject"])
for c in corpus["tag"]:
    n = "".join(ch for ch in c["tag"].strip() if ch not in SEL)
    m = re.match(r"^(.+🎆\d{2}🌙\d{2}☀\d{2})🚩$", n)
    got = c["tag"].strip()[: c["tag"].strip().rfind("🚩")] if m and c["tag"].strip().endswith("🚩" + VS) else None
    if got != c["expected"]:
        failures.append("tag %r" % c["tag"])
for c in corpus["bump"]:
    if bump(c["subjects"], c["contributor"], c["now"], c["wip"]) != c["expected"]:
        failures.append("bump: " + c["name"])
for c in corpus["bumpErrors"]:
    try:
        bump(c["subjects"], c["contributor"], [2026, 10, 21], None)
        failures.append("bump error: " + c["name"])
    except ValueError:
        pass
for c in corpus["second"]:
    y, mo, d, h, mi, s = c["now"]
    if "🎆️%02d🌙️%02d☀️%02d⏰️%02d⌚️%02d⏱️%02d" % (y % 100, mo, d, h, mi, s) != c["expected"]:
        failures.append("second %s" % c["expected"])
for c in corpus["pad"]:
    if str(c["n"]).rjust(2, "0") != c["expected"]:
        failures.append("pad %d" % c["n"])

print(json.dumps({"failures": failures}))
sys.exit(1 if failures else 0)

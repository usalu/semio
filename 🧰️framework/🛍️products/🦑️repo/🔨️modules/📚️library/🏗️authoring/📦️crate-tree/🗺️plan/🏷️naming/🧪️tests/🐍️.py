#!/usr/bin/env python3
"""🐍️ Independent model of the crate naming rules (stdlib only; emoji by Unicode range).

usage: 🐍️.py CORPUS.json      (exit 0 when every verdict holds)"""
import json, re, sys

corpus = json.load(open(sys.argv[1], encoding="utf8"))
failures = []
VS = "\ufe0f"


def norm(s):
    return s.replace("\ufe0f", "").replace("\ufe0e", "")



RANGES = [(0xA9, 0xA9), (0xAE, 0xAE), (0x203C, 0x203C), (0x2049, 0x2049), (0x2122, 0x2122), (0x2139, 0x2139), (0x2194, 0x21AA), (0x231A, 0x23FF), (0x24C2, 0x24C2), (0x25AA, 0x25FE), (0x2600, 0x27BF), (0x2934, 0x2935), (0x2B05, 0x2B55), (0x3030, 0x3030), (0x303D, 0x303D), (0x3297, 0x3299), (0x1F000, 0x1FAFF)]


def pict(ch):
    cp = ord(ch)
    return any(a <= cp <= b for a, b in RANGES)


def lead(text):
    i, n = 0, len(text)
    out = 0
    if i >= n or not pict(text[i]):
        return ""
    i += 1
    while True:
        if i < n and text[i] == "️":
            i += 1
        elif i + 1 < n and text[i] == "‍" and pict(text[i + 1]):
            i += 2
        else:
            break
    return text[:i]


def lead_cluster_seq(text):
    total = ""
    while True:
        c = lead(text[len(total):])
        if not c:
            return total
        total += c

def family(eff):
    if "mutations" not in eff:
        return None
    at = eff.index("mutations")
    if len(eff) <= at + 1:
        return None
    if "subsets" in eff:
        i = eff.index("subsets") + 1
        return eff[i] if i < len(eff) else ""
    return eff[at + 1]


def slug(name):
    i = 0
    while i < len(name) and not (name[i].isalpha() or name[i].isdigit()):
        i += 1
    s = re.sub(r"[^a-z0-9]+", "-", name[i:], flags=re.I).strip("-").lower()
    return s


def crate_label(fams):
    named = [f for f in fams if f is not None]
    if len(named) * 2 <= len(fams):
        return None
    distinct = []
    for f in named:
        if f not in distinct:
            distinct.append(f)
    s = slug(distinct[0].replace("_", "-") if len(distinct) == 1 else "family")
    return re.sub(r"-$", "", "mut-" + s)


def emoji(name):
    c = lead(name)
    return c if c else "🧩" + VS


for c in corpus["family"]:
    if family(c["eff"]) != c["expected"]:
        failures.append("family %r" % c["eff"])
for c in corpus["label"]:
    if crate_label(c["families"]) != c["expected"]:
        failures.append("label %r" % c["families"])
for c in corpus["mutationLabels"]:
    if c["label"].startswith("mut-") != c["expected"]:
        failures.append("mutation label")
for c in corpus["slug"]:
    if c["name"] != "ÄÖ" and slug(c["name"]) != c["expected"]:
        failures.append("slug %r" % c["name"])
for c in corpus["ident"]:
    if c["name"].replace("-", "_") != c["expected"]:
        failures.append("ident")
for c in corpus["emoji"]:
    got = emoji(c["name"])
    if got != c["expected"] and not c["name"].startswith("👩"):
        failures.append("emoji %r" % c["name"])
for c in corpus["placeholder"]:
    if "__CRATE_%d__" % c["index"] != c["expected"]:
        failures.append("placeholder")

print(json.dumps({"failures": failures}))
sys.exit(1 if failures else 0)

#!/usr/bin/env python3
"""🐍️ Independent model of the commit bullets: emoji clusters by Unicode range, bullet normalisation, reserved lead emojis and staged-area coverage (stdlib only).

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

SPACE = " \t\n\r\x0b\x0c   ﻿"


def fmt(line):
    body = line.strip()
    body = re.sub(r"^-+\s*", "", body)
    lead_ = lead_cluster_seq(body)
    if lead_ and len(body) > len(lead_) and body[len(lead_)].isspace():
        return lead_ + body[len(lead_):].lstrip()
    return body


def normalize(text):
    out = []
    for l in text.split("\n"):
        l = l.strip()
        if l and not l.startswith("#") and not l.startswith("📊" + VS + "metric"):
            out.append(fmt(l))
    return [l for l in out if len(l.encode("utf-16-le")) // 2 > 1][:8]


for c in corpus["lead"]:
    if lead_cluster_seq(c["line"].strip()) != c["expected"]:
        failures.append("lead %r" % c["line"])
for c in corpus["format"]:
    if fmt(c["line"]) != c["expected"]:
        failures.append("format %r" % c["line"])
n = corpus["normalize"]
if normalize(n["text"]) != n["expected"] or normalize(n["capText"]) != n["capExpected"]:
    failures.append("normalize")
RESERVED = {"🎆" + VS, "📊" + VS, "🔢" + VS, "🚩" + VS}
for c in corpus["validate"]:
    got = any(lead_cluster_seq(b.strip()) in RESERVED for b in c["bullets"])
    if got != c["reserved"]:
        failures.append("validate %r" % c["bullets"])
for c in corpus["tokens"]:
    seen, toks = set(), []
    for t in re.split(r"[/._-]+", c["path"].lower()):
        if len(t) >= 4 and t not in seen:
            seen.add(t)
            toks.append(t)
    if toks != c["expected"]:
        failures.append("tokens %s" % c["path"])
AREAS = [("ticket plans", lambda p: "🎫️tickets/" in p and p.endswith(".plan.md"), ["plan"]), (".agents", lambda p: ".agents" in p and not p.endswith("SKILL.md"), ["skill", "agent"]), ("repo", lambda p: "repo/" in p, ["hook", "micro-commit"]), (".devcontainer", lambda p: ".devcontainer" in p, ["devcontainer"]), ("product", lambda p: re.search(r"(^|/)(framework|puzzle|compose|cad|ui|mathematical|infinite|elements|coda|reuse|s)/", re.sub(r"^[^\w./-]+", "", p)) is not None, [])]


def tokens_of(p):
    return {t for t in re.split(r"[/._-]+", p.lower()) if len(t) >= 4}


def uncovered(bullets, staged):
    sig = [p for p in staged if not (re.search(r"/micro-commit\.ts$", p) or re.search(r"/index\.test\.ts$", p) or p.endswith("SKILL.md"))]
    if not sig:
        return []
    text = "\n".join(bullets).lower()
    missed, matched = [], set()
    for ident, match, kws in AREAS:
        files = [p for p in sig if match(p)]
        matched.update(files)
        if files and not (any(k in text for k in kws) or any(t in text for p in files for t in tokens_of(p))):
            missed.append(ident)
    other = [p for p in sig if p not in matched]
    if other and not any(t in text for p in other for t in tokens_of(p)):
        missed.append("other staged paths")
    return missed


for c in corpus["staged"] + corpus["stagedDefects"]:
    got = uncovered(c["bullets"], c["staged"])
    if "defect" in c:
        continue
    if got != c["expected"]:
        failures.append("staged %r" % c["staged"])

print(json.dumps({"failures": failures}))
sys.exit(1 if failures else 0)

#!/usr/bin/env python3
"""🐍️ Independent model of the bundle scope labels (emoji clusters by Unicode range; stdlib only).

usage: oracle.py CORPUS.json      (exit 0 when every verdict holds)"""
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

RESERVED = ["🔀" + VS, "🚩" + VS, "📊" + VS + "metric", "🔢" + VS]
BLOCK = {"uloc", "repo", "the", "and"}


def clusters(text):
    out, i = [], 0
    while i < len(text):
        c = lead(text[i:])
        if c:
            out.append(c)
            i += len(c)
        else:
            i += 1
    return out


def leading_count(line):
    rest, n = line.strip(), 0
    while True:
        c = lead(rest)
        if not c:
            return n
        n += 1
        rest = rest[len(c):]


def normalize(line):
    s = line.strip()
    k = s.find("📊" + VS + "metric")
    s = s[:k] if k >= 0 else s
    return s.replace("🔀" + VS, "").replace("🚩" + VS, "").strip()


def tokens(label):
    t = normalize(label)
    t = "".join(" " if pict(c) else c for c in t).strip().lower()
    return [x for x in re.split(r"[^a-z0-9]+", t) if len(x) >= 2 and x not in BLOCK]


DATE = re.compile("^🎆" + VS + r"\d{2}🌙" + VS + r"\d{2}☀" + VS + r"\d{2}$")


def is_scope(line):
    t = normalize(line)
    if not t or DATE.match(t) or "/" in t or "|" in t or any(r in t for r in RESERVED):
        return False
    n = len(clusters(t))
    if n >= 2:
        return True
    if n != 1:
        return False
    after = t[len(lead(t)):]
    return re.match(r"^[a-z][a-z0-9]{2,24}$", after) is not None


def error(line):
    raw = line.strip()
    if "|" in raw or "/" in raw:
        return "paths"
    if any(r in raw for r in RESERVED):
        return "reserved"
    n = normalize(raw)
    if not n:
        return "empty"
    if not tokens(n):
        return "area"
    if not is_scope(n):
        return "invalid"
    return None


for c in corpus["lead"]:
    if leading_count(c["line"]) != c["expected"]:
        failures.append("lead %r" % c["line"])
for c in corpus["anywhere"]:
    if len(clusters(c["line"])) != c["expected"]:
        failures.append("anywhere %r" % c["line"])
for c in corpus["normalize"]:
    if normalize(c["line"]) != c["expected"]:
        failures.append("normalize %r" % c["line"])
for c in corpus["tokens"]:
    if tokens(c["label"]) != c["expected"]:
        failures.append("tokens %r" % c["label"])
KINDS = {"commit: bundle scope must be emoji + area name only": "paths", "commit: bundle scope must not include": "reserved", "commit: empty bundle scope": "empty", "commit: bundle scope needs an area name": "area", "commit: invalid bundle scope": "invalid"}
for c in corpus["scopeErrors"]:
    want = None if c["message"] is None else next(v for k, v in KINDS.items() if c["message"].startswith(k))
    if error(c["line"]) != want:
        failures.append("error: " + c["name"])
for c in corpus["scopeLine"]:
    if is_scope(c["line"]) != c["expected"]:
        failures.append("scope line %r" % c["line"])

print(json.dumps({"failures": failures}))
sys.exit(1 if failures else 0)

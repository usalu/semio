#!/usr/bin/env python3
"""🐍️ Independent model of the bundle date lines: regex on the same literals, exact-decimal metric suffixes (stdlib only).

usage: oracle.py CORPUS.json      (exit 0 when every verdict holds)"""
import json, re, sys

corpus = json.load(open(sys.argv[1], encoding="utf8"))
failures = []
VS = "\ufe0f"


def norm(s):
    return s.replace("\ufe0f", "").replace("\ufe0e", "")


from decimal import Decimal, ROUND_HALF_UP
import math

def rs(n, d=3):
    if not math.isfinite(n) or n == 0:
        return Decimal(0)
    x = Decimal(repr(abs(n)))
    exp = x.adjusted()
    q = Decimal(1).scaleb(exp - d + 1)
    r = x.quantize(q, rounding=ROUND_HALF_UP) if d - 1 - exp >= 0 else (x / q).quantize(Decimal(1), rounding=ROUND_HALF_UP) * q
    return r if n > 0 else -r

def plain(x):
    s = format(x.normalize(), "f")
    if "." in s:
        s = s.rstrip("0").rstrip(".")
    return s or "0"

def coeff(x):
    if x == 0:
        return "0"
    return plain(rs(float(x), 3))

def count(n, units):
    v = abs(n)
    if not math.isfinite(v) or v <= 0:
        return "0"
    r = rs(v, 3)
    if r == 0:
        r = rs(v, 1)
    for size, suffix in units:
        if r >= size:
            return coeff(r / size) + suffix
    return coeff(r) + units[-1][1] if False else coeff(r) + ("" if units is LOC else "B")

LOC = [(Decimal(1000000), "M"), (Decimal(1000), "k")]
SIZE = [(Decimal(1000000000), "GB"), (Decimal(1000000), "MB"), (Decimal(1000), "KB")]

def loc(n):
    return count(n, LOC)

def size(n):
    return count(n, SIZE)

def ratio(n):
    if not math.isfinite(n) or n <= 0:
        return "0"
    pct = n * 100
    v = rs(pct, 3)
    if v == 0:
        v = rs(pct, 1)
    if v >= 1:
        return coeff(v)
    places = 3 if v >= Decimal("0.1") else 2 if v >= Decimal("0.01") else 3 if v >= Decimal("0.001") else 4
    f = Decimal(10) ** places
    capped = (v * f).quantize(Decimal(1), rounding=ROUND_HALF_UP) / f
    if capped == 0:
        return coeff(1 / f)
    return coeff(capped)

H = "📊️metric"
SYM = {"net": "➕️", "edit": "✏️", "rem": "➖️", "sum": "\U0001f7f0️", "bloc": "\U0001f4af️", "up": "\U0001f4c8️", "down": "\U0001f4c9️", "ratio": "➗️"}
TOK = {"uloc": "📃uloc", "size": "💾size"}

def body_line(kind, code, added, edited, removed, le=None, ls=None):
    f = loc if kind == "uloc" else size
    line = H + (le + ls if le and ls else "") + TOK[kind]
    bloc = max(0, int(code + 0.5))
    if bloc > 0:
        line += SYM["bloc"] + f(bloc)
    net = added - removed
    if net != 0:
        line += (SYM["up"] if net > 0 else SYM["down"]) + f(abs(net))
        if bloc - net > 0:
            line += SYM["ratio"] + ratio(abs(net) / (bloc - net))
    for key, v in (("net", added), ("edit", edited), ("rem", removed)):
        if v > 0:
            line += SYM[key] + f(v)
    t = added + edited + removed
    if t > 0:
        line += SYM["sum"] + f(t)
    return line

DAY = "🎆" + VS + r"\d{2}🌙" + VS + r"\d{2}☀" + VS + r"\d{2}"


def from_subject(s):
    m = re.search(DAY, s.strip())
    return m.group(0) if m else None


def from_body(b):
    for raw in b.split("\n"):
        line = raw.strip()
        if re.match("^" + DAY + "⏰" + VS, line):
            return re.match("^" + DAY, line).group(0)
    return None


def normalize(line):
    s = line.strip()
    k = s.find("📊" + VS + "metric")
    return (s[:k] if k >= 0 else s).strip()


for c in corpus["subject"]:
    if from_subject(c["subject"]) != c["expected"]:
        failures.append("subject %r" % c["subject"])
for c in corpus["body"]:
    if from_body(c["body"]) != c["expected"]:
        failures.append("body %r" % c["body"])
for c in corpus["commit"]:
    if (from_body(c["body"]) or from_subject(c["subject"])) != c["expected"]:
        failures.append("commit")
for c in corpus["normalize"]:
    if normalize(c["line"]) != c["expected"]:
        failures.append("normalize %r" % c["line"])
for c in corpus["format"]:
    u, s = c["ulocDelta"], c["sizeDelta"]
    got = normalize(c["line"]) + body_line("uloc", c["ulocBloc"], u["added"], u["edited"], u["removed"]) + body_line("size", c["sizeBloc"], s["added"], s["edited"], s["removed"])
    if got != c["expected"]:
        failures.append("format %r" % c["line"])
for c in corpus["bundleModel"]["wip"]:
    m = re.match("^(.+" + DAY + ")🔀" + VS + "$", c["subject"])
    if (m.group(1) if m else None) != c["expected"]:
        failures.append("wip %r" % c["subject"])
for c in corpus["bundleModel"]["date"]:
    if (re.match("^" + DAY + "$", c["line"]) is not None) != c["expected"]:
        failures.append("date %r" % c["line"])

print(json.dumps({"failures": failures}))
sys.exit(1 if failures else 0)

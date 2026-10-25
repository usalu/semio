#!/usr/bin/env python3
"""🐍️ Independent model of the commit metric summary: totals, language ordering and metric lines from the corpus rows, with exact-decimal number formatting (stdlib only).

usage: 🐍️.py CORPUS.json      (exit 0 when every verdict holds)"""
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

EM = {"Rust": "rust", "TypeScript": "typescript", "Go": "go", "Python": "python"}


def keys(v):
    if isinstance(v, dict):
        return len(v) + sum(keys(x) for x in v.values())
    return sum(keys(x) for x in v) if isinstance(v, list) else 0


def total(rows):
    return {"lang": "Total", "emoji": corpus["totalEmoji"], **{k: sum(r[k] for r in rows) for k in ("code", "edited", "added", "removed")}}


def active(r):
    return any(r[k] > 0 for k in ("code", "edited", "added", "removed"))


def line(kind, r):
    return body_line(kind, r["code"], r["added"], r["edited"], r["removed"], r["emoji"], EM[r["lang"]])


def render(uloc, sz):
    if not uloc and not sz:
        return []
    out = []
    for kind, rows in (("uloc", uloc), ("size", sz)):
        t = total(rows)
        if active(t):
            out.append(body_line(kind, t["code"], t["added"], t["edited"], t["removed"]))
    code = {r["lang"]: r["code"] for r in uloc}
    langs = sorted(code, key=lambda l: (-code[l], l))
    for l in langs:
        for kind, rows in (("uloc", uloc), ("size", sz)):
            r = next((x for x in rows if x["lang"] == l), None)
            if r and active(r):
                out.append(line(kind, r))
    for r in sz:
        if r["lang"] not in langs and active(r):
            out.append(line("size", r))
    return out


if total(corpus["sum"]["rows"]) != corpus["sum"]["expected"]:
    failures.append("sum")
for c in corpus["linesCases"]:
    if render(c["bundle"]["uloc"], c["bundle"]["size"]) != c["expected"]:
        failures.append("lines: " + c["name"])
for c in corpus["lineFormat"]:
    if line(c["kind"], c["row"]) != c["expected"]:
        failures.append("line %s" % c["kind"])
for c in corpus["deltaSums"]:
    if (c["a"] == c["b"]) != c["expected"]:
        failures.append("delta sums")
files = corpus["count"]["files"]
for c in corpus["count"]["cases"]:
    lines_ = 0
    bytes_ = 0
    for path, text in files.items():
        if path.startswith("dist/") or path.endswith(".xyz") or not (not c["prefixes"] or any(path == p or path.startswith(p + "/") for p in c["prefixes"])):
            continue
        bytes_ += len(text.encode())
        lines_ += keys(json.loads(text)) if path.endswith(".json") else len(text.split("\n"))
    if (lines_, bytes_) != (c["lines"], c["bytes"]):
        failures.append("count %s" % c["prefixes"])

print(json.dumps({"failures": failures}))
sys.exit(1 if failures else 0)

#!/usr/bin/env python3
"""🐍️ Independent model of the commit attribution: replays the corpus commits with the git CLI, assigns paths to bundles by label tokens and recomputes the per-bundle and per-day sums from `git diff --numstat` (stdlib + git).

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
import os, subprocess, tempfile, shutil

EMOJI = lambda c: 0x1F000 <= ord(c) <= 0x1FAFF or 0x2600 <= ord(c) <= 0x27BF or 0x2B00 <= ord(c) <= 0x2BFF


def tokens(label):
    t = label.strip()
    k = t.find("📊" + VS + "metric")
    t = (t[:k] if k >= 0 else t).replace("🔀" + VS, "").replace("🚩" + VS, "").strip()
    t = "".join(" " if EMOJI(c) else c for c in t).lower()
    return [x for x in re.split(r"[^a-z0-9]+", t) if len(x) >= 2 and x not in {"uloc", "repo", "the", "and"}]


def assign(path, bundles):
    best, score = -1, 0
    for i, b in enumerate(bundles):
        tk = tokens(b["label"])
        if tk and all(t in path.lower() for t in tk) and len(tk) > score:
            best, score = i, len(tk)
    return best


def git(cwd, *a):
    env = dict(os.environ, GIT_CONFIG_GLOBAL="/dev/null", GIT_CONFIG_NOSYSTEM="1")
    return subprocess.run(["git", "-c", "commit.gpgsign=false", "-c", "core.hooksPath=/dev/null", "-c", "user.name=t", "-c", "user.email=t@e.x"] + list(a), cwd=cwd, env=env, check=True, capture_output=True, text=True).stdout.strip()


def split(a, r):
    e = min(a, r)
    return a - e, e, r - e


root = tempfile.mkdtemp(prefix="attrib-oracle-")
try:
    git(root, "init", "-q")

    def put(files):
        for p, t in files.items():
            f = os.path.join(root, *p.split("/"))
            os.makedirs(os.path.dirname(f), exist_ok=True)
            open(f, "w").write(t)
    put(corpus["repo"]["base"])
    git(root, "add", "-A")
    git(root, "commit", "-q", "-m", "base")
    revs = [git(root, "rev-parse", "HEAD")]
    for c in corpus["repo"]["commits"]:
        put(c["files"])
        git(root, "add", "-A")
        git(root, "commit", "-q", "-m", c["subject"], "-m", c["body"])
        revs.append(git(root, "rev-parse", "HEAD"))
    bundles = corpus["bundles"]
    assigned = [[] for _ in bundles]
    for line in git(root, "diff", "--name-only", revs[0] + ".." + revs[-1]).split("\n"):
        i = assign(line, bundles)
        if i >= 0:
            assigned[i].append(line)
    if assigned != corpus["expected"]["assigned"]:
        failures.append("assigned")

    def numstat(a, b):
        out = []
        for l in git(root, "diff", "--numstat", a + ".." + b).split("\n"):
            x, y, p = l.split("\t")
            out.append((p, int(x), int(y)))
        return out

    def blob_size(rev, p):
        r = subprocess.run(["git", "cat-file", "-s", rev + ":" + p], cwd=root, capture_output=True, text=True)
        return int(r.stdout) if r.returncode == 0 else 0
    for kind, key in (("uloc", "dateUloc"), ("size", "dateSize")):
        got = [dict() for _ in bundles]
        for n in range(1, len(revs)):
            day = re.search("🎆" + VS + r"\d{2}🌙" + VS + r"\d{2}☀" + VS + r"\d{2}", corpus["repo"]["commits"][n - 1]["body"]).group(0)
            for p, a, r in numstat(revs[n - 1], revs[n]):
                i = assign(p, bundles)
                ad, ed, rm = split(a, r) if kind == "uloc" else split(blob_size(revs[n], p), blob_size(revs[n - 1], p))
                cur = got[i].setdefault(day, {"added": 0, "removed": 0, "edited": 0})
                cur["added"] += ad
                cur["edited"] += ed
                cur["removed"] += rm
        want = [{d: v for d, v in days} for days in corpus["expected"][key]]
        if got != want:
            failures.append("per-day " + kind)
    totals = [{"added": 0, "removed": 0, "edited": 0} for _ in bundles]
    for p, a, r in numstat(revs[0], revs[-1]):
        ad, ed, rm = split(a, r)
        i = assign(p, bundles)
        totals[i]["added"] += ad
        totals[i]["edited"] += ed
        totals[i]["removed"] += rm
    if totals != corpus["expected"]["partitionUloc"]["bundleTotals"]:
        failures.append("partition")
    stotals = [{"added": 0, "removed": 0, "edited": 0} for _ in bundles]
    for p, a, r in numstat(revs[0], revs[-1]):
        ad, ed, rm = split(blob_size(revs[-1], p), blob_size(revs[0], p))
        i = assign(p, bundles)
        stotals[i]["added"] += ad
        stotals[i]["edited"] += ed
        stotals[i]["removed"] += rm
    if stotals != corpus["expected"]["partitionSize"]["bundleTotals"]:
        failures.append("size partition")
finally:
    shutil.rmtree(root, ignore_errors=True)
for c in corpus["brief"]:
    d = c["delta"]
    if "➕" + VS + str(d["added"]) + "✏" + VS + str(d["edited"]) + "➖" + VS + str(d["removed"]) + "🟰" + VS + str(d["added"] + d["edited"] + d["removed"]) != c["expected"]:
        failures.append("brief")
for c in corpus["header"]:
    n = c["label"].strip()
    k = n.find("📊" + VS + "metric")
    n = (n[:k] if k >= 0 else n).replace("🔀" + VS, "").replace("🚩" + VS, "").strip()
    u, s = c["uloc"], c["size"]
    if n + body_line("uloc", c["ulocBloc"], u["added"], u["edited"], u["removed"]) + body_line("size", c["sizeBloc"], s["added"], s["edited"], s["removed"]) != c["expected"]:
        failures.append("header")
for c in corpus["validate"]:
    bundles_ = c.get("bundles", corpus["bundles"])
    err = None
    for i, b in enumerate(bundles_):
        listed = {d["dateLine"] for d in b["dates"]}
        per = dict(c["days"][i]) if i < len(c["days"]) else {}
        for day, v in per.items():
            if day not in listed and v["added"] + v["edited"] + v["removed"] != 0:
                err = "missing"
        if c["additive"]:
            tot = c["totals"][i]
            s = {k: sum(per.get(d, {"added": 0, "removed": 0, "edited": 0})[k] for d in listed) for k in ("added", "removed", "edited")}
            if s != tot:
                err = err or "additive"
    if (err is None) != (c["message"] is None):
        failures.append("validate: " + c["name"])

print(json.dumps({"failures": failures}))
sys.exit(1 if failures else 0)

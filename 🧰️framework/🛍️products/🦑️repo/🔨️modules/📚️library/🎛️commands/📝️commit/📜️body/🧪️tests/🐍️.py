#!/usr/bin/env python3
"""🐍️ Independent model of the commit bundle body grammar: a three-state line machine (scope, day, bullet) with emoji classification by Unicode range (stdlib only).

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

DAY = "🎆" + VS + r"\d{2}🌙" + VS + r"\d{2}☀" + VS + r"\d{2}"
RES = ["🔀" + VS, "🚩" + VS, "📊" + VS + "metric", "🔢" + VS]
RESERVED_LEAD = {"🎆" + VS, "📊" + VS, "🔢" + VS, "🚩" + VS}


def normalize_scope(line):
    s = line.strip()
    k = s.find("📊" + VS + "metric")
    s = s[:k] if k >= 0 else s
    return s.replace("🔀" + VS, "").replace("🚩" + VS, "").strip()


def clusters(t):
    n, i = 0, 0
    while i < len(t):
        c = lead(t[i:])
        if c:
            n += 1
            i += len(c)
        else:
            i += 1
    return n


def is_scope(line):
    t = normalize_scope(line)
    if not t or re.match("^" + DAY + "$", t) or "/" in t or "|" in t or any(r in t for r in RES):
        return False
    n = clusters(t)
    if n >= 2:
        return True
    return n == 1 and re.match(r"^[a-z][a-z0-9]{2,24}$", t[len(lead(t)):]) is not None


def scope_error(line):
    raw = line.strip()
    if "|" in raw or "/" in raw or any(r in raw for r in RES):
        return True
    n = normalize_scope(raw)
    tokens = [x for x in re.split(r"[^a-z0-9]+", "".join(" " if pict(c) else c for c in n).strip().lower()) if len(x) >= 2 and x not in {"uloc", "repo", "the", "and"}]
    return not n or not tokens or not is_scope(n)


def bullet_ok(b):
    cl = lead(b)
    return bool(cl) and len(b) > len(cl) and not b[len(cl)].isspace() and True


def validate_bullet(b):
    if not bullet_ok(b):
        raise ValueError("bullet")
    if lead_cluster_seq(b.strip()) in RESERVED_LEAD:
        raise ValueError("reserved")
    if re.match("^" + DAY + "$", b.strip()):
        raise ValueError("date bullet")


def parse(text):
    bundles, cur, ds = [], None, None
    for raw in text.split("\n"):
        line = raw.strip()
        if not line or line.startswith("#") or line.startswith("📊" + VS + "metric") or line.startswith("Signed-off-by:"):
            continue
        cand = normalize_scope(line) if False else (lambda s: (s[:s.find("📊" + VS + "metric")] if s.find("📊" + VS + "metric") >= 0 else s).strip())(line)
        if re.match("^" + DAY + "$", cand):
            if cur is None:
                raise ValueError("date before scope")
            if ds:
                cur["dates"].append(ds)
            ds = {"dateLine": cand, "bullets": []}
            continue
        if ds is not None:
            if is_scope(line):
                if scope_error(line):
                    raise ValueError("scope")
                if cur:
                    cur["dates"].append(ds)
                    bundles.append(cur)
                cur, ds = {"label": normalize_scope(line), "dates": []}, None
                continue
            validate_bullet(line)
            ds["bullets"].append(line)
            continue
        if is_scope(line):
            if scope_error(line):
                raise ValueError("scope")
            if cur:
                bundles.append(cur)
            cur = {"label": normalize_scope(line), "dates": []}
            continue
        raise ValueError("expected")
    if cur:
        if ds:
            cur["dates"].append(ds)
        bundles.append(cur)
    if not bundles:
        raise ValueError("none")
    for b in bundles:
        if not b["dates"]:
            raise ValueError("nodate")
        for d in b["dates"]:
            if not d["bullets"]:
                raise ValueError("nobullet")
    return bundles


if parse(corpus["valid"]["text"]) != corpus["valid"]["expected"]:
    failures.append("valid")
for c in corpus["errors"]:
    try:
        parse(c["text"])
        failures.append("accepted " + c["name"])
    except ValueError:
        pass
for c in corpus["defects"]:
    if "expected" in c:
        continue
    try:
        parse(c["text"])
        failures.append("model accepted " + c["name"])
    except ValueError:
        pass


def body_error(text):
    first = next((l.strip() for l in text.strip().split("\n") if l.strip()), "")
    if re.match("^.+" + DAY + "🔀" + VS + "$", first):
        return "subject"
    if re.match("^(🐙" + VS + "|🧑" + VS + ")", first) and "🚩" + VS in first:
        return "micro"
    if re.search("^📊" + VS + "metric", text, re.M):
        return "footer"
    if re.search("^" + DAY + "📊" + VS + "metric", text, re.M):
        return "per-day"
    return None


KIND = {"must not include the bundle subject": "subject", "must not include micro-commit": "micro", "footer": "footer", "per-day": "per-day"}
for c in corpus["bodyErrors"]:
    want = None if c["message"] is None else next(v for k, v in KIND.items() if k in c["message"])
    if body_error(c["text"]) != want:
        failures.append("body error " + c["name"])

print(json.dumps({"failures": failures}))
sys.exit(1 if failures else 0)

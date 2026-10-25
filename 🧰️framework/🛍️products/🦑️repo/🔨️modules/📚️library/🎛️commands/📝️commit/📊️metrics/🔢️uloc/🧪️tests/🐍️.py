#!/usr/bin/env python3
"""🐍️ Independent model of the unified line counts: JSON keys and key-unit diffs with the stdlib `json` module.

usage: 🐍️.py CORPUS.json      (exit 0 when every verdict holds)"""
import json, re, sys

corpus = json.load(open(sys.argv[1], encoding="utf8"))
failures = []
VS = "\ufe0f"


def norm(s):
    return s.replace("\ufe0f", "").replace("\ufe0e", "")




def keys(v):
    if isinstance(v, dict):
        return len(v) + sum(keys(x) for x in v.values())
    if isinstance(v, list):
        return sum(keys(x) for x in v)
    return 0


def count_keys(text):
    t = text.strip()
    if not t:
        return 0
    try:
        return keys(json.loads(t))
    except ValueError:
        return 0


def fp(v):
    if isinstance(v, dict):
        return "object:" + "\0".join(json.dumps(k, ensure_ascii=False) for k in v)
    if isinstance(v, list):
        return "array:" + "\0".join(("[]" if isinstance(i, list) else "{}") if isinstance(i, (dict, list)) else json.dumps(i, ensure_ascii=False) for i in v)
    return json.dumps(v, ensure_ascii=False)


def units(text):
    try:
        v = json.loads(text)
    except ValueError:
        return None
    out = {}

    def walk(x, path):
        if isinstance(x, list):
            for i, item in enumerate(x):
                walk(item, "%s/%d" % (path, i))
        elif isinstance(x, dict):
            for k, item in x.items():
                p = path + "/" + json.dumps(k, ensure_ascii=False)
                out[p] = fp(item)
                walk(item, p)
    walk(v, "")
    return out or None


def diff(before, after):
    b = {} if before is None else units(before)
    a = {} if after is None else units(after)
    if b is None or a is None:
        return None
    return {"added": sum(1 for p in a if p not in b), "removed": sum(1 for p in b if p not in a), "edited": sum(1 for p in a if p in b and a[p] != b[p])}


def lines(text):
    if text == "":
        return 0
    body = text[:-2] if text.endswith("\r\n") else text[:-1] if text.endswith("\n") else text
    return len(re.split(r"\r?\n", body))


def loc(path, data):
    ext = path[path.rfind("."):].lower()
    if ext in (".json", ".jsonc"):
        k = count_keys(data)
        if k > 0:
            return k
        if not data.strip():
            return 0
    return lines(data)


for c in corpus["keys"]:
    if count_keys(c["text"]) != c["expected"]:
        failures.append("keys %r" % c["text"])
for i, c in enumerate(corpus["diff"]):
    if diff(c["before"], c["after"]) != c["expected"]:
        failures.append("diff %d" % i)
for c in corpus["loc"] + corpus["locDefects"]:
    if loc(c["path"], c["data"]) != c["expected"]:
        failures.append("loc %s %r" % (c["path"], c["data"]))

print(json.dumps({"failures": failures}))
sys.exit(1 if failures else 0)

#!/usr/bin/env python3
"""🐍️ Independent model of RFC 6901 pointers and the owner publication subtable (stdlib only).

usage: 🐍️.py CORPUS.json      (exit 0 when every verdict holds)"""
import json, re, sys

corpus = json.load(open(sys.argv[1], encoding="utf8"))
failures = []
VS = "\ufe0f"


def norm(s):
    return s.replace("\ufe0f", "").replace("\ufe0e", "")



def resolve(d, ptr):
    if not re.match(r"^(?:/(?:[^~]|~[01])*)+$", ptr):
        raise ValueError("invalid")
    cur = d
    for tok in ptr[1:].split("/"):
        key = tok.replace("~1", "/").replace("~0", "~")
        if isinstance(cur, dict) and key in cur:
            cur = cur[key]
        elif isinstance(cur, list) and re.match(r"^(0|[1-9]\d*)$", key) and int(key) < len(cur):
            cur = cur[int(key)]
        else:
            raise KeyError("missing")
    return cur


for c in corpus["pointerOk"]:
    if resolve(corpus["document"], c["pointer"]) != c["expected"]:
        failures.append("pointer " + c["pointer"])
for p in corpus["pointerMissing"]:
    try:
        resolve(corpus["document"], p)
        failures.append("found " + p)
    except KeyError:
        pass
for p in corpus["pointerInvalid"]:
    try:
        resolve(corpus["document"], p)
        failures.append("valid " + p)
    except ValueError:
        pass


def manifest(text):
    active = seen = False
    result = None
    for src in re.split(r"\r?\n", text):
        line = src.strip()
        if line.startswith("["):
            active = line == "[package.metadata.semio.publications]"
            if active and seen:
                raise ValueError("repeated")
            seen = seen or active
            continue
        if not active or not line or line.startswith("#"):
            continue
        m = re.match(r'^manifest\s*=\s*"([^"\\]+)"\s*(?:#.*)?$', line)
        if not m or result is not None:
            raise ValueError("exactly one")
        result = m.group(1)
    if seen and not result:
        raise ValueError("missing")
    return result


for c in corpus["tomlOk"]:
    if manifest(c["text"]) != c["expected"]:
        failures.append("toml " + c["name"])
for c in corpus["tomlErrors"]:
    try:
        manifest(c["text"])
        failures.append("toml accepted " + c["name"])
    except ValueError:
        pass

print(json.dumps({"failures": failures}))
sys.exit(1 if failures else 0)

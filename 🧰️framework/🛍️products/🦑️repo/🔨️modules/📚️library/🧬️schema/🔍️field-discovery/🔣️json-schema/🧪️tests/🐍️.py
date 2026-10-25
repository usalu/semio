#!/usr/bin/env python3
"""🐍️ Independent model of the JSON Schema field extraction (stdlib json only).

usage: 🐍️.py CORPUS.json      (exit 0 when every verdict holds)"""
import json, re, sys

corpus = json.load(open(sys.argv[1], encoding="utf8"))
failures = []
VS = "\ufe0f"


def norm(s):
    return s.replace("\ufe0f", "").replace("\ufe0e", "")



def scalar(s):
    if s.get("contentEncoding") == "base64":
        return "bytes"
    if s.get("contentMediaType") == "application/json":
        return "string"
    t, f = s.get("type"), s.get("format")
    if t == "string":
        return "string"
    if t == "boolean":
        return "bool"
    if t == "integer":
        return f if f in ("int32", "uint32", "int64", "uint64") else "int32"
    if t == "number":
        return {"float": "float32", "double": "float64"}.get(f, "float64")
    if isinstance(s.get("$ref"), str):
        return s["$ref"].split("/")[-1]
    if isinstance(s.get("title"), str):
        return s["title"]
    return t if isinstance(t, str) else "unknown"


def prop(p):
    if p.get("type") == "array":
        sc = scalar(p["items"]) if p.get("items") else "unknown"
        fixed = isinstance(p.get("minItems"), int) and not isinstance(p.get("minItems"), bool) and p.get("minItems") == p.get("maxItems")
        return ("fixedList" if fixed else "list", sc)
    ap = p.get("additionalProperties")
    if p.get("type") == "object" and "additionalProperties" in p and ap is not None and ap is not False:
        return ("map", scalar(ap) if isinstance(ap, dict) else "unknown")
    return ("scalar", scalar(p))


def extract(text):
    try:
        doc = json.loads(text)
    except ValueError:
        return {"typeName": "", "fields": []}
    if not isinstance(doc, dict):
        return {"typeName": "", "fields": []}
    props = doc.get("properties") or {}
    req = set(doc["required"]) if isinstance(doc.get("required"), list) else set()
    out = []
    for name, p in props.items():
        p = p or {}
        card, sc = prop(p)
        st = p.get("x-semio-state")
        out.append({"name": name, "optional": name not in req, "cardinality": card, "scalar": sc, "state": st.strip().lower().replace("_", "-") if isinstance(st, str) and st else ""})
    return {"typeName": doc["title"] if isinstance(doc.get("title"), str) else "", "fields": out}


for c in corpus["scalar"]:
    if scalar(c["schema"]) != c["expected"]:
        failures.append("scalar %r" % c["schema"])
for c in corpus["property"]:
    if prop(c["schema"]) != (c["cardinality"], c["scalar"]):
        failures.append("property %r" % c["schema"])
for c in corpus["extract"]:
    if extract(c["text"]) != c["expected"]:
        failures.append("extract: " + c["name"])

print(json.dumps({"failures": failures}))
sys.exit(1 if failures else 0)

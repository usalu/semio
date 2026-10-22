#!/usr/bin/env python3
"""🐍️ Independent model of the collection catalog: validates the schema extension with its own patterns, re-renders the TypeScript catalog and derives the expected Rust items from the corpus catalog (stdlib only).

usage: oracle.py CORPUS.json      (exit 0 when every verdict holds)"""
import json, re, sys

corpus = json.load(open(sys.argv[1], encoding="utf8"))
failures = []
VS = "\ufe0f"


def norm(s):
    return s.replace("\ufe0f", "").replace("\ufe0e", "")


import copy

X = "x-semio-collection-catalog"
IDENT, PASCAL, CAMEL = r"^[A-Za-z_][A-Za-z0-9_]*$", r"^[A-Z][A-Za-z0-9]*$", r"^[a-z][A-Za-z0-9]*$"
KEBAB = r"^[a-z][a-z0-9]*(?:-[a-z0-9]+)*$"
PATH = r"^(?:crate|[a-z_][a-z0-9_]*)(?:::[A-Za-z_][A-Za-z0-9_]*)*$"
OUTPUT = r"^(?:\.\./)*(?:[^/\\.][^/\\]*/)*[^/\\.][^/\\]*$"


def get(d, k):
    return d.get(k) if isinstance(d, dict) else None


def obj(v):
    if not isinstance(v, dict):
        raise ValueError("must be an object")
    return v


def txt(o, k, pat):
    v = o.get(k)
    if not isinstance(v, str) or not re.match(pat, v):
        raise ValueError("%s is invalid" % k)
    return v


def parse(doc):
    root = obj(doc)
    cat = obj(root.get(X))
    rust, ts, out = obj(cat.get("rust")), obj(cat.get("typescript")), obj(cat.get("output"))
    cols = cat.get("collections")
    if not isinstance(cols, list) or not cols:
        raise ValueError("declares no collections")
    entries = []
    for i, c in enumerate(cols):
        e = obj(c)
        lab = obj(e.get("label"))
        if lab.get("article") not in ("a", "an"):
            raise ValueError("article must be a or an")
        if not isinstance(lab.get("en"), str) or not lab["en"] or not isinstance(lab.get("de"), str) or not lab["de"]:
            raise ValueError("needs English and German labels")
        entries.append({"key": txt(e, "key", CAMEL), "variant": txt(e, "variant", PASCAL), "entity": txt(e, "entity", KEBAB), "row": txt(e, "row", PASCAL), "patch": txt(e, "patch", PASCAL), "delta": txt(e, "delta", PASCAL), "rowsField": txt(e, "rowsField", IDENT), "diffField": txt(e, "diffField", IDENT), "label": {"article": lab["article"], "en": lab["en"], "de": lab["de"]}})
    for f in ("key", "variant", "entity", "diffField"):
        if len({e[f] for e in entries}) != len(entries):
            raise ValueError("%s values must be unique" % f)
    p = {"keyType": txt(cat, "keyType", PASCAL), "rowType": txt(cat, "rowType", PASCAL), "labelType": txt(cat, "labelType", PASCAL), "tag": txt(cat, "tag", CAMEL), "content": txt(cat, "content", CAMEL), "headerField": txt(cat, "headerField", IDENT),
         "rust": {k: txt(rust, k, IDENT if k == "canonicalJsonOwner" else PATH) for k in ("snapshot", "diff", "deltas", "rows", "header", "id", "canonicalJsonOwner")}, "typescript": {"rows": txt(ts, "rows", r"^\.{1,2}/.+\.ts$")},
         "output": {"rust": txt(out, "rust", OUTPUT), "typescript": txt(out, "typescript", OUTPUT)}, "collections": entries}
    defs = obj(root.get("$defs"))
    keys, rows = obj(defs.get(p["keyType"])), obj(defs.get(p["rowType"]))
    if keys.get("enum") != [e["key"] for e in entries]:
        raise ValueError("enum must list the catalog keys in order")
    if not isinstance(rows.get("oneOf"), list) or len(rows["oneOf"]) != len(entries):
        raise ValueError("oneOf must hold one alternative per collection")
    for i, alt in enumerate(rows["oneOf"]):
        props = obj(obj(alt).get("properties"))
        if obj(props.get(p["tag"])).get("const") != entries[i]["key"]:
            raise ValueError("must tag " + entries[i]["key"])
        ref = obj(props.get(p["content"])).get("$ref")
        if not isinstance(ref, str) or not ref.endswith("/$defs/" + entries[i]["row"]):
            raise ValueError("must reference the %s row" % entries[i]["row"])
    return p


def apply(doc, ch):
    keys = ch["path"].split(".")
    cur = doc
    for k in keys[:-1]:
        cur = cur[int(k)] if isinstance(cur, list) else cur[k]
    last = keys[-1]
    if ch["op"] == "delete":
        del cur[last]
    elif isinstance(cur, list):
        cur[int(last)] = ch["value"]
    else:
        cur[last] = ch["value"]
    return doc


p = parse(corpus["document"])
if p != corpus["parsed"]:
    failures.append("parsed")
for c in corpus["invalid"]:
    try:
        parse(apply(copy.deepcopy(corpus["document"]), c["patch"]))
        failures.append("accepted " + c["name"])
    except ValueError as e:
        if c["message"].split(" ")[-1] not in str(e) and c["message"].replace("collection catalog ", "") not in str(e) and c["message"] not in str(e):
            failures.append("message " + c["name"] + ": " + str(e))
for v in corpus["notDocuments"]:
    try:
        parse(v)
        failures.append("accepted non-document")
    except ValueError:
        pass
V = "️"
c = p["collections"]
K, R = p["keyType"], p["rowType"]
for text, owners in ((corpus["renderTs"], corpus["renderTsOwners"]),):
    by = {}
    for e in c:
        by.setdefault(owners[e["row"]], set()).add(e["row"])
    lines = text.split("\n")
    imports = [l for l in lines if l.startswith("import type")]
    if imports != ['import type { %s } from "%s";' % (", ".join(sorted(by[s])), s) for s in sorted(by)]:
        failures.append("ts imports")
    if "export type %s = %s;" % (K, " | ".join(json.dumps(e["key"]) for e in c)) not in text:
        failures.append("ts key union")
    for e in c:
        if "  | { %s: %s; %s: %s }" % (p["tag"], json.dumps(e["key"]), p["content"], e["row"]) not in text:
            failures.append("ts row alt " + e["key"])
        if "{ key: %s, entity: %s, rowsField: %s, diffField: %s," % (json.dumps(e["key"]), json.dumps(e["entity"]), json.dumps(e["rowsField"]), json.dumps(e["diffField"])) not in text:
            failures.append("ts table " + e["key"])
    if not text.endswith("];\n") or "export const SHAPES:" not in text:
        failures.append("ts table const")
rust = corpus["rust"]
root_text = " ".join(rust["root"])
for e in c:
    for needle in ("%s,\n" % e["variant"], "%s(%s)" % (e["variant"], e["row"])):
        if needle not in root_text:
            failures.append("rust root " + needle)
if "pub const ALL: [%s; %d] = [%s];" % (K, len(c), ", ".join("%s::%s" % (K, e["variant"]) for e in c)) not in rust["root"]:
    failures.append("rust ALL")
if ["root"] + sorted(k for k in rust if k != "root") != ["root"] + sorted(["🪪" + V + "key-descriptors", "🔎" + V + "key-base-access", "🧾" + V + "row-operations", "🔁" + V + "row-conversions"]):
    failures.append("rust children")

print(json.dumps({"failures": failures}))
sys.exit(1 if failures else 0)

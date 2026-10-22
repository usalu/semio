#!/usr/bin/env python3
"""🐍️ Independent oracle of the inference stamp laws: classifies implementation rows by the declared protocol with plain Python (no shared code with the module under test) and admits or refuses protocols.

usage: oracle.py classify SCHEMA.json ROWS.json    prints the classified rows as JSON
       oracle.py admit PROTOCOL.json              prints {"ok": true} or {"error": "..."}"""
import json, re, sys

STAMP = re.compile(r"^semio\.graph-inferrer\.[a-z][a-z0-9]*(?:-[a-z0-9]+)*/[1-9][0-9]*$")
ID = re.compile(r"^[a-z][a-z0-9]*(?:-[a-z0-9]+)*$")


def classify(protocol, rows):
    out = []
    for row in rows:
        path = row["path"]
        if any(f in path for f in protocol["none"]) or any(f in path for g in protocol["inferrers"].values() for f in g["sources"]):
            continue
        out.append(row)
    for id_, group in protocol["inferrers"].items():
        out.append({"kind": "stamp", "path": "inferrer:" + id_, "fact": group["stamp"], "scope": group["scope"]})
    return out


def admit(protocol):
    keys = {"version", "scopes", "inferrers", "none"}
    if not isinstance(protocol, dict) or set(protocol) != keys:
        return "keys"
    if protocol["version"] != 1:
        return "version"
    if len(set(protocol["scopes"])) != len(protocol["scopes"]):
        return "scopes"
    seen = {}
    for f in protocol["none"]:
        if not isinstance(f, str) or len(f) < 2 or f in seen:
            return "fragment " + str(f)
        seen[f] = "none"
    if not protocol["inferrers"]:
        return "inferrers"
    for id_, row in protocol["inferrers"].items():
        if set(row) != {"stamp", "scope", "sources"}:
            return "inferrer keys " + id_
        if not ID.match(id_) or not STAMP.match(row["stamp"]) or row["stamp"].split("/")[0] != "semio.graph-inferrer." + id_:
            return "stamp " + id_
        if row["scope"] not in protocol["scopes"] or not row["sources"]:
            return "scope " + id_
        for f in row["sources"]:
            if not isinstance(f, str) or len(f) < 2 or f in seen:
                return "fragment " + str(f)
            seen[f] = id_
    return None


if __name__ == "__main__":
    if sys.argv[1] == "classify":
        protocol = json.load(open(sys.argv[2], encoding="utf8"))["definitions"]["Protocol"]["const"]
        print(json.dumps(classify(protocol, json.load(open(sys.argv[3], encoding="utf8"))), ensure_ascii=False))
    else:
        error = admit(json.load(open(sys.argv[2], encoding="utf8")))
        print(json.dumps({"ok": True} if error is None else {"error": error}))

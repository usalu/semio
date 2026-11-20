#!/usr/bin/env python3
"""Independent model of the json-duplicate-key law (Python standard library only, cross-platform).

It shares only JSON data with the TypeScript law, never code: the standard `json` parser reports every object through `object_pairs_hook`, which sees the key pairs before they collapse into a dict, so a repeated key is found by the parser itself instead of a hand-written scanner. Paths are rebuilt from the nested results.
usage:
  python3 oracle.py fixtures FIXTURES.json        (JSON list of {name, duplicates:[{path,key}]} per case that has an oracle)
  python3 oracle.py files ROOT LIST.json          (JSON object of path -> [{path,key}] for every listed file with a repeated key)"""
import json, re, sys
from pathlib import Path

IDENTIFIER = re.compile(r"^[A-Za-z_][A-Za-z0-9_]*$")

def segment(key):
    return "." + key if IDENTIFIER.match(key) else "[" + json.dumps(key, ensure_ascii=False) + "]"

class Node:
    def __init__(self, duplicates):
        self.duplicates = duplicates

def nested(value, prefix):
    if isinstance(value, Node):
        return [(prefix + path, key) for path, key in value.duplicates]
    if isinstance(value, list):
        return [row for index, item in enumerate(value) for row in nested(item, prefix + "[" + str(index) + "]")]
    return []

def hook(items):
    seen, found = set(), []
    for key, value in items:
        if key in seen:
            found.append((segment(key), key))
        seen.add(key)
        found.extend(nested(value, segment(key)))
    return Node(found)

def duplicates(text):
    root = json.loads(text.lstrip("﻿"), object_pairs_hook=hook)
    return sorted(({"path": "$" + path, "key": key} for path, key in nested(root, "")), key=lambda row: (row["path"], row["key"]))

if __name__ == "__main__":
    if sys.argv[1] == "fixtures":
        cases = json.loads(Path(sys.argv[2]).read_text(encoding="utf-8"))["cases"]
        print(json.dumps([{"name": case["name"], "duplicates": duplicates(case["source"])} for case in cases if case.get("oracle", True)]))
    else:
        root, listed = Path(sys.argv[2]), json.loads(Path(sys.argv[3]).read_text(encoding="utf-8"))
        report = {}
        for name in listed:
            try:
                found = duplicates((root / name).read_text(encoding="utf-8"))
            except (OSError, ValueError):
                continue
            if found:
                report[name] = found
        print(json.dumps(report))

#!/usr/bin/env python3
"""🐍️ Independent oracle of the identity of observation rows: two rows are the same row exactly when their kind, path and fact are equal as JSON values (object key order and number spelling do not matter), whatever the size of the fact.

usage: 🐍️.py < {"pairs":[[row,row],...]}   prints {"same":[bool,...]}"""
import json, sys


def canonical(row):
    return json.dumps([row["kind"], row["path"], row["fact"]], sort_keys=True, separators=(",", ":"), ensure_ascii=False)


print(json.dumps({"same": [canonical(a) == canonical(b) for a, b in json.load(sys.stdin)["pairs"]]}))

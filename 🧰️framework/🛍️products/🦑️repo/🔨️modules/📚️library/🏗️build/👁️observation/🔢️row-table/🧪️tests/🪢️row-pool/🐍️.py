#!/usr/bin/env python3
"""🐍️ Independent oracle of the row pool: generates the rows every crate of a synthetic workspace holds and counts the distinct ones (rows are equal exactly when the compact JSON text of kind, path and fact is equal, nested rows included).

usage: 🐍️.py < {"workspaces":[...]}   prints {"workspaces":[{"crates":[[row,...],...],"distinct":n,"distinctNested":m},...]}"""
import json, sys


def text(value):
    return json.dumps(value, separators=(",", ":"), ensure_ascii=False)


def build(spec):
    state = spec["seed"]

    def draw(limit):
        nonlocal state
        state = (state * 1103515245 + 12345) % (2 ** 31)
        return (state >> 8) % limit

    crates, top, nested = [], set(), set()
    for _ in range(spec["crates"]):
        rows = []
        for _ in range(spec["rowsPerCrate"]):
            file, provider = draw(spec["files"]), draw(spec["providers"])
            digest = format(file * 7919 + provider, "064x")
            inner = [{"kind": "file", "path": "/w/n" + str(file) + "_" + str(at), "fact": {"size": file, "mtimeMs": at, "ctimeMs": 3, "ino": 4, "sha256": digest}} for at in range(spec["nested"])]
            row = {"kind": "analysis", "path": "/w/" + "d" * 80 + str(file) + ".ts", "fact": {"provider": "build:provider:v1:" + str(provider), "revision": digest, "source": {"size": file, "mtimeMs": 1, "ctimeMs": 2, "ino": 3, "sha256": digest}, "resultSha256": digest, "nestedObservations": inner}}
            rows.append(row)
            top.add(text(row))
            nested.update(text(item) for item in inner)
        crates.append(rows)
    return {"crates": crates, "distinct": len(top), "distinctNested": len(nested)}


print(json.dumps({"workspaces": [build(spec) for spec in json.load(sys.stdin)["workspaces"]]}, ensure_ascii=False))

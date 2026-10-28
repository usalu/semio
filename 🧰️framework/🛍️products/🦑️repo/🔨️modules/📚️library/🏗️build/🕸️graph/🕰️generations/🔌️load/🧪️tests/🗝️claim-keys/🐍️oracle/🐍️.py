#!/usr/bin/env python3
"""🐍️ Independent oracle of the claim by path of `shapeComputeResultV1`: reads a corpus (a table of rows, the rows of the capture, the owners with the row numbers of their own record, stubs, implementation rows) and prints, for every owner and for the environment, the numbers of the rows the shaping must return, in order. No code is shared with the Bun test; standard library only.

usage: 🐍️.py CORPUS.json      prints {"owners": {name: [uid...]}, "environment": [uid...]}"""
import json, sys


def main():
    corpus = json.load(open(sys.argv[1], encoding="utf8"))
    table, workspace, separator = corpus["table"], corpus["workspaceRoot"], corpus.get("separator", "/")
    key = {}
    def keyof(uid):
        if uid not in key:
            row = table[uid]
            key[uid] = json.dumps([row["kind"], row["path"], row["fact"]], sort_keys=True, separators=(",", ":"), ensure_ascii=False)
        return key[uid]
    capture = corpus["part"]["rows"]
    implementation = corpus["implementation"]
    claimed, owners = set(), {}
    def prefix(root):
        return None if root == "." else workspace + separator + separator.join(root.split("/")) + separator
    def claim(base, roots):
        prefixes = [p for p in (prefix(r) for r in roots if r) if p is not None]
        found = sorted({at for at, uid in enumerate(capture) if any(table[uid]["path"].startswith(p) for p in prefixes)}) if prefixes else []
        known = {keyof(uid) for uid in base}
        rows, added = list(base), set()
        for at in found:
            uid = capture[at]
            k = keyof(uid)
            if k not in known and k not in added:
                added.add(k)
                rows.append(uid)
        for uid in rows:
            claimed.add(keyof(uid))
        return rows
    for owner in corpus["part"]["owners"]:
        owners[owner["name"]] = claim(owner["nodeRows"], [owner["root"], owner.get("nativeRoot")]) + implementation["all"]
    for owner in corpus["part"]["edgeOnly"]:
        stub = corpus["stubs"][owner["name"]]
        owners[owner["name"]] = claim([], [stub["root"], stub.get("nativeRoot")])
    environment = corpus["env"] + [uid for uid in capture if keyof(uid) not in claimed] + implementation["all"] + implementation["environment"]
    print(json.dumps({"owners": owners, "environment": environment}))


main()

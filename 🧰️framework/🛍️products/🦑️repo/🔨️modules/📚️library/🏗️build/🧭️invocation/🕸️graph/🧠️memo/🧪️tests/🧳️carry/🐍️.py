"""🐍 Independent model of the graph-memo carry rule: `python3 -I 🐍️.py <cases.json>` prints the answer of every case as a JSON list.

The shared digests come from the memo policy (`../../🧬️schema/🔣️.json`, `carry.shared`); owner rows are compared as values (key order does not matter), owners in code point order."""
import json, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
SHARED = json.load(open(os.path.join(HERE, "..", "..", "🧬️schema", "🔣️.json"), encoding="utf8"))["definitions"]["Policy"]["const"]["carry"]["shared"]


def answer(before, after, covered):
    for name in SHARED:
        if before.get(name) != after.get(name):
            return "shared:" + name
    for owner in sorted(covered, key=lambda name: [ord(c) for c in name]):
        if owner not in before["owners"] or owner not in after["owners"] or before["owners"][owner] != after["owners"][owner]:
            return "owner:" + owner
    return "same"


cases = json.load(open(sys.argv[1], encoding="utf8"))["cases"]
print(json.dumps([answer(row["from"], row["to"], row["covered"]) for row in cases]))

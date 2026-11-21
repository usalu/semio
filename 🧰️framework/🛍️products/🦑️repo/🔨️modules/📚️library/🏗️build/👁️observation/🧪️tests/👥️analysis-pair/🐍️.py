import hashlib
import json
import sys

fixture = json.load(sys.stdin)
original = fixture["original"]
values = [{"head": original.split()[0]}, {"length": len(original.encode("utf-8"))}]


def sha(text):
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


print(json.dumps({
    "values": values,
    "originalSha256": sha(original),
    "nestedSha256": sha(fixture["nested"]),
    "resultSha256": [sha(json.dumps(value, ensure_ascii=False, separators=(",", ":"))) for value in values],
}))

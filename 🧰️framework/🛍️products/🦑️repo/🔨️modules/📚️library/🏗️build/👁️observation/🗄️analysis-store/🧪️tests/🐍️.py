"""Independent oracle of the key of a stored source analysis: the sha256 of the compact JSON of [domain, provider id, provider revision, path relative to the workspace with forward slashes, content digest]; none for a source outside the workspace or below the minimum size. Reads the fixture file (argv[1]) and prints the keys."""
import hashlib
import json
import os
import sys

fixture = json.load(open(sys.argv[1], encoding="utf8"))
answer = []
for row in fixture["cases"]:
    local = os.path.relpath(row["path"], row["root"])
    if row["bytes"] < fixture["minimumSourceBytes"] or local == "." or local.startswith(".."):
        answer.append(None)
        continue
    text = json.dumps([fixture["domain"], row["provider"], row["revision"], local.replace(os.sep, "/"), row["sha"]], separators=(",", ":"), ensure_ascii=False)
    answer.append(hashlib.sha256(text.encode("utf8")).hexdigest())
print(json.dumps(answer))

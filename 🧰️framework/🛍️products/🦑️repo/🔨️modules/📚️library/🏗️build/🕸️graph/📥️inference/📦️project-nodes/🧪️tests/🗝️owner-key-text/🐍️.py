"""Independent oracle of the key text of an owner record: the file stem (first 24 hex digits of the sha256 of the root), the record key list, its compact JSON text under the record stamp and the sha256 digest of that text. Reads the fixture file (argv[1]) and prints a list of {stem, text, digest}."""
import hashlib
import json
import sys

fixture = json.load(open(sys.argv[1], encoding="utf8"))
answer = []
for owner in fixture["owners"]:
    key = [*owner["head"], owner["root"], owner["json"], owner["selectedTargets"], owner["selectedInputs"]]
    text = json.dumps([fixture["stamp"], None, key], separators=(",", ":"), ensure_ascii=False)
    answer.append({"stem": hashlib.sha256(owner["root"].encode("utf8")).hexdigest()[:24], "text": text, "digest": hashlib.sha256(text.encode("utf8")).hexdigest()})
print(json.dumps(answer, ensure_ascii=False))

#!/usr/bin/env python3
"""🐍️ Independent oracle of the output record: the sha256 of the canonical JSON (sorted keys, no whitespace, no escaping of non-ASCII) of every output of the corpus.

usage: 🐍️.py digest OUTPUTS.json     prints {probe id: digest}"""
import hashlib, json, sys

if __name__ == "__main__":
    outputs = json.load(open(sys.argv[2], encoding="utf8"))
    print(json.dumps({k: hashlib.sha256(json.dumps(v, ensure_ascii=False, separators=(",", ":"), sort_keys=True).encode("utf8")).hexdigest() for k, v in outputs.items()}))

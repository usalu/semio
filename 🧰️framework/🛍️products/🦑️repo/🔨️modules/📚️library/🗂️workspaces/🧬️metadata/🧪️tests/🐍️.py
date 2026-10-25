#!/usr/bin/env python3
"""🐍️ Independent path model of the workspace metadata directories (stdlib os.path).

usage: 🐍️.py CORPUS.json      (exit 0 when every verdict holds)"""
import json, re, sys

corpus = json.load(open(sys.argv[1], encoding="utf8"))
failures = []
VS = "\ufe0f"


def norm(s):
    return s.replace("\ufe0f", "").replace("\ufe0e", "")


import os
for c in corpus["semio"]:
    if os.path.join(c["root"], corpus["constants"]["SEMIO_ROOT_DIR"]) != c["expected"] and os.sep == "/":
        failures.append("semio " + c["root"])
for c in corpus["repo"]:
    if os.path.join(c["root"], corpus["constants"]["SEMIO_ROOT_DIR"], corpus["constants"]["REPO_META_DIR_NAME"]) != c["expected"] and os.sep == "/":
        failures.append("repo " + c["root"])

print(json.dumps({"failures": failures}))
sys.exit(1 if failures else 0)

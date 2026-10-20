#!/usr/bin/env python3
"""🐍️ Independent scan: every `import … from "./x.json"` of the entry must be listed in the fixture, as excluded (presentation) or included.

usage: oracle.py FIXTURE.json REPO_ROOT"""
import json, os, re, sys

fixture = json.load(open(sys.argv[1], encoding="utf8"))
text = open(os.path.join(sys.argv[2], fixture["entry"]), encoding="utf8").read()
directory = os.path.dirname(fixture["entry"])
found = sorted(os.path.normpath(os.path.join(directory, name)) for name in set(re.findall(r'from\s*"(\./[^"]+\.json)"', text)))
listed = sorted(os.path.normpath(path) for path in fixture["excluded"] + fixture["included"])
print("ok" if found == listed else "mismatch %r %r" % (found, listed))

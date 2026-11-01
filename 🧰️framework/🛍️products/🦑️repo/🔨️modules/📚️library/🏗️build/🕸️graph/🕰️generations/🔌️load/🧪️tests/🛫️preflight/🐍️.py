#!/usr/bin/env python3
"""🛫️ Independent oracle of the aggregate preflight: prints the cache suite ids that no target of the caching project runs through `repo ./📜️script.ts test <id>`.

usage: 🐍️.py CACHING_DIR      prints {"missing": [...]}"""
import json, sys, os

root = sys.argv[1]
suites = json.load(open(os.path.join(root, "🗂️suites", "🔣️.json")))["suites"]
targets = json.load(open(os.path.join(root, "📋️project.json")))["targets"]
commands = {t.get("options", {}).get("command") for t in targets.values()}
print(json.dumps({"missing": [row["id"] for row in suites if "repo ./📜️script.ts test " + row["id"] not in commands]}))

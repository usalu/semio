"""Independent oracle of the non-input roots: `python3 -I 🐍️.py <policy.json> <unit-cache schema.json> <workspace>` walks <workspace> with the rule "a directory whose name the policy lists under generatedDirectories is never entered" and prints {"cacheDirectories":[names the unit cache keeps beside its store that must be listed], "members":[sorted relative file paths]}. Standard library only."""
import json
import os
import sys

policy = json.load(open(sys.argv[1], encoding="utf8"))
cache = json.load(open(sys.argv[2], encoding="utf8"))["definitions"]["Policy"]["const"]
skipped = set(policy["generatedDirectories"])
root = sys.argv[3]
members = []
for current, directories, files in os.walk(root):
    directories[:] = [name for name in directories if name not in skipped and not os.path.islink(os.path.join(current, name))]
    for name in files:
        members.append(os.path.relpath(os.path.join(current, name), root).replace(os.sep, "/"))
print(json.dumps({"cacheDirectories": sorted({cache["seal"]["directory"]} - skipped), "members": sorted(members)}, ensure_ascii=False))

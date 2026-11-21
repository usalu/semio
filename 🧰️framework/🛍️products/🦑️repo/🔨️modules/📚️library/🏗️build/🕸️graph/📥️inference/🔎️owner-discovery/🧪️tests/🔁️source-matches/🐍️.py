import glob
import json
import os
import sys

fixture = json.load(sys.stdin)
os.chdir(fixture["root"])
matched = set()
compiled = set()
targets = {}
for scan in fixture["scans"]:
    included = set()
    excluded = set()
    for pattern in scan["patterns"]:
        negative = pattern.startswith("!")
        paths = glob.glob(pattern[1:] if negative else pattern, recursive=True)
        (excluded if negative else included).update(path.replace("\\", "/") for path in paths)
    found = included - excluded
    matched.update(found)
    if scan["compile"] and found:
        compiled.update(found)
        targets[scan["target"]] = sorted(found)
print(json.dumps({"matched": sorted(matched), "compileMatched": sorted(compiled), "compileByTarget": targets}))

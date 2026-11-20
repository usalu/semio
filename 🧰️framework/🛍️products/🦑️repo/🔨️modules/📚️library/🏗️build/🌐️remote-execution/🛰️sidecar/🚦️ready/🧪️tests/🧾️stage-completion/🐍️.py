import json
import sys

for case in json.load(sys.stdin):
    proven = case["exit"] == 0 and case["change"] == "none"
    result = "applied" if proven else "unproven" if case["exit"] == 0 else None
    print(json.dumps([case["name"], result, proven]))

import json, re, sys

pattern = re.compile(r"([\\/]\.cache[\\/]b[\\/]d[\\/]m[\\/]j[\\/])[0-9a-f]{16,64}(?=[\\/])")
fixture = json.load(open(sys.argv[1], encoding="utf8"))
failures = [case["name"] for case in fixture["cases"] if pattern.sub(lambda m: m.group(1) + "*", case["input"]) != case["expected"]]
print(json.dumps({"failures": failures}))

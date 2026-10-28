"""🐍️ Independent oracle of the action environment channel: recomputes from the authored schema and the fixture, with plain Python string and regular expression rules (no shared code with the TypeScript module), which names of every environment row an action receives and whether each inventory name is delivered. usage: oracle.py SCHEMA < FIXTURE"""
import json, re, sys

policy = json.load(open(sys.argv[1], encoding="utf8"))["definitions"]["Policy"]["const"]
fixture = json.load(sys.stdin)
names = {n.upper() for c in policy["classes"] for n in c["names"]}
prefixes = [p.upper() for c in policy["classes"] for p in c["prefixes"]]
suffixes = [x.upper() for c in policy["classes"] for x in c["suffixes"]]
denied = [re.compile(p) for p in policy["denied"]["patterns"]]

def allowed(name, declared):
    up = name.upper()
    if up in declared:
        return True
    return (up in names or any(up.startswith(p) for p in prefixes) or any(up.endswith(x) for x in suffixes)) and not any(r.search(up) for r in denied)

print(json.dumps({
    "environments": [sorted(n for n in row["environment"] if allowed(n, {d.upper() for d in row["declared"]})) for row in fixture["environments"]],
    "delivered": [allowed(n, set()) for n in fixture["reads"]["delivered"]],
    "withheld": [allowed(row["name"], set()) for row in fixture["reads"]["withheld"]],
}))

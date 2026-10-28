"""🐍️ Independent oracle of the ambient environment rule: a variable is ambient when its upper-cased name equals an upper-cased `names` entry or starts with an upper-cased `prefixes` entry of the witness policy (Windows compares names without regard to case). usage: python3 -I oracle.py <schema.json> < ["NAME", ...] -> [bool, ...]"""
import json, sys
policy = json.load(open(sys.argv[1], encoding="utf8"))["definitions"]["Policy"]["const"]["ambientEnvironment"]
names = {name.upper() for name in policy["names"]}
prefixes = [prefix.upper() for prefix in policy["prefixes"]]
def ambient(name):
    upper = name.upper()
    return upper in names or any(upper.startswith(prefix) for prefix in prefixes)
json.dump([ambient(name) for name in json.load(sys.stdin)], sys.stdout)

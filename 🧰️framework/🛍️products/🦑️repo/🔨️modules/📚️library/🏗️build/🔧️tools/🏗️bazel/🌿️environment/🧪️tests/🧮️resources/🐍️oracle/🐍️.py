"""🐍️ Independent oracle of the keyed command environment: recomputes the kept names of every fixture environment from the two authored schemas with plain Python string rules (no shared code with the TypeScript module)."""
import json, sys

env_schema = json.load(open(sys.argv[1], encoding="utf8"))
witness_schema = json.load(open(sys.argv[2], encoding="utf8"))
diagnostic = set(env_schema["definitions"]["Policy"]["const"]["diagnostic"])
ambient = witness_schema["definitions"]["Policy"]["const"]["ambientEnvironment"]
fixture = json.load(sys.stdin)

def dropped(name):
    return name in diagnostic or name in ambient["names"] or any(name.startswith(prefix) for prefix in ambient["prefixes"])

print(json.dumps([sorted(name for name in row["environment"] if not dropped(name)) for row in fixture["environments"]]))

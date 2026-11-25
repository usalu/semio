"""🚛️ Independent Python oracle of the beyond-host rule (standard library only): a unit is beyond a machine when max(carving entry measuredMegabytes for its (crate, target) else 0, learned peak else 0) is greater than the final budget of the active worker class and the machine's megabytes are at most the class host megabytes. usage: 🐍️.py <schema.json> <vectors.json> prints a JSON list of booleans, one per case."""
import json, sys

schema = json.load(open(sys.argv[1]))
policy = schema["definitions"]["Policy"]["const"]["unitCeiling"]
cls = policy["classes"][policy["workerClass"]]
vectors = json.load(open(sys.argv[2]))
carving = {(e["crate"], e["target"]): e["measuredMegabytes"] for e in policy["carving"]}
print(json.dumps([max(carving.get((c["crate"], c["target"]), 0), c["learned"] or 0) > cls["finalMegabytes"] and c["host"] <= cls["hostMegabytes"] for c in vectors["cases"]]))

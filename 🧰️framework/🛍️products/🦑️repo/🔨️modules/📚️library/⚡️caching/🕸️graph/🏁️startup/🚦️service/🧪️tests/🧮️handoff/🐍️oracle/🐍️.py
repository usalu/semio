"""🚦️ Independent Python oracle of the daemon handoff admission (standard library only): it classifies a wire as over the byte bound, not JSON, not a record with exactly the keys birth and witness, or well formed up to the birth descriptor. Requests arrive as JSON on stdin."""
import json, sys


def classify(wire, maximum):
    if not isinstance(wire, str) or len(wire.encode("utf8")) > maximum:
        return "bound"
    try:
        value = json.loads(wire)
    except ValueError:
        return "syntax"
    if not isinstance(value, dict) or ",".join(sorted(value)) != "birth,witness":
        return "closed"
    return "birth"


request = json.load(sys.stdin)
print(json.dumps([classify(wire, request["maximumBytes"]) for wire in request["wires"]]))

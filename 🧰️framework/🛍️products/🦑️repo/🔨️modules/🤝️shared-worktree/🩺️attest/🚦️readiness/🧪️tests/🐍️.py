"""🚦️ Replays fleet readiness fixtures with the third-party JSON Schema validator."""
import json
from pathlib import Path

from jsonschema import Draft7Validator

test_dir = Path(__file__).parent
fixture = json.loads((test_dir / "🔣️.json").read_text(encoding="utf-8"))
ready = test_dir.parents[3] / "📚️library/🏗️build/🌐️remote-execution/🛰️sidecar/🚦️ready"
receipt_schema = json.loads((ready / "🧬️schema/🔣️.json").read_text(encoding="utf-8"))
validator = Draft7Validator({"$ref": "#/definitions/Status", "definitions": receipt_schema["definitions"]})
tree = json.loads((ready / "🌲️process-tree/🧬️schema/🔣️.json").read_text(encoding="utf-8"))
window = tree["definitions"]["Policy"]["const"]["foregroundFreshMilliseconds"]

for case in fixture["cases"]:
    patch = case["patch"]
    origin = case["origin"]
    doc = {**fixture["receipt"], "origin": origin, **patch} if patch is not None else None
    if not validator.is_valid(doc) or doc["origin"] != origin:
        observed = ["missing", None, None, None]
    else:
        age = fixture["nowMs"] - doc["updatedMs"]
        freshness = "future" if age < 0 else "stale" if age > window else "fresh"
        ready_now = doc["ready"] and doc["state"] == "ready" and doc["stage"] is None and doc["refusal"] is None
        observed = [freshness, ready_now if freshness == "fresh" else None, doc["state"], doc["stage"]]
    assert observed == case["expected"], (origin, observed, case["expected"])
print(f'{len(fixture["cases"])} readiness cases matched')

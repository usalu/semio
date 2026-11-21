"""📌️ Replays deterministic pin publication waves and validates each pin with JSON Schema."""
import json
from pathlib import Path

from jsonschema import Draft7Validator

tests = Path(__file__).parent
fixture = json.loads((tests / "🔣️.json").read_text(encoding="utf-8"))
schema = json.loads((tests.parents[1] / "🧬️schema/🔣️.json").read_text(encoding="utf-8"))
validator = Draft7Validator({"$ref": "#/definitions/PinFile", "definitions": schema["definitions"]})

waves = [fixture["requests"][0], sum(fixture["requests"][1:], [])]
held = set()
published = []
for wave in waves:
    held.update(wave)
    doc = {"version": 1, "holder": "test-held", "pid": 42, "expiresAtMs": 3600000,
           "digests": [char * 64 for char in sorted(held)]}
    validator.validate(doc)
    published.append([digest[0] for digest in doc["digests"]])

assert published == fixture["publications"]
assert published[-1] == fixture["final"]
failed = fixture["failedWrite"]
held.update(failed["request"])
held.update(failed["following"])
assert sorted(held) == failed["expectedFinal"]
print("pin publication waves matched")

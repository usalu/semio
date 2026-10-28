"""Independent model of the sectional digest of a declared JSON data file: the sha256 of the canonical JSON (keys in code point order, no whitespace, no ASCII escapes) of the document without the ignored top level sections; text that is no JSON object digests as `!unparsable:` plus the sha256 of the text. One JSON document on stdin: {"cases": [{"text": s, "ignore": [...]}]} -> {"digests": [...]}."""
import hashlib
import json
import sys


def digest(text, ignore):
    try:
        value = json.loads(text)
    except ValueError:
        return "!unparsable:" + hashlib.sha256(text.encode("utf8")).hexdigest()
    if not isinstance(value, dict):
        return "!unparsable:" + hashlib.sha256(text.encode("utf8")).hexdigest()
    kept = {key: item for key, item in value.items() if key not in ignore}
    return hashlib.sha256(json.dumps(kept, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode("utf8")).hexdigest()


if __name__ == "__main__":
    request = json.load(sys.stdin)
    print(json.dumps({"digests": [digest(case["text"], case["ignore"]) for case in request["cases"]]}))

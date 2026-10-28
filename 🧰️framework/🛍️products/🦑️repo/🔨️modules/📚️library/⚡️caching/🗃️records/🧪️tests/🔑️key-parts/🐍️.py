#!/usr/bin/env python3
"""🐍️ Independent oracle of the record key: SHA-256 over the namespace and every input, each written as `<byte length>:` and its bytes (a text as WHATWG UTF-8, a lone surrogate becomes U+FFFD), so no two input lists collide. The generated cases are expanded here, so the test replays the very cases the model saw.

usage: 🐍️.py < fixture.json   prints {"cases":[...expanded cases...],"keys":[hex...]}"""
import hashlib, json, sys


def encode(text):
    return text.encode("utf-16", "surrogatepass").decode("utf-16", "replace").encode("utf-8")


def generate(spec):
    state = spec["seed"]

    def draw(limit):
        nonlocal state
        state = (state * 1103515245 + 12345) % (2 ** 31)
        return (state >> 8) % limit

    alphabet = ["a", "7", ":", "é", "東", "😀", "\ud800", "\udc00", "🧰️", "\u0000"]
    cases = []
    for number in range(spec["cases"]):
        parts = []
        for _ in range(draw(spec["maximumParts"] + 1)):
            if draw(3) == 0:
                parts.append({"hex": bytes(draw(256) for _ in range(draw(spec["maximumLength"] + 1))).hex()})
            else:
                parts.append({"text": "".join(alphabet[draw(len(alphabet))] for _ in range(draw(spec["maximumLength"] + 1)))})
        cases.append({"name": "generated " + str(number), "namespace": "".join(alphabet[draw(len(alphabet))] for _ in range(draw(6))), "inputs": parts})
    return cases


def key(case):
    chunks = [encode(case["namespace"])] + [encode(part["text"]) if "text" in part else bytes.fromhex(part["hex"]) for part in case["inputs"]]
    data = b"".join(str(len(chunk)).encode() + b":" + chunk for chunk in chunks)
    return hashlib.sha256(data).hexdigest()


fixture = json.load(sys.stdin)
cases = fixture["cases"] + generate(fixture["generated"])
print(json.dumps({"cases": cases, "keys": [key(case) for case in cases]}, ensure_ascii=True))

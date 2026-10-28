#!/usr/bin/env python3
"""🐍️ Independent oracle of the SHA-256 helpers: Python's hashlib over the UTF-8 bytes of each text (a lone surrogate becomes U+FFFD, the WHATWG encoding rule that `update(text)` follows), over the raw bytes, and over each text followed by the delimiter; the first byte of every text digest. The generated run is expanded here, so the test replays the very texts the model saw.

usage: 🐍️.py < fixture.json   prints {"order":[[sign..]],"generatedOrder":[[sign..]],"views":[text..],"texts":[hex...],"firstBytes":[n...],"bytes":[hex...],"delimited":[hex...],"generated":{"texts":[...],"digests":[hex...]}}"""
import hashlib, json, sys


def encode(text):
    return text.encode("utf-16", "surrogatepass").decode("utf-16", "replace").encode("utf-8")


def digest(data):
    return hashlib.sha256(data).hexdigest()


def generate(spec):
    state = spec["seed"]

    def draw(limit):
        nonlocal state
        state = (state * 1103515245 + 12345) % (2 ** 31)
        return (state >> 8) % limit

    alphabet = ["a", "b", "/", ".", "\u0000", "é", "東", "ÿ", "😀", "\ud800", "\udc00", "🧰️", "�", " "]
    return ["".join(alphabet[draw(len(alphabet))] for _ in range(draw(spec["maximumLength"] + 1))) for _ in range(spec["texts"])]


def order(texts):
    encoded = [encode(text) for text in texts]
    return [[(a > b) - (a < b) for b in encoded] for a in encoded]


fixture = json.load(sys.stdin)
generated = generate(fixture["generated"])
print(json.dumps({
    "texts": [digest(encode(row["value"])) for row in fixture["texts"]],
    "firstBytes": [hashlib.sha256(encode(row["value"])).digest()[0] for row in fixture["texts"]],
    "bytes": [digest(bytes.fromhex(row["hex"])) for row in fixture["bytes"]],
    "delimited": [digest(b"".join(encode(text) + encode(row["delimiter"]) for text in ([row["first"]] if "first" in row else []) + row["texts"])) for row in fixture["delimited"]],
    "order": order(fixture["order"]),
    "generatedOrder": order(generated[:90]),
    "views": [encode(text).decode("utf-8") for text in fixture["views"]],
    "generated": {"texts": generated, "digests": [digest(encode(text)) for text in generated], "firstBytes": [hashlib.sha256(encode(text)).digest()[0] for text in generated]},
}, ensure_ascii=True))

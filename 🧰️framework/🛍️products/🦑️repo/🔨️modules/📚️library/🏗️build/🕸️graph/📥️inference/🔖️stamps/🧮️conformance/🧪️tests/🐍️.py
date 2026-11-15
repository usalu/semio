#!/usr/bin/env python3
"""🐍️ Independent oracle of the conformance stamp: the canonical JSON digest of every corpus output, the stamp a digest of parts makes of a declared stamp and the direct imports of a source text, with plain Python (no shared code with the module under test).

usage: 🐍️.py digest OUTPUTS.json        prints {case: sha256 of the canonical JSON}
       🐍️.py stamp DECLARED PARTS.json  prints the stamp
       🐍️.py imports FILE               prints the sorted direct import specifiers"""
import hashlib, json, re, sys


def canonical(value):
    return json.dumps(value, ensure_ascii=False, separators=(",", ":"), sort_keys=True)


def digest(value):
    return hashlib.sha256(canonical(value).encode("utf8")).hexdigest()


SPECIFIERS = [
    re.compile(r"""(?:^|[\s;{}(])(?:import|export)\b[^"'`;]*?\bfrom\s*["']([^"']+)["']"""),
    re.compile(r"""(?:^|[\s;{}(])import\s*["']([^"']+)["']"""),
    re.compile(r"""\bimport\s*\(\s*["']([^"']+)["']\s*\)"""),
    re.compile(r"""\brequire\s*\(\s*["']([^"']+)["']\s*\)"""),
]

if __name__ == "__main__":
    if sys.argv[1] == "digest":
        print(json.dumps({k: digest(v) for k, v in json.load(open(sys.argv[2], encoding="utf8")).items()}))
    elif sys.argv[1] == "stamp":
        print(sys.argv[2] + "+" + digest(json.load(open(sys.argv[3], encoding="utf8")))[:32])
    else:
        text = open(sys.argv[2], encoding="utf8").read()
        print(json.dumps(sorted({m.group(1) for pattern in SPECIFIERS for m in pattern.finditer(text)})))

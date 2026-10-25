"""🐍️ Independent model of the indented JSON text (stdlib only): the exact text `json.dumps` writes for a value with the declared indent, which must equal the text of the streaming writer byte for byte.

usage: 🐍️.py text CORPUS   CORPUS = {"cases": [{"name", "indent", "value"}]}; prints one JSON text per case as a JSON array of strings"""
import json, sys

if __name__ == "__main__":
    if sys.argv[1] != "text":
        raise SystemExit("unknown command")
    corpus = json.load(open(sys.argv[2], encoding="utf8"))
    print(json.dumps([json.dumps(case["value"], indent=case["indent"] or None, ensure_ascii=False, separators=(",", ":") if not case["indent"] else None) for case in corpus["cases"]], ensure_ascii=False))

#!/usr/bin/env python3
"""Independent oracle of the output contract: which argument lists ask for machine output and whether a result stream parses as JSON.

usage: 🐍️.py < request.json > answer.json
The request is {"requested": [[args...]], "documents": ["text"...]}. It reads only the contract protocol and shares no code with the TypeScript implementation; documents are parsed with the standard `json` module.
"""
import json
import os
import sys

PROTOCOL = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "🔣️.json")
with open(PROTOCOL, encoding="utf-8") as handle:
    CONTRACT = json.load(handle)


def requested(args):
    head = args[: args.index(CONTRACT["separator"])] if CONTRACT["separator"] in args else args
    return any(argument == CONTRACT["machineFlag"] for argument in head)


def parses(text):
    lines = [line for line in text.split("\n") if line != ""]
    try:
        for line in lines:
            json.loads(line)
    except ValueError:
        return False
    return len(lines) > 0


request = json.load(sys.stdin)
json.dump({"requested": [requested(args) for args in request["requested"]], "documents": [parses(text) for text in request["documents"]]}, sys.stdout)

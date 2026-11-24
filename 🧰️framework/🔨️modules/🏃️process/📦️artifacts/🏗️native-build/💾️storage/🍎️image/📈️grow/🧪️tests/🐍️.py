"""Independent oracle of the native image grow laws: python3 -I oracle.py < cases.json > result.json"""
import json
import sys

INITIAL = 67108864
TRAILER = 40960
UNIT = 1048576


def admission(case):
    if case["targetBytes"] <= case["currentBytes"]:
        return "already"
    free = case["hostFreeBytes"]
    if free is not None and free < case["targetBytes"] + INITIAL - case["currentBytes"]:
        return "host-short"
    return "grow"


def receipt(case):
    fs = case["filesystemBytes"]
    return {"size": fs + TRAILER, "maximumPhysicalBytes": fs + UNIT}


cases = json.load(sys.stdin)
json.dump({"admission": [admission(c) for c in cases["admission"]], "receipt": [receipt(c) for c in cases["receipt"]]}, sys.stdout)

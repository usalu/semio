"""🪓️ Independent Python oracle of the ceiling-cancel observation law (standard library only). A unit supplies a forecast observation when it is not cached and either exited 0 or was cancelled by the warden above its runtime ceiling: exit 143 (128 + SIGTERM) and a peak, rounded up to whole MiB, above the budget of its kind (final when the crate types of its key hold cdylib, bin, dylib or staticlib, library otherwise) in the active worker class or above the carving budget of its (crate, target) when it is listed. Usage: oracle.py <admission schema> <corpus>; prints the list of `recorded` verdicts, one per case."""
import json, math, sys

schema = json.load(open(sys.argv[1], encoding="utf8"))
corpus = json.load(open(sys.argv[2], encoding="utf8"))
policy = schema["definitions"]["Policy"]["const"]["unitCeiling"]
budget = policy["classes"][policy["workerClass"]]

def carving(crate, target):
    for entry in policy["carving"]:
        if entry["crate"] == crate and entry["target"] == target:
            return entry["budgetMegabytes"]
    return 0

def recorded(case):
    if case["cache"]:
        return False
    if case["exit"] == 0:
        return True
    parts = case["key"].split("|")
    identity, target, types = parts[0], parts[1], parts[3]
    crate = identity.split("@", 1)[1]
    final = any(kind in ("cdylib", "bin", "dylib", "staticlib") for kind in types.split(","))
    own = budget["finalMegabytes"] if final else budget["libraryMegabytes"]
    return case["exit"] == 143 and case["peakRssBytes"] > 0 and math.ceil(case["peakRssBytes"] / 1048576) > max(own, carving(crate, target))

print(json.dumps([recorded(case) for case in corpus["cases"]]))

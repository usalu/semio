import json
import sys


def expect(path):
    fixture = json.load(open(path, encoding="utf-8"))
    out = {"guard": {}, "origin": {}, "decision": {}}
    for row in fixture["guard"]:
        run = row["run"]
        if "throws" in run:
            out["guard"][row["id"]] = {"status": None, "reason": None, "rethrows": True} if row["aborted"] else {"status": "deferred", "reason": "capture-failed", "rethrows": False}
        else:
            out["guard"][row["id"]] = {"status": run["returns"], "reason": None, "rethrows": False}
    for row in fixture["origin"]:
        out["origin"][row["id"]] = row["pass"] if row["shared"] is None else min(row["shared"], row["pass"])
    for row in fixture["decision"]:
        confirmed = row["owner"] if row["replay"] == "match" else row["walk"]
        out["decision"][row["id"]] = "ready" if confirmed == row["owner"] else "custody-changed"
    return out


def caps(descriptor_schema):
    schema = json.load(open(descriptor_schema, encoding="utf-8"))
    return schema["definitions"]["RelativeCreateProtocol"]["const"]["maximumFileBytes"]


command = sys.argv[1]
if command == "expect":
    print(json.dumps(expect(sys.argv[2])))
elif command == "cap":
    print(caps(sys.argv[2]))
else:
    raise SystemExit("usage: followup.oracle.py expect FIXTURE | cap DESCRIPTOR_SCHEMA")

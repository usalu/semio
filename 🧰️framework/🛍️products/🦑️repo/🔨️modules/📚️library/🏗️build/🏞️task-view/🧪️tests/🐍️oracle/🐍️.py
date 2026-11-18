import hashlib
import json
import sys


def digest_files(paths):
    out = []
    for path in paths:
        with open(path, "rb") as handle:
            out.append(hashlib.sha256(handle.read()).hexdigest())
    return out


def inputs_digest(rows):
    parts = []
    for row in rows:
        value = "link:" + row["link"] if "link" in row else row.get("digest", "")
        parts.append(row["path"] + "\0" + value + "\n")
    parts.sort()
    return hashlib.sha256("".join(parts).encode("utf-8")).hexdigest()


def select(rows, now, bounds):
    removable = [r for r in rows if not r["leased"] and now - r["lastUsedMs"] >= bounds["minimumIdleMilliseconds"]]
    removable.sort(key=lambda r: (r["lastUsedMs"], r["hash"]))
    gone = {r["hash"] for r in removable if now - r["lastUsedMs"] > bounds["maximumIdleMilliseconds"]}
    count = len(rows) - len(gone)
    total = sum(r["bytes"] for r in rows if r["hash"] not in gone)
    for r in removable:
        if count <= bounds["maximumViews"] and total <= bounds["maximumLogicalBytes"]:
            break
        if r["hash"] in gone:
            continue
        gone.add(r["hash"])
        count -= 1
        total -= r["bytes"]
    return [r["hash"] for r in removable if r["hash"] in gone]


request = json.load(sys.stdin)
op = request["op"]
if op == "digest-files":
    result = digest_files(request["paths"])
elif op == "inputs-digest":
    result = inputs_digest(request["rows"])
elif op == "select":
    result = [select(case["rows"], case["now"], case["bounds"]) for case in request["cases"]]
else:
    raise SystemExit("unknown op")
json.dump(result, sys.stdout)

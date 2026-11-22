import json
import sys


def closure(case):
    rows = dict(case["rows"])
    for index in range(case["unrelated"]):
        name = "u%04d" % index
        rows[name] = {"targets": {"build": ["^build"]}, "dependencies": ["u%04d" % (index + 1)] if index + 1 < case["unrelated"] else [], "files": [name + "/src/file.ts"]}
    direct = case.get("direct", False)
    seen, pending = set(), [case["target"]["project"]]
    while pending:
        name = pending.pop()
        if name in seen:
            continue
        seen.add(name)
        if direct:
            continue
        row = rows[name]
        pending.extend(row.get("dependencies", []))
        pending.extend(path.split("/")[0] for path in row.get("files", []))
    return {"owners": sorted(seen), "registry": len(rows)}


print(json.dumps({case["name"]: closure(case) for case in json.load(sys.stdin)}))

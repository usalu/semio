import json, sys

def tree(rows, workspaces, limit, roots):
    by = {r["pid"]: r for r in rows}
    def inside(r):
        seen = set()
        while r is not None and r["pid"] not in seen:
            seen.add(r["pid"])
            if r["pid"] in roots or any(w and w in r["command"] for w in workspaces):
                return True
            r = by.get(r["parent"])
        return False
    chosen = [r for r in rows if inside(r)]
    chosen.sort(key=lambda r: (-(r.get("cpuSeconds", -1)), -(r["memoryKilobytes"] // 1024), r["pid"]))
    return [{"pid": r["pid"], "parent": r["parent"], "megabytes": r["memoryKilobytes"] // 1024, "command": r["command"][:240]} for r in chosen[:limit]]

doc = json.load(sys.stdin)
print(json.dumps(tree(doc["rows"], doc["workspaces"], int(sys.argv[1]), set(doc.get("roots", [])))))

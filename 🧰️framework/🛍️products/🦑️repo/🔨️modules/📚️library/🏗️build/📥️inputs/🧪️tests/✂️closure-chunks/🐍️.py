import hashlib, json, sys

fixture = json.load(open(sys.argv[1], encoding="utf8"))

def chunks(paths):
    out, current = [], []
    for path in paths:
        current.append(path)
        if len(current) >= 16 and hashlib.sha256(path.encode("utf8")).digest()[0] % 128 == 0:
            out.append(current)
            current = []
    if current:
        out.append(current)
    return out

paths = sorted(fixture["roster"])
print(json.dumps({"base": chunks(paths), "added": chunks(sorted(paths + fixture["added"])), "removed": chunks([p for p in paths if p != fixture["removed"]])}, ensure_ascii=False))

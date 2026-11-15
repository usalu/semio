import hashlib
import json
import sys

DOMAIN = "semio-graph-snapshot-v1"


def key(text):
    return text.encode("utf-16-be")


def instance(rows):
    h = hashlib.sha256()
    h.update((DOMAIN + "\n").encode("utf-8"))
    for path, sha in sorted(rows, key=lambda row: key(row[0])):
        h.update((path + "\t" + sha + "\n").encode("utf-8"))
    return h.hexdigest()[:16]


def directories(paths):
    out = set()
    for path in paths:
        parts = path.split("/")[:-1]
        for end in range(1, len(parts) + 1):
            out.add("/".join(parts[:end]))
    return out


def diff(before, after):
    old = dict(before)
    new = dict(after)
    rows = []
    for path in sorted(set(old) | set(new), key=key):
        if path not in new:
            rows.append(["r", 0, path])
        elif path not in old:
            rows.append(["a", 0, path])
        elif old[path] != new[path]:
            rows.append(["m", 0, path])
    if not rows:
        return rows
    was = directories(old)
    now = directories(new)
    rows += [["a", 1, d] for d in sorted(now - was, key=key)]
    rows += [["r", 1, d] for d in sorted(was - now, key=key)]
    return rows


def main():
    request = json.load(sys.stdin)
    if request["command"] == "instance":
        print(json.dumps(instance(request["rows"])))
    elif request["command"] == "diff":
        print(json.dumps(diff(request["before"], request["after"])))
    else:
        raise SystemExit("unknown command")


main()

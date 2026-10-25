import hashlib
import json
import os
import stat
import sys


def canonical(value):
    if isinstance(value, dict):
        return "{" + ",".join(json.dumps(key, ensure_ascii=False) + ":" + canonical(value[key]) for key in sorted(value, key=lambda k: k.encode("utf-16-be"))) + "}"
    if isinstance(value, list):
        return "[" + ",".join(canonical(item) for item in value) + "]"
    return json.dumps(value, ensure_ascii=False, separators=(",", ":"))


def sha(data):
    return hashlib.sha256(data if isinstance(data, bytes) else data.encode("utf-8")).hexdigest()


def projected(path, data, inert):
    if path == "package.json" or path.endswith("/package.json"):
        document = json.loads(data.decode("utf-8"))
        if isinstance(document, dict):
            return canonical({key: value for key, value in document.items() if key not in inert}).encode("utf-8")
    return data


def projection(request):
    rows = sorted(([path, sha(projected(path, text.encode("utf-8"), set(request["inert"])))] for path, text in request["files"].items()), key=lambda row: row[0].encode("utf-8"))
    return sha(json.dumps(rows, ensure_ascii=False, separators=(",", ":")))


def tree(root):
    rows, inodes = [], []
    for current, names, files in os.walk(root, followlinks=False):
        names.sort(key=lambda n: n.encode("utf-8"))
        for name in sorted(names + files, key=lambda n: n.encode("utf-8")):
            path = os.path.join(current, name)
            facts = os.lstat(path)
            portable = os.path.relpath(path, root).replace(os.sep, "/")
            if stat.S_ISLNK(facts.st_mode):
                target = os.readlink(path)
                rows.append([portable, "link", target if not os.path.isabs(target) else "ABS"])
            elif stat.S_ISDIR(facts.st_mode):
                rows.append([portable, "directory", stat.S_IMODE(facts.st_mode)])
            else:
                rows.append([portable, "file", stat.S_IMODE(facts.st_mode), sha(open(path, "rb").read())])
                inodes.append(facts.st_ino)
    return {"digest": sha(json.dumps(rows, ensure_ascii=False, separators=(",", ":"))), "entries": len(rows), "inodes": inodes}


def shared(left, right):
    return len(set(tree(left)["inodes"]) & set(tree(right)["inodes"]))


def main():
    command = sys.argv[1]
    if command == "projection":
        print(projection(json.loads(sys.stdin.read())))
    elif command == "tree":
        row = tree(sys.argv[2])
        print(json.dumps({"digest": row["digest"], "entries": row["entries"]}))
    elif command == "modules":
        rows = {}
        for member in ["."] + sys.argv[3:]:
            path = os.path.join(sys.argv[2], member, "node_modules")
            if os.path.isdir(path):
                row = tree(path)
                rows[member] = [row["digest"], row["entries"]]
        print(json.dumps(rows, sort_keys=True))
    elif command == "shared":
        print(shared(sys.argv[2], sys.argv[3]))
    elif command == "seal":
        print(sha(canonical(json.loads(sys.argv[2]))))
    else:
        raise SystemExit("usage: adopt.oracle.py projection (stdin) | tree DIR | modules ROOT MEMBER... | shared DIR DIR | seal JSON")


main()

"""Independent walker for the mini language of the store law: a file lists `import <relative path>` lines (the `.ts` suffix is optional); the closure of an entry is every file reachable from it, entry included. Usage: walker.py <root> <entry> -> JSON list of the relative paths, sorted."""
import json
import os
import sys


def closure(root, entry):
    seen, todo = set(), [entry]
    while todo:
        rel = todo.pop()
        if rel in seen:
            continue
        seen.add(rel)
        with open(os.path.join(root, rel), encoding="utf8") as handle:
            for line in handle.read().splitlines():
                if line.startswith("import "):
                    name = line[7:].strip()
                    if not name.endswith(".ts"):
                        name += ".ts"
                    todo.append(os.path.normpath(os.path.join(os.path.dirname(rel), name)))
    return sorted(seen)


if __name__ == "__main__":
    print(json.dumps(closure(sys.argv[1], sys.argv[2])))

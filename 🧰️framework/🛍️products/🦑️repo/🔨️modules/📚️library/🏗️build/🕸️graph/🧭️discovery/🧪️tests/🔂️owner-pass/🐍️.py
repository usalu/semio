"""🐍️ Independent model of the native closure of each owner of a generated Cargo workspace: reads the manifests (JSON on stdin: {"root", "manifests": {path: text}, "owners": [root, ...]}), follows every `path = "..."` dependency of every dependency table transitively and prints {owner: sorted crate roots relative to the workspace}."""
import json
import os
import re
import sys

spec = json.load(sys.stdin)
root, manifests = spec["root"], spec["manifests"]
PATH = re.compile(r'^\s*[A-Za-z0-9_-]+\s*=\s*\{[^}]*\bpath\s*=\s*"([^"]+)"', re.M)


def children(directory):
    text = manifests[os.path.join(directory, "Cargo.toml")]
    return [os.path.normpath(os.path.join(directory, path)) for path in PATH.findall(text)]


def closure(directory):
    seen, todo = set(), [directory]
    while todo:
        current = todo.pop()
        if current in seen:
            continue
        seen.add(current)
        todo.extend(children(current))
    return sorted(os.path.relpath(path, root) for path in seen)


print(json.dumps({owner: closure(os.path.join(root, owner)) for owner in spec["owners"]}))

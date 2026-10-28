"""Independent walker of the membership frontier (see `🌿️frontier-walk/🟨️.mjs`): `python3 -I 🐍️.py <root>` prints `{directories:[...], members:[[path, mode]...]}` sorted by path for the tree below <root>. Rules (the TypeScript test passes the same): symbolic links are skipped, a directory is entered unless its name is `node_modules`, `dist` or starts with `ext-`, a file is a member when its name is `project.json`, `Cargo.toml` or ends with `.feature`. Standard library only."""
import json
import os
import sys


def entered(name):
    return name not in ("node_modules", "dist") and not name.startswith("ext-")


def member(name):
    return name in ("project.json", "Cargo.toml") or name.endswith(".feature")


def walk(root):
    directories, members = [], []
    stack = ["."]
    while stack:
        directory = stack.pop()
        directories.append(directory)
        base = root if directory == "." else os.path.join(root, directory)
        for entry in os.scandir(base):
            if entry.is_symlink():
                continue
            path = entry.name if directory == "." else directory + "/" + entry.name
            if entry.is_dir(follow_symlinks=False):
                if entered(entry.name):
                    stack.append(path)
            elif entry.is_file(follow_symlinks=False) and member(entry.name):
                members.append([path, entry.stat(follow_symlinks=False).st_mode])
    return {"directories": sorted(directories), "members": sorted(members)}


print(json.dumps(walk(sys.argv[1]), ensure_ascii=False))

#!/usr/bin/env python3
"""🔎 Independent oracle for the journal hooks: a stat-diff tree walk, the dirty set of journal rows, and jsonschema verdicts (third-party validation) for DirtySetV1.
usage: 🐍️.py walk ROOT | 🐍️.py dirty FIXTURES.json | 🐍️.py validate DOCUMENT.json DEFINITION"""
import json, os, sys

HERE = os.path.dirname(os.path.abspath(__file__))
SCHEMA = json.load(open(os.path.join(HERE, "..", "..", "🧬️schema", "🔣️.json"), encoding="utf-8"))
EXCLUDED = {"node_modules", "dist"}
MEMBER_NAMES = {"project.json", "Cargo.toml"}


def descends(name):
    return name not in EXCLUDED


def member(path):
    base = path.rsplit("/", 1)[-1]
    return base in MEMBER_NAMES or base.endswith(".feature")


def walk(root):
    directories, members = ["."], []
    pending = ["."]
    while pending:
        current = pending.pop()
        base = root if current == "." else os.path.join(root, *current.split("/"))
        for entry in os.scandir(base):
            path = entry.name if current == "." else current + "/" + entry.name
            if entry.is_symlink():
                continue
            if entry.is_dir(follow_symlinks=False):
                if descends(entry.name):
                    directories.append(path)
                    pending.append(path)
            elif entry.is_file(follow_symlinks=False) and member(path):
                members.append([path, os.lstat(os.path.join(root, *path.split("/"))).st_mode])
    return {"directories": sorted(directories), "members": sorted(members)}


def parent(path):
    return path.rsplit("/", 1)[0] if "/" in path else ""


def dirty(rows):
    exact, subtrees = set(), set()
    for kind, directory, path in rows:
        exact.add(path)
        exact.add(parent(path))
        if directory == 1 and kind != "m":
            subtrees.add(path)
    return {"exact": sorted(exact), "subtrees": sorted(subtrees)}


def validate(document, definition):
    from jsonschema import Draft7Validator
    validator = Draft7Validator({"$ref": "#/definitions/" + definition, "definitions": SCHEMA["definitions"]})
    return [error.message for error in validator.iter_errors(document)]


if __name__ == "__main__":
    mode = sys.argv[1]
    if mode == "walk":
        print(json.dumps(walk(sys.argv[2]), ensure_ascii=False))
    elif mode == "dirty":
        fixtures = json.load(open(sys.argv[2], encoding="utf-8"))
        print(json.dumps([dict(name=item["name"], **dirty(item["rows"])) for item in fixtures["dirty"]], ensure_ascii=False))
    elif mode == "validate":
        print(json.dumps(validate(json.load(open(sys.argv[2], encoding="utf-8")), sys.argv[3])))

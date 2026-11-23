#!/usr/bin/env python3
"""📇️ Independent oracle of the child facts of one directory: given the entries a directory really has (name -> "file" | "directory") and recorded facts about names inside it, a fact holds when it equals what the directory has now.

usage: 🐍️.py < {"scenarios":[{"entries":{name:type},"rows":[{"kind","name","fact"}]}]}   prints {"answers":[[bool,...],...]}   (one verdict per row, in order)"""
import json
import sys


def holds(entries, row):
    kind, name, fact = row["kind"], row["name"], row["fact"]
    actual = entries.get(name)
    if kind == "lstat":
        return fact == (actual if actual is not None else "!ENOENT")
    if kind == "exists":
        return fact == (actual is not None)
    if kind == "fileExists":
        return fact == (actual == "file")
    if kind == "directoryExists":
        return fact == (actual == "directory")
    raise ValueError(kind)


def main():
    spec = json.load(sys.stdin)
    answers = [[holds(scenario["entries"], row) for row in scenario["rows"]] for scenario in spec["scenarios"]]
    json.dump({"answers": answers}, sys.stdout)


main()

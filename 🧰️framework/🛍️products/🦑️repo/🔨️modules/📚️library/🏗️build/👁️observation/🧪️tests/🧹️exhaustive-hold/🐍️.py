#!/usr/bin/env python3
"""🧹️ Independent oracle of a complete look: given the names a directory really has and recorded `exists` facts about names inside it, the rows that no longer hold are exactly those whose fact differs from the directory, and a set of rows that contradict each other can never hold as a whole.

usage: 🐍️.py < {"scenarios":[{"entries":[name],"rows":[{"name","fact"}]}]}   prints {"answers":[{"stale":[row index],"holds":bool},...]}"""
import json
import sys


def answer(scenario):
    present = set(scenario["entries"])
    stale = [index for index, row in enumerate(scenario["rows"]) if row["fact"] != (row["name"] in present)]
    facts = {}
    contradicts = any(facts.setdefault(row["name"], row["fact"]) != row["fact"] for row in scenario["rows"])
    return {"stale": stale, "holds": not stale and not contradicts}


def main():
    spec = json.load(sys.stdin)
    json.dump({"answers": [answer(scenario) for scenario in spec["scenarios"]]}, sys.stdout)


main()

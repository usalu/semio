#!/usr/bin/env python3
"""🐍️ Independent oracle of the validation round: a memoized computation that read `files` files is reused while every file still has the version it was computed from. Without a round every use checks the files again; inside a round the verdict on each stored fact (the file and the version it was recorded at, whichever computation recorded it) is reached once, at its first check, and kept until the round ends (an edit during the round is seen by the next round).

usage: 🐍️.py < {"scenarios":[{"files":N,"ops":[["hit"]|["edit",i]|["round"],...]}]}   prints {"answers":[{"plain":[verdict,...],"round":[verdict,...]}]}   (verdict: "reuse" | "recompute", one per hit)"""
import json, sys


def run(scenario, rounds):
    files = scenario["files"]
    version = [0] * files
    memo = None          # (generation, versions the computation read)
    generation = 0
    cache = {}           # (file, recorded version) -> verdict, per round
    out = []
    for op in scenario["ops"]:
        if op[0] == "edit":
            version[op[1]] += 1
        elif op[0] == "round":
            cache = {}
        else:
            if memo is not None:
                holds = True
                for index in range(files):
                    key = (index, memo[1][index])
                    if rounds and key in cache:
                        verdict = cache[key]
                    else:
                        verdict = memo[1][index] == version[index]
                        if rounds:
                            cache[key] = verdict
                    if not verdict:
                        holds = False
                        break
                if holds:
                    out.append("reuse")
                    continue
            generation += 1
            memo = (generation, list(version))
            out.append("recompute")
    return out


print(json.dumps({"answers": [{"plain": run(s, False), "round": run(s, True)} for s in json.load(sys.stdin)["scenarios"]]}))

#!/usr/bin/env python3
"""🐍️ Independent oracle of the toolchain activation: answers the corpus with plain Python set logic."""
import json, sys

corpus = json.load(open(sys.argv[1], encoding="utf-8"))
answers = []
for case in corpus["cases"]:
    named = set()
    for target in case["targets"].values():
        for dependency in target.get("dependsOn", []):
            name = dependency if isinstance(dependency, str) else dependency.get("target", "")
            named.add(name.rsplit(":", 1)[-1])
    answers.append(sorted(tool for tool, contract in corpus["toolchains"].items() if any(name in named for name in contract.get("activatedBy", []))))
print(json.dumps(answers))

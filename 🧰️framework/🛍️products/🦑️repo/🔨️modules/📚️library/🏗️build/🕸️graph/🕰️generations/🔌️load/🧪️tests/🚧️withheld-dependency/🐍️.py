#!/usr/bin/env python3
"""🐍️ Independent model of the withheld test dependency: from an inventory of projects it derives which test projects name a missing project, the rows that withhold them and the English and German words for each row. It shares no code with the Bun test and reads the declared message table from the generation schema.

usage: 🐍️.py FIXTURE.json GENERATION_SCHEMA.json GRAPH_SOURCE.ts      (prints the expected block of every case and the verdict of every assembly law as JSON)"""
import json, re, sys

fixture = json.load(open(sys.argv[1], encoding="utf8"))
messages = json.load(open(sys.argv[2], encoding="utf8"))["definitions"]["Messages"]["const"]
EMPTY = fixture["emptyDigest"]


def words(language, key, fields):
    return re.sub(r"\{([a-z]+)\}", lambda token: str(fields[token.group(1)]), messages[language][key])


def model(case):
    inventory, dangling = case["projects"], []
    for source, project in inventory.items():
        if "type:test" not in project.get("tags", []):
            continue
        names = set()
        for target in project.get("targets", {}).values():
            for item in target.get("inputs", []):
                if isinstance(item, dict) and item.get("input") == "nativeSources":
                    names.update(item["projects"])
        missing = sorted(name for name in names if name not in inventory)
        if missing:
            dangling.append({"source": source, "root": project["root"], "missing": missing})
    rows, text = [], []
    for entry in dangling:
        file = case["configs"].get(entry["source"], entry["root"])
        fields = {"project": entry["source"], "source": file, "target": ", ".join(entry["missing"]), "sha": case["digests"].get(file, EMPTY)[:12]}
        rows.append({"root": entry["root"], "project": entry["source"], "source": file, "sha256": case["digests"].get(file, EMPTY),
                     "reason": words("en", "withheldDependency", fields), "missing": entry["missing"]})
        text.append({"en": words("en", "withheldDependency", fields), "de": words("de", "withheldDependency", fields)})
    withheld = [entry["source"] for entry in dangling]
    return {"dangling": dangling, "withheld": withheld, "remaining": sorted(name for name in inventory if name not in withheld), "unavailable": rows, "text": text}


source = open(sys.argv[3], encoding="utf8").read()


def law(row):
    start = source.find(row["start"])
    if start < 0 or source.find(row["start"], start + 1) >= 0:
        return False
    end = source.find(row["end"], start + len(row["start"]))
    if end < 0:
        return False
    text = source[start:end]
    before = text.find(row["before"])
    return before >= 0 and text.find(row["after"], before + 1) > before


result = {case["id"]: model(case) for case in fixture["cases"]}
result["laws"] = {row["name"]: law(row) for row in fixture["laws"]}
print(json.dumps(result, ensure_ascii=False))

"""💊️ Independent selection oracle of the preflight remedies: from the same authored texts it computes which remedy lines a set of
facts must produce, with its own implementation of the rules (swap-less filesystem, container advice, case folding per
platform, missing tools per platform). usage: python3 -I 🐍️.py <texts.json> <cases.json>"""
import json, sys

texts = json.load(open(sys.argv[1], encoding="utf8"))
cases = json.load(open(sys.argv[2], encoding="utf8"))


def lines(case):
    rows = []
    facts = case["facts"]
    if case["requiredExchange"]:
        if facts["exchange"] != "supported":
            rows.append(texts["exchange"])
            if case["container"]:
                rows.append(texts["exchangeContainer"])
        if facts["caseSensitive"] == "no" and facts["platform"] in texts["caseFolding"]:
            rows.append(texts["caseFolding"][facts["platform"]])
    for tool in case["requiredTools"]:
        if tool not in facts["tools"]:
            row = texts["tools"].get(tool, {}).get(facts["platform"])
            if row:
                rows.append(row)
    out = []
    for row in rows:
        out.append(texts["labels"]["en"] + ": " + row["en"])
        out.append(texts["labels"]["de"] + ": " + row["de"])
    return out


print(json.dumps([lines(case) for case in cases], ensure_ascii=False))

"""🐍️ Independent oracle of the refusal class table: scans the sources named by the table and reports, per message and site, how many times the message occurs and how often a typed constructor of the class precedes it in the statement that raises it (from the nearest `throw ` or `return ` before the text), without sharing a line with the implementation. usage: 🐍️.py <repo root> <refusals.json> <churn schema.json> <vectors.json>; it also classifies the vector texts of failed child output"""
import json
import os
import re
import sys


def main():
    root = sys.argv[1]
    table = json.load(open(sys.argv[2], encoding="utf-8"))["definitions"]["Table"]["const"]
    constructors, window = table["constructors"], table["lookbehind"]
    report = {}
    for row in table["classes"]:
        for message in row["messages"]:
            for site in message["sites"]:
                with open(os.path.join(root, site), encoding="utf-8") as handle:
                    text = handle.read()
                found = typed = marked = start = 0
                while True:
                    index = text.find(message["text"], start)
                    if index < 0:
                        break
                    found += 1
                    before = text[max(0, index - window):index]
                    before = before[max(before.rfind("throw "), before.rfind("return "), 0):]
                    typed += any(item in before for item in constructors)
                    marked += any(item in before for item in row["markers"])
                    start = index + 1
                report[row["id"] + "|" + message["text"] + "|" + site] = {"occurrences": found, "typed": typed, "marked": marked}
    churn = json.load(open(sys.argv[3], encoding="utf-8"))["definitions"]["Protocol"]["const"]
    vectors = json.load(open(sys.argv[4], encoding="utf-8"))["texts"]
    exhausted = [".+?".join(re.escape(part) for part in re.split(r"\{[a-z]+\}", language["exhausted"])) for language in churn["texts"].values()]

    def classify(text):
        for row in table["classes"]:
            for message in row["messages"]:
                if message["text"] and message["text"] in text:
                    return {"class": row["id"], "disposition": row["disposition"]}
        for pattern in exhausted:
            if re.search(pattern, text):
                return {"class": "retries-exhausted", "disposition": "reobserve"}
        return None

    print(json.dumps({"scan": report, "texts": {row["id"]: classify(row["text"]) for row in vectors}}, ensure_ascii=False, sort_keys=True))


main()

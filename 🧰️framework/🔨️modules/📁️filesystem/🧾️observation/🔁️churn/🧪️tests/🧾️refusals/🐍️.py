"""🐍️ Independent oracle of the refusal class table: scans the sources named by the table and reports, per message and site, how many times the message occurs and how often a typed constructor of the class precedes it in the statement that raises it (from the nearest `throw ` or `return ` before the text), without sharing a line with the implementation. it also lists every snapshot- or build-layer throw of the typed constructor whose message no class declares. usage: 🐍️.py <repo root> <refusals.json> <churn schema.json> <vectors.json>; it also classifies the vector texts of failed child output"""
import json
import os
import re
import sys


def split(args):
    parts, depth, quote, current, index = [], 0, None, "", 0
    while index < len(args):
        char = args[index]
        if quote:
            current += char
            if char == "\\":
                index += 1
                current += args[index]
            elif char == quote:
                quote = None
        elif char in "\"'`":
            quote = char
            current += char
        elif char in "([{":
            depth += 1
            current += char
        elif char in ")]}":
            depth -= 1
            current += char
        elif char == "," and depth == 0:
            parts.append(current.strip())
            current = ""
        else:
            current += char
        index += 1
    parts.append(current.strip())
    return parts


def uncatalogued(root, table):
    """Every call of the typed constructor from the `snapshot` or `build` layer with a literal message head whose text no class declares for the file that raises it."""
    declared = [(message["text"], site) for row in table["classes"] for message in row["messages"] for site in message["sites"]]
    found = []
    total = 0
    base = os.path.join(root, "🧰️framework")
    for folder, names, files in os.walk(base):
        names[:] = [name for name in names if name not in ("node_modules", ".git", "target", "dist") and "tests" not in name and not name.startswith("🗑")]
        for name in files:
            if not name.endswith((".ts", ".mjs", ".cjs")) or name.endswith(".d.ts"):
                continue
            path = os.path.join(folder, name)
            with open(path, encoding="utf-8") as handle:
                text = handle.read()
            for call in re.finditer(r"concurrentEditV1\(", text):
                start = depth = call.end()
                depth, quote, end = 1, None, start
                while end < len(text) and depth:
                    char = text[end]
                    if quote:
                        if char == "\\":
                            end += 1
                        elif char == quote:
                            quote = None
                    elif char in "\"'`":
                        quote = char
                    elif char in "([{":
                        depth += 1
                    elif char in ")]}":
                        depth -= 1
                    end += 1
                args = split(text[start:end - 1])
                if len(args) < 4 or args[3] not in ('"snapshot"', '"build"'):
                    continue
                literal = re.match(r'^"((?:[^"\\]|\\.)*)"|^`((?:[^`$\\]|\\.|\$(?!\{))*)', args[2])
                head = None if literal is None else (literal.group(1) if literal.group(1) is not None else literal.group(2))
                if not head:
                    continue
                total += 1
                relative = os.path.relpath(path, root)
                if not any(site == relative and declared_text in head for declared_text, site in declared):
                    found.append(relative + "|" + head)
    return {"sites": total, "missing": sorted(found)}


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

    print(json.dumps({"scan": report, "law": uncatalogued(root, table), "texts": {row["id"]: classify(row["text"]) for row in vectors}}, ensure_ascii=False, sort_keys=True))


main()

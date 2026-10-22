#!/usr/bin/env python3
"""Independent checker of the language law: bilingual catalogs, no assumed language, a ratchet of English prose literals.

usage: checker.py cases < corpus.json > result.json     findings of every corpus case and the prose count of every corpus text
       checker.py scan WORKSPACE ALLOWLIST.json          findings of the repository, one JSON list
       checker.py cut WORKSPACE                          the allowlist of the repository
It shares no code with the TypeScript law; the rules are written down once more here.
"""
import json
import os
import re
import sys

WS = "[ \t\n\r\f\v ]"
LITERAL = "(\"(?:[^\"\\\\\\n]|\\\\.)*\"|'(?:[^'\\\\\\n]|\\\\.)*'|`(?:[^`\\\\]|\\\\.)*`)"
SITES = [
    re.compile("(?:new" + WS + "+|throw" + WS + "+)?\\bError\\(" + WS + "*" + LITERAL),
    re.compile("\\bconsole\\.(?:log|error|warn|info|debug|table|group)\\(" + WS + "*" + LITERAL),
    re.compile("\\b(?:stdout|stderr)\\.write\\(" + WS + "*" + LITERAL),
]
CODEISH = re.compile("[\\\\{}()\\[\\]<>=;]|^" + WS + "*/|\\bimport\\b|\\bexport\\b|\\bfunction\\b|\\breturn\\b")
CODE_WORDS = re.compile("[a-z]{3,}" + WS + "+[a-z]{2,}")
PROSE_WORD = re.compile("[A-Za-z]{3,}")
PROSE_PAIR = re.compile("[A-Za-z]{3,}" + WS + "+[A-Za-z]{2,}")
DEFAULTS = [
    ("en-fallback", re.compile(r"\?\?\s*[\"']en[\"']")),
    ("en-fallback", re.compile(r"\|\|\s*[\"']en[\"']")),
    ("en-return", re.compile(r"\breturn\s*[\"']en[\"']")),
    ("first-language", re.compile(r"\blanguages\s*\[\s*0\s*\]")),
    ("de-or-en", re.compile(r"\?\s*[\"']de[\"']\s*:\s*[\"']en[\"']")),
    ("dashboard-locale", re.compile(r"SEMIO_DASHBOARD_LOCALE")),
]
PLACEHOLDER = re.compile(r"\{([A-Za-z_][A-Za-z0-9_]*)\}")
TAGS = {"en", "de", "fr", "es", "it", "pt", "nl", "pl", "ja", "zh"}
SKIP = {"node_modules", "🧪️tests", "🧫️fixtures", ".git", "target", "dist", ".cache", "🗑️generated"}
OWN = "⚖️laws/🗣️language-catalogs/"
ROOTS = ["🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library", "🧰️framework/🔨️modules/🏃️process"]
SOURCE = re.compile(r"\.(?:ts|mts|mjs|cjs)$")


def holders(text):
    return ",".join(sorted(set(PLACEHOLDER.findall(text))))


def comment(line):
    return re.match(r"(?:\/\/|\/\*|\*)", line.strip()) is not None


def prose(text):
    if CODEISH.search(text) and not CODE_WORDS.search(text):
        return False
    if len([token for token in re.split(WS + "+", text) if token != ""]) < 2:
        return False
    return bool(PROSE_WORD.search(text) and PROSE_PAIR.search(text))


def sites(text):
    count = 0
    for line in text.split("\n"):
        if comment(line):
            continue
        for site in SITES:
            for match in site.finditer(line):
                literal = match.group(1)
                if literal.startswith("`") and "\n" in literal:
                    continue
                if prose(re.sub(r"\$\{[^}]*\}", "XX", literal[1:-1])):
                    count += 1
    return count


def is_dict(value):
    return isinstance(value, dict)


def flat(table, base, into):
    for name, entry in table.items():
        if isinstance(entry, str):
            into[base + "/" + name] = entry
        elif is_dict(entry):
            flat(entry, base + "/" + name, into)


def walk(value, path, file, out):
    if isinstance(value, list):
        for index, entry in enumerate(value):
            walk(entry, path + "/" + str(index), file, out)
        return
    if not is_dict(value):
        return
    keys = list(value.keys())
    names = value.get("languages")
    if isinstance(names, list) and all(isinstance(name, str) for name in names) and any(name.lower() in ("en", "de") for name in names) and names != ["en", "de"]:
        out.append({"rule": "languages-not-en-de", "path": file, "detail": path})
    pair = len(keys) == 2 and "en" in value and "de" in value
    if pair and all(isinstance(entry, str) for entry in value.values()):
        if holders(value["en"]) != holders(value["de"]):
            out.append({"rule": "placeholder-mismatch", "path": file, "detail": path})
        return
    if pair and all(is_dict(entry) for entry in value.values()):
        english, german = {}, {}
        flat(value["en"], "", english)
        flat(value["de"], "", german)
        for key in list(english) + list(german):
            if (key in english) != (key in german):
                out.append({"rule": "table-key-mismatch", "path": file, "detail": path + key})
        for key, text in english.items():
            if key in german and holders(text) != holders(german[key]):
                out.append({"rule": "placeholder-mismatch", "path": file, "detail": path + key})
        return
    tagged = keys and all(key in TAGS for key in keys) and all(isinstance(entry, str) or entry is not None and isinstance(entry, (dict, list)) for entry in value.values())
    if tagged and (("en" in value) != ("de" in value) or any(key not in ("en", "de") for key in keys)):
        out.append({"rule": "leaf-missing-language", "path": file, "detail": path})
    for key, entry in value.items():
        walk(entry, path + "/" + key, file, out)


def assumed_lines(text):
    rows = []
    for index, line in enumerate(text.split("\n")):
        if comment(line):
            continue
        for name, pattern in DEFAULTS:
            if pattern.search(line):
                rows.append({"rule": "default-language", "detail": str(index + 1) + ":" + name})
    return rows


def findings(sources, allowlist):
    out, seen, defaults_seen = [], set(), set()
    for source in sources:
        path, text = source["path"], source["text"]
        if path.endswith(".json"):
            try:
                document = json.loads(text)
            except ValueError:
                continue
            walk(document, "", path, out)
        elif SOURCE.search(path):
            if OWN not in path:
                assumed, tolerated = assumed_lines(text), allowlist["defaults"].get(path, 0)
                if len(assumed) > tolerated:
                    out.extend({"rule": row["rule"], "path": path, "detail": row["detail"]} for row in assumed)
                elif len(assumed) < tolerated:
                    out.append({"rule": "allowance-stale", "path": path, "detail": "defaults allowed=%d actual=%d" % (tolerated, len(assumed))})
                defaults_seen.add(path)
            actual, allowed = sites(text), allowlist["files"].get(path, 0)
            seen.add(path)
            if actual > allowed:
                out.append({"rule": "prose-literal", "path": path, "detail": "allowed=%d actual=%d" % (allowed, actual)})
            elif actual < allowed:
                out.append({"rule": "allowance-stale", "path": path, "detail": "allowed=%d actual=%d" % (allowed, actual)})
    for path in allowlist["files"]:
        if path not in seen:
            out.append({"rule": "allowance-stale", "path": path, "detail": "allowed=%d actual=0" % allowlist["files"][path]})
    for path in allowlist["defaults"]:
        if path not in defaults_seen:
            out.append({"rule": "allowance-stale", "path": path, "detail": "defaults allowed=%d actual=0" % allowlist["defaults"][path]})
    total, assumed_total = sum(allowlist["files"].values()), sum(allowlist["defaults"].values())
    if total > allowlist["ceiling"]:
        out.append({"rule": "ceiling-grown", "path": "allowlist", "detail": "ceiling=%d total=%d" % (allowlist["ceiling"], total)})
    if assumed_total > allowlist["defaultsCeiling"]:
        out.append({"rule": "ceiling-grown", "path": "allowlist", "detail": "defaults ceiling=%d total=%d" % (allowlist["defaultsCeiling"], assumed_total)})
    return sorted(out, key=lambda row: (row["path"], row["rule"], row["detail"]))


def read(workspace):
    out = []
    for root in ROOTS:
        top = os.path.join(workspace, root)
        if not os.path.isdir(top):
            continue
        for current, directories, names in os.walk(top):
            directories[:] = sorted(name for name in directories if name not in SKIP and not os.path.islink(os.path.join(current, name)))
            for name in sorted(names):
                full = os.path.join(current, name)
                if os.path.islink(full) or not os.path.isfile(full) or not (name.endswith(".json") or SOURCE.search(name)):
                    continue
                with open(full, encoding="utf-8") as handle:
                    out.append({"path": os.path.relpath(full, workspace).replace(os.sep, "/"), "text": handle.read()})
    return sorted(out, key=lambda row: row["path"])


def cut(sources):
    files, defaults = {}, {}
    for source in sources:
        if SOURCE.search(source["path"]):
            count = sites(source["text"])
            assumed = 0 if OWN in source["path"] else len(assumed_lines(source["text"]))
            if count > 0:
                files[source["path"]] = count
            if assumed > 0:
                defaults[source["path"]] = assumed
    return {"version": 1, "ceiling": sum(files.values()), "files": files, "defaultsCeiling": sum(defaults.values()), "defaults": defaults}


def main():
    command = sys.argv[1]
    if command == "cases":
        corpus = json.load(sys.stdin)
        json.dump({"cases": [{"name": case["name"], "findings": findings(case["sources"], case["allowlist"])} for case in corpus["cases"]], "prose": [sites(row["text"]) for row in corpus["prose"]]}, sys.stdout, ensure_ascii=False)
    elif command == "scan":
        with open(sys.argv[3], encoding="utf-8") as handle:
            allowlist = json.load(handle)
        json.dump(findings(read(sys.argv[2]), allowlist), sys.stdout, ensure_ascii=False)
    elif command == "cut":
        json.dump(cut(read(sys.argv[2])), sys.stdout, ensure_ascii=False)
    else:
        raise SystemExit("usage: checker.py cases|scan|cut")


if __name__ == "__main__":
    main()

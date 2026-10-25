#!/usr/bin/env python3
"""🐍️ Independent Python oracle for the input precision engine: reads the shared fixture and answers the questions the TypeScript engine also answers (input globs, script command words, JSONC, dispatcher branch facts, Cargo package names, tsconfig include selection), so a language-agnostic fixture is checked by two implementations."""
import json, re, shlex, sys


def glob_regex(pattern):
    out, depth, i = [], 0, 0
    while i < len(pattern):
        c = pattern[i]
        if c == "*":
            if pattern[i + 1:i + 2] == "*":
                if pattern[i + 2:i + 3] == "/":
                    out.append("(?:.*/)?")
                    i += 2
                else:
                    out.append(".*")
                    i += 1
            else:
                out.append("[^/]*")
        elif c == "?":
            out.append("[^/]")
        elif c == "{":
            depth += 1
            out.append("(?:")
        elif c == "}" and depth:
            depth -= 1
            out.append(")")
        elif c == "," and depth:
            out.append("|")
        else:
            out.append(re.escape(c))
        i += 1
    return re.compile("^" + "".join(out) + "$")


def command_words(command):
    tokens = shlex.split(command)
    if not tokens or tokens[0] not in ("bun", "repo"):
        return None
    for index, token in enumerate(tokens[1:], 1):
        if re.search(r"script\.ts$", token):
            return {"script": token, "words": tokens[index + 1:]}
    return None


def jsonc(text):
    out, quote, i = [], False, 0
    while i < len(text):
        c = text[i]
        if quote:
            out.append(c)
            if c == "\\":
                i += 1
                out.append(text[i:i + 1])
            elif c == '"':
                quote = False
        elif c == '"':
            quote = True
            out.append(c)
        elif text[i:i + 2] == "//":
            while i < len(text) and text[i] != "\n":
                i += 1
            continue
        elif text[i:i + 2] == "/*":
            i = text.index("*/", i + 2) + 1
        else:
            out.append(c)
        i += 1
    return json.loads(re.sub(r",(\s*[}\]])", r"\1", "".join(out)))


def literal_facts(text):
    literals = [m for m in re.findall(r'"((?:[^"\\\n]|\\.)*)"', text)]
    joined = []
    for call in re.findall(r"\bjoin\(([^)]*)\)", text):
        joined.append("/".join(re.findall(r'"((?:[^"\\\n]|\\.)*)"', call)))
    templates = []
    for body in re.findall(r"`((?:[^`\\]|\\.)*)`", text):
        templates.extend(re.split(r"\$\{[^}]*\}", body))
    crates = re.findall(r'"(?:-p|--package)",\s*"([^"]+)"', text)
    paths = []
    for literal in literals + joined + templates:
        if "/" in literal and not re.search(r"\s", literal) and not re.match(r"^(https?:|@|//)", literal) and not re.search(r"[*?]", literal):
            if literal not in paths:
                paths.append(literal)
    seen = []
    for crate in crates:
        if crate not in seen:
            seen.append(crate)
    return {"crates": seen, "paths": paths, "cargo": bool(re.search(r'\brunCargo\b|"cargo"', text))}


def dispatch_branches(source):
    lines = source.split("\n")
    branches = []
    for index, line in enumerate(lines):
        head = re.match(r'^(\s*)if \(segments\[0\] === "([^"]+)"(?: && segments\[1\] === "([^"]+)")?', line)
        if not head:
            continue
        end = index
        if line.rstrip().endswith("{"):
            end = index + 1
            while end < len(lines) and lines[end] != head.group(1) + "}":
                end += 1
        facts = literal_facts("\n".join(lines[index:end + 1]))
        row = {"key": head.group(2)}
        if head.group(3):
            row["subject"] = head.group(3)
        row.update(facts)
        branches.append(row)
    return branches


def package_name(manifest):
    inside = False
    for line in manifest.split("\n"):
        header = re.match(r"^\s*\[([^\]]+)\]\s*$", line.rstrip("\r"))
        if header:
            inside = header.group(1).strip() == "package"
            continue
        if inside:
            name = re.match(r'^\s*name\s*=\s*"([^"]+)"', line)
            if name:
                return name.group(1)
    return None


def main():
    fixture = json.load(open(sys.argv[1]))
    result = {
        "globs": [bool(glob_regex(case["pattern"]).match(case["path"])) for case in fixture["globs"]],
        "commands": [command_words(case["command"]) for case in fixture["commands"]],
        "jsonc": [jsonc(case["text"]) for case in fixture["jsonc"]],
        "dispatch": [dispatch_branches(case["source"]) for case in fixture["dispatch"]],
        "packageNames": [package_name(case["manifest"]) for case in fixture["packageNames"]],
    }
    json.dump(result, sys.stdout, ensure_ascii=False)


main()

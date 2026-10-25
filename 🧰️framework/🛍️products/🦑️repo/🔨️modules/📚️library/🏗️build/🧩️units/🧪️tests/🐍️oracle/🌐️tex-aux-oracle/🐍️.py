#!/usr/bin/env python3
"""🔬️ Independent stdlib-only oracle for the TeX adapter, written from the policy text of the `tex-aux` interface kind and the scanner contract (no TeX engine is installed on the verification host; with tectonic present the TypeScript test additionally compiles a real document).
usage: 🌐️.py < languages-fixture.json  -> {"aux": ..., "pairs": {name: same}, "scans": {name: refs}}"""
import json, re, sys


def normalize_aux(aux):
    labels, cites, rest = [], [], []
    for line in aux.split("\n"):
        line = line.strip()
        if line.startswith("\\newlabel{"):
            labels.append(line)
        elif line.startswith("\\bibcite{"):
            cites.append(line)
        elif re.match(r"^\\@writefile\{(toc|lof|lot)\}", line) or line.startswith("\\bibstyle{") or line.startswith("\\bibdata{"):
            rest.append(line)
    key = lambda text: [ord(c) for c in text]
    return "\n".join(sorted(labels, key=key) + sorted(cites, key=key) + rest)


def strip_comments(text):
    out = []
    for line in text.split("\n"):
        i, cut = 0, None
        while i < len(line):
            if line[i] == "\\":
                i += 2
                continue
            if line[i] == "%":
                cut = i
                break
            i += 1
        out.append(line if cut is None else line[:cut])
    return "\n".join(out)


def braces(text, command):
    return [m.group(1).strip() for m in re.finditer(r"\\" + command + r"\s*(?:\[[^\]]*\])?\s*\{([^{}]*)\}", text)]


def scan(text):
    clean = strip_comments(text)
    split = lambda values: [p.strip() for v in values for p in v.split(",") if p.strip()]
    return {
        "inputs": braces(clean, "input") + braces(clean, "include") + braces(clean, "subfile"),
        "bibliographies": split(braces(clean, "bibliography")) + braces(clean, "addbibresource"),
        "graphics": braces(clean, "includegraphics"), "packages": split(braces(clean, "usepackage")), "classes": braces(clean, "documentclass"),
        "externalDocuments": braces(clean, "externaldocument"), "isDocument": re.search(r"\\documentclass\b", clean) is not None,
    }


fixture = json.load(sys.stdin)["tex"]
json.dump({
    "aux": normalize_aux(fixture["aux"]),
    "pairs": {p["name"]: normalize_aux(p["before"]) == normalize_aux(p["after"]) for p in fixture["aux-pairs"]},
    "scans": {s["name"]: scan(s["source"]) for s in fixture["scans"]},
}, sys.stdout, ensure_ascii=False)

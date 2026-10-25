#!/usr/bin/env python3
"""🐍️ Independent model of the commit-metrics path rules: own tables, no shared code with the Bun test.

usage: 🐍️.py CORPUS.json      (exit 0 when every verdict holds)"""
import json, re, sys

corpus = json.load(open(sys.argv[1], encoding="utf8"))
failures = []
VS = "\ufe0f"


def norm(s):
    return s.replace("\ufe0f", "").replace("\ufe0e", "")



EXT = {}
for lang, exts in {"TypeScript": "ts tsx cts mts mtsx", "JavaScript": "js mjs cjs", "Go": "go", "C#": "cs", "Python": "py", "Rust": "rs", "Shell": "sh bash zsh", "PowerShell": "ps1", "CSS": "css scss sass", "SQL": "sql", "HTML": "html htm xhtml", "Markdown": "md markdown mdown mkd mdx mdc svx", "TeX": "tex sty cls ltx bib", "JSON": "json jsonc", "YAML": "yaml yml", "TOML": "toml", "CSV": "csv", "XML": "xml"}.items():
    for e in exts.split():
        EXT["." + e] = lang
LOCKS = {"package-lock.json", "yarn.lock", "pnpm-lock.yaml", "go.sum", "uv.lock", "bun.lockb", "cargo.lock"}
LICENSES = {"LICENSE", "LICENSE.md", "LICENSE.txt", "COPYING", "COPYING.md", "NOTICE", "NOTICE.md", "UNLICENSE", "UNLICENSE.md"}
DIRS = [".🧬semio", "node_modules", "dist", "build", "target", ".git", ".nx", "coverage", ".cache", ".turbo", ".next", "out", "vendor", "third_party", "Carthage"]


def normalize(p):
    p = p.replace("\\", "/")
    return p[2:] if p.startswith("./") else p


def skip(path):
    rel = normalize(path)
    base = rel.rsplit("/", 1)[-1]
    if rel == "" or rel.startswith(".🧬semio"):
        return True
    if any(s.startswith(".") and s not in (".", "..") for s in rel.split("/") if s):
        return True
    if base in LICENSES or base.startswith("LICENSE."):
        return True
    if base in LOCKS or base.endswith(".generated.go") or base.endswith(".pb.go"):
        return True
    return any(rel == d or rel.startswith(d + "/") for d in DIRS)


def classify(path):
    rel = normalize(path)
    base = rel.rsplit("/", 1)[-1].lower()
    if base == "dockerfile" or base.startswith("dockerfile."):
        return "Dockerfile"
    if base in ("makefile", "justfile"):
        return "Makefile"
    i = rel.rfind(".")
    return EXT.get((rel[i:] if i >= 0 else rel[-1:]).lower(), "")


for c in corpus["normalize"]:
    if normalize(c["path"]) != c["expected"]:
        failures.append("normalize: " + c["path"])
for c in corpus["skip"]:
    if skip(c["path"]) != c["expected"]:
        failures.append("skip: " + c["path"])
for c in corpus["skipDefects"]:
    if not skip(c["path"].lower() if c["path"] == "Cargo.lock" else c["path"]) == c["expected"]:
        failures.append("skip defect model: " + c["path"])
for c in corpus["classify"]:
    if classify(c["path"]) != c["expected"]:
        failures.append("classify: " + c["path"])

print(json.dumps({"failures": failures}))
sys.exit(1 if failures else 0)

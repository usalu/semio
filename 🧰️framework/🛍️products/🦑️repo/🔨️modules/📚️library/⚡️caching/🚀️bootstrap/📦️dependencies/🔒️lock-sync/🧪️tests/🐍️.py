#!/usr/bin/env python3
"""🐍️ Independent model of the lock-in-sync texts and the scope discovery: reads the same corpus and the same schema texts and shares no code with the Bun test (stdlib only).

usage: 🐍️.py CORPUS.json SCHEMA.json      (exit 0 when every verdict holds)"""
import json, os, re, sys, tempfile

corpus = json.load(open(sys.argv[1], encoding="utf8"))
schema = json.load(open(sys.argv[2], encoding="utf8"))
protocol = schema["$defs"]["Protocol"]["const"]
texts = schema["$defs"]["Texts"]["const"]


def fill(template, values):
    out = template
    for key, value in values.items():
        out = out.replace("{" + key + "}", "" if value is None else str(value))
    return out


def describe(lang, row):
    table = texts[lang]
    member = re.sub(r"/?package\.json$", "", row["manifest"]) or table["root"]
    return fill(table[row["kind"]], {"member": member, "section": row.get("section"), "dependency": row.get("dependency"), "manifestVersion": row.get("manifestVersion", table["absent"]), "lockVersion": row.get("lockVersion", table["absent"])})


def changes(lang, drift):
    table = texts[lang]
    listed = [describe(lang, row) for row in drift[: protocol["maximumListed"]]]
    if len(drift) > len(listed):
        listed.append(fill(table["more"], {"count": len(drift) - len(listed)}))
    return "; ".join(listed) if listed else table["unexplained"]


def message(kind, scope, drift, languages):
    command = fill(protocol["command"], {"scope": scope})
    return "\n".join(fill(texts[lang][kind], {"scope": scope, "changes": changes(lang, drift), "command": command}) for lang in languages)


def norm(member):
    parts = []
    for part in member.replace("\\", "/").split("/"):
        if part in ("", "."):
            continue
        if part == ".." and parts and parts[-1] != "..":
            parts.pop()
        else:
            parts.append(part)
    return "/".join(parts)


def jsonc(text):
    text = re.sub(r'("(?:[^"\\]|\\.)*")|//[^\n]*|/\*[\s\S]*?\*/', lambda m: m.group(1) or "", text)
    return json.loads(re.sub(r",(\s*[}\]])", r"\1", text))


def scopes(files):
    root = tempfile.mkdtemp(prefix="lock-sync-oracle-")
    for rel, content in files.items():
        path = os.path.join(root, *rel.split("/"))
        os.makedirs(os.path.dirname(path), exist_ok=True)
        open(path, "w", encoding="utf8").write(content)
    manifest = jsonc(files["package.json"])
    patterns = manifest.get("semio", {}).get("workspace", {}).get("owners", [])
    directories = {"."}
    for pattern in patterns:
        match = re.match(protocol["ownerPattern"], pattern) if isinstance(pattern, str) else None
        if not match:
            return {"error": "bootstrap.dependencies.owner-pattern-required"}
        for entry in sorted(os.listdir(root)):
            if os.path.isdir(os.path.join(root, entry)) and os.path.exists(os.path.join(root, entry, match.group(1))) and os.path.exists(os.path.join(root, entry, "bun.lock")):
                directories.add(entry)
    out = []
    for directory in sorted(directories):
        base = root if directory == "." else os.path.join(root, directory)
        if not os.path.exists(os.path.join(base, "bun.lock")):
            continue
        doc = jsonc(open(os.path.join(base, "package.json"), encoding="utf8").read())
        keys = ["package.json"]
        for member in doc.get("workspaces", []):
            if not isinstance(member, str) or "*" in member:
                continue
            key = norm(member)
            if os.path.exists(os.path.join(base, *key.split("/"), "package.json")):
                keys.append(key + "/package.json")
        out.append({"scope": directory, "manifests": keys})
    return {"scopes": out}


failures = []
for row in corpus["texts"]:
    if message(row["kind"], row["scope"], row["drift"], row["languages"]) != row["expected"]:
        failures.append("text: " + row["name"])
for row in corpus["scopes"]:
    if scopes(row["files"]) != row["expected"]:
        failures.append("scopes: " + row["name"])
print(json.dumps({"failures": failures}))
sys.exit(1 if failures else 0)

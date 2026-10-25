"""🎬️ Independent Python oracle of the narrow implementation closure (standard library only): it walks the static imports of one entry (no type-only forms, no import() or require() calls), adds the JSON files a module names by quoted relative path, the module-owned schema files (at any depth of the schema folder) beside every module and the declared lazy roots and the declared leaves (bytes only), and answers which files the closure holds. Requests arrive as JSON on stdin: {"entry": <absolute path>, "roots": [<absolute path>...], "leaves": [<absolute path>...]}."""
import json, os, re, sys

STATIC = re.compile(r"""(?:^|[;{}\n])[ \t]*(?:import(?![ \t]+type[ \t{*])(?:[^"';]*?[ \t}*]from)?|export(?![ \t]+type[ \t{*])[ \t]*(?:\*(?:[ \t]+as[ \t]+[\w$]+)?|\{[^}"';]*\})[ \t]*from)[ \t]*(["'])([^"'\n]+)\1""")
JSON_LITERAL = re.compile(r"""(["'])(\.{1,2}/[^"'\\\r\n]+\.json)\1""")
SKIPPED = {"node_modules", ".cache"}


def exists(path):
    return os.path.isfile(path) and not any(part in SKIPPED for part in path.split(os.sep))


def schemas_beside(path):
    folder = os.path.join(os.path.dirname(path), "🧬️schema")
    found = []
    for directory, names, files in os.walk(folder):
        names[:] = sorted(name for name in names if name not in SKIPPED)
        found.extend(os.path.join(directory, name) for name in sorted(files) if name.endswith(".json"))
    return found


def closure(entry, roots):
    seen, pending = set(), [os.path.normpath(entry)] + [os.path.normpath(root) for root in roots]
    while pending:
        path = pending.pop()
        if path in seen or not exists(path):
            continue
        seen.add(path)
        if path.endswith(".json"):
            continue
        text = open(path, encoding="utf-8").read()
        pending.extend(schemas_beside(path))
        for match in STATIC.finditer(text):
            specifier = match.group(2)
            if specifier.startswith("."):
                pending.append(os.path.normpath(os.path.join(os.path.dirname(path), specifier)))
        for match in JSON_LITERAL.finditer(text):
            pending.append(os.path.normpath(os.path.join(os.path.dirname(path), match.group(2))))
    return seen


if __name__ == "__main__":
    request = json.load(sys.stdin)
    held = closure(request["entry"], request.get("roots", []))
    held |= {os.path.normpath(leaf) for leaf in request.get("leaves", []) if exists(os.path.normpath(leaf))}
    print(json.dumps(sorted(held)))

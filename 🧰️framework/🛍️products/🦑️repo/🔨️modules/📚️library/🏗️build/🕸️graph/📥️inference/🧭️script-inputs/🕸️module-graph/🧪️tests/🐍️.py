import json
import os
import re
import sys

IMPORT = re.compile(r"""(?:import|export)\s[^;]*?from\s*["']([^"']+)["']|import\s*["']([^"']+)["']|import\(\s*["']([^"']+)["']\s*\)""")


def specifiers(text):
    return [next(group for group in match.groups() if group) for match in IMPORT.finditer(text)]


def resolve(directory, specifier):
    base = os.path.normpath(os.path.join(directory, specifier))
    for candidate in [base, base + ".ts", os.path.join(base, "index.ts")]:
        if os.path.isfile(candidate):
            return candidate
    return None


def reach(root, entries, kept, generated):
    files, compiled, stack = set(), set(), [(os.path.join(root, entry), True) for entry in entries]
    seen = set()
    while stack:
        path, source = stack.pop()
        if (path, source) in seen:
            continue
        seen.add((path, source))
        files.add(path)
        if not source or not path.endswith(".ts"):
            continue
        compiled.add(path)
        text = open(path, encoding="utf8").read()
        for specifier in specifiers(text):
            if specifier in generated:
                for source in generated[specifier]:
                    stack.append((os.path.join(root, source), True))
                continue
            if not specifier.startswith("."):
                continue
            relative = os.path.relpath(path, root)
            if relative in kept and specifier not in kept[relative]:
                continue
            target = resolve(os.path.dirname(path), specifier)
            if target is None:
                continue
            stack.append((target, target.endswith(".ts")))
    return sorted(os.path.relpath(path, root) for path in files)


request = json.load(sys.stdin)
print(json.dumps(reach(request["root"], request["entries"], request.get("kept", {}), request.get("generated", {}))))

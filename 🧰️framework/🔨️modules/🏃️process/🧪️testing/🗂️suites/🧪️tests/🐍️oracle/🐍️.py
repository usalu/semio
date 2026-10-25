"""🐍️ Independent oracle of the process route table: walks a module root, discovers test files by the protocol patterns of the schema and routes them by the nearest-tree rule; prints one JSON document."""
import json, os, re, sys

root, table_path, schema_path = sys.argv[1:4]
table = json.load(open(table_path, encoding="utf8"))
protocol = json.load(open(schema_path, encoding="utf8"))["definitions"]["Protocol"]["const"]
patterns = [re.compile(p) for p in protocol["importPatterns"]]


def discover():
    found = []
    for directory, names, files in os.walk(root, followlinks=False):
        names[:] = [n for n in names if n not in protocol["skippedDirectories"] and not os.path.islink(os.path.join(directory, n))]
        relative = os.path.relpath(directory, root).split(os.sep)
        if protocol["testDirectory"] not in relative:
            continue
        for name in files:
            path = os.path.join(directory, name)
            if name.endswith(protocol["extension"]) and not os.path.islink(path) and any(p.search(open(path, encoding="utf8").read()) for p in patterns):
                found.append("/".join(os.path.relpath(path, root).split(os.sep)))
    return found


discovered = discover()
helpers = set(table["helpers"])
suites = table["suites"]
owners = {}
for file in discovered:
    if file in helpers:
        continue
    chosen = [s for s in suites if "testNamePattern" not in s and file in s.get("files", [])]
    if not chosen:
        best = -1
        for s in suites:
            for tree in s.get("trees", []):
                prefix = tree.rstrip("/") + "/"
                if file.startswith(prefix):
                    if len(prefix) > best:
                        best, chosen = len(prefix), [s]
                    elif len(prefix) == best and s not in chosen:
                        chosen.append(s)
    allowed = chosen + [s for s in suites if "testNamePattern" in s and file in s.get("files", [])]
    if allowed:
        owners[file] = allowed


def entire(file):
    return [s for s in owners.get(file, []) if "testNamePattern" not in s]


known = set(discovered)
bare = re.compile(r"(?:^|[;{}])[ \t]*import\s*[\"']([^\"']+)[\"']", re.M)
aggregated = []
for f in discovered:
    for target in bare.findall(open(os.path.join(root, f), encoding="utf8").read()):
        if target.startswith("."):
            resolved = os.path.normpath(os.path.join(os.path.dirname(f), target)).replace(os.sep, "/")
            if resolved in known:
                aggregated.append(f + " -> " + resolved)
print(json.dumps({
    "aggregated": aggregated,
    "discovered": discovered,
    "unrouted": [f for f in discovered if f not in owners and f not in helpers],
    "duplicated": [f + " -> " + ",".join(s["id"] for s in entire(f)) for f in owners if len(entire(f)) > 1],
    "filtered": [f for f in owners if not entire(f)],
    "missing": [s["id"] + ":" + f for s in suites for f in s.get("files", []) if f not in known and not os.path.exists(os.path.join(root, f))],
    "staleHelpers": [f for f in table["helpers"] if f not in known],
}, ensure_ascii=False))

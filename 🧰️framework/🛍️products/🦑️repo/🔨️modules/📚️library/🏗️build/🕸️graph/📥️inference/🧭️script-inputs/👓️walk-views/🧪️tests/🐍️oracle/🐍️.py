"""🐍 Independent model of command walk views: the walk is every file reachable from the entry by relative imports; a view drops the files under its prefixes (scheduling: every scheduling domain prefix; native: the same without the native-keyed prefixes), read from the scheduling table itself. A view's answer changes exactly when its roster changes. Usage: python3 oracle.py <corpus.json> <scheduling-table.json>; prints {"rosters": [...], "verdicts": [...]} as JSON."""
import json, posixpath, re, sys

IMPORT = re.compile(r'(?:import|export)\s+(?:type\s+)?(?:[^"\';]*?\s+from\s+)?["\'](\.[^"\']+)["\']')


def walk(files, entry):
    seen, stack = set(), [entry]
    while stack:
        path = stack.pop()
        if path in seen or path not in files:
            continue
        seen.add(path)
        for spec in IMPORT.findall(files[path]):
            stack.append(posixpath.normpath(posixpath.join(posixpath.dirname(path), spec)))
    return sorted(seen)


def views(table):
    scheduling = sorted({prefix for domain in table["domains"] for prefix in domain["prefixes"]})
    keyed = [row for domain in table["domains"] for row in domain.get("nativeKeyed", [])]
    native = [prefix for prefix in scheduling if not any(row.startswith(prefix) or prefix.startswith(row) for row in keyed)]
    return {"whole": [], "scheduling": scheduling, "native": native}


def main():
    corpus, table = (json.load(open(path, encoding="utf8")) for path in sys.argv[1:3])
    prefixes, files = views(table), dict(corpus["files"])
    seen = lambda: {name: [path for path in walk(files, corpus["entry"]) if not any(path.startswith(prefix) for prefix in cut)] for name, cut in prefixes.items()}
    before, rosters, verdicts = seen(), [], []
    for mutation in corpus["mutations"]:
        files.update(mutation["edit"])
        after = seen()
        rosters.append(after["whole"])
        verdicts.append({name: before[name] != after[name] for name in prefixes})
        before = after
    print(json.dumps({"rosters": rosters, "verdicts": verdicts}, ensure_ascii=False))


main()

"""🐍️ Independent oracle of the build route laws. `classes <root> <files.json> <fixture>` prints the sorted files that mention a process-spawning token; `aggregation <library> <build> <schema>` prints every test file of the build tree that imports another test file of the library for its side effects; `pure-aggregators <library> <build> <schema>` prints every source below a `🧪️tests` directory of the build tree that is itself no test file but imports test files of the library for their side effects."""
import json, os, re, sys


def classes(root, files_path, fixture_path):
    tokens = json.load(open(fixture_path, encoding="utf8"))["spawnTokens"]
    spawning = []
    for file in json.load(open(files_path, encoding="utf8")):
        with open(os.path.normpath(os.path.join(root, file)), encoding="utf8") as handle:
            text = handle.read()
        if any(token in text for token in tokens):
            spawning.append(file)
    print(json.dumps(sorted(spawning), ensure_ascii=False))


def aggregation(library, build, schema_path, pure=False):
    protocol = json.load(open(schema_path, encoding="utf8"))["definitions"]["Protocol"]["const"]
    patterns = [re.compile(p) for p in protocol["importPatterns"]]
    bare = re.compile(r"(?:^|[;{}])[ \t]*import\s*[\"']([^\"']+)[\"']", re.M)

    def tests(top):
        found = []
        for directory, names, files in os.walk(top, followlinks=False):
            names[:] = [n for n in names if n not in protocol["skippedDirectories"] and not os.path.islink(os.path.join(directory, n))]
            if protocol["testDirectory"] not in os.path.relpath(directory, top).split(os.sep):
                continue
            for name in files:
                path = os.path.join(directory, name)
                if name.endswith(protocol["extension"]) and not os.path.islink(path):
                    with open(path, encoding="utf8") as handle:
                        if any(p.search(handle.read()) for p in patterns):
                            found.append(os.path.normpath(path))
        return found

    every = set(tests(library))
    rows = []
    if pure:
        candidates = []
        for directory, names, files in os.walk(build, followlinks=False):
            names[:] = [n for n in names if n not in protocol["skippedDirectories"] and not os.path.islink(os.path.join(directory, n))]
            if protocol["testDirectory"] in os.path.relpath(directory, build).split(os.sep):
                candidates += [os.path.normpath(os.path.join(directory, n)) for n in files if n.endswith(protocol["extension"]) and not os.path.islink(os.path.join(directory, n))]
        sources = [c for c in candidates if c not in every]
    else:
        sources = tests(build)
    for path in sources:
        with open(path, encoding="utf8") as handle:
            text = handle.read()
        for spec in bare.findall(text):
            if spec.startswith(".") and os.path.normpath(os.path.join(os.path.dirname(path), spec)) in every:
                rows.append(os.path.relpath(path, build).replace(os.sep, "/") + " -> " + os.path.relpath(os.path.normpath(os.path.join(os.path.dirname(path), spec)), library).replace(os.sep, "/"))
    print(json.dumps(sorted(rows), ensure_ascii=False))


if sys.argv[1] == "pure-aggregators":
    aggregation(*sys.argv[2:5], pure=True)
else:
    {"classes": classes, "aggregation": aggregation}[sys.argv[1]](*sys.argv[2:])

"""🧊️ Independent Python oracle of the cold peak estimate and the external source class (standard library only), written from the prose: a unit without any measurement is estimated as base plus a per-thousand-lines share; a crate fetched from outside the workspace (its source path, with backslashes read as slashes, contains a registry, git checkout or vendor directory marker) uses the lower external base and share and may never be estimated above the budget its kind is held to; no estimate falls below the floor. The size of a unit is the number of newline bytes of the .rs files its root file reaches through `mod name;` declarations (name.rs or name/mod.rs beside a lib.rs/mod.rs/main.rs, or below a directory named after any other file); when a declared module has no file, the size is that of all .rs files below the root's directory, leaving out skipped directories and every nested directory that holds its own Cargo.toml. usage: 🐍️.py <corpus.json> prints {"estimates": [...], "sources": [...]} as JSON."""
import json, math, sys

with open(sys.argv[1], encoding="utf-8") as handle:
    corpus = json.load(handle)
policy = corpus["policy"]


def estimate(case):
    if case["external"]:
        size = math.ceil(policy["externalBaseBytes"] + policy["externalBytesPerKloc"] * case["kloc"])
        if case["budgetBytes"] is not None:
            size = min(size, case["budgetBytes"])
    else:
        size = math.ceil(policy["workspaceBaseBytes"] + policy["workspaceBytesPerKloc"] * case["kloc"])
    return max(size, policy["floorBytes"])


def external(source):
    path = source.replace("\\", "/")
    return any(marker in path for marker in policy["markers"])


def lines(tree):
    files = {path: (count, mods) for path, count, mods in tree["files"]}

    def children(path, mods):
        parts = path.split("/")
        name = parts[-1]
        base = "/".join(parts[:-1]) if name in ("lib.rs", "mod.rs", "main.rs") else "/".join(parts[:-1] + [name[:-3]])
        prefix = base + "/" if base else ""
        for mod in mods:
            found = [candidate for candidate in (prefix + mod + ".rs", prefix + mod + "/mod.rs") if candidate in files]
            yield found[0] if found else None

    reached, queue, broken = set(), ["lib.rs"], False
    while queue:
        path = queue.pop()
        if path in reached:
            continue
        reached.add(path)
        for child in children(path, files[path][1]):
            if child is None:
                broken = True
            else:
                queue.append(child)
    if not broken:
        return sum(files[path][0] for path in reached)
    packages = {path.rsplit("/", 1)[0] for path in files if path.rsplit("/", 1)[-1] == policy["manifestFile"] and "/" in path}
    total = 0
    for path, (count, _) in files.items():
        parts = path.split("/")
        directories = ["/".join(parts[: index + 1]) for index in range(len(parts) - 1)]
        if path.endswith(".rs") and not any(part in policy["skipDirectories"] for part in parts[:-1]) and not any(directory in packages for directory in directories):
            total += count
    return total


print(json.dumps({"estimates": [estimate(case) for case in corpus["estimates"]], "sources": [external(row["source"]) for row in corpus["sources"]], "trees": [lines(tree) for tree in corpus["trees"]]}))

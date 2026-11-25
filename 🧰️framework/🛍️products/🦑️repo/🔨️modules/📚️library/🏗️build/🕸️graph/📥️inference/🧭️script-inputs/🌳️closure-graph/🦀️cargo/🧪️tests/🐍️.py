"""🐍️ Independent oracle of the Cargo part of the module closure graph: reads {"root", "owners": [crate directory], "edits": [{"path"}]} and prints, per edit, the owners whose dependency closure contains the crate that holds the edited file (Cargo's own rule: path dependencies, workspace inheritance, development dependencies only at the root, a lock file belongs to every crate of its workspace), and the crates each owner reaches. It reads the manifests with its own reader of the TOML subset the fixtures use (tables, dotted table names, strings, booleans, arrays of strings, inline tables) and walks them by plain recursion; it shares no code with the graph."""
import json, os, re, sys

def value(text):
    text = text.strip()
    if text.startswith('"'):
        return json.loads(text)
    if text in ("true", "false"):
        return text == "true"
    if text.startswith("["):
        inner = text[1:-1].strip()
        return [value(part) for part in re.findall(r'"[^"]*"', inner)]
    if text.startswith("{"):
        return {key.strip(): value(item) for key, item in (pair.split("=", 1) for pair in re.findall(r'[A-Za-z0-9_-]+\s*=\s*(?:"[^"]*"|true|false)', text[1:-1]))}
    return text

def read_toml(path):
    document, table = {}, None
    with open(path, encoding="utf-8") as handle:
        for line in handle:
            line = line.strip()
            if not line or line.startswith("#"):
                continue
            if line.startswith("["):
                table = document
                for part in line.strip("[]").split("."):
                    table = table.setdefault(part, {})
                continue
            key, item = line.split("=", 1)
            (table if table is not None else document)[key.strip()] = value(item)
    return document

request = json.load(sys.stdin)
root = request["root"]
manifests = {}

def manifest(directory):
    if directory not in manifests:
        manifests[directory] = read_toml(os.path.join(directory, "Cargo.toml"))
    return manifests[directory]

def workspace_of(directory):
    current = directory
    while True:
        path = os.path.join(current, "Cargo.toml")
        if os.path.exists(path) and "workspace" in manifest(current):
            return current
        if current == root:
            return root
        current = os.path.dirname(current)

def children(directory, tests):
    data, workspace = manifest(directory), workspace_of(directory)
    inherited = manifest(workspace).get("workspace", {}).get("dependencies", {}) if os.path.exists(os.path.join(workspace, "Cargo.toml")) else {}
    kinds = ["dependencies", "build-dependencies"] + (["dev-dependencies"] if tests else [])
    found = []
    for table in [data] + list(data.get("target", {}).values()):
        for kind in kinds:
            for alias, declaration in table.get(kind, {}).items():
                if not isinstance(declaration, dict):
                    continue
                if declaration.get("workspace") is True:
                    declared = inherited.get(alias, {})
                    if "path" in declared:
                        found.append(os.path.normpath(os.path.join(workspace, declared["path"])))
                elif "path" in declaration:
                    found.append(os.path.normpath(os.path.join(directory, declaration["path"])))
    return found

def reach(directory, tests):
    seen = {directory}
    stack = [(child, False) for child in children(directory, tests)]
    while stack:
        current, _ = stack.pop()
        if current in seen:
            continue
        seen.add(current)
        stack.extend((child, False) for child in children(current, False))
    return seen

owners = {owner: reach(os.path.join(root, owner), True) for owner in request["owners"]}

def crate_of(path):
    current = os.path.dirname(path)
    while not os.path.exists(os.path.join(current, "Cargo.toml")):
        current = os.path.dirname(current)
    return current

out = {"reach": {owner: sorted(os.path.relpath(crate, root) for crate in crates) for owner, crates in owners.items()}, "moved": []}
for edit in request["edits"]:
    path = os.path.join(root, edit["path"])
    if os.path.basename(path) == "Cargo.lock":
        workspace = os.path.dirname(path)
        out["moved"].append(sorted(owner for owner, crates in owners.items() if any(workspace_of(crate) == workspace for crate in crates)))
    else:
        crate = crate_of(path)
        out["moved"].append(sorted(owner for owner, crates in owners.items() if crate in crates))
print(json.dumps(out))

"""Independent oracle of the owner registry key law: for the fixture file (argv[1]) it prints the scope identity of the selection (the registry never enters it), the digest of every owner row, the digest of the whole registry, the answer of every question, and for every edit which row digests moved, whether the registry digest moved, which answers moved and that the scope stayed the same."""
import copy
import hashlib
import json
import sys

fixture = json.load(open(sys.argv[1], encoding="utf8"))
WORKSPACE = "."


def dumps(value):
    return json.dumps(value, ensure_ascii=False, separators=(",", ":"))


def sha(value):
    return hashlib.sha256(dumps(value).encode("utf8")).hexdigest()


def row_digest(owner):
    return sha([owner["name"], owner["root"], owner["manifest"], sorted(owner["packages"]), sorted(owner.get("nativeRoots", []))])


def registry_digest(registry):
    rows = sorted(owner["name"] for owner in registry["owners"])
    digests = {owner["name"]: row_digest(owner) for owner in registry["owners"]}
    providers = sorted([p["ownerRoot"], p["sourcePath"], sorted(p["consumerRoots"])] for p in registry["catalogProviders"])
    return sha([[[name, digests[name]] for name in rows], providers])


def owned(path, root):
    return root == WORKSPACE or path == root or path.startswith(root + "/")


def manifest_of(root):
    return "Cargo.toml" if root == WORKSPACE else root + "/Cargo.toml"


def answer(registry, question):
    kind, args = question[0], question[1:]
    owners = registry["owners"]
    if kind == "row":
        return next((row_digest(o) for o in owners if o["name"] == args[0]), None)
    if kind == "package":
        return next((o["name"] for o in reversed(owners) if args[0] in o["packages"]), None)
    if kind == "root":
        return next((o["name"] for o in reversed(owners) if args[0] == o["root"] or args[0] in o.get("nativeRoots", [])), None)
    if kind == "within":
        roots = {}
        for o in owners:
            if o["root"] != WORKSPACE and (o["root"] not in roots or o["name"] < roots[o["root"]]):
                roots[o["root"]] = o["name"]
        parts = args[0].split("/")
        for count in range(len(parts), 0, -1):
            hit = roots.get("/".join(parts[:count]))
            if hit is not None:
                return hit
        return None
    if kind == "providers":
        owner_root, has_targets, names = args
        chosen = [p for p in registry["catalogProviders"] if (has_targets and any(owned(owner_root, r) for r in p["consumerRoots"])) or ("{workspaceRoot}/" + p["sourcePath"]) in names]
        return sha([[p["ownerRoot"], p["sourcePath"]] for p in chosen])
    if kind == "candidates":
        found = set()
        for o in owners:
            if o["manifest"].endswith("Cargo.toml"):
                found.add(o["manifest"])
            for native in o.get("nativeRoots", []):
                found.add(manifest_of(native))
            found.add(manifest_of(o["root"]))
        return sha(sorted(found))
    if kind == "digest":
        return registry_digest(registry)
    raise SystemExit("unknown question " + kind)


def scope(selection):
    value = {"kind": selection["kind"], "projects": sorted(selection["projects"]), "exclude": sorted(selection["exclude"])}
    if "targets" in selection:
        value["targets"] = sorted(selection["targets"])
    return sha(value)


def apply(registry, edit):
    changed = copy.deepcopy(registry)
    owners = changed["owners"]
    target = next((o for o in owners if o["name"] == edit.get("owner")), None)
    if edit["op"] == "root":
        target["root"] = edit["value"]
        target["manifest"] = edit["value"] + "/" + target["manifest"].rsplit("/", 1)[1]
    elif edit["op"] == "packages":
        target["packages"] = edit["value"]
    elif edit["op"] == "reorder":
        target["packages"] = list(reversed(target["packages"]))
        if "nativeRoots" in target:
            target["nativeRoots"] = list(reversed(target["nativeRoots"]))
    elif edit["op"] == "add":
        owners.append(edit["row"])
    elif edit["op"] == "remove":
        changed["owners"] = [o for o in owners if o["name"] != edit["owner"]]
    elif edit["op"] == "provider":
        changed["catalogProviders"][edit["index"]]["consumerRoots"] = edit["value"]
    return changed


def digests(registry):
    return {o["name"]: row_digest(o) for o in registry["owners"]}


registry = fixture["registry"]
base_rows, base_digest = digests(registry), registry_digest(registry)
base_answers = [answer(registry, q) for q in fixture["questions"]]
result = {"scope": scope(fixture["selection"]), "rows": base_rows, "digest": base_digest, "answers": base_answers, "edits": []}
for edit in fixture["edits"]:
    changed = apply(registry, edit)
    rows = digests(changed)
    moved = sorted(name for name in set(rows) | set(base_rows) if rows.get(name) != base_rows.get(name))
    result["edits"].append({
        "rows": moved,
        "digest": registry_digest(changed) != base_digest,
        "answers": [answer(changed, q) != before for q, before in zip(fixture["questions"], base_answers)],
        "scope": scope(fixture["selection"]),
    })
print(dumps(result))

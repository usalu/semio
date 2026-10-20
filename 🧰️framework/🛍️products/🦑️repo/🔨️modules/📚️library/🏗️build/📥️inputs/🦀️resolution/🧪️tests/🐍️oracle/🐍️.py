#!/usr/bin/env python3
# 🐍️ Independent Python implementation of the Cargo resolution identity over the language-agnostic fixture (parsed manifest trees): canonical JSON, owning workspace, dependency rows, inherited slices, the folded crate identity, the toolchain digests and the whole-workspace key predicate. Standard library only; prints one JSON document the TypeScript test compares.
import hashlib, json, re, sys

def canon(value):
    if isinstance(value, dict):
        items = [(k, v) for k, v in value.items() if v is not None or True]
        return "{" + ",".join(json.dumps(k, ensure_ascii=False) + ":" + canon(v) for k, v in sorted(items, key=lambda kv: kv[0].encode("utf8"))) + "}"
    if isinstance(value, list):
        return "[" + ",".join(canon(v) for v in value) + "]"
    return json.dumps(value, ensure_ascii=False, separators=(",", ":"))

def sha(text):
    return hashlib.sha256(text.encode("utf8")).hexdigest()

def at(d, n):
    return n if d == "" else d + "/" + n

def parent(d):
    return d[: d.rindex("/")] if "/" in d else ""

def resolve(base, rel):
    parts = [] if base == "" else base.split("/")
    for p in rel.split("/"):
        if p in ("", "."):
            continue
        if p == "..":
            if not parts:
                return None
            parts.pop()
        else:
            parts.append(p)
    return "/".join(parts)

class Refusal(Exception):
    pass

class Repo:
    def __init__(self, trees):
        self.trees = trees
        self.ids = {}
    def read(self, path):
        return self.trees.get(path)
    def workspace(self, crate):
        own = self.read(at(crate, "Cargo.toml")) or {}
        explicit = (own.get("package") or {}).get("workspace")
        root = resolve(crate, explicit) if isinstance(explicit, str) else crate
        if root is None:
            raise Refusal("escape")
        while True:
            manifest = self.read(at(root, "Cargo.toml"))
            if manifest is not None and isinstance(manifest.get("workspace"), dict):
                return root, manifest
            if isinstance(explicit, str):
                raise Refusal("workspace")
            if root == "":
                return None
            root = parent(root)
    def rows(self, crate, manifest, ws, tests):
        shared = ((ws[1].get("workspace") or {}).get("dependencies") or {}) if ws else {}
        tables = [("", manifest)] + [("target." + n, t) for n, t in (manifest.get("target") or {}).items() if isinstance(t, dict)]
        out = []
        for tname, source in tables:
            for kind in ("dependencies", "build-dependencies", "dev-dependencies"):
                if kind == "dev-dependencies" and not tests:
                    continue
                for alias, decl in (source.get(kind) or {}).items():
                    if not isinstance(decl, dict):
                        continue
                    through = decl.get("workspace") is True
                    resolved = shared.get(alias) if through else decl
                    path = None
                    if isinstance(resolved, dict) and isinstance(resolved.get("path"), str):
                        path = resolve(ws[0] if through else crate, resolved["path"])
                        if path is None:
                            raise Refusal("escape")
                    out.append({"table": tname, "kind": kind, "alias": alias, "inherited": through, "path": path})
        out.sort(key=lambda r: (r["table"] + "\0" + r["kind"] + "\0" + r["alias"]).encode("utf8"))
        return out
    def identity(self, crate, tests, stack=()):
        key = ("t" if tests else "b") + "\0" + crate
        if key in stack:
            raise Refusal("cycle")
        if key in self.ids:
            return self.ids[key]
        manifest = self.read(at(crate, "Cargo.toml"))
        if manifest is None:
            value = sha(canon({"missing": crate}))
        else:
            ws = self.workspace(crate)
            wst = (ws[1].get("workspace") or {}) if ws else {}
            rows = self.rows(crate, manifest, ws, tests)
            children = []
            for p in dict.fromkeys(r["path"] for r in rows if r["path"] is not None):
                children.append(self.identity(p, False, stack + (key,)))
            wanted = manifest.get("package") or {}
            fields = wst.get("package") or {}
            taken = sorted((k for k, v in wanted.items() if isinstance(v, dict) and v.get("workspace") is True), key=lambda k: k.encode("utf8"))
            own = {
                "package": {k: fields.get(k) for k in taken},
                "lints": (wst.get("lints") if wst.get("lints") is not None else None) if (manifest.get("lints") or {}).get("workspace") is True else None,
                "dependencies": [[r["table"], r["kind"], r["alias"], (wst.get("dependencies") or {}).get(r["alias"])] for r in rows if r["inherited"]],
            }
            fixed = None
            if ws:
                m = ws[1]
                fixed = {"features": m.get("cargo-features"), "resolver": wst.get("resolver"), "profile": m.get("profile"), "patch": m.get("patch"), "replace": m.get("replace")}
            value = sha(canon({"own": own, "fixed": fixed, "children": sorted(children, key=lambda c: c.encode("utf8"))}))
        self.ids[key] = value
        return value

def pin(tree):
    if tree is None:
        return "missing"
    row = dict(tree.get("toolchain") or {})
    out = dict(row)
    out["components"] = sorted(map(str, row["components"]), key=lambda s: s.encode("utf8")) if isinstance(row.get("components"), list) else row.get("components")
    out["targets"] = sorted(map(str, row["targets"]), key=lambda s: s.encode("utf8")) if isinstance(row.get("targets"), list) else row.get("targets")
    return out

def config(tree, excluded):
    if tree is None:
        return "missing"
    copy = json.loads(json.dumps(tree))
    for key in excluded:
        parts = key.split(".")
        node = copy
        for p in parts[:-1]:
            node = node.get(p) if isinstance(node, dict) else None
        if isinstance(node, dict):
            node.pop(parts[-1], None)
    return copy

def tool(repo, policy):
    t = policy["toolchain"]
    return {"toolchain": sha(canon(pin(repo.read(t["pin"])))), "config": sha(canon(config(repo.read(t["config"]), t["configExcluded"])))}

def apply(tree, path, value):
    node = tree
    for p in path[:-1]:
        node = node[p]
    last = path[-1]
    if value is None:
        node.pop(last, None)
    elif isinstance(value, list) and value and value[0] == "$push":
        node[last].extend(value[1:])
    else:
        node[last] = value

def braces(p):
    m = re.search(r"\{([^{}]*)\}", p)
    if not m:
        return [p]
    return [x for c in m.group(1).split(",") for x in braces(p[: m.start()] + c + p[m.end():])]

def violation(policy, text):
    if text.startswith("!") or not text.startswith("{workspaceRoot}/"):
        return None
    bare = re.sub(r"^(?:\./)+", "", text[len("{workspaceRoot}/"):])
    if bare in policy["workspaceKeys"]["literals"]:
        return bare
    for alt in braces(bare):
        for tail in policy["workspaceKeys"]["tails"]:
            if alt == "**/" + tail:
                return "**/" + tail
    return None

def main():
    fixture = json.load(open(sys.argv[1], encoding="utf8"))
    policy = json.load(open(sys.argv[2], encoding="utf8"))
    queries = [(q.split(":")[0], q.split(":")[1] == "t") for q in fixture["queries"]]
    names = fixture["queries"]
    out = {"canonical": [canon(row["value"]) for row in fixture["canonical"]]}
    base = Repo(json.loads(json.dumps(fixture["trees"])))
    ids = {n: base.identity(c, t) for n, (c, t) in zip(names, queries)}
    out["base"] = ids
    out["tool"] = tool(base, policy)
    edits = {}
    for e in fixture["edits"]:
        trees = json.loads(json.dumps(fixture["trees"]))
        for path, value in e["set"]:
            apply(trees[e["path"]], path, value)
        repo = Repo(trees)
        edits[e["id"]] = [n for n, (c, t) in zip(names, queries) if repo.identity(c, t) != ids[n]]
    out["edits"] = edits
    refusals = {}
    for r in fixture["refusals"]:
        trees = {}
        for p, text in r["files"].items():
            trees[p] = r["trees"][p]
        try:
            Repo(trees).identity(r["crate"], r["tests"])
            refusals[r["id"]] = None
        except Refusal as error:
            refusals[r["id"]] = str(error)
    out["refusals"] = refusals
    out["keys"] = [violation(policy, row["input"]) for row in fixture["keys"]]
    print(json.dumps(out, ensure_ascii=False))

main()

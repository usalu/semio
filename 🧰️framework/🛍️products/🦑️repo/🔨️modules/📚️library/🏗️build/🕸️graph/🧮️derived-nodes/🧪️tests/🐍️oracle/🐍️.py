"""🐍 Independent model of the derived-node DAG with early cutoff.

One command reads one scenario document (see the schema `Scenario`) on stdin and prints, for the initial evaluation and
after every step, the value of each requested node and the exact set of nodes an incremental evaluator has to recompute:
  run -> {"steps": [{"name", "values": {label: value}, "recomputed": [label], "changed": [label], "created": [label]}]}
Labels are `kind|subject`. Every value is computed naively from the files (no memo). A node of the previous evaluation is
recomputed when a raw read of it now answers differently or when a node it read now has another value; a node never seen
before is created. Manifests are parsed with the full `tomllib` / `json` of the standard library, independent of the
restricted parsers of the implementation. Standard library only.
"""
import hashlib
import json
import pathlib
import re
import sys
import tomllib


def canon(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False)


def digest(value):
    return hashlib.sha256(canon(value).encode("utf8")).hexdigest()


def glob_regex(pattern):
    return re.compile(glob_body(pattern) + r"\Z")


def glob_body(pattern):
    out, at = "", 0
    while at < len(pattern):
        c = pattern[at]
        if c == "*" and pattern[at + 1:at + 2] == "*":
            if pattern[at + 2:at + 3] == "/":
                out += "(?:.*/)?"
                at += 2
            else:
                out += ".*"
                at += 1
        elif c == "*":
            out += "[^/]*"
        elif c == "?":
            out += "[^/]"
        elif c == "{" and "}" in pattern[at:]:
            end = pattern.index("}", at)
            out += "(?:" + "|".join(glob_body(p) for p in pattern[at + 1:end].split(",")) + ")"
            at = end
        else:
            out += re.escape(c)
        at += 1
    return out


def matcher(patterns):
    def clean(p):
        return re.sub(r"^(!?)\{projectRoot\}/", r"\1", p)
    inc = [glob_regex(clean(p)) for p in patterns if not p.startswith("!")]
    exc = [glob_regex(clean(p)[1:]) for p in patterns if p.startswith("!")]
    return lambda path: any(r.match(path) for r in inc) and not any(r.match(path) for r in exc)


def resolve_glob(root, pattern):
    clean = re.sub(r"^\{projectRoot\}/", "", pattern)
    if clean.startswith("{workspaceRoot}/"):
        return clean[16:]
    return clean if root == "" else root + "/" + clean


def split_glob(full):
    parts = full.split("/")
    at = next((i for i, p in enumerate(parts) if re.search(r"[*?{]", p)), None)
    cut = len(parts) - 1 if at is None else at
    return "/".join(parts[:cut]), "/".join(parts[cut:])


def spec_of(spec):
    if isinstance(spec, str):
        return {"version": spec}
    out = {k: spec[k] for k in ["version", "path", "git", "branch", "tag", "rev", "package", "optional", "default-features", "workspace"] if k in spec}
    if isinstance(spec.get("features"), list):
        out["features"] = sorted(spec["features"])
    return out


def cargo(text):
    toml = tomllib.loads(text)
    tables = {}
    sections = ["dependencies", "dev-dependencies", "build-dependencies"]
    found = {s: toml[s] for s in sections if s in toml}
    if "dependencies" in toml.get("workspace", {}):
        found["workspace.dependencies"] = toml["workspace"]["dependencies"]
    for target, body in toml.get("target", {}).items():
        for s in sections:
            if s in body:
                found["target." + target + "." + s] = body[s]
    for section, body in found.items():
        tables[section] = {name: spec_of(body[name]) for name in sorted(body)}
    return {"format": "Cargo.toml", "name": toml.get("package", {}).get("name"), "tables": tables}


def package(text):
    data = json.loads(text)
    out = {"format": "package.json", "name": data.get("name")}
    for s in ["dependencies", "devDependencies", "peerDependencies", "optionalDependencies"]:
        out[s] = {k: (data.get(s) or {})[k] for k in sorted(data.get(s) or {})}
    return out


def project_json(text):
    data = json.loads(text)
    targets = {n: {"command": t["command"] if t.get("command") is not None else (t.get("options") or {}).get("command"), "inputs": t["inputs"] if isinstance(t.get("inputs"), list) else []} for n, t in sorted((data.get("targets") or {}).items())}
    return {"format": "project.json", "name": data.get("name"), "implicitDependencies": sorted(set(data.get("implicitDependencies") or [])),
            "namedInputs": {n: v for n, v in sorted((data.get("namedInputs") or {}).items())}, "targets": targets}


def gomod(text):
    module, block, requires = None, False, []
    for raw in text.splitlines():
        line = re.sub(r"//.*$", "", raw).strip()
        if not line:
            continue
        if block:
            if line == ")":
                block = False
            else:
                p = line.split()
                requires.append([p[0], p[1] if len(p) > 1 else None])
            continue
        if line.startswith("module "):
            module = line[7:].strip()
        elif line in ("require (", "require("):
            block = True
        elif line.startswith("require "):
            p = line[8:].strip().split()
            requires.append([p[0], p[1] if len(p) > 1 else None])
    return {"format": "go.mod", "module": module, "requires": sorted(requires, key=lambda r: r[0])}


def manifest(path, text):
    name = path.rsplit("/", 1)[-1]
    try:
        if name == "package.json":
            return package(text)
        if name.endswith("project.json"):
            return project_json(text)
        if name == "Cargo.toml":
            return cargo(text)
        if name == "go.mod":
            return gomod(text)
    except Exception:
        return {"error": "unparseable"}
    return {"error": "unsupported"}


def dependency_names(m):
    if not m or "error" in m:
        return []
    f = m["format"]
    if f == "package.json":
        return [k for s in ["dependencies", "devDependencies", "peerDependencies", "optionalDependencies"] for k in m[s]]
    if f == "Cargo.toml":
        return [k for t in m["tables"].values() for k in t]
    if f == "go.mod":
        return [r[0] for r in m["requires"]]
    return m["implicitDependencies"]



PROJECT_CONFIG_FILES = ["📋️project.json", "project.json", "package.json", "Cargo.toml", "go.mod"]
PROJECT_ROSTER_PATTERNS = ["**/📋️project.json", "**/Cargo.toml"]


def config_entry(name, text):
    try:
        if name == "Cargo.toml":
            return {"ok": True, "toml": tomllib.loads(text)}
        if name == "go.mod":
            return {"ok": True, "go": gomod(text)}
        return {"ok": True, "json": json.loads(text)}
    except Exception:
        return {"ok": False, "sha256": hashlib.sha256(text.encode("utf8")).hexdigest(), "error": "<error>"}


def load_policies():
    library = pathlib.Path(__file__).resolve().parents[5]
    generated = json.loads((library / "⚡️caching" / "🔣️policy.json").read_text(encoding="utf8"))["generatedDirectories"]
    collections = json.loads((library / "🗂️workspaces" / "🔣️policy.json").read_text(encoding="utf8"))["collections"]
    return generated, collections["names"], collections["moduleMember"]


GENERATED, COLLECTIONS, MODULE_MEMBER = load_policies()


def admitted(path):
    """The admission of a project candidate, restated from the declared policies (no opaque payload names occur in the fixtures)."""
    parts = path.split("/")
    if not path or "\0" in path or "\ufffd" in path or path.startswith("/") or re.match(r"^[A-Za-z]:", path):
        return False
    if any(p in ("", ".", "..", ".🧬semio") or p in GENERATED for p in parts):
        return False
    if any(p in COLLECTIONS and (i == 0 or parts[i - 1] != MODULE_MEMBER) for i, p in enumerate(parts)):
        return False
    return not (path.startswith("compose/") or path.startswith("temp/compose/"))


def project_of(config, with_project):
    entry = config["files"].get("📋️project.json" if with_project else "Cargo.toml")
    if entry is None:
        return None, False
    if not entry["ok"]:
        return None, True
    name = (entry.get("json") or {}).get("name") if with_project else ((entry.get("toml") or {}).get("package") or {}).get("name")
    return (name if isinstance(name, str) and name != "" else None), False


class Evaluator:
    def __init__(self, scn, files):
        self.scn, self.files, self.memo = scn, files, {}

    def members(self, root, patterns):
        groups, exclude = [], []
        for pattern in patterns:
            if not isinstance(pattern, str):
                continue
            if pattern.startswith("!"):
                exclude.append(glob_regex(resolve_glob(root, pattern[1:])))
            else:
                base, rest = split_glob(resolve_glob(root, pattern))
                groups.append((base, glob_regex(rest)))

        def has(path):
            if any(r.match(path) for r in exclude):
                return False
            for base, rx in groups:
                if base == "":
                    rel = path
                elif path.startswith(base + "/"):
                    rel = path[len(base) + 1:]
                else:
                    continue
                if rx.match(rel):
                    return True
            return False
        return sorted(p for p in self.files if has(p))

    def sig(self, ref):
        if ref[0] == "file":
            return self.files.get(ref[1])
        if ref[0] == "observed":
            return self.names(ref[1])
        return self.members(ref[1], list(ref[2]))

    def names(self, directory):
        prefix = directory + "/" if directory else ""
        return sorted(set(p[len(prefix):].split("/", 1)[0] for p in self.files if p.startswith(prefix)))

    def globs(self, root, patterns, raws):
        paths = self.members(root, patterns)
        raws.append(("glob", root, tuple(patterns)))
        entry = 0
        for p in paths:
            raws.append(("file", p))
            entry += int(hashlib.sha256(canon([p, hashlib.sha256(self.files[p].encode("utf8")).hexdigest()]).encode("utf8")).hexdigest(), 16)
        return {"files": len(paths), "digest": format(entry % (1 << 256), "064x")}

    def node(self, label):
        if label in self.memo:
            return self.memo[label]
        kind, subject = label.split("|", 1)
        deps, raws = [], []

        def need(l):
            if l not in deps:
                deps.append(l)
            return self.node(l)["value"]

        value = getattr(self, "k_" + kind.replace("-", "_"))(subject, need, raws)
        rec = {"value": value, "deps": deps, "raws": raws}
        rec["sigs"] = [self.sig(r) for r in raws]
        self.memo[label] = rec
        return rec

    def k_manifest(self, subject, need, raws):
        raws.append(("file", subject))
        text = self.files.get(subject)
        return None if text is None else manifest(subject, text)

    def k_project(self, subject, need, raws):
        decl = self.scn["projects"][subject]
        values = [need("manifest|" + m) for m in decl["manifests"]]
        declared = next((v for v in values if v and v.get("format") == "project.json"), None)
        named = next((v for v in values if v and v.get("name")), None)
        name = (declared or {}).get("name")
        if name is None:
            name = (named or {}).get("name")
        return {"root": subject, "name": name, "dependencies": sorted(set(n for v in values for n in dependency_names(v))),
                "namedInputs": (declared or {}).get("namedInputs", {}), "targets": (declared or {}).get("targets", {})}

    def k_named_input(self, subject, need, raws):
        root, name = subject.split("#", 1)
        project = need("project|" + root)
        entries = [e for e in project["namedInputs"].get(name, []) if isinstance(e, str)]
        refs = sorted(set(e for e in entries if e != name and e in project["namedInputs"]))
        result = self.globs(root, [e for e in entries if e not in refs], raws)
        if not refs:
            return result
        return {"files": result["files"], "digest": digest([result["digest"]] + [digest(need("named-input|" + root + "#" + r)) for r in refs])}

    def k_target_inputs(self, subject, need, raws):
        root, target = subject.rsplit(":", 1)
        project = need("project|" + root)
        spec = project["targets"].get(target)
        if spec is None:
            return {"command": None, "inputs": []}
        named, globs = [], []
        for entry in [e for e in spec["inputs"] if isinstance(e, str)]:
            n = entry[1:] if entry.startswith("^") else entry
            (named if n in project["namedInputs"] else globs).append(n if n in project["namedInputs"] else entry)
        inputs = []
        for n in sorted(set(named)):
            inputs.append(["named:" + n, digest(need("named-input|" + root + "#" + n))])
        if globs:
            inputs.append(["globs", self.globs(root, globs, raws)["digest"]])
        return {"command": spec["command"], "inputs": inputs}

    def k_action_key(self, subject, need, raws):
        inputs = need("target-inputs|" + subject)
        env = (self.scn.get("actions", {}).get(subject) or {}).get("env", {})
        return {"key": digest({"command": inputs["command"], "inputs": inputs["inputs"], "env": env})}


    def k_project_config(self, subject, need, raws):
        base = "" if subject == "" else subject + "/"
        files = {}
        for name in PROJECT_CONFIG_FILES:
            raws.append(("file", base + name))
            text = self.files.get(base + name)
            files[name] = None if text is None else config_entry(name, text)
        return {"root": subject, "files": files}

    def k_project_set(self, subject, need, raws):
        listed = [p for p in self.members("", PROJECT_ROSTER_PATTERNS) if admitted(p)]
        self.globs("", PROJECT_ROSTER_PATTERNS, raws)
        with_project = set(p.rsplit("/", 1)[0] if "/" in p else "" for p in listed if p.endswith("📋️project.json"))
        cargo_roots = set(p.rsplit("/", 1)[0] if "/" in p else "" for p in listed if p.endswith("Cargo.toml") and not p.endswith("📋️project.json"))
        roots = sorted(with_project | cargo_roots)
        configs = [need("project-config|" + root) for root in roots]
        projects = []
        for root, c in zip(roots, configs):
            name, unavailable = project_of(c, root in with_project)
            projects.append({"root": root, "name": name, "unavailable": unavailable, "files": [n for n in PROJECT_CONFIG_FILES if c["files"][n] is not None]})
        names, duplicates = {}, set()
        for project in projects:
            if project["name"] is not None:
                if project["name"] in names:
                    duplicates.add(project["name"])
                else:
                    names[project["name"]] = project["root"]
        return {"projects": projects, "names": names, "duplicates": sorted(duplicates)}

    def k_task_key(self, subject, need, raws):
        root, target = subject.rsplit(":", 1)
        config = need("project-config|" + root)
        entry = config["files"].get("📋️project.json") or config["files"].get("project.json")
        declared = (self.scn.get("declarations") or {}).get(subject)
        spec = declared if declared is not None else (((entry or {}).get("json") or {}).get("targets") or {}).get(target)
        if spec is None:
            return {"key": None}
        inputs = need("target-inputs|" + subject)
        env = (self.scn.get("actions", {}).get(subject) or {}).get("env", {})
        return {"key": digest({"spec": spec, "inputs": inputs["inputs"], "env": env})}

    def k_directory_names(self, subject, need, raws):
        raws.append(("observed", subject))
        return {"names": self.names(subject)}

    def k_closure(self, subject, need, raws):
        return {"digest": digest([digest(need(m)) for m in self.scn["closures"][subject]["members"]])}


def run(scn):
    files = dict(scn["files"])
    ev = Evaluator(scn, files)
    old = {}
    steps = []

    def close(label, ev):
        seen, stack = [], [label]
        while stack:
            l = stack.pop()
            if l in seen:
                continue
            seen.append(l)
            stack.extend(ev.node(l)["deps"])
        return seen

    created = []
    for r in scn["requests"]:
        for l in close(r, ev):
            if l not in old:
                old[l] = ev.node(l)
                created.append(l)
    steps.append({"name": "initial", "values": {r: ev.node(r)["value"] for r in scn["requests"]}, "recomputed": sorted(created), "changed": [], "created": sorted(created)})
    for step in scn["steps"]:
        for path, text in step["edit"].items():
            if text is None:
                files.pop(path, None)
            else:
                files[path] = text
        new = Evaluator(scn, files)
        visited, recomputed, changed, made = set(), set(), set(), set()

        def visit(label):
            if label in visited:
                return
            visited.add(label)
            n, o = new.node(label), old.get(label)
            if o is None:
                recomputed.add(label)
                made.add(label)
            else:
                for d in o["deps"]:
                    visit(d)
                dep_changed = any(new.node(d)["value"] != old[d]["value"] for d in o["deps"])
                raw_changed = any(new.sig(r) != s for r, s in zip(o["raws"], o["sigs"]))
                if not (dep_changed or raw_changed):
                    return
                recomputed.add(label)
                if n["value"] != o["value"]:
                    changed.add(label)
            for d in n["deps"]:
                visit(d)

        for label in list(old) + scn["requests"]:
            visit(label)
        for label in visited:
            old[label] = new.node(label)
        steps.append({"name": step["name"], "values": {r: new.node(r)["value"] for r in scn["requests"]}, "recomputed": sorted(recomputed), "changed": sorted(changed), "created": sorted(made)})
    return {"steps": steps}


if __name__ == "__main__":
    if sys.argv[1:] == ["run"]:
        print(json.dumps(run(json.load(sys.stdin)), ensure_ascii=False))
    else:
        sys.exit(2)

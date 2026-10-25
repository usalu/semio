#!/usr/bin/env python3
"""🐍️ Independent implementation of the course input-closure law, written without the TypeScript compiler, Bun, esbuild or the repository's inference: it reads the fixture, expands the manifest inputs with its own glob translator, scans imports with regular expressions, parses Cargo path dependencies with regular expressions and prints, per cached target, the files the target reads and the part of them no declared or inferred input covers.

usage: 🐍️.py <fixture.json> <workspace> <manifest.json> <inferred.json> [<target>=<closure.json> ...]
Each closure file is the JSON the inference returns for that target: {"closure": ["{workspaceRoot}/path", ...]}."""
import json, os, re, sys

fixture_path, workspace, manifest_path = sys.argv[1:4]
inferred = {}
for pair in sys.argv[4:]:
    name, path = pair.split("=", 1)
    inferred[name] = {p.replace("{workspaceRoot}/", "", 1) for p in json.load(open(path, encoding="utf8"))["closure"]}
fixture = json.load(open(fixture_path, encoding="utf8"))
manifest = json.load(open(manifest_path, encoding="utf8"))
site, project = fixture["site"], fixture["project"]

def sub(text):
    return text.replace("{site}", site).replace("{project}", project)

PRESENTATION = set(json.load(open(os.path.join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/🔣️policy.json"), encoding="utf8"))["presentationResources"])
SCHEDULING = tuple(prefix for domain in json.load(open(os.path.join(workspace, "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🔏️closure/🗓️scheduling/🔣️.json"), encoding="utf8"))["domains"] for prefix in domain["prefixes"])
SKIPPED = {"node_modules", "target", "dist", ".git", ".cache"}

def listing():
    found = [n for n in os.listdir(workspace) if os.path.isfile(os.path.join(workspace, n))]
    for top in fixture["inventory"]:
        for directory, names, leaves in os.walk(os.path.join(workspace, top)):
            names[:] = [n for n in names if n not in SKIPPED]
            found += [os.path.relpath(os.path.join(directory, n), workspace).replace(os.sep, "/") for n in leaves]
    return sorted(set(found))

files = listing()
file_set = set(files)

def glob_regex(pattern):
    out, i = "", 0
    while i < len(pattern):
        c = pattern[i]
        if pattern.startswith("**/", i):
            out += "(?:.*/)?"; i += 3; continue
        if pattern.startswith("**", i):
            out += ".*"; i += 2; continue
        if c == "*": out += "[^/]*"
        elif c == "?": out += "[^/]"
        elif c == "{":
            j = pattern.index("}", i)
            out += "(?:" + "|".join(re.escape(part) for part in pattern[i + 1:j].split(",")) + ")"; i = j + 1; continue
        else: out += re.escape(c)
        i += 1
    return re.compile("^" + out + "$")

def expand(inputs):
    nx = json.load(open(os.path.join(workspace, "nx.json"), encoding="utf8")).get("namedInputs", {})
    named = {"default": ["{projectRoot}/**/*"], **nx, **manifest.get("namedInputs", {})}
    patterns = []
    def walk(items, depth=0):
        assert depth < 20
        for item in items:
            if not isinstance(item, str) or item.startswith("^"): continue
            if item in named: walk(named[item], depth + 1); continue
            patterns.append(item.replace("{projectRoot}", project).replace("{workspaceRoot}/", "").lstrip("/"))
    walk(inputs)
    selected = set()
    for p in [p for p in patterns if not p.startswith("!")]:
        if re.search(r"[*?{\[]", p): rx = glob_regex(p); selected.update(f for f in files if rx.match(f))
        elif p in file_set: selected.add(p)
    for p in [p[1:] for p in patterns if p.startswith("!")]:
        rx = glob_regex(p); selected = {f for f in selected if not rx.match(f)}
    return selected

def packages():
    root = os.path.join(workspace, fixture["workspaces"])
    base = os.path.dirname(root)
    members = json.load(open(root, encoding="utf8"))["workspaces"]
    table, manifests = {}, [fixture["workspaces"]]
    for member in members:
        directory = os.path.normpath(os.path.join(base, member))
        relative = os.path.relpath(directory, workspace)
        path = os.path.join(directory, "package.json")
        if not os.path.exists(path): continue
        manifests.append(relative + "/package.json")
        data = json.load(open(path, encoding="utf8"))
        exports = data.get("exports")
        if isinstance(exports, dict) and any(k.startswith(".") for k in exports): entries = exports.items()
        elif exports is not None: entries = [(".", exports)]
        else: entries = []
        for key, target in entries:
            resolved = pick(target)
            if resolved: table[data["name"] + key[1:]] = os.path.normpath(os.path.join(relative, resolved))
    return table, manifests

def pick(target):
    if isinstance(target, str): return target
    if isinstance(target, list): return next((r for r in map(pick, target) if r), None)
    if isinstance(target, dict):
        for condition in ("bun", "node", "import", "default"):
            if condition in target:
                resolved = pick(target[condition])
                if resolved: return resolved
    return None

IMPORT = re.compile(r"""(?:^|[\s;{}()])(?:import|export)\s+(?!type\b)(?:[^'"`;]*?\s+from\s+)?["']([^"']+)["']|\bimport\(\s*["']([^"']+)["']\s*\)|\brequire\(\s*["']([^"']+)["']\s*\)""", re.M)
TYPE_IMPORT = re.compile(r"""(?:^|[\s;{}])(?:import|export)\s+type\s+[^'"`;]*?\s+from\s+["']([^"']+)["']""", re.M)
URL = re.compile(r"""new\s+URL\(\s*["']([^"']+)["']\s*,\s*import\.meta\.url""")
SOURCE = re.compile(r"\.[cm]?[jt]sx?$")

def resolve_relative(origin, specifier, types=False):
    base = os.path.normpath(os.path.join(os.path.dirname(origin), specifier))
    candidates = [base]
    stem, ext = os.path.splitext(base)
    if ext in (".js", ".mjs", ".cjs"): candidates += [stem + e for e in (".ts", ".tsx", ".mts", ".cts")]
    candidates += [base + e for e in (".ts", ".tsx", ".mts", ".js", ".mjs", ".json")] + [base + "/" + n for n in ("🟦️.ts", "🟦️.tsx", "index.ts", "index.js")]
    found = next((c for c in candidates if c in file_set), None)
    if types and found and re.search(r"\.[cm]?js$", found):
        declaration = re.sub(r"\.(m|c)?js$", lambda m: ".d." + (m.group(1) or "") + "ts", found)
        if declaration in file_set: return declaration
    return found

TABLE, WORKSPACE_MANIFESTS = packages()

def resolve_bare(specifier):
    return TABLE.get(specifier)

RUNTIME_EXPORT = re.compile(r"\bexport\s*(?:default|const|let|var|function|class|enum|async|\*|\{(?!\s*type\b))|\bimport\s*[\"']|\bexport\s+declare\s+(?:const|function|class|enum)")
ELIDED = {}

def type_only(path):
    if path not in ELIDED:
        if not path.endswith((".ts", ".tsx", ".mts", ".cts")) or path.endswith((".d.ts", ".d.mts")): ELIDED[path] = path.endswith((".d.ts", ".d.mts"))
        else:
            text = open(os.path.join(workspace, path), encoding="utf8", errors="ignore").read()
            text = re.sub(r"/\*.*?\*/", "", text, flags=re.S)
            text = re.sub(r"(?m)^\s*//.*$", "", text)
            text = re.sub(r"\b(?:import|export)\s*\{\s*(?:type\s+[^,}]+,?\s*)+\}\s*from\s*[\"'][^\"']+[\"']", "", text)
            ELIDED[path] = RUNTIME_EXPORT.search(text) is None and "import.meta" not in text
    return ELIDED[path]

def closure(entries, types=False):
    seen, pending = set(), [e for e in entries if e in file_set]
    while pending:
        path = pending.pop()
        if path in seen: continue
        seen.add(path)
        if not SOURCE.search(path) or (types and re.search(r"\.[cm]?jsx?$", path)): continue
        try: text = open(os.path.join(workspace, path), encoding="utf8", errors="ignore").read()
        except OSError: continue
        text = re.sub(r"/\*.*?\*/", "", text, flags=re.S)
        text = re.sub(r"(?m)^\s*//.*$", "", text)
        if not types and "import.meta.vitest" in text: text = text[:text.index("import.meta.vitest")]
        if not types: text = re.sub(r"\b(?:import|export)\s*\{\s*(?:type\s+[^,}]+,?\s*)+\}\s*from\s*[\"'][^\"']+[\"']", "", text)
        specifiers = [next(g for g in m.groups() if g) for m in IMPORT.finditer(text)]
        if types: specifiers += [m.group(1) for m in TYPE_IMPORT.finditer(text)]
        for specifier in specifiers:
            target = resolve_relative(path, specifier, types) if specifier.startswith(".") else (resolve_bare(specifier) if not specifier.startswith(("node:", "bun:")) else None)
            if target and target in file_set and (types or not type_only(target)): pending.append(target)
        for literal in URL.findall(text):
            target = os.path.normpath(os.path.join(os.path.dirname(path), literal))
            if target in file_set: pending.append(target)
    return seen

def nearest_tsconfig(path):
    directory = os.path.dirname(path)
    while directory:
        candidate = directory + "/tsconfig.json"
        if candidate in file_set: return candidate
        directory = os.path.dirname(directory)
    return "tsconfig.json" if "tsconfig.json" in file_set else None

def strip_comments(text):
    return re.sub(r'(?m)^\s*//.*$|(?<![:"\w])//[^\n"]*$', "", text)

def tsconfig_files(path):
    seen, files_, include_roots = [], set(), []
    current = path
    while current and current not in seen:
        seen.append(current)
        data = json.loads(strip_comments(open(os.path.join(workspace, current), encoding="utf8").read()))
        files_.add(current)
        if "include" in data and not include_roots:
            base = os.path.dirname(current)
            for pattern in data["include"]:
                full = os.path.normpath(os.path.join(base, pattern)).replace("\\", "/")
                rx = glob_regex(full if re.search(r"[*?]", full) else full + "/**/*")
                include_roots += [f for f in files if rx.match(f) and re.search(r"\.(?:[cm]?tsx?|json)$", f) and not f.endswith(".json")]
        parent = data.get("extends")
        current = os.path.normpath(os.path.join(os.path.dirname(current), parent)) if parent else None
    return files_, set(include_roots)

def vitest_entries(config):
    text = open(os.path.join(workspace, config), encoding="utf8").read()
    match = re.search(r"include:\s*\[(.*?)\]", text, re.S)
    names = re.findall(r'"([^"]+)"', match.group(1)) if match else []
    return [config] + [os.path.normpath(os.path.join(site, n)) for n in names]

def cargo_dependencies(manifest_rel, workspace_paths):
    text = open(os.path.join(workspace, manifest_rel), encoding="utf8").read()
    base = os.path.dirname(manifest_rel)
    found, section = set(), ""
    for line in text.split("\n"):
        header = re.match(r"^\s*\[([^\]]+)\]\s*$", line)
        if header: section = header.group(1).strip(); continue
        if not re.search(r"(?:^|\.)(?:dependencies|build-dependencies)$", section) or section.startswith("dev-") or "workspace.dependencies" in section: continue
        match = re.match(r"^\s*([A-Za-z0-9_\-]+)\s*=\s*\{([^}]*)\}", line)
        if not match: continue
        name, body = match.group(1), match.group(2)
        if re.search(r"optional\s*=\s*true", body): continue
        path = re.search(r'path\s*=\s*"([^"]+)"', body)
        if path: found.add(os.path.normpath(os.path.join(base, path.group(1))))
        elif re.search(r"workspace\s*=\s*true", body) and name in workspace_paths: found.add(workspace_paths[name])
    return found

def rust_crates(spec):
    root_manifest = spec["workspaceManifest"]
    root_text = open(os.path.join(workspace, root_manifest), encoding="utf8").read()
    base = os.path.dirname(root_manifest)
    workspace_paths = {}
    section = re.search(r"\[workspace\.dependencies\](.*?)(?=^\[|\Z)", root_text, re.S | re.M)
    for match in re.finditer(r'^\s*([A-Za-z0-9_\-]+)\s*=\s*\{[^}\n]*path\s*=\s*"([^"]+)"', section.group(1) if section else "", re.M):
        workspace_paths[match.group(1)] = os.path.normpath(os.path.join(base, match.group(2)))
    start = os.path.normpath(os.path.join(os.path.dirname(spec["package"]), "."))
    seen, pending = set(), [start]
    while pending:
        directory = pending.pop()
        if directory in seen: continue
        seen.add(directory)
        manifest_rel = directory + "/Cargo.toml"
        if manifest_rel in file_set: pending += list(cargo_dependencies(manifest_rel, workspace_paths))
    out = set()
    for directory in seen:
        for f in files:
            if f.startswith(directory + "/") and not any(part in ("target", "node_modules", "dist") for part in f.split("/")):
                if re.search(spec["pattern"], f.split("/")[-1]): out.add(f)
    return out, workspace_paths

result = {}
for name, spec in fixture["targets"].items():
    target = manifest["targets"][name]
    declared = expand(target.get("inputs", ["default"]))
    covered = declared | inferred.get(name, set())
    reads = set()
    for read in spec["reads"]:
        kind = read["kind"]
        if kind == "vitest": reads |= closure(vitest_entries(sub(read["config"])))
        elif kind == "imports": entries = [sub(e) for e in read["entries"]]; reads |= closure(entries + [resolve_relative(e, s) for e in entries for s in read.get("specifiers", [])] if "specifiers" in read else entries, types=read.get("types", False)) | (set(entries) if "specifiers" in read else set())
        elif kind == "workspace-manifests": reads |= set(WORKSPACE_MANIFESTS) | {os.path.dirname(WORKSPACE_MANIFESTS[0]) + "/bun.lock"}
        elif kind == "nearest-tsconfig": reads |= {t for t in (nearest_tsconfig(f) for f in list(reads) if SOURCE.search(f)) if t}
        elif kind == "files": reads |= {f for f in map(sub, read["paths"]) if f in file_set}
        elif kind == "tsconfig":
            configs, roots = tsconfig_files(sub(read["path"]))
            reads |= configs | closure(roots, types=True)
        elif kind == "rust": crates, _ = rust_crates({k: sub(v) if isinstance(v, str) else v for k, v in read.items()}); reads |= crates
    reads = {r for r in reads if (r in file_set or os.path.exists(os.path.join(workspace, r))) and r not in PRESENTATION and not r.startswith(SCHEDULING)}
    generated = {r for r in reads if "/🤖️generated/" in r}
    gap = sorted(r for r in reads - generated if r not in covered)
    result[name] = {"reads": sorted(reads), "generated": sorted(generated), "gap": gap, "declared": sorted(declared)}
print(json.dumps(result, ensure_ascii=False))

"""Shared Python-oracle helpers of the lawcov-laws tests: Rust line masking, test spans, file-tree access over an in-memory file map.

This module is inlined into every oracle by gen scripts (kept textual so every oracle stays one standalone file)."""
import json, re, sys, os

A = re.ASCII


def expand(case):
    files = {}
    for path, lines in case["files"].items():
        files[path] = "\n".join(lines) + "\n" if lines else ""
    for g in case.get("generate", []):
        for i in range(g["count"]):
            files[g["path"].replace("{i}", str(i))] = "\n".join(l.replace("{i}", str(i)) for l in g.get("lines", []))
    return files


def mask(line):
    line = re.sub(r'"(?:[^"\\\n]|\\.)*"', lambda m: '"' + " " * (len(m.group(0)) - 2) + '"', line)
    return re.sub(r"'(?:\\.|[^'\\\n])'", lambda m: "'" + " " * (len(m.group(0)) - 2) + "'", line)


def code_only(raw):
    return re.sub(r"//.*$", "", mask(raw))


def split_lines(text):
    return re.split(r"\r?\n", text)


def spans_of_mods(lines):
    stack, spans, depth = [], [], 0
    for i, raw in enumerate(lines):
        co = code_only(raw)
        m = re.match(r"^\s*(?:pub\s+)?mod\s+(\w+)\b.*\{\s*$", raw, A)
        if m:
            prev = lines[max(0, i - 2):i]
            is_test = any(re.search(r"#\[cfg\([^\]]*\btest\b[^\]]*\)\]", l, A) for l in prev) or m.group(1) == "tests"
            stack.append((m.group(1), i + 1, depth, is_test))
        depth += co.count("{") - co.count("}")
        while stack and depth <= stack[-1][2]:
            name, start, _, is_test = stack.pop()
            if is_test:
                spans.append((start, i + 1))
    return spans


def in_spans(spans, line_no):
    return any(a <= line_no <= b for a, b in spans)


SKIP_NAMES = ("compose", "node_modules", ".git", ".🧬️semio", "target", "dist", "build", "coverage", "🤖️generated", ".claude", "vendor", ".venv", ".turbo", ".nx", ".cache", ".storybook", "storybook-static")


def walk_files(files, under="", skip=("compose", "node_modules", ".git", ".🧬️semio", "target", "dist", "build", "coverage", "🤖️generated", ".claude", "vendor", ".venv", ".turbo", ".nx", ".cache", ".storybook", "storybook-static")):
    for path in sorted(files):
        parts = path.split("/")
        if under and not (path == under or path.startswith(under + "/")):
            continue
        if any(p in skip for p in parts[:-1]):
            continue
        yield path


def output(result):
    sys.stdout.write(json.dumps(result, ensure_ascii=False))


def jssort(items):
    return sorted(items, key=lambda s: s.encode("utf-16-be"))


class Tree:
    def __init__(self, files):
        self.files = files
        self.dirs = set()
        for p in files:
            parts = p.split("/")
            for i in range(1, len(parts)):
                self.dirs.add("/".join(parts[:i]))

    def exists(self, path):
        return path in self.files or path in self.dirs

    def children(self, d):
        out = {}
        pref = d + "/" if d else ""
        for p in self.files:
            if p.startswith(pref):
                head = p[len(pref):].split("/")
                out.setdefault(head[0], len(head) > 1)
        return out

    def walk(self, roots, pred, skip=None, skip_names=SKIP_NAMES):
        found = set()
        for root in roots:
            pref = root + "/" if root else ""
            for p in self.files:
                if root and not p.startswith(pref):
                    continue
                rel = p[len(pref):].split("/")
                dirs, name = rel[:-1], rel[-1]
                if any(d in skip_names or d.startswith(".") or (skip and skip(pref + "/".join(dirs[:i + 1]), d)) for i, d in enumerate(dirs)):
                    continue
                if pred(p, name):
                    found.add(p)
        return sorted(found, key=lambda s: s.encode("utf-16-be"))

import posixpath
DOMAIN_ALLOW = {"flow", "trinity_jack", "trinity_ram", "mathematical_graph_drawing", "mathematical_geometry"}
JSON_ALLOW_PREFIXES = ("coda/",)
STRUCTURAL = ("@doc", "@edit", "@change", "@checkpoint", "@alternative", "@active")


def strip_non_ascii(s):
    return re.sub(r"[^\x00-\x7f]", "", s)


def row(i, kind, prio, line=0):
    return [i, kind, prio, line]


def plugin_owners(t):
    owners = []
    for p, c in t.files.items():
        if p.endswith("/📦️packages/🦀️rust/Cargo.toml") and re.search(r'^role\s*=\s*"plugin"', c, re.M):
            owners.append(p[: -len("/📦️packages/🦀️rust/Cargo.toml")])
    return sorted(set(owners), key=lambda o: -len(o))


def scope_key(t, rel):
    for o in plugin_owners(t):
        if rel == o or rel.startswith(o + "/"):
            return strip_non_ascii(o.split("/")[-1])
    return ""


def priority(crate, p):
    return p if crate["pluginId"] else "low"


def cargo_artifacts(t, crates):
    return [row("stray-cargo-artifact-%s-Cargo.lock" % c["dir"], "app-plugin/stray-cargo-artifact", priority(c, "high")) for c in crates if (c["dir"] + "/Cargo.lock") in t.files]


def app_coupling(t, crates):
    dirs = {c["dir"] for c in crates}
    out = []
    for c in crates:
        manifest = c["dir"] + "/Cargo.toml"
        if manifest not in t.files:
            continue
        for m in re.finditer(r'^([\w.-]+)\s*=\s*\{[^\n]*?\bpath\s*=\s*"([^"]+)"[^\n]*\}\s*$', t.files[manifest], re.M | A):
            name, dep = m.group(1), m.group(2)
            resolved = posixpath.normpath(posixpath.join(c["dir"], dep))
            other = scope_key(t, resolved)
            if not other or other == c["pluginId"]:
                continue
            if resolved in dirs:
                out.append(row("app-coupling-plugin-%s-%s" % (c["dir"], name), "app-plugin/app-coupling", priority(c, "medium")))
                continue
            if name in DOMAIN_ALLOW:
                continue
            out.append(row("app-coupling-domain-%s-%s" % (c["dir"], name), "app-plugin/app-coupling", "low"))
    return out


def json_fixtures(t):
    out = []
    for p in t.walk([""], lambda p, n: n.endswith(".json") and p.split("/")[-2:-1] == ["example"]):
        if p.startswith(JSON_ALLOW_PREFIXES):
            continue
        out.append(row("no-json-fixture-" + p, "dsl-migration/no-json-fixture", "high"))
    return out


def ops_grammar(t):
    out = []
    for p in t.walk([""], lambda p, n: n.endswith(".ops")):
        raw = split_lines(t.files[p])
        lines = raw[:-1] if raw and raw[-1] == "" else raw
        for i, line in enumerate(lines):
            no = i + 1
            if not line.strip():
                out.append(row("ops-grammar-blank-%s-%d" % (p, no), "dsl-migration/ops-grammar", "medium", no))
                continue
            if not (line.startswith(STRUCTURAL) or (line.startswith("  ") and line.strip())):
                out.append(row("ops-grammar-line-%s-%d" % (p, no), "dsl-migration/ops-grammar", "medium", no))
    return out


def run(c):
    t = Tree(expand(c))
    crates = c.get("crates", [])
    return {"policyCargoArtifactBreaches": lambda: cargo_artifacts(t, crates), "policyAppCouplingBreaches": lambda: app_coupling(t, crates), "policyJsonFixtureBreaches": lambda: json_fixtures(t), "policyOpsGrammarBreaches": lambda: ops_grammar(t)}[c["rule"]]()


cases = json.load(open(sys.argv[1], encoding="utf8"))["cases"]
output({c["id"]: run(c) for c in cases})

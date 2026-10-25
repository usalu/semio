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


def strip_ts_text(content):
    out, i, n = [], 0, len(content)
    while i < n:
        ch, nx = content[i], content[i + 1] if i + 1 < n else None
        if ch == "/" and nx == "/":
            while i < n and content[i] != "\n":
                out.append(" ")
                i += 1
            continue
        if ch == "/" and nx == "*":
            end = content.find("*/", i + 2)
            end = n if end == -1 else end + 2
            out.append(" " * (end - i))
            i = end
            continue
        if ch in "\"'`":
            j = i + 1
            while j < n:
                if content[j] == "\\":
                    j += 2
                    continue
                if content[j] == ch:
                    j += 1
                    break
                j += 1
            out.append(" " * (min(j, n) - i))
            i = min(j, n)
            continue
        out.append(ch)
        i += 1
    return "".join(out)


def strip_rust_text(content):
    out, i, n = [], 0, len(content)
    while i < n:
        two = content[i:i + 2]
        if two == "//":
            while i < n and content[i] != "\n":
                i += 1
            continue
        if two == "/*":
            i += 2
            depth = 1
            while i < n and depth > 0:
                pair = content[i:i + 2]
                if pair == "/*":
                    depth += 1
                    i += 2
                elif pair == "*/":
                    depth -= 1
                    i += 2
                else:
                    i += 1
            continue
        if content[i] == '"':
            out.append('"')
            i += 1
            while i < n and content[i] != '"':
                i += 2 if content[i] == "\\" else 1
            if i < n:
                i += 1
            out.append('"')
            continue
        out.append(content[i])
        i += 1
    return "".join(out)

import posixpath
CLOSURES = "🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/🏗️build/📥️source/🛗️zero-touch/🗂️closures/🔣️.json"
BUILTINS = {"fs", "path", "os", "crypto", "util", "url", "child_process", "events", "stream", "http", "zlib", "assert", "buffer", "readline", "module", "worker_threads"}
EXTS = [".ts", ".tsx", ".mts", ".js", ".mjs"]
CODE = re.compile(r"\.(?:[cm]?[jt]s|tsx)$")


def specifiers(text):
    found = {}
    for m in re.finditer(r"""(?:^|[\s;{}(])(?:import|export)\b[^'"`;]*?\bfrom\s*['"]([^'"]+)['"]""", text, re.M):
        found[("static", m.group(1))] = 1
    for m in re.finditer(r"""(?:^|[\s;{}(])import\s*['"]([^'"]+)['"]""", text, re.M):
        found[("static", m.group(1))] = 1
    for m in re.finditer(r"""\bimport\(\s*['"]([^'"]+)['"]\s*\)""", text):
        found[("dynamic", m.group(1))] = 1
    for m in re.finditer(r"""\brequire\(\s*['"]([^'"]+)['"]\s*\)""", text):
        found[("require", m.group(1))] = 1
    return sorted(found, key=lambda kv: (kv[1].encode(), kv[0].encode()))


def relative_target(t, frm, spec):
    base = posixpath.normpath(posixpath.join(posixpath.dirname(frm), spec.split("?")[0]))
    for cand in [base] + [base + e for e in EXTS]:
        if CODE.search(cand) and cand in t.files:
            return cand
    return None


def check(t):
    roster = json.loads(t.files[CLOSURES])
    pkg = json.loads(t.files["package.json"])
    packages = {}
    for d in pkg.get("workspaces", []):
        p = d + "/package.json"
        if p in t.files:
            j = json.loads(t.files[p])
            if isinstance(j.get("name"), str) and "exports" in j:
                packages[j["name"]] = (d, {".": j["exports"]} if isinstance(j["exports"], str) else j["exports"])
    seams = {s["module"] for s in roster["seams"]}
    out = []

    def add(closure, kind, file, spec):
        out.append(["zero-touch-%s-%s-%s-%s" % (closure, kind, file, spec), "repo/zero-touch-closure", "high", False, file])

    for s in roster["seams"]:
        if s["module"] not in t.files:
            add("seams", "missing-entry", s["module"], s["module"])
    for c in roster["closures"]:
        seen, pending = set(), [c["entry"]]
        if c["entry"] not in t.files:
            add(c["id"], "missing-entry", c["entry"], c["entry"])
        while pending:
            f = pending.pop()
            if f in seen:
                continue
            seen.add(f)
            if f not in t.files:
                continue
            for kind, spec in specifiers(t.files[f]):
                if c["mode"] == "static" and kind != "static":
                    continue
                if spec.startswith("."):
                    target = relative_target(t, f, spec)
                    if target is None:
                        continue
                    if target not in seams:
                        pending.append(target)
                    elif kind == "static":
                        add(c["id"], "seam-static", f, spec)
                elif spec.startswith(("node:", "bun:")) or spec == "bun" or spec in BUILTINS:
                    continue
                elif any(spec == n or spec.startswith(n + "/") for n in packages):
                    add(c["id"], "workspace-specifier", f, spec)
                else:
                    add(c["id"], "external-package", f, spec)
    return out


def run(c):
    return sorted([[r[0], r[1], r[2], r[3], r[4]] for r in check(Tree(expand(c)))], key=lambda r: json.dumps(r, ensure_ascii=False, separators=(",", ":")).encode("utf-16-be"))


cases = json.load(open(sys.argv[1], encoding="utf8"))["cases"]
output({c["id"]: run(c) for c in cases})

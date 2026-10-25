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

UI_ROOTS = ["🧰️framework/🔨️modules/🖱️ui/", "🧰️framework/🛍️products/💻️os/🔨️modules/📺️renderer/", "🧰️framework/🛍️products/💻️os/🖥️host/", "✏️s/🔌️plugins/"]
BRIDGE = ["🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/🖥️host/🧵️shard/🧵️executor/🦀️.rs"]
SANCTIONED = ["🧰️framework/🔨️modules/⏳️async/"]
ROOTS = ["✏️s", "🧰️framework", "🌎️hub"]
UI_PATTERNS = [("blocking-bridge", r"\bblock_on\s*\("), ("blocking-bridge", r"\brun_blocking\s*\("), ("sync-fs", r"\bstd::fs::"), ("sync-net", r"\bstd::net::"), ("sync-net", r"\breqwest::blocking\b"), ("sync-clipboard", r"\barboard::"), ("sync-process", r"\bstd::process::Command\b"), ("sync-db", r"\brusqlite::"), ("sync-db", r"\bsqlx::")]
RUNTIME_PATTERNS = [("thread-pool", r"\b(?:std::)?thread::spawn\s*\("), ("thread-pool", r"\b(?:std::)?thread::Builder\b"), ("thread-pool", r"\b(?:std::)?thread::scope\s*\("), ("thread-pool", r"\bWorkerPool::new\s*\("), ("thread-pool", r"\brayon::"), ("thread-pool", r"\btokio::runtime::Builder\b")]


def is_runtime_source(p):
    return any(p == r or p.startswith(r + "/") for r in ROOTS) and "/🧫️fixtures/" not in p and "/🧪️tests/" not in p and "/tests/" not in p and "/benches/" not in p and "/examples/" not in p and not p.endswith("/build.rs")


def mask_comments(raw, depth):
    m = mask(raw)
    code, i = "", 0
    while i < len(m):
        pair = m[i:i + 2]
        if depth > 0:
            if pair == "/*":
                depth += 1
                i += 2
            elif pair == "*/":
                depth -= 1
                i += 2
            else:
                i += 1
            continue
        if pair == "//":
            break
        if pair == "/*":
            depth = 1
            i += 2
            continue
        code += m[i]
        i += 1
    return code, depth


def same_line_kind(item):
    depth = 0
    for ch in item:
        if ch in "([":
            depth += 1
        elif ch in ")]":
            if depth == 0:
                return "member"
            depth -= 1
        elif depth == 0 and ch == ",":
            return "member"
        elif depth == 0 and ch in "{;":
            return "item"
    return "item"


def cfg_test_item_spans(lines):
    spans, stack, pending, depth = [], [], None, 0
    for i, raw in enumerate(lines):
        co = code_only(raw)
        attr = re.search(r"#\[cfg\((?![^\]]*\bnot\s*\()[^\]]*\btest\b[^\]]*\)\]", co, A)
        if attr:
            pending = i + 1
        same = co[attr.end():] if pending == i + 1 and attr else ""
        if same.strip() and not re.match(r"^\s*#\[", same) and same_line_kind(same) == "member":
            pending = None
        item = "" if pending is None else (co if i + 1 > pending else same)
        if pending is not None and item.strip() and not re.match(r"^\s*#\[", item):
            if item.count("{") > 0:
                stack.append((pending, depth))
                pending = None
            elif re.search(r";\s*$", item):
                spans.append((pending, i + 1))
                pending = None
            elif re.search(r",\s*$", item):
                pending = None
        depth += co.count("{") - co.count("}")
        while stack and depth <= stack[-1][1]:
            start, _ = stack.pop()
            spans.append((start, i + 1))
    return spans


def scan_file(t, path, patterns):
    content = t.files[path]
    if not content:
        return []
    lines = re.split(r"\r?\n", content)
    spans = spans_of_mods(lines) + cfg_test_item_spans(lines)
    depth, out = 0, []
    for i, raw in enumerate(lines):
        code, depth = mask_comments(raw, depth)
        if in_spans(spans, i + 1):
            continue
        for cat, rx in patterns:
            if re.search(rx, code, A):
                out.append([cat, path, i + 1, raw.strip()[:200]])
    return out


def scan(t):
    out = []
    for p in t.walk(ROOTS, lambda p, n: n.endswith(".rs")):
        if not is_runtime_source(p):
            continue
        if any(p.startswith(r) for r in UI_ROOTS) or p in BRIDGE:
            out += scan_file(t, p, UI_PATTERNS)
        if not any(p.startswith(r) for r in SANCTIONED):
            out += scan_file(t, p, RUNTIME_PATTERNS)
    return sorted(out, key=lambda r: json.dumps(r, ensure_ascii=False, separators=(",", ":")).encode("utf-16-be"))


def run(c):
    if c["rule"] == "interactivityIsRuntimeSource":
        return is_runtime_source(c["args"][0])
    return scan(Tree(expand(c)))


cases = json.load(open(sys.argv[1], encoding="utf8"))["cases"]
output({c["id"]: run(c) for c in cases})

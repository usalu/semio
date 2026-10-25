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

BUILDER = re.compile(r'\b(?:MutationOutcome::(error|fatal)|MutationMessage::(info|warning|error|fatal))\s*\(\s*"([^"]*)"', A)
CHAIN = re.compile(r'\.(info|warning)\s*\(\s*"([^"]*)"', A)
RETIRED = re.compile(r"(?:\bMutationMessage::|\.)warn\s*\(", A)
APPLY = re.compile(r'\bMutationApplyError(?:::new\s*\(\s*|\s*\{\s*code:\s*)"([^"]*)"', A)
TS_REFUSE = re.compile(r'\brefuse\(\s*"(info|warn|warning|error|fatal)"\s*,\s*"([^"]*)"', A)
TS_OBJECT = re.compile(r'\blevel:\s*"(info|warn|warning|error|fatal)"\s*,\s*code:\s*"(mutation\.[^"]*)"|\bcode:\s*"(mutation\.[^"]*)"\s*,\s*level:\s*"(info|warn|warning|error|fatal)"', A)
PY_TUPLE = re.compile(r'\(\s*"(info|warn|warning|error|fatal)"\s*,\s*"([a-z0-9]+(?:[.-][a-z0-9]+)+)"\s*\)', A)
FEATURE_MESSAGE = re.compile(r"\blevel:\s*([a-z]+)\s*,\s*code:\s*(mutation\.[a-z0-9][a-z0-9.-]*[a-z0-9])(?![\w.<*-])", A)
LEVEL_ALIAS = re.compile(r'(?:"[a-z]+"\s*\|\s*)*"warn"\s*(?:\|\s*"[a-z]+"\s*)*=>\s*(?:Ok\(|Some\()?(?:[A-Za-z_]\w*::)*Severity::Warning\b', A)
LITERAL = re.compile(r"""["'`](mutation\.[a-z0-9][a-z0-9.-]*)["'`]""", A)
FEATURE_TOKEN = re.compile(r"(?<![\w.-])(mutation\.[a-z0-9][a-z0-9-]*(?:\.[a-z0-9][a-z0-9-]*)*)(?![\w-]|\.[<*])", A)
K = "mutation-migration/message-code"


def line_of(c, i):
    return c[:i].count("\n") + 1


def body_mask(content):
    masked = "\n".join(mask(l) for l in content.split("\n"))
    close, stack = {}, []
    for i, ch in enumerate(masked):
        if ch == "{":
            stack.append(i)
        elif ch == "}" and stack:
            close[stack.pop()] = i
    inside = [False] * len(content)
    for m in re.finditer(r"\bfn\s+[A-Za-z_]\w*", masked, A):
        op = masked.find("{", m.start())
        end = close.get(op, -1) if op != -1 else -1
        if end != -1 and "MutationOutcome" in content[op:end + 1]:
            for i in range(op, end + 1):
                inside[i] = True
    return inside


def outcome_code_file(vocab, path, content):
    out = []
    if not content or (path.endswith(".py") and "🧪️tests/" not in path):
        return out
    codes = {c["code"]: c["level"] for c in vocab["codes"]}
    apply_re = re.compile(vocab["applyPattern"])
    reported = set()

    def check(code, level, index, apply=False):
        expected = codes.get(code) or (vocab["applyLevel"] if apply_re.search(code) else None)
        if apply:
            fault = None if apply_re.search(code) else "x"
        elif expected is None:
            fault = "x"
        else:
            fault = "x" if level is not None and level != expected else None
        if fault is None:
            return
        n = line_of(content, index)
        if (n, code) in reported:
            return
        reported.add((n, code))
        out.append(["mutation-message-code-%s-%d-%s" % (path, n, code), K, "high", n])

    leaf = any(s in "/" + path for s in ("/🧬️mutations/", "/🔺️diff/"))
    if path.endswith(".rs"):
        for m in BUILDER.finditer(content):
            check(m.group(3), m.group(1) or m.group(2), m.start())
        for m in APPLY.finditer(content):
            check(m.group(1), None, m.start(), True)
        if leaf:
            for m in LITERAL.finditer(content):
                check(m.group(1), None, m.start())
        bodies = body_mask(content) if "MutationOutcome" in content else [False] * len(content)
        for m in CHAIN.finditer(content):
            if bodies[m.start()]:
                check(m.group(2), m.group(1), m.start())
        for m in RETIRED.finditer(content):
            if m.group(0).startswith(".") and not bodies[m.start()]:
                continue
            n = line_of(content, m.start())
            out.append(["mutation-message-retired-builder-%s-%d-%d" % (path, n, m.start()), K, "high", n])
        for m in LEVEL_ALIAS.finditer(content):
            n = line_of(content, m.start())
            out.append(["mutation-message-level-alias-%s-%d" % (path, n), K, "high", n])
    elif path.endswith(".ts") or path.endswith(".tsx"):
        for m in TS_REFUSE.finditer(content):
            check(m.group(2), m.group(1), m.start())
        for m in TS_OBJECT.finditer(content):
            check(m.group(2) or m.group(3), m.group(1) or m.group(4), m.start())
        if leaf:
            for m in LITERAL.finditer(content):
                check(m.group(1), None, m.start())
    elif path.endswith(".py"):
        for m in PY_TUPLE.finditer(content):
            check(m.group(2), m.group(1), m.start())
        for m in LITERAL.finditer(content):
            check(m.group(1), None, m.start())
    elif path.endswith(".feature"):
        for m in FEATURE_TOKEN.finditer(content):
            if not re.search(r"\.(json|rs|ts|tsx|py|md|txt)$", m.group(1)):
                check(m.group(1), None, m.start())
        for m in FEATURE_MESSAGE.finditer(content):
            check(m.group(2), m.group(1), m.start())
    else:
        try:
            doc = json.loads(content)
        except ValueError:
            return out
        if not isinstance(doc, dict):
            return out
        cursor = 0
        messages = doc.get("messages") if isinstance(doc.get("messages"), list) else []
        for msg in [doc] + messages:
            if isinstance(msg, dict) and isinstance(msg.get("code"), str):
                at = content.find(json.dumps(msg["code"], ensure_ascii=False), cursor)
                if at != -1:
                    cursor = at + 1
                level = msg.get("level") if isinstance(msg.get("level"), str) else None
                check(msg["code"], level, max(at, 0))
    return out


cases = json.load(open(sys.argv[1], encoding="utf8"))["cases"]
output({c["id"]: outcome_code_file(c["vocabulary"], c["path"], "\n".join(c["source"]) + "\n") for c in cases})

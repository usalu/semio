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

OS_ROOT = "🧰️framework/🛍️products/💻️os/"
EXCLUDED = ("compose/", "♻️mit-bestand/", "🌎️hub/")
ROOT_SCRIPT_DIRS = ["🧰️framework/🛍️products/🦑️repo/🔨️modules/📚️library/" + d for d in ("🎛️commands", "⚖️laws/🎯️tool-job-coverage", "⚖️laws/🔬️interactivity-audit", "⚖️laws/🧩️plugin-consistency", "⚖️laws/🧪️self-test-lanes", "🗄️stdio-ledger", "🧹️normalization/🔣️taxonomy/🏷️filenames", "🔍️discovery/📖️source-access/🦀️rust-evidence", "🗂️workspaces/📍️checkout-root")]
PACKAGE_SCRIPT_DIRS = ["🧰️framework/🛍️products/💻️os/📦️packages/🦀️rust/🎛️commands"]
NAME_RE = re.compile(r"(Store|Registry|Host|Session|Engine|Kernel|World|Scene|State|Cache)$")
INTERIOR = re.compile(r"\b(?:OnceLock|OnceCell|LazyLock|Mutex|RwLock|RefCell|Cell)\s*<", A)
STATIC_MUT = re.compile(r"^\s*(?:pub(?:\s*\([^)]*\))?\s+)?static\s+mut\b", A)
STATIC_CONST = re.compile(r"^\s*(?:pub(?:\s*\([^)]*\))?\s+)?(?:static|const)\s+(?:mut\s+)?\w+\s*:\s*([^;=]+)", A)
FN_ITEM = re.compile(r"^\s*(?:pub(?:\s*\([^)]*\))?\s+)?(?:async\s+)?(?:unsafe\s+)?(?:const\s+)?fn\s+\w+", A)
STRUCT = re.compile(r"^\s*(?:pub(?:\s*\([^)]*\))?\s+)?struct\s+(\w+)\b", A)
FIELD_MAP = re.compile(r"^\s*(?:pub(?:\s*\([^)]*\))?\s+)?(?:r#)?\w+\s*:\s*[^=\n]*(?:\bHashMap\s*<|\bBTreeMap\s*<)", A)
SEQ = re.compile(r"\bseq\s*:\s*u32\b", A)
ATOMIC = re.compile(r"\bAtomicU(?:32|64)\b", A)
TLS = re.compile(r"\bthread_local!\s*(?:\{|\()", A)
LAZY = re.compile(r"\blazy_static!\s*\{", A)
SPAWN = re.compile(r"\b(spawnSync|execSync|execFileSync|Bun\.spawn|spawn)\s*\(")


def row(i, kind, line=0):
    return [i, kind, "high", line]


def rust_files(t):
    return t.walk([""], lambda p, n: n.endswith(".rs"))


def cargo_files(t):
    return t.walk([""], lambda p, n: n == "Cargo.toml")


def db_server_only(t):
    out = []
    for p in cargo_files(t):
        d = p[: p.rindex("/") + 1] if "/" in p else ""
        if d.startswith(("db/", "hub/")):
            continue
        for m in re.finditer(r'^(db(?:_[a-z0-9]+)*)\s*=\s*\{[^\n]*?\bpath\s*=\s*"([^"]+)"[^\n]*\}\s*$', t.files[p], re.M):
            out.append(row("db-server-only-%s-%s" % (p, m.group(1)), "protocol-migration/db-server-only"))
    return out


def os_state(t):
    out = []
    for p in rust_files(t):
        n = re.sub(r"^\./", "", p)
        if n.startswith(OS_ROOT) or n.startswith(EXCLUDED):
            continue
        lines = split_lines(t.files[p])
        spans = spans_of_mods(lines)
        depth, fn_depths, structs = 0, [], []
        for i, raw in enumerate(lines):
            no = i + 1
            co = code_only(raw)
            delta = co.count("{") - co.count("}")
            if in_spans(spans, no):
                depth += delta
                while fn_depths and depth < fn_depths[-1]:
                    fn_depths.pop()
                while structs and depth <= structs[-1][1]:
                    structs.pop()
                continue
            in_fn = any(depth >= d for d in fn_depths)
            opens = (not in_fn) and FN_ITEM.search(raw) and "{" in co
            sm = None if in_fn else STRUCT.search(raw)
            if not in_fn:
                if STATIC_MUT.search(raw):
                    out.append(row("os-state-authority-static-mut-%s-%d" % (p, no), "os-state-authority/item-scope-global", no))
                if TLS.search(co):
                    out.append(row("os-state-authority-thread-local-%s-%d" % (p, no), "os-state-authority/item-scope-global", no))
                if LAZY.search(co):
                    out.append(row("os-state-authority-lazy-static-%s-%d" % (p, no), "os-state-authority/item-scope-global", no))
                sc = STATIC_CONST.search(raw)
                ty = sc.group(1) if sc else ""
                if ty and INTERIOR.search(ty):
                    name = re.search(r"\b(OnceLock|OnceCell|LazyLock|Mutex|RwLock|RefCell|Cell)\s*<", ty, A)
                    out.append(row("os-state-authority-static-%s-%s-%d" % (name.group(1) if name else "interior", p, no), "os-state-authority/item-scope-global", no))
                if SEQ.search(co) or ATOMIC.search(co):
                    out.append(row("os-state-authority-id-mint-%s-%d" % (p, no), "os-state-authority/id-minting", no))
                top = structs[-1] if structs and depth > structs[-1][1] else None
                if top and top[2] and FIELD_MAP.search(raw):
                    out.append(row("os-state-authority-map-field-%s-%d-%s" % (p, no, top[0]), "os-state-authority/authority-struct-map", no))
            if opens:
                fn_depths.append(depth + 1)
            if sm and "{" in co:
                structs.append((sm.group(1), depth, bool(NAME_RE.search(sm.group(1)))))
            depth += delta
            while fn_depths and depth < fn_depths[-1]:
                fn_depths.pop()
            while structs and depth <= structs[-1][1]:
                structs.pop()
    return out


def extract_body(content, start):
    masked = "\n".join(mask(l) for l in content.split("\n"))
    op = masked.find("{", start)
    if op == -1:
        return ""
    depth = 0
    for i in range(op, len(masked)):
        if masked[i] == "{":
            depth += 1
        elif masked[i] == "}":
            depth -= 1
            if depth == 0:
                return content[op:i + 1]
    return content[op:]


def declares_fields(content, name):
    m = re.search(r"(?:pub(?:\s*\([^)]*\))?\s+)?struct\s+%s\b" % re.escape(name), content, A)
    if not m:
        return None
    line = content[:m.start()].count("\n") + 1
    rest = content[m.end():]
    o = re.match(r"^\s*([(;{])", rest)
    if not o or o.group(1) == ";":
        return (line, False)
    if o.group(1) == "(":
        tup = re.match(r"^\s*\(([^)]*)\)", rest)
        return (line, len(re.sub(r"//[^\n]*", "", tup.group(1) if tup else "").strip()) > 0)
    body = extract_body(content, m.start())
    if not body.startswith("{"):
        return (line, False)
    stripped = re.sub(r"/\*[\s\S]*?\*/", "", re.sub(r"//[^\n]*", "", body[1:-1])).strip()
    return (line, len(stripped) > 0)


def document_app_usages(content):
    out = []
    for m in re.finditer(r"impl\s+DocumentApp\s+for\s+(\w+)\s*\{", content, A):
        body = extract_body(content, m.start())
        if re.search(r"type\s+Snapshot\s*=\s*([\w:<>]+)\s*;", body, A) and re.search(r"type\s+Mutation\s*=\s*([\w:<>]+)\s*;", body, A):
            out.append((content[:m.start()].count("\n") + 1, m.group(1)))
    return out


def document_app_shape(t):
    out = []
    for p in rust_files(t):
        if re.sub(r"^\./", "", p).startswith(EXCLUDED):
            continue
        content = t.files[p]
        spans = spans_of_mods(split_lines(content))
        for line, app in document_app_usages(content):
            if in_spans(spans, line):
                continue
            decl = declares_fields(content, app)
            if decl and decl[1]:
                out.append(row("document-app-shape-%s-%s" % (p, app), "os-state-authority/document-app-shape", decl[0]))
    return out


def no_pack(t):
    return [row("no-pack-file-" + p, "pack-migration/no-pack-file") for p in t.walk([""], lambda p, n: n.endswith(".pack"))]


def strip_ts(content):
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


def script_files(t):
    found = set(t.walk([""], lambda p, n: n == "📜️script.ts"))
    for d in ROOT_SCRIPT_DIRS + PACKAGE_SCRIPT_DIRS:
        found |= set(t.walk([d], lambda p, n: n == "🟦️.ts", skip=lambda rel, name: name == "🧪️tests"))
    return sorted(found, key=lambda s: s.encode("utf-16-be"))


def raw_spawn(t):
    out = []
    for p in script_files(t):
        content = t.files[p]
        seen = set()
        for m in SPAWN.finditer(strip_ts(content)):
            line = len(re.split(r"\r?\n", content[:m.start()]))
            if line not in seen:
                seen.add(line)
                out.append(row("no-raw-spawn-%s-%d" % (p, line), "budget/no-raw-spawn", line))
    return out


def budget_null(t):
    out = []
    for p in script_files(t):
        content = t.files[p]
        seen = set()
        for m in re.finditer(r"budgetMs:\s*null", content):
            line = len(re.split(r"\r?\n", content[:m.start()]))
            if line not in seen:
                seen.add(line)
                out.append(row("no-budget-null-%s-%d" % (p, line), "budget/no-budget-null", line))
    return out


def run(c):
    t = Tree(expand(c))
    r = c["rule"]
    if r == "policyStripTsCommentsAndStrings":
        return [strip_ts(c["input"])]
    if r == "policyDiscoverCargoTomlFiles":
        return cargo_files(t)
    return {"policyDbServerOnlyBreaches": db_server_only, "policyOsStateAuthorityBreaches": os_state, "policyDocumentAppShapeBreaches": document_app_shape, "policyNoPackFilesBreaches": no_pack, "policyRawSpawnBreaches": raw_spawn, "policyBudgetNullBreaches": budget_null}[r](t)


cases = json.load(open(sys.argv[1], encoding="utf8"))["cases"]
output({c["id"]: run(c) for c in cases})

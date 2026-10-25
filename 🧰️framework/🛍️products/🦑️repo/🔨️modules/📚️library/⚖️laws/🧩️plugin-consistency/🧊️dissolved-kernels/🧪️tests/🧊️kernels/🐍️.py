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

import unicodedata

ALIASES = {"protocol": "spr"}
LEAF = "🦀️.rs"


def strip_emoji(s):
    return re.sub(r"[^\x00-\x7f]", "", s)


def app_id(path):
    seg = path.split("/")
    if "🎛️apps" in seg:
        i = seg.index("🎛️apps")
        if len(seg) > i + 1:
            return strip_emoji(seg[i + 1])
    return ""


def component(seg):
    a = strip_emoji(seg)
    return ALIASES.get(a, a)


def file_suffix(tail, default):
    if len(tail) == 1 and tail[0] == default:
        return ""
    return "#" + "-".join(strip_emoji(re.sub(r"\.rs$", "", s)) for s in tail)


def normalize_rel(rel):
    norm = rel[2:] if rel.startswith(".") else rel
    seg = norm.split("/")
    if "⚡️implementations" in seg and seg.index("⚡️implementations") > 1:
        ii = seg.index("⚡️implementations")
        module = seg[ii - 1]
        suffix = file_suffix(seg[ii + 2:], LEAF)
        chain = [s for s in seg[:ii - 1] if s not in ("🔨️modules", "🛂️manifest")]
        if "🔌️plugins" in chain and len(chain) > chain.index("🔌️plugins") + 1:
            pi = chain.index("🔌️plugins")
            plugin = strip_emoji(chain[pi + 1])
            app = strip_emoji(chain[chain.index("🎛️apps") + 1]) if "🎛️apps" in chain and len(chain) > chain.index("🎛️apps") + 1 else None
            comp = component(module)
            return ("%s/%s/%s" % (plugin, app, comp) if app and app != plugin else "%s/%s" % (plugin, comp)) + suffix
        owner = strip_emoji(chain[-1] if chain else "")
        if owner:
            return "%s/%s%s" % (owner, component(module), suffix)
    if "🗿️artifacts" in seg:
        ai = seg.index("🗿️artifacts")
        if ai > 0 and len(seg) > ai + 2:
            plugin = strip_emoji(seg[ai - 1])
            art = strip_emoji(seg[ai + 1])
            comp = component(seg[ai + 2])
            suffix = file_suffix(seg[ai + 3:], LEAF)
            return ("%s/%s/%s" % (plugin, art, comp) if art and art != plugin else "%s/%s" % (plugin, comp)) + suffix
    return norm


def region_events(lines):
    out = []
    for i, line in enumerate(lines):
        for kind, rx in (("open", r"^(\s*)//(\s*)#region(?:\s+(.*))?\s*$"), ("close", r"^(\s*)//(\s*)#endregion(?:\s+(.*))?\s*$")):
            m = re.match(rx, line)
            if m:
                out.append({"kind": kind, "line": i + 1, "spaceAfterSlashes": len(m.group(2) or "") > 0, "label": (m.group(3) or "").strip()})
                break
    return out


def pair_spans(events):
    stack, out = [], []
    for ev in events:
        if ev["kind"] == "open":
            stack.append(ev)
            continue
        if not stack:
            continue
        op = stack.pop()
        out.append({"label": op["label"], "closeLabel": ev["label"], "startLine": op["line"], "endLine": ev["line"]})
    return out


def mod_spans(lines):
    stack, out, depth = [], [], 0
    for i, raw in enumerate(lines):
        co = code_only(raw)
        m = re.match(r"^\s*pub mod (\w+)\b.*\{\s*$", raw, A)
        if m:
            stack.append((m.group(1), i + 1, depth))
        depth += co.count("{") - co.count("}")
        while stack and depth <= stack[-1][2]:
            name, start, _ = stack.pop()
            out.append({"name": name, "startLine": start, "endLine": i + 1})
    return out


def mod_at(spans, line):
    inside = sorted((s for s in spans if s["startLine"] <= line <= s["endLine"]), key=lambda s: s["endLine"] - s["startLine"])
    return inside[0]["name"] if inside else ""


def label_name(label):
    i = 0
    while i < len(label) and not unicodedata.category(label[i]).startswith("L"):
        i += 1
    return label[i:].strip()


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


def pascal(i):
    return "".join(p[:1].upper() + p[1:] for p in re.split(r"[-_]+", i) if p) + "App"



REP_TYPES = ["HalfedgeMesh", "BrepEngineHost", "DrawingStore", "DrawingEngine", "EngineCache"]
CACHE_DIRS = ["🧰️framework/🔨️modules/◻️2d/🧮️compute", "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin"]
REP_ALLOW = {"✏️s/🔌️plugins/📐️cad/🗿️artifacts/📐️cad/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️.rs", "✏️s/🔌️plugins/🏭️process/🗿️artifacts/🧊️process3d/🏅️standards/🔖️1/🪆️subsets/✳️any/⚙️engine/🦀️.rs", "🧰️framework/🔨️modules/◻️2d/🗄️store/🦀️.rs", "🧰️framework/🔨️modules/🧊️3d/📐️brep/⚙️engine/🖥️host/🦀️.rs"}
CANONICAL_TYPES = ["SemioAnimationSnapshot", "SemioAudioSnapshot", "SemioBrepSnapshot", "SemioCadSnapshot", "SemioDocumentSnapshot", "SemioDrawingSnapshot", "SemioFlowSnapshot", "SemioGraphSnapshot", "SemioImageSnapshot", "SemioKitSnapshot", "SemioMeshSnapshot", "SemioModelSnapshot", "SemioObjectSnapshot", "SemioPresentationSnapshot", "SemioTableSnapshot", "SemioTextSnapshot", "SemioValueSnapshot", "SemioVideoSnapshot"]
CHILD = {"ts": r"@child\s+kind\s*=\s*([^\s*/]+)", "json": r'"x-semio-child"\s*:\s*"([^"]*)"', "graphql": r'@child\s*\(\s*kind\s*:\s*"([^"]*)"\s*\)', "proto": r"@child\s+kind\s*=\s*([^\s*/]+)"}


def row(i, kind, prio, line=0):
    return [i, kind, prio, line]


def rust(t):
    return t.walk([""], lambda p, n: n.endswith(".rs"))


def cache_owner(p):
    return any(p == d or p.startswith(d + "/") for d in CACHE_DIRS)


def rep_escape(t):
    out = []
    for p in rust(t):
        c = t.files[p]
        held = [r for r in REP_TYPES if re.search(r"^\s*(pub\s+)?[a-z_][a-z0-9_]*\s*:\s*[^,;]*\b%s\b" % r, c, re.M | A) or re.search(r"static\s+[A-Z0-9_]+\s*:\s*[^=;]*\b%s\b" % r, c, A)]
        allow = normalize_rel(p) in REP_ALLOW
        if held:
            if not allow:
                out.append(row("dissolved-rep-escape-" + p, "dissolved-kernels/rep-escape", "medium"))
        elif allow:
            out.append(row("dissolved-rep-escape-stale-" + p, "dissolved-kernels/rep-escape", "low"))
    return out


def cache_scope(t):
    return [row("dissolved-engine-cache-scope-" + p, "dissolved-kernels/engine-cache-scope", "medium") for p in rust(t) if not cache_owner(p) and re.search(r"\bEngineCache::new\b", t.files[p], A)]


def replace(t):
    seen = {}
    for p in rust(t):
        if "🧬️mutations/" in p and "/🟤️set-snapshot/" in p:
            d = p[: p.index("/🟤️set-snapshot/") + len("/🟤️set-snapshot")]
            seen["dissolved-whole-document-replace-" + d] = row("dissolved-whole-document-replace-" + d, "dissolved-kernels/whole-document-replace", "medium")
    return list(seen.values())


def canonical(kind):
    seg = kind.split(".")
    ok = lambda s: len(s) > 0 and not s.startswith("-") and not s.endswith("-") and "--" not in s and re.fullmatch(r"[a-z0-9-]+", s)
    return len(seg) == 3 and seg[0] == "s" and bool(ok(seg[1])) and bool(ok(seg[2]))


def commented(c, idx):
    start = c.rfind("\n", 0, idx) + 1
    return "//" in c[start:idx]


def artifact_kinds(t):
    out = []
    for p in rust(t):
        c = t.files[p]
        if "ArtifactKindSpec" not in c:
            continue
        for m in re.finditer(r'ArtifactKindSpec\s*\{\s*id:\s*"((?:[^"\\]|\\.)*)"', c):
            if commented(c, m.start()) or canonical(m.group(1)):
                continue
            n = c[:m.start()].count("\n") + 1
            out.append(row("canonical-artifact-kind-%s-%d" % (p, n), "composition/canonical-artifact-kind", "medium", n))
    return out


def child_kinds(t):
    out = []
    pred = lambda p, n: "/🧬️schema/" in p and "/🧪️tests/" not in p and "/🧫️fixtures/" not in p and (n.endswith(".rs") or n.split(".")[-1] in CHILD)
    for p in t.walk(["✏️s/🔌️plugins"], pred):
        fmt = p.split(".")[-1]
        if fmt == "rs":
            continue
        c = t.files[p]
        for m in re.finditer(CHILD[fmt], c):
            start = c.rfind("\n", 0, m.start()) + 1
            if fmt == "graphql" and "#" in c[start:m.start()]:
                continue
            if not canonical(m.group(1)):
                n = c[:m.start()].count("\n") + 1
                out.append(row("canonical-child-kind-%s-%d" % (p, n), "composition/canonical-child-kind", "high", n))
    return out


def redefinition(t):
    out = []
    for p in rust(t):
        if "🔌️plugins/" not in p or "🔌️plugins/🗄️stdio/" in p:
            continue
        c = t.files[p]
        for name in CANONICAL_TYPES:
            if name not in c:
                continue
            m = re.search(r"^[ \t]*(?:pub(?:\([^)]*\))?\s+)?(?:struct|enum)\s+%s\b" % name, c, re.M | A)
            if not m or commented(c, m.start()):
                continue
            n = c[:m.start()].count("\n") + 1
            out.append(row("dissolved-kind-redefinition-%s-%s" % (p, name), "composition/dissolved-kind-redefinition", "high", n))
    return out


def run(c):
    t = Tree(expand(c))
    r = c["rule"]
    return {
        "policyDissolvedKernelsBreaches": lambda: rep_escape(t) + cache_scope(t) + replace(t), "policyCanonicalArtifactKindBreaches": lambda: artifact_kinds(t),
        "policyCanonicalChildKindBreaches": lambda: child_kinds(t), "policyDissolvedKindRedefinitionBreaches": lambda: redefinition(t),
        "policyCompositionBreaches": lambda: artifact_kinds(t) + child_kinds(t) + redefinition(t),
    }[r]()


cases = json.load(open(sys.argv[1], encoding="utf8"))["cases"]
output({c["id"]: run(c) for c in cases})

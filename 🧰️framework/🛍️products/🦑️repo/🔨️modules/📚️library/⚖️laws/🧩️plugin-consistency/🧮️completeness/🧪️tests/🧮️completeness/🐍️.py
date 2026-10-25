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



GENERIC_BRIDGES = {"Value", "SetDocumentMutation"}
PACK_ALLOW = set()
ENVELOPE_ALLOW = {"architect/spine", "os/dsl", "products/os", "lowpoly/spr", "os/workflow", "animate/presentation/spr", "gis/artifact", "mathematical/equation/artifact", "note/artifact", "raster/artifact", "reasoning/artifact", "space/artifact", "vcs/artifact"}
FACETS = {"⚙️engine", "🎒️pack", "📡️spr", "🔧️op", "🔺️diff", "🗣️dsl"}
NAMES = ["Mutation", "MutationDiff", "MutationMeta", "OpText", "Edit", "merge_concurrent_diffs", "ReconcileReport", "ReconcileSeverity", "HybridLogicalTimestamp", "MutationEnvelope", "MutationDag", "UndoPolicy", "MergeStrategyKind", "HubClientFrame", "HubServerFrame"]
QUALIFIED = re.compile(r"\bvcs::(%s)\b" % "|".join(NAMES), A)
USE_BLOCK = re.compile(r"use\s+(?:::)?vcs::\{([^}]*)\}", re.S | A)
DERIVE = re.compile(r"derive\([^)]*\bDsl(?:Document|Ops|Record|Enum|Scalar)\b[^)]*\)", A)


def row(i, kind, prio="high", line=0):
    return [i, kind, prio, line]


def usages(scope, content):
    out = []
    for m in re.finditer(r"impl\s+DocumentApp\s+for\s+(\w+)\s*\{", content, A):
        body = extract_body(content, m.start())
        s = re.search(r"type\s+Snapshot\s*=\s*([\w:<>]+)\s*;", body, A)
        mu = re.search(r"type\s+Mutation\s*=\s*([\w:<>]+)\s*;", body, A)
        if not s or not mu:
            continue
        out.append({"line": content[:m.start()].count("\n") + 1, "app": m.group(1), "snapshot": re.sub(r"<.*$", "", s.group(1).split("::")[-1]), "mutation": re.sub(r"<.*$", "", mu.group(1).split("::")[-1])})
    return out


def rust_files(t):
    return t.walk([""], lambda p, n: n.endswith(".rs"))


def dsl_completeness(t):
    files = [(p, t.files[p]) for p in rust_files(t)]
    doc, ops, alias = set(), set(), {}
    for _, c in files:
        lines = c.split("\n")
        for m in re.finditer(r"^\s*(?:pub\s+)?(?:struct|enum)\s+(\w+)\b", c, re.M | A):
            n = c[:m.start()].count("\n") + 1
            if DERIVE.search("\n".join(lines[max(0, n - 6): n - 1])):
                doc.add(m.group(1))
                ops.add(m.group(1))
        for m in re.finditer(r"impl(?:<[^>]*>)?\s+(?:vcs::|protocol::)?(DocumentDsl|OpText)\s+for\s+(?:vcs::|protocol::)?(\w+)", c, A):
            (doc if m.group(1) == "DocumentDsl" else ops).add(m.group(2))
        for m in re.finditer(r"\b(\w+)\s+as\s+(\w+)\b", c, A):
            alias[m.group(2)] = m.group(1)

    def resolve(name):
        for _ in range(5):
            nxt = alias.get(name)
            if not nxt or nxt == name:
                break
            name = nxt
        return name

    out = []
    for p, c in files:
        for u in usages(p, c):
            for label, name, names in (("Snapshot", u["snapshot"], doc), ("Mutation", u["mutation"], ops)):
                if name in GENERIC_BRIDGES or name in names or resolve(name) in names:
                    continue
                out.append(row("dsl-completeness-%s-%d-%s" % (p, u["line"], label), "dsl-migration/completeness", "high", u["line"]))
    return out


def pack(t):
    out = []
    for p in rust_files(t):
        if normalize_rel(p) in PACK_ALLOW:
            continue
        c = t.files[p]
        if not ("assert_dsl_round_trip(" in c or "assert_document_text_round_trip(" in c):
            continue
        if "assert_dsl_pack_equivalence(" in c or "assert_document_pack_round_trip(" in c:
            continue
        out.append(row("pack-completeness-" + p, "pack-migration/completeness"))
    return out


def envelope(t):
    out = []
    for p in rust_files(t):
        if normalize_rel(p) in ENVELOPE_ALLOW:
            continue
        c = t.files[p]
        if not ("assert_dsl_pack_equivalence(" in c or "assert_document_pack_round_trip(" in c) or "assert_command_envelope_round_trip" in c:
            continue
        out.append(row("command-envelope-completeness-" + p, "protocol-migration/command-envelope-completeness"))
    return out


def ts_facade(t):
    out = []
    for p in t.walk(["✏️s"], lambda p, n: n == "🟦️.ts"):
        if "/🗿️artifacts/" not in p:
            continue
        c = t.files[p]
        stub = re.search(r"wire\s+.+\s+to plugin WASM", c, re.I) or re.search(r"""throw new Error\(\s*["'`][^"'`]*WASM[^"'`]*["'`]\s*\)""", c)
        parts = p.split("/")
        if not stub or ("🗿️artifacts" in parts and parts[-2] in FACETS):
            continue
        out.append(row("ts-facade-misplaced-stub-" + p, "dsl-migration/ts-facade-completeness", "medium"))
    return out


def protocol_migration(t):
    out = []
    for p in rust_files(t):
        c = t.files[p]
        lines = re.split(r"\r?\n", c)
        seen = set()
        line_of = lambda i: len(re.split(r"\r?\n", c[:i]))
        for m in QUALIFIED.finditer(c):
            if c[max(0, m.start() - 7): m.start()] == "crate::":
                continue
            n = line_of(m.start())
            if re.match(r"^\s*(//|\*)", lines[n - 1] if n - 1 < len(lines) else "") or n in seen:
                continue
            seen.add(n)
            out.append(row("protocol-migration-%s-%d" % (p, n), "protocol-migration/vcs-shim-regression", "high", n))
        for m in USE_BLOCK.finditer(c):
            tokens = {x for x in re.split(r"[^\w]+", m.group(1), flags=A) if x}
            if not any(n in tokens for n in NAMES):
                continue
            n = line_of(m.start())
            if n in seen:
                continue
            seen.add(n)
            out.append(row("protocol-migration-use-%s-%d" % (p, n), "protocol-migration/vcs-shim-regression", "high", n))
    return out


def run(c):
    t = Tree(expand(c))
    r = c["rule"]
    if r == "policyDocumentAppUsages":
        return ["%s|%d|%s|%s|%s" % (c["scope"], u["line"], u["app"], u["snapshot"], u["mutation"]) for u in usages(c["scope"], "\n".join(c["source"]) + "\n")]
    if r in ("policyGrammarFileBreaches", "policyProtocolFileBreaches"):
        return []
    return {"policyDslCompletenessBreaches": dsl_completeness, "policyPackCompletenessBreaches": pack, "policyCommandEnvelopeCompletenessBreaches": envelope, "policyTsFacadeBreaches": ts_facade, "policyProtocolMigrationBreaches": protocol_migration}[r](t)


cases = json.load(open(sys.argv[1], encoding="utf8"))["cases"]
output({c["id"]: run(c) for c in cases})

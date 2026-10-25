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



CRDT = re.compile(r"\b(merge_strategy|MergeStrategyKind|merge_concurrent_diffs|ConflictRule|ResolutionPlan|assert_crdt_)", A)
VALIDATE = re.compile(r"\bfn\s+validate\s*\(\s*&self\b", A)
KIND_IMPL = re.compile(r"\bimpl\b[^\n{]*\b(?:CompositeMutationKind|MutationKind)\s*<", A)
HINT = re.compile(r"\bSeverity(?:::|\.)Hint\b", A)
SURFACES = ["🧰️framework/🔨️modules/📡️replication/🧾️wire/🦀️.rs", "🧰️framework/🔨️modules/🎠️kernel/🔄️merge-outcome/🟦️.ts", "🧰️framework/🔨️modules/🖱️ui/🧱️elements/📚️I18n/🟦️.tsx", "🧰️framework/🔨️modules/🖱️ui/🎯️targets/⚛️react/🌐️i18n/🤝️collaboration/🟦️.ts"]
SPELL = {"LaissezFaire": ["LaissezFaire", "laissezFaire"], "Normal": ["Normal", "normal"], "Vigilant": ["Vigilant", "vigilant"]}
GLUE = "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/📦️packages/🦀️rust/🦀️.rs"
OWNER = "🧰️framework/🛍️products/💻️os/🔨️modules/🗣️dsl/✨️derive/🦀️.rs"


def row(i, kind, line=0):
    return [i, "mutation-migration/" + kind, "high", line]


def inventory(t):
    out = []
    for p in sorted(t.files):
        if not (p.endswith((".rs", ".ts", ".tsx", ".py", ".feature")) or p.endswith("🎯️outcome/🔣️.json")):
            continue
        if any(s in SKIP_NAMES for s in p.split("/")) or p == "📜️script.ts":
            continue
        out.append(p)
    return jssort(out)


def line_of(c, i):
    return c[:i].count("\n") + 1


def crdt(t):
    out = []
    for p in [p for p in inventory(t) if p.endswith(".rs")]:
        c, seen = t.files[p], set()
        for m in CRDT.finditer(c):
            n = line_of(c, m.start())
            if n not in seen:
                seen.add(n)
                out.append(row("no-crdt-vocabulary-%s-%d" % (p, n), "no-crdt-vocabulary", n))
    return out


def validate(t):
    out = []
    for p in [p for p in inventory(t) if p.endswith(".rs")]:
        c, reported = t.files[p], set()

        def push(n):
            if n not in reported:
                reported.add(n)
                out.append(row("no-validate-override-%s-%d" % (p, n), "no-validate-override", n))

        if "/🧬️mutations/" in p:
            for m in VALIDATE.finditer(c):
                push(line_of(c, m.start()))
        for im in KIND_IMPL.finditer(c):
            body = extract_body(c, im.start())
            vm = VALIDATE.search(body)
            if vm:
                push(line_of(c, c.find("{", im.start()) + vm.start()))
    return out


def severity(t):
    out = []
    for p in [p for p in inventory(t) if p.endswith((".rs", ".ts", ".tsx"))]:
        c, reported = t.files[p], set()
        for m in HINT.finditer(c):
            n = line_of(c, m.start())
            if n not in reported:
                reported.add(n)
                out.append(row("severity-hint-%s-%d" % (p, n), "no-severity-hint", n))
        for i, l in enumerate(split_lines(c)):
            if re.search(r"""["']hint["']""", l, re.I) and re.search(r"severity|level", l, re.I) and (i + 1) not in reported:
                reported.add(i + 1)
                out.append(row("severity-hint-literal-%s-%d" % (p, i + 1), "no-severity-hint", i + 1))
    return out


def parity(t):
    out = []
    for s in SURFACES:
        c = t.files.get(s, "")
        missing = [v for v in SPELL if not any(x in c for x in SPELL[v])] if c else list(SPELL)
        if missing:
            out.append(row("merge-policy-parity-" + s, "merge-policy-parity"))
    return out


def derive(t):
    glue = t.files.get(GLUE, "")
    if t.files.get(OWNER) and re.search(r'#\[path\s*=\s*"\.\./\.\./🦀️\.rs"\]\s*mod\s+component\s*;', glue) and not re.search(r"\bfn\s+expand_", glue, A):
        return []
    return [row("derive-glue-mount-" + GLUE, "derive-glue-mount")]


def run(c):
    t = Tree(expand(c))
    return {"policyNoCrdtVocabularyBreaches": crdt, "policyNoValidateOverrideBreaches": validate, "policySeverityInfoBreaches": severity, "policyMergePolicyParityBreaches": parity, "policyDeriveGlueMountBreaches": derive}[c["rule"]](t)


cases = json.load(open(sys.argv[1], encoding="utf8"))["cases"]
output({c["id"]: run(c) for c in cases})

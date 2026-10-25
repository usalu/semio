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



TREE_ALLOW = {"puzzle#3d", "puzzle#5d"}
LABELS_ALLOW = {"cad#CadLabels", "puzzle#Puzzle2dLabels", "puzzle#Puzzle3dLabels", "puzzle#Puzzle5dLabels"}
K = "app-plugin/"


def row(i, kind, line, prio="medium"):
    return [i, K + kind, prio, line]


def line_of(content, idx):
    return content[:idx].count("\n") + 1


def region_format(scope, lines):
    out, events = [], region_events(lines)
    for ev in events:
        if ev["spaceAfterSlashes"]:
            out.append(row("region-format-space-%s-%d" % (scope, ev["line"]), "region-marker-format", ev["line"]))
        if not ev["label"]:
            out.append(row("region-format-missing-label-%s-%d" % (scope, ev["line"]), "region-marker-format", ev["line"]))
    for span in pair_spans(events):
        if re.match(r"^tests?$", label_name(span["label"]), re.I) and span["label"] != "🧪️Tests":
            out.append(row("region-tests-label-%s-%d" % (scope, span["startLine"]), "region-tests-label", span["startLine"]))
        if span["closeLabel"] and span["closeLabel"] != span["label"]:
            out.append(row("region-label-mismatch-%s-%d" % (scope, span["startLine"]), "region-marker-format", span["endLine"]))
    return out


def manifest_region(scope, lines):
    spans, tests, out = pair_spans(region_events(lines)), spans_of_mods(lines), []
    for i, line in enumerate(lines):
        if not re.search(r"App::builder\(", line):
            continue
        no = i + 1
        if in_spans(tests, no):
            continue
        if not any(s["label"] == "🔖️Manifest" and s["startLine"] <= no <= s["endLine"] for s in spans):
            out.append(row("manifest-region-%s-%d" % (scope, no), "manifest-region", no))
    return out


def struct_naming(scope, content):
    ids = {m.group(1): m.group(2) for m in re.finditer(r'const\s+([A-Z][A-Z0-9_]*_APP_ID)\s*:\s*&str\s*=\s*"([^"]+)"', content, A)}
    out = []
    for m in re.finditer(r"impl\s+DocumentApp\s+for\s+(\w+)\s*\{", content, A):
        window = content[m.end(): m.end() + 2000]
        cm = re.search(r"\b([A-Z][A-Z0-9_]*_APP_ID)\b", window, A)
        ident = ids.get(cm.group(1)) if cm else None
        if not ident:
            continue
        if m.group(1) != pascal(ident):
            out.append(row("struct-naming-%s-%s" % (scope, m.group(1)), "struct-naming", line_of(content, m.start())))
    return out


def mod_layout(scope, lines):
    impls = [i + 1 for i, l in enumerate(lines) if re.search(r"impl\s+DocumentApp\s+for\s+\w+\s*\{", l, A)]
    if len(impls) < 2:
        return []
    spans = mod_spans(lines)
    return [row("mod-layout-%s-%d" % (scope, n), "mod-layout", n, "high") for n in impls if not any(s["startLine"] <= n <= s["endLine"] for s in spans)]


def selection_ids(scope, content):
    am = re.search(r"\bselection_ids\s+as\s+(\w+)\b", content, A)
    alias = am.group(1) if am else None
    out = []
    for m in re.finditer(r"fn\s+selection_ids\s*\(", content, A):
        body = extract_body(content, m.start())
        if "semio_framework_plugin::selection_ids(" in body or (alias and re.search(r"\b%s\(" % alias, body, A)):
            continue
        n = line_of(content, m.start())
        out.append(row("sdk-selection-ids-%s-%d" % (scope, n), "sdk-selection-ids", n))
    return out


def delegate(scope, content):
    out = []
    specs = [(r"fn\s+new_app_with_registry\s*[<(]", r"artifact_app_laws::new_app_with_registry(?:::<[^>]*>)?\s*\("), (r"fn\s+new_app\s*[<(]", r"artifact_app_laws::new_app(?:::<[^>]*>)?\s*\("), (r"fn\s+meta\s*\(", r"artifact_app_laws::meta\s*\(")]
    for rx, must in specs:
        for m in re.finditer(rx, content, A):
            if re.search(must, extract_body(content, m.start()), A):
                continue
            n = line_of(content, m.start())
            out.append(row("sdk-artifact-app-laws-delegate-%s-%d" % (scope, n), "sdk-artifact-app-laws-delegate", n))
    return out


def tree_item(crate, content, lines):
    scope, out, spans = crate["dir"], [], mod_spans(lines)
    for m in re.finditer(r"fn\s+tree_item_with_action\s*\(", content, A):
        n = line_of(content, m.start())
        mod = mod_at(spans, n) or app_id(scope)
        if ("%s#%s" % (crate["pluginId"], mod)) in TREE_ALLOW:
            continue
        out.append(row("sdk-tree-item-redefinition-%s-%d" % (scope, n), "sdk-tree-item", n))
    for m in re.finditer(r"fn\s+(tree_item_\w+)\s*\(", content, A):
        if m.group(1) == "tree_item_with_action":
            continue
        end = content.find(")", m.start())
        sig = "" if end == -1 else content[m.start(): content.find("{", end)]
        if "ActionDescriptor" not in sig:
            continue
        if "tree_item_with_action(" in extract_body(content, m.start()):
            continue
        n = line_of(content, m.start())
        out.append(row("sdk-tree-item-wrapper-%s-%d" % (scope, n), "sdk-tree-item", n))
    return out


def labels_struct(crate, content):
    scope, out = crate["dir"], []
    for m in re.finditer(r"struct\s+(\w*Labels)\s*\{", content, A):
        n = line_of(content, m.start())
        before = content.split("\n")[max(0, n - 8): n - 1]
        if any("app_labels!" in l for l in before):
            continue
        allowed = ("%s#%s" % (crate["pluginId"], m.group(1))) in LABELS_ALLOW
        out.append(row("sdk-labels-struct-%s-%s" % (scope, m.group(1)), "sdk-labels-struct", n, "low" if allowed else "medium"))
    return out


def run(c):
    lines = c["source"]
    content = "\n".join(lines) + "\n"
    r = c["rule"]
    return {
        "policyRegionFormatBreaches": lambda: region_format(c["scope"], lines), "policyManifestRegionBreaches": lambda: manifest_region(c["scope"], lines),
        "policyStructNamingBreaches": lambda: struct_naming(c["scope"], content), "policyModLayoutBreaches": lambda: mod_layout(c["scope"], lines),
        "policySelectionIdsBreaches": lambda: selection_ids(c["scope"], content), "policyArtifactAppLawDelegateBreaches": lambda: delegate(c["scope"], content),
        "policyTreeItemBreaches": lambda: tree_item(c["crate"], content, lines), "policyLabelsStructBreaches": lambda: labels_struct(c["crate"], content),
    }[r]()


cases = json.load(open(sys.argv[1], encoding="utf8"))["cases"]
output({c["id"]: run(c) for c in cases})

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



import posixpath

_tax = os.path.dirname(os.path.abspath(__file__))
while not os.path.exists(os.path.join(_tax, "🔣️taxonomy.json")):
    _tax = os.path.dirname(_tax)
TAXONOMY = json.load(open(os.path.join(_tax, "🔣️taxonomy.json"), encoding="utf8"))
RS, TS = "🦀️.rs", "🟦️.ts"
PLUGINS = "✏️s/🔌️plugins"


def strip_non_ascii(s):
    return re.sub(r"[^\x00-\x7f]", "", s)


def utf16_index(text, idx):
    return len(text[:idx].encode("utf-16-le")) // 2


def subset_id(name):
    if name == TAXONOMY["subsetAnyDirName"]:
        return "*"
    prefix = TAXONOMY["subsetDirPrefix"]
    if not name.startswith(prefix):
        return None
    rest = name[len(prefix):]
    return rest if re.search(TAXONOMY["subsetSlugPattern"], rest) else None


def top_level_subsets(t):
    out = []
    for plugin, is_dir in t.children(PLUGINS).items():
        if not is_dir:
            continue
        arts = "%s/%s/%s" % (PLUGINS, plugin, TAXONOMY["artifactsDirName"])
        for art, a_dir in t.children(arts).items():
            if not a_dir:
                continue
            standards = "%s/%s/%s" % (arts, art, TAXONOMY["standardsDirName"])
            for std, s_dir in t.children(standards).items():
                if not s_dir or not std.startswith(TAXONOMY["standardDirPrefix"]):
                    continue
                subsets = "%s/%s/%s" % (standards, std, TAXONOMY["subsetsDirName"])
                for sub, u_dir in t.children(subsets).items():
                    if u_dir and subset_id(sub) is not None:
                        out.append("%s/%s" % (subsets, sub))
    return out

BUDGET = TAXONOMY["libWiringLineBudget"]
OPEN = re.compile(r"^\s*(?:pub(?:\([^)]*\))?\s+)?(?:async\s+)?(?:fn\s+\w+\s*(?:<[^>]*>)?\s*\([^;{]*\)[^;{]*|impl(?:<[^>]*>)?\s+[^;{]+)\{\s*$", A)
WIRING = re.compile(r'^\s*(//.*)?$|^\s*export\s+(type\s+)?(\*|\{[^}]*\})\s+from\s+"[^"]+";?\s*(//.*)?$|^\s*import\s+.+\s+from\s+"[^"]+";?\s*(//.*)?$')


def lib_shape(t, crates):
    out = []
    for c in crates:
        if c["libRelPath"] not in t.files:
            continue
        content = t.files[c["libRelPath"]]
        lines = split_lines(content)
        starts, offset = [], 0
        for line in lines:
            starts.append(offset)
            offset += len(line) + 1
        body_lines, covered = 0, 0
        for i, line in enumerate(lines):
            no = i + 1
            if no <= covered or not OPEN.search(line):
                continue
            body = extract_body(content, starts[i])
            lst = split_lines(body)
            body_lines += sum(1 for l in lst if l.strip() and not l.strip().startswith("//"))
            covered = no + len(lst) - 1
        if body_lines > BUDGET:
            out.append(["taxonomy-lib-shape-" + c["libRelPath"], "taxonomy/lib-shape", "medium" if c["pluginId"] else "low", 0])
    return out


def barrel_shape(t):
    out = []
    for p in sorted(t.files):
        if not p.endswith("/📦️packages/🟦️typescript/package.json"):
            continue
        meta = json.loads(t.files[p])
        if "role" not in meta.get("semio", {}):
            continue
        entry = p[: -len("package.json")] + "🟦️.ts"
        if entry not in t.files:
            continue
        non_wiring = [l for l in split_lines(t.files[entry]) if l.strip() and not WIRING.search(l)]
        if len(non_wiring) > BUDGET:
            out.append(["taxonomy-barrel-shape-" + p, "taxonomy/barrel-shape", "medium", 0])
    return out


def run(c):
    t = Tree(expand(c))
    return lib_shape(t, c["crates"]) if c["rule"] == "policyTaxonomyLibShapeBreaches" else barrel_shape(t)


cases = json.load(open(sys.argv[1], encoding="utf8"))["cases"]
output({c["id"]: run(c) for c in cases})

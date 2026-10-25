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

STEMS = {s.lower() for s in TAXONOMY["bannedNameStems"]}
SUBSET_EXEMPT = {s.lower() for s in TAXONOMY["bannedNameStemExemptions"]["subsetDirectories"]}
VENDORED = set(TAXONOMY["bannedNameStemExemptions"]["vendoredFiles"])
ROOTS = ["✏️s", "🧰️framework", "🌎️hub", "♻️mit-bestand"]
SDK = "🧰️framework/🛍️products/💻️os/🔨️modules/🔌️plugin/"


def row(i, kind, prio="high", line=0):
    return [i, kind, prio, line]


def banned_stems(t):
    out = []

    def walk(rel):
        subset_owner = rel.split("/")[-1] == TAXONOMY["subsetsDirName"]
        for name, is_dir in sorted(t.children(rel).items()):
            if name in SKIP_NAMES or name in ("node_modules", "target", "dist", "pkg"):
                continue
            child = rel + "/" + name
            stem = strip_non_ascii(re.sub(r"\.(rs|ts|tsx|js|mjs|cjs|go|py|cs)$", "", name)).lower()
            if stem in STEMS and not (subset_owner and is_dir and stem in SUBSET_EXEMPT) and child not in VENDORED:
                out.append(row("banned-name-stem-" + child, "taxonomy/banned-name-stem"))
            if is_dir:
                walk(child)

    for root in ROOTS:
        walk(root)
    return out


def plugin_root_shape(t):
    out = []
    for name, is_dir in sorted(t.children("✏️s/🔌️plugins").items()):
        if not is_dir or name in SKIP_NAMES:
            continue
        owner = "✏️s/🔌️plugins/" + name
        if t.exists(owner + "/🔌️plugin"):
            out.append(row("plugin-root-nested-" + owner, "taxonomy/plugin-root-shape"))
        if not t.exists(owner + "/🦀️.rs"):
            out.append(row("plugin-root-leaf-" + owner, "taxonomy/plugin-root-shape"))
        for child in TAXONOMY["pluginRequiredChildDirs"]:
            if not t.exists("%s/%s/🦀️.rs" % (owner, child)):
                out.append(row("plugin-root-child-%s/%s" % (owner, child), "taxonomy/plugin-root-shape"))
    return out


def plugin_builder(t, crates):
    out = []
    for c in crates:
        if c["role"] not in ("plugin", "extension") or c["libRelPath"] not in t.files or c["libRelPath"].startswith(SDK):
            continue
        content = t.files[c["libRelPath"]]
        if "semio_plugin!" in content:
            out.append(row("plugin-builder-macro-" + c["libRelPath"], "taxonomy/plugin-builder"))
        if "PluginBundle::new" in content or "PluginBundle {" in content:
            out.append(row("plugin-builder-bundle-" + c["libRelPath"], "taxonomy/plugin-builder"))
    return out


def spr_naming(t, crates):
    out = []
    for c in crates:
        if c["shape"] != "taxonomy" or c["role"] != "plugin":
            continue

        def walk(rel):
            for name, is_dir in sorted(t.children(rel).items()):
                if name in SKIP_NAMES:
                    continue
                child = rel + "/" + name
                if re.match(r"^protocol$", strip_non_ascii(re.sub(r"\.rs$", "", name)), re.I):
                    out.append(row("spr-naming-path-" + child, "taxonomy/spr-naming", "medium"))
                if is_dir:
                    walk(child)

        walk(c["ownerRel"])
        if c["libRelPath"] not in t.files:
            continue
        content = t.files[c["libRelPath"]]
        for m in re.finditer(r"\b(?:mod|struct|enum|type)\s+(\w*[Pp]rotocol\w*)\b", content, A):
            out.append(row("spr-naming-ident-%s-%d" % (c["libRelPath"], utf16_index(content, m.start())), "taxonomy/spr-naming", "medium", content[:m.start()].count("\n") + 1))
    return out


def run(c):
    t = Tree(expand(c))
    r = c["rule"]
    return {"policyBannedNameStemBreaches": lambda: banned_stems(t), "policyPluginRootShapeBreaches": lambda: plugin_root_shape(t), "policyPluginBuilderBreaches": lambda: plugin_builder(t, c.get("crates", [])), "policySprNamingBreaches": lambda: spr_naming(t, c.get("crates", []))}[r]()


cases = json.load(open(sys.argv[1], encoding="utf8"))["cases"]
output({c["id"]: run(c) for c in cases if c.get("oracle") is not False})

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

SLUG = re.compile(TAXONOMY["exampleSlugPattern"].replace("\\uFE0F", chr(0xFE0F)))
FORBIDDEN_SLUGS = TAXONOMY["forbiddenExampleSlugs"]
PLURALS = TAXONOMY["forbiddenExamplePluralDirs"]
RS_EXT, TS_EXT = TAXONOMY["fileKinds"]["rust-source"]["extensionChains"], TAXONOMY["fileKinds"]["typescript-source"]["extensionChains"]
LEAF_PARENTS = set(TAXONOMY["taxonomyLeafParentDirs"])
FACET_DIRS = set(TAXONOMY["artifactComponentDirs"])
K = "taxonomy/semio-examples"


def row(i, kind, prio, line=0):
    return [i, kind, prio, line]


def dirs(t, d):
    return [n for n, is_dir in sorted(t.children(d).items()) if is_dir and n not in SKIP_NAMES]


def scope_id(c):
    return c["pluginId"] or strip_non_ascii(c["ownerRel"].split("/")[-1])


def new_surface(c, p):
    return p if c["pluginId"] else "low"


def surface_roots(t, owner):
    roots, arts = [], owner + "/🗿️artifacts"
    for a in dirs(t, arts):
        for s in dirs(t, "%s/%s/🏅️standards" % (arts, a)):
            sub_root = "%s/%s/🏅️standards/%s/🪆️subsets" % (arts, a, s)
            for u in dirs(t, sub_root):
                for role in TAXONOMY["surfaceRoles"]:
                    surface = "%s/%s/%s" % (sub_root, u, TAXONOMY["surfaceDirNames"][role])
                    if t.exists(surface):
                        roots.append(surface)
    return roots


def validate_unit(t, rel, prio):
    out, slug = [], rel.split("/")[-1]
    if slug in FORBIDDEN_SLUGS or not SLUG.search(slug):
        out.append(row("semio-examples-slug-" + rel, K, prio))
    for plural in PLURALS:
        if t.exists(rel + "/" + plural):
            out.append(row("semio-examples-plural-%s-%s" % (rel, plural), K, prio))
    if not t.exists(rel + "/🦀️.rs"):
        out.append(row("semio-examples-leaf-rs-" + rel, K, prio))
    if not t.exists(rel + "/🟦️.ts"):
        out.append(row("semio-examples-leaf-ts-" + rel, K, prio))
    assets = rel + "/🖼️assets"
    if not t.exists(assets):
        out.append(row("semio-examples-assets-" + rel, K, prio))
    else:
        for name, is_dir in sorted(t.children(assets).items()):
            if not is_dir and name not in SKIP_NAMES and len(t.files[assets + "/" + name].encode("utf-8")) == 0:
                out.append(row("semio-examples-zero-byte-asset-%s-%s" % (rel, name), K, prio))
    tests = rel + "/🧪️tests"
    if not t.exists(tests):
        out.append(row("semio-examples-tests-" + rel, K, prio))
    else:
        if not t.exists(tests + "/🦀️.rs"):
            out.append(row("semio-examples-test-rs-" + rel, K, prio))
        if not t.exists(tests + "/🟦️.ts"):
            out.append(row("semio-examples-test-ts-" + rel, K, prio))
    return out


def examples(t, crates):
    out = []
    for c in crates:
        owner, prio = c["ownerRel"], new_surface(c, "medium")
        if t.exists(owner + "/📚️examples"):
            out.append(row("semio-examples-plugin-root-" + owner, K, new_surface(c, "high")))
        for art in dirs(t, owner + "/🗿️artifacts"):
            ex = "%s/🗿️artifacts/%s/📚️examples" % (owner, art)
            if not t.exists(ex):
                out.append(row("semio-examples-missing-" + ex, K, prio))
                continue
            sets = dirs(t, ex)
            if not sets:
                out.append(row("semio-examples-empty-" + ex, K, prio))
            for s in sets:
                out += validate_unit(t, ex + "/" + s, prio)
        for root in surface_roots(t, owner):
            ex = root + "/📚️examples"
            if not t.exists(ex):
                continue
            sets = dirs(t, ex)
            if not sets:
                out.append(row("semio-examples-surface-empty-" + ex, K, prio))
            for s in sets:
                out += validate_unit(t, ex + "/" + s, prio)
    return out


def component_files(t, crates):
    out = []
    for c in crates:
        def walk(rel):
            files = [n for n, d in t.children(rel).items() if not d and n not in SKIP_NAMES]
            parent = rel.split("/")[-1]
            has_rs = any(any(f.endswith(e) for e in RS_EXT) for f in files)
            has_ts = any(any(f.endswith(e) for e in TS_EXT) for f in files)
            parts = rel.split("/")
            prio = new_surface(c, "medium")
            if len(parts) >= 2 and parts[-2] == "📚️examples":
                if has_rs and "🦀️.rs" not in files:
                    out.append(row("component-file-example-" + rel, "taxonomy/component-file", prio))
                if has_ts and "🟦️.ts" not in files:
                    out.append(row("component-file-example-ts-" + rel, "taxonomy/component-file", prio))
            elif parent in LEAF_PARENTS:
                if has_rs and "🦀️.rs" not in files:
                    out.append(row("component-file-" + rel, "taxonomy/component-file", prio))
                if parent in FACET_DIRS and has_ts and "🟦️.ts" not in files:
                    out.append(row("component-file-ts-" + rel, "taxonomy/component-file", prio))
            for d in dirs(t, rel):
                walk(rel + "/" + d)

        walk(c["ownerRel"] + "/🗿️artifacts")
    return out


def run(c):
    t = Tree(expand(c))
    return {"policySemioArtifactExamplesBreaches": examples, "policyComponentFileBreaches": component_files}[c["rule"]](t, c["crates"])


cases = json.load(open(sys.argv[1], encoding="utf8"))["cases"]
output({c["id"]: run(c) for c in cases})

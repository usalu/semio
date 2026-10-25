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

def kind_file(kind):
    k = TAXONOMY["fileKinds"][kind]
    return k["emoji"] + k["extensionChains"][0]


MARKER = kind_file(TAXONOMY["windowEmptyFacetFileKindId"])
COMPONENTS = {k: kind_file(v) for k, v in TAXONOMY["componentFileKinds"].items()}
COMPONENT_FILES = set(COMPONENTS.values())
SCHEMA_DIRS, REPS, IO_DIRS, IO_CHILD = TAXONOMY["schemaChildDirs"], TAXONOMY["representationDirs"], TAXONOMY["ioDirectionDirs"], TAXONOMY["ioDirectionChildDirs"]
ARTIFACT_CHILDREN = TAXONOMY["artifactChildDirs"] + TAXONOMY["newArtifactChildDirs"]


def dirs(t, d):
    return [n for n, is_dir in sorted(t.children(d).items()) if is_dir and n not in SKIP_NAMES]


def scope_id(crate):
    return crate["pluginId"] or strip_non_ascii(crate["ownerRel"].split("/")[-1])


def row(i, kind, prio, line=0):
    return [i, kind, prio, line]


def surface_roots(t, owner):
    roots = []
    arts = owner + "/" + TAXONOMY["artifactsDirName"]
    for a in dirs(t, arts):
        std_root = "%s/%s/%s" % (arts, a, TAXONOMY["standardsDirName"])
        for s in dirs(t, std_root):
            sub_root = "%s/%s/%s" % (std_root, s, TAXONOMY["subsetsDirName"])
            for u in dirs(t, sub_root):
                for role in TAXONOMY["surfaceRoles"]:
                    surface = "%s/%s/%s" % (sub_root, u, TAXONOMY["surfaceDirNames"][role])
                    if t.exists(surface):
                        roots.append(surface)
    return roots


def taxonomy_dirs(t, crates):
    out = []
    for crate in crates:
        owner, prio = crate["ownerRel"], "medium" if crate["pluginId"] else "low"
        sid = scope_id(crate)
        add = lambda i: out.append(row("taxonomy-dirs-" + i, "taxonomy/dirs", prio))

        def schema_facet(nested_root):
            for nested in dirs(t, nested_root):
                if nested not in SCHEMA_DIRS:
                    add("artifact-%s-%s" % (nested_root, nested))

        def io_facet(nested_root):
            for nested in dirs(t, nested_root):
                if nested in REPS:
                    continue
                if nested not in IO_DIRS:
                    add("artifact-%s-%s" % (nested_root, nested))
                    continue
                expected = IO_CHILD.get(nested)
                direction = nested_root + "/" + nested
                for codec in dirs(t, direction):
                    if expected and codec == expected:
                        codec_root = direction + "/" + codec
                        for mid in dirs(t, codec_root):
                            if mid != TAXONOMY["artifactsDirName"]:
                                add("artifact-%s-%s" % (codec_root, mid))
                        continue
                    add("artifact-%s-%s" % (direction, codec))

        arts = owner + "/" + TAXONOMY["artifactsDirName"]
        for art in dirs(t, arts):
            art_dir = arts + "/" + art
            for child in dirs(t, art_dir):
                if child not in ARTIFACT_CHILDREN:
                    add("artifact-%s-%s" % (art_dir, child))
                    continue
                if child == "🧬️schema":
                    schema_facet(art_dir + "/" + child)
                elif child == "🚪️io":
                    io_facet(art_dir + "/" + child)
                elif child == TAXONOMY["standardsDirName"]:
                    std_root = art_dir + "/" + child
                    for std in dirs(t, std_root):
                        std_dir = std_root + "/" + std
                        for sc in dirs(t, std_dir):
                            if sc not in TAXONOMY["standardChildDirs"]:
                                add("standard-%s-%s" % (std_dir, sc))
                                continue
                            if sc != TAXONOMY["subsetsDirName"]:
                                continue
                            sub_root = std_dir + "/" + sc
                            for sub in dirs(t, sub_root):
                                sub_dir = sub_root + "/" + sub
                                for subc in dirs(t, sub_dir):
                                    if subc not in TAXONOMY["subsetChildDirs"]:
                                        add("subset-%s-%s" % (sub_dir, subc))
                                        continue
                                    if subc == "🧬️schema":
                                        schema_facet(sub_dir + "/" + subc)
                                    elif subc == "🚪️io":
                                        io_facet(sub_dir + "/" + subc)
                                    elif subc in TAXONOMY["surfaceDirNames"].values():
                                        surface = sub_dir + "/" + subc
                                        for sf in dirs(t, surface):
                                            if sf not in TAXONOMY["surfaceChildDirs"]:
                                                add("surface-%s-%s" % (surface, sf))

        def walk_windows(rel):
            for entry in dirs(t, rel):
                child = rel + "/" + entry
                if entry == TAXONOMY["windowsDirName"]:
                    for w in dirs(t, child):
                        wdir = child + "/" + w
                        for c in dirs(t, wdir):
                            if c in TAXONOMY["windowChildDirs"]:
                                continue
                            if TAXONOMY["windowsDirName"] in TAXONOMY["testOwnerKinds"] and c in (TAXONOMY["testsDirName"], TAXONOMY["testFixturesDirName"]):
                                continue
                            add("window-%s-%s" % (wdir, c))
                    continue
                walk_windows(child)

        for root in surface_roots(t, owner):
            walk_windows(root)
    return out


def window_completeness(t, crates):
    out = []
    for crate in crates:
        def walk(rel):
            for entry in dirs(t, rel):
                child = rel + "/" + entry
                if entry != TAXONOMY["windowsDirName"]:
                    walk(child)
                    continue
                for w in dirs(t, child):
                    wdir = child + "/" + w
                    actual = set(dirs(t, wdir))
                    for req in TAXONOMY["windowRequiredChildDirs"]:
                        cap = wdir + "/" + req
                        if req not in actual:
                            out.append(row("taxonomy-window-completeness-%s-%s" % (wdir, req), "taxonomy/window-completeness", "high"))
                            continue
                        members = dirs(t, cap)
                        marker = cap + "/" + MARKER
                        for f in sorted(COMPONENT_FILES, key=lambda s: s.encode("utf-16-be")):
                            if t.exists(cap + "/" + f):
                                out.append(row("taxonomy-window-facet-component-%s-%s" % (cap, f), "taxonomy/window-facet-component", "high"))
                        if not members:
                            if not t.exists(marker):
                                out.append(row("taxonomy-window-empty-facet-" + cap, "taxonomy/window-empty-facet", "high"))
                            continue
                        if t.exists(marker):
                            out.append(row("taxonomy-window-populated-facet-marker-" + cap, "taxonomy/window-empty-facet", "high"))
                        for m in members:
                            for lang in TAXONOMY["windowComponentLangs"]:
                                if not t.exists("%s/%s/%s" % (cap, m, COMPONENTS[lang])):
                                    out.append(row("taxonomy-window-component-%s/%s-%s" % (cap, m, lang), "taxonomy/window-component", "high"))
        for root in surface_roots(t, crate["ownerRel"]):
            walk(root)
    return out


def mode_completeness(t, crates):
    out, required = [], TAXONOMY["modeRequiredChildDirs"]
    for crate in crates:
        def walk(rel):
            for entry in dirs(t, rel):
                child = rel + "/" + entry
                if entry != TAXONOMY["modesDirName"]:
                    walk(child)
                    continue
                for mode in dirs(t, child):
                    mdir = child + "/" + mode
                    actual = set(dirs(t, mdir))
                    for c in required:
                        cdir, marker = mdir + "/" + c, mdir + "/" + c + "/" + MARKER
                        if c not in actual:
                            out.append(row("taxonomy-mode-completeness-%s-%s" % (mdir, c), "taxonomy/mode-completeness", "high"))
                            continue
                        members = dirs(t, cdir)
                        if not members:
                            if not t.exists(marker):
                                out.append(row("taxonomy-mode-empty-child-" + cdir, "taxonomy/mode-empty-child", "high"))
                            continue
                        if t.exists(marker):
                            out.append(row("taxonomy-mode-populated-child-marker-" + cdir, "taxonomy/mode-empty-child", "high"))
        for root in surface_roots(t, crate["ownerRel"]):
            walk(root)
    return out


def run(c):
    t = Tree(expand(c))
    return {"policyTaxonomyDirsBreaches": taxonomy_dirs, "policyWindowCompletenessBreaches": window_completeness, "policyModeCompletenessBreaches": mode_completeness}[c["rule"]](t, c["crates"])


cases = json.load(open(sys.argv[1], encoding="utf8"))["cases"]
output({c["id"]: run(c) for c in cases})

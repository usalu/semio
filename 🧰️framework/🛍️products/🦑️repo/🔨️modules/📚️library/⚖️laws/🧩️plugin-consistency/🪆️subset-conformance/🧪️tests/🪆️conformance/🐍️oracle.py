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

ENGINE = "⚙️engine"
SURFACE_ROLES = TAXONOMY["surfaceRoles"]
SURFACE_DIRS = TAXONOMY["surfaceDirNames"]
LEAVES = {"🦀️rust": RS, "🟦️typescript": TS}
OS_CONFIG = "🧰️framework/🛍️products/💻️os/🎚️config"


def row(i, kind, prio, line=0):
    return [i, kind, prio, line]


def facet_totality(t, subs):
    return [row("subset-facet-missing-%s/%s" % (s, c), "subset-conformance/facet-totality", "medium") for s in subs for c in TAXONOMY["subsetChildDirs"] if not t.exists(s + "/" + c)]


def artifact_dirs(t):
    out = []
    for plugin, pd in t.children(PLUGINS).items():
        if pd:
            for art, ad in t.children("%s/%s/🗿️artifacts" % (PLUGINS, plugin)).items():
                if ad:
                    out.append("%s/%s/🗿️artifacts/%s" % (PLUGINS, plugin, art))
    return out


def engine_forbidden(t):
    out = []
    for art in artifact_dirs(t):
        owners = [art]
        standards = art + "/🏅️standards"
        for std, sd in t.children(standards).items():
            if not sd:
                continue
            owners.append(standards + "/" + std)
            for sub, ud in t.children("%s/%s/🪆️subsets" % (standards, std)).items():
                if ud:
                    owners.append("%s/%s/🪆️subsets/%s" % (standards, std, sub))
        out += [row("artifact-engine-forbidden-" + o, "subset-conformance/engine-forbidden", "low") for o in owners if t.exists(o + "/" + ENGINE)]
    return out


def examples_not_at_artifact(t):
    out = []
    for art in artifact_dirs(t):
        if t.exists(art + "/📚️examples"):
            out.append(row("artifact-examples-remaining-%s/📚️examples" % art, "subset-conformance/example-placement", "medium"))
        for std, sd in t.children(art + "/🏅️standards").items():
            if sd and t.exists("%s/🏅️standards/%s/📚️examples" % (art, std)):
                out.append(row("standard-examples-remaining-%s/🏅️standards/%s/📚️examples" % (art, std), "subset-conformance/example-placement", "medium"))
    return out


def phantom(t):
    out = []

    def walk(d):
        for name, is_dir in t.children(d).items():
            if not is_dir:
                continue
            child = d + "/" + name
            if "标准" in name:
                out.append(row("phantom-standards-" + child, "subset-conformance/phantom-standards", "medium"))
            walk(child)

    walk(PLUGINS)
    return out


def ts_parity(t, subs):
    out = []
    for s in subs:
        p = s + "/🧬️schema/" + TS
        if p in t.files:
            n = len(split_lines(t.files[p]))
            if 0 < n <= 7:
                out.append(row("subset-ts-meta-stub-" + s, "subset-conformance/ts-parity", "medium"))
    return out


def glue_shim(t):
    out = []
    for plugin, pd in t.children(PLUGINS).items():
        glue = "%s/%s/📦️packages/🦀️rust/🦀️.rs" % (PLUGINS, plugin)
        if glue in t.files and re.search(r"Shims:\s*keep pre-migration|pre-migration module paths", t.files[glue], re.I):
            out.append(row("glue-shim-" + glue, "subset-conformance/no-glue-shim-blocks", "medium"))
    return out


def surface_completeness(t, subs):
    out = []
    for s in subs:
        if not t.exists(s + "/🧬️schema"):
            continue
        for role in SURFACE_ROLES:
            surface = "%s/%s" % (s, SURFACE_DIRS[role])
            if not t.exists(surface):
                out.append(row("subset-surface-missing-" + surface, "taxonomy/surface-completeness", "medium"))
                continue
            modes = surface + "/" + TAXONOMY["modesDirName"]
            complete = False
            for mode, md in t.children(modes).items():
                if not md:
                    continue
                windows = "%s/%s/%s" % (modes, mode, TAXONOMY["windowsDirName"])
                for w, wd in t.children(windows).items():
                    if wd and all((windows + "/" + w + "/" + LEAVES[lang]) in t.files for lang in TAXONOMY["windowLeafLangs"]):
                        complete = True
            if not complete:
                out.append(row("subset-surface-window-incomplete-" + surface, "taxonomy/surface-completeness", "medium"))
            if any("SCAFFOLD" in t.files[p] for p in t.walk([surface], lambda p, n: n in (RS, TS))):
                out.append(row("subset-surface-scaffold-residue-" + surface, "taxonomy/surface-scaffold-residue", "medium"))
    return out


def viewer_purity(t, subs):
    out = []
    pats = [(r"\.mutation\(", ".mutation("), (r"Emit::mutations", "Emit::mutations"), (r"artifact_mutations", "artifact_mutations"), (r"::editor::", "::editor::")]
    for s in subs:
        viewer = "%s/%s" % (s, TAXONOMY["viewerDirName"])
        if not t.exists(viewer):
            continue
        for p in t.walk([viewer], lambda p, n: True):
            for rx, label in pats:
                if re.search(rx, t.files[p]):
                    out.append(row("viewer-purity-%s-%s" % (p, label), "taxonomy/viewer-purity", "high"))
    return out


def own_components(t, owner):
    return t.walk([owner], lambda p, n: n == RS, skip=lambda rel, name: name == "🧩️extensions")


def contributed_target(t, subs):
    out, owners = [], {}
    root_of = lambda s: "/".join(s.split("/")[:3])
    for s in subs:
        if t.exists(s + "/🧬️schema"):
            owners.setdefault(s[len(root_of(s)) + 1:], root_of(s))
    for s in subs:
        if t.exists(s + "/🧬️schema"):
            continue
        plugin = root_of(s)
        owner = owners.get(s[len(plugin) + 1:])
        for role in SURFACE_ROLES:
            surface = "%s/%s" % (s, SURFACE_DIRS[role])
            if not t.exists(surface):
                continue
            if not owner:
                out.append(row("contributed-surface-unknown-owner-" + surface, "plugin-dependency/contributed-surface-target", "medium"))
                continue
            if owner == plugin:
                continue
            owner_id = strip_non_ascii(owner.split("/")[-1])
            if any(re.search(r'\.depends_on\s*\(\s*"%s"' % re.escape(owner_id), t.files[p]) for p in own_components(t, plugin)):
                continue
            out.append(row("contributed-surface-target-undeclared-" + surface, "plugin-dependency/contributed-surface-target", "high"))
    return out


def os_config(t):
    if not t.exists(OS_CONFIG + "/🧬️schema"):
        return [row("os-config-shape-missing-schema", "taxonomy/os-config-shape", "high")]
    return None


def run(c):
    t = Tree(expand(c))
    subs = top_level_subsets(t)
    r = c["rule"]
    if r == "policyListTopLevelSubsetDirs":
        return subs
    if r == "policySubsetConformanceBreaches":
        return facet_totality(t, subs) + engine_forbidden(t) + examples_not_at_artifact(t) + phantom(t) + ts_parity(t, subs) + glue_shim(t)
    return {
        "policySubsetFacetTotalityBreaches": lambda: facet_totality(t, subs), "policyArtifactEngineFacetForbiddenBreaches": lambda: engine_forbidden(t),
        "policyExampleNotAtArtifactLevelBreaches": lambda: examples_not_at_artifact(t), "policyPhantomStandardsBreaches": lambda: phantom(t),
        "policySubsetTsParityBreaches": lambda: ts_parity(t, subs), "policyNoGlueShimBlocksBreaches": lambda: glue_shim(t),
        "policySubsetSurfaceCompletenessBreaches": lambda: surface_completeness(t, subs), "policyViewerPurityBreaches": lambda: viewer_purity(t, subs),
        "policyContributedSurfaceTargetBreaches": lambda: contributed_target(t, subs), "policyOsConfigShapeBreaches": lambda: os_config(t),
    }[r]()


cases = json.load(open(sys.argv[1], encoding="utf8"))["cases"]
output({c["id"]: run(c) for c in cases})

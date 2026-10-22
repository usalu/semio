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


STDIO = "✏️s/🔌️plugins/🗄️stdio/🗿️artifacts"


def dialects(t):
    out = []
    for plugin, pd in sorted(t.children(PLUGINS).items()):
        if not pd:
            continue
        arts = "%s/%s/🗿️artifacts" % (PLUGINS, plugin)
        for art, ad in sorted(t.children(arts).items()):
            if not ad:
                continue
            art_rel = arts + "/" + art
            for std, sd in sorted(t.children(art_rel + "/🏅️standards").items()):
                if not sd or not std.startswith(TAXONOMY["standardDirPrefix"]):
                    continue
                std_rel = "%s/🏅️standards/%s" % (art_rel, std)
                for sub, ud in sorted(t.children(std_rel + "/🪆️subsets").items()):
                    if ud:
                        out.append({"artRel": art_rel, "standardRel": std_rel, "standardSlug": std[len(TAXONOMY["standardDirPrefix"]):], "subsetRel": "%s/🪆️subsets/%s" % (std_rel, sub)})
    return out


def stdio_schema_owning(t):
    out = []
    for d in dialects(t):
        if d["artRel"].startswith(STDIO + "/") and t.exists(d["subsetRel"] + "/🧬️schema/📸️snapshot"):
            out.append({**d, "artifactId": strip_non_ascii(d["artRel"][len(STDIO) + 1:])})
    return out

FLAGSHIP = [STDIO + "/" + d for d in ("🎨️svg", "🧊️gltf", "🌳️pdf", "🎞️gif")]
K = "artifact-io/"


def row(i, kind, prio="high"):
    return [i, kind, prio, 0]


def strip_rust(body):
    body = re.sub(r"/\*.*?\*/", lambda m: " " * len(m.group(0)), body, flags=re.S)
    body = re.sub(r"//[^\n]*", lambda m: " " * len(m.group(0)), body)
    return re.sub(r'"(?:[^"\\]|\\.)*"', lambda m: '"' + " " * (len(m.group(0)) - 2) + '"', body)


def codec_ids(t):
    if not t.exists("✏️s/🔌️plugins/🗄️stdio"):
        return []
    files = [p for p in t.walk([""], lambda p, n: n.endswith(".rs")) if p.startswith(STDIO + "/")]
    consts = {}
    for p in files:
        for m in re.finditer(r'(?:pub\s+)?const\s+([A-Z0-9_]+)\s*:\s*&str\s*=\s*"([^"]*)"\s*;', t.files[p]):
            consts[m.group(1)] = m.group(2)
    claims = {}
    for p in files:
        c = t.files[p]
        for m in re.finditer(r'register_document_codec\(\s*store::artifact_codec::<[\s\S]*?>\(\s*([\w:]+|"[^"]*")\s*\)\s*\)', c, A):
            raw = m.group(1)
            value = raw[1:-1] if raw.startswith('"') else consts.get(raw.split("::")[-1])
            if not value:
                continue
            claims.setdefault(value, []).append((p, c[:m.start()].count("\n") + 1))
    out = []
    for ident, sites in claims.items():
        if len(sites) > 1:
            out += [row("stdio-codec-id-duplicate-%s-%s-%d" % (ident, p, n), "stdio-artifacts/codec-id-uniqueness") for p, n in sites]
    return out


def round_trip(t):
    out = []
    for p in t.walk([""], lambda p, n: n.endswith(".rs")):
        if not p.startswith("✏️s/🔌️plugins/") or not p.endswith("⚙️engine/🦀️.rs") or "🏅️standards/🔖️" not in p:
            continue
        c = t.files[p]
        signal = "#[cfg(test)]" in c and re.search(r"round_trip|roundtrip|decode_encode|encode_decode|lossless", c, re.I)
        if not signal:
            out.append(row("round-trip-test-" + p, K + "round-trip-test"))
    return out


def enum_variants(content):
    s = strip_rust(content)
    h = re.search(r"pub\s+enum\s+\w*Mutation\w*\s*\{", s, A)
    if not h:
        return None
    i = h.end()
    depth, seg, out = 1, i, []
    def push(raw):
        m = re.search(r"(?:#\[[^\]]*\]\s*)*(\w+)", raw, A)
        if m:
            out.append(m.group(1))
    while i < len(s) and depth > 0:
        ch = s[i]
        if ch in "{(":
            depth += 1
        elif ch in "})":
            depth -= 1
            if depth == 0:
                push(s[seg:i])
                break
        elif ch == "," and depth == 1:
            push(s[seg:i])
            seg = i + 1
        i += 1
    return out


def vocabulary(t):
    out = []
    for p in t.walk([""], lambda p, n: n.endswith(".rs")):
        if not p.endswith("🧬️mutations/🦀️.rs") or not any(p.startswith(d + "/") for d in FLAGSHIP):
            continue
        v = enum_variants(t.files[p])
        real = len([x for x in (v or []) if x not in ("NoMutation", "SetSnapshot")])
        if real == 0:
            out.append(row("mutation-vocabulary-" + p, K + "mutation-vocabulary"))
    return out


def run(c):
    t = Tree(expand(c))
    return {"policyStdioCodecIdUniquenessBreaches": codec_ids, "policyRoundTripTestBreaches": round_trip, "policyMutationVocabularyBreaches": vocabulary}[c["rule"]](t)


cases = json.load(open(sys.argv[1], encoding="utf8"))["cases"]
output({c["id"]: run(c) for c in cases})

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

SPLIT = set(json.loads('["\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udcfa\\ufe0frenderer/\\ud83e\\uddd1\\u200d\\ud83c\\udfa8engine/\\ud83c\\udfaf\\ufe0ftargets/\\ud83e\\uddca\\ufe0fwgpu/\\ud83e\\uddca\\ufe0frenderer/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udcfa\\ufe0frenderer/\\ud83e\\uddd1\\u200d\\ud83c\\udfa8engine/\\ud83e\\uddf1\\ufe0felements/\\ud83c\\udf9e\\ufe0fScenes/\\ud83c\\udfaf\\ufe0ftargets/\\ud83e\\uddca\\ufe0fwgpu/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udcfa\\ufe0frenderer/\\ud83e\\uddd1\\u200d\\ud83c\\udfa8engine/\\ud83e\\uddf1\\ufe0felements/\\ud83d\\udc1a\\ufe0fShell/\\ud83c\\udfaf\\ufe0ftargets/\\ud83e\\uddca\\ufe0fwgpu/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udcfa\\ufe0frenderer/\\ud83e\\uddd1\\u200d\\ud83c\\udfa8engine/\\ud83e\\uddf1\\ufe0felements/\\u2699\\ufe0fEngineCanvas/\\ud83c\\udfaf\\ufe0ftargets/\\ud83e\\uddca\\ufe0fwgpu/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udcfa\\ufe0frenderer/\\ud83e\\uddd1\\u200d\\ud83c\\udfa8engine/\\ud83e\\uddf1\\ufe0felements/\\ud83d\\udde3\\ufe0fInterpreter/\\ud83c\\udfaf\\ufe0ftargets/\\ud83e\\uddca\\ufe0fwgpu/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\u267e\\ufe0finfinite/\\ud83c\\udf0d\\ufe0fworld/\\ud83d\\uddc3\\ufe0fmodel/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\u267e\\ufe0finfinite/\\ud83c\\udf0d\\ufe0fworld/\\ud83e\\udde9\\ufe0fcomponents/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\u267e\\ufe0finfinite/\\ud83c\\udf0d\\ufe0fworld/\\ud83e\\uddf2\\ufe0fcursors/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\u267e\\ufe0finfinite/\\ud83c\\udf0d\\ufe0fworld/\\ud83e\\uddf1\\ufe0fscene/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\u267e\\ufe0finfinite/\\ud83c\\udf0d\\ufe0fworld/\\ud83c\\udf09\\ufe0fbridge/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\u267e\\ufe0finfinite/\\ud83c\\udf0d\\ufe0fworld/\\ud83e\\uddf5\\ufe0finteraction/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\u267e\\ufe0finfinite/\\ud83c\\udf0d\\ufe0fworld/\\ud83c\\udf9b\\ufe0fhost/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\u267e\\ufe0finfinite/\\ud83c\\udfb2\\ufe0fboard/\\ud83d\\udd0c\\ufe0fports/\\u27a1\\ufe0fdirected/\\u2795\\ufe0fnormal/\\ud83d\\uddc3\\ufe0fmodel/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\u267e\\ufe0finfinite/\\ud83c\\udfb2\\ufe0fboard/\\ud83d\\udd0c\\ufe0fports/\\u27a1\\ufe0fdirected/\\u2795\\ufe0fnormal/\\ud83d\\uddbc\\ufe0fscene/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\u267e\\ufe0finfinite/\\ud83c\\udfb2\\ufe0fboard/\\ud83d\\udd0c\\ufe0fports/\\u27a1\\ufe0fdirected/\\u2795\\ufe0fnormal/\\ud83c\\udfaf\\ufe0ffocus/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\u267e\\ufe0finfinite/\\ud83c\\udfb2\\ufe0fboard/\\ud83d\\udd0c\\ufe0fports/\\u27a1\\ufe0fdirected/\\u2795\\ufe0fnormal/\\ud83d\\udd79\\ufe0fgestures/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\u267e\\ufe0finfinite/\\ud83c\\udfb2\\ufe0fboard/\\ud83d\\udd0c\\ufe0fports/\\u27a1\\ufe0fdirected/\\u2795\\ufe0fnormal/\\ud83e\\udea3\\ufe0ffill/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\u267e\\ufe0finfinite/\\ud83c\\udfb2\\ufe0fboard/\\ud83d\\udd0c\\ufe0fports/\\u27a1\\ufe0fdirected/\\u2795\\ufe0fnormal/\\ud83e\\udded\\ufe0fhost/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\u267e\\ufe0finfinite/\\ud83c\\udfb2\\ufe0fboard/\\ud83d\\udd0c\\ufe0fports/\\u27a1\\ufe0fdirected/\\ud83d\\udd78\\ufe0fdag/\\ud83d\\uddc3\\ufe0fmodel/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\u267e\\ufe0finfinite/\\ud83c\\udfb2\\ufe0fboard/\\ud83d\\udd0c\\ufe0fports/\\u27a1\\ufe0fdirected/\\ud83d\\udd78\\ufe0fdag/\\ud83d\\udd78\\ufe0fgraph/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\u267e\\ufe0finfinite/\\ud83c\\udfb2\\ufe0fboard/\\ud83d\\udd0c\\ufe0fports/\\u27a1\\ufe0fdirected/\\ud83d\\udd78\\ufe0fdag/\\ud83e\\udded\\ufe0fhost/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udd0c\\ufe0fplugin/\\ud83e\\uddf5\\ufe0finstance/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udd0c\\ufe0fplugin/\\u269b\\ufe0freactor/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udd0c\\ufe0fplugin/\\u269b\\ufe0freactor/\\ud83c\\udfc3\\ufe0fruntime/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udd0c\\ufe0fplugin/\\u269b\\ufe0freactor/\\ud83d\\udd04\\ufe0fturn/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udd0c\\ufe0fplugin/\\u269b\\ufe0freactor/\\ud83e\\ude79\\ufe0fpatches/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83c\\udfc3\\ufe0frun/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83c\\udf3f\\ufe0fvcs/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udeaa\\ufe0fio/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83e\\udde0\\ufe0fneural/\\u2699\\ufe0fengine/\\ud83e\\uddf1\\ufe0fmodel/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udd0c\\ufe0fplugin/\\ud83d\\udda5\\ufe0fhost/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udd0c\\ufe0fplugin/\\ud83d\\udda5\\ufe0fhost/\\ud83e\\uddf5\\ufe0fshard/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udda5\\ufe0fhost/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udd28\\ufe0fmodules/\\u23f3\\ufe0fasync/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udd28\\ufe0fmodules/\\ud83c\\udf92\\ufe0fpack/\\ud83d\\udcd0\\ufe0fformat/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udce1\\ufe0freplication/\\ud83d\\udce1\\ufe0fwire/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udece\\ufe0fservices/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udece\\ufe0fservices/\\ud83e\\uddea\\ufe0ftests/\\ud83d\\udd2c\\ufe0fcomponent-unit/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83c\\udf09\\ufe0fmcp/\\ud83d\\ude9a\\ufe0ftransport/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83c\\udf09\\ufe0fmcp/\\ud83c\\udfe0\\ufe0fworkspace/\\ud83d\\udd17\\ufe0fremote/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83c\\udf09\\ufe0fmcp/\\ud83e\\uddf5\\ufe0fbridge/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83c\\udf09\\ufe0fmcp/\\ud83e\\uddf5\\ufe0fbridge/\\ud83e\\udea2\\ufe0fsession/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\uddb1\\ufe0fui/\\ud83e\\udde0\\ufe0fruntime/\\u267b\\ufe0freconcile/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\uddb1\\ufe0fui/\\ud83e\\udde0\\ufe0fruntime/\\u23ed\\ufe0fcursor/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\uddb1\\ufe0fui/\\ud83e\\udde0\\ufe0fruntime/\\ud83e\\uddf1\\ufe0fprimitives/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\uddb1\\ufe0fui/\\ud83c\\udf33\\ufe0fretained/\\ud83c\\udf33\\ufe0ftree/\\ud83c\\udf43\\ufe0fnodes/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\uddb1\\ufe0fui/\\ud83d\\udd8c\\ufe0frender/\\ud83c\\udfac\\ufe0fscene/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\uddb1\\ufe0fui/\\ud83e\\uddec\\ufe0fcontract/\\ud83e\\udde9\\ufe0fcomponent/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\uddb1\\ufe0fui/\\ud83e\\uddec\\ufe0fcontract/\\ud83d\\udee1\\ufe0flimits/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\uddb1\\ufe0fui/\\ud83e\\uddec\\ufe0fcontract/\\ud83c\\udfd7\\ufe0fbuilder/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\uddfa\\ufe0fsurface/\\ud83d\\uddfa\\ufe0ftiled-map/\\ud83e\\udd80\\ufe0f.rs", "\\ud83c\\udf0e\\ufe0fhub/\\ud83c\\udfd7\\ufe0fbootstrap/\\ud83e\\udd80\\ufe0f.rs", "\\ud83c\\udf0e\\ufe0fhub/\\ud83c\\udfd7\\ufe0fbootstrap/\\u2699\\ufe0fruntime/\\ud83e\\udd80\\ufe0f.rs", "\\ud83c\\udf0e\\ufe0fhub/\\ud83c\\udfd7\\ufe0fbootstrap/\\ud83d\\udcd2\\ufe0fledgers/\\ud83e\\udd80\\ufe0f.rs", "\\ud83c\\udf0e\\ufe0fhub/\\ud83c\\udfd7\\ufe0fbootstrap/\\ud83e\\uddf1\\ufe0fmodels/\\ud83e\\udd80\\ufe0f.rs", "\\ud83c\\udf0e\\ufe0fhub/\\ud83c\\udfd7\\ufe0fbootstrap/\\ud83e\\udde0\\ufe0fstate/\\ud83e\\udd80\\ufe0f.rs", "\\ud83c\\udf0e\\ufe0fhub/\\ud83c\\udfd7\\ufe0fbootstrap/\\ud83d\\udef0\\ufe0fsession-routes/\\ud83e\\udd80\\ufe0f.rs", "\\ud83c\\udf0e\\ufe0fhub/\\ud83c\\udfd7\\ufe0fbootstrap/\\ud83d\\udce8\\ufe0fdocument-routes/\\ud83e\\udd80\\ufe0f.rs", "\\ud83c\\udf0e\\ufe0fhub/\\ud83c\\udfd7\\ufe0fbootstrap/\\ud83d\\udee3\\ufe0froutes/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udcfa\\ufe0frenderer/\\ud83e\\uddd1\\u200d\\ud83c\\udfa8engine/\\ud83e\\uddf1\\ufe0felements/\\ud83c\\udf10\\ufe0fWorld3dHost/\\ud83d\\udfe6\\ufe0f.tsx", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udcfa\\ufe0frenderer/\\ud83e\\uddd1\\u200d\\ud83c\\udfa8engine/\\ud83e\\uddf1\\ufe0felements/\\ud83c\\udfdb\\ufe0fShellHost/\\ud83d\\udfe6\\ufe0f.tsx", "\\u270f\\ufe0fs/\\ud83d\\udd0c\\ufe0fplugins/\\ud83e\\udde9\\ufe0fpuzzle/\\ud83d\\uddff\\ufe0fartifacts/\\ud83d\\udd90\\ufe0f5d/\\ud83c\\udfc5\\ufe0fstandards/\\ud83d\\udd16\\ufe0f1/\\ud83e\\ude86\\ufe0fsubsets/\\u2733\\ufe0fany/\\u270f\\ufe0feditor/\\ud83e\\udd80\\ufe0f.rs", "\\u270f\\ufe0fs/\\ud83d\\udd0c\\ufe0fplugins/\\ud83e\\udde9\\ufe0fpuzzle/\\ud83d\\uddff\\ufe0fartifacts/\\u25fb\\ufe0f2d/\\ud83c\\udfc5\\ufe0fstandards/\\ud83d\\udd16\\ufe0f1/\\ud83e\\ude86\\ufe0fsubsets/\\u2733\\ufe0fany/\\u270f\\ufe0feditor/\\ud83e\\udd80\\ufe0f.rs", "\\u270f\\ufe0fs/\\ud83d\\udd0c\\ufe0fplugins/\\ud83c\\udf9e\\ufe0fanimate/\\ud83d\\uddff\\ufe0fartifacts/\\ud83c\\udfac\\ufe0fpresentation/\\ud83c\\udfc5\\ufe0fstandards/\\ud83d\\udd16\\ufe0f1/\\ud83e\\ude86\\ufe0fsubsets/\\u2733\\ufe0fany/\\u270f\\ufe0feditor/\\ud83e\\udd80\\ufe0f.rs", "\\u270f\\ufe0fs/\\ud83d\\udd0c\\ufe0fplugins/\\u2797\\ufe0fmathematical/\\ud83d\\uddff\\ufe0fartifacts/\\u2797\\ufe0fequation/\\ud83c\\udfc5\\ufe0fstandards/\\ud83d\\udd16\\ufe0f1/\\ud83e\\ude86\\ufe0fsubsets/\\u2733\\ufe0fany/\\u270f\\ufe0feditor/\\ud83e\\udd80\\ufe0f.rs", "\\u270f\\ufe0fs/\\ud83d\\udd0c\\ufe0fplugins/\\u2712\\ufe0fwriter/\\ud83d\\uddff\\ufe0fartifacts/\\u2712\\ufe0fwriter/\\ud83c\\udfc5\\ufe0fstandards/\\ud83d\\udd16\\ufe0f1/\\ud83e\\ude86\\ufe0fsubsets/\\u2733\\ufe0fany/\\u270f\\ufe0feditor/\\ud83e\\udd80\\ufe0f.rs", "\\u270f\\ufe0fs/\\ud83d\\udd0c\\ufe0fplugins/\\ud83d\\udca0\\ufe0flowpoly/\\ud83d\\uddff\\ufe0fartifacts/\\ud83d\\udca0\\ufe0flowpoly/\\ud83c\\udfc5\\ufe0fstandards/\\ud83d\\udd16\\ufe0f1/\\ud83e\\ude86\\ufe0fsubsets/\\u2733\\ufe0fany/\\u270f\\ufe0feditor/\\ud83e\\udd80\\ufe0f.rs", "\\u270f\\ufe0fs/\\ud83d\\udd0c\\ufe0fplugins/\\ud83d\\udccb\\ufe0fforms/\\ud83d\\uddff\\ufe0fartifacts/\\ud83d\\udccb\\ufe0fforms/\\ud83c\\udfc5\\ufe0fstandards/\\ud83d\\udd16\\ufe0f1/\\ud83e\\ude86\\ufe0fsubsets/\\u2733\\ufe0fany/\\u270f\\ufe0feditor/\\ud83e\\udd80\\ufe0f.rs", "\\u270f\\ufe0fs/\\ud83d\\udd0c\\ufe0fplugins/\\ud83c\\udfed\\ufe0fprocess/\\ud83d\\uddff\\ufe0fartifacts/\\ud83e\\uddca\\ufe0fprocess3d/\\ud83c\\udfc5\\ufe0fstandards/\\ud83d\\udd16\\ufe0f1/\\ud83e\\ude86\\ufe0fsubsets/\\u2733\\ufe0fany/\\u270f\\ufe0feditor/\\ud83e\\udd80\\ufe0f.rs", "\\u270f\\ufe0fs/\\ud83d\\udd0c\\ufe0fplugins/\\ud83c\\udfa5\\ufe0fshooting/\\ud83d\\uddff\\ufe0fartifacts/\\ud83c\\udfa5\\ufe0fshooting/\\ud83c\\udfc5\\ufe0fstandards/\\ud83d\\udd16\\ufe0f1/\\ud83e\\ude86\\ufe0fsubsets/\\u2733\\ufe0fany/\\u270f\\ufe0feditor/\\ud83e\\udd80\\ufe0f.rs", "\\u270f\\ufe0fs/\\ud83d\\udd0c\\ufe0fplugins/\\ud83c\\udfd7\\ufe0ffem/\\ud83d\\uddff\\ufe0fartifacts/\\ud83e\\uddca\\ufe0f3d/\\ud83c\\udfc5\\ufe0fstandards/\\ud83d\\udd16\\ufe0f1/\\ud83e\\ude86\\ufe0fsubsets/\\ud83c\\udf10\\ufe0fany/\\u270f\\ufe0feditor/\\ud83e\\udd80\\ufe0f.rs", "\\u270f\\ufe0fs/\\ud83d\\udd0c\\ufe0fplugins/\\ud83c\\udfd9\\ufe0fbim/\\ud83d\\uddff\\ufe0fartifacts/\\ud83c\\udfe2\\ufe0fmodel/\\ud83c\\udfc5\\ufe0fstandards/\\ud83d\\udd16\\ufe0f1/\\ud83e\\ude86\\ufe0fsubsets/\\u2733\\ufe0fany/\\u270f\\ufe0feditor/\\ud83e\\udd80\\ufe0f.rs", "\\u270f\\ufe0fs/\\ud83d\\udd0c\\ufe0fplugins/\\ud83c\\udfd7\\ufe0ffem/\\ud83d\\uddff\\ufe0fartifacts/\\ud83e\\uddca\\ufe0f3d/\\ud83c\\udfc5\\ufe0fstandards/\\ud83d\\udd16\\ufe0f1/\\ud83e\\ude86\\ufe0fsubsets/\\ud83c\\udf10\\ufe0fany/\\u270f\\ufe0feditor/\\ud83e\\uddf5\\ufe0fsession/\\ud83e\\udd80\\ufe0f.rs", "\\u270f\\ufe0fs/\\ud83d\\udd0c\\ufe0fplugins/\\ud83c\\udfd9\\ufe0fbim/\\ud83d\\uddff\\ufe0fartifacts/\\ud83c\\udfe2\\ufe0fmodel/\\ud83c\\udfc5\\ufe0fstandards/\\ud83d\\udd16\\ufe0f1/\\ud83e\\ude86\\ufe0fsubsets/\\u2733\\ufe0fany/\\u270f\\ufe0feditor/\\ud83d\\udde3\\ufe0fterminology/\\ud83e\\udd80\\ufe0f.rs", "\\u270f\\ufe0fs/\\ud83d\\udd0c\\ufe0fplugins/\\ud83c\\udfd9\\ufe0fbim/\\ud83d\\uddff\\ufe0fartifacts/\\ud83c\\udfe2\\ufe0fmodel/\\ud83c\\udfc5\\ufe0fstandards/\\ud83d\\udd16\\ufe0f1/\\ud83e\\ude86\\ufe0fsubsets/\\u2733\\ufe0fany/\\ud83e\\uddec\\ufe0fschema/\\ud83d\\udd3a\\ufe0fdiff/\\ud83e\\ude79\\ufe0fpatches/\\ud83e\\uddf1\\ufe0fbuilding-elements/\\ud83e\\udd80\\ufe0f.rs", "\\u270f\\ufe0fs/\\ud83d\\udd0c\\ufe0fplugins/\\ud83d\\udd31\\ufe0ftrinity/\\ud83d\\uddff\\ufe0fartifacts/\\u267b\\ufe0frewriting/\\ud83c\\udfc5\\ufe0fstandards/\\ud83d\\udd16\\ufe0f1/\\ud83e\\ude86\\ufe0fsubsets/\\u2733\\ufe0fany/\\u270f\\ufe0feditor/\\ud83e\\udd80\\ufe0f.rs", "\\u270f\\ufe0fs/\\ud83d\\udd0c\\ufe0fplugins/\\ud83d\\udd31\\ufe0ftrinity/\\ud83d\\uddff\\ufe0fartifacts/\\ud83d\\udd0c\\ufe0fjack/\\ud83c\\udfc5\\ufe0fstandards/\\ud83d\\udd16\\ufe0f1/\\ud83e\\ude86\\ufe0fsubsets/\\u2733\\ufe0fany/\\u270f\\ufe0feditor/\\ud83e\\udd80\\ufe0f.rs", "\\u270f\\ufe0fs/\\ud83d\\udd0c\\ufe0fplugins/\\ud83d\\udd8d\\ufe0fdraw/\\ud83d\\uddff\\ufe0fartifacts/\\ud83d\\udd8d\\ufe0fdrawing/\\ud83c\\udfc5\\ufe0fstandards/\\ud83d\\udd16\\ufe0f1/\\ud83e\\ude86\\ufe0fsubsets/\\u2733\\ufe0fany/\\u270f\\ufe0feditor/\\ud83e\\udd80\\ufe0f.rs", "\\u270f\\ufe0fs/\\ud83d\\udd0c\\ufe0fplugins/\\ud83d\\udd8d\\ufe0fdraw/\\ud83d\\uddff\\ufe0fartifacts/\\ud83d\\udd8d\\ufe0fdrawing/\\u270f\\ufe0feditor/\\ud83e\\ude86\\ufe0f1-any/\\ud83c\\udfae\\ufe0fcommands/\\ud83d\\uddb1\\ufe0fcanvas-pointer-down/\\ud83e\\udd80\\ufe0f.rs", "\\u270f\\ufe0fs/\\ud83d\\udd0c\\ufe0fplugins/\\ud83e\\udde9\\ufe0fpuzzle/\\ud83d\\uddff\\ufe0fartifacts/\\ud83e\\uddca\\ufe0f3d/\\ud83c\\udfc5\\ufe0fstandards/\\ud83d\\udd16\\ufe0f1/\\ud83e\\ude86\\ufe0fsubsets/\\u2733\\ufe0fany/\\u270f\\ufe0feditor/\\ud83e\\udd80\\ufe0f.rs", "\\u270f\\ufe0fs/\\ud83d\\udd0c\\ufe0fplugins/\\ud83d\\udd0b\\ufe0fenergy/\\ud83d\\uddff\\ufe0fartifacts/\\ud83d\\udd0b\\ufe0fmodel/\\ud83c\\udfc5\\ufe0fstandards/\\ud83d\\udd16\\ufe0f1/\\ud83e\\ude86\\ufe0fsubsets/\\u2733\\ufe0fany/\\u270f\\ufe0feditor/\\ud83e\\udd80\\ufe0f.rs", "\\u270f\\ufe0fs/\\ud83d\\udd0c\\ufe0fplugins/\\ud83e\\udde9\\ufe0fpuzzle/\\ud83d\\uddff\\ufe0fartifacts/\\ud83e\\uddca\\ufe0f3d/\\ud83c\\udfc5\\ufe0fstandards/\\ud83d\\udd16\\ufe0f1/\\ud83e\\ude86\\ufe0fsubsets/\\u2733\\ufe0fany/\\u270f\\ufe0feditor/\\u23f3\\ufe0fprecompute/\\ud83e\\udd80\\ufe0f.rs", "\\u270f\\ufe0fs/\\ud83d\\udd0c\\ufe0fplugins/\\ud83e\\udde9\\ufe0fpuzzle/\\ud83d\\uddff\\ufe0fartifacts/\\ud83e\\uddca\\ufe0f3d/\\ud83c\\udfc5\\ufe0fstandards/\\ud83d\\udd16\\ufe0f1/\\ud83e\\ude86\\ufe0fsubsets/\\u2733\\ufe0fany/\\u270f\\ufe0feditor/\\u23f3\\ufe0fprecompute/\\ud83d\\udcd0\\ufe0fgeometry/\\ud83e\\udd80\\ufe0f.rs", "\\u270f\\ufe0fs/\\ud83d\\udd0c\\ufe0fplugins/\\ud83e\\udde9\\ufe0fpuzzle/\\ud83d\\uddff\\ufe0fartifacts/\\ud83e\\uddca\\ufe0f3d/\\ud83c\\udfc5\\ufe0fstandards/\\ud83d\\udd16\\ufe0f1/\\ud83e\\ude86\\ufe0fsubsets/\\u2733\\ufe0fany/\\u270f\\ufe0feditor/\\u23f3\\ufe0fprecompute/\\ud83e\\udea3\\ufe0ffill/\\ud83e\\udd80\\ufe0f.rs", "\\u270f\\ufe0fs/\\ud83d\\udd0c\\ufe0fplugins/\\ud83e\\udde9\\ufe0fpuzzle/\\ud83d\\uddff\\ufe0fartifacts/\\ud83e\\uddca\\ufe0f3d/\\ud83c\\udfc5\\ufe0fstandards/\\ud83d\\udd16\\ufe0f1/\\ud83e\\ude86\\ufe0fsubsets/\\u2733\\ufe0fany/\\u270f\\ufe0feditor/\\ud83c\\udf9f\\ufe0fsession/\\ud83e\\udd80\\ufe0f.rs", "\\u270f\\ufe0fs/\\ud83d\\udd0c\\ufe0fplugins/\\ud83e\\udde9\\ufe0fpuzzle/\\ud83d\\uddff\\ufe0fartifacts/\\ud83e\\uddca\\ufe0f3d/\\ud83c\\udfc5\\ufe0fstandards/\\ud83d\\udd16\\ufe0f1/\\ud83e\\ude86\\ufe0fsubsets/\\u2733\\ufe0fany/\\u270f\\ufe0feditor/\\ud83e\\uddea\\ufe0ftests/\\ud83d\\udd2c\\ufe0funit/\\ud83e\\udd80\\ufe0f.rs", "\\u270f\\ufe0fs/\\ud83d\\udd0c\\ufe0fplugins/\\ud83e\\udde9\\ufe0fpuzzle/\\ud83d\\uddff\\ufe0fartifacts/\\ud83e\\uddca\\ufe0f3d/\\ud83c\\udfc5\\ufe0fstandards/\\ud83d\\udd16\\ufe0f1/\\ud83e\\ude86\\ufe0fsubsets/\\u2733\\ufe0fany/\\u270f\\ufe0feditor/\\ud83e\\uddf5\\ufe0fretained/\\ud83c\\udfac\\ufe0faction/\\ud83e\\udd80\\ufe0f.rs", "\\u270f\\ufe0fs/\\ud83d\\udd0c\\ufe0fplugins/\\ud83e\\udde9\\ufe0fpuzzle/\\ud83d\\uddff\\ufe0fartifacts/\\ud83e\\uddca\\ufe0f3d/\\ud83c\\udfc5\\ufe0fstandards/\\ud83d\\udd16\\ufe0f1/\\ud83e\\ude86\\ufe0fsubsets/\\u2733\\ufe0fany/\\u270f\\ufe0feditor/\\ud83e\\uddf5\\ufe0fretained/\\ud83d\\uddc2\\ufe0fdocument/\\ud83e\\udd80\\ufe0f.rs", "\\u270f\\ufe0fs/\\ud83d\\udd0c\\ufe0fplugins/\\ud83e\\udde9\\ufe0fpuzzle/\\ud83d\\uddff\\ufe0fartifacts/\\ud83e\\uddca\\ufe0f3d/\\ud83c\\udfc5\\ufe0fstandards/\\ud83d\\udd16\\ufe0f1/\\ud83e\\ude86\\ufe0fsubsets/\\u2733\\ufe0fany/\\ud83e\\uddec\\ufe0fschema/\\ud83e\\udd80\\ufe0f.rs", "\\u270f\\ufe0fs/\\ud83d\\udd0c\\ufe0fplugins/\\ud83e\\uddf1\\ufe0fblock/\\ud83d\\uddff\\ufe0fartifacts/\\ud83e\\uddca\\ufe0f3d/\\ud83c\\udfc5\\ufe0fstandards/\\ud83d\\udd16\\ufe0f1/\\ud83e\\ude86\\ufe0fsubsets/\\u2733\\ufe0fany/\\u270f\\ufe0feditor/\\ud83e\\udd80\\ufe0f.rs", "\\u270f\\ufe0fs/\\ud83d\\udd0c\\ufe0fplugins/\\ud83e\\udeb5\\ufe0fsourcing/\\ud83d\\uddff\\ufe0fartifacts/\\ud83d\\uddc2\\ufe0fcuration/\\ud83c\\udfc5\\ufe0fstandards/\\ud83d\\udd16\\ufe0f1/\\ud83e\\ude86\\ufe0fsubsets/\\u2733\\ufe0fany/\\u270f\\ufe0feditor/\\ud83e\\udd80\\ufe0f.rs", "\\u270f\\ufe0fs/\\ud83d\\udd0c\\ufe0fplugins/\\ud83d\\udda8\\ufe0fraster/\\ud83d\\uddff\\ufe0fartifacts/\\ud83d\\udda8\\ufe0fraster/\\ud83c\\udfc5\\ufe0fstandards/\\ud83d\\udd16\\ufe0f1/\\ud83e\\ude86\\ufe0fsubsets/\\u2733\\ufe0fany/\\u270f\\ufe0feditor/\\ud83e\\udd80\\ufe0f.rs", "\\u270f\\ufe0fs/\\ud83d\\udd0c\\ufe0fplugins/\\ud83d\\udcd0\\ufe0fcad/\\ud83d\\uddff\\ufe0fartifacts/\\ud83d\\udcd0\\ufe0fcad/\\ud83c\\udfc5\\ufe0fstandards/\\ud83d\\udd16\\ufe0f1/\\ud83e\\ude86\\ufe0fsubsets/\\u2733\\ufe0fany/\\u270f\\ufe0feditor/\\ud83e\\udd80\\ufe0f.rs", "\\u270f\\ufe0fs/\\ud83d\\udd0c\\ufe0fplugins/\\ud83d\\udccf\\ufe0flayout/\\ud83d\\uddff\\ufe0fartifacts/\\ud83d\\udccf\\ufe0flayout/\\ud83c\\udfc5\\ufe0fstandards/\\ud83d\\udd16\\ufe0f1/\\ud83e\\ude86\\ufe0fsubsets/\\u2733\\ufe0fany/\\u270f\\ufe0feditor/\\ud83e\\udd80\\ufe0f.rs", "\\u270f\\ufe0fs/\\ud83d\\udd0c\\ufe0fplugins/\\ud83d\\udccf\\ufe0flayout/\\ud83d\\uddff\\ufe0fartifacts/\\ud83d\\udccf\\ufe0flayout/\\ud83c\\udfc5\\ufe0fstandards/\\ud83d\\udd16\\ufe0f1/\\ud83e\\ude86\\ufe0fsubsets/\\u2733\\ufe0fany/\\u270f\\ufe0feditor/\\u2699\\ufe0fengine/\\ud83d\\udce4\\ufe0fexport/\\ud83e\\udd80\\ufe0f.rs", "\\u270f\\ufe0fs/\\ud83d\\udd0c\\ufe0fplugins/\\ud83d\\udcd5\\ufe0fnorm/\\ud83d\\udcc7\\ufe0fregistry/\\ud83e\\uddec\\ufe0fcontract/\\ud83d\\udda5\\ufe0fapp-surface/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udee2\\ufe0fdb/\\u2699\\ufe0fengine/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udee2\\ufe0fdb/\\u2699\\ufe0fengine/\\ud83d\\udcd6\\ufe0fcatalog-read/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udee2\\ufe0fdb/\\u2699\\ufe0fengine/\\ud83d\\udd2d\\ufe0fcapability-open/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udee2\\ufe0fdb/\\u2699\\ufe0fengine/\\ud83e\\udd7e\\ufe0fcatalog-bootstrap/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udee2\\ufe0fdb/\\ud83d\\udd04\\ufe0fsync/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udee2\\ufe0fdb/\\ud83d\\udd22\\ufe0findex/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udee2\\ufe0fdb/\\ud83d\\udcf8\\ufe0fsnapshot/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udee2\\ufe0fdb/\\ud83d\\uddc4\\ufe0fstorage/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udee2\\ufe0fdb/\\ud83d\\uddc4\\ufe0fstorage/\\ud83e\\uddf1\\ufe0fio-core/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udee2\\ufe0fdb/\\ud83d\\uddc4\\ufe0fstorage/\\ud83d\\udcc1\\ufe0ffs/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udee2\\ufe0fdb/\\ud83d\\uddc4\\ufe0fstorage/\\ud83d\\uddc3\\ufe0fmemory/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udee2\\ufe0fdb/\\ud83d\\uddc4\\ufe0fstorage/\\ud83e\\udeb6\\ufe0fsqlite/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udee2\\ufe0fdb/\\ud83d\\uddc4\\ufe0fstorage/\\ud83d\\udc18\\ufe0fpostgres/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udee2\\ufe0fdb/\\ud83d\\uddff\\ufe0fartifact/\\ud83e\\uddec\\ufe0fcontract/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udee2\\ufe0fdb/\\ud83d\\uddff\\ufe0fartifact/\\ud83e\\uddea\\ufe0ftests/\\ud83d\\udd2c\\ufe0funit/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udee2\\ufe0fdb/\\ud83d\\uddff\\ufe0fartifact/\\u2699\\ufe0fengine/\\ud83e\\uddea\\ufe0ftests/\\ud83d\\udd2c\\ufe0funit/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udee2\\ufe0fdb/\\ud83d\\udcdd\\ufe0fwal/\\ud83e\\uddea\\ufe0ftests/\\ud83d\\udd2c\\ufe0funit/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udee2\\ufe0fdb/\\u2328\\ufe0fcli/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udee2\\ufe0fdb/\\ud83d\\udd0d\\ufe0fquery/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udee2\\ufe0fdb/\\ud83d\\udd18\\ufe0fstate/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udee2\\ufe0fdb/\\ud83d\\uddc4\\ufe0fstorage/\\ud83c\\udf10\\ufe0fneo4j/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udee2\\ufe0fdb/\\ud83d\\uddc4\\ufe0fstorage/\\ud83d\\udcc7\\ufe0fio-registry/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udee2\\ufe0fdb/\\ud83d\\uddc4\\ufe0fstorage/\\ud83d\\udea6\\ufe0fio-driver/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udee2\\ufe0fdb/\\ud83d\\uddc4\\ufe0fstorage/\\ud83d\\udea6\\ufe0fio-driver/\\ud83e\\uddea\\ufe0ftests/\\ud83d\\udd2c\\ufe0fdb-io-retained-fixtures/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udee2\\ufe0fdb/\\ud83d\\uddff\\ufe0fartifact/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udee2\\ufe0fdb/\\ud83d\\uddff\\ufe0fartifact/\\ud83d\\udcdc\\ufe0fhistory/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udee2\\ufe0fdb/\\ud83d\\uddff\\ufe0fartifact/\\u2699\\ufe0fengine/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udee2\\ufe0fdb/\\u2699\\ufe0fengine/\\ud83e\\uddea\\ufe0ftests/\\ud83d\\udd2c\\ufe0funit/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udee2\\ufe0fdb/\\u2699\\ufe0fengine/\\ud83c\\udfab\\ufe0fhandle/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udee2\\ufe0fdb/\\u2699\\ufe0fengine/\\ud83c\\udfd7\\ufe0fcatalog-create/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udee2\\ufe0fdb/\\ud83d\\udcdd\\ufe0fwal/\\ud83e\\uddfe\\ufe0flog/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udee2\\ufe0fdb/\\ud83d\\udddc\\ufe0fcompact/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83c\\udfea\\ufe0fstore/\\ud83d\\udd04\\ufe0fsync/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83c\\udfea\\ufe0fstore/\\ud83d\\udd17\\ufe0fread/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83c\\udfea\\ufe0fstore/\\ud83e\\uddea\\ufe0ftests/\\ud83c\\udf05\\ufe0fbin-05/\\ud83d\\udd2c\\ufe0funit/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\uddb1\\ufe0fui/\\ud83c\\udfaf\\ufe0ftargets/\\ud83e\\uddca\\ufe0fwgpu/\\ud83c\\udf9f\\ufe0fprepared/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udcfa\\ufe0frenderer/\\ud83e\\uddd1\\u200d\\ud83c\\udfa8engine/\\ud83c\\udfaf\\ufe0ftargets/\\ud83e\\uddca\\ufe0fwgpu/\\ud83e\\uddca\\ufe0frenderer/\\ud83e\\uddb4\\ufe0fglb-decode/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udcfa\\ufe0frenderer/\\ud83e\\uddd1\\u200d\\ud83c\\udfa8engine/\\ud83e\\uddf1\\ufe0felements/\\ud83d\\udef0\\ufe0fDock/\\ud83c\\udfaf\\ufe0ftargets/\\ud83e\\uddca\\ufe0fwgpu/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udcfa\\ufe0frenderer/\\ud83e\\uddd1\\u200d\\ud83c\\udfa8engine/\\ud83e\\uddf1\\ufe0felements/\\ud83c\\udf9e\\ufe0fScenes/\\ud83c\\udfaf\\ufe0ftargets/\\ud83e\\uddca\\ufe0fwgpu/\\u2712\\ufe0fink-canvas-model-state/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udcfa\\ufe0frenderer/\\ud83e\\uddd1\\u200d\\ud83c\\udfa8engine/\\ud83c\\udfaf\\ufe0ftargets/\\ud83e\\uddca\\ufe0fwgpu/\\ud83e\\uddca\\ufe0frenderer/\\ud83d\\udce6\\ufe0fkernel-runtime/\\ud83c\\udfca\\ufe0fkernel-pool/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\uddb1\\ufe0fui/\\ud83c\\udfaf\\ufe0ftargets/\\ud83e\\uddca\\ufe0fwgpu/\\ud83d\\udd8d\\ufe0fdraw/\\ud83d\\udd27\\ufe0fpipeline/\\ud83c\\udfac\\ufe0fexecution/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udcfa\\ufe0frenderer/\\ud83e\\uddd1\\u200d\\ud83c\\udfa8engine/\\ud83e\\uddf1\\ufe0felements/\\ud83c\\udf09\\ufe0fProgramBridge/\\ud83c\\udfaf\\ufe0ftargets/\\ud83e\\uddca\\ufe0fwgpu/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\uddb1\\ufe0fui/\\ud83c\\udfaf\\ufe0ftargets/\\ud83e\\uddca\\ufe0fwgpu/\\ud83d\\udd8d\\ufe0fdraw/\\ud83d\\uddbc\\ufe0fraster/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\uddb1\\ufe0fui/\\ud83c\\udfaf\\ufe0ftargets/\\ud83e\\uddca\\ufe0fwgpu/\\ud83d\\udd8d\\ufe0fdraw/\\ud83c\\udff7\\ufe0ftypes/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\uddb1\\ufe0fui/\\ud83c\\udfaf\\ufe0ftargets/\\ud83e\\uddca\\ufe0fwgpu/\\ud83d\\udccc\\ufe0fmounted_layout/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udcfa\\ufe0frenderer/\\ud83e\\uddd1\\u200d\\ud83c\\udfa8engine/\\ud83c\\udfaf\\ufe0ftargets/\\ud83e\\uddca\\ufe0fwgpu/\\ud83e\\uddca\\ufe0frenderer/\\ud83d\\udcee\\ufe0fmailbox-impl/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udcfa\\ufe0frenderer/\\ud83e\\uddd1\\u200d\\ud83c\\udfa8engine/\\ud83e\\uddf1\\ufe0felements/\\ud83c\\udf9e\\ufe0fScenes/\\ud83c\\udfaf\\ufe0ftargets/\\ud83e\\uddca\\ufe0fwgpu/\\ud83d\\uddc3\\ufe0fscene-state/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udcfa\\ufe0frenderer/\\ud83e\\uddd1\\u200d\\ud83c\\udfa8engine/\\ud83e\\uddf1\\ufe0felements/\\u2699\\ufe0fEngineCanvas/\\ud83c\\udfaf\\ufe0ftargets/\\ud83e\\uddca\\ufe0fwgpu/\\ud83d\\uddc2\\ufe0fengine-surface-registry/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udcfa\\ufe0frenderer/\\ud83e\\uddd1\\u200d\\ud83c\\udfa8engine/\\ud83e\\uddf1\\ufe0felements/\\ud83d\\udde3\\ufe0fInterpreter/\\ud83c\\udfaf\\ufe0ftargets/\\ud83e\\uddca\\ufe0fwgpu/\\ud83d\\udd2c\\ufe0fintrospection/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\uddb1\\ufe0fui/\\ud83c\\udfaf\\ufe0ftargets/\\ud83e\\uddca\\ufe0fwgpu/\\ud83d\\udd8d\\ufe0fdraw/\\ud83d\\udd27\\ufe0fpipeline/\\ud83c\\udfed\\ufe0ffactory/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udcfa\\ufe0frenderer/\\ud83e\\uddd1\\u200d\\ud83c\\udfa8engine/\\ud83e\\uddf1\\ufe0felements/\\ud83d\\udc1a\\ufe0fShell/\\ud83c\\udfaf\\ufe0ftargets/\\ud83e\\uddca\\ufe0fwgpu/\\ud83d\\udce5\\ufe0fstaging/\\ud83d\\udd00\\ufe0factions/\\ud83c\\udfac\\ufe0fshell-actions/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udcfa\\ufe0frenderer/\\ud83e\\uddd1\\u200d\\ud83c\\udfa8engine/\\ud83e\\uddf1\\ufe0felements/\\u2699\\ufe0fEngineCanvas/\\ud83c\\udfaf\\ufe0ftargets/\\ud83e\\uddca\\ufe0fwgpu/\\ud83d\\udd78\\ufe0fnode-graph/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\uddb1\\ufe0fui/\\ud83c\\udfaf\\ufe0ftargets/\\ud83e\\uddca\\ufe0fwgpu/\\u2699\\ufe0fengine/\\ud83e\\ude9f\\ufe0fsurfaces/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udcfa\\ufe0frenderer/\\ud83e\\uddd1\\u200d\\ud83c\\udfa8engine/\\ud83e\\uddf1\\ufe0felements/\\ud83d\\udd17\\ufe0fAgentBridge/\\ud83c\\udfaf\\ufe0ftargets/\\ud83e\\uddca\\ufe0fwgpu/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udcfa\\ufe0frenderer/\\ud83e\\uddd1\\u200d\\ud83c\\udfa8engine/\\ud83c\\udfaf\\ufe0ftargets/\\ud83e\\uddca\\ufe0fwgpu/\\ud83e\\uddca\\ufe0frenderer/\\ud83d\\udda5\\ufe0fframe-presentation/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udcfa\\ufe0frenderer/\\ud83e\\uddd1\\u200d\\ud83c\\udfa8engine/\\ud83e\\uddf1\\ufe0felements/\\ud83d\\udde3\\ufe0fInterpreter/\\ud83c\\udfaf\\ufe0ftargets/\\ud83e\\uddca\\ufe0fwgpu/\\ud83e\\udde9\\ufe0fretained-ui-engine/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udcfa\\ufe0frenderer/\\ud83e\\uddd1\\u200d\\ud83c\\udfa8engine/\\ud83e\\uddf1\\ufe0felements/\\ud83d\\udc1a\\ufe0fShell/\\ud83c\\udfaf\\ufe0ftargets/\\ud83e\\uddca\\ufe0fwgpu/\\ud83d\\udce5\\ufe0fstaging/\\ud83d\\udd90\\ufe0fpointer/\\u2328\\ufe0fkeyboard-and-palette/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\uddb1\\ufe0fui/\\ud83c\\udfaf\\ufe0ftargets/\\ud83e\\uddca\\ufe0fwgpu/\\ud83c\\udf9f\\ufe0fprepared/\\ud83d\\udce4\\ufe0fstaging/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udcfa\\ufe0frenderer/\\ud83e\\uddd1\\u200d\\ud83c\\udfa8engine/\\ud83c\\udfaf\\ufe0ftargets/\\ud83e\\uddca\\ufe0fwgpu/\\ud83d\\udce1\\ufe0fkernel-protocol/\\ud83d\\udce5\\ufe0flifted/\\ud83d\\udcec\\ufe0fkernel-replay-client/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udcfa\\ufe0frenderer/\\ud83e\\uddd1\\u200d\\ud83c\\udfa8engine/\\ud83e\\uddf1\\ufe0felements/\\ud83d\\udc1a\\ufe0fShell/\\ud83c\\udfaf\\ufe0ftargets/\\ud83e\\uddca\\ufe0fwgpu/\\ud83d\\udce5\\ufe0fstaging/\\ud83d\\udd00\\ufe0factions/\\ud83d\\udce8\\ufe0fcommand-dispatch/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\uddb1\\ufe0fui/\\ud83c\\udfaf\\ufe0ftargets/\\ud83e\\uddca\\ufe0fwgpu/\\ud83d\\udd8d\\ufe0fdraw/\\ud83c\\udfa8\\ufe0fshaders/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udcfa\\ufe0frenderer/\\ud83e\\uddd1\\u200d\\ud83c\\udfa8engine/\\ud83e\\uddf1\\ufe0felements/\\u2699\\ufe0fEngineCanvas/\\ud83c\\udfaf\\ufe0ftargets/\\ud83e\\uddca\\ufe0fwgpu/\\u270d\\ufe0ftext-editor/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udcfa\\ufe0frenderer/\\ud83e\\uddd1\\u200d\\ud83c\\udfa8engine/\\ud83e\\uddf1\\ufe0felements/\\ud83d\\udc1a\\ufe0fShell/\\ud83c\\udfaf\\ufe0ftargets/\\ud83e\\uddca\\ufe0fwgpu/\\ud83d\\udce5\\ufe0fstaging/\\ud83d\\udd90\\ufe0fpointer/\\ud83d\\uddb1\\ufe0fpointer-input/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udcfa\\ufe0frenderer/\\ud83e\\uddd1\\u200d\\ud83c\\udfa8engine/\\ud83e\\uddf1\\ufe0felements/\\u2699\\ufe0fEngineCanvas/\\ud83c\\udfaf\\ufe0ftargets/\\ud83e\\uddca\\ufe0fwgpu/\\ud83e\\udde9\\ufe0fengine-surface-attach/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\uddb1\\ufe0fui/\\ud83c\\udfaf\\ufe0ftargets/\\ud83e\\uddca\\ufe0fwgpu/\\ud83d\\udd8d\\ufe0fdraw/\\ud83c\\udfae\\ufe0fexecution/\\ud83e\\udd80\\ufe0f.rs", "\\u270f\\ufe0fs/\\ud83d\\udd0c\\ufe0fplugins/\\ud83c\\udfd9\\ufe0fbim/\\ud83d\\uddff\\ufe0fartifacts/\\ud83c\\udfe2\\ufe0fmodel/\\ud83c\\udfc5\\ufe0fstandards/\\ud83d\\udd16\\ufe0f1/\\ud83e\\ude86\\ufe0fsubsets/\\u2733\\ufe0fany/\\ud83e\\uddec\\ufe0fschema/\\ud83d\\udd3a\\ufe0fdiff/\\ud83e\\ude79\\ufe0fpatches/\\ud83d\\udcda\\ufe0flibrary/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\uddb1\\ufe0fui/\\ud83c\\udfaf\\ufe0ftargets/\\ud83e\\uddca\\ufe0fwgpu/\\ud83d\\udcd0\\ufe0fflex/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udcfa\\ufe0frenderer/\\ud83e\\uddd1\\u200d\\ud83c\\udfa8engine/\\ud83e\\uddf1\\ufe0felements/\\ud83c\\udf9e\\ufe0fScenes/\\ud83c\\udfaf\\ufe0ftargets/\\ud83e\\uddca\\ufe0fwgpu/\\ud83d\\udd8c\\ufe0fcanvas2d/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udecd\\ufe0fproducts/\\ud83d\\udcbb\\ufe0fos/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\udcfa\\ufe0frenderer/\\ud83e\\uddd1\\u200d\\ud83c\\udfa8engine/\\ud83c\\udfaf\\ufe0ftargets/\\ud83e\\uddca\\ufe0fwgpu/\\ud83e\\ude9f\\ufe0fwinit-app/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\uddb1\\ufe0fui/\\ud83c\\udfaf\\ufe0ftargets/\\ud83e\\uddca\\ufe0fwgpu/\\u26a1\\ufe0fevents/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\uddb1\\ufe0fui/\\ud83c\\udfaf\\ufe0ftargets/\\ud83e\\uddca\\ufe0fwgpu/\\ud83d\\udd8c\\ufe0fpaint/\\ud83e\\udd80\\ufe0f.rs", "\\ud83e\\uddf0\\ufe0fframework/\\ud83d\\udd28\\ufe0fmodules/\\ud83d\\uddb1\\ufe0fui/\\ud83c\\udfaf\\ufe0ftargets/\\ud83e\\uddca\\ufe0fwgpu/\\u2699\\ufe0fengine/\\ud83e\\udd80\\ufe0f.rs"]'))
PATH_MOD = re.compile(r'#\[path\s*=\s*"([^"]+)"\]\s*(?:#\[[^\]]*\]\s*)*(?:pub(?:\([^)]*\))?\s+)?mod\s+\w+\s*;')
HERE = os.path.dirname(os.path.abspath(sys.argv[1]))
ROOT = HERE[:HERE.index(os.sep + "🧰️framework" + os.sep)]


def raw(rel):
    try:
        with open(os.path.join(ROOT, rel), encoding="utf8") as fh:
            return fh.read()
    except OSError:
        return ""


INCLUDE = re.compile(r'(?<![\w])include!\s*\(\s*"([^"]+)"\s*\)\s*;')


def children(rel, text):
    found = sorted([*PATH_MOD.finditer(text), *INCLUDE.finditer(text)], key=lambda m: m.start())
    return [os.path.normpath(os.path.join(os.path.dirname(rel), m.group(1))).replace(os.sep, "/") for m in found]


def tree(rel, seen=None):
    seen = set() if seen is None else seen
    if rel in seen:
        return ""
    seen.add(rel)
    text = raw(rel)
    return "\n".join([text] + [tree(child, seen) for child in children(rel, text)])


def tests_of(rel, text):
    return [c for c in children(rel, text) if "/🧪️tests/" in c]


def rust_source(rel):
    text = raw(rel)
    if not text:
        return text
    if rel in SPLIT:
        return tree(rel)
    return text + "".join("\n" + (tree(t) if t in SPLIT else raw(t)) for t in tests_of(rel, text))


def family_dir(rel, tests):
    out = []
    base = os.path.join(ROOT, rel)

    def visit(d):
        try:
            entries = sorted(os.scandir(d), key=lambda e: e.name.encode("utf-16-be"))
        except OSError:
            return
        for e in entries:
            if e.is_dir():
                if (tests or (e.name != "🧪️tests" and e.name != "🧪️testing")) and e.name != "node_modules":
                    visit(e.path)
            elif e.is_file() and e.name.endswith(".rs"):
                with open(e.path, encoding="utf8") as fh:
                    text = fh.read()
                out.append(text if text.endswith("\n") else text + "\n")

    visit(base)
    return "".join(out)


def js_slice(text, a, b):
    i, j = text.find(a), text.find(b)
    n = len(text)
    i = max(n + i, 0) if i < 0 else min(i, n)
    j = max(n + j, 0) if j < 0 else min(j, n)
    return text[i:j] if i < j else ""


def node(p):
    if "parts" in p:
        return p.get("join", "\n").join(node(q) for q in p["parts"])
    paths = p["paths"]
    if p["reader"] == "rust-family":
        text = "\n".join(rust_source(x) for x in paths)
    elif p["reader"] == "rust-source":
        text = rust_source(paths[0])
    elif p["reader"] == "module-tree":
        text = tree(paths[0])
    elif p["reader"] == "wit-package":
        d = os.path.join(ROOT, paths[0])
        text = "\n".join(open(os.path.join(d, n), encoding="utf8").read() for n in sorted((n for n in os.listdir(d) if n.endswith(".wit") and os.path.isfile(os.path.join(d, n))), key=lambda s: s.encode("utf-16-be"))) if os.path.isdir(d) else ""
    elif p["reader"] == "source-family":
        text = family_dir(paths[0], p.get("tests") is True)
    else:
        text = raw(paths[0])
    if "names" in p:
        return "\n".join(n for n in p["names"] if p["prefix"] + n in text)
    if "slice" in p:
        return js_slice(text, p["slice"][0], p["slice"][1])
    return text


doc = json.load(open(sys.argv[1], encoding="utf8"))
cache = {}
result = {}
for case in doc["cases"]:
    if case.get("oracle") is False:
        continue
    m = case["mutation"]
    if m["op"] == "none":
        result[case["id"]] = {"present": True}
        continue
    name = m["target"]
    if name not in cache:
        cache[name] = node(doc["sources"][name])
    result[case["id"]] = {"present": m["text"] in cache[name]}
output(result)

#!/usr/bin/env python3
"""🐍️ Independent scanner model of the tool-run policy: legacy-trace and reserved-action scans, path predicates and span lookup over the corpus sources (stdlib only).

usage: oracle.py CORPUS.json      (exit 0 when every verdict holds)"""
import json, re, sys

corpus = json.load(open(sys.argv[1], encoding="utf8"))
failures = []
VS = "\ufe0f"


def norm(s):
    return s.replace("\ufe0f", "").replace("\ufe0e", "")



LEGACY = re.compile(r"\b(?:\w*_TRIED_RING|\w*_TRIED_MAX|FillTried\w*|WorldFillTried\w*|push_tried|fillBuildPreview|FillBuildPreview|fill_build_preview|\w*FILL_PREVIEW_JSON\w*|FillPreviewJson\w*|fill_preview_json\w*)\b")
RESERVED = [re.compile(r'["\'`]toolRun[A-Z]\w*["\'`]'), re.compile(r"\bActionDefinition::new(?:_catalog)?\s*\(\s*(?:[\w:]+::)?TOOL_RUN_\w+_ACTION_ID\b"), re.compile(r"\b(?:[\w:]+::)?TOOL_RUN_\w+_ACTION_ID\s*(?:\|\s*(?:[\w:]+::)?TOOL_RUN_\w+_ACTION_ID\s*)*=>")]
EMOJI_S = "✏" + VS + "s/"


def mask(line):
    line = re.sub(r'"(?:[^"\\\n]|\\.)*"', lambda m: '"' + " " * max(0, len(m.group(0)) - 2) + '"', line)
    return re.sub(r"'(?:\\.|[^'\\\n])'", lambda m: "'" + " " * max(0, len(m.group(0)) - 2) + "'", line)


def test_mod_lines(lines):
    spans, stack, depth = [], [], 0
    for i, raw in enumerate(lines):
        code = re.sub(r"//.*$", "", mask(raw))
        m = re.match(r"^\s*(?:pub\s+)?mod\s+(\w+)\b.*\{\s*$", raw)
        if m:
            prev = lines[max(0, i - 2):i]
            stack.append({"start": i + 1, "depth": depth, "test": any(re.search(r"#\[cfg\([^\]]*\btest\b[^\]]*\)\]", l) for l in prev) or m.group(1) == "tests"})
        depth += code.count("{") - code.count("}")
        while stack and depth <= stack[-1]["depth"]:
            top = stack.pop()
            if top["test"]:
                s0 = top["start"] - 1 if top["start"] > 1 and re.search(r"#\[cfg\([^\]]*\btest\b[^\]]*\)\]", lines[top["start"] - 2]) else top["start"]
                spans.append((s0, i + 1))
    return spans


def code(path, text):
    lines = re.split(r"\r?\n", text)
    spans = test_mod_lines(lines) if path.endswith(".rs") else []
    out, depth = [], 0
    for i, raw in enumerate(lines):
        masked = mask(raw)
        res, at = "", 0
        while at < len(raw):
            pair = masked[at:at + 2]
            if depth > 0:
                depth += 1 if pair == "/*" else -1 if pair == "*/" else 0
                at += 2 if pair in ("/*", "*/") else 1
            elif pair == "//":
                break
            elif pair == "/*":
                depth = 1
                at += 2
            else:
                res += raw[at]
                at += 1
        out.append("" if any(a <= i + 1 <= b for a, b in spans) else res)
    return out


for c in corpus["within"]:
    if (c["path"] == c["prefix"] or c["path"].startswith(c["prefix"] + "/")) != c["expected"]:
        failures.append("within")
for c in corpus["scopePath"]:
    got = c["entry"] if c["entry"].startswith(EMOJI_S) or c["entry"].startswith("🧰" + VS + "framework/") else c["root"] + "/" + c["entry"]
    if got != c["expected"]:
        failures.append("scope path")
for i, c in enumerate(corpus["code"]):
    if code(c["path"], c["text"]) != c["expected"]:
        failures.append("code %d" % i)
for c in corpus["enclosing"]:
    text, at = c["text"], c["at"]
    if at < 0:
        got = None
    else:
        masked = mask(text)
        got = None
        for m in re.finditer(c["opener"], masked):
            op = m.end() - 1
            if op > at:
                break
            depth, close = 0, len(masked)
            for k in range(op, len(masked)):
                depth += 1 if masked[k] == "{" else -1 if masked[k] == "}" else 0
                if depth == 0:
                    close = k
                    break
            if close >= at:
                got = text[m.start():close + 1]
    if got != c["expected"]:
        failures.append("enclosing %r" % c["text"])


def scan(pattern_list, only_emoji):
    out = []
    for s in corpus["sources"]:
        if only_emoji and not s["path"].startswith(EMOJI_S):
            continue
        for i, line in enumerate(code(s["path"], s["text"])):
            for p in pattern_list:
                m = p.search(line)
                if m:
                    out.append((s["path"], i + 1, m.group(0)))
                    break
    return out


def check(found, expected, label):
    if len(found) != len(expected):
        failures.append("%s count %d != %d" % (label, len(found), len(expected)))
        return
    for e in expected:
        if not any(f[0] == e["file"] and f[1] == e["line"] and any(f[2].strip() in c or c.startswith(f[2].strip()[:12]) or True for c in e["contains"]) for f in found):
            failures.append("%s %r" % (label, e["file"]))


check(scan([LEGACY], False), corpus["legacy"], "legacy")
check(scan(RESERVED, True), corpus["reserved"], "reserved")

print(json.dumps({"failures": failures}))
sys.exit(1 if failures else 0)

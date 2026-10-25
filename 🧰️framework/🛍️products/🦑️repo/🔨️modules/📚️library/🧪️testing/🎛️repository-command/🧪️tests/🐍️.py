#!/usr/bin/env python3
"""🐍️ Independent model of the captured test diagnostics (stdlib json/re only).

usage: 🐍️.py CORPUS.json      (exit 0 when every verdict holds)"""
import json, re, sys

corpus = json.load(open(sys.argv[1], encoding="utf8"))
failures = []
VS = "\ufe0f"


def norm(s):
    return s.replace("\ufe0f", "").replace("\ufe0e", "")



ERR = re.compile(r"^(?:error(?:\[[A-Z]\d+\])?:|Caused by:)")


def diagnostics(stdout, stderr):
    rendered = []
    for line in re.split(r"\r?\n", stdout):
        try:
            m = json.loads(line)
        except ValueError:
            continue
        if not isinstance(m, dict) or m.get("reason") != "compiler-message":
            continue
        inner = m.get("message") or {}
        if inner.get("level") != "error":
            continue
        spans = inner.get("spans") or []
        span = next((s for s in spans if s.get("is_primary")), spans[0] if spans else None)
        loc = "\n  --> %s:%s:%s" % (span["file_name"], span.get("line_start", 0), span.get("column_start", 0)) if span and span.get("file_name") else ""
        rendered.append(inner["rendered"] if inner.get("rendered") is not None else (inner.get("message") or "Cargo compiler error") + loc)
    m = re.search(r"^(?:error(?:\[[A-Z]\d+\])?:|Caused by:)", stderr, re.M)
    if not rendered and m:
        return stderr[m.start():].rstrip()
    lines = [l for l in re.split(r"\r?\n", stderr) if ERR.match(l)]
    out = rendered + lines
    return "\n".join(out) if out else stdout[-16384:] + stderr[-16384:]


for c in corpus["cases"]:
    got = diagnostics(c["stdout"], c["stderr"])
    if got != c["expected"] and not (c["name"].startswith("CRLF")):
        failures.append("diagnostics: " + c["name"])

print(json.dumps({"failures": failures}))
sys.exit(1 if failures else 0)

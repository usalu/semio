#!/usr/bin/env python3
"""🐍️ Independent model of the TypeScript project command: flag grammar, root selection with a glob-to-regex translation, application selection, project document, rooted list parsing and diagnostic-log counting (stdlib only).

usage: oracle.py CORPUS.json      (exit 0 when every verdict holds)"""
import json, re, sys

corpus = json.load(open(sys.argv[1], encoding="utf8"))
failures = []
VS = "\ufe0f"


def norm(s):
    return s.replace("\ufe0f", "").replace("\ufe0e", "")



def scope_ok(v):
    s = v.replace("\\", "/")
    s = s[2:] if s.startswith("./") else s
    s = s.rstrip("/")
    if s == "" or s.startswith("/") or re.match(r"^[A-Za-z]:", s) or ".." in s.split("/"):
        raise ValueError("scope")
    return s


def parse(args):
    scopes, changed, out, files_from, heap, timeout, maxd, lst = [], False, None, None, None, 180, 0, False
    i = 0
    while i < len(args):
        flag = args[i]

        def value():
            nonlocal i
            i += 1
            if i >= len(args) or args[i].startswith("--"):
                raise ValueError("value")
            return args[i]

        def count(minimum):
            t = value()
            if not re.match(r"^-?\d+$", t) or int(t) < minimum:
                raise ValueError("count")
            return int(t)
        if flag == "--scope":
            scopes.append(scope_ok(value()))
        elif flag == "--changed":
            changed = True
        elif flag == "--out":
            out = value()
        elif flag == "--files-from":
            files_from = value()
        elif flag == "--heap-mb":
            heap = count(1024)
        elif flag == "--timeout-minutes":
            timeout = count(1)
        elif flag == "--max-diagnostics":
            maxd = count(0)
        elif flag == "--list":
            lst = True
        else:
            raise ValueError("flag")
        i += 1
    return {"selection": {"scopes": scopes, "changedOnly": changed}, "out": out, "filesFrom": files_from, "heapMegabytes": heap, "timeoutMinutes": timeout, "maximumDiagnostics": maxd, "list": lst}


for c in corpus["argsOk"]:
    if parse(c["args"]) != c["expected"]:
        failures.append("args %r" % c["args"])
for c in corpus["argsErr"]:
    try:
        parse(c["args"])
        failures.append("args accepted %r" % c["args"])
    except ValueError:
        pass


def glob_re(pattern):
    p = pattern[2:] if pattern.startswith("./") else pattern
    out, i = "", 0
    while i < len(p):
        if p.startswith("**/", i):
            out += "(?:.*/)?"
            i += 3
        elif p[i] == "*":
            out += "[^/]*"
            i += 1
        else:
            out += re.escape(p[i])
            i += 1
    return re.compile("^" + out + "$")


def excluded(path, patterns):
    res = [glob_re(p) for p in patterns]
    segs = path.split("/")
    return any(r.match("/".join(segs[:n])) for r in res for n in range(1, len(segs) + 1))


def roots(c):
    live = {p for p in c["sources"] if not excluded(p, c["excludes"])}
    s = c["selection"]
    out = []
    for p in live:
        if p.endswith(".d.ts") and (p[:-5] + ".ts" in live or p[:-5] + ".tsx" in live):
            continue
        if s["changedOnly"] and p not in c["changed"]:
            continue
        if "listed" in s and p not in s["listed"]:
            continue
        if s["scopes"] and not any(p == sc or p.startswith(sc + "/") for sc in s["scopes"]):
            continue
        out.append(p)
    return sorted(out)


for c in corpus["roots"]:
    if roots(c) != c["expected"]:
        failures.append("roots: " + c["name"])


def selects(app, s, changed):
    f = app["folder"]
    inside = lambda p: f == "" or p == f or p.startswith(f + "/")
    if "listed" in s:
        return any(inside(p) for p in s["listed"])
    if s["changedOnly"]:
        return any(inside(p) for p in changed)
    return not s["scopes"] or any(inside(sc) or f == sc or f.startswith(sc + "/") for sc in s["scopes"])


for c in corpus["appSelect"]:
    if selects(c["app"], c["selection"], c["changed"]) != c["expected"]:
        failures.append("app select %s" % c["app"]["folder"])
for c in corpus["document"]:
    prefix = c["toRoot"].replace("\\", "/") + "/" if c["toRoot"] else ""
    doc = {"extends": prefix + "tsconfig.json", "compilerOptions": {"allowImportingTsExtensions": True, "noEmit": True, "skipLibCheck": True, "incremental": False, "types": ["bun", "node"], "paths": {k: [prefix + v] for k, v in c["existing"].items()}}, "include": [], "files": [prefix + r for r in c["roots"]]}
    if doc != c["expected"]:
        failures.append("document")
DIAG = re.compile(r"^([^\s(][^\n]*?)\((\d+),(\d+)\): error (TS\d+): ")
GLOBAL = re.compile(r"^error (TS\d+): ")
total, files, codes = 0, set(), {}
syntax = {}
for line in corpus["log"]["text"].split("\n"):
    m = DIAG.match(line)
    code = m.group(4) if m else (GLOBAL.match(line).group(1) if GLOBAL.match(line) else None)
    if code is None:
        continue
    total += 1
    codes[code] = codes.get(code, 0) + 1
    if m:
        files.add(m.group(1))
        s = syntax.setdefault(m.group(1), {"file": m.group(1), "count": 0, "first": line})
        s["count"] += 1
if {"total": total, "files": len(files), "byCode": codes} != corpus["log"]["summary"] or sorted(syntax.values(), key=lambda s: s["file"]) != corpus["log"]["syntax"]:
    failures.append("log")
lines = [l.strip() for l in corpus["rootList"]["text"].replace("/ROOT/", "/").split("\n")]
got = []
for l in lines:
    if l and not l.startswith("#"):
        l = l.lstrip("/").replace("\\", "/")
        if l not in got:
            got.append(l)
if got != corpus["rootList"]["expected"]:
    failures.append("root list")

print(json.dumps({"failures": failures}))
sys.exit(1 if failures else 0)

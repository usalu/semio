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

PLAY = "✏️s/🧑‍💻dev/🧩️catalog/🤖️generated/🚀️playgrounds.json"
REG = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🎮️registry/🦀️.rs"
REG_SCHEMA = "🧰️framework/🛍️products/🦑️repo/🔨️modules/🎛️dashboard/🧬️schema/🎮️registry/🔣️.json"
ROOT_MANIFEST = "📋️project.json"
ROOTS = ["✏️s/🔌️plugins", "🌎️hub/🧩️compositions"]
EXT = "🧩️extensions"
OWNERSHIP = {"🧫️fixtures", "🧪️tests"}
GATES = [{"id": "interactivity", "check": ["interactivity"]}, {"id": "interactivity-tool-jobs", "check": ["interactivity", "tool-jobs"]}, {"id": "interactivity-apps", "check": ["interactivity", "apps"]}, {"id": "interactivity-apps-actions", "check": ["interactivity", "apps", "--actions"]}, {"id": "dependencies", "check": ["dependencies"]}, {"id": "dependencies-literal-external", "check": ["dependencies", "literal-external"]}, {"id": "composed-child-refs", "check": ["composed-child-refs"]}]


def js_obj(x):
    if isinstance(x, dict):
        return x
    if isinstance(x, list):
        return {}
    return None


def dget(o, k):
    return o.get(k) if isinstance(o, dict) else None


def trimmed(x):
    return x.strip() if isinstance(x, str) else ""


def isnum(x):
    return isinstance(x, (int, float)) and not isinstance(x, bool)


def label_exact(v):
    if not isinstance(v, dict) or "default" in v:
        return False
    loc = [e for e in (v.get("native"), v.get("reuse")) if isinstance(e, (dict, list))]
    return bool(loc) and all(isinstance(e, dict) and isinstance(e.get("en"), str) and e["en"] != "" and isinstance(e.get("de"), str) and e["de"] != "" and "default" not in e for e in loc)


def bundle_id(ext):
    m = re.search(r'ExtensionBundle::new\(\s*([A-Za-z_][A-Za-z0-9_]*|"[^"]*")', ext)
    if not m:
        return ""
    arg = m.group(1)
    if arg.startswith('"'):
        return arg[1:-1].strip()
    c = re.search(r'const\s+%s\s*:\s*&str\s*=\s*"([^"]+)"' % arg, ext)
    return c.group(1).strip() if c else ""


def descriptor(file, source, ext=""):
    fails = []
    try:
        d = json.loads(source)
    except ValueError:
        return {"failures": [file + ": descriptor is not valid JSON"]}
    is_ext = ("/" + EXT + "/") in file
    role = "extension" if is_ext else "plugin"
    v = dget(d, "descriptorVersion")
    if not (isnum(v) and v == 1):
        fails.append(file + ": descriptorVersion must be 1")
    if dget(d, "role") != role:
        fails.append(file + ": role must be " + role)
    man = js_obj(dget(d, "manifest"))
    if man is None:
        return {"failures": fails + [file + ": manifest is missing"]}
    apps = dget(man, "apps")
    if not isinstance(apps, list):
        return {"failures": fails + [file + ": manifest.apps is missing"]}
    if is_ext:
        pid = bundle_id(ext)
        pm = re.search(r'\.extends\(\s*"([^"]+)"\s*\)', ext)
        parent = pm.group(1) if pm else ""
        if pid == "":
            fails.append(file + ": extension source declares no resolvable ExtensionBundle::new(<pluginId>, …)")
        if parent == "":
            fails.append(file + ": extension source has no parent .extends(...) activation")
        if len(apps) != 0:
            fails.append(file + ": extension descriptor must delegate app activation to its parent instead of duplicating apps")
        row = {"file": file, "kind": "extension", "pluginId": pid}
        if parent:
            row["parentPluginId"] = parent
        row["appIds"] = []
        row["actions"] = []
        return {"row": row, "failures": fails}
    pid = trimmed(dget(man, "pluginId"))
    if pid == "" or pid == "empty":
        fails.append(file + ": pluginId must be a non-placeholder owned identifier")
    if len(apps) == 0:
        fails.append(file + ": manifest.apps is empty and has no parent activation evidence")
    if len(apps) > 64:
        fails.append("%s: %d apps exceed fixed capacity 64" % (file, len(apps)))
    app_ids, actions = [], []
    for index, av in enumerate(apps[:64]):
        app = js_obj(av)
        if app is None:
            fails.append("%s: app %d is not an object" % (file, index))
            continue
        aid = trimmed(app.get("id"))
        if aid == "":
            fails.append("%s: app %d has no owned id" % (file, index))
        else:
            app_ids.append(aid)
        an = aid or str(index)
        if not label_exact(app.get("label")):
            fails.append("%s: app %s lacks equivalent en/de labels or declares a default language" % (file, aid or index))
        windows = app.get("windowKinds") if isinstance(app.get("windowKinds"), list) else []
        count = 0
        for wi, wv in enumerate(windows):
            win = js_obj(wv)
            if win is None:
                fails.append("%s: app %s window %d is not an object" % (file, an, wi))
                continue
            wid = trimmed(win.get("id"))
            wn = wid or str(wi)
            wacts = win.get("actions") if isinstance(win.get("actions"), list) else []
            count += len(wacts)
            for ai, acv in enumerate(wacts[:512]):
                act = js_obj(acv)
                if act is None:
                    fails.append("%s: app %s window %s action %d is not an object" % (file, an, wn, ai))
                    continue
                acid = trimmed(act.get("id"))
                if acid == "":
                    fails.append("%s: app %s window %s action %d has no owned id" % (file, an, wn, ai))
                if not label_exact(act.get("label")):
                    fails.append("%s: action %s/%s/%s lacks equivalent en/de labels or declares a default language" % (file, an, wn, acid or ai))
                sem = js_obj(act.get("semantics"))
                ex = js_obj(sem.get("execution")) if sem is not None else None
                disp = ex.get("interactiveJob") if ex is not None and isinstance(ex.get("interactiveJob"), str) else None
                entry = {"appId": aid, "windowId": wid, "actionId": acid}
                if disp:
                    entry["disposition"] = disp
                actions.append(entry)
        if count > 512:
            fails.append("%s: app %s has %d action rows exceeding fixed capacity 512" % (file, an, count))
    return {"row": {"file": file, "kind": "app", "pluginId": pid, "appIds": app_ids, "actions": actions}, "failures": fails}


def counts(file, source):
    r = descriptor(file, source)
    row = r.get("row")
    return {"failures": r["failures"], "appIds": len(row["appIds"]) if row else None, "actions": len(row["actions"]) if row else None}


def disposition(descs):
    return [d["file"] + ": %s/%s/%s has interactiveJob=%s" % (a["appId"], a["windowId"], a["actionId"], json.dumps(a.get("disposition", "missing"), ensure_ascii=False)) for d in descs for a in d["actions"] if a.get("disposition") != "migrated"]


def owner(file):
    return "/".join(file.split("/")[:3])


def production(descs, commands, routes):
    shared = set(routes)
    owned = set(owner(c["file"]) + "\0" + c["id"] for c in commands)
    return [d["file"] + ": %s/%s/%s declares migrated without an accepted owner-local production command or shared reserved route" % (a["appId"], a["windowId"], a["actionId"]) for d in descs for a in d["actions"] if a.get("disposition") == "migrated" and a["actionId"] not in shared and owner(d["file"]) + "\0" + a["actionId"] not in owned]


def playgrounds(source):
    fails = []
    try:
        cat = json.loads(source)
    except ValueError:
        return {"rows": [], "failures": [PLAY + ": invalid JSON"]}
    if not isinstance(cat, list):
        return {"rows": [], "failures": [PLAY + ": catalog is not a list"]}
    rows, seen = [], set()
    for v in cat:
        row = js_obj(v) or {}
        ports = js_obj(row.get("ports")) or {}
        variant = row.get("variant") if isinstance(row.get("variant"), str) else ""
        pid = row.get("pluginId") if isinstance(row.get("pluginId"), str) else ""
        react = ports["react"] if isnum(ports.get("react")) else 0
        wgpu = ports["wgpu"] if isnum(ports.get("wgpu")) else 0
        if variant == "" or pid == "":
            fails.append(PLAY + ": malformed playground row")
            continue
        if variant in seen:
            fails.append(PLAY + ": duplicate variant " + json.dumps(variant, ensure_ascii=False))
        seen.add(variant)
        out = {"variant": variant, "pluginId": pid, "react": react, "wgpu": wgpu}
        if isinstance(row.get("app"), str) and row["app"] != "":
            out["appId"] = row["app"]
        rows.append(out)
    if not rows:
        fails.append(PLAY + ": no playground rows discovered")
    return {"rows": rows, "failures": fails}


def renderers(source):
    m = re.search(r"\bconst\s+RENDERERS\s*:\s*&\[&str\]\s*=\s*&\[([^\]]*)\]", source)
    names = re.findall(r'"([^"]+)"', m.group(1)) if m else []
    if not names:
        return {"renderers": names, "failures": [REG + ": the RENDERERS axis of the playground commands is not readable"]}
    react = len([n for n in names if n == "react"])
    native = len([n for n in names if n.endswith("native")])
    wasm = len([n for n in names if n.endswith("wasm")])
    ok = react == 1 and native == 1 and wasm == 1 and len(names) == 3
    return {"renderers": names, "failures": [] if ok else [REG + ": RENDERERS must offer exactly one React, one Wasm and one native surface, found " + ", ".join(names)]}


def surfaces(pl, rs):
    return [r for r in rs if (pl["react"] if r == "react" else pl["wgpu"]) > 0]


def coverage(descs, pls, rs):
    complete = set(p["variant"] for p in pls if len(surfaces(p, rs)) == len(rs))
    neutral = lambda i: re.sub(r"#(?:editor|viewer)\Z", "#surface", i)
    out = []
    for d in [x for x in descs if x["kind"] == "app"]:
        for aid in d["appIds"]:
            cands = [p for p in pls if p["pluginId"] == d["pluginId"] and (not p.get("appId") or neutral(p["appId"]) == neutral(aid))]
            if not any(c["variant"] in complete for c in cands):
                out.append(d["file"] + ": " + aid + " has no owner-qualified playground command with React + WGPU Wasm + WGPU native surfaces")
    return out


def rec(x):
    return x if isinstance(x, dict) else {}


def gates(msrc, ssrc, gs=None):
    gs = GATES if gs is None else gs
    fails = []
    try:
        man = rec(json.loads(msrc))
    except ValueError:
        return [ROOT_MANIFEST + ": invalid JSON"]
    try:
        sch = rec(json.loads(ssrc))
    except ValueError:
        return [REG_SCHEMA + ": invalid JSON"]
    param = rec(rec(rec(sch.get("$defs")).get("Parameter")).get("properties"))
    kinds = rec(param.get("kind")).get("enum")
    if not isinstance(kinds, list) or "text" not in kinds:
        fails.append(REG_SCHEMA + ": the Parameter schema declares no text kind")
    if "valuePositional" not in param:
        fails.append(REG_SCHEMA + ": the Parameter schema declares no valuePositional")
    if man.get("name") != "workspace":
        fails.append(ROOT_MANIFEST + ": the root project must be named workspace")
    verify = rec(rec(man.get("targets")).get("verify"))
    if len(verify) == 0:
        fails.append(ROOT_MANIFEST + ": target workspace:verify is missing, so no verification gate is a dashboard command")
    opts = rec(verify.get("options"))
    if not isinstance(opts.get("command"), str) or not opts["command"].endswith("📜️script.ts verify") or opts.get("forwardAllArgs") is not True:
        fails.append(ROOT_MANIFEST + ": workspace:verify must forward free arguments to 📜️script.ts verify")
    params = rec(rec(rec(verify.get("metadata")).get("semio")).get("dashboard")).get("parameters")
    pos = [rec(p) for p in (params if isinstance(params, list) else [])]
    pos = [p for p in pos if p.get("kind") == "text" and p.get("valuePositional") is True]
    if len(pos) != 1:
        fails.append(ROOT_MANIFEST + ": workspace:verify must declare exactly one positional text parameter in metadata.semio.dashboard, found %d" % len(pos))
    seen = set()
    for g in gs:
        key = " ".join(g["check"])
        if len(g["check"]) == 0 or any(w == "" or re.search(r"\s", w) for w in g["check"]):
            fails.append("gate %s: check must be a list of non-empty words" % g["id"])
        if key in seen:
            fails.append("gate %s: duplicate check %s" % (g["id"], json.dumps(key, ensure_ascii=False)))
        seen.add(key)
    return fails


def coordinate(path):
    parts = path.split("/")
    if len(parts) == 4:
        return True
    return len(parts) == 6 and parts[3] == EXT and parts[4] not in OWNERSHIP


def discovery(files):
    texts = {p: ("\n".join(l) + "\n" if l else "") for p, l in files.items()}
    read = lambda p: texts.get(p, "")
    skip = {"compose", "node_modules", ".git", ".🧬️semio", "target", "dist", "build", "coverage", "🤖️generated", ".claude", "vendor", ".venv", ".turbo", ".nx", ".cache", ".storybook", "storybook-static"}
    found = set()
    for root in ROOTS:
        for p in texts:
            if not p.startswith(root + "/"):
                continue
            rest = p[len(root) + 1:].split("/")
            if any(d in skip or d.startswith(".") for d in rest[:-1]):
                continue
            if rest[-1] == "🔣️.json" and coordinate(p):
                found.add(p)
    fl = jssort(list(found))
    fails = []
    if not fl:
        fails.append("no plugin descriptors discovered")
    if len(fl) > 256:
        fails.append("%d descriptors exceed fixed capacity 256" % len(fl))
    descs, plugin_owners, app_owners = [], {}, {}
    for f in fl[:256]:
        ext = read(f.rsplit("/", 1)[0] + "/🦀️.rs") if ("/" + EXT + "/") in f else ""
        r = descriptor(f, read(f), ext)
        fails += r["failures"]
        if "row" not in r:
            continue
        row = r["row"]
        descs.append(row)
        plugin_owners.setdefault(row["pluginId"], []).append(f)
        for i in row["appIds"]:
            app_owners.setdefault(i, []).append(f)
    for pid, owners in plugin_owners.items():
        if pid != "" and len(owners) > 1:
            fails.append("pluginId %s is ambiguous across %s" % (json.dumps(pid, ensure_ascii=False), ", ".join(owners)))
    app_plugins = set(d["pluginId"] for d in descs if d["kind"] == "app" and d["appIds"])
    for e in [d for d in descs if d["kind"] == "extension"]:
        if not e.get("parentPluginId") or e["parentPluginId"] not in app_plugins:
            fails.append("%s: parent %s has no discovered app activation" % (e["file"], json.dumps(e.get("parentPluginId", ""), ensure_ascii=False)))
    dups = sorted(({"id": i, "files": o} for i, o in app_owners.items() if len(o) > 1), key=lambda x: x["id"])
    gf = gates(read(ROOT_MANIFEST), read(REG_SCHEMA))
    fails += gf
    ax = renderers(read(REG))
    fails += ax["failures"]
    pg = playgrounds(read(PLAY))
    fails += pg["failures"]
    sf = coverage(descs, pg["rows"], ax["renderers"])
    fails += sf
    app_count = sum(len(d["appIds"]) for d in descs)
    return {
        "descriptors": sorted(descs, key=lambda d: d["file"].encode("utf-16-be")),
        "appCount": app_count,
        "surfaceCount": sum(len(surfaces(p, ax["renderers"])) for p in pg["rows"]),
        "actionCount": sum(len(d["actions"]) for d in descs),
        "migratedActionCount": sum(len([a for a in d["actions"] if a.get("disposition") == "migrated"]) for d in descs),
        "missingActionCount": sum(len([a for a in d["actions"] if a.get("disposition") != "migrated"]) for d in descs),
        "surfaceCoveredAppCount": app_count - len(sf),
        "surfaceMissingAppCount": len(sf),
        "duplicateAppIds": dups,
        "playgrounds": len(pg["rows"]),
        "renderers": ax["renderers"],
        "gates": len(GATES) if not gf else 0,
        "failures": jssort(list(set(fails))),
    }


def run(c):
    a, r = c.get("args", []), c["rule"]
    if r == "descriptor":
        return descriptor(*a)
    if r == "descriptorCounts":
        return counts(*a)
    if r == "disposition":
        return disposition(a[0])
    if r == "production":
        return production(*a)
    if r == "playgrounds":
        return playgrounds(a[0])
    if r == "renderers":
        return renderers(a[0])
    if r == "surfaces":
        return surfaces(*a)
    if r == "coverage":
        return coverage(*a)
    if r == "gates":
        return gates(*a)
    if r == "discovery":
        return discovery(c["files"])
    if r == "discoverySummary":
        out = discovery(c["files"])
        out["descriptors"] = len(out["descriptors"])
        return out
    if r == "constants":
        return {"apps": 64, "actions": 512, "gates": GATES}
    raise KeyError(r)


cases = json.load(open(sys.argv[1], encoding="utf8"))["cases"]
output({c["id"]: run(c) for c in cases if c.get("oracle") is not False})

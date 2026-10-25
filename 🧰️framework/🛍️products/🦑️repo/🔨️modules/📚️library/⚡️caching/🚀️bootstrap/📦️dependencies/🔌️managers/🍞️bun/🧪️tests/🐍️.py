#!/usr/bin/env python3
"""🐍️ Independent model of the Bun lock check: reads the same corpus and shares no code with the Bun test (stdlib only).

usage: 🐍️.py CORPUS.json      (exit 0 when every verdict holds)"""
import json, re, sys

corpus = json.load(open(sys.argv[1], encoding="utf8"))
TABLES = ["dependencies", "devDependencies", "optionalDependencies", "peerDependencies"]


def jsonc(text):
    return json.loads(re.sub(r",(\s*[}\]])", r"\1", text))


def locked_names(packages):
    names = set()
    for key in (packages or {}):
        parts = key.split("/")
        last = parts.pop()
        names.add(parts.pop() + "/" + last if parts and parts[-1].startswith("@") else last)
    return names


def used_names(packages, manifests):
    """Names some manifest declares (members included) plus everything reachable through the locked packages' own dependency lists."""
    edges, todo, reached = {}, [], set()
    for row in (packages or {}).values():
        spec = row[0] if isinstance(row, list) and row and isinstance(row[0], str) else None
        at = spec.rfind("@") if spec else -1
        if at <= 0:
            continue
        meta = row[2] if len(row) > 2 and isinstance(row[2], dict) else {}
        for section in ("dependencies", "optionalDependencies", "peerDependencies"):
            edges.setdefault(spec[:at], []).extend(meta[section] if isinstance(meta.get(section), dict) else [])
    for _, doc in manifests:
        if isinstance(doc.get("name"), str):
            reached.add(doc["name"])
        for section in TABLES:
            todo.extend(doc[section] if isinstance(doc.get(section), dict) else [])
    while todo:
        name = todo.pop()
        if name not in reached:
            reached.add(name)
            todo.extend(edges.get(name, []))
    return reached


def as_table(value):
    return {k: str(v) for k, v in value.items()} if isinstance(value, dict) else {}


def as_names(value):
    return {n: "" for n in value if isinstance(n, str)} if isinstance(value, list) else {}


def check(text, manifests):
    try:
        lock = jsonc(text)
    except ValueError:
        return [{"kind": "invalid-lockfile", "manifest": "bun.lock", "frozen": True}]
    rows = lock.get("workspaces") if isinstance(lock.get("workspaces"), dict) else {}
    locked = locked_names(lock.get("packages") if isinstance(lock.get("packages"), dict) else {})
    used = used_names(lock.get("packages") if isinstance(lock.get("packages"), dict) else {}, manifests)
    out, seen = [], set()

    def emit(kind, manifest, frozen, **fields):
        row = {"kind": kind, "manifest": manifest, **{k: v for k, v in fields.items() if v is not None}, "frozen": frozen}
        out.append(row)

    def table_diff(manifest, section, authored, table, certain):
        for name in set(authored) | set(table):
            a, t = authored.get(name), table.get(name)
            if a is None:
                emit("stale-in-lock", manifest, certain(name, "stale"), section=section, dependency=name, lockVersion=t)
            elif t is None:
                emit("missing-in-lock", manifest, certain(name, "missing"), section=section, dependency=name, manifestVersion=a)
            elif a != t:
                emit("version-mismatch", manifest, certain(name, "version"), section=section, dependency=name, manifestVersion=a, lockVersion=t)

    for path, doc in manifests:
        manifest = path.replace("\\", "/")
        key = "" if manifest == "package.json" else re.sub(r"/package\.json$", "", manifest)
        seen.add(key)
        row = rows.get(key)
        if not isinstance(row, dict):
            emit("workspace-missing", manifest, True)
            continue
        for field in (["name"] if key == "" else ["name", "version"]):
            a = doc.get(field) if isinstance(doc.get(field), str) else None
            t = row.get(field) if isinstance(row.get(field), str) else None
            if a != t:
                emit("identity-mismatch", manifest, field == "name" and key != "", section=field, manifestVersion=a, lockVersion=t)
        for section in TABLES:
            table_diff(manifest, section, as_table(doc.get(section)), as_table(row.get(section)), lambda n, k: (n not in locked) if k == "missing" else (k == "stale" and n not in used))
        if key == "":
            overrides = {**as_table(doc.get("resolutions")), **as_table(doc.get("overrides"))}
            table_diff(manifest, "overrides", overrides, as_table(lock.get("overrides")), lambda n, k: "resolutions" not in doc)
            table_diff(manifest, "patchedDependencies", as_table(doc.get("patchedDependencies")), as_table(lock.get("patchedDependencies")), lambda n, k: False)
            table_diff(manifest, "trustedDependencies", as_names(doc.get("trustedDependencies")), as_names(lock.get("trustedDependencies")), lambda n, k: False)
    for key in rows:
        if key not in seen:
            emit("workspace-stale", "package.json" if key == "" else key + "/package.json", True)
    out.sort(key=lambda r: (r["manifest"].encode("utf-16-be"), r.get("section", "").encode("utf-16-be"), r.get("dependency", "").encode("utf-16-be"), r["kind"].encode("utf-16-be")))
    return out


failures = []
for case in corpus["cases"]:
    if check(case["lock"], case["manifests"]) != case["expected"]:
        failures.append(case["name"])
print(json.dumps({"failures": failures}))
sys.exit(1 if failures else 0)

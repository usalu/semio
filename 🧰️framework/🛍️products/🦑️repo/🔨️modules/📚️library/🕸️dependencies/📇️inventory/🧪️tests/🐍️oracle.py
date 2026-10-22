#!/usr/bin/env python3
"""🐍️ Independent model of the dependency inventory for Go, .NET, classification, the test domain, the composition rule and the Bun lock audit: reads the same corpus and shares no code with the Bun test (stdlib only: json, re, xml.etree).

usage: oracle.py CORPUS.json      (exit 0 when every verdict holds)"""
import json, re, sys
import xml.etree.ElementTree as ET

corpus = json.load(open(sys.argv[1], encoding="utf8"))
failures = []
RT, BU, TR, TO = "production-runtime", "production-build", "test-runner", "repository-tooling"
tax = corpus["taxonomy"]
dirs = [tax[k] for k in ("testsDirName", "testFixturesDirName", "testExamplesDirName", "testOraclesDirName", "testProbeDirName", "testGeneratorDirName")]


def owned(path):
    return path.startswith(tax["testDomainPath"] + "/") or any(p in dirs for p in path.split("/")[:-1])


def go_dirs(work):
    found = []
    inblock = False
    for raw in work.splitlines():
        line = raw.strip()
        if inblock:
            if line == ")":
                inblock = False
            elif line and not line.startswith("//"):
                found.append(line)
        elif re.match(r"use\s*\($", line):
            inblock = True
        else:
            m = re.match(r"use\s+(\S+)$", line)
            if m:
                found.append(m.group(1))
    stripped = {d[2:] if d.startswith("./") else d for d in found}
    return sorted(d for d in stripped if d not in ("", "compose") and not d.startswith("compose/"))


def go_module(text):
    mod, replaces, reqs, block = None, set(), [], None
    for raw in text.splitlines():
        line = raw.strip()
        m = re.match(r"module\s+(\S+)$", line)
        if m:
            mod = m.group(1)
        if re.match(r"(require|replace)\s*\($", line):
            block = line.split()[0]
            continue
        if block and line == ")":
            block = None
            continue
        rq = line if block == "require" else line[len("require "):] if line.startswith("require ") else ""
        m = re.match(r"(\S+)\s+(\S+)(\s*//\s*indirect)?", rq)
        if m:
            reqs.append({"name": m.group(1), "version": m.group(2), "kind": BU if m.group(3) else RT})
        rp = line if block == "replace" else line[len("replace "):] if line.startswith("replace ") else ""
        m = re.match(r"(\S+)(?:\s+v\S+)?\s+=>\s+(\S+)", rp)
        if m and re.match(r"(\.{1,2}[\\/]|[\\/]|[A-Za-z]:[\\/])", m.group(2)):
            replaces.add(m.group(1))
    out = {"localReplaces": sorted(replaces), "requirements": reqs}
    if mod:
        out["module"] = mod
    return out


def merge(records):
    by = {}
    for eco, name, version, kind, user in records:
        kind = TR if owned(user) and kind in (RT, BU) else kind
        e = by.setdefault((eco, name), {"ecosystem": eco, "name": name, "version": version, "kinds": [], "users": [], "productionReachable": False, "declarations": []})
        if kind not in e["kinds"]:
            e["kinds"].append(kind)
        if user not in e["users"]:
            e["users"].append(user)
        d = {"user": user, "version": version, "kind": kind}
        if d not in e["declarations"]:
            e["declarations"].append(d)
    for e in by.values():
        e["declarations"].sort(key=lambda d: (d["user"], d["kind"], d["version"]))
        e["kinds"].sort()
        e["users"].sort()
        e["productionReachable"] = any(k in (RT, BU) for k in e["kinds"])
    return sorted(by.values(), key=lambda e: (e["ecosystem"], e["name"].lower(), e["name"]))


def go_freeze(files):
    work = files.get("go.work", "")
    dirs_ = go_dirs(work)
    internal = set(go_module(work)["localReplaces"])
    for d in dirs_:
        mod = go_module(files.get(d + "/go.mod", ""))
        internal |= set(mod["localReplaces"]) | ({mod["module"]} if "module" in mod else set())
    records = []
    for d in dirs_:
        path = d + "/go.mod"
        for r in go_module(files.get(path, ""))["requirements"]:
            if "." not in r["name"] or r["name"].startswith("semio.tech/") or r["name"] in internal:
                continue
            records.append(("go", r["name"], r["version"], r["kind"], path))
    return merge(records)


def dotnet_freeze(files):
    records = []
    for path in sorted(files):
        if not path.endswith(".csproj") or "compose" in path.split("/"):
            continue
        root = ET.fromstring(files[path])
        refs = [(r.get("Include"), r.get("Version") if r.get("Version") is not None else (r.findtext("Version") or "*")) for r in root.iter("PackageReference")]
        is_test = (root.findtext(".//IsTestProject") or "").strip().lower() == "true" or any(n.startswith(("xunit", "Microsoft.NET.Test", "NUnit")) for n, _ in refs)
        for name, version in refs:
            records.append(("dotnet", name, version, TR if is_test else RT, path))
    return merge(records)


for case in corpus["freeze"]:
    if case["oracle"] != "python":
        continue
    got = go_freeze(case["files"]) if any(p.endswith("go.mod") or p == "go.work" for p in case["files"]) else dotnet_freeze(case["files"])
    if got != case["expected"]:
        failures.append("freeze: " + case["name"])

for case in corpus["goModule"]:
    if go_module(case["content"]) != case["expected"]:
        failures.append("go.mod: " + case["name"])

for case in corpus["composition"]:
    p = case["path"]
    if (p.startswith("compose/") or p.startswith("temp/compose/")) != case["expected"]:
        failures.append("composition: " + p)

for case in corpus["testDomain"]:
    t = case["taxonomy"]
    keys = ("testsDirName", "testFixturesDirName", "testExamplesDirName", "testOraclesDirName", "testProbeDirName", "testGeneratorDirName")
    valid = all(isinstance(t.get(k), str) and t.get(k) for k in keys) and isinstance(t.get("testDomainPath"), str)
    if valid != (case["expected"] is not None) or (valid and case["expected"] != {"directoryNames": [t[k] for k in keys], "domainPath": t["testDomainPath"]}):
        failures.append("testDomain: " + case["name"])

OWNER = corpus["classify"][4]["entry"]["users"][0]
for case in corpus["classify"]:
    e = json.loads(json.dumps(case["entry"]))
    ids = case["oracleIds"]
    if ids is not None:
        e["oracleIds"] = sorted(ids)
        owner = e["name"] == "three" and e["ecosystem"] == "js"
        prod = [d for d in e["declarations"] if not owned(d["user"]) and d["kind"] in (RT, BU) and not (owner and d["user"] == OWNER)]
        if prod:
            e["oracleConflictUsers"] = sorted({d["user"] for d in prod})
        else:
            e["kinds"] = ["test-oracle"]
    if e != case["expected"]:
        failures.append("classify: " + case["name"])

SECTIONS = ["dependencies", "devDependencies", "optionalDependencies", "peerDependencies"]
for case in corpus["lockMismatches"]:
    ws = case["workspaces"] if isinstance(case["workspaces"], dict) else {}
    out = []
    for manifest, value in case["manifests"]:
        norm = manifest.replace("\\", "/")
        key = "" if norm == "package.json" else re.sub(r"/package\.json$", "", norm)
        lock = ws.get(key)
        if not isinstance(lock, dict):
            out.append({"kind": "workspace-missing", "manifest": manifest})
            continue
        for section in SECTIONS:
            mt, lt = value.get(section) or {}, lock.get(section) or {}
            for dep in sorted(set(mt) | set(lt)):
                row = {"dependency": dep, "manifest": manifest, "section": section}
                if dep not in mt:
                    row.update(kind="stale-in-lock", lockVersion=str(lt[dep]))
                elif dep not in lt:
                    row.update(kind="missing-in-lock", manifestVersion=str(mt[dep]))
                elif str(mt[dep]) != str(lt[dep]):
                    row.update(kind="version-mismatch", lockVersion=str(lt[dep]), manifestVersion=str(mt[dep]))
                else:
                    continue
                out.append(row)
    out.sort(key=lambda r: (r["manifest"], r.get("section", ""), r.get("dependency", ""), r["kind"]))
    if out != case["expected"]:
        failures.append("lock: " + case["name"])

print(json.dumps({"failures": failures}))
sys.exit(1 if failures else 0)

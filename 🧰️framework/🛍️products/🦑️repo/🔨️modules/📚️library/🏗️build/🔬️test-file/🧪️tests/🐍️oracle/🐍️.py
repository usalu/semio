"""Independent reading of `repo test-file`: argument parsing and suite routing, written from the documented rules and not from the TypeScript."""
import json
import os
import re
import sys

VALUED = ["--lang"]
PLAIN = ["--json", "--bail", "--dry-run"]
REFUSE = "refusal"


def parse(args):
    seen, flags, files, language, tail, i = set(), set(), [], None, False, 0
    while i < len(args):
        a = args[i]
        i += 1
        if tail or not a.startswith("-") or a == "-":
            files.append(a)
            continue
        if a == "--":
            tail = True
            continue
        name, eq, value = a.partition("=")
        if name not in VALUED and name not in PLAIN:
            return {REFUSE: "option.unknown"}
        if name in seen:
            return {REFUSE: "option.repeated"}
        seen.add(name)
        if name in VALUED:
            if not eq:
                if i >= len(args):
                    return {REFUSE: "option.value"}
                value = args[i]
                i += 1
            if not value or value.startswith("--"):
                return {REFUSE: "option.value"}
            language = value
        elif eq:
            return {REFUSE: "option.unknown"}
        else:
            flags.add(name)
    if language is not None and language not in ("en", "de"):
        return {REFUSE: "language.unknown"}
    if not files:
        return {REFUSE: "usage"}
    unique = list(dict.fromkeys(files))
    return {"files": unique, "json": "--json" in flags, "bail": "--bail" in flags, "dryRun": "--dry-run" in flags, "language": language}


def owners(table, module):
    if module in table["helpers"]:
        return None
    suites = table["suites"]
    chosen = [s for s in suites if "testNamePattern" not in s and module in s.get("files", [])]
    if not chosen:
        depth = -1
        for s in suites:
            for tree in s.get("trees", []):
                prefix = tree.rstrip("/") + "/"
                if not module.startswith(prefix):
                    continue
                if len(prefix) > depth:
                    depth, chosen = len(prefix), [s]
                elif len(prefix) == depth and s not in chosen:
                    chosen.append(s)
    filtered = [s for s in suites if "testNamePattern" in s and module in s.get("files", [])]
    return [s["id"] for s in chosen + filtered]


def route(root, policy, files):
    out = {}
    for file in files:
        hit = None
        for t in policy["tables"]:
            if file.startswith(t["root"] + "/"):
                hit = t
                break
        ids = []
        if hit is not None:
            with open(os.path.join(root, hit["table"]), encoding="utf-8") as handle:
                table = json.load(handle)
            ids = owners(table, file[len(hit["root"]) + 1:])
            if ids is None:
                out[file] = "helper"
                continue
        out[file] = [{"table": hit["id"], "suite": i} for i in ids] if ids else [{"table": None, "suite": policy["plain"]["id"]}]
    return out


def declared_arguments(policy, file, text, existing):
    oracle = policy["oracle"]
    found = re.search("🐍️\\.py " + oracle["command"] + " " + oracle["usage"], text)
    if not found:
        return []
    parts = found.group(1).split("/")
    directory = file.rsplit("/", 1)[0] if "/" in file else ""
    while True:
        candidate = "/".join(filter(None, [directory] + parts))
        if candidate in existing:
            return [oracle["command"], candidate]
        if not directory:
            return {REFUSE: "oracle.fixtures-missing"}
        directory = directory.rsplit("/", 1)[0] if "/" in directory else ""


def owner(file, entries):
    if file in entries:
        return file
    name = file.rsplit("/", 1)[1]
    parent = file.rsplit("/", 1)[0]
    while "/" in parent:
        parent = parent.rsplit("/", 1)[0]
        if parent + "/" + name in entries:
            return parent + "/" + name
    return {REFUSE: "oracle.orphan"}


def main():
    mode = sys.argv[1]
    if mode == "oracle-arguments":
        with open(sys.argv[2], encoding="utf-8") as handle:
            policy = json.load(handle)
        rows = json.load(sys.stdin)
        print(json.dumps([declared_arguments(policy, r["file"], r["text"], set(r["exists"])) for r in rows], ensure_ascii=False))
        return
    if mode == "oracle-owners":
        print(json.dumps([owner(r["file"], set(r["entries"])) for r in json.load(sys.stdin)], ensure_ascii=False))
        return
    if mode == "arguments":
        vectors = json.load(sys.stdin)
        print(json.dumps([parse(v) for v in vectors], ensure_ascii=False))
    elif mode == "routes":
        root, policy_path = sys.argv[2], sys.argv[3]
        with open(policy_path, encoding="utf-8") as handle:
            policy = json.load(handle)
        files = json.load(sys.stdin)
        print(json.dumps(route(root, policy, files), ensure_ascii=False))
    else:
        sys.exit(2)


main()

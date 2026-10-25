import hashlib
import json
import os
import stat
import sys

TIERS = "walk stamp, runtime, installer configuration files, installer inputs, workspace manifests"


def sha(data):
    return hashlib.sha256(data).hexdigest()


def resolve(location, workspace, store):
    kind, rest = location[0], location[2:]
    if location[1] != ":" or not rest:
        raise ValueError("closed location required")
    return {"w": lambda: os.path.join(workspace, rest), "s": lambda: os.path.join(store, rest), "a": lambda: rest}[kind]()


def lstat(path):
    try:
        return os.lstat(path)
    except FileNotFoundError:
        return None
    except NotADirectoryError:
        return None


def configuration(row, workspace, store):
    path = resolve(row["path"], workspace, store)
    try:
        facts = os.stat(path)
    except (FileNotFoundError, NotADirectoryError):
        return {"kind": "missing", "bytes": 0, "sha256": None}
    if not stat.S_ISREG(facts.st_mode) or facts.st_size > 1048576:
        return {"kind": "other", "bytes": 0, "sha256": None}
    try:
        data = open(path, "rb").read()
    except OSError:
        return {"kind": "other", "bytes": 0, "sha256": None}
    return {"kind": "file", "bytes": len(data), "sha256": sha(data)}


def verify(receipt, workspace, store, stamp, runtime, maximum_source_bytes):
    if receipt["walkStamp"] != stamp:
        return {"status": "mismatch", "class": "protocol-changed", "detail": receipt["walkStamp"]}
    for field in ["manager", "revision", "node", "platform", "arch", "sha256"]:
        if runtime[field] != receipt["runtime"][field]:
            return {"status": "mismatch", "class": "runtime-changed", "detail": field}
    for row in sorted(receipt["producer"]["files"], key=lambda item: item["path"].encode("utf-8")):
        live = configuration(row, workspace, store)
        if (live["kind"], live["bytes"], live["sha256"]) != (row["kind"], row["bytes"], row["sha256"]):
            return {"status": "mismatch", "class": "configuration-changed", "path": row["path"]}
    for group in receipt["inputs"]:
        for source in group["sources"]:
            path = resolve(source["path"], workspace, store)
            facts = lstat(path)
            if source["kind"] == "missing":
                if facts:
                    return {"status": "mismatch", "class": "input-changed", "path": source["path"], "detail": "appeared"}
                continue
            wanted = stat.S_ISREG if source["kind"] == "file" else stat.S_ISDIR
            if not facts or stat.S_ISLNK(facts.st_mode) or not wanted(facts.st_mode) or str(facts.st_dev) != source["dev"] or str(facts.st_ino) != source["ino"] or facts.st_mode != source["mode"]:
                return {"status": "mismatch", "class": "input-changed", "path": source["path"], "detail": "identity"}
            if source["kind"] == "file" and (facts.st_size > maximum_source_bytes or sha(open(path, "rb").read()) != source["sha256"]):
                return {"status": "mismatch", "class": "input-changed", "path": source["path"], "detail": "bytes"}
    for text in receipt["membership"]["texts"]:
        path = os.path.join(workspace, text["path"])
        try:
            data = open(path, "rb").read()
            data.decode("utf-8")
        except (OSError, UnicodeDecodeError):
            return {"status": "mismatch", "class": "manifest-changed", "path": text["path"], "detail": "unreadable"}
        if sha(data) != text["sha256"]:
            return {"status": "mismatch", "class": "manifest-changed", "path": text["path"], "detail": "bytes"}
    return {"status": "match"}


def render(texts, language, row, protocol):
    table = texts[language]
    template = table.get(row["class"] + "." + row.get("detail", "")) or table[row["class"]]
    values = {"path": row.get("path", ""), "detail": row.get("detail", ""), "expected": protocol["walkStamp"], "limit": protocol["maximumRelist"]}
    for key, value in values.items():
        template = template.replace("{" + key + "}", str(value))
    return template


def main():
    command = sys.argv[1]
    if command == "verify":
        receipt = json.load(open(sys.argv[2], encoding="utf-8"))
        protocol = json.load(open(sys.argv[6], encoding="utf-8"))["$defs"]["Protocol"]["const"]
        print(json.dumps(verify(receipt, sys.argv[3], sys.argv[4], protocol["walkStamp"], json.loads(sys.argv[5]), protocol["maximumSourceFileBytes"]), ensure_ascii=False))
    elif command == "render":
        protocol = json.load(open(sys.argv[2], encoding="utf-8"))["$defs"]["Protocol"]["const"]
        rows = json.loads(sys.argv[3])
        print(json.dumps([{language: render(protocol["texts"], language, row, protocol) for language in protocol["languages"]} for row in rows], ensure_ascii=False))
    else:
        raise SystemExit("usage: keying.oracle.py verify RECEIPT WORKSPACE STORE RUNTIME_JSON SCHEMA | render SCHEMA ROWS_JSON (" + TIERS + ")")


main()

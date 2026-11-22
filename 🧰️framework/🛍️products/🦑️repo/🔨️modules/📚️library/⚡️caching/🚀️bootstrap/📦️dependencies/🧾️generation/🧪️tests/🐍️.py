import hashlib
import json
import os
import stat
import sys

PROFILE = json.load(open(os.path.join(os.path.dirname(os.path.abspath(__file__)), "../🪪️profile/🧬️schema/🔣️.json"), encoding="utf-8"))["$defs"]["Policy"]["const"]


def profile(path, data):
    if path.split("/")[-1] != PROFILE["fileName"] or len(data) > PROFILE["maximumBytes"]:
        return None
    try:
        value = json.loads(data.decode("utf-8"))
    except ValueError:
        return None
    if not isinstance(value, dict):
        return None
    kept = {k: v for k, v in value.items() if k not in PROFILE["excludedTopLevelFields"]}
    return hashlib.sha256(json.dumps(kept, sort_keys=True, separators=(",", ":"), ensure_ascii=False).encode()).hexdigest()


TIERS = "walk stamp, receipt scoping, runtime, installer configuration files, installer inputs, workspace manifests"


def owner_of(directories, path):
    depth = lambda directory: 0 if directory == "." else len(directory.split("/"))
    best = "."
    for directory in directories:
        if (directory == "." or path == directory or path.startswith(directory + "/")) and depth(directory) > depth(best):
            best = directory
    return best


def text_owner(directories, path):
    return owner_of(directories, path.rsplit("/", 1)[0] if "/" in path else ".")


def scoped_texts(texts, required):
    return [text["path"] for text in texts if text["owner"] in set(required)]


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


def verify(receipt, workspace, store, stamp, runtime, maximum_source_bytes, environment):
    if receipt["walkStamp"] != stamp:
        return {"status": "mismatch", "class": "protocol-changed", "detail": receipt["walkStamp"]}
    owned = set(receipt["required"])
    if any(group["scope"] not in owned for group in receipt["inputs"]) or any(text["owner"] not in owned for text in receipt["membership"]["texts"]):
        return {"status": "mismatch", "class": "receipt-invalid"}
    for field in ["manager", "revision", "node", "platform", "arch", "sha256"]:
        if runtime[field] != receipt["runtime"][field]:
            return {"status": "mismatch", "class": "runtime-changed", "detail": field}
    for row in sorted(receipt["producer"]["files"], key=lambda item: item["path"].encode("utf-8")):
        live = configuration(row, workspace, store)
        if (live["kind"], live["bytes"], live["sha256"]) != (row["kind"], row["bytes"], row["sha256"]):
            return {"status": "mismatch", "class": "configuration-changed", "path": row["path"]}
    policy = environment["policy"]
    recorded = {row["name"]: row["sha256"] for row in receipt["producer"]["environment"]}
    live = {
        name: sha(value.encode("utf-8"))
        for name, value in environment["env"].items()
        if name.upper() not in policy["environmentIgnored"]
        and (name.upper() in policy["environmentNames"] or name.upper().startswith(tuple(policy["environmentPrefixes"])))
    }
    for name in sorted(live, key=str.encode):
        if live[name] != recorded.get(name):
            return {"status": "mismatch", "class": "configuration-changed", "path": "env:" + name}
    for name in sorted(recorded.keys() - live.keys(), key=str.encode):
        return {"status": "mismatch", "class": "configuration-changed", "path": "env:" + name}
    for group in receipt["inputs"]:
        for source in group["sources"]:
            path = resolve(source["path"], workspace, store)
            facts = lstat(path)
            if source["kind"] == "missing":
                if facts:
                    return {"status": "mismatch", "class": "input-changed", "path": source["path"], "detail": "appeared"}
                continue
            wanted = stat.S_ISREG if source["kind"] == "file" else stat.S_ISDIR
            moved = not facts or str(facts.st_dev) != source["dev"] or str(facts.st_ino) != source["ino"] or facts.st_mode != source["mode"]
            if not facts or stat.S_ISLNK(facts.st_mode) or not wanted(facts.st_mode) or moved and source["profileSha256"] is None:
                return {"status": "mismatch", "class": "input-changed", "path": source["path"], "detail": "identity"}
            if source["kind"] == "file":
                if facts.st_size > maximum_source_bytes:
                    return {"status": "mismatch", "class": "input-changed", "path": source["path"], "detail": "bytes"}
                data = open(path, "rb").read()
                if sha(data) != source["sha256"] and (source["profileSha256"] is None or profile(path, data) != source["profileSha256"]):
                    return {"status": "mismatch", "class": "input-changed", "path": source["path"], "detail": "bytes"}
    for text in receipt["membership"]["texts"]:
        path = os.path.join(workspace, text["path"])
        try:
            data = open(path, "rb").read()
            data.decode("utf-8")
        except (OSError, UnicodeDecodeError):
            return {"status": "mismatch", "class": "manifest-changed", "path": text["path"], "detail": "unreadable"}
        if sha(data) != text["sha256"] and (text["profileSha256"] is None or profile(text["path"], data) != text["profileSha256"]):
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
        print(json.dumps(verify(receipt, sys.argv[3], sys.argv[4], protocol["walkStamp"], json.loads(sys.argv[5]), protocol["maximumSourceFileBytes"], json.loads(sys.argv[7])), ensure_ascii=False))
    elif command == "owners":
        directories = json.loads(sys.argv[2])
        print(json.dumps({path: text_owner(directories, path) for path in json.loads(sys.argv[3])}, ensure_ascii=False))
    elif command == "scoped":
        print(json.dumps(scoped_texts(json.loads(sys.argv[2]), json.loads(sys.argv[3])), ensure_ascii=False))
    elif command == "render":
        protocol = json.load(open(sys.argv[2], encoding="utf-8"))["$defs"]["Protocol"]["const"]
        rows = json.loads(sys.argv[3])
        print(json.dumps([{language: render(protocol["texts"], language, row, protocol) for language in protocol["languages"]} for row in rows], ensure_ascii=False))
    else:
        raise SystemExit("usage: keying.oracle.py verify RECEIPT WORKSPACE STORE RUNTIME_JSON SCHEMA ENVIRONMENT_JSON | owners DIRECTORIES_JSON PATHS_JSON | scoped TEXTS_JSON REQUIRED_JSON | render SCHEMA ROWS_JSON (" + TIERS + ")")


main()

import hashlib
import json
import os
import re
import stat
import subprocess
import sys

OPAQUE = {"🗑️generated", "🤖️generated", "node_modules", "target", "dist", "📤️dist", "build", "storybook-static", "temp", "coverage", "__pycache__", "🔌️plugin-modules", "compose"}


def canonical(value):
    if isinstance(value, dict):
        return "{" + ",".join(json.dumps(key, ensure_ascii=False) + ":" + canonical(value[key]) for key in sorted(value, key=lambda k: k.encode("utf-16-be"))) + "}"
    if isinstance(value, list):
        return "[" + ",".join(canonical(item) for item in value) + "]"
    return json.dumps(value, ensure_ascii=False, separators=(",", ":"))


def sha(text):
    return hashlib.sha256(text.encode("utf-8")).hexdigest()


def kind_of(mode):
    return "link" if stat.S_ISLNK(mode) else "directory" if stat.S_ISDIR(mode) else "file" if stat.S_ISREG(mode) else "foreign"


def children(path):
    rows = []
    for name in sorted(os.listdir(path), key=lambda n: n.encode("utf-8")):
        rows.append([name, kind_of(os.lstat(os.path.join(path, name)).st_mode)])
    return rows


def digest(rows, units=32):
    return sha(json.dumps(rows, ensure_ascii=False, separators=(",", ":")))[:units]


def ledger(root, start_ms):
    threshold = (int(start_ms) - 5000) * 1000000
    rows = []

    def walk(path, portable):
        facts = os.lstat(path)
        kids = children(path)
        count = lambda kind: sum(1 for row in kids if row[1] == kind)
        rows.append([portable, str(facts.st_dev), str(facts.st_ino), str(facts.st_mode), str(facts.st_nlink), str(facts.st_size), str(facts.st_mtime_ns), str(facts.st_ctime_ns), 1 if facts.st_mtime_ns < threshold and facts.st_ctime_ns < threshold else 0, digest(kids), count("directory"), count("file"), count("link")])
        for name, kind in kids:
            if kind == "directory":
                walk(os.path.join(path, name), name if portable == "." else portable + "/" + name)

    walk(root, ".")
    rows.sort(key=lambda row: row[0].encode("utf-8"))
    entries = 1 + sum(row[10] + row[11] + row[12] for row in rows)
    return {"rows": rows, "entries": entries, "directories": len(rows), "files": sum(row[11] for row in rows), "links": sum(row[12] for row in rows)}


def verify(root, receipt_device, rows):
    for row in rows:
        path = root if row["path"] == "." else os.path.join(root, *row["path"].split("/"))
        try:
            facts = os.lstat(path)
        except OSError:
            return {"status": "mismatch", "class": "directory-changed", "path": row["path"]}
        if not stat.S_ISDIR(facts.st_mode):
            return {"status": "mismatch", "class": "directory-changed", "path": row["path"]}
        if str(facts.st_dev) != receipt_device:
            return {"status": "mismatch", "class": "device-changed", "path": row["path"]}
        if [str(facts.st_dev), str(facts.st_ino), str(facts.st_mode)] != row["stamp"][:3]:
            return {"status": "mismatch", "class": "directory-changed", "path": row["path"]}
        if digest(children(path)) != row["children"]:
            return {"status": "mismatch", "class": "listing-changed", "path": row["path"]}
    return {"status": "match"}


def retryable(row, policy):
    if row["aborted"] or row["unresolved"] or row["attempt"] + 1 >= policy["maximumAttempts"]:
        return False
    lower = row["message"].lower()
    if any(word in lower for word in policy["excluded"]):
        return False
    return any(re.search(pattern, row["message"]) for pattern in policy["patterns"])


def replay(row, settle=5000):
    threshold = (int(row["startedAtMs"]) - settle) * 1000000
    for obs in row["observations"]:
        if obs["full"] and obs["stamp"] is not None:
            parts = json.loads(obs["stamp"])
            if int(parts[5]) >= threshold or int(parts[6]) >= threshold:
                return {"status": "deferred", "path": obs["path"]}
    for obs in row["observations"]:
        live = row["live"].get(obs["path"])
        answer = None if live is None else live["stamp"] if obs["full"] else live["anchor"]
        expected = obs["stamp"] if obs["full"] else obs["anchor"]
        if answer != expected or (obs["kind"] == "link" and live is not None and live.get("target") != obs["target"]):
            return {"status": "mismatch", "path": obs["path"]}
    return {"status": "match"}


def expect(path):
    fixture = json.load(open(path, encoding="utf-8"))
    out = {"retry": {}, "replay": {}, "lock": {}, "singleLink": {}}
    for row in fixture["retry"]:
        out["retry"][row["id"]] = retryable(row, fixture["retryPolicy"])
    for row in fixture["replay"]:
        out["replay"][row["id"]] = replay(row)
    for row in fixture["lock"]:
        keys = ["scope", "generation", "lockSha256", "recipeSha256", "runtimeSha256"]
        out["lock"][row["id"]] = all(row["receipt"][key] == row["current"][key] for key in keys)
    for row in fixture["singleLink"]:
        out["singleLink"][row["id"]] = row["nlink"] != 1
    return out


def ownership(root):
    rows = {}
    for current, names, _files in os.walk(root, followlinks=False):
        names[:] = [name for name in names if name not in OPAQUE and not name.startswith(".")]
        facts = os.lstat(current)
        entries = []
        for name in sorted(os.listdir(current), key=lambda n: n.encode("utf-8")):
            kind = kind_of(os.lstat(os.path.join(current, name)).st_mode)
            if name == "package.json" or (not name.startswith(".") and name not in OPAQUE and kind in ("link", "directory")):
                entries.append([name, kind])
        rows[os.path.relpath(current, root) if current != root else "."] = {"stamp": ":".join(str(v) for v in (facts.st_dev, facts.st_ino, facts.st_mode, facts.st_mtime_ns, facts.st_ctime_ns)), "entries": entries}
    return rows


def manifests(root, names):
    listing = subprocess.run(["git", "-C", root, "ls-files", "-z", "--", *[":(glob)**/" + name for name in names]], capture_output=True, check=True).stdout
    offenders, count = [], 0
    for raw in listing.split(b"\0"):
        if not raw:
            continue
        path = raw.decode("utf-8")
        try:
            facts = os.lstat(os.path.join(root, path))
        except FileNotFoundError:
            continue
        count += 1
        if facts.st_nlink != 1:
            offenders.append([path, facts.st_nlink])
    return {"count": count, "offenders": sorted(offenders)}


def main():
    command = sys.argv[1]
    if command == "ledger":
        print(json.dumps(ledger(sys.argv[2], sys.argv[3])))
    elif command == "verify":
        request = json.loads(sys.stdin.read())
        print(json.dumps(verify(request["root"], request["device"], request["rows"])))
    elif command == "seal":
        print(sha(canonical(json.loads(sys.argv[2]))))
    elif command == "expect":
        print(json.dumps(expect(sys.argv[2])))
    elif command == "ownership":
        print(json.dumps(ownership(sys.argv[2])))
    elif command == "sha":
        print(hashlib.sha256(open(sys.argv[2], "rb").read()).hexdigest())
    elif command == "nlink":
        print(os.lstat(sys.argv[2]).st_nlink)
    elif command == "manifests":
        print(json.dumps(manifests(sys.argv[2], sys.argv[3:])))
    else:
        raise SystemExit("usage: incremental.oracle.py ledger ROOT START_MS | verify (stdin) | seal JSON | expect FIXTURE | ownership ROOT | sha FILE | nlink FILE | manifests ROOT NAME...")


main()

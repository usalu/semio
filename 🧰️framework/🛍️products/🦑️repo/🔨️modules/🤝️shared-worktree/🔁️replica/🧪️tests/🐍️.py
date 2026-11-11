#!/usr/bin/env python3
"""🐍️ Independent oracle of the shared worktree replica: a second implementation, in Python's standard library only, of the journal fold and of the worktree comparison. It shares no code with `../🟦️.ts`; the TypeScript tests run both on the same scenarios and require that they agree.

usage: python3 -I oracle.py fold FIXTURE                  replays the fold vectors of FIXTURE; exit 1 on the first disagreement
       python3 -I oracle.py conflicts FIXTURE                    replays the conflict vectors: an unpublished local edit beats a remote event only when its modification
                                                          time is later than the wall time inside the remote hlc; a local delete (no time) always loses
       python3 -I oracle.py compare STORE WORKTREE [PATTERN ...]
                                                          folds the journal below STORE/v1 (newest snapshot, then every event after it)
                                                          and compares the shared files of WORKTREE with the folded manifest; PATTERN
                                                          names are fnmatch-ed against every path component; exit 1 on drift
"""
import fnmatch
import hashlib
import json
import os
import re
import sys

HLC = re.compile(r"^[0-9]{13}-[0-9]{6}-[a-z0-9][a-z0-9-]{0,39}$")


def entry(event):
    if event["operation"] == "put":
        return {"hlc": event["hlc"], "operation": "put", "sha256": event["sha256"], "bytes": event["bytes"], "executable": event["executable"]}
    if event["operation"] == "link":
        return {"hlc": event["hlc"], "operation": "link", "target": event["target"]}
    return {"hlc": event["hlc"], "operation": "delete"}


def fold(manifest, events):
    for event in events:
        current = manifest.get(event["path"])
        if current is None or event["hlc"] > current["hlc"]:
            manifest[event["path"]] = entry(event)
    return manifest


def winner(local_mtime_ms, remote_hlc):
    wall = int(remote_hlc.split("-", 1)[0])
    return "local" if local_mtime_ms is not None and local_mtime_ms > wall else "remote"


def read_json(path):
    with open(path, "r", encoding="utf-8") as handle:
        return json.load(handle)


def segment_events(path, start, limit):
    with open(path, "rb") as handle:
        handle.seek(start)
        data = handle.read() if limit is None else handle.read(max(0, limit - start))
    end = data.rfind(b"\n")
    if end < 0:
        return [], start
    lines = data[: end + 1].split(b"\n")
    return [json.loads(line.decode("utf-8")) for line in lines if line], start + end + 1


def newest_snapshot(root):
    found = []
    base = os.path.join(root, "snapshots")
    if os.path.isdir(base):
        for origin in os.listdir(base):
            folder = os.path.join(base, origin)
            for name in os.listdir(folder) if os.path.isdir(folder) else []:
                if name.endswith(".json") and HLC.match(name[:-5]):
                    found.append((name[:-5], os.path.join(folder, name)))
    for hlc, path in sorted(found, reverse=True):
        try:
            value = read_json(path)
            if value["hlc"] == hlc:
                return value
        except (OSError, ValueError, KeyError):
            continue
    return None


def fold_store(store):
    root = os.path.join(store, "v1")
    snapshot = newest_snapshot(root)
    manifest = dict(snapshot["manifest"]) if snapshot else {}
    cursors = snapshot["cursors"] if snapshot else {}
    origins = os.path.join(root, "origins")
    for origin in sorted(os.listdir(origins)) if os.path.isdir(origins) else []:
        head_path = os.path.join(origins, origin, "head.json")
        if not os.path.isfile(head_path):
            continue
        head = read_json(head_path)
        start = cursors.get(origin, {"segment": 0, "bytes": 0})
        for segment in range(start["segment"], head["segment"] + 1):
            path = os.path.join(origins, origin, "segments", "%012d.jsonl" % segment)
            begin = start["bytes"] if segment == start["segment"] else 0
            limit = head["bytes"] if segment == head["segment"] else None
            events, _ = segment_events(path, begin, limit)
            fold(manifest, events)
    return manifest


def ignored(path, patterns):
    parts = path.split("/")
    if parts[0] == ".git" or parts[:2] == [".cache", "f"]:
        return True
    return any(fnmatch.fnmatchcase(part, pattern) for part in parts for pattern in patterns)


def tree(worktree, patterns):
    found = {}
    for folder, directories, files in os.walk(worktree):
        relative = os.path.relpath(folder, worktree).replace(os.sep, "/")
        relative = "" if relative == "." else relative
        directories[:] = [d for d in directories if not ignored((relative + "/" + d).lstrip("/"), patterns)]
        for name in files + [d for d in os.listdir(folder) if os.path.islink(os.path.join(folder, d))]:
            path = (relative + "/" + name).lstrip("/")
            if not ignored(path, patterns):
                found[path] = os.path.join(folder, name)
    return found


def compare(store, worktree, patterns):
    manifest = fold_store(store)
    local = tree(worktree, patterns)
    drift = []
    shared = {path: item for path, item in manifest.items() if item["operation"] != "delete" and not ignored(path, patterns)}
    for path in sorted(shared):
        want = shared[path]
        if path not in local:
            drift.append({"path": path, "kind": "missing-local"})
            continue
        full = local[path]
        if want["operation"] == "link":
            if not os.path.islink(full) or os.readlink(full) != want["target"]:
                drift.append({"path": path, "kind": "target"})
            continue
        if os.path.islink(full) or hashlib.sha256(open(full, "rb").read()).hexdigest() != want["sha256"]:
            drift.append({"path": path, "kind": "content"})
        elif os.name != "nt" and bool(os.stat(full).st_mode & 0o111) != want["executable"]:
            drift.append({"path": path, "kind": "executable"})
    for path in sorted(set(local) - set(shared)):
        drift.append({"path": path, "kind": "extra-local"})
    return {"ok": not drift, "entries": len(shared), "drift": drift}


def main(argv):
    if len(argv) >= 3 and argv[1] == "fold":
        for vector in read_json(argv[2])["folds"]:
            manifest = fold({}, vector["events"])
            if manifest != vector["manifest"]:
                print(json.dumps({"ok": False, "vector": vector["name"], "manifest": manifest}))
                return 1
        print(json.dumps({"ok": True}))
        return 0
    if len(argv) >= 3 and argv[1] == "conflicts":
        for vector in read_json(argv[2])["conflicts"]:
            if winner(vector["localMtimeMs"], vector["remoteHlc"]) != vector["winner"]:
                print(json.dumps({"ok": False, "vector": vector["name"]}))
                return 1
        print(json.dumps({"ok": True}))
        return 0
    if len(argv) >= 4 and argv[1] == "compare":
        report = compare(argv[2], argv[3], argv[4:])
        print(json.dumps(report))
        return 0 if report["ok"] else 1
    sys.stderr.write(__doc__)
    return 2


if __name__ == "__main__":
    sys.exit(main(sys.argv))

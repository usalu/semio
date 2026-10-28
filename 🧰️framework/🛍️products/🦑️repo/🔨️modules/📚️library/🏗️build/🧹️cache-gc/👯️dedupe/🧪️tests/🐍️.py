"""🐍️ Independent oracle of the duplicate census: hashlib over a real tree, groups by (size, sha256), canonical = most links then smallest path among unprotected files, as the specification says. argv: root minimumBytes quietMs nowMs; prints the plan as JSON."""
import hashlib
import json
import os
import re
import sys

root, minimum, quiet, now = sys.argv[1], int(sys.argv[2]), int(sys.argv[3]), int(sys.argv[4])
volatile = re.compile(r"(\.sqlite|\.sqlite3|\.db|\.log|\.jsonl|\.lock|\.pid|-wal|-shm|\.tmp)$", re.I)
groups = {}
for dirpath, dirnames, filenames in os.walk(root):
    for name in filenames:
        path = os.path.join(dirpath, name)
        info = os.lstat(path)
        if not os.path.isfile(path) or os.path.islink(path) or info.st_size < minimum:
            continue
        digest = hashlib.sha256(open(path, "rb").read()).hexdigest()
        recency = max(info.st_mtime, info.st_atime) * 1000
        groups.setdefault((info.st_size, digest), []).append({"path": path, "identity": "%d:%d" % (info.st_dev, info.st_ino), "links": info.st_nlink, "protected": now - recency < quiet or bool(volatile.search(path))})
out = {"groups": 0, "files": 0, "replacements": []}
for key in sorted(groups):
    members = groups[key]
    if len({m["identity"] for m in members}) < 2:
        continue
    stable = [m for m in members if not m["protected"]]
    if not stable:
        continue
    canonical = sorted(stable, key=lambda m: (-m["links"], m["path"]))[0]
    victims = sorted((m for m in stable if m["identity"] != canonical["identity"]), key=lambda m: m["path"])
    if not victims:
        continue
    out["groups"] += 1
    out["files"] += len(members)
    out["replacements"].extend({"canonical": canonical["path"], "path": v["path"]} for v in victims)
print(json.dumps(out))

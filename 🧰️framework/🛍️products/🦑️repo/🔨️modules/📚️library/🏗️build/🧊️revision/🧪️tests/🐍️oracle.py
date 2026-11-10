"""🐍️ Independent oracle of revisions: tarball snapshots and Merkle digests from the standard library only (tarfile, hashlib, json)."""
import hashlib, io, json, os, stat, sys, tarfile, tempfile

def canonical(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False)

def sha(data):
    return hashlib.sha256(data if isinstance(data, bytes) else data.encode("utf-8")).hexdigest()

def snapshot(root, paths, target):
    with tarfile.open(target, "w") as archive:
        for path in sorted(set(paths)):
            full = os.path.join(root, *path.split("/"))
            if os.path.lexists(full):
                archive.add(full, arcname=path, recursive=False)

def entries_of(tar):
    out = []
    with tarfile.open(tar) as archive:
        for member in archive.getmembers():
            if member.issym():
                data = member.linkname.encode("utf-8")
                out.append({"path": member.name, "mode": "symlink", "digest": sha(data), "size": len(data)})
            elif member.isfile():
                data = archive.extractfile(member).read()
                mode = "executable" if member.mode & 0o111 and os.name != "nt" else "file"
                out.append({"path": member.name, "mode": mode, "digest": sha(data), "size": len(data)})
    return out

def tree_digest(entries):
    root = {}
    for entry in entries:
        node = root
        parts = entry["path"].split("/")
        for part in parts[:-1]:
            node = node.setdefault(part, {})
        node[parts[-1]] = (entry,)
    def seal(node):
        rows, total = [], 0
        for name in sorted(node):
            child = node[name]
            if isinstance(child, tuple):
                leaf = child[0]
                rows.append({"digest": leaf["digest"], "kind": leaf["mode"], "name": name, "size": leaf["size"]})
                total += leaf["size"]
            else:
                digest, size = seal(child)
                rows.append({"digest": digest, "kind": "tree", "name": name, "size": size})
                total += size
        return sha(canonical({"entries": rows})), total
    return seal(root)[0]

def action_key(spec):
    return sha(canonical({"command": spec["command"], "environment": spec.get("environment", {}), "manifest": spec["manifest"], "tool": spec["tool"]}))

def main():
    request = json.load(sys.stdin)
    if request["op"] == "snapshot":
        snapshot(request["root"], request["paths"], request["tar"])
        print(json.dumps({"ok": True}))
    elif request["op"] == "digest":
        entries = sorted(entries_of(request["tar"]), key=lambda entry: entry["path"])
        print(json.dumps({"digest": tree_digest(entries), "entries": entries}))
    elif request["op"] == "action":
        print(json.dumps({"key": action_key(request)}))

main()

"""📦 Independent Python auditor of a unit cache root (standard library only): it re-derives the invariants of the content-addressed store from the bytes on disk, and digests directory trees the way build script facts are keyed. Requests arrive as JSON on stdin."""
import hashlib, json, os, sys


def sha(data):
    return hashlib.sha256(data).hexdigest()


def audit(request):
    root, layout = request["root"], request["layout"]
    objects, dangling, leaks, invalid, variants = {}, [], [], [], {}
    base = os.path.join(root, layout["objects"])
    for shard in sorted(os.listdir(base)) if os.path.isdir(base) else []:
        for name in sorted(os.listdir(os.path.join(base, shard))):
            path = os.path.join(base, shard, name)
            data = open(path, "rb").read()
            if sha(data) != name or shard != name[:2]:
                invalid.append("o/" + shard + "/" + name)
            objects[name] = data
    entries = runs = 0
    for kind, counter in ((layout["entries"], "entries"), (layout["runs"], "runs")):
        folder = os.path.join(root, kind)
        for shard in sorted(os.listdir(folder)) if os.path.isdir(folder) else []:
            for key in sorted(os.listdir(os.path.join(folder, shard))):
                files = sorted(f for f in os.listdir(os.path.join(folder, shard, key)) if f.endswith(".json"))
                variants[kind + "/" + key] = len(files)
                for file in files:
                    relative = "/".join([kind, shard, key, file])
                    text = open(os.path.join(folder, shard, key, file), encoding="utf8").read()
                    if counter == "entries":
                        entries += 1
                    else:
                        runs += 1
                    try:
                        entry = json.loads(text)
                    except ValueError:
                        invalid.append(relative)
                        continue
                    if entry.get("key") != key or entry.get("version") != 1:
                        invalid.append(relative)
                    for forbidden in request["forbidden"]:
                        if forbidden in text:
                            leaks.append(relative + " contains an absolute lane path")
                    for row in entry.get("outputs", []):
                        if row["sha256"] not in objects:
                            dangling.append(relative)
                        elif row.get("template") and any(f.encode() in objects[row["sha256"]] for f in request["forbidden"]):
                            leaks.append("o/" + row["sha256"])
                    for stream in ("stdout", "stderr"):
                        digest = entry.get(stream)
                        if digest is not None:
                            if digest not in objects:
                                dangling.append(relative)
                            elif any(f.encode() in objects[digest] for f in request["forbidden"]):
                                leaks.append("o/" + digest)
    return {"objects": len(objects), "entries": entries, "runs": runs, "dangling": sorted(set(dangling)), "leaks": sorted(set(leaks)), "invalid": sorted(set(invalid)), "maxVariants": max(variants.values(), default=0)}


def tree(request):
    rows, overflow = [], False
    opts = request["options"]
    root = request["root"]

    def walk(directory):
        nonlocal overflow
        try:
            names = sorted(os.listdir(directory))
        except OSError:
            return
        for name in names:
            if overflow:
                return
            path = os.path.join(directory, name)
            relative = os.path.relpath(path, root).replace(os.sep, "/")
            if os.path.isdir(path) and not os.path.islink(path):
                if name not in opts["ignoredDirectories"]:
                    walk(path)
                continue
            if any(name.endswith(ext) for ext in opts["ignoredExtensions"]):
                continue
            if len(rows) >= opts["maximumFiles"]:
                overflow = True
                return
            if os.path.islink(path):
                rows.append([relative, "link:" + os.readlink(path)])
            elif os.path.isfile(path):
                rows.append([relative, sha(open(path, "rb").read())])

    walk(root)
    return None if overflow else sha(json.dumps(rows, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))


print(json.dumps([audit(r) if r["kind"] == "audit" else {"digest": tree(r)} for r in json.load(sys.stdin)["requests"]], ensure_ascii=False))

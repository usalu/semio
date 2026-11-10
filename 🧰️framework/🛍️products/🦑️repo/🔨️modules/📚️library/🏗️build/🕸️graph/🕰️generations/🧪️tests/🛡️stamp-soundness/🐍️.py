import json
import os
import sys
import time

PAST = time.time() - 600


def build(root):
    for directory in ["src", "src/sub", "src/dir", "other"]:
        os.makedirs(os.path.join(root, directory), exist_ok=True)
    for name in ["src/a.js", "src/b.js", "src/sub/c.js", "other/o.js"]:
        with open(os.path.join(root, name), "w") as handle:
            handle.write(name)
    os.symlink("b.js", os.path.join(root, "src/lnk.js"))
    os.symlink("../other/o.js", os.path.join(root, "src/far.js"))
    for current, directories, files in os.walk(root, topdown=False):
        for name in files + directories:
            full = os.path.join(current, name)
            if not os.path.islink(full):
                os.utime(full, (PAST, PAST))
        os.utime(current, (PAST, PAST))


def kind(path, follow):
    try:
        info = os.stat(path) if follow else os.lstat(path)
    except OSError:
        return "!ENOENT"
    import stat
    if stat.S_ISLNK(info.st_mode):
        return "symlink"
    return "directory" if stat.S_ISDIR(info.st_mode) else "file"


def resolve_relative(source, request):
    base = os.path.normpath(os.path.join(os.path.dirname(source), request))
    for candidate in [base, base + ".js", base + ".json", base + ".node"]:
        if os.path.isfile(candidate):
            return os.path.realpath(candidate)
    if os.path.isdir(base):
        index = os.path.join(base, "index.js")
        if os.path.isfile(index):
            return os.path.realpath(index)
    return "!MODULE_NOT_FOUND"


def rows(root):
    out = []
    paths = []
    for current, directories, files in os.walk(root):
        for name in directories + files:
            paths.append(os.path.join(current, name))
    paths += [os.path.join(root, "src/missing.js"), os.path.join(root, "src/nodir/x/y.js"), os.path.join(root, "other/deep/er/z.js")]
    for path in sorted(paths):
        out.append({"kind": "exists", "path": path, "fact": os.path.exists(path)})
        out.append({"kind": "fileExists", "path": path, "fact": os.path.isfile(path)})
        out.append({"kind": "directoryExists", "path": path, "fact": os.path.isdir(path)})
        out.append({"kind": "lstat", "path": path, "fact": kind(path, False)})
        out.append({"kind": "stat", "path": path, "fact": kind(path, True)})
    source = os.path.join(root, "src/a.js")
    for request in ["./b", "./lnk", "./sub/c", "../other/o", "./dir", "./nothing"]:
        out.append({"kind": "resolve", "path": json.dumps([source, request], separators=(",", ":")), "fact": resolve_relative(source, request)})
    for directory in [os.path.join(root, "src"), os.path.join(root, "src/sub")]:
        out.append({"kind": "readdirNames", "path": directory, "fact": sorted(os.listdir(directory))})
    return out


def holds(row):
    kind_name, path, fact = row["kind"], row["path"], row["fact"]
    if kind_name == "exists":
        return os.path.exists(path) == fact
    if kind_name == "fileExists":
        return os.path.isfile(path) == fact
    if kind_name == "directoryExists":
        return os.path.isdir(path) == fact
    if kind_name == "lstat":
        return kind(path, False) == fact
    if kind_name == "stat":
        return kind(path, True) == fact
    if kind_name == "resolve":
        source, request = json.loads(path)
        return resolve_relative(source, request) == fact
    if kind_name == "readdirNames":
        return sorted(os.listdir(path)) == fact
    raise SystemExit("unknown kind " + kind_name)


def mutate(root, name):
    src = os.path.join(root, "src")
    if name == "add":
        open(os.path.join(src, "new.js"), "w").write("n")
    elif name == "add-candidate":
        open(os.path.join(src, "b.json"), "w").write("{}")
    elif name == "remove":
        os.remove(os.path.join(src, "a.js"))
    elif name == "rename":
        os.rename(os.path.join(src, "b.js"), os.path.join(src, "b2.js"))
    elif name == "replace-file-with-dir":
        os.remove(os.path.join(src, "b.js"))
        os.mkdir(os.path.join(src, "b.js"))
    elif name == "retarget-symlink":
        os.remove(os.path.join(src, "lnk.js"))
        os.symlink("dir", os.path.join(src, "lnk.js"))
    elif name == "rename-over":
        open(os.path.join(src, "tmp.js"), "w").write("t")
        os.rename(os.path.join(src, "tmp.js"), os.path.join(src, "b.js"))
    elif name == "replace-dir-with-file":
        os.rmdir(os.path.join(src, "dir"))
        open(os.path.join(src, "dir"), "w").write("f")
    elif name == "remove-link-target":
        os.remove(os.path.join(root, "other", "o.js"))
    elif name == "replace-link-target-with-dir":
        os.remove(os.path.join(root, "other", "o.js"))
        os.mkdir(os.path.join(root, "other", "o.js"))
    elif name == "add-missing-tree":
        os.makedirs(os.path.join(src, "nodir", "x"))
        open(os.path.join(src, "nodir", "x", "y.js"), "w").write("y")
    elif name == "add-deep-missing-tree":
        os.makedirs(os.path.join(root, "other", "deep", "er"))
        open(os.path.join(root, "other", "deep", "er", "z.js"), "w").write("z")
    elif name == "touch":
        open(os.path.join(src, "tmp.touch"), "w").write("t")
        os.remove(os.path.join(src, "tmp.touch"))
    elif name == "nested-add":
        open(os.path.join(src, "sub", "d.js"), "w").write("d")
    elif name == "none":
        pass
    else:
        raise SystemExit("unknown mutation " + name)


command = sys.argv[1]
if command == "build":
    build(sys.argv[2])
elif command == "rows":
    print(json.dumps(rows(sys.argv[2])))
elif command == "mutate":
    mutate(sys.argv[2], sys.argv[3])
elif command == "truth":
    print(json.dumps([holds(row) for row in json.load(open(sys.argv[2]))]))

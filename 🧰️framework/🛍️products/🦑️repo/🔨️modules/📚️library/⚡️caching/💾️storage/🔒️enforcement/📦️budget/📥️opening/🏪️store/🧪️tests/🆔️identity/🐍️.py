"""🧮️ Independent oracle of the cache store identity: applies one change to a scratch root and backing file and answers whether (device, inode) of the root and (device, inode, size) of the backing file are still the same tuples; shares no code with the TypeScript implementation."""
import json, os, shutil, sys, tempfile
rows = json.load(open(sys.argv[1]))["rows"]


def identity(root, backing):
    try:
        r, b = os.lstat(root), os.lstat(backing)
    except OSError:
        return None
    if not os.path.isdir(root) or os.path.islink(root) or not os.path.isfile(backing) or os.path.islink(backing):
        return None
    return (r.st_dev, r.st_ino, b.st_dev, b.st_ino, b.st_size)


out = {}
for row in rows:
    base = tempfile.mkdtemp(prefix="oracle-")
    try:
        root, backing = os.path.join(base, "m"), os.path.join(base, "fixed.img")
        os.mkdir(root)
        open(backing, "wb").write(b"x" * 4096)
        before = identity(root, backing)
        change = row["change"]
        if change == "write-below-root":
            open(os.path.join(root, "payload"), "wb").write(b"y" * 100000)
        elif change == "replace-root":
            os.rename(root, root + ".old"); os.mkdir(root)
        elif change == "root-symlink":
            os.rename(root, root + ".old"); os.symlink(root + ".old", root)
        elif change == "replace-backing":
            os.rename(backing, backing + ".old"); open(backing, "wb").write(b"x" * 4096)
        elif change == "grow-backing":
            open(backing, "ab").write(b"z" * 10)
        elif change == "remove-backing":
            os.remove(backing)
        after = identity(root, backing)
        out[row["id"]] = after is not None and after == before
    finally:
        shutil.rmtree(base, ignore_errors=True)
print(json.dumps(out, sort_keys=True))

"""🗂️ Independent model of the stat index decisions.

One command reads one scenario on stdin and prints, per step, the digest of every queried file and the files that had to be read:
  run {"racy": ms, "steps": [{"now": ms, "files": {path: {"size", "mtime", "ino", "content"}}, "query": [path]}]}
      -> {"steps": [{"digests": {path: sha256 | "absent"}, "reads": [path]}]}
A file is read when it has no record, when its size, mtime or inode differ from the record, or when the record was made less than `racy` milliseconds after the file's mtime. A file that is not there is `absent` and is not read. Standard library only.
"""
import hashlib
import json
import sys


def run(scn):
    cache, out = {}, []
    for step in scn["steps"]:
        digests, reads = {}, []
        for path in step["query"]:
            f = step["files"].get(path)
            if f is None:
                digests[path] = "absent"
                continue
            rec = cache.get(path)
            if rec and rec[:3] == [f["size"], f["mtime"], f["ino"]] and rec[4] - f["mtime"] >= scn["racy"]:
                digests[path] = rec[3]
                continue
            digest = hashlib.sha256(f["content"].encode("utf8")).hexdigest()
            cache[path] = [f["size"], f["mtime"], f["ino"], digest, step["now"]]
            digests[path] = digest
            reads.append(path)
        out.append({"digests": digests, "reads": sorted(reads)})
    return {"steps": out}


if __name__ == "__main__":
    if sys.argv[1:] == ["run"]:
        print(json.dumps(run(json.load(sys.stdin))))
    else:
        sys.exit(2)

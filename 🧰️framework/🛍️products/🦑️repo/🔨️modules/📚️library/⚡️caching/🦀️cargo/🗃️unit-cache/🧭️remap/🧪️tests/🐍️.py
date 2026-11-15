#!/usr/bin/env python3
"""Independent model of rustc's path remapping (FilePathMapping::map_prefix): reads the vectors and prints one line per case with the computed name."""
import json, sys
def strip_prefix(path, prefix):
    p = [c for c in path.split("/") if c != ""]
    q = [c for c in prefix.split("/") if c != ""]
    if p[:len(q)] != q or (path.startswith("/") != prefix.startswith("/")):
        return None
    return p[len(q):]
def join(to, rest):
    if not rest: return to
    return (to.rstrip("/") + "/" if to else "") + "/".join(rest)
def remap(remaps, path):
    for frm, to in reversed(remaps):
        rest = strip_prefix(path, frm)
        if rest is not None:
            return to if not rest else join(to, rest)
    return path
for case in json.load(open(sys.argv[1]))["cases"]:
    remaps = [tuple(r.rsplit("=", 1)) for r in case["remaps"]]
    print(json.dumps({"label": case["label"], "name": remap(remaps, case["path"])}, ensure_ascii=False))

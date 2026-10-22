#!/usr/bin/env python3
"""🐍️ Independent reader of the graph record format: rebuilds a pinned graph from its manifest and gzip records with plain Python and prints it as canonical JSON.

usage: oracle.py DIRECTORY MANIFEST_DIGEST"""
import gzip, hashlib, json, os, sys

directory, digest = sys.argv[1], sys.argv[2]
raw = open(os.path.join(directory, digest + ".json"), "rb").read()
assert hashlib.sha256(raw).hexdigest() == digest, "manifest digest"
manifest = json.loads(raw)


def record(name):
    packed = open(os.path.join(directory, name + ".gz"), "rb").read()
    assert hashlib.sha256(packed).hexdigest() == name, "record digest " + name
    return json.loads(gzip.decompress(packed))


graph = dict(manifest["graph"])
graph["projects"] = {row[0]: record(row[1]) for row in manifest["projects"]}
graph["dependencies"] = [edge for name in manifest["dependencies"] for edge in record(name)]
if "externalNodes" in manifest:
    nodes = {}
    for name in manifest["externalNodes"]:
        nodes.update(record(name))
    graph["externalNodes"] = nodes
print(json.dumps(graph, sort_keys=True, separators=(",", ":"), ensure_ascii=False))

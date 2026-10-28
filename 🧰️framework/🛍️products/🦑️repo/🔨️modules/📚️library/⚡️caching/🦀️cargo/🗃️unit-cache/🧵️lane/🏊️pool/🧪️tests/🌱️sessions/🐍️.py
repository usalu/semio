"""🌱 Independent oracle for the lane pool's session-free size: walks each lane, sums the logical bytes of its files outside the Cargo incremental directories (`b/<profile>/incremental`, `b/<triple>/<profile>/incremental`) and without the size file the measurement writes, then trims least recently used idle lanes until the sum fits the quota.
usage: python3 -I 🐍️.py <cache root> <fixture.json>   prints {"bytes": {lane: n}, "trims": {case id: [lane ids]}}"""
import json, os, sys
root, fixture = sys.argv[1], json.load(open(sys.argv[2], encoding="utf8"))
def size(base):
    total = 0
    for directory, names, files in os.walk(base):
        rel = os.path.relpath(directory, os.path.join(base, "b")).split(os.sep)
        names[:] = [n for n in names if not (n == "incremental" and len(rel) <= 2 and rel[0] != "..")]
        total += sum(os.path.getsize(os.path.join(directory, f)) for f in files if not (directory == base and f == "lane.size"))
    return total
bytes_ = {lane["id"]: size(os.path.join(root, "l", lane["id"])) for lane in fixture["lanes"]}
order = [lane["id"] for lane in fixture["lanes"]]
def trim(quota):
    total, out = sum(bytes_.values()), []
    for lane in order:
        if total <= quota: break
        out.append(lane); total -= bytes_[lane]
    return out
print(json.dumps({"bytes": bytes_, "trims": {case["id"]: trim(case["quotaBytes"]) for case in fixture["cases"]}}))

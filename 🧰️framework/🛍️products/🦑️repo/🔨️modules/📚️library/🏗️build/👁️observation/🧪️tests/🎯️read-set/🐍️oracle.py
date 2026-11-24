import hashlib
import json
import sys

scenario = json.load(sys.stdin)
digest = lambda text: hashlib.sha256(text.encode("utf-8")).hexdigest()[:12]
files = scenario["files"]
edited = {**files, **scenario["edits"]}
stale = {}
for owner, reads in scenario["reads"].items():
    moved = [name for name in reads if edited[name] != files[name]]
    if moved:
        first = moved[0]
        stale[owner] = {"file": first, "recorded": digest(files[first]), "current": digest(edited[first])}
json.dump({"stale": stale, "holds": sorted(set(scenario["reads"]) - set(stale))}, sys.stdout)

"""🐍 Independent answer for the analysis store law: for every closure store path of the fixture, the analysis store directory Python's own `os.path` derives on this platform (`sa` beside the closure store), or `null` where the path is not absolute here and must be refused."""
import json
import os.path
import sys

fixture = json.load(open(sys.argv[1], encoding="utf-8"))
paths = [row["closureStore"] for row in fixture["derived"]] + fixture["refused"]
print(json.dumps({path: os.path.join(os.path.dirname(path), "sa") if os.path.isabs(path) else None for path in paths}))

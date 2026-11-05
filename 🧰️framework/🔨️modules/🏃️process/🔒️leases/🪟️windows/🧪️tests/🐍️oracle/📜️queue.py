"""📜️ Independently observes a queue test source through Python filesystem and JSON libraries."""
import json, pathlib, sys
path = pathlib.Path(sys.argv[1])
print(json.dumps({"exists": True, "value": json.loads(path.read_text(encoding="utf-8"))} if path.exists() else {"exists": False}))

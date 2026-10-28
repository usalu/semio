"""🚧️ Independent model of the unfounded withheld owners: an untagged withheld owner inside the workspace is founded when one of its own rows names its source file, whole path components only. Reads the fixture path; prints the sorted unfounded names."""
import json
import sys

fixture = json.load(open(sys.argv[1], encoding="utf8"))
workspace = fixture["workspace"]
result = []
for name, owner in fixture["owners"].items():
    if owner["kind"] != "unavailable" or owner.get("tag") or owner["config"].startswith(".."):
        continue
    names_source = any(row["kind"] in ("file", "analysis", "digest") and (row["path"] == owner["config"] or row["path"] == workspace + "/" + owner["config"] or row["path"].endswith("/" + owner["config"])) for row in owner["rows"])
    if not names_source:
        result.append(name)
print(json.dumps(sorted(result)))

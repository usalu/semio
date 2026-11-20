"""🐍️ Independent oracle of the digest memo: decides from the schema's racy window whether a file's digest may be remembered and hashes the fixture files with hashlib, sharing no line with the implementation."""
import hashlib
import json
import sys


def main(fixture_path, schema_path):
    with open(fixture_path, encoding="utf-8") as handle:
        fixture = json.load(handle)
    with open(schema_path, encoding="utf-8") as handle:
        protocol = json.load(handle)["definitions"]["Protocol"]["const"]
    window = protocol["racyMilliseconds"]
    racy = {}
    for row in fixture["racy"]:
        newest = max(int(row["mtimeNs"]), int(row["ctimeNs"]))
        racy[row["id"]] = newest <= (row["startedMs"] - window) * 1000000
    digests = {row["name"]: hashlib.sha256(row["text"].encode("utf-8")).hexdigest() for row in fixture["files"]}
    json.dump({"racy": racy, "digests": digests}, sys.stdout)


main(sys.argv[1], sys.argv[2])

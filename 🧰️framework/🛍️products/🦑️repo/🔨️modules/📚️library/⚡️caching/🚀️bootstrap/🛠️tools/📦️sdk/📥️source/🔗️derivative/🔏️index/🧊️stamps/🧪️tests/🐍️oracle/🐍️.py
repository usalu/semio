import hashlib, json, re, sys

POLICY = {"maximumAgeMilliseconds": 86400000, "maximumRows": 100000}
STAMP = re.compile(r"^[0-9]+(:[0-9]+){4}$")
SHA = re.compile(r"^[a-f0-9]{64}$")


def key(facts):
    return ":".join(str(int(facts[name])) for name in ("dev", "ino", "size", "mtimeNs", "mode"))


def memo_file(base, directory, source_index):
    digest = hashlib.sha256((directory + "\0" + source_index).encode()).hexdigest()[:32]
    return base.rstrip("/") + "/sdk-verified/" + digest + ".json"


def warm(document, source_index, now):
    if not isinstance(document, dict):
        return False
    if set(document) != {"version", "sourceIndex", "verifiedAt", "stamps"}:
        return False
    if document["version"] != 1 or not isinstance(document["sourceIndex"], str) or not SHA.match(document["sourceIndex"]):
        return False
    verified = document["verifiedAt"]
    if isinstance(verified, bool) or not isinstance(verified, int) or verified < 0:
        return False
    stamps = document["stamps"]
    if not isinstance(stamps, dict) or len(stamps) > POLICY["maximumRows"]:
        return False
    if any(not isinstance(value, str) or not STAMP.match(value) for value in stamps.values()):
        return False
    return document["sourceIndex"] == source_index and verified <= now and now - verified <= POLICY["maximumAgeMilliseconds"]


def decided(row):
    try:
        document = json.loads(row["raw"]) if "raw" in row else row["document"]
    except ValueError:
        return False
    return warm(document, row["sourceIndex"], row["now"])


def main():
    fixture = json.load(sys.stdin)
    print(json.dumps({
        "keys": [key(row) for row in fixture["keys"]],
        "files": [memo_file(row["base"], row["directory"], row["sourceIndex"]) for row in fixture["files"]],
        "documents": [decided(row) for row in fixture["documents"]],
    }))


main()

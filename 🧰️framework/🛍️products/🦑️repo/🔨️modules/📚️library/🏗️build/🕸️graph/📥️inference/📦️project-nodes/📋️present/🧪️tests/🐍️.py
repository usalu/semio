"""Independent oracle of the owner record census: the key of a record is the first 24 hex digits of the sha256 of the owner root, a root is present when `<key>.json` is among the names of the directory. Reads {"cases"} from the file in argv[1] and prints, per case, the sorted present roots."""
import hashlib
import json
import sys


def key(root):
    return hashlib.sha256(root.encode("utf8")).hexdigest()[:24]


def census(case):
    names = set(case["names"])
    return sorted(root for root in case["roots"] if key(root) + ".json" in names)


print(json.dumps([census(case) for case in json.load(open(sys.argv[1], encoding="utf8"))["cases"]], ensure_ascii=False))

import hashlib
import json
import sys


def canonical(value):
    return json.dumps(value, sort_keys=True, separators=(",", ":"), ensure_ascii=False)


def extract(document, pointer):
    if pointer == "":
        return True, document
    value = document
    for raw in pointer[1:].split("/"):
        key = raw.replace("~1", "/").replace("~0", "~")
        if isinstance(value, dict) and key in value:
            value = value[key]
        elif isinstance(value, list) and key.isdigit() and int(key) < len(value):
            value = value[int(key)]
        else:
            return False, None
    return True, value


corpus = json.load(open(sys.argv[1], encoding="utf8"))
out = {}
for document in corpus["documents"]:
    parsed = json.loads(document["text"])
    for pointer in document["pointers"]:
        found, value = extract(parsed, pointer)
        out[document["id"] + "|" + pointer] = hashlib.sha256(canonical(value).encode("utf8")).hexdigest() if found else "!missing"
print(json.dumps(out))

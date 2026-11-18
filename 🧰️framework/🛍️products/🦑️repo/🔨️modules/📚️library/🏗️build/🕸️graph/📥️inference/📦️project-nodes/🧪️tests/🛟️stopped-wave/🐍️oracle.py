"""Independent oracle of the identity of a stored project record: the header digest is the sha256 of the compact JSON text of [record stamp, null, key], whatever process, session, machine or generation wrote it. Reads {"stamp", "keys"} from the file in argv[1] and prints the list of digests."""
import hashlib
import json
import sys

data = json.load(open(sys.argv[1], encoding="utf8"))
print(json.dumps([hashlib.sha256(json.dumps([data["stamp"], None, key], separators=(",", ":"), ensure_ascii=False).encode("utf8")).hexdigest() for key in data["keys"]]))

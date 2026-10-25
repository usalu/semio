import hashlib, json, sys

store = json.load(open(sys.argv[1], encoding="utf8"))["definitions"]["Identity"]["const"]
registries = json.load(open(sys.argv[2], encoding="utf8"))["registries"]
print(json.dumps({name: hashlib.sha256(json.dumps([store, [[p["ownerRoot"], p["sourcePath"]] for p in registry["catalogProviders"]]], separators=(",", ":"), ensure_ascii=False).encode("utf8")).hexdigest() for name, registry in registries.items()}))

import hashlib, json, sys

def key(provider, revision, rel, sha): return json.dumps([provider, revision, rel, sha])

def reanalysed(case):
    stored = {key(case["provider"], case["revision"], rel, sha) for rel, sha in case["before"].items()}
    return sorted(rel for rel, sha in case["after"].items() if key(case["provider"], case["revision"], rel, sha) not in stored)

print(json.dumps(reanalysed(json.load(open(sys.argv[1])))))

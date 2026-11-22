"""Independent oracle of the Cargo target key: filters the observed recipe inputs to the keyed ones and hashes the canonical tuple.
Usage: python -I oracle.py <vectors.json>  (prints the verdict of every case as JSON)"""
import hashlib, json, sys

WITNESS_ONLY = ("Cargo.toml", "Cargo.lock")


def keyed(path):
    return path.replace("\\", "/").rsplit("/", 1)[-1] not in WITNESS_ONLY


def canonical(recipe, observed):
    inputs = sorted(({"path": r["path"], "sha256": r["sha256"]} for r in observed if keyed(r["path"])), key=lambda r: r["path"].encode())
    env = sorted(({"name": r["name"], "value": r["value"]} for r in recipe["environment"]), key=lambda r: r["name"].encode())
    row = [1, recipe["cwd"], recipe["manifest"], recipe["command"], recipe["args"], recipe["buildDirectory"], recipe["compilerIdentity"], recipe["toolchainIdentity"], inputs, env]
    return json.dumps(row, separators=(",", ":"), ensure_ascii=False)


def verdict(recipe, observed):
    text = canonical(recipe, observed)
    return {"keyed": sorted(r["path"] for r in observed if keyed(r["path"])), "sha256": hashlib.sha256(text.encode()).hexdigest()}


if __name__ == "__main__":
    corpus = json.load(open(sys.argv[1], encoding="utf-8"))
    print(json.dumps({c["id"]: verdict({**corpus["recipe"], **c.get("recipe", {})}, c["observed"]) for c in corpus["cases"]}))

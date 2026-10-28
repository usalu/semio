"""🐍 Independent Python oracle of the Nx engine digest (standard library only): reads {"root", "family", "files", "schedulingPolicy", "enginePolicy"} as JSON on stdin and prints {"keyed", "unkeyed", "digest"}: the unkeyed prefixes are the engine domains plus every scheduling prefix (the `native` family without those that equal, contain or lie inside a `nativeKeyed` row); the digest is sha256 over `name NUL sha256(bytes) LF` rows of the keyed files, then `NUL unkeyed NUL sha256(prefixes joined by LF) LF`."""
import hashlib, json, os, sys

request = json.load(sys.stdin)
engine, scheduling = request["enginePolicy"], request["schedulingPolicy"]
prefixes = {prefix for domain in engine["domains"] if request["family"] not in domain.get("keyedFor", []) for prefix in domain["prefixes"]}
rows = {prefix for domain in scheduling["domains"] for prefix in domain["prefixes"]}
if request["family"] == "native":
    kept = [row for domain in scheduling["domains"] for row in domain.get("nativeKeyed", [])]
    rows = {prefix for prefix in rows if not any(k.startswith(prefix) or prefix.startswith(k) for k in kept)}
prefixes = sorted(prefixes | rows)
keyed = [f for f in request["files"] if not any(f.startswith(p) for p in prefixes)]
unkeyed = [f for f in request["files"] if any(f.startswith(p) for p in prefixes)]
digest = hashlib.sha256()
for f in keyed:
    with open(os.path.join(request["root"], f), "rb") as handle:
        digest.update(f.encode() + b"\0" + hashlib.sha256(handle.read()).hexdigest().encode() + b"\n")
if prefixes:
    digest.update(b"\0unkeyed\0" + hashlib.sha256("\n".join(prefixes).encode()).hexdigest().encode() + b"\n")
print(json.dumps({"keyed": keyed, "unkeyed": unkeyed, "digest": digest.hexdigest()}, ensure_ascii=False))

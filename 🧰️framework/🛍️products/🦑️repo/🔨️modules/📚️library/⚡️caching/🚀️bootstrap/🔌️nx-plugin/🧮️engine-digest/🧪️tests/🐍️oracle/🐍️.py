"""🐍 Independent Python oracle of the Nx engine digest (standard library only): reads {"root", "family", "files", "enginePolicy"} as JSON on stdin and prints {"covered", "digest"}: the covered files are the declared surface files and, for the `native` family, the closure files below a declared Cargo argument prefix that have no excluded path segment; the digest is sha256 over `name NUL sha256(bytes) LF` rows of the covered files sorted, then `NUL identity NUL sha256("semantics <revision>" LF "family <name>") LF`."""
import hashlib, json, os, sys

request = json.load(sys.stdin)
semantics = request["enginePolicy"]["semantics"]
covered = set(semantics["surface"])
if request["family"] == "native":
    for f in request["files"]:
        if any(f.startswith(p) for p in semantics["nativeSurface"]) and not any(part in semantics["excludedSegments"] for part in f.split("/")):
            covered.add(f)
covered = sorted(covered)
digest = hashlib.sha256()
for f in covered:
    with open(os.path.join(request["root"], f), "rb") as handle:
        digest.update(f.encode() + b"\0" + hashlib.sha256(handle.read()).hexdigest().encode() + b"\n")
identity = "semantics %d\nfamily %s" % (semantics["revision"], request["family"])
digest.update(b"\0identity\0" + hashlib.sha256(identity.encode()).hexdigest().encode() + b"\n")
print(json.dumps({"covered": covered, "digest": digest.hexdigest()}, ensure_ascii=False))

"""🔁️ Independent Python oracle of the graph retry diagnostic projection (standard library only, POSIX paths): only bounded identities and digests leave the owner, paths outside the root are redacted, facts are keyed by their own digest or the digest of their canonical JSON. Requests arrive as JSON on stdin."""
import hashlib, json, posixpath, re, sys

REASONS = ["membership", "admission", "observed-facts", "producer-error", "owner-registry", "publication-facts"]
DIGEST = re.compile(r"[a-f0-9]{64}")


def utf16(text):
    return len(text.encode("utf-16-le")) // 2


def project(row, root):
    if row["reason"] not in REASONS or any(v is not None and not DIGEST.fullmatch(v) for v in (row["before"], row["after"])):
        return {"error": "Declared graph retry reason and content digests required"}
    facts = []
    for item in row["stale"][:8]:
        absolute = posixpath.normpath(posixpath.join(root, item["path"]))
        distance = posixpath.relpath(absolute, root)
        distance = "" if distance == "." else distance
        inside = distance != ".." and not distance.startswith("../") and not posixpath.isabs(distance) and utf16(distance) <= 256
        encoded = json.dumps(item["fact"], separators=(",", ":"), ensure_ascii=False)
        if len(encoded.encode("utf8")) > 65536:
            return {"error": "Bounded observed graph fact required"}
        sha = item["fact"].get("sha256") if isinstance(item["fact"], dict) else None
        expected = sha if item["kind"] == "file" and isinstance(sha, str) and DIGEST.fullmatch(sha) else hashlib.sha256(encoded.encode("utf8")).hexdigest()
        actual = None
        if inside and item["kind"] == "file":
            value = row["current"].get(distance)
            if value is not None and DIGEST.fullmatch(value):
                actual = value
        facts.append({"kind": str(item["kind"])[:64], "path": distance if inside else None, "before": expected, "after": actual})
    return {"value": {"version": 1, "reason": row["reason"], "before": row["before"], "after": row["after"], "facts": facts}}


request = json.load(sys.stdin)
print(json.dumps([project(row, request["root"]) for row in request["rows"]], ensure_ascii=False))

"""🐍 Independent Python oracle of the scheduling policy (standard library only): reads the policy JSON on stdin and prints, as JSON, the sorted prefixes that may leave the key of a command that compiles Rust (every scheduling prefix that no `nativeKeyed` row equals, contains or lies inside); with the argument `islands` it prints instead every pair of a scheduling prefix and a content island (`keyed`) of which one contains the other."""
import json, sys

policy = json.load(sys.stdin)
if len(sys.argv) > 1 and sys.argv[1] == "islands":
    prefixes = sorted({prefix for domain in policy["domains"] for prefix in domain["prefixes"]})
    print(json.dumps([[prefix, row["path"]] for prefix in prefixes for row in policy.get("keyed", []) if prefix.startswith(row["path"]) or row["path"].startswith(prefix)], ensure_ascii=False))
    sys.exit(0)
keyed = [row for domain in policy["domains"] for row in domain.get("nativeKeyed", [])]
prefixes = sorted({prefix for domain in policy["domains"] for prefix in domain["prefixes"]})
print(json.dumps([p for p in prefixes if not any(k.startswith(p) or p.startswith(k) for k in keyed)], ensure_ascii=False))

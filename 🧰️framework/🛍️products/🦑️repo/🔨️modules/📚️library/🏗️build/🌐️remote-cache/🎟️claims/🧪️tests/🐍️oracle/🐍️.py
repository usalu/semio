"""Independent reference model of the claim table: replays operation sequences on a virtual clock (JSON on stdin, JSON on stdout)."""
import json
import sys

MIN_TTL, MAX_TTL = 1000, 300000
MIN_LIFE, MAX_LIFE, MULTIPLE, COOLOFF = 120000, 1800000, 4, 30000


def lifetime(claim):
    return min(MAX_LIFE, max(MIN_LIFE, claim["ttl"], (claim["expected"] or 0) * MULTIPLE))


def sweep_full(claims, at, maximum_claims, live):
    if len(claims) < maximum_claims:
        return False
    for key in list(claims):
        live(key, at)
    return len(claims) >= maximum_claims


def run(steps, maximum_claims):
    claims = {}
    capped = {}
    stored = set()
    out = []

    def live(address, at):
        claim = claims.get(address)
        if claim is None:
            return None
        if claim["until"] > at:
            return claim
        del claims[address]
        return None

    for step in steps:
        at, op = step["at"], step["op"]
        address = step.get("key", "")
        owner = step.get("owner", "")
        ttl = min(MAX_TTL, max(MIN_TTL, int(step.get("ttl", 0))))
        if op == "claim":
            current = live(address, at)
            cooling = capped.get(address)
            if step.get("stored"):
                out.append({"state": "stored"})
            elif cooling is not None and cooling["until"] > at and cooling["owner"] == owner:
                out.append({"state": "held", "by": "lifetime-cap", "untilMilliseconds": cooling["until"]})
            elif current is not None and current["owner"] == owner and at - current["granted"] >= lifetime(current):
                del claims[address]
                capped[address] = {"owner": owner, "until": at + COOLOFF}
                out.append({"state": "held", "by": "lifetime-cap", "untilMilliseconds": at + COOLOFF})
            elif current is not None and current["owner"] != owner:
                row = {"state": "held", "by": current["owner"], "untilMilliseconds": current["until"]}
                if current["expected"] is not None:
                    row["expectedMilliseconds"] = current["expected"]
                out.append(row)
            elif current is None and sweep_full(claims, at, maximum_claims, live):
                out.append({"state": "full"})
            else:
                expected = step.get("expected")
                granted = current["granted"] if current else at
                keep = current["expected"] if current else None
                claims[address] = {"owner": owner, "granted": granted, "until": at + ttl, "ttl": ttl, "expected": expected if expected is not None else keep}
                out.append({"state": "granted", "owner": owner, "untilMilliseconds": at + ttl, "ttlMilliseconds": ttl})
        elif op == "renew":
            current = live(address, at)
            if current is None or current["owner"] != owner:
                out.append({"state": "lost"})
            elif at - current["granted"] >= lifetime(current):
                del claims[address]
                capped[address] = {"owner": owner, "until": at + COOLOFF}
                out.append({"state": "lost"})
            else:
                current["until"] = min(at + ttl, current["granted"] + lifetime(current))
                current["ttl"] = ttl
                out.append({"state": "granted", "owner": owner, "untilMilliseconds": current["until"], "ttlMilliseconds": ttl})
        elif op == "release":
            current = live(address, at)
            if current is None or current["owner"] != owner:
                out.append({"state": "absent"})
            else:
                del claims[address]
                out.append({"state": "released"})
        elif op == "stored":
            claims.pop(address, None)
            stored.add(address)
            out.append({"state": "ok"})
        elif op == "sweep":
            for key in list(claims):
                live(key, at)
            out.append({"state": "ok"})
        elif op == "claimed":
            out.append({"claimed": live(address, at) is not None})
        elif op == "summary":
            active = [c for c in claims.values() if c["until"] > at]
            oldest = max([at - c["granted"] for c in active], default=0)
            out.append({"active": len(active), "oldestSeconds": oldest // 1000})
    return out


if __name__ == "__main__":
    request = json.load(sys.stdin)
    batches = request["batches"] if "batches" in request else [request]
    json.dump([run(b["steps"], b.get("maximumClaims", 65536)) for b in batches], sys.stdout, separators=(",", ":"))

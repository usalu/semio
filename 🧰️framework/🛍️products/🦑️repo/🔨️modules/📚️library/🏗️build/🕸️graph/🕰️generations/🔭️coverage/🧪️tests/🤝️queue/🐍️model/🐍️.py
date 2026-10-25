"""Independent state machine of the graph builder queue with verified stamps.

Time is an abstract integer. One scenario is a list of owners with edit times, a list of requests (id, requestedAt, closure) and the builder custody (FIFO by request time, one holder at a time, a build takes `duration`). Rules, exactly the protocol of the store:
  * a request first verifies without custody: it is served `verified` when no owner of its closure was edited after the time that owner was last verified (never edited owners count as verified at time 0) up to its request time;
  * otherwise it queues for the custody; the holder at time b first consults the stamp of the sealed generation: a stamp not older than the request covers the owners it names, `covered` when nothing of the closure is left;
  * otherwise the holder verifies the uncovered owners at time b and, when one is stale, builds them: it re-verifies them as of b, publishes a generation whose stamp is (min(carried time, b) if it reused coverage else b, union of the closure and the reused owners) after `duration`; a crash of the holder publishes nothing and releases the custody.
Prints one JSON document {"builds": n, "served": {id: kind}, "stamp": {"atMs": n, "owners": [...]}|null, "generations": n}.
"""
import json
import sys


def run(scenario):
    edits = scenario["edits"]
    verified_at = {}
    stamp = None
    generations = 0
    builds = 0
    served = {}
    clock = scenario.get("busyUntil", 0)
    crashes = set(scenario.get("crashes", []))

    def stale(owner, upto):
        last = verified_at.get(owner, 0)
        return any(e["owner"] == owner and last < e["at"] <= upto for e in edits)

    queue = []
    for request in sorted(scenario["requests"], key=lambda r: (r["requestedAt"], r["id"])):
        if any(stale(o, request["requestedAt"]) for o in request["closure"]):
            queue.append(request)
        else:
            served[request["id"]] = "verified"
    for request in queue:
        start = max(clock, request["requestedAt"])
        covered = None
        if stamp is not None and stamp["atMs"] >= request["requestedAt"]:
            covered = set(stamp["owners"])
        open_owners = [o for o in request["closure"] if covered is None or o not in covered]
        if not open_owners:
            served[request["id"]] = "covered"
            clock = start
            continue
        if request["id"] in crashes:
            served[request["id"]] = "crashed"
            clock = start + scenario["duration"] // 2
            continue
        if not any(stale(o, start) for o in open_owners):
            served[request["id"]] = "verified-in-custody"
            clock = start
            continue
        for o in open_owners:
            verified_at[o] = start
        carried = covered is not None
        owners = set(open_owners) | (covered if carried else set())
        at = min(stamp["atMs"], start) if carried else start
        stamp = {"atMs": at, "owners": sorted(owners)}
        generations += 1
        builds += 1
        served[request["id"]] = "built"
        clock = start + scenario["duration"]
    return {"builds": builds, "served": served, "stamp": stamp, "generations": generations}


for scenario in json.load(sys.stdin)["scenarios"]:
    print(json.dumps({"id": scenario["id"], **run(scenario)}, sort_keys=True, separators=(",", ":")))

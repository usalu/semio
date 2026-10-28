"""Independent model of the claim protocol of a graph refresh (reads only the schema file and JSON on stdin).

Commands (JSON on stdin, JSON on stdout):
  chunk    one ChunkCase               -> open, take, running, prepared, covered
  slots    one SlotCase                -> slots
  claim    {claim}                     -> accepted (closed claim document)
  explore  {participants, owners, chunk, tail, crashes, broken} -> exhaustive search of every interleaving of
           admit / finish / crash of the participants: states, terminals, violations (a violation is a state where
           two live or done claims overlap, or a terminal state that left an owner of a finished participant undone)
"""
import json
import re
import sys
from collections import deque

POLICY = None


def policy():
    global POLICY
    if POLICY is None:
        import os
        here = os.path.dirname(os.path.abspath(__file__))
        with open(os.path.join(here, "..", "..", "🧬️schema", "🔣️.json"), encoding="utf8") as handle:
            POLICY = json.load(handle)["definitions"]["Policy"]["const"]
    return POLICY


def chunk(case):
    names = sorted(set(case["wanted"]))
    wanted = set(names)
    stamps = case.get("stamps", {})
    now = case.get("nowMs")
    covered, done, running = set(), set(), set()
    for claim in case["claims"]:
        if claim["base"] != case["base"] or claim["implementation"] != case["implementation"]:
            continue
        own = claim.get("stamps", {})
        same = [name for name in claim["owners"] if own.get(name) == stamps.get(name)]
        if claim["state"] == "running":
            if not claim["live"]:
                continue
            if wanted.intersection(claim["owners"]):
                running.add(claim["token"])
            covered.update(claim["owners"])
        elif claim["state"] == "done":
            done.update(same)
            covered.update(same)
        elif now is None or "endedMs" not in claim or now - claim["endedMs"] < policy()["failedRetryMilliseconds"]:
            covered.update(same)
    open_ = [name for name in names if name not in covered]
    take = open_ if len(open_) <= case["chunk"] + case["tail"] else open_[: case["chunk"]]
    return {
        "open": open_,
        "take": take,
        "running": sorted(running),
        "prepared": sum(1 for name in names if name in done),
        "covered": len(names) - len(open_),
    }


def slots(case):
    p = policy()
    forced = case["environment"].get(p["environmentSlots"])
    if forced is not None and re.fullmatch(r"[+-]?\d+(\.0+)?", forced.strip() or "x"):
        value = int(float(forced))
        if 1 <= value <= p["maximumSlots"]:
            return {"slots": value}
    share = case["memoryMegabytes"] * p["slotMemoryBasisPoints"] // 10000 // p["slotMegabytes"]
    return {"slots": max(1, min(p["maximumSlots"], share))}


def claim_accepted(value):
    required = {"version", "token", "base", "implementation", "owners", "state", "startedMs"}
    if not isinstance(value, dict) or not required <= set(value) or not set(value) <= required | {"endedMs"}:
        return False
    if value["version"] != 1 or not isinstance(value["token"], str) or not re.fullmatch(r"[a-f0-9]{24}", value["token"]):
        return False
    if not isinstance(value["base"], int) or isinstance(value["base"], bool) or value["base"] < -1:
        return False
    if not isinstance(value["implementation"], str) or not re.fullmatch(r"[a-f0-9]{64}", value["implementation"]):
        return False
    if value["state"] not in ("running", "done", "failed"):
        return False
    for key in ("startedMs", "endedMs"):
        if key in value and (not isinstance(value[key], int) or isinstance(value[key], bool) or value[key] < 0):
            return False
    owners = value["owners"]
    if not isinstance(owners, list) or len(owners) > policy()["maximumOwners"]:
        return False
    if any(not isinstance(name, str) or name == "" for name in owners):
        return False
    return all(owners[at - 1] < owners[at] for at in range(1, len(owners)))


def select(wanted, claims, size, tail, broken):
    covered = set()
    for owners, state, alive in claims:
        if state in ("done", "failed"):
            covered.update(owners)
        elif state == "running" and alive and broken != "ignore-running":
            covered.update(owners)
    open_ = sorted(set(wanted) - covered)
    take = open_ if len(open_) <= size + tail else open_[:size]
    blockers = [i for i, (owners, state, alive) in enumerate(claims) if state == "running" and alive and set(owners) & set(wanted)]
    return open_, take, blockers


def explore(config):
    participants = [tuple(sorted(set(w))) for w in config["participants"]]
    size, tail, budget, broken = config["chunk"], config["tail"], config["crashes"], config.get("broken")
    count = len(participants)
    start = ((), tuple(("idle", -1) for _ in range(count)), budget)
    seen = {start}
    queue = deque([start])
    violations, terminals = [], 0
    while queue:
        claims, status, crashes = queue.popleft()
        live = [set(owners) for owners, state, alive in claims if state == "done" or (state == "running" and alive)]
        for i in range(len(live)):
            for j in range(i + 1, len(live)):
                if live[i] & live[j]:
                    violations.append(["overlap", list(map(sorted, (live[i], live[j])))])
        moves = []
        for p in range(count):
            phase, token = status[p]
            if phase == "idle":
                moves.append(("admit", p))
            elif phase == "computing":
                moves.append(("finish", p))
                if crashes > 0:
                    moves.append(("crash", p))
        if not moves:
            terminals += 1
            done = set()
            for owners, state, alive in claims:
                if state == "done":
                    done.update(owners)
            for p in range(count):
                if status[p][0] == "finished" and not set(participants[p]) <= done:
                    violations.append(["undone", p, sorted(set(participants[p]) - done)])
            continue
        for kind, p in moves:
            nclaims, nstatus, ncrashes = list(claims), list(status), crashes
            if kind == "admit":
                pruned = [(owners, state, alive) for owners, state, alive in nclaims if not (state == "running" and not alive)]
                remap = {}
                for at, (owners, state, alive) in enumerate(nclaims):
                    if not (state == "running" and not alive):
                        remap[at] = len(remap)
                nclaims = pruned
                nstatus = [(phase, remap.get(token, -1) if phase == "computing" else token) for phase, token in nstatus]
                open_, take, blockers = select(participants[p], nclaims, size, tail, broken)
                if not open_:
                    nstatus[p] = ("finished", -1) if not blockers else ("idle", -1)
                    if blockers:
                        if tuple(nclaims) == claims and tuple(nstatus) == status:
                            continue
                else:
                    nclaims.append((tuple(take), "running", True))
                    nstatus[p] = ("computing", len(nclaims) - 1)
            elif kind == "finish":
                token = status[p][1]
                owners, state, alive = nclaims[token]
                nclaims[token] = (owners, "done", True)
                nstatus[p] = ("idle", -1)
            else:
                token = status[p][1]
                owners, state, alive = nclaims[token]
                nclaims[token] = (owners, "running", False)
                nstatus[p] = ("crashed", -1)
                ncrashes -= 1
            key = (tuple(sorted(nclaims)), tuple(nstatus), ncrashes)
            if key not in seen:
                seen.add(key)
                queue.append((tuple(nclaims), tuple(nstatus), ncrashes))
    return {"states": len(seen), "terminals": terminals, "violations": violations[:5], "violationCount": len(violations)}


if __name__ == "__main__":
    command = sys.argv[1]
    data = json.loads(sys.stdin.read())
    if command == "chunk":
        out = chunk(data)
    elif command == "slots":
        out = slots(data)
    elif command == "claim":
        out = {"accepted": claim_accepted(data["claim"])}
    elif command == "explore":
        out = explore(data)
    else:
        raise SystemExit("unknown command " + command)
    sys.stdout.write(json.dumps(out, ensure_ascii=False))

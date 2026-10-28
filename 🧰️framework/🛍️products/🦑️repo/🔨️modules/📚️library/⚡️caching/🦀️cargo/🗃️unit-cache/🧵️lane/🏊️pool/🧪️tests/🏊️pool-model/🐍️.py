"""🏊 Independent Python model of the Cargo lane pool (standard library only, no code shared with the TypeScript reference).
usage: 🐍️.py plan <cases.json>   reads {"acquire":[...],"trim":[...],"quota":[...]} and prints one verdict per case as JSON
       🐍️.py sweep               explores every interleaving of small pools and prints the counts per regime

Rules. Acquire: take the newest free lane built for the same crate set (warm); else a NEW lane while the pool holds fewer than `maximumLanes` lanes and its recorded bytes are within `quotaBytes`; else the least recently used free lane (lru); else a NEW lane while fewer than `maximumLiveLanes` lanes exist; else wait (busy). Trim: remove idle lanes, least recently used first, until the pool holds at most `maximumLanes` lanes and at most `quotaBytes` recorded bytes; a claimed lane is never removed. Quota: floor(min(capacity, machine quota or capacity) * laneShare)."""
import json, sys
from fractions import Fraction


def recorded(lanes):
    return sum(l["bytes"] or 0 for l in lanes)


def plan_acquire(case):
    lanes, key, b = case["lanes"], case["crateKey"], case["bounds"]
    free = [l for l in lanes if not l["claimed"]]
    warm = sorted([l for l in free if l["crateKey"] == key], key=lambda l: (-l["lastUsedMs"], l["id"]))
    if warm:
        return {"kind": "warm", "id": warm[0]["id"]}
    if len(lanes) < b["maximumLanes"] and recorded(lanes) <= b["quotaBytes"]:
        return {"kind": "new"}
    cold = sorted(free, key=lambda l: (l["lastUsedMs"], l["id"]))
    if cold:
        return {"kind": "lru", "id": cold[0]["id"]}
    return {"kind": "new"} if len(lanes) < b["maximumLiveLanes"] else {"kind": "busy"}


def plan_trim(case):
    lanes, b = case["lanes"], case["bounds"]
    count, total, victims = len(lanes), recorded(lanes), []
    for lane in sorted([l for l in lanes if not l["claimed"]], key=lambda l: (l["lastUsedMs"], l["id"])):
        if count <= b["maximumLanes"] and total <= b["quotaBytes"]:
            break
        victims.append(lane["id"])
        count -= 1
        total -= lane["bytes"] or 0
    return victims


def quota(case):
    share = Fraction(case["laneShareNumerator"], case["laneShareDenominator"])
    machine = case["machineQuotaBytes"]
    basis = case["capacityBytes"] if machine is None else min(case["capacityBytes"], machine)
    return int(basis * share // 1)


def explore(config):
    """Every interleaving of the acquirers (read, claim, run, release) and at most one trimmer (read, then one delete per victim) over a pool of lanes. `claim`: a lane is taken or deleted only by an atomic test-and-set of its claim; `unguarded`: the trimmer deletes what its snapshot called idle, without the claim (the negative control)."""
    maximum, live, regime = config["maximumLanes"], config["maximumLiveLanes"], config["regime"]
    initial = tuple(sorted((i, -1, k) for i, k in enumerate(config["lanes"])))
    procs = config["keys"]
    trimmer = config["trimmer"]
    n = len(procs)

    def plan(snapshot, key):
        free = [l for l in snapshot if l[1] < 0]
        warm = sorted([l for l in free if l[2] == key], key=lambda l: (-l[3], l[0]))
        if warm:
            return ("warm", warm[0][0])
        if len(snapshot) < maximum:
            return ("new", None)
        cold = sorted(free, key=lambda l: (l[3], l[0]))
        if cold:
            return ("lru", cold[0][0])
        return ("new", None) if len(snapshot) < live else ("busy", None)

    def trim(snapshot):
        victims, count = [], len(snapshot)
        for l in sorted([l for l in snapshot if l[1] < 0], key=lambda l: (l[3], l[0])):
            if count <= maximum:
                break
            victims.append(l[0])
            count -= 1
        return tuple(victims)

    # lane = (id, holder, key, lastUsed); proc = (pc, snapshot-choice id/kind, ...); a global tick orders last use
    lanes0 = tuple((i, -1, k, 0) for (i, _h, k) in initial)
    proc0 = tuple(("read", None, None) for _ in procs)
    trim0 = ("off",) if not trimmer else ("read", ())
    start = (lanes0, proc0, trim0, 0, len(lanes0))
    seen, stack, edges = {start}, [start], {}
    violations, stuck, overlap, warm_hits = 0, 0, False, 0

    def moves(state):
        lanes, ps, tr, tick, fresh = state
        out = []
        by_id = {l[0]: l for l in lanes}
        for i in range(n):
            pc, a, b = ps[i]
            def setp(value, lanes_=lanes, tick_=tick, fresh_=fresh, tr_=tr):
                q = list(ps)
                q[i] = value
                return (lanes_, tuple(q), tr_, tick_, fresh_)
            if pc == "read":
                kind, ident = plan(lanes, procs[i])
                if kind == "busy":
                    continue
                out.append(setp(("claim", kind, ident)))
            elif pc == "claim":
                kind, ident = a, b
                if kind == "new":
                    new = (fresh, i, procs[i], tick)
                    out.append(setp(("run", fresh, kind), lanes_=tuple(sorted(lanes + (new,))), fresh_=fresh + 1, tick_=tick))
                else:
                    lane = by_id.get(ident)
                    if lane is not None and lane[1] < 0:
                        taken = tuple(sorted((x if x[0] != ident else (x[0], i, x[2], x[3])) for x in lanes))
                        out.append(setp(("run", ident, kind), lanes_=taken))
                    else:
                        out.append(setp(("read", None, None)))
            elif pc == "run":
                out.append(setp(("release", a, b)))
            elif pc == "release":
                released = tuple(sorted((x if x[0] != a else (x[0], -1, procs[i], tick + 1)) for x in lanes))
                out.append(setp(("done", None, None), lanes_=released, tick_=tick + 1))
        if tr[0] == "read":
            victims = trim(lanes)
            out.append((lanes, ps, ("delete", victims), tick, fresh))
        elif tr[0] == "delete":
            victims = tr[1]
            if not victims:
                out.append((lanes, ps, ("done",), tick, fresh))
            else:
                head, rest = victims[0], victims[1:]
                lane = by_id.get(head)
                if lane is None:
                    out.append((lanes, ps, ("delete", rest), tick, fresh))
                elif regime == "claim":
                    if lane[1] < 0:
                        out.append((tuple(x for x in lanes if x[0] != head), ps, ("delete", rest), tick, fresh))
                    else:
                        out.append((lanes, ps, ("delete", rest), tick, fresh))
                else:
                    out.append((tuple(x for x in lanes if x[0] != head), ps, ("delete", rest), tick, fresh))
        return out

    def finished(state):
        return all(p[0] == "done" for p in state[1]) and state[2][0] in ("off", "done")

    while stack:
        state = stack.pop()
        lanes, ps, tr, tick, fresh = state
        running = [p for p in ps if p[0] == "run"]
        if len(running) > 1:
            overlap = True
        held = {}
        for i, p in enumerate(ps):
            if p[0] in ("run", "release"):
                held[p[1]] = held.get(p[1], 0) + 1
        ids = {l[0]: l for l in lanes}
        for ident, count in held.items():
            lane = ids.get(ident)
            if lane is None or count > 1 or lane[1] < 0:
                violations += 1
        next_states = moves(state)
        edges[state] = next_states
        if not next_states and not finished(state):
            stuck += 1
        for nxt in next_states:
            if nxt not in seen:
                seen.add(nxt)
                stack.append(nxt)

    reverse = {}
    for s, outs in edges.items():
        for t in outs:
            reverse.setdefault(t, []).append(s)
    good = {s for s in seen if finished(s)}
    frontier = list(good)
    while frontier:
        t = frontier.pop()
        for s in reverse.get(t, []):
            if s not in good:
                good.add(s)
                frontier.append(s)
    unreachable = sum(1 for s in seen if s not in good)
    return {"states": len(seen), "violations": violations, "stuck": stuck, "unreachableDone": unreachable, "overlap": overlap}


def sweep():
    out = {}
    for regime in ("claim", "unguarded"):
        totals = {"configs": 0, "states": 0, "violations": 0, "stuck": 0, "unreachableDone": 0, "overlaps": 0}
        for keys in ([], ["a"], ["a", "a"], ["a", "b"], ["a", "b", "a"]):
            for lanes in ([], ["a"], ["a", "b"], ["b", "b"]):
                for maximum in (1, 2):
                    for trimmer in (False, True):
                        if not keys and not trimmer:
                            continue
                        r = explore({"maximumLanes": maximum, "maximumLiveLanes": maximum + 1, "regime": regime, "lanes": lanes, "keys": keys, "trimmer": trimmer})
                        totals["configs"] += 1
                        totals["states"] += r["states"]
                        totals["violations"] += r["violations"]
                        totals["stuck"] += r["stuck"]
                        totals["unreachableDone"] += r["unreachableDone"]
                        totals["overlaps"] += 1 if r["overlap"] else 0
        out[regime] = totals
    return out


if __name__ == "__main__":
    if sys.argv[1] == "plan":
        data = json.load(open(sys.argv[2]) if sys.argv[2] != "/dev/stdin" else sys.stdin)
        print(json.dumps({"acquire": [plan_acquire(c) for c in data["acquire"]], "trim": [plan_trim(c) for c in data["trim"]], "quota": [quota(c) for c in data["quota"]]}))
    elif sys.argv[1] == "sweep":
        print(json.dumps(sweep()))
    elif sys.argv[1] == "explore":
        print(json.dumps(explore(json.loads(sys.stdin.read()))))

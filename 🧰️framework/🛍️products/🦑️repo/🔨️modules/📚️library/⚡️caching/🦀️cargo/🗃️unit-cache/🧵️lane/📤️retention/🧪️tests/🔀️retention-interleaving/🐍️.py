"""🔀 Independent Python model of lane retention under concurrency (standard library only, no code shared with the TypeScript reference).
usage: 🐍️.py sweep   explores every interleaving of small pools and prints the totals per regime and release order as JSON

Processes take a lane (read, claim), write one receipt into it (run), then either publish it and release the lane (order `retain-first`, the rule) or release the lane and publish afterwards (order `release-first`, the negative control). One trimmer reads the pool and deletes its victims one by one: `claim` deletes only a lane nobody holds, `unguarded` deletes whatever it planned. A receipt is LOST when a lane is deleted while it still holds an unpublished receipt (or a receipt is written into a lane that no longer exists).
Pool rules (simplified identically to the pool model): acquire takes the newest free lane of the same crate set, else a new lane below `maximumLanes`, else the least recently used free lane, else a new lane below `live`, else waits; trim removes least recently used free lanes while more than `maximumLanes` exist.
"""
import json
import sys


def explore(config):
    maximum, live, regime, order = config["maximumLanes"], config["maximumLiveLanes"], config["regime"], config["order"]
    keys, n = config["keys"], len(config["keys"])

    def plan(lanes, key):
        free = [l for l in lanes if l[1] < 0]
        warm = sorted([l for l in free if l[2] == key], key=lambda l: (-l[3], l[0]))
        if warm:
            return ("warm", warm[0][0])
        if len(lanes) < maximum:
            return ("new", None)
        cold = sorted(free, key=lambda l: (l[3], l[0]))
        if cold:
            return ("lru", cold[0][0])
        return ("new", None) if len(lanes) < live else ("busy", None)

    def victims(lanes):
        out, count = [], len(lanes)
        for l in sorted([l for l in lanes if l[1] < 0], key=lambda l: (l[3], l[0])):
            if count <= maximum:
                break
            out.append(l[0])
            count -= 1
        return tuple(out)

    # lane = (id, holder, key, used, pending); proc = (pc, kind, id); state = (lanes, procs, trim, tick, fresh, shared, lost)
    lanes0 = tuple((i, -1, k, 0, ()) for i, k in enumerate(config["lanes"]))
    start = (lanes0, tuple(("read", None, None) for _ in keys), ("read", ()) if config["trimmer"] else ("off",), 0, len(lanes0), (), ())
    after_run = "retain" if order == "retain-first" else "release"

    def finished(state):
        return all(p[0] == "done" for p in state[1]) and state[2][0] in ("off", "done")

    def moves(state):
        lanes, ps, tr, tick, fresh, shared, lost = state
        out = []
        by_id = {l[0]: l for l in lanes}
        for i in range(n):
            pc, a, b = ps[i]

            def put(proc, lanes_=lanes, tick_=tick, fresh_=fresh, shared_=shared, lost_=lost):
                q = list(ps)
                q[i] = proc
                return (tuple(sorted(lanes_)), tuple(q), tr, tick_, fresh_, tuple(sorted(shared_)), tuple(sorted(lost_)))

            if pc == "read":
                kind, ident = plan(lanes, keys[i])
                if kind != "busy":
                    out.append(put(("claim", kind, ident)))
            elif pc == "claim":
                if a == "new":
                    out.append(put(("run", fresh, a), lanes_=lanes + ((fresh, i, keys[i], tick, ()),), fresh_=fresh + 1))
                else:
                    lane = by_id.get(b)
                    if lane is not None and lane[1] < 0:
                        out.append(put(("run", b, a), lanes_=tuple(x if x[0] != b else (x[0], i, x[2], x[3], x[4]) for x in lanes)))
                    else:
                        out.append(put(("read", None, None)))
            elif pc == "run":
                lane = by_id.get(a)
                if lane is None:
                    out.append(put((after_run, a, b), lost_=lost + (i,)))
                else:
                    out.append(put((after_run, a, b), lanes_=tuple(x if x[0] != a else (x[0], x[1], x[2], x[3], x[4] + (i,)) for x in lanes)))
            elif pc == "retain":
                lane = by_id.get(a)
                moved = lane[4] if lane is not None else ()
                nxt = "release" if order == "retain-first" else "done"
                out.append(put((nxt, a, b), lanes_=tuple(x if x[0] != a else (x[0], x[1], x[2], x[3], ()) for x in lanes), shared_=shared + moved))
            elif pc == "release":
                nxt = "done" if order == "retain-first" else "retain"
                out.append(put((nxt, a, b), lanes_=tuple(x if x[0] != a else (x[0], -1, keys[i], tick + 1, x[4]) for x in lanes), tick_=tick + 1))
        if tr[0] == "read":
            out.append((lanes, ps, ("delete", victims(lanes)), tick, fresh, shared, lost))
        elif tr[0] == "delete":
            if not tr[1]:
                out.append((lanes, ps, ("done",), tick, fresh, shared, lost))
            else:
                head, rest = tr[1][0], tr[1][1:]
                lane = by_id.get(head)
                if lane is None or (regime == "claim" and lane[1] >= 0):
                    out.append((lanes, ps, ("delete", rest), tick, fresh, shared, lost))
                else:
                    out.append((tuple(x for x in lanes if x[0] != head), ps, ("delete", rest), tick, fresh, shared, tuple(sorted(lost + lane[4]))))
        return out

    seen, stack = {start}, [start]
    lost_states = lost_final = stuck = 0
    while stack:
        state = stack.pop()
        if state[6]:
            lost_states += 1
        nxt = moves(state)
        if finished(state):
            everything = tuple(range(n))
            if state[6] or tuple(sorted(set(state[5]))) != everything:
                lost_final += 1
        elif not nxt:
            stuck += 1
        for s in nxt:
            if s not in seen:
                seen.add(s)
                stack.append(s)
    return {"states": len(seen), "lostStates": lost_states, "lostFinal": lost_final, "stuck": stuck}


def sweep(regime, order):
    totals = {"configs": 0, "states": 0, "lostStates": 0, "lostFinal": 0, "stuck": 0}
    for keys in (["a"], ["a", "a"], ["a", "b"], ["a", "b", "a"]):
        for lanes in ([], ["a"], ["a", "b"]):
            for maximum in (1, 2):
                result = explore({"maximumLanes": maximum, "maximumLiveLanes": maximum + 1, "regime": regime, "order": order, "lanes": lanes, "keys": keys, "trimmer": True})
                totals["configs"] += 1
                for name in ("states", "lostStates", "lostFinal", "stuck"):
                    totals[name] += result[name]
    return totals


if __name__ == "__main__":
    if sys.argv[1] != "sweep":
        raise SystemExit("usage: sweep")
    json.dump({"rule": sweep("claim", "retain-first"), "releaseFirst": sweep("claim", "release-first"), "unguarded": sweep("unguarded", "retain-first")}, sys.stdout)

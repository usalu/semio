"""⛓️ Independent Python oracle (standard library only) of the scheduler scenarios of the host capacity gate: jobs that hold a memory grant and ask to grow it, plain requests, other applications that hold memory. It models the contract, not the implementation, one second at a time:

1. grants that reached their end release their memory; a preempted grant is gone `preemptLatencySeconds` after its marker and queues again with the arrival it had;
2. growths of running grants are served before every request that starts new work, in the order they asked; the other requests by class (interactive before batch; a batch request that waited the aging time is interactive and, like every request past it, heads its class), then by arrival; a request is admitted when its need fits the room (the smaller of the memory the host holds free above its floor and the budget share left after the interactive reserve); a growth that declared a floor starts reduced (what the room carries, at least the floor) once it waited sixty seconds;
3. a request that does not fit and waited the bypass time reserves the line: when everything of ours that ended would carry it, no later request that starts new work is admitted this round; when other applications hold the memory (it could not fit even then), later requests are admitted only while nothing of ours runs;
4. when nothing was admitted and every running grant has waited `deadlockMilliseconds` for its growth, grants are preempted, batch before interactive, youngest first, as many as it takes for a waiting request of a grant that stays to fit what is spare plus what they release;
5. a request whose least need exceeds everything this host could give out even if all of ours ended is refused after `unreachableMilliseconds` of that.

`python3 🐍️.py` prints the event timeline [second, kind, id, megabytes] of every scenario of the fixture."""
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
FIX = json.load(open(os.path.join(HERE, "..", "🧫️fixtures", "🔣️.json"), encoding="utf-8"))
BASE = json.load(open(os.path.join(HERE, "..", "..", "🧫️fixtures", "🔣️.json"), encoding="utf-8"))["policy"]
ORDER = ["finish", "requeue", "admit", "grow", "preempt", "refuse"]
POLICY_MINIMUM = BASE["estimates"]["minimumMegabytes"]


def simulate(sc):
    mem = {**BASE["memory"], **FIX["memory"]}
    wait = {**BASE["wait"], **sc["wait"]}
    total, foreign = sc["totalMegabytes"], sc["foreignMegabytes"]
    floor = min(total, max(mem["floorMinimumMegabytes"], min(mem["floorMaximumMegabytes"], total * mem["floorBasisPoints"] // 10000)))
    reserve = min(total, max(mem["interactiveReserveMinimumMegabytes"], min(mem["interactiveReserveMaximumMegabytes"], total * mem["interactiveReserveBasisPoints"] // 10000)))
    cap = max(0, total - floor)
    budget = max(0, cap - reserve) * mem["budgetBasisPoints"] // 10000
    limit = cap * mem["budgetBasisPoints"] // 10000
    drained_room = total - foreign - floor
    possible = max(0, min(drained_room, limit))
    jobs = {j["id"]: {**j, "state": "pending", "resumed": False, "kill": None, "marked": False} for j in sc["jobs"]}
    plains = {r["id"]: {**r, "state": "pending", "end": None} for r in sc["requests"]}
    for s in sc["streams"]:
        for i in range(s["count"]):
            rid = "%s-%d" % (s["prefix"], i)
            plains[rid] = {"id": rid, "priority": s["priority"], "arriveSecond": s["firstSecond"] + i * s["everySeconds"], "megabytes": s["megabytes"], "workSeconds": s["workSeconds"], "state": "pending", "end": None}
    events = []
    since = {}

    def emit(t, kind, ident, mb=0):
        events.append([t, kind, ident, mb])

    def held(j):
        return j["grantMegabytes"] + (j["grown"] if j["state"] == "grown" else 0)

    for t in range(sc["horizonSeconds"]):
        for j in jobs.values():
            if j["state"] == "grown" and j["end"] <= t:
                j["state"] = "done"
                emit(t, "finish", j["id"], j["grantMegabytes"] + j["grown"])
        for p in plains.values():
            if p["state"] == "running" and p["end"] <= t:
                p["state"] = "done"
                emit(t, "finish", p["id"], p["megabytes"])
        for j in jobs.values():
            if j["kill"] is not None and j["kill"] <= t:
                j.update(state="queued", kill=None, marked=False, resumed=True)
                emit(t, "requeue", j["id"])
        for j in jobs.values():
            if j["state"] == "pending" and j["arriveSecond"] == t:
                j["state"] = "queued"
        for p in plains.values():
            if p["state"] == "pending" and p["arriveSecond"] == t:
                p["state"] = "queued"
        for j in jobs.values():
            if j["state"] == "running" and t >= j["admitAt"] + (0 if j["resumed"] else j["askAfterSeconds"]):
                j["state"] = "asked"
                j["askedAt"] = t
        used = sum(held(j) for j in jobs.values() if j["state"] in ("running", "asked", "grown")) + sum(p["megabytes"] for p in plains.values() if p["state"] == "running")
        rows = sum(1 + (j["state"] == "grown") for j in jobs.values() if j["state"] in ("running", "asked", "grown")) + sum(p["state"] == "running" for p in plains.values())
        room_a, room_b = total - foreign - floor - used, budget - used
        spare = max(0, min(room_a, room_b))
        growths = sorted([j for j in jobs.values() if j["state"] == "asked"], key=lambda j: (j["askedAt"], j["id"]))
        news = []
        for j in jobs.values():
            if j["state"] == "queued":
                news.append({"id": j["id"], "kind": "job", "priority": j["priority"], "arrive": j["arriveSecond"], "need": j["grantMegabytes"]})
        for p in plains.values():
            if p["state"] == "queued":
                news.append({"id": p["id"], "kind": "plain", "priority": p["priority"], "arrive": p["arriveSecond"], "need": p["megabytes"]})

        def key(w):
            waited = (t - w["arrive"]) * 1000
            aged = waited >= wait["agingMilliseconds"]
            return (0 if w["priority"] == "interactive" or aged else 1, 0 if aged else 1, w["arrive"], w["id"])

        queue = [{"id": j["id"], "kind": "growth", "arrive": j["askedAt"], "need": j["growthMegabytes"], "floor": j.get("floorMegabytes")} for j in growths] + sorted(news, key=key)
        blocked = reserved = False
        admitted = 0
        waiting = []
        for w in queue:
            growth = w["kind"] == "growth"
            need = min(w["need"], cap)
            waited = (t - w["arrive"]) * 1000
            if not growth and (blocked or (reserved and (rows > 0 or admitted > 0))):
                waiting.append(w)
                continue
            fits = need <= room_a and need <= room_b
            floor_mb = w.get("floor")
            relaxed = floor_mb is not None and floor_mb < w["need"] and (waited >= 60000 or min(drained_room, budget) < w["need"])
            room = min(room_a, room_b)
            if fits or (relaxed and room >= floor_mb):
                mb = need if fits else min(need, room)
                room_a -= mb
                room_b -= mb
                admitted += 1
                if growth:
                    j = jobs[w["id"]]
                    j.update(state="grown", grown=mb, end=t + j["workSeconds"])
                    emit(t, "grow", j["id"], mb)
                elif w["kind"] == "job":
                    j = jobs[w["id"]]
                    j.update(state="running", admitAt=t)
                    emit(t, "admit", j["id"], mb)
                else:
                    p = plains[w["id"]]
                    p.update(state="running", end=t + p["workSeconds"])
                    emit(t, "admit", p["id"], mb)
                continue
            waiting.append(w)
            if waited >= wait["bypassMilliseconds"]:
                if need <= min(drained_room, budget):
                    blocked = True
                else:
                    reserved = True
        limit_ms = wait.get("deadlockMilliseconds")
        if limit_ms is not None and admitted == 0:
            roots = [j for j in jobs.values() if j["state"] in ("running", "asked", "grown")] + [p for p in plains.values() if p["state"] == "running"]
            stalled = {j["id"] for j in growths if (t - j["askedAt"]) * 1000 >= limit_ms}
            if roots and all(r["id"] in stalled for r in roots):
                ranked = sorted(roots, key=lambda r: (0 if r["priority"] == "batch" else 1, -r["admitAt"], r["id"]))
                victims, released = [], 0
                for r in ranked:
                    victims.append(r["id"])
                    released += held(r)
                    if any(x["need"] <= spare + released for x in queue if not (x["kind"] == "growth" and x["id"] in victims)):
                        for v in victims:
                            if not jobs[v]["marked"]:
                                jobs[v].update(marked=True, kill=t + sc["preemptLatencySeconds"])
                                emit(t, "preempt", v)
                        break
        ms = wait.get("unreachableMilliseconds")
        for w in waiting:
            if w["kind"] == "growth":
                least = max(POLICY_MINIMUM, w.get("floor") or 0)
            elif w["kind"] == "plain" and plains[w["id"]].get("minimum"):
                least = max(POLICY_MINIMUM, w["need"])
            else:
                least = POLICY_MINIMUM
            if least > possible:
                since.setdefault(w["id"], t)
                if ms is not None and (t - since[w["id"]]) * 1000 >= ms:
                    emit(t, "refuse", w["id"])
                    (plains if w["id"] in plains else jobs)[w["id"]]["state"] = "refused"
            else:
                since.pop(w["id"], None)
    events.sort(key=lambda e: (e[0], ORDER.index(e[1]), e[2]))
    return events


print(json.dumps({sc["name"]: simulate(sc) for sc in FIX["scenarios"]}))

"""🐍️ Independent Python oracle (standard library only) of the starvation scenario of host admission: a heavy agent session holds 4000 MB and asks for 3500 MB more (over its session cap) while a light session streams small requests that keep the room saturated. The oracle models the contract, not the implementation: before the heavy request has waited `agingSeconds` the light stream is served first; from then on the heavy request is the head of the line, no later request is admitted and it is admitted the first step the room carries it. `python3 🐍️.py` prints the second at which the heavy request is admitted. The room arithmetic (floor, interactive reserve, session cap) is restated here from the fixture; it shares no code with the TypeScript implementation."""
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
FIX = json.load(open(os.path.join(HERE, "🧫️fixtures", "🔣️.json"), encoding="utf-8"))
POLICY = json.load(open(os.path.join(HERE, "..", "🧫️fixtures", "🔣️.json"), encoding="utf-8"))["policy"]["memory"]


def run():
    total = FIX["totalMegabytes"]
    floor = min(total, max(POLICY["floorMinimumMegabytes"], min(POLICY["floorMaximumMegabytes"], total * POLICY["floorBasisPoints"] // 10000)))
    mem = FIX["memory"]
    reserve = min(total, max(mem["interactiveReserveMinimumMegabytes"], min(mem["interactiveReserveMaximumMegabytes"], total * mem["interactiveReserveBasisPoints"] // 10000)))
    share = (total - floor) - reserve
    heavy, light = FIX["heavy"], FIX["light"]
    ends = list(light["initialEndSeconds"])
    queue = []
    for t in range(0, FIX["horizonSeconds"], FIX["stepSeconds"]):
        ends = [end for end in ends if end > t]
        if t >= light["firstArrivalSecond"] and (t - light["firstArrivalSecond"]) % light["arrivalEverySeconds"] == 0:
            queue.append(t)
        room = share - heavy["heldMegabytes"] - len(ends) * light["requestMegabytes"]
        if t >= FIX["agingSeconds"]:
            if room >= heavy["bigMegabytes"]:
                return {"admittedAtSecond": t}
            continue
        while queue and room >= light["requestMegabytes"]:
            queue.pop(0)
            ends.append(t + light["runSeconds"])
            room -= light["requestMegabytes"]
    return {"admittedAtSecond": None}


def run_foreign():
    total = FIX["totalMegabytes"]
    floor = min(total, max(POLICY["floorMinimumMegabytes"], min(POLICY["floorMaximumMegabytes"], total * POLICY["floorBasisPoints"] // 10000)))
    mem = FIX["memory"]
    reserve = min(total, max(mem["interactiveReserveMinimumMegabytes"], min(mem["interactiveReserveMaximumMegabytes"], total * mem["interactiveReserveBasisPoints"] // 10000)))
    share = (total - floor) - reserve
    wait = json.load(open(os.path.join(HERE, "..", "🧫️fixtures", "🔣️.json"), encoding="utf-8"))["policy"]["wait"]
    cfg = FIX["foreign"]
    big, small = cfg["bigMegabytes"], cfg["smallMegabytes"]
    live = [(end, -1) for end in cfg["initialEndSeconds"]]
    queue = []
    for t in range(0, FIX["horizonSeconds"], FIX["stepSeconds"]):
        live = [row for row in live if row[0] > t]
        if t % cfg["arrivalEverySeconds"] == 0:
            queue.append(t)
        foreign = cfg["lowMegabytes"] if t % cfg["periodSeconds"] == cfg["lowAtSecond"] else cfg["highMegabytes"]
        used = small * len(live)
        if big <= total - foreign - floor - used and big <= share - used:
            return {"admittedAtSecond": t}
        since = len([row for row in live if row[1] >= 0])
        drains = big <= total - foreign - floor and big <= share
        patient = t * 1000 >= wait["bypassMilliseconds"] or since >= wait["maximumBypasses"]
        blocked = patient and drains
        reserved = patient and not drains
        bypasses = since
        admitted = 0
        while queue:
            if blocked or (reserved and (len(live) > 0 or admitted > 0)):
                break
            used = small * len(live)
            if small > total - foreign - floor - used or small > share - used:
                break
            queue.pop(0)
            live.append((t + cfg["runSeconds"], t))
            admitted += 1
            if not patient and drains:
                bypasses += 1
                if bypasses >= wait["maximumBypasses"]:
                    blocked = True
    return {"admittedAtSecond": None}


print(json.dumps({**run(), "foreignAdmittedAtSecond": run_foreign()["admittedAtSecond"]}))

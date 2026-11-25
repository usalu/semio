"""🐍️ Independent Python oracle (standard library only) of the interactive reserve and the session cap of host admission: `python3 🐍️.py` prints, for every case of `🧫️.json`, the reserve, the session cap and the expected verdict and reason of every queued request; the session cap binds only while another session has a request waiting that the round has not admitted and that is itself servable under the cap (work-conserving). The floor, budget and room arithmetic is restated here from the authored policy; it shares no code with the TypeScript implementation."""
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
FIX = json.load(open(os.path.join(HERE, "🧫️fixtures", "🔣️.json"), encoding="utf-8"))
POLICY = json.load(open(os.path.join(HERE, "..", "🧫️fixtures", "🔣️.json"), encoding="utf-8"))["policy"]["memory"]


def run(case):
    total = case["totalMegabytes"]
    avail = case.get("availableMegabytes", FIX["host"]["availableMegabytes"])
    mem = FIX["memory"]
    floor = min(total, max(POLICY["floorMinimumMegabytes"], min(POLICY["floorMaximumMegabytes"], total * POLICY["floorBasisPoints"] // 10000)))
    reserve = 0
    cap_session = None
    if case["withPolicy"]:
        reserve = min(total, max(mem["interactiveReserveMinimumMegabytes"], min(mem["interactiveReserveMaximumMegabytes"], total * mem["interactiveReserveBasisPoints"] // 10000)))
        cap_session = total * mem["sessionCapBasisPoints"] // 10000
    cap = max(0, total - floor)
    committed = sum(g["reservedMegabytes"] for g in case["ledger"])
    unmanaged = max(0, reserve)
    share = max(0, cap - unmanaged)
    budget_room = share - committed
    observed_room = avail - floor
    taken = 0
    session_taken = {}
    for g in case["ledger"]:
        session_taken[g["session"]] = session_taken.get(g["session"], 0) + g["reservedMegabytes"]
    admitted = set()
    start = {}
    for g in case["ledger"]:
        start[g["session"]] = start.get(g["session"], 0) + g["reservedMegabytes"]
    order = sorted(range(len(case["queue"])), key=lambda i: (start.get(case["queue"][i]["session"], 0) if case["queue"][i]["session"] is not None else 0, i))
    out = [None] * len(case["queue"])
    for index in order:
        q = case["queue"][index]
        need = min(q["fixedMegabytes"], cap)
        s = q["session"]
        holding = session_taken.get(s, 0)
        contested = any(
            other["session"] is not None and other["session"] != s and at not in admitted
            and (session_taken.get(other["session"], 0) == 0 or session_taken.get(other["session"], 0) + min(other["fixedMegabytes"], cap) <= cap_session)
            for at, other in enumerate(case["queue"]) if cap_session is not None
        )
        if s is not None and cap_session is not None and contested and holding > 0 and holding + need > cap_session:
            out[index] = {"verdict": "wait", "reason": "session-cap"}
            continue
        if need <= observed_room - taken and need <= budget_room - taken:
            out[index] = {"verdict": "admit", "reason": "fits"}
            taken += need
            admitted.add(index)
            if s is not None:
                session_taken[s] = holding + need
        else:
            out[index] = {"verdict": "wait", "reason": "memory"}
    return {"reserve": reserve, "sessionCap": cap_session, "decisions": out}


print(json.dumps([run(c) for c in FIX["cases"]]))

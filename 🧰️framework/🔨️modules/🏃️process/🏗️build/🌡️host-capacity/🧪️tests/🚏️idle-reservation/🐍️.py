"""🐍️ Independent Python oracle (standard library only) of the idle-reservation scenario of host admission: a snapshot of the live gate where long-running cargo grants reserve 1536 MB and hold about a third of it while a graph plan waited 52 minutes for its minimum. The oracle states the contract, not the implementation: a settled retained grant is charged what it holds plus a margin (never a graph plan, never before it settled), the room is what the machine has above its floor beside those charges and within the share the host leaves for grants, the oldest request is the head of the line and takes the room when its minimum fits (all of it up to its full ask), and no other request passes it unless it fits beside the head's minimum. `python3 🐍️.py` prints, per case, the verdict and reservation of every waiter. It shares no code with the TypeScript implementation."""
import json
import math
import os

HERE = os.path.dirname(os.path.abspath(__file__))
FIX = json.load(open(os.path.join(HERE, "🔣️.json"), encoding="utf-8"))
POLICY = json.load(open(os.path.join(HERE, *[".."] * 6, "🛍️products", "🦑️repo", "🔨️modules", "📚️library", "🏗️build", "🔣️policy.json"), encoding="utf-8"))["hostCapacity"]
MEMORY = POLICY["memory"]


def room_of(case):
    total = FIX["totalMegabytes"]
    floor = min(MEMORY["floorMaximumMegabytes"], max(MEMORY["floorMinimumMegabytes"], total * MEMORY["floorBasisPoints"] // 10000))
    reserve = min(MEMORY["interactiveReserveMaximumMegabytes"], max(MEMORY["interactiveReserveMinimumMegabytes"], total * MEMORY["interactiveReserveBasisPoints"] // 10000))
    settle = case.get("settle", True)
    grants = [dict(row, ageSeconds=case.get("grantAgeSeconds", row["ageSeconds"]) if row.get("retain") and row["operation"] != "graph-plan" else row["ageSeconds"]) for row in FIX["grants"]] + case.get("extraGrants", [])
    unrealized = committed = observed_sum = 0
    for row in grants:
        held, reserved = row["observedMegabytes"], row["reservedMegabytes"]
        observed_sum += held
        ramping = row["ageSeconds"] * 1000 < 300000
        if row.get("retain"):
            charge = reserved
            if settle and row["operation"] != "graph-plan" and row["ageSeconds"] * 1000 >= MEMORY["settledAfterMilliseconds"] and held > 0:
                charge = min(reserved, max(math.ceil(held * MEMORY["retainedKeepBasisPoints"] / 10000), held + MEMORY["retainedKeepMegabytes"]))
            unrealized += max(0, charge - held)
            committed += max(charge, held)
        elif ramping:
            unrealized += max(0, reserved - held)
            committed += max(reserved, held)
        else:
            committed += held
    unmanaged = max(reserve, total - case["availableMegabytes"] - observed_sum)
    share = total - floor - unmanaged
    return min(case["availableMegabytes"] - floor - unrealized, share - committed)


def run():
    out = []
    by_id = {row["id"]: row for row in FIX["waiters"]}
    for case in FIX["cases"]:
        room = room_of(case)
        queue = [by_id[name] for name in case["waiters"]]
        queue.sort(key=lambda row: (0 if row["priority"] == "interactive" else 1, -row["waitedSeconds"]))
        head = max(queue, key=lambda row: row["waitedSeconds"] if not row.get("quiet") and row["waitedSeconds"] >= POLICY["wait"]["headOfLineMilliseconds"] / 1000 else -1)
        head = head if head["waitedSeconds"] >= POLICY["wait"]["headOfLineMilliseconds"] / 1000 and not head.get("quiet") else None
        decided, head_done = {}, head is None
        for row in queue:
            least = row.get("minimumMegabytes", row["fixedMegabytes"])
            if row is head:
                if room >= least:
                    taken = min(row["fixedMegabytes"], room)
                    decided[row["id"]] = {"verdict": "admit", "reservedMegabytes": taken}
                    room -= taken
                    head_done = True
                else:
                    decided[row["id"]] = {"verdict": "wait", "reservedMegabytes": 0}
                continue
            need = row["fixedMegabytes"] + (0 if head_done else head.get("minimumMegabytes", head["fixedMegabytes"]))
            if need <= room:
                decided[row["id"]] = {"verdict": "admit", "reservedMegabytes": row["fixedMegabytes"]}
                room -= row["fixedMegabytes"]
            else:
                decided[row["id"]] = {"verdict": "wait", "reservedMegabytes": 0}
        out.append({"name": case["name"], "decisions": decided})
    return out


print(json.dumps(run()))

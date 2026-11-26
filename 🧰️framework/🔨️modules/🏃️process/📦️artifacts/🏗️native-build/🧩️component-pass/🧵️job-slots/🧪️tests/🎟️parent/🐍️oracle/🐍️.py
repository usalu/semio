import json
import sys


def admit(case):
    baseline = case.get("baseline", 0)
    local = baseline + case["peak"] + case.get("localFootprint", 0)
    grant = case["grant"]
    fleet = case.get("fleet")
    if grant is None or local <= grant:
        return {"routes": ["local"]}
    routes = []
    slots = fleet and fleet["slots"]
    if slots == 0 and fleet.get("waitForCompatibleSlot"):
        slots = fleet.get("laterSlots")
    if (fleet and slots and not case.get("native") and
            not case.get("critical") and baseline + fleet["footprint"] <= grant):
        routes.append("fleet")
        if not fleet.get("fails"):
            return {"routes": routes}
    return {"routes": routes, "required": local}


json.dump([admit(case) for case in json.load(sys.stdin)], sys.stdout)

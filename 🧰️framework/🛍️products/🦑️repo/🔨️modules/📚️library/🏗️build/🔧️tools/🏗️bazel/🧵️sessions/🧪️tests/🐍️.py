"""🐍️ Independent oracle of the Bazel session decisions (mode `garbage`, the default; `plan`, `growth`, `reap`): reads {"policy": Policy, "rows": [{inventory, slots, services, now, held, pressure?}]} from stdin and prints the garbage of every row. It shares no code with the TypeScript planner."""
import json
import sys

DAY = 86400000


def plan(row, policy):
    retention, pressure_policy, servers = policy["retention"], policy["pressure"], policy["servers"]
    valid = {"s%d" % i for i in range(row["slots"])} | {"d%d" % i for i in range(row["services"])}
    valid |= {"s%d" % i for i in range(row["slots"], max(row["slots"], servers["growthMaximum"]))}
    slots = [entry for entry in row["inventory"]["slots"] if entry["slot"] not in row["held"]]
    packages, bases, kinds = {}, {}, {}

    def newest_first(entry):
        return sorted(entry["packages"], key=lambda package: (-package["modifiedAt"], package["key"]))

    for entry in slots:
        slot = entry["slot"]
        if slot not in valid:
            if entry["outputBase"]:
                bases[slot] = "capacity"
            for package in entry["packages"]:
                packages[(slot, package["key"])] = "capacity"
            continue
        state = entry.get("state")
        if entry["outputBase"] and state is not None and row["now"] - state["usedAt"] > retention["outputBaseMaximumIdleDays"] * DAY:
            bases[slot] = "idle"
        for rank, package in enumerate(newest_first(entry)):
            if rank == 0 or (state is not None and package["key"] == state["key"]):
                continue
            if rank >= retention["packagesPerSlot"]:
                packages[(slot, package["key"])] = "count"
            elif row["now"] - package["modifiedAt"] > retention["packageMaximumAgeDays"] * DAY:
                packages[(slot, package["key"])] = "age"

    pressure = row.get("pressure")
    if pressure is not None and pressure["freeBytes"] < pressure["softBytes"]:
        need = pressure["targetBytes"] - pressure["freeBytes"]
        for entry in slots:
            need -= sum(package.get("bytes", 0) for package in entry["packages"] if (entry["slot"], package["key"]) in packages)
            if entry["slot"] in bases:
                need -= entry.get("outputBaseBytes", 0)
        candidates, idle = [], []
        for entry in slots:
            slot = entry["slot"]
            if slot not in valid:
                continue
            state = entry.get("state")
            protected = {package["key"] for package in newest_first(entry)[: pressure_policy["protectedPackagesPerSlot"]]}
            if state is not None:
                protected.add(state["key"])
            for package in entry["packages"]:
                young = row["now"] - package["modifiedAt"] < pressure_policy["packageGraceMinutes"] * 60000
                if package["key"] not in protected and (slot, package["key"]) not in packages and not young:
                    candidates.append((package["modifiedAt"], slot + package["key"], slot, package["key"], package.get("bytes", 0)))
            if entry["outputBase"] and state is not None and slot not in bases and row["now"] - state["usedAt"] >= pressure_policy["outputBaseMinimumIdleHours"] * 3600000:
                idle.append((state["usedAt"], slot, entry.get("outputBaseBytes", 0)))
        for _, _, slot, key, size in sorted(candidates):
            if need <= 0:
                break
            packages[(slot, key)] = "pressure"
            need -= size
        for _, slot, size in sorted(idle):
            if need <= 0:
                break
            bases[slot] = "pressure"
            need -= size
        extra = {"s%d" % i for i in range(row["slots"], max(row["slots"], servers["growthMaximum"]))}
        retired = []
        for entry in slots:
            slot = entry["slot"]
            state = entry.get("state")
            used = state["usedAt"] if state is not None else 0
            if slot not in extra or row["now"] - used < pressure_policy["growthIdleMinutes"] * 60000:
                continue
            left = [(package["key"], package.get("bytes", 0)) for package in entry["packages"] if (slot, package["key"]) not in packages]
            base = entry.get("outputBaseBytes", 0) if entry["outputBase"] and slot not in bases else None
            if left or base is not None:
                retired.append((used, slot, left, base))
        for _, slot, left, base in sorted(retired, key=lambda item: (item[0], item[1])):
            if need <= 0:
                break
            for key, size in left:
                packages[(slot, key)] = "growth"
                need -= size
            if base is not None:
                bases[slot] = "growth"
                need -= base

    temporaries = [
        {"slot": entry["slot"], "key": package["key"]}
        for entry in slots
        for package in entry["packages"]
        if package.get("temporary") and (entry["slot"], package["key"]) not in packages
    ]
    return {
        "packages": [{"slot": slot, "key": key, "reason": reason} for (slot, key), reason in sorted(packages.items(), key=lambda item: item[0][0] + item[0][1])],
        "outputBases": [{"slot": slot, "reason": reason} for slot, reason in sorted(bases.items())],
        "temporaries": sorted(temporaries, key=lambda item: item["slot"] + item["key"]),
        "foreign": sorted(row["inventory"]["foreign"]),
    }


def server_plan(row, policy):
    servers, host, memory = policy["servers"], row["host"], row["memory"]
    floor = min(memory["floorMaximumMegabytes"], max(memory["floorMinimumMegabytes"], host["memoryMegabytes"] * memory["floorBasisPoints"] // 10000))
    spare = max(0, host["memoryMegabytes"] - floor)
    budget = spare * servers["memoryShareBasisPoints"] // 10000
    by_memory, by_cores = budget // servers["residentMegabytes"], host["cores"] // servers["coresPerServer"]
    bounded = min(by_memory, by_cores, servers["maximum"])
    slots = max(servers["minimum"], bounded)
    limit = "floor" if bounded < servers["minimum"] else "memory" if bounded == by_memory else "cores" if bounded == by_cores else "policy"
    overhead = servers["residentMegabytes"] - servers["heapMegabytes"]
    heap = servers["heapMegabytes"] if by_memory >= 1 else max(servers["minimumHeapMegabytes"], min(servers["heapMegabytes"], budget - overhead))
    return {"slots": slots, "services": servers["serviceMaximum"], "heapMegabytes": heap, "residentMegabytes": heap + overhead, "budgetMegabytes": budget, "cpuPerServer": max(1, host["cores"] // slots), "memoryPerServerMegabytes": max(256, (spare - budget) // slots), "limit": limit}


def growth(row, policy):
    servers = policy["servers"]
    if row["kind"] != "build":
        return "wait" if row["used"] >= row["services"] else "base"
    if row["used"] < row["slots"]:
        return "base"
    allowance = row["idle"] // servers["growthSpareMegabytes"]
    extra = servers["growthMaximum"] - row["slots"]
    if row["growth"] and row["idle"] >= servers["growthSpareMegabytes"] and row["image"] >= servers["growthImageFreeBytes"] and row["busyGrowth"] < min(allowance, extra):
        return "growth"
    return "wait"


def reap(row, policy):
    held = sum(1 for entry in row["rows"] if entry["held"])
    idle = sorted((entry for entry in row["rows"] if entry["live"] and not entry["held"]), key=lambda entry: (-entry["usedAt"], entry["slot"]))
    keep = 0 if row.get("headroom") is not None and row["headroom"] < 0 else max(0, row["budget"] - held)
    return [entry["slot"] for entry in reversed(idle[keep:])]


request = json.load(sys.stdin)
mode = {"plan": server_plan, "growth": growth, "reap": reap}.get(request.get("mode", "garbage"), plan)
print(json.dumps([mode(row, request["policy"]) for row in request["rows"]]))

"""🐍️ Independent oracle of the Bazel session garbage plan: reads {"policy": Policy, "rows": [{inventory, slots, services, now, held, pressure?}]} from stdin and prints the garbage of every row. It shares no code with the TypeScript planner."""
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


request = json.load(sys.stdin)
print(json.dumps([plan(row, request["policy"]) for row in request["rows"]]))

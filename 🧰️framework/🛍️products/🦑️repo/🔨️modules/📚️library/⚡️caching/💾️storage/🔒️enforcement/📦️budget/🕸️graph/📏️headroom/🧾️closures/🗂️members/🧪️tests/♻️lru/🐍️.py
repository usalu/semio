"""Independent oracle of the least-recently-used retirement of optional graph cache groups (and, with the argument `entries`, of the units inside one group: a record header with its packs, a memory pack with its index, any other leaf alone; observation blocks belong to no unit; two tiers, each oldest first: the accelerator packs of every unit first, whole units only while that is not enough): groups without optional bytes are not candidates; candidates go oldest newest-write first (ties by name) while the cache holds more than the target; the caller's own groups, groups written within the protected idle time and groups whose writer is busy stay. Reads {"cases"} from the file in argv[1] and prints, per case, the sorted retired groups and the remaining bytes."""
import json
import re
import sys


def retire(case):
    groups = sorted((g for g in case["groups"] if g["bytes"] > 0), key=lambda g: (g["newest"], g["group"]))
    remaining, retired = sum(g["bytes"] for g in groups), []
    for g in groups:
        if remaining <= case["targetBytes"]:
            break
        if g["group"] in case["exclude"] or case["now"] - g["newest"] < case["protectedMs"] or g["group"] in case["busy"]:
            continue
        retired.append(g["group"])
        remaining -= g["bytes"]
    return {"retired": sorted(retired), "remainingBytes": max(0, remaining)}


def unit(name):
    if re.search(r"(^|/)ob/", name):
        return None
    header = re.match(r"((?:np|ip|rr)/[a-f0-9]{24}\.json)(\.|$)", name)
    if header:
        return header.group(1)
    memory = re.match(r"(mr/[0-9a-z]+-[0-9]+-[a-f0-9]{16})\.(pack|idx)", name)
    return memory.group(1) if memory else name


def accelerator(name):
    return re.search(r"\.records/|^mr/", name) is not None


def retire_entries(case):
    units = {}
    for leaf in case["leaves"]:
        key = unit(leaf["name"])
        if key is None:
            continue
        row = units.setdefault(key, {"key": key, "newest": 0, "fast": 0, "rest": 0, "has_rest": False, "has_fast": False})
        row["newest"] = max(row["newest"], leaf["newest"])
        if accelerator(leaf["name"]):
            row["fast"] += leaf["bytes"]
            row["has_fast"] = True
        else:
            row["rest"] += leaf["bytes"]
            row["has_rest"] = True
    remaining = sum(leaf["bytes"] for leaf in case["leaves"]) + case["otherBytes"]
    order = sorted(units.values(), key=lambda r: (r["newest"], r["key"]))
    shed, retired = [], []
    for row in order:
        if remaining <= case["targetBytes"]:
            break
        if not row["has_fast"] or case["now"] - row["newest"] < case["protectedMs"]:
            continue
        shed.append(row["key"])
        remaining -= row["fast"]
    for row in order:
        if remaining <= case["targetBytes"]:
            break
        if not row["has_rest"] or case["now"] - row["newest"] < case["protectedMs"]:
            continue
        retired.append(row["key"])
        remaining -= row["rest"]
    return {"shed": sorted(shed), "retired": sorted(retired), "remainingBytes": max(0, remaining)}


handler = retire_entries if len(sys.argv) > 2 and sys.argv[2] == "entries" else retire
print(json.dumps([handler(case) for case in json.load(open(sys.argv[1], encoding="utf8"))["cases"]]))

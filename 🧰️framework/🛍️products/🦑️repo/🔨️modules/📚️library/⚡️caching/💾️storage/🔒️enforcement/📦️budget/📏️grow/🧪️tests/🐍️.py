"""Independent oracle of `repo cache grow`: the target size of a growth (declared size and rule maximum bound it, the host volume gives only the unreserved share of its free bytes, whole MiB) and the re-stamping of the storage journal (parentIdentity size and mount device, dev fields of the old mount device). Reads {"cases"} from argv[2] and prints one answer per case; argv[1] selects `target` or `restamp`."""
import json
import sys

MIB = 1048576


def target(case):
    declared = min(case["declaredBytes"], case["ruleMaximumBytes"])
    allowed = case["hostFreeBytes"] * (100 - case["hostFreeReservePercent"]) // 100
    current, requested = case["currentBytes"], case.get("requestedBytes")
    if requested is not None:
        if requested % MIB:
            return {"refused": "misaligned"}
        if requested > declared:
            return {"refused": "above-declared"}
        if requested <= current:
            return {"refused": "nothing-to-grow"}
        if requested - current > allowed:
            return {"refused": "host-free"}
        return {"targetBytes": requested}
    if declared <= current:
        return {"refused": "nothing-to-grow"}
    value = min(declared, current + allowed) // MIB * MIB
    return {"refused": "host-free"} if value <= current else {"targetBytes": value}


def walk(value, key, old, new):
    if isinstance(value, list):
        return [walk(item, "", old, new) for item in value]
    if isinstance(value, dict):
        return {name: walk(item, name, old, new) for name, item in value.items()}
    if key == "parentIdentity" and isinstance(value, str):
        try:
            row = json.loads(value)
        except ValueError:
            return value
        if isinstance(row, list) and len(row) >= 4:
            if row[2] == old["size"]:
                row[2] = new["size"]
            if old["device"] is not None and new["device"] is not None and row[3] == old["device"]:
                row[3] = new["device"]
            return json.dumps(row, separators=(",", ":"))
        return value
    if key == "dev" and old["device"] is not None and new["device"] is not None:
        if value == old["device"] and not isinstance(value, bool):
            return new["device"]
        if value == str(old["device"]):
            return str(new["device"])
    return value


def restamp(case):
    out = []
    for line in case["lines"]:
        out.append(json.dumps(walk(json.loads(line), "", case["from"], case["to"]), separators=(",", ":"), ensure_ascii=False))
    return out


handler = {"target": target, "restamp": restamp}[sys.argv[1]]
print(json.dumps([handler(case) for case in json.load(open(sys.argv[2], encoding="utf8"))["cases"]], ensure_ascii=False))

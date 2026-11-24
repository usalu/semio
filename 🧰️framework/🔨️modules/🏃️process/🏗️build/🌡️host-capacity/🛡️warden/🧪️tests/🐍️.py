"""🐍️ Independent Python oracle (standard library only) of the host warden: the decision law of one tick (guard, danger and resume lines, overdue suspensions, cancels, suspends, calm hysteresis) and the classification of a process table into subjects (roots, protected paths, never patterns, kinds, megabytes, owner label, ids). It reads the policy and the pattern tables as data from the schema and the cases from the fixtures; it shares no code with the TypeScript implementation.
`python3 🐍️.py --decide` prints the decisions of every case as JSON, `--classify` the subjects of every classification case, `--write` stores both as `expect` in the fixtures, `--check` fails when the stored expectations are stale."""
import hashlib
import json
import os
import re
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
SCHEMA = json.load(open(os.path.join(HERE, "..", "🧬️schema", "🔣️.json"), encoding="utf-8"))["definitions"]
FIXTURES = os.path.join(HERE, "🧫️fixtures", "🔣️.json")
POLICY = SCHEMA["Policy"]["const"]
KINDS = SCHEMA["Kinds"]["const"]
ROOTS = SCHEMA["Roots"]["const"]
NEVER = SCHEMA["Never"]["const"]


def order_key(subject):
    return (subject["rank"], subject["elapsedSeconds"], -subject["megabytes"], subject["pid"])


def decide(policy, host, subjects, previous):
    total = host["totalMegabytes"]
    avail = host["availableMegabytes"]
    guard = min(policy["guardMaximumMegabytes"], max(policy["guardMinimumMegabytes"], total * policy["guardBasisPoints"] // 10000))
    danger = min(policy["dangerMaximumMegabytes"], max(policy["dangerMinimumMegabytes"], total * policy["dangerBasisPoints"] // 10000))
    resume = max(guard, total * policy["resumeBasisPoints"] // 10000)
    swap = host["swapOutMegabytesPerSecond"]
    swap_worked = swap is not None and swap >= policy["swapActiveMegabytesPerSecond"]
    if avail < danger or host["pressure"] == "critical":
        level = "danger"
    elif avail < guard or ((swap_worked or host["pressure"] == "warn") and avail < resume):
        level = "guard"
    else:
        level = "calm"

    actions = []
    acted = set()

    def count(name):
        return sum(1 for a in actions if a["action"] == name)

    overdue = sorted([s for s in subjects if s["suspended"] and s["suspendedSeconds"] >= policy["suspendMaximumSeconds"]], key=order_key)
    cancelled = []
    if level != "calm":
        for s in overdue:
            if s["canCancel"] and count("cancel") < policy["maximumCancelsPerTick"]:
                actions.append({"action": "cancel", "id": s["id"], "reason": "overdue"})
                acted.add(s["id"])
                cancelled.append(s)
    for s in overdue:
        if not s["canCancel"]:
            actions.append({"action": "resume", "id": s["id"], "reason": "overdue"})
            acted.add(s["id"])

    if level == "danger":
        deficit = max(guard - avail, 1)
        freed = sum(s["megabytes"] for s in cancelled)
        candidates = [s for s in subjects if s["canCancel"] and s["id"] not in acted]
        candidates.sort(key=lambda s: (0 if s["suspended"] else 1,) + order_key(s))
        for s in candidates:
            if freed >= deficit or count("cancel") == policy["maximumCancelsPerTick"]:
                break
            actions.append({"action": "cancel", "id": s["id"], "reason": "danger"})
            acted.add(s["id"])
            freed += s["megabytes"]

    if level in ("guard", "danger"):
        deficit = max(guard - avail, 1)
        every_running = sorted([s for s in subjects if not s["suspended"]], key=order_key)
        keep = policy["keepRunning"]
        kept = set(s["id"] for s in every_running[len(every_running) - keep:]) if keep > 0 else set()
        covered = sum(s["megabytes"] for s in subjects if s["suspended"] and s["id"] not in acted)
        for s in [r for r in every_running if r["id"] not in acted]:
            if covered >= deficit or count("suspend") == policy["maximumSuspendsPerTick"]:
                break
            if s["id"] in kept or not s["canSuspend"]:
                continue
            actions.append({"action": "suspend", "id": s["id"], "reason": level})
            acted.add(s["id"])
            covered += s["megabytes"]

    if level == "calm":
        calm_ticks = (previous["calmTicks"] if previous["level"] == "calm" else 0) + 1
    else:
        calm_ticks = 0
    if level == "calm" and calm_ticks >= policy["calmTicks"] and avail >= resume:
        waiting = sorted([s for s in subjects if s["suspended"] and s["id"] not in acted], key=order_key)
        if waiting:
            actions.append({"action": "resume", "id": waiting[-1]["id"], "reason": "calm"})

    return {"level": level, "calmTicks": calm_ticks, "hold": level != "calm", "guardMegabytes": guard, "dangerMegabytes": danger, "resumeMegabytes": resume, "actions": actions}


def owner_label(command, prefixes):
    space = command.find(" ")
    executable = command if space < 0 else command[:space]
    rest = "" if space < 0 else command[space:]
    text = executable[executable.rfind("/") + 1:] + rest
    for prefix in sorted([p for p in prefixes if p], key=len, reverse=True):
        text = text.replace(prefix, "")
    return text[:96]


def classify(rows, tables, options):
    roots = tables["roots"]
    never = tables["never"]
    kinds = tables["kinds"]
    protect = options.get("protect", [])
    within = options.get("within")
    prefixes = options.get("prefixes", [])
    now = options.get("nowMs")
    since = options.get("suspendedSince", {})
    by_pid = {r["pid"]: r for r in rows}

    def chain(row):
        links = []
        seen = set()
        current = row
        while current is not None and current["pid"] not in seen and len(links) < 256:
            seen.add(current["pid"])
            links.append(current)
            current = by_pid.get(current["parent"])
        return links

    def matches(patterns, command):
        return any(re.search(p, command) for p in patterns)

    def root_of(row):
        for link in chain(row):
            if matches(roots, link["command"]):
                return link
        return None

    subjects = {}
    for row in rows:
        if root_of(row) is None:
            continue
        if any(fragment in link["command"] for link in chain(row) for fragment in protect):
            continue
        if within is not None and within not in [link["pid"] for link in chain(row)[1:]]:
            continue
        if matches(never, row["command"]):
            continue
        kind = next((k for k in kinds if re.search(k["pattern"], row["command"])), None)
        if kind is not None:
            subjects[row["pid"]] = (row, kind)

    kilobytes = {}
    for row in rows:
        for link in chain(row):
            if link["pid"] in subjects:
                kilobytes[link["pid"]] = kilobytes.get(link["pid"], 0) + row["rssKilobytes"]
                break

    result = []
    for pid in sorted(subjects):
        row, kind = subjects[pid]
        ident = "%d.%s" % (pid, hashlib.sha256(row["start"].encode("utf-8")).hexdigest()[:8])
        suspended_seconds = 0
        if row["stopped"] and ident in since and now is not None:
            suspended_seconds = max(0, (now - since[ident]) // 1000)
        top = root_of(row)
        result.append({
            "id": ident, "pid": pid, "parent": row["parent"], "kind": kind["name"], "rank": kind["rank"],
            "elapsedSeconds": row["elapsedSeconds"], "megabytes": kilobytes.get(pid, 0) // 1024,
            "suspended": row["stopped"], "suspendedSeconds": suspended_seconds,
            "canSuspend": "suspend" in kind["actions"], "canCancel": "cancel" in kind["actions"],
            "start": row["start"], "owner": owner_label(top["command"], prefixes), "scope": kind["scope"], "signal": kind["signal"],
        })
    return result


def default_tables():
    return {"roots": ROOTS, "never": NEVER, "kinds": KINDS}


def decisions(fixture):
    out = []
    for case in fixture["decisions"]:
        policy = dict(POLICY)
        policy.update(case.get("policy", {}))
        out.append(decide(policy, case["host"], case["subjects"], case["previous"]))
    return out


def classifications(fixture):
    out = []
    for case in fixture["classifications"]:
        tables = case.get("tables") or default_tables()
        out.append(classify(case["rows"], tables, case.get("options", {})))
    return out


def main():
    mode = sys.argv[1] if len(sys.argv) > 1 else "--check"
    fixture = json.load(open(FIXTURES, encoding="utf-8"))
    if mode == "--decide":
        print(json.dumps(decisions(fixture), ensure_ascii=False))
    elif mode == "--classify":
        print(json.dumps(classifications(fixture), ensure_ascii=False))
    elif mode in ("--write", "--check"):
        for case, expect in zip(fixture["decisions"], decisions(fixture)):
            if mode == "--write":
                case["expect"] = expect
            elif case.get("expect") != expect:
                sys.exit("stale decision expectation: " + case["name"])
        for case, expect in zip(fixture["classifications"], classifications(fixture)):
            if mode == "--write":
                case["expect"] = expect
            elif case.get("expect") != expect:
                sys.exit("stale classification expectation: " + case["name"])
        if mode == "--write":
            with open(FIXTURES, "w", encoding="utf-8") as handle:
                json.dump(fixture, handle, ensure_ascii=False, indent=1)
                handle.write("\n")
    else:
        sys.exit("usage: 🐍️.py --decide|--classify|--write|--check")


if __name__ == "__main__":
    main()

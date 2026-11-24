"""🐍️ Independent Python oracle (standard library only) of the host warden: the decision law of one tick (guard, danger and resume lines, the headroom of available memory plus swap that can still grow, overdue suspensions, cancels, suspends, calm hysteresis), the classification of a process table into subjects (roots, identities, protected paths, never patterns, kinds, megabytes, owner label, ids), the discovery of workspace roots and the reading of swap facts from platform command text. It reads the policy and the pattern tables as data from the schema and the cases from the fixtures; it shares no code with the TypeScript implementation.
`python3 🐍️.py --decide` prints the decisions of every case as JSON, `--classify` the subjects of every classification case, `--discover` the workspaces of every discovery case, `--swap` the swap facts of every swap case, `--write` stores all as `expect` in the fixtures, `--check` fails when the stored expectations are stale."""
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
IDENTITIES = SCHEMA["Identities"]["const"]
ANCHORS = SCHEMA["Anchors"]["const"]
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
    head_guard = min(policy["headroomGuardMaximumMegabytes"], max(policy["headroomGuardMinimumMegabytes"], total * policy["headroomGuardBasisPoints"] // 10000))
    head_danger = min(policy["headroomDangerMaximumMegabytes"], max(policy["headroomDangerMinimumMegabytes"], total * policy["headroomDangerBasisPoints"] // 10000))
    swap_free = host.get("swapFreeMegabytes")
    if swap_free is None:
        room = None
        headroom = None
    else:
        growable = max(0, (host.get("swapVolumeFreeMegabytes") or 0) - policy["swapDiskReserveMegabytes"])
        room = min(swap_free + growable, total * policy["swapRoomMaximumBasisPoints"] // 10000)
        headroom = avail + room
    if avail < danger or host["pressure"] == "critical" or (headroom is not None and headroom < head_danger):
        level = "danger"
    elif avail < guard or (headroom is not None and headroom < head_guard) or ((swap_worked or host["pressure"] == "warn") and avail < resume):
        level = "guard"
    else:
        level = "calm"
    deficit_all = max(guard - avail, 0 if headroom is None else head_guard - headroom, 1)

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
        deficit = deficit_all
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
        deficit = deficit_all
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

    return {"level": level, "calmTicks": calm_ticks, "hold": level != "calm", "guardMegabytes": guard, "dangerMegabytes": danger, "resumeMegabytes": resume, "swapRoomMegabytes": room, "headroomMegabytes": headroom, "headroomGuardMegabytes": head_guard, "headroomDangerMegabytes": head_danger, "actions": actions}


def owner_label(command, prefixes):
    space = command.find(" ")
    executable = command if space < 0 else command[:space]
    rest = "" if space < 0 else command[space:]
    text = executable[executable.rfind("/") + 1:] + rest
    for prefix in sorted([p for p in prefixes if p], key=len, reverse=True):
        text = text.replace(prefix, "")
    return text[:96]


def escape(text):
    return "".join("\\" + c if c in "\\^$.*+?()[]{}|" else c for c in text)


def expand(source, workspaces):
    anchors = {"workspace": workspaces, "browsers": [t.replace("<workspace>", w) for t in ANCHORS["browsers"] for w in workspaces]}
    empty = []

    def swap(match):
        names = anchors[match.group(1)]
        if not names:
            empty.append(1)
            return ""
        return "(?:" + "|".join(escape(n) for n in names) + ")"

    text = re.sub("<(" + "|".join(ANCHORS["names"]) + ")>", swap, source)
    return None if empty else text


def discover(rows, seed):
    found = set(seed)
    for row in rows:
        match = re.search(ANCHORS["workspace"], row["command"])
        if match:
            found.add(match.group(1))
    return sorted(found)


def parse_swap(platform, texts):
    if platform == "darwin":
        usage = re.search(r"free = ([\d.]+)([KMGT])", texts.get("usage") or "")
        scale = {"K": 1 / 1024, "M": 1, "G": 1024, "T": 1048576}
        free = int(float(usage.group(1)) * scale[usage.group(2)]) if usage else None
        lines = (texts.get("disk") or "").strip().split("\n")
        volume = None
        if len(lines) > 1:
            columns = lines[-1].split()
            if len(columns) > 3 and columns[3].isdigit():
                volume = int(columns[3]) // 1024
        return {"swapFreeMegabytes": free, "swapVolumeFreeMegabytes": volume}
    if platform == "linux":
        def kib(name):
            match = re.search(r"^" + name + r":\s+(\d+) kB", texts.get("meminfo") or "", re.M)
            return int(match.group(1)) if match else None
        total = kib("SwapTotal")
        free = kib("SwapFree")
        return {"swapFreeMegabytes": None if total is None or free is None or total == 0 else free // 1024, "swapVolumeFreeMegabytes": None}
    return {"swapFreeMegabytes": None, "swapVolumeFreeMegabytes": None}


def classify(rows, tables, options):
    roots = tables["roots"]
    never = tables["never"]
    workspaces = options.get("workspaces", [])
    identities = [t for t in (expand(p, workspaces) for p in tables.get("identities", [])) if t is not None]
    kinds = [dict(k, pattern=t) for k, t in ((k, expand(k["pattern"], workspaces)) for k in tables["kinds"]) if t is not None]
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
        if root_of(row) is None and not matches(identities, row["command"]):
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
        top = root_of(row) or row
        result.append({
            "id": ident, "pid": pid, "parent": row["parent"], "kind": kind["name"], "rank": kind["rank"],
            "elapsedSeconds": row["elapsedSeconds"], "megabytes": kilobytes.get(pid, 0) // 1024,
            "suspended": row["stopped"], "suspendedSeconds": suspended_seconds,
            "canSuspend": "suspend" in kind["actions"], "canCancel": "cancel" in kind["actions"],
            "start": row["start"], "owner": owner_label(top["command"], prefixes), "scope": kind["scope"], "signal": kind["signal"],
        })
    return result


def default_tables():
    return {"roots": ROOTS, "identities": IDENTITIES, "never": NEVER, "kinds": KINDS}


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


def discoveries(fixture):
    return [discover(case["rows"], case.get("seed", [])) for case in fixture["discoveries"]]


def swaps(fixture):
    return [parse_swap(case["platform"], case["texts"]) for case in fixture["swaps"]]


def main():
    mode = sys.argv[1] if len(sys.argv) > 1 else "--check"
    fixture = json.load(open(FIXTURES, encoding="utf-8"))
    if mode == "--decide":
        print(json.dumps(decisions(fixture), ensure_ascii=False))
    elif mode == "--classify":
        print(json.dumps(classifications(fixture), ensure_ascii=False))
    elif mode == "--discover":
        print(json.dumps(discoveries(fixture), ensure_ascii=False))
    elif mode == "--swap":
        print(json.dumps(swaps(fixture), ensure_ascii=False))
    elif mode in ("--write", "--check"):
        for key, produce in (("discoveries", discoveries), ("swaps", swaps)):
            for case, expect in zip(fixture[key], produce(fixture)):
                if mode == "--write":
                    case["expect"] = expect
                elif case.get("expect") != expect:
                    sys.exit("stale %s expectation: %s" % (key, case["name"]))
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
        sys.exit("usage: 🐍️.py --decide|--classify|--discover|--swap|--write|--check")


if __name__ == "__main__":
    main()

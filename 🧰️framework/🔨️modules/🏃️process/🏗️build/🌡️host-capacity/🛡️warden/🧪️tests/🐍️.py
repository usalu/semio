"""🐍️ Independent Python oracle (standard library only) of the host warden: the decision law of one tick (runtime ceilings of compile units, guard, danger and resume lines, the headroom of available memory plus swap that can still grow, overdue suspensions, cancels, suspends, calm hysteresis), the classification of a process table into subjects (roots, identities, protected paths, never patterns, kinds, megabytes, owner label, ids), the discovery of workspace roots and the reading of swap facts from platform command text. It reads the policy and the pattern tables as data from the schema and the cases from the fixtures; it shares no code with the TypeScript implementation.
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
CEILINGS = json.load(open(os.path.join(HERE, "..", "..", "..", "..", "📦️artifacts", "🏗️native-build", "🎟️admission", "🧬️schema", "🔣️.json"), encoding="utf-8"))["definitions"]["Policy"]["const"]["unitCeiling"]


def flag_values(tokens, flag):
    out = []
    for i, token in enumerate(tokens):
        if token == flag:
            out.append(tokens[i + 1] if i + 1 < len(tokens) else "")
        elif token.startswith(flag + "="):
            out.append(token[len(flag) + 1:])
    return out


def ceiling_of(command, policy=None):
    policy = policy or CEILINGS
    tokens = command.split()
    names = flag_values(tokens, "--crate-name")
    if not names or not names[0]:
        return None
    crate = names[0]
    target = (flag_values(tokens, "--target") or [""])[0] or "host"
    types = [x for value in flag_values(tokens, "--crate-type") for x in value.split(",") if x] + (["bin"] if "--test" in tokens else [])
    cls = policy["classes"][policy["workerClass"]]
    final = bool({"cdylib", "bin", "dylib", "staticlib"} & set(types))
    base = cls["finalMegabytes"] if final else cls["libraryMegabytes"]
    entry = next((e for e in policy["carving"] if e["crate"] == crate and e["target"] == target), None) or next((f for f in policy.get("families", []) if f["target"] == target and re.fullmatch(f["pattern"], crate)), None)
    out = {"unit": crate + "|" + target, "kind": "final" if final else "library", "megabytes": max(entry["budgetMegabytes"], base) if entry else base, "listed": entry is not None}
    if entry:
        out["measuredMegabytes"] = entry["measuredMegabytes"]
    return out


def order_key(subject):
    from fractions import Fraction
    return (1 if subject.get("capCritical") else 0, Fraction(subject["elapsedSeconds"], max(1, subject["megabytes"])), subject["rank"], subject["pid"])


def swap_depth(policy, host):
    total = host["totalMegabytes"]
    used = host.get("swapUsedMegabytes")
    free = host.get("swapFreeMegabytes")
    compressed = host.get("compressedMegabytes")
    growth = max(0, (host.get("swapVolumeFreeMegabytes") or 0) - policy["swapDiskReserveMegabytes"])
    capacity = used + free + growth if used is not None and free is not None else 0

    def depth_of(value, whole, guard_bp, danger_bp):
        if value is None:
            return 0
        if value * 10000 >= whole * danger_bp:
            return 2
        return 1 if value * 10000 >= whole * guard_bp else 0

    lines = [
        ("swap-used", depth_of(used, total, policy["swapUsedGuardBasisPoints"], policy["swapUsedDangerBasisPoints"])),
        ("swap-share", depth_of(used, capacity, policy["swapShareGuardBasisPoints"], policy["swapShareDangerBasisPoints"]) if capacity else 0),
        ("compressed", depth_of(compressed, total, policy["compressedGuardBasisPoints"], policy["compressedDangerBasisPoints"])),
    ]
    depth = max(d for _, d in lines)
    source = next((n for n, d in lines if d == depth), None) if depth > 0 else None
    return depth, source, lines[2][1]


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
    depth, source, comp = swap_depth(policy, host)
    pressure = {"normal": 0, "warn": 1, "critical": 2}[host["pressure"]]
    cap_limit = total * policy["capMaximumBasisPoints"] // 10000

    def cap_of(s):
        return min(s["capMegabytes"], cap_limit)

    def over(s):
        return s.get("ceiling") is not None and s["canCancel"] and s["megabytes"] > s["ceiling"]["megabytes"]

    crossed = [s for s in subjects if s["canCancel"] and (over(s) or (s.get("capMegabytes") is not None and s.get("capCritical") and s["megabytes"] > cap_of(s)))]
    overage = sum(s["megabytes"] - (s["ceiling"]["megabytes"] if over(s) else cap_of(s)) for s in crossed)
    acute = swap_worked and (comp == 2 or (pressure >= 1 and comp >= 1 and previous["level"] != "calm"))
    severe = avail < danger or (headroom is not None and headroom < head_danger) or (depth == 2 and avail < guard) or acute
    if severe or pressure == 2 or comp == 2:
        level = "danger"
    elif avail < guard or (headroom is not None and headroom < head_guard) or pressure >= 1 or comp >= 1 or ((swap_worked or depth >= 1) and avail < resume):
        level = "guard"
    else:
        level = "calm"
    base = level
    if base == "calm" and crossed:
        level = "guard"
    trigger = None
    if level != "calm" and base == "calm":
        trigger = "cap"
    elif level != "calm":
        line_guard, line_head = (danger, head_danger) if level == "danger" else (guard, head_guard)
        if avail < line_guard:
            trigger = "available"
        elif headroom is not None and headroom < line_head:
            trigger = "headroom"
        elif level == "danger":
            if depth == 2 and avail < guard:
                trigger = source
            elif comp == 2:
                trigger = "compressed"
            else:
                trigger = "pressure"
        elif pressure == 1:
            trigger = "pressure"
        elif comp >= 1:
            trigger = "compressed"
        elif depth >= 1 and avail < resume and source is not None:
            trigger = source
        else:
            trigger = "swap"
    quiet = pressure < 2 and avail >= guard and not (headroom is not None and headroom < head_guard) and not swap_worked
    held = policy["pressureDeficitMegabytes"] if (acute or (not severe and (pressure >= 1 or comp >= 1))) else 0
    target = resume if depth >= 1 else guard
    deficit_all = max(target - avail, 0 if headroom is None else head_guard - headroom, held, overage, 1)

    def damped(s):
        if level == "danger" and severe:
            return False
        return s.get("suspensions", 0) >= policy["flapMaximumSuspensions"] or (s.get("resumedSeconds") is not None and s["resumedSeconds"] < policy["resumeHoldSeconds"])

    actions = []
    acted = set()
    capped = 0

    for s in sorted(subjects, key=order_key):
        if over(s):
            if level != "danger":
                actions.append({"action": "report", "id": s["id"], "reason": "ceiling"})
                continue
            actions.append({"action": "cancel", "id": s["id"], "reason": "ceiling"})
            acted.add(s["id"])
            capped += s["megabytes"]
    for s in sorted(subjects, key=order_key):
        if s.get("capMegabytes") is None or not s["canCancel"] or s["id"] in acted or over(s):
            continue
        cap = cap_of(s)
        if s["megabytes"] <= cap:
            continue
        if s.get("capCritical") and level != "danger":
            actions.append({"action": "report", "id": s["id"], "reason": "cap", "capMegabytes": cap})
            continue
        actions.append({"action": "cancel", "id": s["id"], "reason": "cap", "capMegabytes": cap})
        acted.add(s["id"])
        capped += s["megabytes"]

    def count(name):
        return sum(1 for a in actions if a["action"] == name and a["reason"] not in ("cap", "ceiling"))

    overdue = sorted([s for s in subjects if s["suspended"] and s["suspendedSeconds"] >= policy["suspendMaximumSeconds"] and s["id"] not in acted], key=order_key)
    cancelled = []
    if level != "calm" and not quiet:
        for s in overdue:
            if s["canCancel"] and (not s.get("capCritical") or severe) and count("cancel") < policy["maximumCancelsPerTick"]:
                actions.append({"action": "cancel", "id": s["id"], "reason": "overdue"})
                acted.add(s["id"])
                cancelled.append(s)
    for s in overdue:
        if not s["canCancel"] or (level != "calm" and quiet) or (s.get("capCritical") and not severe):
            actions.append({"action": "resume", "id": s["id"], "reason": "overdue"})
            acted.add(s["id"])

    if level == "danger" and severe and not quiet:
        deficit = deficit_all
        freed = capped + sum(s["megabytes"] for s in cancelled)
        def paused(s):
            return s["suspended"] and s["suspendedSeconds"] >= policy["pauseDeadlineSeconds"]

        pausing_failed = all(paused(s) for s in subjects if s["canSuspend"])
        candidates = [s for s in subjects if s["canCancel"] and s["id"] not in acted and (paused(s) if s["canSuspend"] else pausing_failed)]
        candidates.sort(key=lambda s: (0 if s["suspended"] else 1,) + order_key(s))
        for s in candidates:
            if freed >= deficit or count("cancel") == policy["maximumCancelsPerTick"]:
                break
            actions.append({"action": "cancel", "id": s["id"], "reason": "danger"})
            acted.add(s["id"])
            freed += s["megabytes"]

    if level in ("guard", "danger") and (not quiet or crossed):
        deficit = deficit_all
        every_running = sorted([s for s in subjects if not s["suspended"] and not (quiet and s.get("capCritical"))], key=lambda s: (1 if s.get("capCritical") else 0,) + order_key(s))
        keep = 0 if acute else policy["keepRunning"]
        kept = set(s["id"] for s in every_running[len(every_running) - keep:]) if keep > 0 else set()
        covered = sum(s["megabytes"] for s in subjects if s["suspended"] and s["id"] not in acted)
        for s in [r for r in every_running if r["id"] not in acted]:
            if covered >= deficit or count("suspend") == policy["maximumSuspendsPerTick"]:
                break
            if s["id"] in kept or not s["canSuspend"] or damped(s):
                continue
            actions.append({"action": "suspend", "id": s["id"], "reason": level})
            acted.add(s["id"])
            covered += s["megabytes"]

    if level == "calm":
        calm_ticks = (previous["calmTicks"] if previous["level"] == "calm" else 0) + 1
    else:
        calm_ticks = 0
    if level != "calm" and quiet and not crossed:
        waiting = sorted([s for s in subjects if s["suspended"] and s["id"] not in acted], key=order_key)
        if waiting:
            actions.append({"action": "resume", "id": waiting[-1]["id"], "reason": "calm"})
    if level == "calm" and calm_ticks >= policy["calmTicks"] and avail >= resume:
        waiting = sorted([s for s in subjects if s["suspended"] and s["id"] not in acted], key=order_key)
        if waiting:
            actions.append({"action": "resume", "id": waiting[-1]["id"], "reason": "calm"})

    return {"level": level, "calmTicks": calm_ticks, "hold": level != "calm", "trigger": trigger, "guardMegabytes": guard, "dangerMegabytes": danger, "resumeMegabytes": resume, "swapRoomMegabytes": room, "headroomMegabytes": headroom, "headroomGuardMegabytes": head_guard, "headroomDangerMegabytes": head_danger, "actions": actions}


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
                kilobytes[link["pid"]] = kilobytes.get(link["pid"], 0) + row["memoryKilobytes"]
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
        if "capMegabytes" in kind:
            result[-1]["capMegabytes"] = kind["capMegabytes"]
        if kind.get("capCritical"):
            result[-1]["capCritical"] = True
        if kind.get("ceiling") == "unit":
            ceiling = ceiling_of(row["command"], options.get("ceilings"))
            if ceiling is not None:
                result[-1]["ceiling"] = ceiling
    return result


def default_tables():
    return {"roots": ROOTS, "identities": IDENTITIES, "never": NEVER, "kinds": KINDS}


def others(rows, subjects, count, prefixes):
    by_pid = {r["pid"]: r for r in rows}
    owned = set(s["pid"] for s in subjects)

    def owns(row):
        seen = 0
        at = row
        while at is not None and seen < 256:
            if at["pid"] in owned:
                return True
            at = by_pid.get(at["parent"])
            seen += 1
        return False

    def top(row):
        at = row
        for _ in range(256):
            parent = by_pid.get(at["parent"])
            if parent is None or parent["pid"] <= 1:
                break
            at = parent
        return at

    groups = {}
    for row in rows:
        if row["pid"] <= 1 or owns(row):
            continue
        root = top(row)
        g = groups.setdefault(root["pid"], {"root": root, "kb": 0, "n": 0})
        g["kb"] += row["memoryKilobytes"]
        g["n"] += 1
    ranked = sorted(groups.values(), key=lambda g: (-g["kb"], g["root"]["pid"]))[:count]
    return [{"pid": g["root"]["pid"], "megabytes": g["kb"] // 1024, "processes": g["n"], "command": owner_label(g["root"]["command"], prefixes)} for g in ranked]


def smaps_footprint(text):
    values = {}
    for line in text.split("\n"):
        match = re.match(r"^(\w+):\s+(\d+)\s+kB\s*$", line)
        if match:
            values[match.group(1)] = int(match.group(2))
    if "Pss" not in values:
        return None
    return values["Pss"] + values.get("SwapPss", 0)


def charge(case):
    out = []
    for row in case["rows"]:
        kilobytes = row["memoryKilobytes"]
        pid = str(row["pid"])
        if case["platform"] == "darwin" and case["darwinBytes"].get(pid) is not None:
            kilobytes = case["darwinBytes"][pid] // 1024
        elif case["platform"] == "linux" and kilobytes >= case["minimumKilobytes"] and pid in case["smaps"]:
            found = smaps_footprint(case["smaps"][pid])
            if found is not None:
                kilobytes = found
        out.append(kilobytes)
    return out


def holds(case):
    state = case["state"]
    if state is None:
        return False
    heavy = case["needMegabytes"] >= POLICY["holdMinimumMegabytes"] or case.get("operation") in POLICY["holdOperations"]
    return bool(state["hold"]) and case["nowMs"] - state["atMs"] <= POLICY["staleMilliseconds"] and heavy


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
    if mode in ("--acute", "--warning-churn"):
        sample = json.load(open(os.path.join(HERE, "🚨️swap-churn", "🔣️.json"), encoding="utf-8"))
        key, level = ("host", "danger") if mode == "--acute" else ("warningHost", "guard")
        print(json.dumps(decide(POLICY, sample[key], sample["subjects"], {"level": level, "calmTicks": 0})))
    elif mode == "--others":
        print(json.dumps([others(c["rows"], c["subjects"], c["count"], c.get("prefixes", [])) for c in fixture["others"]], ensure_ascii=False))
    elif mode == "--charge":
        print(json.dumps([charge(c) for c in fixture["charges"]]))
    elif mode == "--holds":
        print(json.dumps([holds(c) for c in fixture["holds"]]))
    elif mode == "--decide":
        print(json.dumps(decisions(fixture), ensure_ascii=False))
    elif mode == "--classify":
        print(json.dumps(classifications(fixture), ensure_ascii=False))
    elif mode == "--discover":
        print(json.dumps(discoveries(fixture), ensure_ascii=False))
    elif mode == "--swap":
        print(json.dumps(swaps(fixture), ensure_ascii=False))
    elif mode in ("--write", "--check"):
        extras = (("others", lambda f: [others(c["rows"], c["subjects"], c["count"], c.get("prefixes", [])) for c in f["others"]]), ("holds", lambda f: [holds(c) for c in f["holds"]]), ("charges", lambda f: [charge(c) for c in f["charges"]]))
        for key, produce in (("discoveries", discoveries), ("swaps", swaps)) + extras:
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
        sys.exit("usage: 🐍️.py --acute|--warning-churn|--decide|--classify|--discover|--swap|--others|--holds|--charge|--write|--check")


if __name__ == "__main__":
    main()

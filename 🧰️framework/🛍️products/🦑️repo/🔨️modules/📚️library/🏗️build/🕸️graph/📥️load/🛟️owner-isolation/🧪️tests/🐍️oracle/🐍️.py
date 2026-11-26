import json, sys

RETRY_ROUNDS = 4
FOREIGN = {"BUILD_AUTHORITY_CHANGED", "BUILD_COLLECTED_ERRORS", "BUILD_SOURCE_UNPARSEABLE", "BUILD_MEMBERSHIP_CHANGED"}


def decide(attempts, rounds=RETRY_ROUNDS):
    for index, attempt in enumerate(attempts, start=1):
        if not attempt["fails"]:
            return {"outcome": "value", "attempts": index}
        if attempt.get("foreign") in FOREIGN:
            return {"outcome": "foreign", "attempts": index}
        moved = not attempt["rowsHold"]
        if not moved and not attempt["young"]:
            return {"outcome": "stable", "attempts": index}
        if index >= rounds:
            return {"outcome": "unsettled" if moved else "stable", "attempts": index}
    raise SystemExit("scripted attempts ended before a verdict")


def decision(foreign, moved, young, round_, rounds=RETRY_ROUNDS, cancelled=False):
    if foreign:
        return "foreign"
    if cancelled and round_ < rounds:
        return "retry"
    if not moved and not young:
        return "stable"
    if round_ >= rounds:
        return "unsettled" if moved else "stable"
    return "retry"


def revalidate(entry, fragment, now, cooldown, attempts, implementation=None, transient=False):
    stamp = {} if implementation is None else {"implementation": implementation}
    if entry is None or entry["fragment"] != fragment:
        return {"revalidate": False, "entry": {"fragment": fragment, "attempts": 0, "at": now, **stamp}}
    if implementation is not None and entry.get("implementation") != implementation:
        return {"revalidate": True, "entry": {"fragment": fragment, "attempts": 1, "at": now, **stamp}}
    used = 0 if transient and entry["attempts"] >= attempts else entry["attempts"]
    if used >= attempts or now - entry["at"] < cooldown:
        return {"revalidate": False, "entry": entry if used == entry["attempts"] else {**entry, "attempts": used}}
    return {"revalidate": True, "entry": {"fragment": fragment, "attempts": used + 1, "at": now, **stamp}}


if __name__ == "__main__":
    corpus = json.load(sys.stdin)
    rounds = corpus["retryRounds"]
    table = {
        "%d%d%d%d%s" % (foreign, moved, young, round_, "c" if cancelled else ""): decision(bool(foreign), bool(moved), bool(young), round_, rounds, bool(cancelled))
        for foreign in (0, 1) for moved in (0, 1) for young in (0, 1) for round_ in range(1, rounds + 1) for cancelled in (0, 1)
    }
    print(json.dumps({
        "retry": {case["name"]: decide(case["attempts"], rounds) for case in corpus["cases"]},
        "decisions": table,
        "revalidation": {case["name"]: revalidate(case.get("entry"), case["fragment"], case["now"], corpus["revalidation"]["cooldownMilliseconds"], corpus["revalidation"]["attempts"], case.get("implementation"), case.get("transient", False)) for case in corpus["revalidationCases"]},
    }, ensure_ascii=False))

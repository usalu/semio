import itertools, json, sys

VERDICTS = ["admitted", "empty", "persistent-state", "foreign-charge", "incomplete-charge", "unreadable"]


def plan(record, due, verdict):
    if record:
        return "resume"
    if not due:
        return "skip"
    return "reset" if verdict == "admitted" else "decline"


def explore():
    start = (False, False, "admitted")
    seen, queue, violations = {start}, [start], []
    while queue:
        record, due, verdict = queue.pop()
        step = plan(record, due, verdict)
        if record and step != "resume":
            violations.append(("record-not-resumed", record, due, verdict))
        if step == "reset" and (verdict != "admitted" or record or not due):
            violations.append(("reset-without-admission", record, due, verdict))
        if not record and step == "decline" and verdict == "admitted":
            violations.append(("declined-admitted", record, due, verdict))
        successors = [(record, True, verdict), (record, False, verdict)]
        if not record:
            successors += [(record, due, other) for other in VERDICTS]
        if step in ("reset", "resume"):
            successors += [(True, due, "unreadable"), (False, False, "unreadable")]
        for nxt in successors:
            if nxt not in seen:
                seen.add(nxt)
                queue.append(nxt)
    return len(seen), violations


def main():
    table = json.load(sys.stdin)
    expected = {(r, d, v): plan(r, d, v) for r, d, v in itertools.product([False, True], [False, True], VERDICTS)}
    actual = {(row["record"], row["due"], row["verdict"]): row["plan"] for row in table}
    mismatches = [key for key in expected if actual.get(key) != expected[key]]
    states, violations = explore()
    print(json.dumps({"combinations": len(expected), "mismatches": mismatches, "states": states, "violations": violations}))


main()

"""🔒️ Independent Python oracle of the leases status summary (standard library only): from the JSON status document it rebuilds the first line the text report must show and checks the admission section against the authored minimum floor. Requests arrive as JSON on stdin."""
import json, sys


def summary(document):
    return "Leases in %s: %d held, %d waiting, %d deadlock cycles, %d recent order violations." % (document["directory"], len(document["holders"]), len(document["waiters"]), len(document["cycles"]), len(document["violations"]))


def check(document, floor):
    problems = []
    for key in ("version", "directory", "atMs", "holders", "waiters", "waitFor", "cycles", "violations"):
        if key not in document:
            problems.append("missing " + key)
    admission = document.get("admission")
    if admission is not None:
        if not floor["minimum"] <= admission["floorMegabytes"] <= floor["maximum"]:
            problems.append("floor out of bounds")
        if not isinstance(admission["headroomMegabytes"], int):
            problems.append("headroom not an integer")
        for grant in admission["grants"]:
            if grant["heldSeconds"] < 0 or grant["jobs"] < 0:
                problems.append("bad grant")
        for waiter in admission["waiters"]:
            if waiter["waitedSeconds"] < 0 or waiter["needMegabytes"] < 0:
                problems.append("bad waiter")
    return problems


request = json.load(sys.stdin)
if "samples" in request:
    floor = request["floor"]
    print(json.dumps([min(row["totalMegabytes"], max(floor["minimum"], min(floor["maximum"], row["totalMegabytes"] * floor["basisPoints"] // 10000))) for row in request["samples"]]))
else:
    print(json.dumps({"summary": summary(request["document"]), "problems": check(request["document"], request["floor"])}))

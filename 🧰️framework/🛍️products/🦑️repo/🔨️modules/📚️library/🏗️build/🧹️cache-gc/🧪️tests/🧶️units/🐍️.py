"""Independent oracle of the collection of pid-named unit directories: only names that match the unit pattern are units, a unit whose process is alive or whose newest file is younger than the quiet time is never removed, a unit unused for the area's age goes first, and while the area is still over its budget the least recently used unit that may go goes next (ties by name). Reads {"cases"} from the file in argv[1] and prints, per case, the sorted names removed."""
import json
import re
import sys


def removed(case):
    pattern = re.compile(case["units"].replace("(?<pid>", "(?P<pid>"))
    units = sorted((e for e in case["entries"] if pattern.search(e["name"])), key=lambda e: e["name"])
    now, gone = case["now"], set()

    def blocked(e):
        return e["alive"] or now - e["recencyMs"] < case["quietMs"]

    for e in units:
        if case["unusedAgeMs"] is not None and now - e["recencyMs"] >= case["unusedAgeMs"] and not blocked(e):
            gone.add(e["name"])
    remaining = sum(e["bytes"] for e in units) - sum(e["bytes"] for e in units if e["name"] in gone)
    if case["budgetBytes"] is not None and remaining > case["budgetBytes"]:
        for e in sorted((u for u in units if u["name"] not in gone), key=lambda u: (u["recencyMs"], u["name"])):
            if remaining <= case["budgetBytes"]:
                break
            if blocked(e):
                continue
            gone.add(e["name"])
            remaining -= e["bytes"]
    return sorted(gone)


if __name__ == "__main__":
    document = json.load(open(sys.argv[1], encoding="utf8"))
    print(json.dumps([removed(case) for case in document["cases"]]))

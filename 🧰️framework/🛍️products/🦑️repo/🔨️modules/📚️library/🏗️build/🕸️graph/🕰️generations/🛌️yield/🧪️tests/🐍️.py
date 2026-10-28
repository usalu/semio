"""🐍️ Independent oracle of the graph queue yield: the yield decision and the front door's wake rule, simulated with plain integers from the policy, sharing no code with the JavaScript."""
import json
import sys

policy = json.load(open(sys.argv[1], encoding="utf-8"))
vectors = json.load(open(sys.argv[2], encoding="utf-8"))


def round_of(text):
    try:
        value = int(text)
    except (TypeError, ValueError):
        return 0
    return value if value >= 0 else 0


def decide(row):
    return bool(row["ticket"]) and round_of(row["round"]) < policy["maximumYields"] and row["queuedMs"] >= policy["yieldAfterMilliseconds"]


def reading(series, at):
    value = -1
    for moment, generation in series:
        if moment <= at:
            value = generation
    return value


def wake(row):
    at, last_report, progress = 0, 0, 0
    while True:
        generation = reading(row["series"], at)
        if generation > row["generation"]:
            return {"elapsedMs": at, "reason": "advanced", "progress": progress}
        if at >= policy["wakeMilliseconds"]:
            return {"elapsedMs": at, "reason": "timeout", "progress": progress}
        if at - last_report >= policy["progressMilliseconds"]:
            last_report = at
            progress += 1
        at += policy["pollMilliseconds"]


json.dump({"decide": {row["id"]: decide(row) for row in vectors["decide"]}, "wake": {row["id"]: wake(row) for row in vectors["wake"]}}, sys.stdout)

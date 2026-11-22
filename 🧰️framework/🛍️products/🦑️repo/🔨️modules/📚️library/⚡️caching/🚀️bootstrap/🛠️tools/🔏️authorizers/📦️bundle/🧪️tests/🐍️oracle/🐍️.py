"""🗑️ Independent Python oracle of the authorizer bundle retirement plan (standard library only): sort the rows newest first (ties by key), keep the newest ones, and retire the rest that no reader holds and that are at least the minimum age old, listed oldest first. Requests arrive as JSON on stdin."""
import json, sys


def plan(case):
    ordered = sorted(case["rows"], key=lambda row: (-row["touchedAt"], row["identity"]))
    behind = ordered[case["keepNewest"]:]
    due = [row["identity"] for row in behind if not row["held"] and case["nowMs"] - row["touchedAt"] >= case["minimumAgeMilliseconds"]]
    return list(reversed(due))


request = json.load(sys.stdin)
print(json.dumps([plan(case) for case in request["cases"]]))

"""Independent model of the owner claims: a claim is granted when no claim file exists, to the holder again, or when the claimant is dead or older than the stale bound (the claim is then taken over); it is `running` while a live other process holds a young claim; release removes the claim of the releasing holder only. Reads the fixture file (argv[1]) and prints the answer of every claim step."""
import json
import sys

fixture = json.load(open(sys.argv[1], encoding="utf8"))
files = {}
answers = []
for step in fixture["steps"]:
    stem, pid, now = step["stem"], step["pid"], step["at"]
    if step["op"] == "release":
        if files.get(stem, {}).get("pid") == pid:
            del files[stem]
        continue
    row = files.get(stem)
    if row is None or row["pid"] == pid:
        files[stem] = {"pid": pid, "start": row["start"] if row else now}
        answers.append("granted")
    elif fixture["pids"].get(str(row["pid"]), False) and now - row["start"] < fixture["staleMilliseconds"]:
        answers.append("running")
    else:
        files[stem] = {"pid": pid, "start": now}
        answers.append("granted")
print(json.dumps(answers))

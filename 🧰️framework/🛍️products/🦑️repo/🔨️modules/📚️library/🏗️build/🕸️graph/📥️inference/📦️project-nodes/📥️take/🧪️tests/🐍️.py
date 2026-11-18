"""Independent oracle of the take rules: the pause before look number round (integer growth up to a maximum) and the report line of a ledger, the looks of a lane under a cut. Reads the schema file (argv[1]) and the fixture file (argv[2]) and prints {"waits": [...], "lines": [...]}."""
import json
import sys

schema = json.load(open(sys.argv[1], encoding="utf8"))["definitions"]
fixture = json.load(open(sys.argv[2], encoding="utf8"))
policy = schema["Policy"]["const"]
texts = schema["Texts"]["const"]


def wait(round_):
    value = policy["pollFirstMilliseconds"]
    for _ in range(round_):
        if value >= policy["pollMaximumMilliseconds"]:
            break
        value = min(policy["pollMaximumMilliseconds"], value * policy["pollGrowthPercent"] // 100)
    return min(value, policy["pollMaximumMilliseconds"])


def line(row):
    states = row["states"]
    count = lambda name: sum(1 for state in states.values() if state == name)
    present, fetched, here, stale = count("present"), count("fetched"), count("inferred"), count("stale")
    wanted = len([name for name in row["wanted"] if states.get(name) not in ("present", "fetched", "inferred")])
    values = {"total": row["total"], "present": present, "fetched": fetched, "here": here, "stale": stale, "running": row["running"], "wanted": wanted, "done": present + fetched + here}
    text = texts["de" if row["language"] == "de" else "en"]["progress"]
    for key, value in values.items():
        text = text.replace("{" + key + "}", str(value))
    return text


lanes = lambda cut: policy["concurrency"] if cut is None else max(1, min(policy["concurrency"], cut))

def split(row):
    heavy = set(row["heavy"])
    return {"light": [n for n in row["names"] if n not in heavy], "heavy": [n for n in row["names"] if n in heavy]}


print(json.dumps({"splits": [split(row) for row in fixture["splits"]], "waits": [wait(r) for r in fixture["rounds"]], "lines": [line(row) for row in fixture["ledgers"]], "lanes": [lanes(cut) for cut in fixture["cuts"]]}, ensure_ascii=False))

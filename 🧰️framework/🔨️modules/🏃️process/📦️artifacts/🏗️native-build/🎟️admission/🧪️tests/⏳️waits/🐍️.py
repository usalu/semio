"""Independent oracle: the status lines of waiting units, filled from the admission schema texts, for the corpus rows (argv[1] = corpus, argv[2] = admission schema)."""
import json, re, sys

corpus = json.load(open(sys.argv[1], encoding="utf-8"))
messages = json.load(open(sys.argv[2], encoding="utf-8"))["definitions"]["Messages"]["const"]


def fill(text, values):
    return re.sub(r"\{([a-z]+)\}", lambda m: str(values.get(m.group(1), m.group(0))), text)


out = {"en": [], "de": []}
for item in corpus["rows"]:
    row = item["row"]
    seconds = row["seconds"] + max(0, round((item["now"] - item["published"]) / 1000))
    name = {"growth": "growth", "idle": "idle", "queue": "queued"}[row["reason"]]
    values = {"unit": row["unit"], "predicted": row["predictedMegabytes"], "seconds": seconds, "place": row["place"], "count": row["count"], "running": row["running"], "held": row["heldMegabytes"], "budget": row["budgetMegabytes"], "grant": row["grantMegabytes"]}
    for language in out:
        waits = messages[language]["waits"]
        out[language].append(fill(waits["row"], {"pid": 4242, "text": fill(waits[name], values)}))
print(json.dumps(out, ensure_ascii=False))

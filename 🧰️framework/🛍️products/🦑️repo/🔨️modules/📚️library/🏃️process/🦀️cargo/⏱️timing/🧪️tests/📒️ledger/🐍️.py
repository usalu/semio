"""🐍 Independent oracle of the Cargo phase ledger: pairs begin with ready/end per phase and sums the elapsed milliseconds (stdlib only). usage: python -I this.py ledger.jsonl"""
import json, sys

rows = [json.loads(line) for line in open(sys.argv[1], encoding="utf8")]
begun, spans = {}, []
for row in rows:
    assert row["version"] == 1 and row["command"] == "cargo:build" and row["pid"] == 4242
    if row["point"] == "begin":
        begun[row["phase"]] = row["elapsedMs"]
    else:
        spans.append([row["phase"], row["elapsedMs"] - begun[row["phase"]]])
print(json.dumps({"phases": spans, "rows": len(rows)}))

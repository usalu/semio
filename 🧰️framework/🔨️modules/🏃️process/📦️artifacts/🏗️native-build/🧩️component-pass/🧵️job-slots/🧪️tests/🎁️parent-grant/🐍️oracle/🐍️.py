"""🎁️ Independent Python oracle (standard library only) of a component pass inside a parent memory grant. It models the contract, not the implementation: the pass needs `peak + footprint` beside the controller (`baseline`); the grant carries `grant - baseline`. A pass that fits starts. Otherwise a need that cannot be carried even by the most this machine could ever give (`baseline + need > ceiling`) is refused at once, as is a need without any growth path (none, or an exact grant whose holder never grows it and no supplementary reservation); with a supplementary reservation the difference `baseline + need - grant` is asked once: granted, the grant grows by it and the pass starts, refused, the pass is refused at once. `python3 🐍️.py` prints the outcome of every case."""
import json
import os

HERE = os.path.dirname(os.path.abspath(__file__))
FIX = json.load(open(os.path.join(HERE, "..", "🧫️fixtures", "🔣️.json"), encoding="utf-8"))


def decide(case):
    need = case["peakMegabytes"] + case["footprintMegabytes"]
    carried = case["grantMegabytes"] - case["baselineMegabytes"]
    if need <= carried:
        return {"outcome": "started", "acquired": 0, "allowance": case["grantMegabytes"]}
    refused = {"outcome": "refused", "acquired": 0, "allowance": case["grantMegabytes"]}
    if case["baselineMegabytes"] + need > case["ceilingMegabytes"]:
        return refused
    if case["growth"] in ("none", "exact"):
        return refused
    extra = case["baselineMegabytes"] + need - case["grantMegabytes"]
    if case["growth"] == "acquire-refused":
        return {"outcome": "refused", "acquired": extra, "allowance": case["grantMegabytes"]}
    return {"outcome": "started", "acquired": extra, "allowance": case["grantMegabytes"] + extra}


print(json.dumps({case["name"]: decide(case) for case in FIX["cases"]}))

"""💤️ Independent model of the idle rule: reads {"base", "implementation", "wanted", "case"} on stdin and answers {"idle": bool}. A command may build at once when nobody holds the builder custody and no live running claim of its own plan names a wanted owner."""
import json
import sys

request = json.load(sys.stdin)
case, wanted = request["case"], set(request["wanted"])


def same_plan(claim):
    return claim["plan"] == "same"


busy = case["builderHeld"] or any(
    claim["state"] == "running" and claim["live"] and same_plan(claim) and wanted & set(claim["owners"])
    for claim in case["claims"]
)
print(json.dumps({"idle": not busy}))

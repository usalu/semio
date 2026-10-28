"""Independent model of the retention rule of the test and policy owners: one JSON world on stdin -> {"retained": bool}.
The owners are kept when there is a previous generation (not cold, a known change list), at least one owner is tagged and all tags are the current ones, the environment digest did not move, no tagged owner's root is among the roots being refreshed and no `.feature` file or `📜️script.ts` was added, removed or changed."""
import json
import sys

CURRENT = ("tests-v2", "policy-v2")


def retained(w):
    tagged = [stub for stub in w["stubs"].values() if stub.get("tag") is not None]
    if w["cold"] or w["changed"] is None or not tagged:
        return False
    if any(stub["tag"] not in CURRENT for stub in tagged):
        return False
    if w["environment"] != w["previousEnvironment"]:
        return False
    if any(stub["root"] in w["roots"] for stub in tagged):
        return False
    return not any(path.endswith(".feature") or path.endswith("📜️script.ts") for path in w["changed"])


if __name__ == "__main__":
    print(json.dumps({"retained": retained(json.load(sys.stdin))}))

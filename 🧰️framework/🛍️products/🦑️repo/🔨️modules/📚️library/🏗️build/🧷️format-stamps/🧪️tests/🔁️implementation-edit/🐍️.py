"""🐍️ Independent oracle of the implementation edit vectors: classifies a failure and decides what the outermost command does, from the declared protocol alone, without sharing a line with the implementation."""
import json
import sys


def load(path):
    with open(path, encoding="utf-8") as handle:
        return json.load(handle)


def edit_of(churn, error, depth=0):
    if depth > 4 or not isinstance(error, dict):
        return None
    if error.get("code") == churn["concurrentEditCode"] and isinstance(error.get("path"), str) and error.get("reason") in churn["reasons"]["en"]:
        return {"reason": error["reason"], "path": error["path"]}
    if "errors" in error and error.get("message") in churn["primaryFirstAggregates"]:
        return edit_of(churn, error["errors"][0], depth) if error["errors"] else None
    inner = ([error["cause"]] if "cause" in error else []) + error.get("errors", [])
    if inner and "errors" in error:
        found = [edit_of(churn, member, depth) for member in error["errors"]]
        return found[0] if found and all(found) else None
    return edit_of(churn, error["cause"], depth + 1) if "cause" in error else None


def classify(protocol, churn, error, bundled=False, depth=0):
    if depth > 8 or not isinstance(error, dict):
        return None
    if error.get("code") in churn["staleImplementationCodes"]:
        return "error"
    load_failure = protocol["loadFailure"]
    if not bundled and (error.get("name") in load_failure["names"] or error.get("name") == "SyntaxError" and error.get("message", "").startswith(load_failure["syntaxErrorPrefix"])):
        return "load"
    if error.get("name") == "OwnedCommandFailure" and error.get("outcome", {}).get("status") == protocol["exitCode"]:
        return "exit"
    inner = ([error["cause"]] if "cause" in error else []) + error.get("errors", [])
    for member in inner:
        found = classify(protocol, churn, member, bundled or "errors" in error, depth + 1)
        if found:
            return found
    return None


def decide(protocol, row):
    attempt = 0 if row["ranMilliseconds"] >= protocol["progressMilliseconds"] else row["attempt"]
    if not row["outermost"]:
        return {"status": "yielded", "code": protocol["exitCode"]}
    if attempt >= protocol["maximumReexecutions"]:
        return {"status": "exhausted", "code": protocol["exitCode"]}
    table = {"load": protocol["loadFailure"]["backoffMs"], "edit": protocol["editFailure"]["backoffMs"]}.get(row["origin"])
    delay = table[min(attempt, len(table) - 1)] if table else 0
    announced = {"load": "loadfailure", "edit": "editfailure"}.get(row["origin"], "restart")
    return {"status": "reexecuted", "code": 0, "delayMs": delay, "nextAttempt": attempt + 1, "announced": announced}


def main():
    fixture, protocol, churn = load(sys.argv[1]), load(sys.argv[2])["$defs"]["Protocol"]["const"], load(sys.argv[3])["definitions"]["Protocol"]["const"]
    print(json.dumps({
        "classification": {row["id"]: classify(protocol, churn, row["error"]) or ("edit" if edit_of(churn, row["error"]) else None) for row in fixture["classification"]},
        "policy": {row["id"]: decide(protocol, row) for row in fixture["policy"]},
    }, ensure_ascii=False, sort_keys=True))


main()

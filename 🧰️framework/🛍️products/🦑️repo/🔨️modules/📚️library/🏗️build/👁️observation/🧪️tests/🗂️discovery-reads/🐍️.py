import json
import sys

def model(scenario):
    directories = [""] + [p.rstrip("/") for p in scenario["tree"] if p.endswith("/")]
    names = set(p.rstrip("/") for p in scenario["tree"])
    kinds = {p.rstrip("/"): ("directory" if p.endswith("/") else "file") for p in scenario["tree"]}
    answers = []
    for kind, path in scenario["asks"]:
        if kind == "exists":
            answers.append(path in names)
        elif kind == "fileExists":
            answers.append(kinds.get(path) == "file")
        else:
            answers.append(kinds.get(path) == "directory")
    return {"listings": len(directories), "answers": answers, "distinctQuestions": len(scenario["asks"])}

payload = json.load(sys.stdin)
print(json.dumps({"expected": [model(s) for s in payload["scenarios"]]}))

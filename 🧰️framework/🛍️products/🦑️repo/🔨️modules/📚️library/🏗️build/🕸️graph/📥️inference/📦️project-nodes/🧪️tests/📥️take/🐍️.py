"""Independent oracle of the key of a project record in the fleet cache: the state digest and the slot tooling a record is stored under, from the inputs of an owner (head parts, root, project JSON, selected targets and inputs, revision, platform). The text of the key is the JSON array [revision, null, [...head, root, json, selectedTargets or null, selectedInputs]]; the state digest is the sha256 of the JSON array ["project-inference", form, neutral(keyText), platform, neutral(fleet)] where neutral replaces the root of the machine, where it stands as a whole path prefix, by the root token. Reads the fixture (argv[1]) and prints {"keys": [...], "digests": [...], "tooling": [...]}."""
import hashlib
import json
import re
import sys

TOKEN = "⟦semio-root⟧"
fixture = json.load(open(sys.argv[1], encoding="utf8"))


def sha(text):
    return hashlib.sha256(text.encode("utf8")).hexdigest()


def dumps(value):
    return json.dumps(value, separators=(",", ":"), ensure_ascii=False)


def neutral(text, root):
    root = root.rstrip("/\\")
    pattern = r"(?<![\w.-])" + re.escape(root) + r"(?=$|[^\w.-])"
    return re.sub(pattern, lambda _m: TOKEN, text)


def key_text(case):
    return dumps([fixture["revision"], None, [*case["head"], case["json"]["root"], case["json"], case.get("selectedTargets"), case["selectedInputs"]]])


keys, digests, tooling = [], [], []
for case in fixture["cases"]:
    text = key_text(case)
    keys.append(text)
    digests.append(sha(dumps(["project-inference", fixture["form"], neutral(text, case["workspace"]), fixture["platform"], neutral(case["fleet"], case["workspace"])])))
    tooling.append(sha(neutral(case["fleet"], case["workspace"])))
print(dumps({"keys": keys, "digests": digests, "tooling": tooling}))

"""♨️ Independent Python oracle of the Bazel warm verdicts (standard library only): it attributes every warm action from the raw execution-log rows of a cold and a warm run. Requests arrive as JSON on stdin."""
import json, re, sys

UNITS = lambda text: text.encode("utf-16-be")
DERIVED = re.compile(r"^((implementation|adapter)\.json|t_[0-9a-f]+\.(input|json))$")


def digests(rows, key):
    return {entry[key]: entry["digest"]["hash"] for entry in rows}


def verdicts(case):
    cold = {row["targetLabel"]: row for row in case["cold"]}
    result = []
    for row in sorted(case["warm"], key=lambda spawn: UNITS(spawn["targetLabel"])):
        label = row["targetLabel"]
        empty = {"label": label, "changedInputs": [], "addedInputs": [], "removedInputs": [], "argumentsChanged": False, "environmentChanged": [], "outputsIdentical": None, "derivedInputsChanged": False, "sourceInputsChanged": 0}
        if row["cacheHit"]:
            result.append({**empty, "verdict": "hit"})
            continue
        if row.get("cacheable", True) is False:
            result.append({**empty, "verdict": "uncached"})
            continue
        if label not in cold:
            result.append({**empty, "verdict": "new"})
            continue
        before = cold[label]
        old_in, new_in = digests(before["inputs"], "path"), digests(row["inputs"], "path")
        changed = sorted((path for path in new_in if path in old_in and old_in[path] != new_in[path]), key=UNITS)
        added = sorted(set(new_in) - set(old_in), key=UNITS)
        removed = sorted(set(old_in) - set(new_in), key=UNITS)
        old_env = {entry["name"]: entry["value"] for entry in before["environmentVariables"]}
        new_env = {entry["name"]: entry["value"] for entry in row["environmentVariables"]}
        names = sorted((name for name in set(old_env) | set(new_env) if old_env.get(name) != new_env.get(name)), key=UNITS)
        args = before["commandArgs"] != row["commandArgs"]
        old_out, new_out = digests(before["actualOutputs"], "path"), digests(row["actualOutputs"], "path")
        paths = set(old_out) | set(new_out)
        identical = None if not paths else all(old_out.get(path) == new_out.get(path) for path in paths)
        moved = bool(changed or added or removed or args or names)
        result.append({"label": label, "verdict": "key-changed" if moved else "executed-with-stable-key", "changedInputs": changed, "addedInputs": added, "removedInputs": removed, "argumentsChanged": args, "environmentChanged": names, "outputsIdentical": identical, "derivedInputsChanged": any(DERIVED.match(path) for path in changed), "sourceInputsChanged": sum(1 for path in changed if not DERIVED.match(path)) + len(added) + len(removed)})
    return result


print(json.dumps([verdicts(case) for case in json.load(sys.stdin)["cases"]], ensure_ascii=False))

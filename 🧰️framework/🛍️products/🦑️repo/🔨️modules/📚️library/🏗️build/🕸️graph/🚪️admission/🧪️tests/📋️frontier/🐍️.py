import json
import sys
from collections import defaultdict


def admission(case):
    selection = case["selection"]
    rows = case["rows"]
    providers = case.get("providers", [])
    direct = selection.get("excludeDependencies", False)
    calls, live, chosen, available, revisions = [], set(), defaultdict(dict), {}, {}
    required_inputs = defaultdict(set)
    source_files, owned_files, foreign_files, catalog = set(), set(), set(), {}

    def task(owner, target, required):
        live.add(owner)
        chosen[owner][target] = chosen[owner].get(target, False) or required

    def inspect(owner, targets, inputs):
        if len(calls) == case.get("abortAfter"):
            raise ValueError("admission fixture cancelled")
        calls.append([owner, targets, inputs])
        row = rows[owner]
        return row.get("taskTargets", row["targets"]) if targets else row["targets"]

    if selection["kind"] == "owners":
        live.update(selection["projects"])
        for owner in live:
            for target in selection.get("targets", []):
                task(owner, target, False)
    else:
        for request in selection["tasks"]:
            task(request["project"], request["target"], True)

    try:
        changed = True
        while changed:
            changed = False
            for owner in sorted(live):
                if owner not in available and any(not required for required in chosen[owner].values()):
                    available[owner] = inspect(owner, [], [])
                chosen[owner] = {key: required for key, required in chosen[owner].items()
                                 if required or key in available.get(owner, {})}
                targets = sorted(chosen[owner])
                inputs = sorted(required_inputs[owner])
                if revisions.get(owner) == (targets, inputs):
                    continue
                revisions[owner] = (targets, inputs)
                if targets or inputs or owner not in available:
                    available[owner] = inspect(owner, targets, inputs)
                changed = True
                row = rows[owner]
                source_files.add(owner + "/📋️project.json")
                owned_files.update(row.get("files", []))
                foreign_files.update(row.get("observedFiles", []))
                if not direct:
                    live.update(path.split("/")[0] for path in row.get("files", []))
                    live.update(row.get("dependencies", []))
                    for dependency, groups in row.get("inputs", {}).items():
                        live.add(dependency)
                        required_inputs[dependency].update(groups)
                for target in targets:
                    if target not in available[owner]:
                        raise ValueError("Unknown admitted build target: " + owner + ":" + target)
                    if direct:
                        continue
                    for dep in available[owner][target] + row.get("generated", {}).get(target, []):
                        if dep.startswith("^"):
                            for neighbor in row.get("dependencies", []):
                                task(neighbor, dep[1:], False)
                        elif ":" in dep:
                            project, name = dep.rsplit(":", 1)
                            task(project, name, True)
                        else:
                            task(owner, dep, True)

            for provider in providers:
                path = provider["sourcePath"]
                absolute = "{workspaceRoot}/" + path
                target_match = any(
                    owner in provider["consumerRoots"]
                    and any(target in available.get(owner, {}) for target in chosen[owner])
                    for owner in chosen
                )
                if path not in catalog and (target_match or any(absolute in values for values in required_inputs.values())):
                    provider_owner = provider["ownerRoot"]
                    catalog[path] = {"ownerRoot": provider_owner, "sourcePath": path}
                    source_files.add(path)
                    live.add(provider_owner)
                    required_inputs[provider_owner].add(absolute)
                    changed = True

    except ValueError as error:
        return {"calls": calls, "error": str(error)}
    result = {"owners": sorted(live), "files": sorted(source_files | owned_files | foreign_files),
              "targets": {owner: sorted(targets) for owner, targets in chosen.items() if targets},
              "inputs": {owner: sorted(inputs) for owner, inputs in required_inputs.items() if inputs},
              "catalogProviders": [catalog[path] for path in sorted(catalog)]}
    if foreign_files - owned_files:
        result["observedFiles"] = sorted(foreign_files - owned_files)
    generated = {owner: rows[owner]["generated"] for owner in available if rows[owner].get("generated")}
    if generated and not direct:
        result["generatedDependencies"] = generated
    return {"calls": calls, "admission": result}


print(json.dumps({case["name"]: admission(case) for case in json.load(sys.stdin)}))

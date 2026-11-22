"""Independent oracle of what the generator output owners of one owner root depend on: whether an edit of one field of one contract changes any fact that `generatorOutputOwners(contracts, root, ...)` consults (the ownership, the owner path, the output roots, the producer of every output, and the target and the output paths where the contract or an output concerns the root). Reads the fixture file (argv[1]) and prints, for every edit, whether the record of the owner stays valid."""
import copy
import json
import sys

fixture = json.load(open(sys.argv[1], encoding="utf8"))
root = fixture["owner"]


def facts(contracts):
    seen = {"ids": list(contracts.keys())}
    for cid, contract in contracts.items():
        seen[cid + ".ownership"] = contract.get("ownership")
        if contract.get("ownership") != "owned":
            continue
        seen[cid + ".ownerPath"] = contract.get("ownerPath")
        concerned = contract.get("ownerPath") == root
        if concerned:
            seen[cid + ".target"] = contract.get("target")
        seen[cid + ".outputs"] = len(contract.get("outputRoots", []))
        for index, output in enumerate(contract.get("outputRoots", [])):
            producer = output.get("producer")
            owner = producer if producer is not None else contract
            seen[f"{cid}.{index}.producer"] = None if producer is None else json.dumps(producer, sort_keys=True)[:0] or "set"
            seen[f"{cid}.{index}.owner"] = owner.get("ownerPath")
            if owner.get("ownerPath") == root:
                seen[f"{cid}.{index}.path"] = output.get("path")
                seen[f"{cid}.{index}.target"] = owner.get("target")
    return seen


before = facts(fixture["contracts"])
answers = []
for edit in fixture["edits"]:
    changed = copy.deepcopy(fixture["contracts"])
    changed[edit["id"]][edit["field"]] = edit["value"]
    answers.append(facts(changed) == before)
print(json.dumps(answers))

import json, sys

def main():
    trace = json.load(sys.stdin)
    inventory = {}
    live = {}
    intents = {}
    violations = []
    for number, step in enumerate(trace["steps"]):
        op = step["op"]
        if op == "leaf":
            inventory[step["identity"]] = step["allocatedBytes"]
        elif op == "publish":
            live[step["record"]] = set(step["references"])
        elif op == "evict":
            intents[step["record"]] = live.pop(step["record"])
        elif op == "settle":
            intents.pop(step["record"])
        elif op == "retire":
            referenced = set().union(*live.values(), *intents.values()) if live or intents else set()
            rows = sorted(((i, b) for i, b in inventory.items() if i not in referenced), key=lambda r: (-r[1], r[0]))
            available = step["availableBytes"]
            expected = []
            for identity, allocated in rows:
                if available >= step["minimumAvailableBytes"]:
                    break
                expected.append(identity)
                available += allocated
            if step["selected"] != expected:
                violations.append({"step": number, "kind": "selection", "expected": expected, "selected": step["selected"]})
            for identity in step["selected"]:
                if identity in referenced:
                    violations.append({"step": number, "kind": "referenced-deleted", "identity": identity})
                inventory.pop(identity, None)
        for holder in (live, intents):
            for record, references in holder.items():
                for identity in references:
                    if identity not in inventory:
                        violations.append({"step": number, "kind": "dangling", "record": record, "identity": identity})
    print(json.dumps({"violations": violations}))

main()

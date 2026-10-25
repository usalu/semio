import json, sys

def settles(case):
    waits = {"start": []}
    if case["body"] == "return":
        return True
    for index in range(case["nested"]):
        owner = "owner%d" % index
        waits["start"].append(owner)
        waits[owner] = ["retire"]
    waits["retire"] = [] if case["retire"] == "before-exit" else ["start"]
    state = {}

    def cyclic(node):
        if state.get(node) == 1:
            return True
        if state.get(node) == 2:
            return False
        state[node] = 1
        if any(cyclic(next_node) for next_node in waits.get(node, [])):
            return True
        state[node] = 2
        return False

    return not cyclic("start")

print(json.dumps([settles(case) for case in json.load(sys.stdin)["cases"]]))

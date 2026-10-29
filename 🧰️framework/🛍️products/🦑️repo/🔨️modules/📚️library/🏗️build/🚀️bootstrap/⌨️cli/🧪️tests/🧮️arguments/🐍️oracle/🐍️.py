"""⌨️ Independent Python oracle of the native recovery selection (standard library only): it decides, from the authored protocol and an argument vector, which fixed recovery command is meant, if any. Requests arrive as JSON on stdin."""
import json, sys


def select(protocol, args):
    if not args or args[0] != "run" or len(args) < 2:
        return None
    parts = args[1].split(":")
    if len(parts) != 2:
        return None
    project, target = parts
    maintenance = project == protocol["maintenance"]["project"] and target in protocol["maintenance"]["targets"]
    frontend = project == protocol["project"] and target in protocol["targets"]
    reset = project == protocol["reset"]["project"] and target in protocol["reset"]["targets"]
    if not (maintenance or frontend or reset):
        return None
    index = 2
    while index < len(args):
        option = args[index]
        if option == "--output" and index + 1 < len(args) and args[index + 1] == "stream":
            index += 2
            continue
        if option not in protocol["options"] and not (frontend and target == "build" and option in protocol["buildOnlyOptions"]):
            return None
        index += 1
    if reset:
        return {"kind": "cache-reset", "target": protocol["reset"]["targets"][target], "owner": protocol["reset"]["scriptRoot"]}
    if maintenance:
        return {"kind": "maintenance", "target": protocol["maintenance"]["targets"][target], "owner": protocol["maintenance"]["scriptRoot"]}
    return {"kind": "frontend", "target": target, "owner": protocol["owner"]}


request = json.load(sys.stdin)
print(json.dumps([select(request["protocol"], row) for row in request["rows"]], ensure_ascii=False))

"""🐍️ Independent model of the content-addressed executable publication: reads the scenarios on stdin, replays them on a table of digest directories (absent, intact or damaged) and live holders, and prints one expected outcome per step. It shares no code with the TypeScript and names every executable by Python's own sha256."""
import hashlib, json, sys

def payload(identifier):
    return ("#!/bin/sh\n# executable " + identifier + "\n").encode()

def digest(identifier):
    return hashlib.sha256(payload(identifier)).hexdigest()

def replay(operations):
    table, holders, outcomes = {}, {}, []
    for operation in operations:
        verb, identifier = operation[0], operation[1]
        state, running = table.get(identifier), holders.get(identifier, 0)
        if verb == "build":
            if state == "intact":
                outcome = "reused"
            elif state == "damaged" and running:
                outcome = "blocked"
            else:
                table[identifier], outcome = "intact", "published"
        elif verb == "run":
            outcome = "ok" if state else "absent"
            if state:
                holders[identifier] = running + 1
        elif verb == "stop":
            outcome = "ok" if running else "absent"
            if running:
                holders[identifier] = running - 1
        elif verb == "damage":
            outcome = "ok" if state else "absent"
            if state:
                table[identifier] = "damaged"
        elif verb == "retire":
            if not state:
                outcome = "absent"
            elif running:
                outcome = "held"
            else:
                del table[identifier]
                outcome = "retired"
        else:
            raise SystemExit("unknown verb " + verb)
        outcomes.append(outcome)
    return outcomes

scenarios = json.load(sys.stdin)["scenarios"]
print(json.dumps([{"name": row["name"], "outcomes": replay(row["operations"]), "digests": {operation[1]: digest(operation[1]) for operation in row["operations"]}} for row in scenarios], ensure_ascii=False))

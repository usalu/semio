"""🐍️ Independent oracle of the build holder predicate: reads the grace in milliseconds as argv[1] and a list of evidence rows from stdin, prints the verdict of every row. It shares no code with the TypeScript predicate."""
import json
import sys

grace = int(sys.argv[1])


def live(row):
    proofs = [row["lane"], row["lease"], row["lock"] == "held"]
    if row["lock"] == "unsupported":
        proofs.append(row["idleMilliseconds"] < grace)
    return any(proofs)


print(json.dumps([live(row) for row in json.load(sys.stdin)]))

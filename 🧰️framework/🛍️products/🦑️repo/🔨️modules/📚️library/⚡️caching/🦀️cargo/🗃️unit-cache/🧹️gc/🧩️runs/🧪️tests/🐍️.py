"""🐍️ Independent oracle of the component run plan: reads a list of states from stdin, prints the removals of every state. It shares no code with the TypeScript planner."""
import json
import sys


def plan(state):
    runs = sorted(state["runs"], key=lambda run: (run["lastUsedMs"], run["path"]))
    settled = [run for run in runs if not run["held"]]
    latest = {}
    for run in settled:
        for component in run["components"]:
            latest[component] = run["path"]
    materialized = [run for run in settled if run["components"]]
    newest = (materialized[-1]["lastUsedMs"], materialized[-1]["path"]) if materialized else None
    removed, freed = [], 0
    for run in runs:
        if freed >= state["needBytes"]:
            break
        if run["held"]:
            continue
        if run["components"]:
            superseded = all(latest[component] != run["path"] for component in run["components"])
        else:
            superseded = newest is not None and newest > (run["lastUsedMs"], run["path"])
        if superseded:
            removed.append({"path": run["path"], "bytes": run["bytes"]})
            freed += run["bytes"]
    return removed


print(json.dumps([plan(state) for state in json.load(sys.stdin)]))

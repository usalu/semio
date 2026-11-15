"""Independent oracle of the scoped plan a reply of the graph service means: projects of its tasks, dependency edges and the tasks that have to run. Usage: oracle.py FIXTURE.json -> JSON list of expectations, one per case (null when a dependency leaves the plan)."""
import json
import sys


def split(task):
    project, _, target = task.rpartition(":")
    return project, target


def expect(case):
    ids = {task["id"] for task in case["tasks"]}
    edges = []
    for task in case["tasks"]:
        for dependency in task["dependsOn"]:
            if dependency not in ids:
                return None
            if [task["id"], dependency] not in edges:
                edges.append([task["id"], dependency])
    tasks = list(dict.fromkeys(task["id"] for task in case["tasks"]))
    needed = {edge[1] for edge in edges}
    named = list(dict.fromkeys(target for target in case["targets"] if target != "*:*"))
    roots = [task for task in tasks if task not in needed]
    return {
        "projects": sorted({split(task)[0] for task in tasks}),
        "tasks": tasks,
        "edges": edges,
        "stale": list(dict.fromkeys(task["id"] for task in case["tasks"] if task["stale"])),
        "scope": named or roots or tasks,
    }


if __name__ == "__main__":
    print(json.dumps([expect(case) for case in json.load(open(sys.argv[1], encoding="utf-8"))["scoped"]]))

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
            edges.append([task["id"], dependency])
    return {
        "projects": sorted({split(task["id"])[0] for task in case["tasks"]}),
        "tasks": [task["id"] for task in case["tasks"]],
        "edges": edges,
        "stale": [task["id"] for task in case["tasks"] if task["stale"]],
    }


if __name__ == "__main__":
    print(json.dumps([expect(case) for case in json.load(open(sys.argv[1], encoding="utf-8"))["scoped"]]))

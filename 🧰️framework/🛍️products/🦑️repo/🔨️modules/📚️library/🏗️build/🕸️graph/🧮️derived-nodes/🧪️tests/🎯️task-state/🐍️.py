"""🎯 Independent model of the task state: the key of every planned task and the tasks a plan reports stale.

One command reads one scenario on stdin and prints, for the initial plan and after every step, the key of each task and the stale tasks:
  run -> {"steps": [{"name", "keys": {id: key|null}, "stale": [id]}]}
A task key is the digest of the node `task-key` of its root and target and of the keys of its dependencies (null when the task cannot be keyed or a dependency has none); a task is stale when it is not cacheable, has no key, or its key differs from the one recorded after its last successful run. Every step names the tasks that succeeded after its plan (`run`): their keys are recorded. The node values come from the naive evaluator of the derived-node oracle (standard library `json` / `tomllib`, no memo). Standard library only.
"""
import importlib.util
import json
import os
import sys

HERE = os.path.dirname(os.path.abspath(__file__))
spec = importlib.util.spec_from_file_location("derived_oracle", os.path.join(HERE, "..", "🐍️oracle", "🐍️.py"))
oracle = importlib.util.module_from_spec(spec)
spec.loader.exec_module(oracle)


def plan(scn, files, recorded):
    ev = oracle.Evaluator({**scn, "actions": {}}, files)
    keys = {}
    for tid in scn["order"]:
        task = scn["tasks"][tid]
        own = ev.node("task-key|" + task["root"] + ":" + task["target"])["value"]["key"]
        deps = sorted(d for d in task["dependsOn"] if d in keys)
        dep_keys = [keys[d] for d in deps]
        keys[tid] = None if own is None or None in dep_keys else oracle.digest({"own": own, "dependencies": dep_keys})
    stale = [tid for tid in scn["order"] if not scn["tasks"][tid]["cacheable"] or keys[tid] is None or recorded.get(tid) != keys[tid]]
    return keys, stale


def run(scn):
    files = dict(scn["files"])
    recorded = {}
    out = []
    keys, stale = plan(scn, files, recorded)
    steps = [{"name": "initial", "edit": {}, "run": scn.get("run", [])}] + scn["steps"]
    for index, step in enumerate(steps):
        if index:
            for path, text in step["edit"].items():
                if text is None:
                    files.pop(path, None)
                else:
                    files[path] = text
            keys, stale = plan(scn, files, recorded)
        out.append({"name": step["name"], "keys": keys, "stale": stale})
        for tid in step.get("run", []):
            if keys[tid] is not None and scn["tasks"][tid]["cacheable"]:
                recorded[tid] = keys[tid]
    return {"steps": out}


if __name__ == "__main__":
    print(json.dumps(run(json.loads(sys.stdin.read())), sort_keys=True, ensure_ascii=False))

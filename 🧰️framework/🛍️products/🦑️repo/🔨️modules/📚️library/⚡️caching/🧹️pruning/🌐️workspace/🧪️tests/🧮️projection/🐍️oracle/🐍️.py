"""🌐️ Independent Python oracle of the cache area projection (standard library only): from the scanners' answers and the authored storage policy it derives the five areas with their budgets, ages, guard ages and units, the scanner calls that must have happened, the progress events by area, and the root a unit of an area was measured against. Requests arrive as JSON on stdin."""
import json, sys


def project(layout, storage, repo):
    scans = layout["scans"]
    shared = layout["build"] == layout["target"]
    calls = [["build", layout["build"]]] + ([] if shared else [["target", layout["target"]]]) + [["incremental", layout["build"]], ["vite", repo + "/cache/vite", []], ["agents", repo + "/cache/agents", ["resource-leases"]]]
    areas = [
        {"name": "cargo", "budgetBytes": storage["cargo"]["budgetBytes"], "unusedAgeMs": storage["cargo"]["unusedAgeMs"], "units": scans["build"] + ([] if shared else scans["target"])},
        {"name": "cargo-incremental", "budgetBytes": storage["cargo"]["incremental"]["budgetBytes"], "unusedAgeMs": storage["cargo"]["incremental"]["unusedAgeMs"], "guardAgeMs": storage["cargo"]["incremental"]["guardAgeMs"], "units": scans["incremental"]["units"]},
        {"name": "cargo-incremental-sessions", "budgetBytes": None, "unusedAgeMs": 0, "units": scans["incremental"]["staleSessions"]},
        {"name": "vite", "budgetBytes": None, "unusedAgeMs": storage["vite"]["unusedAgeMs"], "units": scans["vite"]},
        {"name": "agents", "budgetBytes": None, "unusedAgeMs": storage["agents"]["unusedAgeMs"], "units": scans["agents"]},
    ]
    progress = [["cargo", u["path"]] for u in scans["build"]] + ([] if shared else [["cargo", u["path"]] for u in scans["target"]]) \
        + [["cargo-incremental-sessions" if u["kind"] == "cargo-incremental-session" else "cargo-incremental", u["path"]] for u in scans["incremental"]["units"] + scans["incremental"]["staleSessions"]] \
        + [["vite", u["path"]] for u in scans["vite"]] + [["agents", u["path"]] for u in scans["agents"]]
    return {"calls": calls, "areas": areas, "progress": progress}


def root(area, kind, build, target, repo):
    if area in ("cargo", "cargo-incremental", "cargo-incremental-sessions"):
        return target if kind == "cargo-target-file" else build
    return repo + "/cache/" + area


request = json.load(sys.stdin)
if request["op"] == "project":
    print(json.dumps([project(layout, request["storage"], request["repoRoot"]) for layout in request["layouts"]]))
else:
    print(json.dumps([root(r["area"], r["kind"], "/repo/b", "/repo/t", request["repoRoot"]) for r in request["rows"]]))

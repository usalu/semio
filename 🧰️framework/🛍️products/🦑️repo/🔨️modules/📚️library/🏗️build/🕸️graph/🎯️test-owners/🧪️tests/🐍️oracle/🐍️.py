"""Independent model of the test owner selection (see `🎯️test-owners/🟨️.mjs`). `python3 -I 🐍️.py generate <seed> <worlds>` prints a fixture (JSON) with the model's expectation for seeded random workspaces, `python3 -I 🐍️.py check <fixture.json>` recomputes every expectation and prints the failures. Standard library only.

The rule, stated once: a feature file `<owner>/🧪️tests/<case>/<feature>` belongs to the group keyed by its owner directory (two levels above the file). A group is discovered again when
- every group is (`all`), or
- the owner is already published (`known`) and its directory is a stale root or a stale root lies below it, or
- the owner is not published and a stale root is the root project, equals the directory, contains it or lies below it.
"""
import json
import random
import sys


def parent(path):
    cut = path.rfind("/")
    return path[:cut] if cut > 0 else ""


def select(features, all_, roots, known):
    groups = {}
    for feature in features:
        owner = parent(parent(feature))
        if all_:
            chosen = True
        elif owner in known:
            chosen = owner in roots or any(root.startswith(owner + "/") for root in roots)
        else:
            chosen = any(root == "." or owner == root or owner.startswith(root + "/") or root.startswith(owner + "/") for root in roots)
        if chosen:
            groups.setdefault(owner, []).append(feature)
    return [[owner, groups[owner]] for owner in groups]


def world(rng, index):
    modules = ["m%d" % i for i in range(rng.randint(2, 6))]
    owners = ["%s/%s/tests" % (rng.choice(modules), rng.choice(["a", "b", "c"])) for _ in range(rng.randint(2, 7))]
    features = []
    for owner in sorted(set(owners)):
        for case in range(rng.randint(1, 3)):
            features.append("%s/c%d/f.feature" % (owner, case))
    known = sorted(set(rng.sample(sorted(set(owners)), rng.randint(0, len(set(owners))))))
    candidates = sorted(set(owners)) + sorted(set(owner.rsplit("/", 1)[0] for owner in owners)) + sorted(set(f.rsplit("/", 1)[0] for f in features)) + [".", "zz/new"]
    roots = sorted(set(rng.sample(candidates, rng.randint(0, 3))))
    return {"name": "world %d" % index, "features": features, "all": rng.random() < 0.15, "roots": roots, "known": known}


def expected(row):
    return select(row["features"], row["all"], set(row["roots"]), set(row["known"]))


def main():
    command = sys.argv[1]
    if command == "generate":
        rng = random.Random(int(sys.argv[2]))
        worlds = [world(rng, index) for index in range(int(sys.argv[3]))]
        for row in worlds:
            row["groups"] = expected(row)
        print(json.dumps({"version": 1, "worlds": worlds}, ensure_ascii=False, indent=1))
    elif command == "check":
        data = json.load(open(sys.argv[2], encoding="utf8"))
        failures = [row["name"] for row in data["worlds"] if expected(row) != row["groups"]]
        print(json.dumps({"failures": failures}))
        sys.exit(1 if failures else 0)


main()

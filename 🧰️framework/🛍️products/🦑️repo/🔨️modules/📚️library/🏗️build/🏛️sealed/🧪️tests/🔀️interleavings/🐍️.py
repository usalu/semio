"""🏛 Independent Python model of the promotion of a sealed implementation snapshot (standard library only).

The model is the protocol, not the code: a live tree (a label per content state; a label is valid or broken), a store (current pointer, published snapshots, blocked record, one exclusive lock that is released when its holder dies) and processes that run the promotion one gate at a time. The gates are the step names of the TypeScript implementation (`start`, `read-current`, `drift-check`, `lock-attempt`, `reread`, `capture`, `validate`, `load`, `publish-snapshot`, `publish-pointer`, `gc`, `unlock`); the code between two gates is one atomic transition.

usage:
  🐍️.py generate <seed> <count> <corpus.json>   writes the language-agnostic corpus (cases + expected observables)
  🐍️.py expected <corpus.json>                   prints the expected observables of every case as JSON
  🐍️.py explore [quick]                          exhaustive search over all interleavings of small configurations, checks the laws
"""
import copy, json, os, random, sys

MUTATION = os.environ.get("SEALED_MODEL_MUTATION", "")

STEPS = ["start", "read-current", "drift-check", "lock-attempt", "reread", "capture", "validate", "load", "publish-snapshot", "publish-pointer", "gc", "unlock"]


class World:
    def __init__(self, trees, tree, current, published, processes):
        self.trees = trees
        self.tree = tree
        self.current = current
        self.published = set(published)
        self.blocked = None
        self.lock = None
        self.captures = {}
        self.procs = [{"gate": "start", "state": {}, "result": None, "crashed": False} for _ in range(processes)]
        self.violations = []

    def key(self):
        return json.dumps([self.tree, self.current, sorted(self.published), self.blocked, self.lock, sorted(self.captures.items()), [[p["gate"], p["result"], p["crashed"], sorted((k, v) for k, v in p["state"].items())] for p in self.procs]], sort_keys=True)

    def valid(self, label):
        return self.trees[label] == "valid"

    def done(self, index):
        proc = self.procs[index]
        return proc["gate"] is None or proc["crashed"]

    def alive(self):
        return [i for i in range(len(self.procs)) if not self.done(i)]

    def finish(self, index, status, label):
        proc = self.procs[index]
        proc["result"] = {"status": status, "label": label}
        proc["gate"] = None

    def release(self, index):
        if self.lock == index:
            self.lock = None


def to_unlock(world, index, outcome):
    proc = world.procs[index]
    proc["state"]["outcome"] = outcome
    world.release(index)
    proc["gate"] = "unlock"


def blocked_result(world, index, current):
    if current is None:
        world.finish(index, "none", None)
    else:
        world.finish(index, "blocked", current)


def advance(world, index):
    proc = world.procs[index]
    gate, state = proc["gate"], proc["state"]
    if gate == "start":
        proc["gate"] = "read-current"
    elif gate == "read-current":
        state["current"] = world.current
        proc["gate"] = "drift-check"
    elif gate == "drift-check":
        if state["current"] is not None and state["current"] == world.tree:
            world.finish(index, "fresh", state["current"])
        elif world.blocked is not None and world.blocked == world.tree:
            blocked_result(world, index, state["current"])
        else:
            proc["gate"] = "lock-attempt"
    elif gate == "lock-attempt":
        if world.lock is None or MUTATION == "no-lock":
            world.lock = index
            proc["gate"] = "capture" if MUTATION == "skip-reread" else "reread"
        elif world.current is not None and world.current == world.tree and world.current != state["current"]:
            proc["gate"] = "unlock"
            state["outcome"] = None
        else:
            proc["gate"] = "lock-attempt"
    elif gate == "reread":
        state["current"] = world.current
        if state["current"] is not None and state["current"] == world.tree:
            to_unlock(world, index, ("served", state["current"]))
        elif world.blocked is not None and world.blocked == world.tree:
            to_unlock(world, index, ("blocked", state["current"]))
        else:
            proc["gate"] = "capture"
    elif gate == "capture":
        label = world.tree
        state["candidate"] = label
        world.captures[label] = world.captures.get(label, 0) + 1
        if state["current"] == label or label in world.published:
            proc["gate"] = "publish-snapshot"
        else:
            proc["gate"] = "validate"
    elif gate == "validate":
        proc["gate"] = "load"
    elif gate == "load":
        label = state["candidate"]
        if not world.valid(label) and MUTATION != "skip-validate":
            world.blocked = label
            to_unlock(world, index, ("blocked", state["current"]))
        else:
            proc["gate"] = "publish-snapshot"
    elif gate == "publish-snapshot":
        world.published.add(state["candidate"])
        proc["gate"] = "publish-pointer"
    elif gate == "publish-pointer":
        label = state["candidate"]
        world.current = label
        world.blocked = None
        state["result"] = ("refreshed" if state["current"] == label else "promoted", label)
        proc["gate"] = "gc"
    elif gate == "gc":
        to_unlock(world, index, state["result"])
    elif gate == "unlock":
        outcome = state["outcome"]
        if outcome is None:
            if world.current is None:
                world.finish(index, "none", None)
            elif world.current == world.tree:
                world.finish(index, "served", world.current)
            else:
                world.finish(index, "busy", world.current)
        elif outcome[0] == "blocked":
            blocked_result(world, index, outcome[1])
        else:
            world.finish(index, outcome[0], outcome[1])
    else:
        raise AssertionError(gate)


def crash(world, index):
    proc = world.procs[index]
    proc["crashed"] = True
    world.release(index)


def laws(world):
    errors = []
    if world.current is not None and (world.current not in world.published or not world.valid(world.current)):
        errors.append("the pointer names a snapshot that is not published or not valid")
    holders = [i for i, p in enumerate(world.procs) if p["gate"] in ("reread", "capture", "validate", "load", "publish-snapshot", "publish-pointer", "gc") and not p["crashed"]]
    if len(holders) > 1 or (holders and world.lock != holders[0]):
        errors.append("two lock holders or a holder without the lock")
    for p in world.procs:
        if p["result"] and p["result"]["status"] in ("fresh", "served", "promoted", "refreshed", "blocked") and p["result"]["label"] is not None and not world.valid(p["result"]["label"]):
            errors.append("a process runs a broken snapshot: " + str(p["result"]))
    return errors


def observe(world):
    return {
        "current": world.current,
        "blocked": world.blocked,
        "captures": dict(sorted(world.captures.items())),
        "results": [None if p["crashed"] else p["result"] for p in world.procs],
        "lockFree": world.lock is None,
    }


def run_case(case):
    trees = case["trees"]
    world = World(trees, case["initial"]["current"] or case["initial"]["tree"], case["initial"]["current"], [case["initial"]["current"]] if case["initial"]["current"] else [], case["processes"])
    world.tree = case["initial"]["tree"]
    for token in case["tokens"]:
        apply_token(world, token)
    turn = 0
    while world.alive():
        alive = world.alive()
        advance(world, alive[turn % len(alive)])
        turn += 1
    return observe(world)


def apply_token(world, token):
    kind = token[0]
    if kind == "p":
        if not world.done(token[1]):
            advance(world, token[1])
    elif kind == "edit":
        world.tree = token[1]
    elif kind == "crash":
        if not world.done(token[1]):
            crash(world, token[1])


def random_case(rng, index):
    trees = {"A": "valid", "B": "valid", "C": "valid", "X": "broken"}
    processes = rng.choice([1, 2, 2, 3])
    current = rng.choice([None, "A", "A", "B"])
    tree = rng.choice(["A", "B", "X"]) if current else rng.choice(["A", "B"])
    tokens, edits, crashes = [], 0, 0
    for _ in range(rng.randint(0, processes * 11)):
        pick = rng.random()
        if pick < 0.08 and edits < 3:
            tokens.append(["edit", rng.choice(sorted(trees))]); edits += 1
        elif pick < 0.12 and crashes < 1:
            tokens.append(["crash", rng.randrange(processes)]); crashes += 1
        else:
            tokens.append(["p", rng.randrange(processes)])
    case = {"id": "random-%03d" % index, "trees": trees, "initial": {"current": current, "tree": tree}, "processes": processes, "tokens": tokens}
    return case


def generate(seed, count, path):
    rng = random.Random(seed)
    cases = []
    for i in range(count):
        case = random_case(rng, i)
        case["expected"] = run_case(case)
        cases.append(case)
    json.dump({"version": 1, "cases": cases}, open(path, "w", encoding="utf-8"), ensure_ascii=False, indent=1)


def explore():
    trees = {"A": "valid", "B": "valid", "X": "broken"}
    configurations = []
    for processes in ((2,) if "quick" in sys.argv else (2, 3)):
        for current, tree in ((None, "A"), ("A", "A"), ("A", "B"), ("A", "X"), (None, "X")):
            for edits in ([], ["B"], ["X"], ["B", "X"]):
                configurations.append((processes, current, tree, tuple(edits)))
    checked = 0
    for processes, current, tree, edits in configurations:
        world = World(trees, tree, current, [current] if current else [], processes)
        seen = set()
        stack = [(world, 0, 0)]
        while stack:
            at, edit_index, crashed = stack.pop()
            key = at.key() + "|%d|%d" % (edit_index, crashed)
            if key in seen:
                continue
            seen.add(key)
            checked += 1
            errors = laws(at)
            if errors:
                raise SystemExit("LAW BROKEN %s in %s" % (errors, (processes, current, tree, edits)))
            moves = [("p", i) for i in at.alive()]
            if edit_index < len(edits):
                moves.append(("edit", edits[edit_index]))
            if not crashed:
                moves += [("crash", i) for i in at.alive()]
            if not moves:
                final(at, current, tree, edits, crashed)
                continue
            for move in moves:
                nxt = copy.deepcopy(at)
                nedit, ncrash = edit_index, crashed
                apply_token(nxt, list(move))
                if move[0] == "edit":
                    nedit += 1
                if move[0] == "crash":
                    ncrash += 1
                if not nxt.alive() and nedit < len(edits):
                    pass
                stack.append((nxt, nedit, ncrash))
    print(json.dumps({"configurations": len(configurations), "states": checked}))


def final(world, current, tree, edits, crashed):
    if world.lock is not None:
        raise SystemExit("LAW BROKEN lock held at the end")
    if not crashed and not edits and tree == "A" and current is None and world.captures.get("A", 0) != 1:
        raise SystemExit("LAW BROKEN one capture per digest: %s" % world.captures)
    if not crashed and not edits:
        if world.captures.get(tree, 0) > 1:
            raise SystemExit("LAW BROKEN a tree was captured twice: %s" % world.captures)
        if world.valid(tree) and world.current != tree and not any(p["crashed"] for p in world.procs):
            raise SystemExit("LAW BROKEN a quiet valid tree was not promoted: current=%s tree=%s" % (world.current, tree))
        if not world.valid(tree) and world.current != current:
            raise SystemExit("LAW BROKEN a broken tree changed the pointer")


if __name__ == "__main__":
    command = sys.argv[1]
    if command == "generate":
        generate(int(sys.argv[2]), int(sys.argv[3]), sys.argv[4])
    elif command == "expected":
        print(json.dumps([run_case(case) for case in json.load(open(sys.argv[2], encoding="utf-8"))["cases"]], ensure_ascii=False))
    elif command == "explore":
        explore()
    else:
        raise SystemExit(__doc__)

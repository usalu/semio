"""🔐️ Independent Python model of Cargo's fine-grain unit locking under the four admission regimes of a build directory (standard library only).
usage: 🐍️.py <scenarios.json>  (reads the scenario list, prints one verdict per scenario as JSON), or 🐍️.py --sweep (every pair of process shapes over three units, counted per regime).

Cargo (`-Zfine-grain-locking`): a process takes a SHARED flock on every unit of its graph during the traversal; a dirty unit's shared lock is dropped at once and re-taken as EXCLUSIVE when its job starts, while the shared locks of all FRESH units are kept until the end. Regimes: `free` (the processes share one build directory), `directory` (one custody for the whole directory), `lanes` (every process has its own directory), `units` (custody per unit, taken in ascending unit order before Cargo starts: exclusive for the units the process might dirty, shared for the rest).
A verdict says whether some interleaving deadlocks (`deadlock`) and whether some state has two processes inside Cargo at the same time (`overlap`)."""
import json, sys
from functools import lru_cache


def verdict(scenario):
    regime, processes = scenario["regime"], scenario["processes"]
    count = len(processes)
    graphs = [tuple(p["graph"]) for p in processes]
    dirty = [frozenset(p["dirty"]) for p in processes]
    might = [frozenset(p.get("mightDirty", p["dirty"])) for p in processes]

    def namespaced(index, unit):
        return (index, unit) if regime == "lanes" else (0, unit)

    def program(index):
        steps = []
        if regime == "directory":
            steps.append(("custody", ("dir",), "x"))
        if regime == "units":
            for unit in sorted(graphs[index]):
                steps.append(("custody", ("unit", unit), "x" if unit in might[index] else "s"))
        steps.append(("enter",))
        for unit in graphs[index]:
            steps.append(("lock", namespaced(index, unit), "s"))
            if unit in dirty[index]:
                steps.append(("unlock", namespaced(index, unit), "s"))
        for unit in graphs[index]:
            if unit in dirty[index]:
                steps.append(("lock", namespaced(index, unit), "x"))
                steps.append(("unlock", namespaced(index, unit), "x"))
        for unit in graphs[index]:
            if unit not in dirty[index]:
                steps.append(("unlock", namespaced(index, unit), "s"))
        steps.append(("leave",))
        steps.append(("release",))
        return steps

    programs = [program(i) for i in range(count)]

    def holders(held, key, exclude):
        readers, writers = 0, 0
        for other in range(count):
            if other == exclude:
                continue
            for (k, mode) in held[other]:
                if k == key:
                    if mode == "x":
                        writers += 1
                    else:
                        readers += 1
        return readers, writers

    def enabled(state, index):
        position, held = state
        if position[index] >= len(programs[index]):
            return False
        step = programs[index][position[index]]
        if step[0] in ("custody", "lock"):
            readers, writers = holders(held, ("c" if step[0] == "custody" else "f", step[1]), index)
            return writers == 0 and (step[2] == "s" or readers == 0)
        return True

    def apply(state, index):
        position, held = state
        step = programs[index][position[index]]
        mine = set(held[index])
        if step[0] in ("custody", "lock"):
            mine.add((("c" if step[0] == "custody" else "f", step[1]), step[2]))
        elif step[0] == "unlock":
            mine.discard((("f", step[1]), step[2]))
        elif step[0] == "release":
            mine = {row for row in mine if row[0][0] != "c"}
        new_position = position[:index] + (position[index] + 1,) + position[index + 1:]
        new_held = held[:index] + (frozenset(mine),) + held[index + 1:]
        return (new_position, new_held)

    start = (tuple(0 for _ in range(count)), tuple(frozenset() for _ in range(count)))
    seen, stack, deadlock, overlap = {start}, [start], False, False
    while stack:
        state = stack.pop()
        position = state[0]
        inside = [i for i in range(count) if programs[i].index(("enter",)) < position[i] <= programs[i].index(("leave",))]
        if len(inside) > 1:
            overlap = True
        moves = [i for i in range(count) if enabled(state, i)]
        if not moves and any(position[i] < len(programs[i]) for i in range(count)):
            deadlock = True
        for i in moves:
            nxt = apply(state, i)
            if nxt not in seen:
                seen.add(nxt)
                stack.append(nxt)
    return {"id": scenario["id"], "deadlock": deadlock, "overlap": overlap, "states": len(seen)}


def sweep():
    from itertools import combinations, product
    units = ["x", "y", "s"]
    shapes = []
    for size in (1, 2, 3):
        for graph in combinations(units, size):
            for dirt in product([False, True], repeat=size):
                shapes.append({"graph": list(graph), "dirty": [u for u, d in zip(graph, dirt) if d]})
    counts = {}
    for regime in ("free", "directory", "lanes", "units", "units-under"):
        deadlocks, overlaps, total = 0, 0, 0
        for a, b in combinations(shapes, 2):
            if regime == "units-under":
                processes, real = [{**a, "mightDirty": []}, {**b, "mightDirty": []}], "units"
            else:
                processes, real = [a, b], regime
            result = verdict({"id": "s", "regime": real, "processes": processes})
            total += 1
            deadlocks += result["deadlock"]
            overlaps += result["overlap"]
        counts[regime] = {"pairs": total, "deadlocks": deadlocks, "overlaps": overlaps}
    return counts


if __name__ == "__main__":
    if sys.argv[1] == "--sweep":
        print(json.dumps(sweep()))
    else:
        print(json.dumps([verdict(s) for s in json.load(open(sys.argv[1], encoding="utf-8"))], ensure_ascii=False))

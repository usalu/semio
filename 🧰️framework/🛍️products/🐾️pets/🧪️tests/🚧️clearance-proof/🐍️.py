#!/usr/bin/env python3
"""🚧️ Oracle of the pets product's clearance (design §18 of the second round), in Python.

Nothing here walks an interval or a list of obstacles the way the subjects do. Boxes are numpy rows
``x0 y0 x1 y1`` and two of them overlap when the rectangle they share has a positive width and a positive
height (``minimum`` of the far edges minus ``maximum`` of the near edges) — an absent box is a row of NaN
and shares nothing. Every committed coordinate lies on the quarter-pixel lattice and the seam is 1/64 px, so
every place a subject may answer lies on the 1/64 px lattice and the references search that lattice:

* a free place (``slotIn``, ``spotOn``, ``columnOver``) is the lattice place nearest to the wanted one at
  which the box, grown by a seam to either side, shares nothing with any obstacle (``numpy.argmin`` over the
  admissible places, the first one among equals);
* a guarded stride (``guardedStride``, ``slideOf``) is the last step of a walk along the lattice before the
  front of the box, a seam ahead, reaches an obstacle that lies ahead;
* a claim is sampled densely: the path is an array of one box per tick, its slices are the hulls
  ``numpy.minimum.reduceat`` and ``numpy.maximum.reduceat`` cut out of it, and every owner becomes a time
  line of one box per tick up to a horizon beyond the last plan (its body, then its slices, then its rest).
  A plan is clear when its time line shares nothing with any other time line at any tick; a place is free
  from a tick on when it shares nothing with the bodies, with the rows of the other time lines from that
  tick on, and with their rests;
* a seating (``seatOf``) is a quadratic programme: the least squares of the moves, subject to "a seam
  between neighbours, the order kept, every box without its margin on the perch". It is answered as the
  plain binary64 result of pooling adjacent violators written out below (the comparison grid of
  ``pets-float-v1`` is decimal and a number that is right to 1e-13 can round to the other side), and
  refused unless ``scipy.optimize.isotonic_regression`` (clipped into the bounds) reaches the same seats
  within 1e-9 and ``scipy.optimize.minimize`` (SLSQP) on the programme itself within 1e-5. Who has to leave
  is whoever makes the programme infeasible, the last in the list first.

Heads are found with boolean masks over all tops at once, the push of a held body is the stated rule
replayed and held to two judgements of numpy (the place it ends at shares nothing, and out of a single
obstacle it is the shortest of the four ways along the axes), and the last resort is read from the dense
time lines.

@see https://docs.scipy.org/doc/scipy/reference/generated/scipy.optimize.isotonic_regression.html
@see https://docs.scipy.org/doc/scipy/reference/optimize.minimize-slsqp.html
@see https://numpy.org/doc/stable/reference/generated/numpy.ufunc.reduceat.html
@see ../../🧫️fixtures/🚧️clearance-proof/🔣️.json
"""

# region 🔖️Imports
import json
import math

import numpy
from scipy import optimize

from semio_repo_test import Adapter, Outcome

# endregion 🔖️Imports


# region 🔖️Reference
VECTORS = "shared://🚧️clearance-proof/🔣️.json"
MARGIN = 4
SEAM = 0.015625
FOREVER = 4294967295
STEERINGS = [0, 30, -30, 60, -60, 90, -90, 130, -130, 180, -180, 240, -240]
PUSHES = 4
PATIENCE = 40
DELAY = 32
SLIDE_OFF_SPEED = 32
SLIDE_OFF_GAIN = 2
SLIDE_OFF_LIMIT = 128
LEAN = 0.25
LANE_LIFT = 0.9
SCOOT_HASTE = 1.5
TICKS_PER_SECOND = 64
LATTICE = 64
AGREEMENT = 1e-12
ISOTONIC = 1e-9
PROGRAMME = 1e-5
CONSTANTS = {
    "margin": MARGIN,
    "seam": SEAM,
    "forever": FOREVER,
    "steerings": STEERINGS,
    "pushes": PUSHES,
    "patience": PATIENCE,
    "delay": DELAY,
    "slideOffSpeed": SLIDE_OFF_SPEED,
    "slideOffGain": SLIDE_OFF_GAIN,
    "slideOffLimit": SLIDE_OFF_LIMIT,
    "lean": LEAN,
    "laneLift": LANE_LIFT,
    "scootHaste": SCOOT_HASTE,
}


def rows(boxes):
    """🧮️ Boxes (``[x0, y0, x1, y1]`` each) as rows of an array; an absent box is a row of NaN."""
    return numpy.array([[numpy.nan] * 4 if box is None else box for box in boxes], dtype=float).reshape(-1, 4)


def boxed(row):
    """📦️ A row as a box, ``None`` for a row of NaN."""
    return None if bool(numpy.isnan(row).any()) else [float(edge) for edge in row]


def sharing(one, other):
    """💥️ Whether boxes share an area: the rectangle they have in common has a positive width and a positive height."""
    width = numpy.minimum(one[..., 2], other[..., 2]) - numpy.maximum(one[..., 0], other[..., 0])
    height = numpy.minimum(one[..., 3], other[..., 3]) - numpy.maximum(one[..., 1], other[..., 1])
    return (width > 0) & (height > 0)


def whole(value, what):
    """🔢️ A number of lattice steps; a length off the 1/64 px lattice is refused."""
    count = value * LATTICE
    if count != round(count):
        raise AssertionError("%s: %r is not a whole number of lattice steps" % (what, value))
    return int(round(count))


def body(vector):
    """📐️ The box of an actor: the corners numpy adds to its feet."""
    feet, size, kind = vector["feet"], vector["size"], vector["posture"]
    if kind["kind"] == "leaning":
        left = right = LEAN * kind["height"] * abs(kind["sine"])
        above = 0
    elif kind["kind"] == "canopied":
        left = right = max((kind["canopy"]["width"] - size["width"]) / 2, 0)
        above = max(kind["canopy"]["height"], 0)
    elif kind["kind"] == "plain":
        left, right, above = kind["left"], kind["right"], kind["above"]
    else:
        left = right = above = 0
    margin = vector["margin"]
    corners = numpy.array([feet["x"], feet["y"], feet["x"], feet["y"]]) + numpy.array([-(size["width"] / 2 + left + margin), -(size["height"] + above + margin), size["width"] / 2 + right + margin, vector["hover"] + margin])
    return boxed(corners)


def crowd(vector):
    """🚨️ The pairs of a crowd that overlap, and the pairs that are apart but closer than the margin on both axes, by the first and then the second in the list."""
    boxes = rows([entry["box"] for entry in vector["bodies"]])
    owners = [entry["owner"] for entry in vector["bodies"]]
    over = sharing(boxes[:, None, :], boxes[None, :, :])
    across = numpy.maximum(boxes[None, :, 0] - boxes[:, None, 2], boxes[:, None, 0] - boxes[None, :, 2])
    down = numpy.maximum(boxes[None, :, 1] - boxes[:, None, 3], boxes[:, None, 1] - boxes[None, :, 3])
    near = (across < vector["margin"]) & (down < vector["margin"]) & ~over
    later = numpy.triu(numpy.ones(over.shape, dtype=bool), 1)
    return {"overlaps": [[owners[first], owners[second]] for first, second in numpy.argwhere(over & later)], "nears": [[owners[first], owners[second]] for first, second in numpy.argwhere(near & later)]}


def slot(x, box, low, high, obstacles):
    """🎯️ The lattice place between two ends nearest to ``x`` at which the box, a seam wider to either side, shares nothing with any obstacle; the first one among equals, ``None`` without one."""
    if not low <= high:
        return None
    places = low + numpy.arange(whole(high - low, "the stretch") + 1) / LATTICE
    whole(x - low, "the wanted place")
    there = numpy.stack([box[0] + (places - x) - SEAM, numpy.full(places.size, box[1]), box[2] + (places - x) + SEAM, numpy.full(places.size, box[3])], axis=1)
    admissible = numpy.ones(places.size, dtype=bool)
    for obstacle in rows(obstacles):
        admissible &= ~sharing(there, obstacle[None, :])
    if not admissible.any():
        return None
    free = places[admissible]
    return float(free[numpy.argmin(numpy.abs(free - x))])


def spot(vector):
    """📍️ A free place: between two ends (``slot``), on a perch with the footprint on it (``spot``), or the column down to a perch (``column``)."""
    box = vector["box"]
    if vector["kind"] == "slot":
        return slot(vector["x"], box, vector["low"], vector["high"], vector["obstacles"])
    perch, foot = vector["perch"], vector["foot"]
    if vector["kind"] == "column":
        box = box[:3] + [box[3] + max(vector["drop"], 0)]
    return slot(vector["x"], box, perch["x0"] + foot, perch["x1"] - foot, vector["obstacles"])


def walked(box, wanted, obstacles):
    """👣️ How far a walk along the lattice gets before the front of the box, a seam ahead, reaches an obstacle of its height whose middle lies ahead."""
    if wanted == 0:
        return 0.0
    side = 1 if wanted > 0 else -1
    steps = numpy.arange(1, whole(abs(wanted), "the stride") + 1) / LATTICE
    blocked = numpy.zeros(steps.size, dtype=bool)
    for obstacle in rows(obstacles):
        if not min(box[3], obstacle[3]) - max(box[1], obstacle[1]) > 0:
            continue
        if not (obstacle[0] + obstacle[2] - box[0] - box[2]) * side > 0:
            continue
        blocked |= (box[2] + steps + SEAM > obstacle[0]) if side > 0 else (box[0] - steps - SEAM < obstacle[2])
    reach = int(numpy.argmax(blocked)) if blocked.any() else steps.size
    return 0.0 if reach == 0 else float(side * steps[reach - 1])


def order(vector):
    """🧷️ The owners from left to right (a stable ``numpy.argsort`` of the middles), whether a second order keeps the first, and whether a rise vaults a hurdle."""
    if vector["kind"] == "rank":
        boxes = rows([entry["box"] for entry in vector["bodies"]])
        return [vector["bodies"][int(index)]["owner"] for index in numpy.argsort(boxes[:, 0] + boxes[:, 2], kind="stable")]
    if vector["kind"] == "kept":
        places = numpy.array([vector["after"].index(owner) for owner in vector["before"] if owner in vector["after"]], dtype=int)
        return bool(numpy.all(numpy.diff(places) > 0))
    return bool(numpy.less_equal(vector["hopper"][3] - vector["rise"], vector["hurdle"][1] - SEAM))


def slices(spec):
    """🎞️ The slices of a claim as rows ``from until x0 y0 x1 y1``: the hulls numpy reduces out of the path, a span of ticks at a time."""
    path = rows(spec["path"])
    if path.shape[0] == 0:
        return []
    width = max(int(math.floor(spec["span"])), 1)
    starts = numpy.arange(0, path.shape[0], width)
    near = numpy.minimum.reduceat(path[:, :2], starts, axis=0)
    far = numpy.maximum.reduceat(path[:, 2:], starts, axis=0)
    ends = numpy.minimum(starts + width, path.shape[0]) - 1
    return [[int(spec["from"] + start), int(spec["from"] + end), float(low[0]), float(low[1]), float(high[0]), float(high[1])] for start, end, low, high in zip(starts, ends, near, far)]


def horizon(specs):
    """🌅️ A tick beyond the last slice of every claim: from there on everybody rests."""
    last = [cut[-1][1] for cut in (slices(spec) for spec in specs) if cut]
    return max(last + [0]) + 2


def line(own, spec, span):
    """🧵️ The box of an owner at every tick from 0 to the horizon: its body until its claim begins (for ever without one), the slices of the claim, then its rest."""
    boxes = numpy.full((span + 1, 4), numpy.nan)
    cut = slices(spec) if spec is not None else []
    if not cut:
        if own is not None:
            boxes[:] = own
        return boxes
    if own is not None:
        boxes[: cut[0][0]] = own
    for start, end, x0, y0, x1, y1 in cut:
        boxes[start : end + 1] = [x0, y0, x1, y1]
    if spec["rest"] is not None:
        boxes[cut[-1][1] + 1 :] = spec["rest"]
    return boxes


def clear(plan, bodies, claims):
    """🚦️ Whether a plan shares nothing with anybody at any tick: its time line against the time line of every other owner."""
    span = horizon([plan] + claims)
    mine = line(None, plan, span)
    embodied = []
    for entry in bodies:
        if entry["owner"] == plan["owner"]:
            continue
        own = next((index for index, other in enumerate(claims) if other["owner"] == entry["owner"]), None)
        if own is not None:
            embodied.append(own)
        if sharing(mine, line(entry["box"], None if own is None else claims[own], span)).any():
            return False
    for index, other in enumerate(claims):
        if other["owner"] == plan["owner"] or index in embodied:
            continue
        if sharing(mine, line(None, other, span)).any():
            return False
    return True


def ahead(spec, tick):
    """🔭️ What is left of a claim from a tick on: the rows of its time line from that tick to its last slice, the number of slices they come from, and its rest."""
    cut = slices(spec)
    if not cut:
        return numpy.empty((0, 4)), 0, spec["rest"]
    dense = line(None, spec, cut[-1][1])
    start = max(tick, cut[0][0])
    width = max(int(math.floor(spec["span"])), 1)
    return dense[start : cut[-1][1] + 1], int(numpy.unique((numpy.arange(start, cut[-1][1] + 1) - cut[0][0]) // width).size), spec["rest"]


def free(box, owner, bodies, claims, tick):
    """✅️ Whether a place shares nothing with the bodies of the others, with what is left of their claims from a tick on, and with their rests."""
    if box is None:
        return False
    probe = rows([box])
    if sharing(probe, rows([entry["box"] for entry in bodies if entry["owner"] != owner])).any():
        return False
    for other in claims:
        if other["owner"] == owner:
            continue
        left, _count, rest = ahead(other, tick)
        if sharing(probe, left).any() or sharing(probe, rows([rest])).any():
            return False
    return True


def claim(vector):
    """🎫️ Everything asked about one plan: its slices, where it is at the stated ticks, whether it is clear, and for a probe at every stated tick whether its place is free, how many obstacles there are, what is left of the claims, and whose claims remain once the prober's are released."""
    plan, bodies, claims = vector["plan"], vector["bodies"], vector["claims"]
    cut = slices(plan)
    span = max(horizon([plan] + claims), max(vector["ticks"] + [0]) + 1)
    dense = line(None, plan, span)
    obstacles, pruned = [], []
    for tick in vector["ticks"]:
        lefts = [(other["owner"], ahead(other, tick)) for other in claims]
        obstacles.append(sum(1 for entry in bodies if entry["owner"] != vector["prober"]) + sum(count + (0 if rest is None else 1) for owner, (_left, count, rest) in lefts if owner != vector["prober"]))
        pruned.append([[owner, count] for owner, (_left, count, _rest) in lefts if count > 0])
    return {
        "slices": cut,
        "at": [None if not cut or tick < cut[0][0] else boxed(dense[tick]) for tick in vector["ticks"]],
        "clear": clear(plan, bodies, claims),
        "free": [free(vector["probe"], vector["prober"], bodies, claims, tick) for tick in vector["ticks"]],
        "obstacles": obstacles,
        "pruned": pruned,
        "released": [other["owner"] for other in claims if other["owner"] != vector["prober"]],
    }


def pooled(wanted):
    """🪣️ Pool adjacent violators, written out: a block is the mean of its members, and two neighbouring blocks merge while the left mean is larger than the right one."""
    sums, counts = [], []
    for value in wanted:
        sums.append(value)
        counts.append(1)
        while len(sums) > 1 and sums[-2] / counts[-2] > sums[-1] / counts[-1]:
            total, members = sums.pop(), counts.pop()
            sums[-1] = sums[-1] + total
            counts[-1] = counts[-1] + members
    return sums, counts


def feasible(boxes, low, high):
    """🧩️ Whether boxes in this order fit side by side between two ends, a seam between neighbours: the seating packed against the left end keeps the right end."""
    widths = boxes[:, 2] - boxes[:, 0]
    packed = low + numpy.concatenate([[0], numpy.cumsum(widths[:-1] + SEAM)])
    return bool(packed[-1] + widths[-1] <= high)


def programme(boxes, low, high):
    """🏫️ The seats SLSQP finds: least squares of the moves subject to a seam between neighbours and both ends of the perch, started from the seating packed against the left end. SLSQP may stop at the solution with status 8 (no direction left to descend in, as when the perch leaves no room to move at all); what it returns must keep every constraint, and the caller holds it to the pooled seats."""
    widths = boxes[:, 2] - boxes[:, 0]
    wanted = boxes[:, 0]
    start = low + numpy.concatenate([[0], numpy.cumsum(widths[:-1] + SEAM)])
    constraints = [{"type": "ineq", "fun": lambda places: places[0] - low}, {"type": "ineq", "fun": lambda places: high - widths[-1] - places[-1]}]
    for at in range(len(widths) - 1):
        constraints.append({"type": "ineq", "fun": lambda places, at=at: places[at + 1] - places[at] - widths[at] - SEAM})
    found = optimize.minimize(lambda places: float(numpy.sum((places - wanted) ** 2)), start, jac=lambda places: 2 * (places - wanted), method="SLSQP", constraints=constraints, options={"ftol": 1e-12, "maxiter": 500})
    if found.status not in (0, 8) or min(float(constraint["fun"](found.x)) for constraint in constraints) < -PROGRAMME:
        raise AssertionError("SLSQP did not solve the programme: %s" % found.message)
    return found.x


def seating(vector):
    """💺️ The seating of a perch: who leaves, how far everybody else moves, and where the first tick of the scoot takes them. The moves are judged by scipy's isotonic regression and by SLSQP on the quadratic programme."""
    bodies, perch, margin = vector["bodies"], vector["perch"], vector["margin"]
    boxes = rows([entry["box"] for entry in bodies])
    members = [int(index) for index in numpy.argsort(boxes[:, 0] + boxes[:, 2], kind="stable")]
    leavers = []
    low = perch["x0"] - margin
    while members and not feasible(boxes[members], low, perch["x1"] + margin):
        worst = max(members)
        leavers.append(bodies[worst]["owner"])
        members.remove(worst)
    if not members:
        return {"seats": [], "leavers": leavers, "fraction": 1, "places": []}
    offsets = [0]
    for at in range(1, len(members)):
        before = boxes[members[at - 1]]
        offsets.append(offsets[at - 1] + (before[2] - before[0]) + SEAM)
    last = boxes[members[-1]]
    high = perch["x1"] + margin - (last[2] - last[0]) - offsets[-1]
    wanted = [float(boxes[member][0] - offsets[at]) for at, member in enumerate(members)]
    sums, counts = pooled(wanted)
    shifts, at = [], 0
    for total, count in zip(sums, counts):
        mean = min(max(total / count, low), high)
        for _ in range(count):
            shifts.append(float(mean - wanted[at]))
            at += 1
    judged = numpy.clip(optimize.isotonic_regression(numpy.array(wanted), increasing=True).x, low, high) - numpy.array(wanted)
    if float(numpy.abs(judged - numpy.array(shifts)).max()) > ISOTONIC:
        raise AssertionError("seatings/%s: pooling answers %r, scipy's isotonic regression %r" % (vector["id"], shifts, judged.tolist()))
    solved = programme(boxes[members], low, perch["x1"] + margin) - boxes[members, 0]
    if float(numpy.abs(solved - numpy.array(shifts)).max()) > PROGRAMME:
        raise AssertionError("seatings/%s: pooling answers %r, SLSQP on the programme %r" % (vector["id"], shifts, solved.tolist()))
    farthest = float(numpy.abs(numpy.array(shifts)).max())
    stride = vector["stride"]
    fraction = 1 if not farthest > 0 or not farthest > stride else max(stride, 0) / farthest
    places = []
    for member, shift in zip(members, shifts):
        start = float(boxes[member][0])
        goal = start + shift
        places.append(goal if fraction >= 1 else start + (goal - start) * fraction)
    widths = boxes[members, 2] - boxes[members, 0]
    gaps = boxes[members[1:], 0] - boxes[members[:-1], 2]
    for share in numpy.linspace(0, 1, 9):
        there = boxes[members, 0] + numpy.array(shifts) * share
        if not numpy.all(there[1:] - (there[:-1] + widths[:-1]) >= numpy.minimum(gaps, SEAM) - ISOTONIC):
            raise AssertionError("seatings/%s: a share of %r of the way brings neighbours closer than they were and than a seam" % (vector["id"], share))
    return {"seats": [[bodies[member]["owner"], shift] for member, shift in zip(members, shifts)], "leavers": leavers, "fraction": fraction, "places": places}


def head(vector):
    """🎩️ What is asked about heads: the host a descending box lands on (boolean masks over all tops) and how far it is lifted onto it, whether a rider still rests on its host, the side it slides off to, the strides of a slide, and one tick of a guarded slide."""
    kind = vector["kind"]
    if kind == "landing":
        before, after = vector["before"], vector["after"]
        boxes = rows([entry["box"] for entry in vector["bodies"]])
        if boxes.shape[0] == 0:
            return {"host": None, "lift": None}
        tops = boxes[:, 1]
        crossed = (before[3] <= tops) & (after[3] > tops - SEAM) & (after[3] >= before[3])
        beside = numpy.minimum(after[2], boxes[:, 2]) - numpy.maximum(after[0], boxes[:, 0]) > 0
        others = numpy.array([entry["owner"] != vector["owner"] for entry in vector["bodies"]])
        landed = crossed & beside & others
        if not landed.any():
            return {"host": None, "lift": None}
        host = int(numpy.argmin(numpy.where(landed, tops, numpy.inf)))
        return {"host": vector["bodies"][host]["owner"], "lift": float(tops[host] - SEAM - after[3])}
    if kind == "resting":
        rider, host = vector["rider"], vector["host"]
        gap = host[1] - rider[3]
        return bool(min(rider[2], host[2]) - max(rider[0], host[0]) > 0 and 0 <= gap <= 2 * SEAM)
    if kind == "side":
        rider, host = vector["rider"], vector["host"]
        return 1 if (rider[0] + rider[2]) / 2 >= (host[0] + host[2]) / 2 else -1
    if kind == "strides":
        return [float(stride) for stride in numpy.minimum(SLIDE_OFF_SPEED + SLIDE_OFF_GAIN * numpy.array(vector["ticks"]), SLIDE_OFF_LIMIT) / TICKS_PER_SECOND]
    rider = vector["rider"]
    wanted = float(min(SLIDE_OFF_SPEED + SLIDE_OFF_GAIN * vector["ticks"], SLIDE_OFF_LIMIT) / TICKS_PER_SECOND)
    room = max(vector["high"] - rider[2], 0) if vector["side"] > 0 else max(rider[0] - vector["low"], 0)
    stride = walked(rider, vector["side"] * min(wanted, room), vector["obstacles"])
    return [0.0, -vector["side"]] if stride == 0 else [stride, vector["side"]]


def push(vector):
    """🫸️ The push of a held body out of its obstacles: the stated rule replayed, and judged twice by numpy — the place it ends at shares nothing with any obstacle, and out of a single obstacle it is the shortest of the four ways along the axes, a seam to spare."""
    extent, obstacles = rows([vector["box"]])[0], rows(vector["obstacles"])
    move = numpy.zeros(2)
    settled = False
    for _ in range(vector["iterations"]):
        clean = True
        for obstacle in obstacles:
            there = extent + numpy.array([move[0], move[1], move[0], move[1]])
            if not bool(sharing(there, obstacle)):
                continue
            clean = False
            ways = [there[2] - obstacle[0] + SEAM, obstacle[2] - there[0] + SEAM, there[3] - obstacle[1] + SEAM, obstacle[3] - there[1] + SEAM]
            way = int(numpy.argmin(ways))
            move = move + numpy.array([[-ways[0], 0], [ways[1], 0], [0, -ways[2]], [0, ways[3]]][way])
        if clean:
            settled = True
            break
    there = extent + numpy.array([move[0], move[1], move[0], move[1]])
    if not settled and bool(sharing(there[None, :], obstacles).any()):
        return None
    if bool(sharing(there[None, :], obstacles).any()):
        raise AssertionError("pushes/%s: the push ends inside an obstacle" % vector["id"])
    if obstacles.shape[0] == 1 and bool(sharing(extent, obstacles[0])) and vector["iterations"] > 0:
        only = obstacles[0]
        steps = numpy.arange(1, 1024 * LATTICE) / LATTICE
        ways = []
        for axis, side in ((0, -1), (0, 1), (1, -1), (1, 1)):
            moved = numpy.tile(extent, (steps.size, 1))
            moved[:, axis] += side * steps
            moved[:, axis + 2] += side * steps
            moved[:, axis] -= SEAM
            moved[:, axis + 2] += SEAM
            out = ~sharing(moved, only[None, :])
            ways.append(float(steps[int(numpy.argmax(out))]) if out.any() else math.inf)
        if abs(abs(move[0]) + abs(move[1]) - min(ways)) > AGREEMENT:
            raise AssertionError("pushes/%s: the push is %r, the shortest way out along an axis %r" % (vector["id"], move.tolist(), min(ways)))
    return [float(move[0]), float(move[1])]


def resort(vector):
    """💨️ The last resort: whether a pet has to poof (it may not stay and none of its plans is clear), and who is evicted after a change from outside."""
    bodies, claims, tick = vector["bodies"], vector["claims"], vector["tick"]
    if vector["kind"] == "poof":
        return not free(vector["stay"], vector["owner"], bodies, claims, tick) and not any(clear(plan, bodies, claims) for plan in vector["plans"])
    gone = []
    for index, entry in enumerate(bodies):
        probe = rows([entry["box"]])
        blocked = bool(sharing(probe, rows([other["box"] for other in bodies[:index] if other["owner"] not in gone])).any())
        if not blocked and all(other["owner"] != entry["owner"] for other in claims):
            for other in claims:
                if other["owner"] in gone:
                    continue
                left, _count, rest = ahead(other, tick)
                blocked = blocked or bool(sharing(probe, left).any()) or bool(sharing(probe, rows([rest])).any())
        if blocked:
            gone.append(entry["owner"])
    return gone


# endregion 🔖️Reference


# region 🔖️Handlers
def committed(ctx):
    """🧫️ The committed vectors."""
    return json.loads(ctx.input_bytes(VECTORS))


def close(produced, expected):
    """🔍️ Deep equality with floats compared within 1e-12."""
    if isinstance(produced, bool) or isinstance(expected, bool):
        return produced == expected
    if isinstance(produced, (int, float)) and isinstance(expected, (int, float)):
        return math.isclose(produced, expected, rel_tol=AGREEMENT, abs_tol=AGREEMENT)
    if isinstance(produced, list) and isinstance(expected, list):
        return len(produced) == len(expected) and all(close(left, right) for left, right in zip(produced, expected))
    if isinstance(produced, dict) and isinstance(expected, dict):
        return produced.keys() == expected.keys() and all(close(produced[key], expected[key]) for key in produced)
    return produced == expected


def agree(scenario, produced, vectors):
    """⚖️ Holds every produced answer to the committed one — the vectors may never drift from the reference."""
    for vector in vectors:
        if not close(produced[vector["id"]], vector["expected"]):
            raise AssertionError("%s/%s: the reference answers %r, the committed vector says %r" % (scenario, vector["id"], produced[vector["id"]], vector["expected"]))
    return Outcome(produced)


REFERENCES = {
    "bodies": body,
    "crowds": crowd,
    "spots": spot,
    "strides": lambda vector: walked(vector["box"], vector["stride"], vector["obstacles"]),
    "orders": order,
    "claims": claim,
    "seatings": seating,
    "heads": head,
    "pushes": push,
    "resorts": resort,
}


def answered(name):
    """🗂️ The handler of one scenario: every committed vector of its list answered by its reference."""

    def handler(ctx):
        vectors = committed(ctx)[name]
        return agree(name, {vector["id"]: REFERENCES[name](vector) for vector in vectors}, vectors)

    return handler


def constants(ctx):
    """🎚️ The tuning constants the vectors were generated with; they must be the ones of this reference."""
    document = committed(ctx)
    if document["constants"] != CONSTANTS:
        raise AssertionError("constants: the reference is tuned to %r, the committed vectors to %r" % (CONSTANTS, document["constants"]))
    return Outcome(document["constants"])


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter():
    """🧭️ Oracle role only: numpy and scipy are the reference, the TypeScript and Rust twins are judged against it."""
    registered = Adapter("python").oracle("constants", constants)
    for name in REFERENCES:
        registered = registered.oracle(name, answered(name))
    return registered


# endregion 🔖️Registration

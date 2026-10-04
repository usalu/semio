#!/usr/bin/env python3
"""🧗️ Oracle of the walls of the pets product (design-v2 §16, MECH §5): the free stretches of the surveyed side edges, the line of sight, the holds, the climb, the grip, the slide and the mantle, in Python.

Nothing here subtracts an interval or clips a segment. A layout is rasterised: every committed
coordinate lies on a quarter-pixel lattice, the wall is cut into lattice cells, and numpy boolean
masks decide by membership of the cell centres which cells of a wall lie inside the viewport and
beside no keep-out that reaches into the band on its air side, from the lip to the clearance. The
pitches are the runs of free cells (``numpy.diff``). The pitch at a height is read from a raster of
lattice points, the nearest pitch is found by brute force over the sampled points of every pitch
(``numpy.argmin``). A line of sight is judged by the separating axes of a segment and a box — the
extents on both axes and the signs of the ``numpy.cross`` products of the box's corners with the
segment — and laid over 4097 sampled points of the segment, which must agree, so a vector whose
verdict hangs on less than a sample is refused.

What moves is judged by its closed form: the height of a climb after ``k`` ticks is the
``numpy.cumsum`` of the eased speeds, held at the goal; the grip after a run of ticks is a clipped
line; a slide is the sum of its clamped speeds, held at its floor; the mantle is three cubic Hermite
splines with flat ends between the keys of its definition (``scipy.interpolate.CubicHermiteSpline``).
Every number the oracle answers is the plain binary64 value of the stated tick, written here from the
design text, and refused unless the libraries reach it within 1e-9: the harness compares on a decimal
grid of 1e-9, on which a number that is right to 1e-13 can round to the other side.

The holds and the routes are decided by a second reading of the design written with numpy
comparisons: a supplement that holds the twins to one reading, on top of the geometry above.

@see https://numpy.org/doc/stable/reference/generated/numpy.cross.html
@see https://docs.scipy.org/doc/scipy/reference/generated/scipy.interpolate.CubicHermiteSpline.html
@see ../../🧫️fixtures/🧗️wall-climbing/🔣️.json
"""

# region 🔖️Imports
import json
import math

import numpy
import scipy.interpolate

from semio_repo_test import Adapter, Outcome

# endregion 🔖️Imports


# region 🔖️Constants
VECTORS = "shared://🧗️wall-climbing/🔣️.json"
TICKS_PER_SECOND = 64
WALL_LIP = 6.0
HAND_HEIGHT = 0.85
CLIMB_RISE = 30.0
CLIMB_DESCENT = 40.0
CLIMB_RAMP = 6
GRIP_SPACING = 0.3
GRIP_BUDGET = 384
GRIP_CLIMB = 1.0
GRIP_HANG = 0.25
GRIP_REST = 2.0
GRIP_BITE = 8.0
WALL_FOLLOW = 6.0
SLIDE_START = 30.0
SLIDE_GAIN = 600.0
SLIDE_SPEED = 160.0
SLIP_PUSH = 140.0
SLIP_LIFT = 160.0
WALL_GRAB_TICKS = 8
WALL_HANG_TICKS = 10
MANTLE_TICKS = 28
MANTLE_INSET = 8.0
HOIST_HUMP = 0.1
HOIST_RISE = 0.65
CROSS_REACH = 1.5
LUNGE_TICKS = 16
LONGEST = 4096
CONSTANTS = {
    "wallLip": WALL_LIP,
    "handHeight": HAND_HEIGHT,
    "climbRise": CLIMB_RISE,
    "climbDescent": CLIMB_DESCENT,
    "climbRamp": CLIMB_RAMP,
    "gripSpacing": GRIP_SPACING,
    "gripBudget": GRIP_BUDGET,
    "gripClimb": GRIP_CLIMB,
    "gripHang": GRIP_HANG,
    "gripRest": GRIP_REST,
    "gripBite": GRIP_BITE,
    "wallFollow": WALL_FOLLOW,
    "slideStart": SLIDE_START,
    "slideGain": SLIDE_GAIN,
    "slideSpeed": SLIDE_SPEED,
    "slipPush": SLIP_PUSH,
    "slipLift": SLIP_LIFT,
    "wallGrabTicks": WALL_GRAB_TICKS,
    "wallHangTicks": WALL_HANG_TICKS,
    "mantleTicks": MANTLE_TICKS,
    "mantleInset": MANTLE_INSET,
    "hoistHump": HOIST_HUMP,
    "hoistRise": HOIST_RISE,
    "crossReach": CROSS_REACH,
    "lungeTicks": LUNGE_TICKS,
}
AGREEMENT = 1e-12
STEPPING = 1e-9
MARGIN = 1e-6
SAMPLES = 4097
# endregion 🔖️Constants


# region 🔖️Terrain
def cells(lattice, low, high):
    """🔢️ How many lattice cells lie between two lattice coordinates; a coordinate off the lattice is refused."""
    count = (high - low) / lattice
    if count != round(count):
        raise AssertionError("%r … %r is not a whole number of lattice cells of %r" % (low, high, lattice))
    return int(round(count))


def aligned(lattice, values):
    """📐️ Refuses a layout with a coordinate off the lattice: the raster could not tell its cells apart."""
    for value in values:
        cells(lattice, 0.0, value)


def pitches(lattice, layout):
    """🗻️ The pitches of a layout as the runs of free cells on every wall whose band lies inside the viewport."""
    width, height, clearance, minimum = layout["width"], layout["height"], layout["clearance"], layout["minimum"]
    levels = [0.0, float(height)] + [level for wall in layout["walls"] for level in (wall["y0"], wall["y1"])] + [level for keepout in layout["keepouts"] for level in (keepout["y"], keepout["y"] + keepout["height"])]
    sides = [float(width), float(clearance), WALL_LIP] + [wall["x"] for wall in layout["walls"]] + [side for keepout in layout["keepouts"] for side in (keepout["x"], keepout["x"] + keepout["width"])]
    aligned(lattice, levels + sides)
    origin = min(levels) - lattice
    rows = origin + (numpy.arange(cells(lattice, origin, max(levels) + lattice)) + 0.5) * lattice
    found = []
    for wall in layout["walls"]:
        low, high = (wall["x"] - clearance, wall["x"] - WALL_LIP) if wall["side"] < 0 else (wall["x"] + WALL_LIP, wall["x"] + clearance)
        outer = wall["x"] + wall["side"] * clearance
        if min(wall["x"], outer) < 0 or max(wall["x"], outer) > width:
            continue
        columns = low + (numpy.arange(cells(lattice, low, high)) + 0.5) * lattice if high > low else numpy.zeros(0)
        covered = numpy.zeros(rows.size, dtype=bool)
        for keepout in layout["keepouts"]:
            beside = (columns > keepout["x"]) & (columns < keepout["x"] + keepout["width"])
            if beside.any():
                covered |= (rows > keepout["y"]) & (rows < keepout["y"] + keepout["height"])
        free = (rows > wall["y0"]) & (rows < wall["y1"]) & (rows > 0) & (rows < height) & ~covered
        edges = numpy.flatnonzero(numpy.diff(numpy.concatenate(([False], free, [False])).astype(numpy.int8)))
        for start, end in zip(edges[0::2], edges[1::2]):
            y0, y1 = float(origin + start * lattice), float(origin + end * lattice)
            if y1 - y0 >= minimum:
                found.append({"wall": wall["id"], "surface": wall["surface"], "side": wall["side"], "x": wall["x"], "y0": y0, "y1": y1})
    return found


def stretches(lattice, stand):
    """📌️ For every query the index of the pitch of its wall that owns the lattice point ``y``, the first one where two touch; ``None`` off the wall."""
    levels = [level for pitch in stand["pitches"] for level in (pitch["y0"], pitch["y1"])] + [query["y"] for query in stand["queries"]]
    origin = min(levels)
    answers = []
    for query in stand["queries"]:
        owner = numpy.full(cells(lattice, origin, max(levels)) + 1, -1, dtype=int)
        for index in range(len(stand["pitches"]) - 1, -1, -1):
            pitch = stand["pitches"][index]
            if pitch["wall"] == query["wall"]:
                owner[cells(lattice, origin, pitch["y0"]) : cells(lattice, origin, pitch["y1"]) + 1] = index
        owned = int(owner[cells(lattice, origin, query["y"])])
        answers.append(None if owned < 0 else owned)
    return answers


def nearest(lattice, near):
    """🧲️ For every point the index of the pitch with the closest sampled lattice point, the first one among equals; ``None`` without pitches."""
    answers = []
    aligned(lattice, [side for point in near["points"] for side in (point["x"], point["y"])])
    for point in near["points"]:
        least = []
        for pitch in near["pitches"]:
            samples = pitch["y0"] + numpy.arange(cells(lattice, pitch["y0"], pitch["y1"]) + 1) * lattice
            least.append(float(numpy.min((pitch["x"] - point["x"]) ** 2 + (samples - point["y"]) ** 2)))
        answers.append(int(numpy.argmin(numpy.array(least))) if least else None)
    return answers


def crossed(start, end, rect, margin):
    """⚔️ Whether a segment meets the inside of a box grown by a margin, by their separating axes: the extents overlap on both axes of the box and the corners of the box lie on both sides of the segment's line (``numpy.cross``); a segment of no length is a point inside."""
    left, right, top, bottom = rect["x"] - margin, rect["x"] + rect["width"] + margin, rect["y"] - margin, rect["y"] + rect["height"] + margin
    if not (rect["width"] > 0 and rect["height"] > 0 and left < right and top < bottom):
        return False
    ends = numpy.array([[start["x"], start["y"], 0.0], [end["x"], end["y"], 0.0]], dtype=float)
    if not (ends[:, 0].min() < right and ends[:, 0].max() > left and ends[:, 1].min() < bottom and ends[:, 1].max() > top):
        return False
    if (ends[0] == ends[1]).all():
        return True
    corners = numpy.array([[left, top, 0.0], [right, top, 0.0], [right, bottom, 0.0], [left, bottom, 0.0]])
    turns = numpy.cross(ends[1] - ends[0], corners - ends[0])[:, 2]
    return bool(turns.min() < 0 < turns.max())


def threaded(start, end, rect, margin):
    """🧵️ Whether one of the sampled points of a segment lies inside a box grown by a margin by more than 1e-9."""
    if not (rect["width"] > 0 and rect["height"] > 0):
        return False
    shares = numpy.linspace(0.0, 1.0, SAMPLES)
    xs = start["x"] + (end["x"] - start["x"]) * shares
    ys = start["y"] + (end["y"] - start["y"]) * shares
    return bool(((xs - (rect["x"] - margin) > STEPPING) & (rect["x"] + rect["width"] + margin - xs > STEPPING) & (ys - (rect["y"] - margin) > STEPPING) & (rect["y"] + rect["height"] + margin - ys > STEPPING)).any())


def hit(label, start, end, rect, margin):
    """🪡️ Whether a segment hits a box: the verdict of the separating axes, refused unless the sampled segment agrees."""
    verdict = crossed(start, end, rect, margin)
    if verdict != threaded(start, end, rect, margin):
        raise AssertionError("%s: the separating axes say %r, the sampled segment the opposite — the verdict hangs on less than a sample" % (label, verdict))
    return verdict


def sight(vector):
    """🔭️ For every committed segment whether it hits each box of the vector grown by the margin, and whether it is clear of all of them."""
    answers = []
    for index, segment in enumerate(vector["segments"]):
        hits = [hit("sights/%s[%d]" % (vector["id"], index), segment["from"], segment["to"], rect, vector["margin"]) for rect in vector["rects"]]
        answers.append({"hits": hits, "clear": not any(hits)})
    return answers


# endregion 🔖️Terrain


# region 🔖️Holds
def cling_of(pitch, size):
    """🦎️ The x of the feet of an actor that clings to a pitch."""
    return pitch["x"] + (pitch["side"] * size["width"]) / 2


def ledge_of(pitch, size):
    """🏝️ The x of the spot on top of the wall where a mantle ends."""
    return pitch["x"] - pitch["side"] * (size["width"] / 2 + MANTLE_INSET)


def rim_of(pitch, size):
    """🧢️ The height of the feet at which the hands reach the top of a pitch."""
    return pitch["y0"] + HAND_HEIGHT * size["height"]


def foot_of(pitch, size):
    """🥾️ The height of the feet at which the hands hold the bite above the lower end of a pitch."""
    return pitch["y1"] - GRIP_BITE + HAND_HEIGHT * size["height"]


def clings(pitch, y, size):
    """🦀️ Whether an actor with its feet at a height has its hands on a pitch."""
    return bool(numpy.logical_and(rim_of(pitch, size) <= y, y <= foot_of(pitch, size)))


def crowns(perch, pitch, size):
    """👑️ Whether a perch lies on top of a pitch and carries the ledge."""
    ledge = ledge_of(pitch, size)
    return perch["surface"] == pitch["surface"] and perch["y"] == pitch["y0"] and bool(numpy.logical_and(perch["x0"] <= ledge, ledge <= perch["x1"]))


def grip_for(perch, pitch, size):
    """🫴️ The hold an actor on a perch takes on a pitch: over the rim from the perch that crowns it, else from where it can stand beside the wall with its hands on it; ``None`` when it cannot."""
    x, rim = cling_of(pitch, size), rim_of(pitch, size)
    if crowns(perch, pitch, size):
        return {"x": x, "y": rim, "over": True} if clings(pitch, rim, size) else None
    return {"x": x, "y": perch["y"], "over": False} if perch["x0"] <= x <= perch["x1"] and clings(pitch, perch["y"], size) else None


def holds(stage):
    """🧤️ The hold of every perch on every pitch of a stage, the perch that crowns every pitch, and the measures of every pitch."""
    size = stage["size"]
    return {
        "grips": [[grip_for(perch, pitch, size) for pitch in stage["pitches"]] for perch in stage["perches"]],
        "rims": [next((index for index, perch in enumerate(stage["perches"]) if crowns(perch, pitch, size)), None) for pitch in stage["pitches"]],
        "measures": [{"cling": cling_of(pitch, size), "ledge": ledge_of(pitch, size), "rim": rim_of(pitch, size), "foot": foot_of(pitch, size)} for pitch in stage["pitches"]],
    }


def survey(vector):
    """🧐️ The index of the pitch that still carries a climber after a survey, and the throw that takes it off the wall when none does."""
    pitch, size = vector["pitch"], vector["size"]
    for index, after in enumerate(vector["pitches"]):
        if after["wall"] == pitch["wall"] and after["side"] == pitch["side"] and abs(after["x"] - pitch["x"]) <= WALL_FOLLOW and clings(after, vector["y"], size):
            return {"pitch": index, "slip": None}
    return {"pitch": None, "slip": {"vx": pitch["side"] * SLIP_PUSH, "vy": 0 - SLIP_LIFT}}


# endregion 🔖️Holds


# region 🔖️Motion
def confirmed(label, stated, judged):
    """🤝️ Refuses stated numbers the libraries do not reach within 1e-9."""
    stated, judged = numpy.asarray(stated, dtype=float), numpy.asarray(judged, dtype=float)
    if stated.size == 0 and judged.size == 0:
        return
    if stated.shape != judged.shape or float(numpy.abs(stated - judged).max()) > STEPPING:
        raise AssertionError("%s: the stated ticks give %r, the libraries %r" % (label, stated.tolist(), judged.tolist()))


def smoothstep(amount):
    """🛝️ The Hermite ease ``t²·(3 − 2t)`` of an amount held in [0, 1]."""
    held = 0 if amount < 0 else amount
    held = 1 if held > 1 else held
    return held * held * (3 - 2 * held)


def eased(amounts):
    """🎢️ The same ease over an array, by numpy."""
    held = numpy.clip(numpy.asarray(amounts, dtype=float), 0.0, 1.0)
    return held * held * (3.0 - 2.0 * held)


def stride_to(x, goal, speed):
    """👣️ One stated tick of walking: a step of ``max(speed, 0) ÷ 64`` towards the goal, the goal itself within one step."""
    step = max(speed, 0) / TICKS_PER_SECOND
    gap = goal - x
    return x + step if gap > step else x - step if gap < 0 - step else goal


def climb_step(y, goal, ticks):
    """🐜️ One stated tick of climbing: up at the rise, down at the descent, gathered over the ramp."""
    return stride_to(y, goal, (CLIMB_RISE if goal < y else CLIMB_DESCENT) * smoothstep((ticks + 1) / CLIMB_RAMP))


def climb_ticks(y, goal):
    """⏱️ How many stated ticks a climb takes, one more than the budget when no rested grip lasts for it."""
    height, ticks = y, 0
    while height != goal and ticks <= GRIP_BUDGET:
        height = climb_step(height, goal, ticks)
        ticks += 1
    return ticks


def climb(vector):
    """🧮️ The ticks of a climb and, when a rested grip lasts for it, the height and the phase of the clip after every stated tick; refused unless ``numpy.cumsum`` of the eased speeds, held at the goal, reaches the same heights and the same tick of arrival."""
    start, goal, size, pitch = vector["from"], vector["goal"], vector["size"], vector["pitch"]
    ticks = climb_ticks(start, goal)
    heights, height = [], start
    for tick in range(ticks if ticks <= GRIP_BUDGET else 0):
        height = climb_step(height, goal, tick)
        heights.append(height)
    speed = CLIMB_RISE if goal < start else CLIMB_DESCENT
    covered = numpy.cumsum(speed * eased(numpy.arange(1, GRIP_BUDGET + 2) / CLIMB_RAMP) / TICKS_PER_SECOND)
    distance = abs(goal - start)
    if distance > 0 and float(numpy.abs(covered - distance).min()) < MARGIN:
        raise AssertionError("climbs/%s: the climb arrives within %r of a tick, on the edge of two tick counts" % (vector["id"], MARGIN))
    confirmed("climbs/%s: the heights" % vector["id"], heights, start + numpy.sign(goal - start) * numpy.minimum(covered[: len(heights)], distance))
    arrival = 0 if distance == 0 else int(numpy.argmax(covered >= distance)) + 1 if covered[-1] >= distance else GRIP_BUDGET + 1
    if arrival != ticks:
        raise AssertionError("climbs/%s: %d stated ticks, %d by the sum of the speeds" % (vector["id"], ticks, arrival))
    stride = 2 * GRIP_SPACING * size["height"]
    phases = [(pitch["y0"] - height) / stride - math.floor((pitch["y0"] - height) / stride) for height in heights]
    turned = numpy.mod((pitch["y0"] - numpy.asarray(heights, dtype=float)) / stride, 1.0)
    if len(phases) and float(numpy.abs((numpy.asarray(phases) - turned + 0.5) % 1.0 - 0.5).max()) > STEPPING:
        raise AssertionError("climbs/%s: the phases leave numpy.mod of the distance from the top" % vector["id"])
    return {"ticks": ticks, "heights": heights, "phases": phases}


def grip_step(grip, effort):
    """🔌️ One stated tick of grip: climbing and hanging spend it down to nothing, resting gives it back up to the budget."""
    return min(grip + GRIP_REST, GRIP_BUDGET) if effort == "rest" else max(grip - (GRIP_CLIMB if effort == "climb" else GRIP_HANG), 0)


def grip(vector):
    """🔋️ The grip after every run of ticks of one effort; refused unless the clipped line of the run reaches it."""
    answers, held = [], vector["grip"]
    for effort, ticks in vector["runs"]:
        judged = float(numpy.minimum(held + GRIP_REST * ticks, GRIP_BUDGET)) if effort == "rest" else float(numpy.maximum(held - (GRIP_CLIMB if effort == "climb" else GRIP_HANG) * ticks, 0.0))
        for _ in range(ticks):
            held = grip_step(held, effort)
        confirmed("grips/%s: %d ticks of %s" % (vector["id"], ticks, effort), [held], [judged])
        answers.append(held)
    return answers


def slide_step(y, vy, floor):
    """🧈️ One stated tick of sliding: the velocity first, from the slip speed up to the fastest slide, then the height, held at the floor."""
    speed = min(max(vy, SLIDE_START) + SLIDE_GAIN / TICKS_PER_SECOND, SLIDE_SPEED)
    return min(y + speed / TICKS_PER_SECOND, floor), speed


def slide(vector):
    """🛷️ The heights and speeds after every stated tick of a slide; refused unless the clamped line of the speeds and ``numpy.cumsum`` of them, held at the floor, reach the same."""
    heights, speeds, y, vy = [], [], vector["y"], vector["vy"]
    for _ in range(vector["ticks"]):
        y, vy = slide_step(y, vy, vector["floor"])
        heights.append(y)
        speeds.append(vy)
    judged = numpy.minimum(max(vector["vy"], SLIDE_START) + numpy.arange(1, vector["ticks"] + 1) * (SLIDE_GAIN / TICKS_PER_SECOND), SLIDE_SPEED)
    confirmed("slides/%s: the speeds" % vector["id"], speeds, judged)
    confirmed("slides/%s: the heights" % vector["id"], heights, numpy.minimum(vector["y"] + numpy.cumsum(judged) / TICKS_PER_SECOND, vector["floor"]))
    return {"heights": heights, "speeds": speeds}


def hoist_path(start, end, height, phase):
    """🦘️ The stated feet of a hoist at a phase: the rise over the first 0.65 of the way, the way across over the last 0.65, the settling over the rest."""
    if phase >= 1:
        return [end["x"], end["y"]]
    hump = HOIST_HUMP * height
    rise = smoothstep(phase / HOIST_RISE)
    across = smoothstep((phase - (1 - HOIST_RISE)) / HOIST_RISE)
    settle = smoothstep((phase - HOIST_RISE) / (1 - HOIST_RISE))
    return [start["x"] + (end["x"] - start["x"]) * across, start["y"] + (end["y"] - hump - start["y"]) * rise + hump * settle]


def hoist(label, start, end, height, ticks):
    """🏋️ The feet of a hoist after every tick of its ticks, the start itself first and the end itself last; refused unless three cubic Hermite splines with flat ends between the keys of its definition reach the same path."""
    path = [hoist_path(start, end, height, tick / ticks) for tick in range(ticks + 1)]
    phases = numpy.arange(ticks + 1) / ticks
    flat = numpy.zeros(3)
    rise = scipy.interpolate.CubicHermiteSpline([0.0, HOIST_RISE, 1.0], [0.0, 1.0, 1.0], flat)(phases)
    across = scipy.interpolate.CubicHermiteSpline([0.0, 1 - HOIST_RISE, 1.0], [0.0, 0.0, 1.0], flat)(phases)
    settle = scipy.interpolate.CubicHermiteSpline([0.0, HOIST_RISE, 1.0], [0.0, 0.0, 1.0], flat)(phases)
    hump = HOIST_HUMP * height
    confirmed("%s: the path" % label, path, numpy.stack([start["x"] + (end["x"] - start["x"]) * across, start["y"] + (end["y"] - hump - start["y"]) * rise + hump * settle], axis=1))
    if path[0] != [start["x"], start["y"]] or path[-1] != [end["x"], end["y"]]:
        raise AssertionError("%s: the hoist does not begin on its start and end on its end" % label)
    return path


def mantle(vector):
    """🧘️ The mantle from a pitch over its rim onto the ledge, tick by tick."""
    pitch, size = vector["pitch"], vector["size"]
    return hoist("mantles/%s" % vector["id"], {"x": cling_of(pitch, size), "y": rim_of(pitch, size)}, {"x": ledge_of(pitch, size), "y": pitch["y0"]}, size["height"], MANTLE_TICKS)


def crossings(pitches, size):
    """🌉️ Which pitch lunges to which, as a boolean matrix (row: from, column: to) by numpy broadcasting: one wall line (the same side, no more than the follow apart), the target wholly above or wholly below, both pitches holding the actor somewhere, and the gap from the top of the lower one to the lowest hold of the upper one within the reach."""
    if not pitches:
        return numpy.zeros((0, 0), dtype=bool)
    side = numpy.array([pitch["side"] for pitch in pitches], dtype=float)
    x = numpy.array([pitch["x"] for pitch in pitches], dtype=float)
    top = numpy.array([pitch["y0"] for pitch in pitches], dtype=float)
    bottom = numpy.array([pitch["y1"] for pitch in pitches], dtype=float)
    hand = HAND_HEIGHT * size["height"]
    holding = top + hand <= bottom - GRIP_BITE + hand
    line = (side[:, None] == side[None, :]) & (numpy.abs(x[None, :] - x[:, None]) <= WALL_FOLLOW)
    reach = CROSS_REACH * size["height"]
    upward = (bottom[None, :] <= top[:, None]) & (top[:, None] - bottom[None, :] + GRIP_BITE <= reach)
    downward = (bottom[:, None] <= top[None, :]) & (top[None, :] - bottom[:, None] + GRIP_BITE <= reach)
    return line & holding[:, None] & holding[None, :] & (upward | downward)


def chain_of(index, pitches, size):
    """⛓️ The wall line of a pitch as indices from the top down: from the pitch upwards the reachable one with the lowest end, downwards the reachable one with the highest top, the first among equals (``numpy.argmax`` over the masked ends), never one equal to a pitch the line holds already."""
    matrix = crossings(pitches, size)
    top = numpy.array([pitch["y0"] for pitch in pitches], dtype=float)
    bottom = numpy.array([pitch["y1"] for pitch in pitches], dtype=float)
    chain = [index]
    fresh = lambda: numpy.array([all(pitch != pitches[held] for held in chain) for pitch in pitches], dtype=bool)
    while True:
        above = matrix[chain[0]] & (bottom <= top[chain[0]]) & fresh()
        if not above.any():
            break
        chain.insert(0, int(numpy.argmax(numpy.where(above, bottom, -numpy.inf))))
    while True:
        below = matrix[chain[-1]] & (top >= bottom[chain[-1]]) & fresh()
        if not below.any():
            break
        chain.append(int(numpy.argmax(numpy.where(below, -top, -numpy.inf))))
    return chain


def lunge(start, end, size):
    """🐇️ The ends of a lunge from one pitch to another: from its hold nearest to the other (the rim upwards, the foot downwards) to the hold of the other nearest to it."""
    upward = end["y1"] <= start["y0"]
    return {"x": cling_of(start, size), "y": rim_of(start, size) if upward else foot_of(start, size)}, {"x": cling_of(end, size), "y": foot_of(end, size) if upward else rim_of(end, size)}


def wall_path(chain, start, entry, x, y, end, goal, mantling, size):
    """🪨️ The stated way along a wall line, tick by tick, as ``[x, y, hold, work]``: taking hold (a grab beside the wall, eased onto the hold; or the mantle path backwards over the rim), the climbs from pitch to pitch with a lunge across every gap, and the mantle. The pieces are judged on their own by the caller."""
    path = []
    first = chain[start]
    hold = cling_of(first, size)
    height = y
    if entry == "grab":
        for tick in range(1, WALL_GRAB_TICKS + 1):
            path.append([hold if tick == WALL_GRAB_TICKS else x + (hold - x) * smoothstep(tick / WALL_GRAB_TICKS), y, start, "grab"])
    elif entry == "hang":
        for tick in range(1, WALL_HANG_TICKS + 1):
            path.append(hoist_path({"x": hold, "y": rim_of(first, size)}, {"x": ledge_of(first, size), "y": first["y0"]}, size["height"], 1 - tick / WALL_HANG_TICKS) + [start, "hang"])
        height = rim_of(first, size)
    at = start
    while True:
        pitch = chain[at]
        cling = cling_of(pitch, size)
        aim = goal if at == end else rim_of(pitch, size) if at > end else foot_of(pitch, size)
        tick = 0
        while height != aim and tick < LONGEST:
            height = climb_step(height, aim, tick)
            path.append([cling, height, at, "climb"])
            tick += 1
        if at == end:
            break
        following = at - 1 if at > end else at + 1
        origin, target = lunge(pitch, chain[following], size)
        for tick in range(1, LUNGE_TICKS + 1):
            path.append(hoist_path(origin, target, size["height"], tick / LUNGE_TICKS) + [following, "lunge"])
        height = target["y"]
        at = following
    if mantling:
        for tick in range(1, MANTLE_TICKS + 1):
            path.append(hoist_path({"x": cling_of(chain[end], size), "y": rim_of(chain[end], size)}, {"x": ledge_of(chain[end], size), "y": chain[end]["y0"]}, size["height"], tick / MANTLE_TICKS) + [end, "mantle"])
    return path


def wall_cost(path):
    """🪫️ The grip a way along a wall costs: one climbing tick for every tick of it."""
    return len(path) * GRIP_CLIMB


def judged_way(label, chain, path, size, x, y):
    """🔬️ Holds every piece of a stated way that begins with the feet at (``x``, ``y``) to the libraries: the climbs to ``numpy.cumsum`` of their eased speeds held at their ends, the grab to the numpy ease, the hang, the lunges and the mantle to the cubic Hermite splines of ``hoist``."""
    index = 0
    while index < len(path):
        work = path[index][3]
        run = index
        while run < len(path) and path[run][3] == work and path[run][2] == path[index][2]:
            run += 1
        piece = path[index:run]
        pitch = chain[piece[0][2]]
        if work == "climb":
            start = path[index - 1][1] if index > 0 else y
            end = piece[-1][1]
            speed = CLIMB_RISE if end < start else CLIMB_DESCENT
            covered = numpy.cumsum(speed * eased(numpy.arange(1, len(piece) + 1) / CLIMB_RAMP) / TICKS_PER_SECOND)
            confirmed("%s: the climb at step %d" % (label, index), [step[1] for step in piece], start + numpy.sign(end - start) * numpy.minimum(covered, abs(end - start)))
        elif work == "grab":
            confirmed("%s: the grab" % label, [step[0] for step in piece], x + (cling_of(pitch, size) - x) * eased(numpy.arange(1, len(piece) + 1) / WALL_GRAB_TICKS))
        else:
            if work == "hang":
                origin, target = {"x": ledge_of(pitch, size), "y": pitch["y0"]}, {"x": cling_of(pitch, size), "y": rim_of(pitch, size)}
                judged = hoist("%s: the hang" % label, target, origin, size["height"], len(piece))[::-1][1:]
            elif work == "lunge":
                previous = chain[path[index - 1][2]]
                origin, target = lunge(previous, pitch, size)
                judged = hoist("%s: the lunge at step %d" % (label, index), origin, target, size["height"], len(piece))[1:]
            else:
                judged = hoist("%s: the mantle" % label, {"x": cling_of(pitch, size), "y": rim_of(pitch, size)}, {"x": ledge_of(pitch, size), "y": pitch["y0"]}, size["height"], len(piece))[1:]
            confirmed("%s: the %s at step %d" % (label, work, index), [step[:2] for step in piece], judged)
        index = run
    return path


def crossing(vector):
    """🌁️ The lunges, the wall lines and the ways of a committed stage: the matrix of which pitch lunges to which, the line of every pitch, and every committed way tick by tick with its grip."""
    pitches, size = vector["pitches"], vector["size"]
    ways = []
    for wish in vector["ways"]:
        line = chain_of(wish["pitch"], pitches, size)
        chain = [pitches[index] for index in line]
        path = judged_way("crossings/%s" % vector["id"], chain, wall_path(chain, line.index(wish["pitch"]), wish["entry"], wish["x"], wish["y"], wish["end"], wish["goal"], wish["mantle"], size), size, wish["x"], wish["y"])
        ways.append({"path": path, "cost": wall_cost(path)})
    return {"crossable": crossings(pitches, size).tolist(), "chains": [chain_of(index, pitches, size) for index in range(len(pitches))], "ways": ways}


def scaled(index, pitches, start, end, size, held):
    """🧱️ The way over the wall line of a pitch between two perches: from a hold at its foot up the line to the rim of the nearest pitch the target crowns, or over the rim the start crowns down the line to the nearest pitch where the target passes its foot, when the grip lasts for the whole way; ``None`` otherwise."""
    pitch = pitches[index]
    hold = grip_for(start, pitch, size)
    if hold is None:
        return None
    line = chain_of(index, pitches, size)
    chain = [pitches[member] for member in line]
    first = line.index(index)
    if hold["over"]:
        for at in range(first, len(chain)):
            landing = grip_for(end, chain[at], size)
            if landing is None or landing["over"]:
                continue
            path = wall_path(chain, first, "hang", hold["x"], hold["y"], at, landing["y"], False, size)
            return {"at": ledge_of(pitch, size), "hold": hold, "goal": landing["y"], "exit": line[at]} if wall_cost(path) <= held else None
        return None
    for at in range(first, -1, -1):
        if not crowns(end, chain[at], size):
            continue
        rim = rim_of(chain[at], size)
        path = wall_path(chain, first, "grab", hold["x"], hold["y"], at, rim, True, size)
        return {"at": hold["x"], "hold": hold, "goal": rim, "exit": line[at]} if wall_cost(path) <= held else None
    return None


def route(vector):
    """🗺️ The wall lines an actor with the gear to climb can take between two perches of a stage, one leg per pitch it takes hold of, in pitch order; ``None`` when there is none."""
    perches = vector["perches"]
    legs = []
    for index in range(len(vector["pitches"])):
        leg = scaled(index, vector["pitches"], perches[vector["from"]], perches[vector["to"]], vector["size"], vector["grip"])
        if leg is not None:
            legs.append(dict(leg, pitch=index))
    return legs if legs else None


# endregion 🔖️Motion


# region 🔖️Handlers
def committed(ctx):
    """🧫️ The committed vectors."""
    return json.loads(ctx.fixture_bytes(VECTORS))


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


def constants(ctx):
    """🎚️ The tuning constants the vectors were generated with; they must be the ones of this reference."""
    document = committed(ctx)
    if document["constants"] != CONSTANTS:
        raise AssertionError("constants: the reference is tuned to %r, the committed vectors to %r" % (CONSTANTS, document["constants"]))
    return Outcome(document["constants"])


def wall_cutting(ctx):
    """✂️ The pitches of every committed layout."""
    document = committed(ctx)
    return agree("pitches", {layout["id"]: pitches(document["lattice"], layout) for layout in document["layouts"]}, document["layouts"])


def wall_standing(ctx):
    """📍️ The pitch at every committed height."""
    document = committed(ctx)
    return agree("stretches", {stand["id"]: stretches(document["lattice"], stand) for stand in document["stretches"]}, document["stretches"])


def wall_nearest(ctx):
    """📡️ The nearest pitch of every committed point."""
    document = committed(ctx)
    return agree("nearest", {near["id"]: nearest(document["lattice"], near) for near in document["nearests"]}, document["nearests"])


def sights(ctx):
    """👁️ Every committed line of sight."""
    vectors = committed(ctx)["sights"]
    return agree("sights", {vector["id"]: sight(vector) for vector in vectors}, vectors)


def wall_holds(ctx):
    """✊️ Every hold of every committed stage."""
    vectors = committed(ctx)["holds"]
    return agree("holds", {vector["id"]: holds(vector) for vector in vectors}, vectors)


def surveys(ctx):
    """🌋️ What every committed survey leaves of a climber's wall."""
    vectors = committed(ctx)["surveys"]
    return agree("surveys", {vector["id"]: survey(vector) for vector in vectors}, vectors)


def climbs(ctx):
    """🐌️ Every committed climb, tick by tick."""
    vectors = committed(ctx)["climbs"]
    return agree("climbs", {vector["id"]: climb(vector) for vector in vectors}, vectors)


def grips(ctx):
    """💪️ The grip after every committed run."""
    vectors = committed(ctx)["grips"]
    return agree("grips", {vector["id"]: grip(vector) for vector in vectors}, vectors)


def slides(ctx):
    """🚒️ Every committed slide, tick by tick."""
    vectors = committed(ctx)["slides"]
    return agree("slides", {vector["id"]: slide(vector) for vector in vectors}, vectors)


def mantles(ctx):
    """🤸️ Every committed mantle and hoist, tick by tick."""
    document = committed(ctx)
    produced = {vector["id"]: mantle(vector) for vector in document["mantles"]}
    produced.update({vector["id"]: hoist("hoists/%s" % vector["id"], vector["from"], vector["to"], vector["height"], vector["ticks"]) for vector in document["hoists"]})
    return agree("mantles", produced, document["mantles"] + document["hoists"])


def routes(ctx):
    """🧭️ The walls between the perches of every committed stage."""
    vectors = committed(ctx)["routes"]
    return agree("routes", {vector["id"]: route(vector) for vector in vectors}, vectors)


def wall_lines(ctx):
    """🪢️ The lunges, wall lines and ways of every committed stage of pitches."""
    vectors = committed(ctx)["crossings"]
    return agree("crossings", {vector["id"]: crossing(vector) for vector in vectors}, vectors)


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter():
    """🔮️ Oracle role only: numpy and scipy are the reference, the TypeScript and Rust twins are judged against it."""
    return Adapter("python").oracle("constants", constants).oracle("pitches", wall_cutting).oracle("stretches", wall_standing).oracle("nearest", wall_nearest).oracle("sights", sights).oracle("holds", wall_holds).oracle("surveys", surveys).oracle("climbs", climbs).oracle("grips", grips).oracle("slides", slides).oracle("mantles", mantles).oracle("routes", routes).oracle("crossings", wall_lines)


# endregion 🔖️Registration

#!/usr/bin/env python3
"""🏞️ Oracle of the pets product's perches and strides (design §4.5), in Python.

Nothing here subtracts an interval. A layout is rasterised instead: every committed coordinate lies on a
lattice, the viewport is cut into lattice cells, and numpy boolean masks decide by membership of the
cell centres which cells of a surface are inside the viewport and which lie under a keep-out that
reaches into the headroom band ``[y − clearance, y)`` above it. The perches are the runs of free cells
(``numpy.diff`` of the mask), so touching, nested, overlapping and empty keep-outs need no case of their
own. Standing is answered from a raster of lattice points owned by the perches of a surface, the
nearest perch by brute force over the sampled points of every perch (``numpy.argmin`` of the squared
distances), and a walk is judged by its closed form: after ``k`` ticks the walker has covered
``min(k × speed ÷ 64, distance)``. The walk is answered as the plain binary64 values of its stated ticks,
refused unless the closed form reaches them within 1e-9 — the harness compares on a decimal grid of
1e-9, on which a number that is right to 1e-13 can round to the other side.

@see https://numpy.org/doc/stable/reference/generated/numpy.diff.html
@see https://numpy.org/doc/stable/reference/generated/numpy.argmin.html
@see ../../🧫️fixtures/🏞️terrain-walking/🔣️.json
"""

# region 🔖️Imports
import json
import math

import numpy

from semio_repo_test import Adapter, Outcome

# endregion 🔖️Imports


# region 🔖️Reference
VECTORS = "shared://🏞️terrain-walking/🔣️.json"
TICKS_PER_SECOND = 64
AGREEMENT = 1e-12
STEPPING = 1e-9


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


def perches(lattice, layout):
    """🪺️ The perches of a layout as the runs of free cells on every surface between the clearance line and the floor."""
    width, height, clearance, minimum = layout["width"], layout["height"], layout["clearance"], layout["minimum"]
    sides = [0.0, float(width)] + [side for surface in layout["surfaces"] for side in (surface["x0"], surface["x1"])] + [side for keepout in layout["keepouts"] for side in (keepout["x"], keepout["x"] + keepout["width"])]
    levels = [float(height), float(clearance)] + [surface["y"] for surface in layout["surfaces"]] + [level for keepout in layout["keepouts"] for level in (keepout["y"], keepout["y"] + keepout["height"])]
    aligned(lattice, sides + levels)
    origin = min(sides) - lattice
    columns = origin + (numpy.arange(cells(lattice, origin, max(sides) + lattice)) + 0.5) * lattice
    found = []
    for surface in layout["surfaces"]:
        if not clearance <= surface["y"] <= height:
            continue
        rows = surface["y"] - clearance + (numpy.arange(cells(lattice, 0.0, clearance)) + 0.5) * lattice
        covered = numpy.zeros((rows.size, columns.size), dtype=bool)
        for keepout in layout["keepouts"]:
            beside = (columns > keepout["x"]) & (columns < keepout["x"] + keepout["width"])
            above = (rows > keepout["y"]) & (rows < keepout["y"] + keepout["height"])
            covered |= above[:, None] & beside[None, :]
        free = (columns > surface["x0"]) & (columns < surface["x1"]) & (columns > 0) & (columns < width) & ~covered.any(axis=0)
        edges = numpy.flatnonzero(numpy.diff(numpy.concatenate(([False], free, [False])).astype(numpy.int8)))
        for start, end in zip(edges[0::2], edges[1::2]):
            x0, x1 = float(origin + start * lattice), float(origin + end * lattice)
            if x1 - x0 >= minimum:
                found.append({"surface": surface["id"], "x0": x0, "x1": x1, "y": surface["y"]})
    return found


def standing(lattice, stand):
    """📍️ For every query the index of the perch of its surface that owns the lattice point ``x``, the first one where two touch; ``None`` on free ground."""
    sides = [side for perch in stand["perches"] for side in (perch["x0"], perch["x1"])] + [query["x"] for query in stand["queries"]]
    origin = min(sides)
    answers = []
    for query in stand["queries"]:
        owner = numpy.full(cells(lattice, origin, max(sides)) + 1, -1, dtype=int)
        for index in range(len(stand["perches"]) - 1, -1, -1):
            perch = stand["perches"][index]
            if perch["surface"] == query["surface"]:
                owner[cells(lattice, origin, perch["x0"]) : cells(lattice, origin, perch["x1"]) + 1] = index
        owned = int(owner[cells(lattice, origin, query["x"])])
        answers.append(None if owned < 0 else owned)
    return answers


def nearest(lattice, near):
    """🧲️ For every point the index of the perch with the closest sampled lattice point, the first one among equals; ``None`` without perches."""
    answers = []
    aligned(lattice, [side for point in near["points"] for side in (point["x"], point["y"])])
    for point in near["points"]:
        least = []
        for perch in near["perches"]:
            samples = perch["x0"] + numpy.arange(cells(lattice, perch["x0"], perch["x1"]) + 1) * lattice
            least.append(float(numpy.min((samples - point["x"]) ** 2 + (perch["y"] - point["y"]) ** 2)))
        answers.append(int(numpy.argmin(numpy.array(least))) if least else None)
    return answers


def strides(walk):
    """👣️ The x after every stated tick of a walk, confirmed by its closed form: ``min(k × step, distance)`` covered towards the goal, the goal itself once it is reached."""
    step = max(walk["speed"], 0) / TICKS_PER_SECOND
    distance = abs(walk["goal"] - walk["x"])
    covered = numpy.arange(1, walk["ticks"] + 1) * step
    judged = numpy.where(covered >= distance, walk["goal"], walk["x"] + numpy.sign(walk["goal"] - walk["x"]) * covered)
    positions, position = [], walk["x"]
    for _ in range(walk["ticks"]):
        gap = walk["goal"] - position
        position = position + step if gap > step else position - step if gap < -step else walk["goal"]
        positions.append(position)
    if len(positions) and float(numpy.abs(numpy.array(positions, dtype=float) - judged).max()) > STEPPING:
        raise AssertionError("strides/%s: the stated ticks give %r, the closed form %r" % (walk["id"], positions, judged.tolist()))
    return positions


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


def perch_cutting(ctx):
    """✂️ The perches of every committed layout."""
    document = committed(ctx)
    return agree("perches", {layout["id"]: perches(document["lattice"], layout) for layout in document["layouts"]}, document["layouts"])


def perch_standing(ctx):
    """🧍️ The perch under every committed stand."""
    document = committed(ctx)
    return agree("standing", {stand["id"]: standing(document["lattice"], stand) for stand in document["standings"]}, document["standings"])


def perch_nearest(ctx):
    """📡️ The nearest perch of every committed point."""
    document = committed(ctx)
    return agree("nearest", {near["id"]: nearest(document["lattice"], near) for near in document["nearests"]}, document["nearests"])


def walking(ctx):
    """🚶️ Every committed walk, tick by tick."""
    walks = committed(ctx)["strides"]
    return agree("strides", {walk["id"]: strides(walk) for walk in walks}, walks)


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter():
    """🧭️ Oracle role only: numpy is the reference, the TypeScript and Rust twins are judged against it."""
    return Adapter("python").oracle("perches", perch_cutting).oracle("standing", perch_standing).oracle("nearest", perch_nearest).oracle("strides", walking)


# endregion 🔖️Registration

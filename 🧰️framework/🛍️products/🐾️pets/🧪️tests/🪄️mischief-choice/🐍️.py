#!/usr/bin/env python3
"""🪄️ Oracle of the pets mischief (design-v2 §14 rule 7, §20), in Python.

Whether a ground covers a key is Python's own string test: the two are equal, or ``key.startswith(ground + "/")``;
an empty ground covers nothing. The pick among the fitting fixtures is ``numpy.floor(unit × count)``, clipped by
``numpy.clip``. The gates are one boolean expression evaluated by numpy over all committed occasions at once.
The station is found by listing every perch and every wall stretch beside the fixture and taking
``numpy.argmax`` of their rooms (the first of equals). The path of a lifted copy is written without a single
branch: every phase is a ramp ``numpy.interp`` clamps to [0, 1], eased by the cubic ``3t² − 2t³`` and bent by
``numpy.sin``; the subject walks through its phases with comparisons instead.

The scenario ``leaks`` is the heart of rule 7: a host description carries, beside the key and the box of every
item, its value, whether it is correct and what the learner answered. The reference never looks at those three
and so cannot follow them; the subject is handed the whole description again and again with the three permuted
among the items (``numpy.random.Generator.permutation`` chose the committed permutations), and must name the
same fixture every time.

@see https://docs.python.org/3/library/stdtypes.html#str.startswith
@see https://numpy.org/doc/stable/reference/generated/numpy.interp.html
@see https://numpy.org/doc/stable/reference/generated/numpy.argmax.html
@see ../../🧫️fixtures/🪄️mischief-choice/🔣️.json
"""

# region 🔖️Imports
import json

import numpy

from semio_repo_test import Adapter, Outcome

# endregion 🔖️Imports


# region 🔖️Constants
VECTORS = "shared://🪄️mischief-choice/🔣️.json"
TOLERANCE = 1e-12
MISCHIEF_WIDTH = 1024
MISCHIEF_PATIENCE = 768
MISCHIEF_PATIENCE_QUIET = 1920
MISCHIEF_COOLDOWN = {"calm": 11520, "lively": 2880}
STATION_GAP = 24
STATION_SLACK = 2
STATION_STEP = 12
LIFT_ROOM = 12
LIFT_BRACE = 24
LIFT_SHOVE = 40
LIFT_WOBBLE = 48
LIFT_HOLD = 512
LIFT_RETURN = 56
LIFT_FADE = 8
LIFT_RETURNS = LIFT_BRACE + LIFT_SHOVE + LIFT_WOBBLE + LIFT_HOLD
LIFT_TICKS = LIFT_RETURNS + LIFT_RETURN + LIFT_FADE
LIFT_LEAST = 0.6
LIFT_GIVE = 1.5
LIFT_RISE = 2.0
LIFT_TILT = 0.004
LIFT_WOBBLES = 2
THROW_SPEED = 120.0
THROW_SPREAD = 80.0
THROW_LIFT = 220.0
# endregion 🔖️Constants


# region 🔖️Choice
def fits(ground, key):
    """🧩️ Whether a ground covers a key: equal, or a prefix of it up to a ``/``; an empty ground covers nothing."""
    return ground != "" and (key == ground or key.startswith(ground + "/"))


def fitting(grounds, fixtures):
    """🗂️ The fixtures a species may play with, in the survey's order; only keys are read."""
    return [fixture for fixture in fixtures if any(fits(ground, fixture["key"]) for ground in grounds)]


def position(count, unit):
    """🎯️ The candidate a unit draw picks among ``count``: ``⌊unit × count⌋`` held inside the list, −1 without candidates."""
    if count == 0:
        return -1
    return int(numpy.clip(numpy.floor(numpy.float64(unit) * count), 0, count - 1))


def chosen(grounds, fixtures, unit):
    """🧲️ The id of the fixture a species picks, ``None`` when nothing fits."""
    candidates = fitting(grounds, fixtures)
    index = position(len(candidates), unit)
    return None if index < 0 else candidates[index]["id"]


def leak(vector):
    """🙈️ The pick over the plain host description and over every committed permutation of the values, the correctness flags and the answers of its items; the reference reads none of the three."""
    items = vector["items"]
    plain = chosen(vector["grounds"], items, vector["unit"])
    shuffled = []
    for shuffle in vector["shuffles"]:
        if sorted(shuffle) != list(range(len(items))):
            raise AssertionError("leaks/%s: %r is not a permutation of the items" % (vector["id"], shuffle))
        permuted = [dict(item, value=items[source]["value"], correct=items[source]["correct"], answered=items[source]["answered"]) for item, source in zip(items, shuffle)]
        shuffled.append(chosen(vector["grounds"], permuted, vector["unit"]))
    if any(pick != plain for pick in shuffled):
        raise AssertionError("leaks/%s: the pick follows a value, a flag or an answer" % vector["id"])
    return {"plain": plain, "shuffled": shuffled}


# endregion 🔖️Choice


# region 🔖️Gates
def gates(occasions):
    """🚥️ Whether mischief may start and from which tick the gates of time are open, for all occasions at once."""
    column = lambda name, kind: numpy.array([occasion[name] for occasion in occasions], dtype=kind)
    modes = [occasion["mode"] for occasion in occasions]
    patience = numpy.where(column("quiet", bool), MISCHIEF_PATIENCE_QUIET, MISCHIEF_PATIENCE)
    cooldown = numpy.array([MISCHIEF_COOLDOWN.get(mode, 0) for mode in modes], dtype=numpy.int64)
    lively = numpy.array([mode in MISCHIEF_COOLDOWN for mode in modes], dtype=bool)
    standing = column("permitted", bool) & column("fine", bool) & ~column("lifting", bool) & lively & (column("width", numpy.float64) >= MISCHIEF_WIDTH)
    opening = numpy.maximum(column("stirred", numpy.int64) + patience, column("rested", numpy.int64) + cooldown)
    allowed = standing & (column("tick", numpy.int64) >= opening)
    return [{"allowed": bool(yes), "from": int(tick) if open_ else None} for yes, tick, open_ in zip(allowed, opening, standing)]


# endregion 🔖️Gates


# region 🔖️Station
def station(vector):
    """🧭️ Where a pusher works on a fixture: every perch and wall stretch beside it, the one with the most room first (``numpy.argmax``), ``None`` when none has room."""
    fixture = vector["fixture"]
    left = fixture["x"]
    right = fixture["x"] + fixture["width"]
    top = fixture["y"]
    bottom = fixture["y"] + fixture["height"]
    found = []
    for perch in vector["perches"]:
        if not (top < perch["y"] <= bottom + STATION_STEP):
            continue
        if perch["x0"] < left and left - min(perch["x1"], left) <= STATION_GAP:
            found.append({"footing": "perch", "wall": None, "surface": perch["surface"], "x": min(perch["x1"], left), "y": perch["y"], "side": 1, "room": vector["width"] - right})
        if perch["x1"] > right and max(perch["x0"], right) - right <= STATION_GAP:
            found.append({"footing": "perch", "wall": None, "surface": perch["surface"], "x": max(perch["x0"], right), "y": perch["y"], "side": -1, "room": left})
    for pitch in vector["pitches"]:
        if not (pitch["y0"] < bottom and pitch["y1"] > top):
            continue
        feet = float(numpy.clip(numpy.float64(bottom), pitch["y0"], pitch["y1"]))
        if pitch["side"] == -1 and -STATION_SLACK <= left - pitch["x"] <= STATION_GAP:
            found.append({"footing": "wall", "wall": pitch["wall"], "surface": pitch["surface"], "x": pitch["x"], "y": feet, "side": 1, "room": vector["width"] - right})
        if pitch["side"] == 1 and -STATION_SLACK <= pitch["x"] - right <= STATION_GAP:
            found.append({"footing": "wall", "wall": pitch["wall"], "surface": pitch["surface"], "x": pitch["x"], "y": feet, "side": -1, "room": left})
    roomy = [entry for entry in found if entry["room"] >= LIFT_ROOM]
    if not roomy:
        return None
    return roomy[int(numpy.argmax(numpy.array([entry["room"] for entry in roomy], dtype=numpy.float64)))]


# endregion 🔖️Station


# region 🔖️Lift
def ease(amounts):
    """🛝️ ``3t² − 2t³``."""
    return amounts * amounts * (3.0 - 2.0 * amounts)


def ramp(ages, start, length):
    """📈️ The phase of a stretch of the lift: 0 before it, 1 after it, rising evenly in between (``numpy.interp``)."""
    return numpy.interp(ages, [start, start + length], [0.0, 1.0])


def trajectory(vector):
    """🎢️ The copy of a lifted fixture at every committed tick: dx, dy, tilt and opacity as four lists, and the tick the lift ends."""
    ticks = numpy.arange(vector["first"], vector["last"] + 1, vector["step"], dtype=numpy.float64)
    ages = ticks - vector["since"]
    side = float(vector["side"])
    travel = max(0.0, float(vector["room"])) * (LIFT_LEAST + (1.0 - LIFT_LEAST) * numpy.float64(vector["unit"]))
    lean = min(LIFT_TILT, LIFT_RISE / (numpy.pi * vector["span"])) if vector["span"] > 0 else LIFT_TILT
    give = ramp(ages, LIFT_FADE, LIFT_BRACE - LIFT_FADE)
    shove = ramp(ages, LIFT_BRACE, LIFT_SHOVE)
    wobble = ramp(ages, LIFT_BRACE + LIFT_SHOVE, LIFT_WOBBLE)
    home = ramp(ages, LIFT_RETURNS, LIFT_RETURN)
    rock = numpy.sin(2.0 * numpy.pi * LIFT_WOBBLES * wobble) * (1.0 - ease(wobble))
    dx = side * (-LIFT_GIVE * numpy.sin(numpy.pi * give) + travel * (ease(shove) - ease(home)) + LIFT_GIVE * rock)
    dy = -LIFT_RISE * (numpy.sin(numpy.pi * shove) + numpy.sin(numpy.pi * home)) + 0.5 * LIFT_RISE * numpy.sin(4.0 * numpy.pi * LIFT_WOBBLES * wobble) * (1.0 - ease(wobble))
    tilt = side * lean * (numpy.sin(numpy.pi * shove) - rock - numpy.sin(numpy.pi * home))
    opacity = ease(ramp(ages, 0, LIFT_FADE)) * (1.0 - ease(ramp(ages, LIFT_TICKS - LIFT_FADE, LIFT_FADE)))
    during = (ages >= 0) & (ages < LIFT_TICKS)
    if numpy.any(numpy.abs(dy) > LIFT_RISE + 1e-9) or numpy.any(numpy.abs(tilt) > LIFT_TILT + 1e-12):
        raise AssertionError("lifts/%s: the copy leaves its row or tilts too far" % vector["id"])
    listed = lambda values: [float(value) for value in numpy.where(during, values, 0.0) + 0.0]
    return {"dx": listed(dx), "dy": listed(dy), "tilt": listed(tilt), "opacity": listed(opacity), "ends": vector["since"] + LIFT_TICKS}


def toss(vector):
    """💨️ The velocity of a pusher thrown off: away from the fixture's middle, faster by the draw, and upwards."""
    speed = float(THROW_SPEED + THROW_SPREAD * numpy.float64(vector["unit"]))
    middle = vector["fixture"]["x"] + vector["fixture"]["width"] / 2
    return {"vx": float(numpy.where(vector["pusher"]["x"] < middle, -speed, speed)), "vy": -THROW_LIFT}


# endregion 🔖️Lift


# region 🔖️Handlers
def committed(ctx):
    """🧫️ The committed vectors."""
    return json.loads(ctx.input_bytes(VECTORS))


def close(produced, expected):
    """🤏️ Structural equality with a tolerance of 1e-12 on fractions (numpy's sine may differ by an ulp between builds); everything else compares exactly."""
    if isinstance(produced, bool) or isinstance(expected, bool) or produced is None or expected is None or isinstance(produced, str) or isinstance(expected, str):
        return produced == expected and type(produced) == type(expected)
    if isinstance(produced, (int, float)) and isinstance(expected, (int, float)):
        return abs(produced - expected) <= TOLERANCE * max(1.0, abs(expected))
    if isinstance(produced, dict) and isinstance(expected, dict):
        return produced.keys() == expected.keys() and all(close(produced[key], expected[key]) for key in produced)
    if isinstance(produced, list) and isinstance(expected, list):
        return len(produced) == len(expected) and all(close(left, right) for left, right in zip(produced, expected))
    return produced == expected


def agree(scenario, produced, vectors):
    """⚖️ Holds every produced answer to the committed one — the vectors may never drift from the reference."""
    for vector in vectors:
        if not close(produced[vector["id"]], vector["expected"]):
            raise AssertionError("%s/%s: the reference answers %r, the committed vector says %r" % (scenario, vector["id"], produced[vector["id"]], vector["expected"]))
    return Outcome(produced)


def matches(ctx):
    """🔤️ Whether every committed ground covers its key."""
    vectors = committed(ctx)["matches"]
    return agree("matches", {vector["id"]: fits(vector["ground"], vector["key"]) for vector in vectors}, vectors)


def candidates(ctx):
    """📇️ The ids of the fixtures that fit every committed species, in the survey's order."""
    vectors = committed(ctx)["candidates"]
    return agree("candidates", {vector["id"]: [fixture["id"] for fixture in fitting(vector["grounds"], vector["fixtures"])] for vector in vectors}, vectors)


def choices(ctx):
    """🎰️ The position every committed unit picks among every committed number of candidates."""
    vectors = committed(ctx)["choices"]
    return agree("choices", {vector["id"]: [position(vector["count"], unit) for unit in vector["units"]] for vector in vectors}, vectors)


def leaks(ctx):
    """🕵️ The pick over every committed host description, plain and with its values, flags and answers permuted."""
    vectors = committed(ctx)["leaks"]
    return agree("leaks", {vector["id"]: leak(vector) for vector in vectors}, vectors)


def gate_rows(ctx):
    """🚦️ The verdict of the gates for every committed occasion."""
    vectors = committed(ctx)["gates"]
    return agree("gates", {vector["id"]: verdict for vector, verdict in zip(vectors, gates(vectors))}, vectors)


def stations(ctx):
    """🏗️ The station of every committed fixture."""
    vectors = committed(ctx)["stations"]
    return agree("stations", {vector["id"]: station(vector) for vector in vectors}, vectors)


def lifts(ctx):
    """🪞️ The path of every committed lift."""
    vectors = committed(ctx)["lifts"]
    return agree("lifts", {vector["id"]: trajectory(vector) for vector in vectors}, vectors)


def throws(ctx):
    """🤾️ The velocity of every committed throw."""
    vectors = committed(ctx)["throws"]
    return agree("throws", {vector["id"]: toss(vector) for vector in vectors}, vectors)


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter():
    """🔮️ Oracle role only: Python's strings and numpy are the reference, the TypeScript and Rust twins are judged against them."""
    return Adapter("python").oracle("matches", matches).oracle("candidates", candidates).oracle("choices", choices).oracle("leaks", leaks).oracle("gates", gate_rows).oracle("stations", stations).oracle("lifts", lifts).oracle("throws", throws)


# endregion 🔖️Registration

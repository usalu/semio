#!/usr/bin/env python3
"""🎣️ Oracle of the grappling rope of the pets product (design-v2 §16, MECH §3): which edge a shot reaches, the flight of the hook, the haul up the rope — straight or swinging — and the ways between two perches, in Python.

A shot is chosen among candidate points on the edges above — their corners, where the hook bites, and
the point nearest to the feet: the rope must have the right length (``numpy.hypot``), rise steeply
enough and have a clear line, and the least rope and detour wins (``numpy.argmin``, the first among
equals). A steep rope is reeled in straight, a slanted one swings — unless the swing would carry the
actor into a keep-out under the hook: then it is hauled in straight too. Whether a line is clear is not clipped: the neighbouring
oracle of case 🧗️wall-climbing judges segment against box by their separating axes (``numpy.cross``)
and lays 4097 sampled points over it. The hook flies ``min(speed × k ÷ 64, distance)`` along its line;
a straight haul shortens the rope by ``numpy.cumsum`` of its eased speeds down to its least, the hands
stay on the taut line (``numpy.hypot`` and ``numpy.cross``).

A swinging haul is a pendulum on a rope that shortens: ``scipy.integrate.solve_ivp`` (through the
neighbouring oracle of case 🪢️swing-dynamics, DOP853 with tolerances of 1e-12) integrates
``r·θ'' = −g·sin θ − γ·r·θ' − 2·r'·θ'`` with the length ``r(t)`` the reel prescribes, from rest half a
tick before the first step — a step that begins with no displacement is at rest between two ticks. The
restated steps must stay within the tolerance in degrees every committed swing states (MECH: 0.4°)
while the rope is longer than 0.4 of its start, the range MECH vouches for; they are what the oracle
answers, since the approximation is far coarser than the comparison tolerance of 1e-9.

Every other number the oracle answers is the plain binary64 value of the stated arithmetic, written
here from the design text, and refused unless the libraries reach it within 1e-9. The reasons a
candidate is refused (too low, too short, too long, too flat, blocked) are part of every committed
shot, so the vectors keep covering every rule. The routes are a second reading of design-v2 §16 that
composes the three oracles of walls, ladders and ropes: a supplement, on top of the geometry above.

@see https://docs.scipy.org/doc/scipy/reference/generated/scipy.integrate.solve_ivp.html
@see https://numpy.org/doc/stable/reference/generated/numpy.argmin.html
@see ../🧗️wall-climbing/🐍️.py
@see ../🪜️ladder-geometry/🐍️.py
@see ../🪢️swing-dynamics/🐍️.py
@see ../../🧫️fixtures/🎣️grapple-reach/🔣️.json
"""

# region 🔖️Imports
import importlib.util
import json
import math
import os

import numpy

from semio_repo_test import Adapter, Outcome

# endregion 🔖️Imports


# region 🔖️Constants
VECTORS = "shared://🎣️grapple-reach/🔣️.json"
TICKS_PER_SECOND = 64
HOOK_SPEED = 640.0
HOOK_RETURN = 1280.0
ROPE_SHORT = 0.8
ROPE_LONG = 8.0
ROPE_ELEVATION = 0.42
ROPE_MARGIN = 2.0
ROPE_DETOUR = 0.3
ROPE_FOLLOW = 8.0
ROPE_RISE = 0.8
MUZZLE_FORWARD = 0.3
MUZZLE_HEIGHT = 0.65
HOOK_INSET = 4.0
HOOK_LIFT = 1.0
ZIP_SLANT = 0.35
ZIP_SPEED = 150.0
ZIP_RAMP = 10
ROPE_AIM_TICKS = 14
ROPE_RECOIL_TICKS = 6
ROPE_TUG_TICKS = 4
ROPE_HOIST_TICKS = 46
ROPE_SHRUG_TICKS = 40
ROPE_MISS_CHANCE = 0.1
ROPE_MISS_OVERSHOOT = 14.0
ROPE_REST = 256
ROPE_SULK = 3840
CONSTANTS = {
    "hookSpeed": HOOK_SPEED,
    "hookReturn": HOOK_RETURN,
    "ropeShort": ROPE_SHORT,
    "ropeLong": ROPE_LONG,
    "ropeElevation": ROPE_ELEVATION,
    "ropeMargin": ROPE_MARGIN,
    "ropeDetour": ROPE_DETOUR,
    "ropeFollow": ROPE_FOLLOW,
    "ropeRise": ROPE_RISE,
    "muzzleForward": MUZZLE_FORWARD,
    "muzzleHeight": MUZZLE_HEIGHT,
    "hookInset": HOOK_INSET,
    "hookLift": HOOK_LIFT,
    "zipSlant": ZIP_SLANT,
    "zipSpeed": ZIP_SPEED,
    "zipRamp": ZIP_RAMP,
    "ropeAimTicks": ROPE_AIM_TICKS,
    "ropeRecoilTicks": ROPE_RECOIL_TICKS,
    "ropeTugTicks": ROPE_TUG_TICKS,
    "ropeHoistTicks": ROPE_HOIST_TICKS,
    "ropeShrugTicks": ROPE_SHRUG_TICKS,
    "ropeMissChance": ROPE_MISS_CHANCE,
    "ropeMissOvershoot": ROPE_MISS_OVERSHOOT,
    "ropeRest": ROPE_REST,
    "ropeSulk": ROPE_SULK,
}
STEPPING = 1e-9
MARGIN = 1e-6
PATIENCE = 1024
VOUCHED = 0.4
MEANS = ("climb", "ladder", "grapple")
# endregion 🔖️Constants


# region 🔖️Neighbours
def neighbour(case):
    """🚪️ The oracle adapter of another case of this owner, loaded from its file."""
    spec = importlib.util.spec_from_file_location("pets_" + case.encode("ascii", "ignore").decode("ascii").replace("-", "_"), os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", case, "🐍️.py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


WALLS = neighbour("🧗️wall-climbing")
LADDERS = neighbour("🪜️ladder-geometry")
SWING = neighbour("🪢️swing-dynamics")
# endregion 🔖️Neighbours


# region 🔖️Shots
def span(start, end):
    """🪡️ The stated distance between two points."""
    dx = end["x"] - start["x"]
    dy = end["y"] - start["y"]
    return math.sqrt(dx * dx + dy * dy)


def held(value, low, high):
    """🗜️ A value held inside two bounds: raised to the lower, then lowered to the upper."""
    raised = low if value < low else value
    return high if raised > high else raised


def aimed(label, feet, perch, x, keepouts, size):
    """🔦️ The shot from the feet at a point of an edge and the reason there is none: ``(shot, verdict)``."""
    facing = -1 if x < feet["x"] else 1
    muzzle = {"x": feet["x"] + facing * MUZZLE_FORWARD * size["width"], "y": feet["y"] - MUZZLE_HEIGHT * size["height"]}
    hook = {"x": x, "y": perch["y"] - HOOK_LIFT}
    across = abs(hook["x"] - muzzle["x"])
    rise = muzzle["y"] - hook["y"]
    length = span(muzzle, hook)
    WALLS.confirmed("%s: the rope" % label, [length], [float(numpy.hypot(hook["x"] - muzzle["x"], hook["y"] - muzzle["y"]))])
    if not ROPE_SHORT * size["height"] <= length:
        return None, "too-short"
    if not length <= ROPE_LONG * size["height"]:
        return None, "too-long"
    if not rise >= ROPE_ELEVATION * length:
        return None, "too-flat"
    under = [{"x": x, "y": perch["y"]}]
    if not LADDERS.sighted(label, muzzle, hook, keepouts, ROPE_MARGIN, under):
        return None, "blocked"
    swings = across > ZIP_SLANT * rise and LADDERS.sighted("%s: the swing under the hook" % label, hook, {"x": x, "y": hook["y"] + length}, keepouts, ROPE_MARGIN, under)
    return {"surface": perch["surface"], "facing": facing, "muzzle": muzzle, "hook": hook, "length": length, "reel": "swing" if swings else "zip"}, "granted"


def shot_for(label, feet, perches, keepouts, size):
    """🎇️ The shot with the least rope and detour among the candidates of every perch high enough above the feet — its corners, where the hook bites, and the point between them nearest to the feet (``numpy.argmin``, the first among equals) —, and the verdict of every candidate: ``(shot, verdicts)``."""
    shots, verdicts = [], []
    for index, perch in enumerate(perches):
        if not feet["y"] - perch["y"] >= ROPE_RISE * size["height"]:
            verdicts.append(["too-low"])
            continue
        low, high = perch["x0"] + HOOK_INSET, perch["x1"] - HOOK_INSET
        found = []
        for number, spot in enumerate([low, high, held(feet["x"], low, high)] if low <= high else [(perch["x0"] + perch["x1"]) / 2]):
            shot, verdict = aimed("%s perch %d spot %d" % (label, index, number), feet, perch, spot, keepouts, size)
            found.append(verdict)
            if shot is not None:
                shots.append(shot)
        verdicts.append(found)
    if not shots:
        return None, verdicts
    return shots[int(numpy.argmin(numpy.array([shot["length"] + ROPE_DETOUR * abs(shot["hook"]["x"] - shot["muzzle"]["x"]) for shot in shots])))], verdicts


def shot_holds(label, shot, perches, keepouts):
    """🧿️ The shot as it holds after a survey: the hook follows the edge of its surface up or down while that edge still carries it and the line stays clear; ``None`` when the hook lost its edge."""
    for perch in perches:
        if perch["surface"] != shot["surface"] or not perch["x0"] <= shot["hook"]["x"] <= perch["x1"] or abs(perch["y"] - HOOK_LIFT - shot["hook"]["y"]) > ROPE_FOLLOW:
            continue
        hook = {"x": shot["hook"]["x"], "y": perch["y"] - HOOK_LIFT}
        if LADDERS.sighted(label, shot["muzzle"], hook, keepouts, ROPE_MARGIN, [{"x": hook["x"], "y": perch["y"]}]):
            return {"surface": shot["surface"], "facing": shot["facing"], "muzzle": shot["muzzle"], "hook": hook, "length": span(shot["muzzle"], hook), "reel": shot["reel"]}
    return None


def landing(vector):
    """🏕️ Where an actor stands once it is up its rope, ``numpy.clip`` of the spot inside the hook, and where a shot that is meant to miss is aimed."""
    shot, perch, size = vector["shot"], vector["perch"], vector["size"]
    inward = 1 if shot["hook"]["x"] <= (perch["x0"] + perch["x1"]) / 2 else -1
    spot = held(shot["hook"]["x"] + inward * (size["width"] / 2 + WALLS.MANTLE_INSET), perch["x0"], perch["x1"])
    WALLS.confirmed("landings/%s" % vector["id"], [spot], [float(numpy.clip(shot["hook"]["x"] + inward * (size["width"] / 2 + WALLS.MANTLE_INSET), perch["x0"], perch["x1"]))])
    return {"landing": {"x": spot, "y": perch["y"]}, "miss": {"x": perch["x0"] - ROPE_MISS_OVERSHOOT if shot["hook"]["x"] - perch["x0"] <= perch["x1"] - shot["hook"]["x"] else perch["x1"] + ROPE_MISS_OVERSHOOT, "y": shot["hook"]["y"]}}


# endregion 🔖️Shots


# region 🔖️Flight
def ceiling(value):
    """🧮️ The least whole number that is not below a value, by ``floor`` alone."""
    return 0 - math.floor(0 - value)


def hook_step(start, end, speed, ticks):
    """🪃️ The stated hook after a number of ticks on its straight line: the start itself at 0, the end itself once it has arrived."""
    length = span(start, end)
    if not ticks > 0:
        return [start["x"], start["y"]]
    if ticks >= ceiling((length * TICKS_PER_SECOND) / speed):
        return [end["x"], end["y"]]
    share = (speed * ticks) / TICKS_PER_SECOND / length
    return [start["x"] + (end["x"] - start["x"]) * share, start["y"] + (end["y"] - start["y"]) * share]


def flight(vector):
    """🏹️ The ticks of a flight and the hook after every one of them, from tick 0 to one tick after its arrival; refused unless the closed form ``start + direction × min(speed × k ÷ 64, distance)`` reaches the same path and ``numpy.ceil`` the same ticks."""
    label = "flights/%s" % vector["id"]
    start, end, speed = vector["from"], vector["to"], vector["speed"]
    length = span(start, end)
    ticks = ceiling((length * TICKS_PER_SECOND) / speed)
    distance = float(numpy.hypot(end["x"] - start["x"], end["y"] - start["y"]))
    due = distance * TICKS_PER_SECOND / speed
    if abs(due - round(due)) < MARGIN and due != round(due):
        raise AssertionError("%s: the flight lasts %r ticks, on the edge of two tick counts" % (label, due))
    if ticks != int(numpy.ceil(due)):
        raise AssertionError("%s: %d stated ticks, %d by numpy" % (label, ticks, int(numpy.ceil(due))))
    path = [hook_step(start, end, speed, tick) for tick in range(ticks + 2)]
    covered = numpy.minimum(speed * numpy.arange(ticks + 2) / TICKS_PER_SECOND, distance)
    direction = numpy.array([end["x"] - start["x"], end["y"] - start["y"]]) / (distance if distance > 0 else 1.0)
    WALLS.confirmed("%s: the path" % label, path, numpy.array([start["x"], start["y"]]) + covered[:, None] * direction[None, :])
    return {"ticks": ticks, "path": path}


# endregion 🔖️Flight


# region 🔖️Haul
def hung(shot, rope, hand, before, size):
    """🐟️ The actor whose hands hold a rope at a point: its feet hang under the hands as they stood under the muzzle."""
    return {"rope": rope, "hand": hand, "before": before, "x": hand["x"] - shot["facing"] * MUZZLE_FORWARD * size["width"], "y": hand["y"] + MUZZLE_HEIGHT * size["height"]}


def lined(shot, rope):
    """🐠️ The point of the taut line of a shot a length of rope from the hook."""
    share = rope / shot["length"]
    return {"x": shot["hook"]["x"] + (shot["muzzle"]["x"] - shot["hook"]["x"]) * share, "y": shot["hook"]["y"] + (shot["muzzle"]["y"] - shot["hook"]["y"]) * share}


def haul_of(shot, rope, size):
    """🎏️ The actor at rest on the taut line of a shot with a length of rope left."""
    hand = lined(shot, rope)
    return hung(shot, rope, hand, hand, size)


def zip_step(shot, haul, ticks, size):
    """🚠️ One stated tick of reeling straight up a rope: shortened by the zip speed, gathered over its ramp, down to the least rope."""
    rope = max(haul["rope"] - (ZIP_SPEED * WALLS.smoothstep((ticks + 1) / ZIP_RAMP)) / TICKS_PER_SECOND, min(SWING.REEL_LEAST * size["height"], haul["rope"]))
    return hung(shot, rope, lined(shot, rope), haul["hand"], size)


def sway_step(shot, haul, ticks, size):
    """🎪️ One stated tick of swinging on a rope that is reeled in: the hands are the pendulum of the neighbouring oracle's restated reel step."""
    reel = SWING.reel_step(shot["hook"], haul["hand"], haul["before"], haul["rope"], SWING.REEL_LEAST * size["height"], ticks)
    return hung(shot, reel["length"], reel["bob"], haul["hand"], size)


def hauled(vector, step):
    """🎬️ The states of a haul after every stated tick until the least rope is left."""
    shot, size = vector["shot"], vector["size"]
    least = SWING.REEL_LEAST * size["height"]
    haul, states = haul_of(shot, shot["length"], size), []
    while haul["rope"] > least and len(states) < PATIENCE:
        haul = step(shot, haul, len(states), size)
        states.append(haul)
    return states, least


def projected(states):
    """📽️ What a haul projects: its ticks, and the rope, the hands and the feet after every one of them."""
    return {"ticks": len(states), "ropes": [state["rope"] for state in states], "hands": [[state["hand"]["x"], state["hand"]["y"]] for state in states], "feet": [[state["x"], state["y"]] for state in states]}


def zipped(vector):
    """⚡️ A straight haul, tick by tick; refused unless ``numpy.cumsum`` of the eased speeds, held at the least rope, reaches the ropes, and ``numpy.hypot`` and ``numpy.cross`` find the hands that far from the hook on the taut line, the feet under them."""
    label = "zips/%s" % vector["id"]
    shot, size = vector["shot"], vector["size"]
    states, least = hauled(vector, zip_step)
    ticks = len(states)
    covered = numpy.cumsum(ZIP_SPEED * WALLS.eased(numpy.arange(1, ticks + 1) / ZIP_RAMP) / TICKS_PER_SECOND)
    reach = shot["length"] - least
    if ticks and (float(numpy.abs(covered - reach).min()) < MARGIN or int(numpy.argmax(covered >= reach)) + 1 != ticks):
        raise AssertionError("%s: the zip ends on the edge of two tick counts, or not on the tick the sum of its speeds says" % label)
    WALLS.confirmed("%s: the ropes" % label, [state["rope"] for state in states], numpy.maximum(shot["length"] - covered, min(least, shot["length"])))
    hook = numpy.array([shot["hook"]["x"], shot["hook"]["y"]])
    line = numpy.array([shot["muzzle"]["x"] - hook[0], shot["muzzle"]["y"] - hook[1], 0.0])
    hands = numpy.array([[state["hand"]["x"], state["hand"]["y"]] for state in states], dtype=float).reshape(-1, 2) - hook
    WALLS.confirmed("%s: how far the hands are from the hook" % label, [state["rope"] for state in states], numpy.hypot(hands[:, 0], hands[:, 1]))
    WALLS.confirmed("%s: the hands on the taut line" % label, numpy.zeros(ticks), numpy.cross(numpy.tile(line, (ticks, 1)), numpy.column_stack([hands, numpy.zeros(ticks)]))[:, 2] / shot["length"])
    WALLS.confirmed("%s: the feet under the hands" % label, [[state["x"], state["y"]] for state in states], hands + hook + numpy.array([-shot["facing"] * MUZZLE_FORWARD * size["width"], MUZZLE_HEIGHT * size["height"]]))
    return projected(states)


def swung(vector):
    """🎡️ A swinging haul, tick by tick, and how far its angle leaves the integrated pendulum at most, in degrees, while the rope is longer than 0.4 of its start: ``(projection, deviation)``. Refused unless the rope follows the ramped reel down to its least, the hands never leave its reach and no step is cut by the cap — a swing the cap bends is no pendulum."""
    label = "swings/%s" % vector["id"]
    shot, size = vector["shot"], vector["size"]
    states, least = hauled(vector, sway_step)
    ticks = len(states)
    ropes = numpy.concatenate([[shot["length"]], SWING.reel_lengths({"length": shot["length"], "least": least}, ticks)])
    WALLS.confirmed("%s: the ropes" % label, [state["rope"] for state in states], ropes[1:])
    hands = numpy.array([[state["hand"]["x"] - shot["hook"]["x"], state["hand"]["y"] - shot["hook"]["y"]] for state in states], dtype=float)
    strides = numpy.diff(numpy.concatenate([[[shot["muzzle"]["x"] - shot["hook"]["x"], shot["muzzle"]["y"] - shot["hook"]["y"]]], hands]), axis=0)
    if float((numpy.hypot(hands[:, 0], hands[:, 1]) - ropes[1:]).max()) > STEPPING or float(numpy.hypot(strides[:, 0], strides[:, 1]).max()) * TICKS_PER_SECOND > SWING.REEL_CAP - MARGIN:
        raise AssertionError("%s: the hands leave the reach of the rope, or a step is cut by the cap" % label)
    gravity, drag = SWING.felt(SWING.GRAVITY, SWING.REEL_DAMPING)

    def winding(time):
        """⏳️ The length of the rope and its rate at a time counted from half a tick before the first step."""
        passed = (time - SWING.TICK / 2) * TICKS_PER_SECOND
        if passed <= 0:
            return float(ropes[0]), 0.0
        if passed >= ticks:
            return float(ropes[-1]), 0.0
        index = int(passed)
        rate = float(ropes[index + 1] - ropes[index]) * TICKS_PER_SECOND
        return float(ropes[index]) + rate * (passed - index) / TICKS_PER_SECOND, rate

    start = math.atan2(shot["muzzle"]["x"] - shot["hook"]["x"], shot["muzzle"]["y"] - shot["hook"]["y"])
    lead = float(SWING.pendulum(start, 0.0, gravity, drag, winding, SWING.still, [SWING.TICK / 2])[0]) - start
    reference = numpy.degrees(SWING.pendulum(start - lead, 0.0, gravity, drag, winding, SWING.still, (numpy.arange(1, ticks + 1) + 0.5) * SWING.TICK))
    measured = SWING.degrees_of([state["hand"] for state in states], [shot["hook"]] * ticks)
    vouched = ropes[1:] >= VOUCHED * shot["length"]
    return projected(states), SWING.apart(measured[vouched], reference[vouched])


# endregion 🔖️Haul


# region 🔖️Routes
def rests(ladder, perch):
    """🛤️ Whether a ladder stands on a perch."""
    return perch["surface"] == ladder["surface"] and perch["y"] == ladder["foot"]["y"] and perch["x0"] <= ladder["foot"]["x"] <= perch["x1"]


def ridden(ladder, start, end, pitches, size):
    """🚏️ The way over a standing ladder between two perches: up when it stands on the first and leans against a pitch the second crowns, down the other way round; ``None`` otherwise."""
    for pitch in pitches:
        if pitch["wall"] != ladder["wall"] or pitch["side"] != ladder["side"]:
            continue
        if rests(ladder, start) and WALLS.crowns(end, pitch, size):
            return {"means": "ladder", "at": ladder["foot"]["x"], "up": True}
        if WALLS.crowns(start, pitch, size) and rests(ladder, end):
            return {"means": "ladder", "at": WALLS.ledge_of(pitch, size), "up": False}
    return None


def route(vector):
    """🗾️ The ways between two perches of a committed stage for an actor with its gear and grip, most preferred first: a standing ladder, a wall, a ladder of its own where none stands, a shot from the first of its aims that has one; ``None`` when there is none."""
    label = "routes/%s" % vector["id"]
    perches, pitches, size, gear = vector["perches"], vector["pitches"], vector["size"], vector["gear"]
    start, end = perches[vector["from"]], perches[vector["to"]]
    legs = []
    if any(means in gear for means in MEANS):
        for index, ladder in enumerate(vector["ladders"]):
            leg = ridden(ladder, start, end, pitches, size)
            if leg is not None:
                legs.append(dict(leg, ladder=index))
    joined = len(legs) > 0
    if "climb" in gear:
        for index in range(len(pitches)):
            leg = WALLS.scaled(index, pitches, start, end, size, vector["grip"])
            if leg is not None:
                legs.append(dict(leg, means="wall", pitch=index))
    if "ladder" in gear and not joined:
        for index, pitch in enumerate(pitches):
            ladder, _verdict = LADDERS.ladder_for("%s pitch %d" % (label, index), start, end, pitch, vector["keepouts"], size)
            if ladder is not None:
                legs.append({"means": "raise", "at": ladder["foot"]["x"], "ladder": ladder})
    if "grapple" in gear:
        for number, stand in enumerate(aims(vector["x"], start, end, size)):
            shot, _verdicts = shot_for("%s stand %d" % (label, number), {"x": stand, "y": start["y"]}, [end], vector["keepouts"], size)
            if shot is not None:
                legs.append({"means": "grapple", "at": stand, "shot": shot})
                break
    return legs if legs else None


def aims(x, start, end, size):
    """🔙️ Where a shooter on a perch tries to shoot at another from, in order: where it stands, the points of its perch nearest to either end and to the middle of the other, then back from either end of the other a width farther at a time while a rope could still reach across."""
    stands = [x, held(end["x0"], start["x0"], start["x1"]), held(end["x1"], start["x0"], start["x1"]), held((end["x0"] + end["x1"]) / 2, start["x0"], start["x1"])]
    back = size["width"]
    while back <= ROPE_LONG * size["height"]:
        stands += [held(end["x0"] - back, start["x0"], start["x1"]), held(end["x1"] + back, start["x0"], start["x1"])]
        back = back + size["width"]
    return stands


# endregion 🔖️Routes


# region 🔖️Handlers
def committed(ctx):
    """🧫️ The committed vectors."""
    return json.loads(ctx.input_bytes(VECTORS))


def constants(ctx):
    """🎚️ The tuning constants the vectors were generated with; they must be the ones of this reference."""
    document = committed(ctx)
    if document["constants"] != CONSTANTS:
        raise AssertionError("constants: the reference is tuned to %r, the committed vectors to %r" % (CONSTANTS, document["constants"]))
    return Outcome(document["constants"])


def shots(ctx):
    """🎯️ The shot of every committed stand, ``None`` where no edge is in reach — every candidate for its committed reason."""
    produced = {}
    vectors = committed(ctx)["shots"]
    for vector in vectors:
        produced[vector["id"]], verdicts = shot_for("shots/%s" % vector["id"], vector["feet"], vector["perches"], vector["keepouts"], vector["size"])
        if verdicts != vector["verdicts"]:
            raise AssertionError("shots/%s: the reference says %r, the committed vector says %r" % (vector["id"], verdicts, vector["verdicts"]))
    return WALLS.agree("shots", produced, vectors)


def flights(ctx):
    """🕊️ Every committed flight of a hook, tick by tick."""
    vectors = committed(ctx)["flights"]
    return WALLS.agree("flights", {vector["id"]: flight(vector) for vector in vectors}, vectors)


def zips(ctx):
    """🧗️ Every committed straight haul, tick by tick."""
    vectors = committed(ctx)["zips"]
    return WALLS.agree("zips", {vector["id"]: zipped(vector) for vector in vectors}, vectors)


def swings(ctx):
    """🐒️ Every committed swinging haul, tick by tick, held to the pendulum within its stated tolerance."""
    produced = {}
    vectors = committed(ctx)["swings"]
    for vector in vectors:
        produced[vector["id"]], deviation = swung(vector)
        if not deviation <= vector["tolerance"]:
            raise AssertionError("swings/%s: the restated steps leave the integrated pendulum by %r°, more than the %r° the vector allows" % (vector["id"], deviation, vector["tolerance"]))
    return WALLS.agree("swings", produced, vectors)


def surveys(ctx):
    """🧐️ What every committed survey leaves of its shot."""
    vectors = committed(ctx)["surveys"]
    return WALLS.agree("surveys", {vector["id"]: shot_holds("surveys/%s" % vector["id"], vector["shot"], vector["perches"], vector["keepouts"]) for vector in vectors}, vectors)


def landings(ctx):
    """🛬️ Where every committed rope lets its actor off, and where its miss is aimed."""
    vectors = committed(ctx)["landings"]
    return WALLS.agree("landings", {vector["id"]: landing(vector) for vector in vectors}, vectors)


def routes(ctx):
    """🧭️ The ways between the perches of every committed stage."""
    vectors = committed(ctx)["routes"]
    return WALLS.agree("routes", {vector["id"]: route(vector) for vector in vectors}, vectors)


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter():
    """🔮️ Oracle role only: numpy and scipy are the reference, the TypeScript and Rust twins are judged against it."""
    return Adapter("python").oracle("constants", constants).oracle("shots", shots).oracle("flights", flights).oracle("zips", zips).oracle("swings", swings).oracle("surveys", surveys).oracle("landings", landings).oracle("routes", routes)


# endregion 🔖️Registration

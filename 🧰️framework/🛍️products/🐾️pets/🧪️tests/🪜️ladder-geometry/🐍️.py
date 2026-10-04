#!/usr/bin/env python3
"""🪜️ Oracle of the ladders of the pets product (design-v2 §16, MECH §4): where a ladder may stand, its length, lean and rungs, the climb along it, and whether it still stands after a survey, in Python.

A ladder is a segment from a foot on a lower perch to a contact on a wall: just under the rim, to reach the perch
on top, or at the lower end of the wall, to take hold of the wall from its exit. Its line keeps clear of every
keep-out on the air side of the wall; the body of the element whose side the wall is, which a survey grows a few
pixels past it, is what the ladder leans against. Its
measures are judged by numpy: the length by ``numpy.hypot``, the lean as the foot's distance from the
wall over the rise, the rungs as the whole spacings that fit, the place of a climber by its distance
from the foot (``numpy.hypot``) and by the ``numpy.cross`` product that keeps it on the rails, the
climb by ``numpy.cumsum`` of its eased speeds, held at its goal. Whether the line of a ladder is
clear is not clipped: the neighbouring oracle of case 🧗️wall-climbing judges segment against box by
their separating axes and lays 4097 sampled points over it, and refuses a vector whose verdict hangs
on less than a sample. Every number the oracle answers is the plain binary64 value of the stated
arithmetic, written here from the design text, and refused unless numpy reaches it within 1e-9.

Why a ladder is refused is part of every committed placement (no rim, too short, too tall, off the
pitch, no footing, too steep, too flat, blocked) and of every committed lean against a wall (the same,
and no grip where the hands cannot reach the wall from the exit): the oracle must find the same
reason, so the vectors keep covering every rule.

@see https://numpy.org/doc/stable/reference/generated/numpy.hypot.html
@see https://numpy.org/doc/stable/reference/generated/numpy.cross.html
@see ../🧗️wall-climbing/🐍️.py
@see ../../🧫️fixtures/🪜️ladder-geometry/🔣️.json
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
VECTORS = "shared://🪜️ladder-geometry/🔣️.json"
TICKS_PER_SECOND = 64
LADDER_LEAN = 0.25
LADDER_STEEP = 0.14
LADDER_FLAT = 0.4
LADDER_SHORT = 0.8
LADDER_TALL = 8.0
LADDER_TUCK = 7.0
LADDER_HORNS = 14.0
RUNG_SPACING = 10.5
LADDER_FOOTING = 10.0
LADDER_GIRTH = 6.0
LADDER_RISE = 26.0
LADDER_DESCENT = 34.0
LADDER_RAMP = 6
LADDER_EXIT = 0.4
LADDER_FOLLOW = 8.0
LADDER_SHIFT = 12.0
LADDER_MOUNT_TICKS = 8
LADDER_DISMOUNT_TICKS = 24
LADDER_RAISE_TICKS = 30
LADDER_IDLE = 1280
LADDER_LIFE = 7680
TOPPLE_STIFFNESS = 150.0
TOPPLE_DAMPING = 24.0
TOPPLE_PUSH = 70.0
TOPPLE_STEP = 0.5
CONSTANTS = {
    "ladderLean": LADDER_LEAN,
    "ladderSteep": LADDER_STEEP,
    "ladderFlat": LADDER_FLAT,
    "ladderShort": LADDER_SHORT,
    "ladderTall": LADDER_TALL,
    "ladderTuck": LADDER_TUCK,
    "ladderHorns": LADDER_HORNS,
    "rungSpacing": RUNG_SPACING,
    "ladderFooting": LADDER_FOOTING,
    "ladderGirth": LADDER_GIRTH,
    "ladderRise": LADDER_RISE,
    "ladderDescent": LADDER_DESCENT,
    "ladderRamp": LADDER_RAMP,
    "ladderExit": LADDER_EXIT,
    "ladderFollow": LADDER_FOLLOW,
    "ladderShift": LADDER_SHIFT,
    "ladderMountTicks": LADDER_MOUNT_TICKS,
    "ladderDismountTicks": LADDER_DISMOUNT_TICKS,
    "ladderRaiseTicks": LADDER_RAISE_TICKS,
    "ladderIdle": LADDER_IDLE,
    "ladderLife": LADDER_LIFE,
    "toppleStiffness": TOPPLE_STIFFNESS,
    "toppleDamping": TOPPLE_DAMPING,
    "topplePush": TOPPLE_PUSH,
    "toppleStep": TOPPLE_STEP,
}
STEPPING = 1e-9
MARGIN = 1e-6
PATIENCE = 1024
# endregion 🔖️Constants


# region 🔖️Neighbours
def neighbour(case):
    """🚪️ The oracle adapter of another case of this owner, loaded from its file."""
    spec = importlib.util.spec_from_file_location("pets_" + case.encode("ascii", "ignore").decode("ascii").replace("-", "_"), os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", case, "🐍️.py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


WALLS = neighbour("🧗️wall-climbing")
# endregion 🔖️Neighbours


# region 🔖️Placement
def holds(rect, point):
    """📦️ Whether a box has an area and holds a point, its edges included."""
    return rect["width"] > 0 and rect["height"] > 0 and rect["x"] <= point["x"] <= rect["x"] + rect["width"] and rect["y"] <= point["y"] <= rect["y"] + rect["height"]


def sighted(label, start, end, keepouts, margin, supports):
    """🔭️ Whether a line hits none of the keep-outs grown by a margin, passing over every keep-out that holds one of the points the line stands on or ends at; judged by the separating axes and the sampled segment of the neighbouring oracle."""
    for index, keepout in enumerate(keepouts):
        if not any(holds(keepout, support) for support in supports) and WALLS.hit("%s against keep-out %d" % (label, index), start, end, keepout, margin):
            return False
    return True


def length_of(ladder):
    """🎋️ The stated length of a ladder from its foot to its top."""
    dx = ladder["top"]["x"] - ladder["foot"]["x"]
    dy = ladder["top"]["y"] - ladder["foot"]["y"]
    return math.sqrt(dx * dx + dy * dy)


def flanking(keepouts, pitch):
    """🧾️ The keep-outs a ladder against a pitch keeps clear of: all but those that begin within the lip of the wall line on the side of its element — the body of the element, grown past its side by the survey, which the ladder leans against."""
    if pitch["side"] < 0:
        return [keepout for keepout in keepouts if keepout["x"] < pitch["x"] - WALLS.WALL_LIP]
    return [keepout for keepout in keepouts if keepout["x"] + keepout["width"] > pitch["x"] + WALLS.WALL_LIP]


def stood(label, low, pitch, x, y, keepouts, size):
    """🧍️ The ladder with its foot at ``x`` on a perch and its top touching a pitch at the height ``y``, and the reason it cannot stand so: ``(ladder, verdict)``; the contact lies on the pitch no higher than the tuck under its top."""
    top = {"x": pitch["x"], "y": y}
    foot = {"x": x, "y": low["y"]}
    rise = foot["y"] - top["y"]
    lean = (x - pitch["x"]) * pitch["side"]
    if not LADDER_SHORT * size["height"] <= rise:
        return None, "too-short"
    if not rise <= LADDER_TALL * size["height"]:
        return None, "too-tall"
    if not pitch["y0"] + LADDER_TUCK <= top["y"] <= pitch["y1"]:
        return None, "off-the-pitch"
    if not low["x0"] + LADDER_FOOTING <= x <= low["x1"] - LADDER_FOOTING:
        return None, "no-footing"
    if not LADDER_STEEP * rise <= lean:
        return None, "too-steep"
    if not lean <= LADDER_FLAT * rise:
        return None, "too-flat"
    if not sighted(label, foot, top, flanking(keepouts, pitch), LADDER_GIRTH, [foot, top]):
        return None, "blocked"
    return {"wall": pitch["wall"], "surface": low["surface"], "side": pitch["side"], "foot": foot, "top": top}, "granted"


def footed(low, pitch, top):
    """🦶️ Where the foot of a ladder with its top at a height stands: at the lean of real ladders, held inside the footing of its perch."""
    wanted = pitch["x"] + pitch["side"] * LADDER_LEAN * (low["y"] - top)
    raised = low["x0"] + LADDER_FOOTING if wanted < low["x0"] + LADDER_FOOTING else wanted
    return low["x1"] - LADDER_FOOTING if raised > low["x1"] - LADDER_FOOTING else raised


def ladder_for(label, low, high, pitch, keepouts, size):
    """🔨️ The ladder raised on a lower perch against a pitch to reach the perch that crowns it, its top the tuck under the rim, its foot as near to the lean of real ladders as the perch lets it, and the reason there is none: ``(ladder, verdict)``."""
    if not WALLS.crowns(high, pitch, size):
        return None, "no-rim"
    top = pitch["y0"] + LADDER_TUCK
    return stood(label, low, pitch, footed(low, pitch, top), top, keepouts, size)


def ladder_to(label, low, pitch, keepouts, size):
    """🪵️ The ladder raised on a perch against a pitch to take hold of the wall from its exit, its top on the lower end of the pitch, its foot as for any ladder, and the reason there is none: ``(ladder, verdict)`` — ``no-grip`` when a climber at its exit has no hands on the pitch."""
    ladder, verdict = stood(label, low, pitch, footed(low, pitch, pitch["y1"]), pitch["y1"], keepouts, size)
    if ladder is None:
        return None, verdict
    if not WALLS.clings(pitch, ladder_at(ladder, max(length_of(ladder) - LADDER_EXIT * size["height"], 0))[1], size):
        return None, "no-grip"
    return ladder, verdict


def measures(label, ladder, size):
    """📏️ The stated length, rungs, lean, exit and landing of a ladder; refused unless ``numpy.hypot`` reaches the length, the lean lies within the steepest and the flattest, and the rungs are the whole spacings that fit."""
    length = length_of(ladder)
    rise = ladder["foot"]["y"] - ladder["top"]["y"]
    lean = abs(ladder["foot"]["x"] - ladder["top"]["x"]) / rise
    judged = float(numpy.hypot(ladder["foot"]["x"] - ladder["top"]["x"], rise))
    fit = judged / RUNG_SPACING
    if abs(fit - round(fit)) < MARGIN and fit != round(fit):
        raise AssertionError("%s: %r rung spacings fit, on the edge of two rung counts" % (label, fit))
    WALLS.confirmed("%s: the length" % label, [length, lean], [judged, float(numpy.abs(ladder["foot"]["x"] - ladder["top"]["x"]) / rise)])
    if not LADDER_STEEP - STEPPING <= lean <= LADDER_FLAT + STEPPING:
        raise AssertionError("%s: a lean of %r is not one a ladder stands at" % (label, lean))
    rungs = math.floor(length / RUNG_SPACING)
    if rungs != int(numpy.floor(fit)):
        raise AssertionError("%s: %d stated rungs, %d by numpy" % (label, rungs, int(numpy.floor(fit))))
    return {"length": length, "rungs": rungs, "lean": lean, "exit": max(length - LADDER_EXIT * size["height"], 0), "landing": {"x": ladder["top"]["x"] - ladder["side"] * (size["width"] / 2 + WALLS.MANTLE_INSET), "y": ladder["top"]["y"] - LADDER_TUCK}}


def placement(vector):
    """🏗️ The ladder of a committed placement with its measures, ``None`` when there is none, and the reason."""
    label = "placements/%s" % vector["id"]
    ladder, verdict = ladder_for(label, vector["low"], vector["high"], vector["pitch"], vector["keepouts"], vector["size"])
    return (None if ladder is None else dict(measures(label, ladder, vector["size"]), ladder=ladder)), verdict


def lean(vector):
    """🪤️ The ladder of a committed lean against a wall with the feet of a climber at its exit, ``None`` when there is none, and the reason; refused unless ``numpy.hypot`` finds the exit as far from the foot as the length less the exit."""
    label = "leans/%s" % vector["id"]
    ladder, verdict = ladder_to(label, vector["low"], vector["pitch"], vector["keepouts"], vector["size"])
    if ladder is None:
        return None, verdict
    travel = max(length_of(ladder) - LADDER_EXIT * vector["size"]["height"], 0)
    exit_at = ladder_at(ladder, travel)
    WALLS.confirmed("%s: the exit" % label, [travel], [float(numpy.hypot(exit_at[0] - ladder["foot"]["x"], exit_at[1] - ladder["foot"]["y"]))])
    return {"ladder": ladder, "exit": {"x": exit_at[0], "y": exit_at[1]}}, verdict


# endregion 🔖️Placement


# region 🔖️Climbing
def ladder_step(travel, goal, ticks):
    """🐛️ One stated tick of climbing a ladder: up at the rise, down at the descent, gathered over the ramp."""
    return WALLS.stride_to(travel, goal, (LADDER_RISE if goal > travel else LADDER_DESCENT) * WALLS.smoothstep((ticks + 1) / LADDER_RAMP))


def ladder_at(ladder, travel):
    """📍️ The stated feet of a climber after a distance along the rails."""
    length = length_of(ladder)
    if not travel > 0:
        return [ladder["foot"]["x"], ladder["foot"]["y"]]
    if travel >= length:
        return [ladder["top"]["x"], ladder["top"]["y"]]
    share = travel / length
    return [ladder["foot"]["x"] + (ladder["top"]["x"] - ladder["foot"]["x"]) * share, ladder["foot"]["y"] + (ladder["top"]["y"] - ladder["foot"]["y"]) * share]


def climb(vector):
    """🧮️ The distance, the feet and the phase of the clip after every stated tick of a climb between the foot of a ladder and its exit, and its ticks; refused unless ``numpy.cumsum`` of the eased speeds reaches the distances, ``numpy.hypot`` finds every place as far from the foot, and ``numpy.cross`` finds it on the rails."""
    label = "climbs/%s" % vector["id"]
    ladder, size = vector["ladder"], vector["size"]
    exit_at = max(length_of(ladder) - LADDER_EXIT * size["height"], 0)
    start, goal = (exit_at, 0) if vector["down"] else (0, exit_at)
    travels, travel = [], start
    while travel != goal and len(travels) < PATIENCE:
        travel = ladder_step(travel, goal, len(travels))
        travels.append(travel)
    speed = LADDER_DESCENT if vector["down"] else LADDER_RISE
    covered = numpy.cumsum(speed * WALLS.eased(numpy.arange(1, len(travels) + 1) / LADDER_RAMP) / TICKS_PER_SECOND)
    if covered.size and (float(numpy.abs(covered - exit_at).min()) < MARGIN or int(numpy.argmax(covered >= exit_at)) + 1 != len(travels)):
        raise AssertionError("%s: the climb arrives on the edge of two tick counts, or not on the tick the sum of its speeds says" % label)
    WALLS.confirmed("%s: the distances" % label, travels, start + (-1 if vector["down"] else 1) * numpy.minimum(covered, exit_at))
    feet = [ladder_at(ladder, travel) for travel in travels]
    foot = numpy.array([ladder["foot"]["x"], ladder["foot"]["y"]])
    rails = numpy.array([ladder["top"]["x"], ladder["top"]["y"], 0.0]) - numpy.append(foot, 0.0)
    offsets = numpy.asarray(feet, dtype=float).reshape(-1, 2) - foot
    WALLS.confirmed("%s: how far the feet are from the foot" % label, travels, numpy.hypot(offsets[:, 0], offsets[:, 1]))
    WALLS.confirmed("%s: the feet on the rails" % label, numpy.zeros(len(feet)), numpy.cross(numpy.tile(rails, (len(feet), 1)), numpy.column_stack([offsets, numpy.zeros(len(feet))]))[:, 2] / max(float(numpy.hypot(rails[0], rails[1])), 1.0))
    phases = [travel / (2 * RUNG_SPACING) - math.floor(travel / (2 * RUNG_SPACING)) for travel in travels]
    if len(phases) and float(numpy.abs((numpy.asarray(phases) - numpy.mod(numpy.asarray(travels, dtype=float) / (2 * RUNG_SPACING), 1.0) + 0.5) % 1.0 - 0.5).max()) > STEPPING:
        raise AssertionError("%s: the phases leave numpy.mod of the distance climbed" % label)
    return {"ticks": len(travels), "travels": travels, "feet": feet, "phases": phases}


# endregion 🔖️Climbing


# region 🔖️Surveys
def ladder_holds(label, ladder, perches, pitches, keepouts, size):
    """🩺️ The ladder as it stands after a survey: its foot keeps its x on a perch of its surface that moved no more than the shift, its top follows a pitch of its wall — touching it under the rim or at its lower end, as ladders lean —, no farther than ``numpy.hypot`` allows; ``None`` when it topples. A ladder leans on its wall, not on what lies on top of it."""
    for low in perches:
        if low["surface"] != ladder["surface"] or abs(low["y"] - ladder["foot"]["y"]) > LADDER_SHIFT:
            continue
        for pitch in pitches:
            if pitch["wall"] != ladder["wall"] or pitch["side"] != ladder["side"]:
                continue
            for top in (pitch["y0"] + LADDER_TUCK, pitch["y1"]):
                after, _verdict = stood(label, low, pitch, ladder["foot"]["x"], top, keepouts, size)
                if after is None:
                    continue
                moved = float(numpy.hypot(after["top"]["x"] - ladder["top"]["x"], after["top"]["y"] - ladder["top"]["y"]))
                if abs(moved - LADDER_FOLLOW) < MARGIN and moved != LADDER_FOLLOW:
                    raise AssertionError("%s: the top moved by %r, on the edge of what a ladder follows" % (label, moved))
                if moved <= LADDER_FOLLOW:
                    return after
    return None


def survey(vector):
    """🌋️ What a committed survey leaves of a ladder."""
    return ladder_holds("surveys/%s" % vector["id"], vector["ladder"], vector["perches"], vector["pitches"], vector["keepouts"], vector["size"])


def spill(vector):
    """🎳️ What a toppling ladder does to a climber at each committed height: ``None`` where it steps off, the throw away from the wall where it falls."""
    ladder, size = vector["ladder"], vector["size"]
    return [None if ladder["foot"]["y"] - height < TOPPLE_STEP * size["height"] else {"vx": ladder["side"] * TOPPLE_PUSH, "vy": 0} for height in vector["heights"]]


# endregion 🔖️Surveys


# region 🔖️Handlers
def committed(ctx):
    """🧫️ The committed vectors."""
    return json.loads(ctx.fixture_bytes(VECTORS))


def constants(ctx):
    """🎚️ The tuning constants the vectors were generated with; they must be the ones of this reference."""
    document = committed(ctx)
    if document["constants"] != CONSTANTS:
        raise AssertionError("constants: the reference is tuned to %r, the committed vectors to %r" % (CONSTANTS, document["constants"]))
    return Outcome(document["constants"])


def placements(ctx):
    """📐️ The ladder of every committed placement, ``None`` where none stands — for the committed reason."""
    produced = {}
    vectors = committed(ctx)["placements"]
    for vector in vectors:
        produced[vector["id"]], verdict = placement(vector)
        if verdict != vector["verdict"]:
            raise AssertionError("placements/%s: the reference says %s, the committed vector says %s" % (vector["id"], verdict, vector["verdict"]))
    return WALLS.agree("placements", produced, vectors)


def leans(ctx):
    """🧷️ The ladder of every committed lean against a wall with the feet at its exit, ``None`` where none stands — for the committed reason."""
    produced = {}
    vectors = committed(ctx)["leans"]
    for vector in vectors:
        produced[vector["id"]], verdict = lean(vector)
        if verdict != vector["verdict"]:
            raise AssertionError("leans/%s: the reference says %s, the committed vector says %s" % (vector["id"], verdict, vector["verdict"]))
    return WALLS.agree("leans", produced, vectors)


def climbs(ctx):
    """🧗️ Every committed climb, tick by tick."""
    vectors = committed(ctx)["climbs"]
    return WALLS.agree("climbs", {vector["id"]: climb(vector) for vector in vectors}, vectors)


def surveys(ctx):
    """🧐️ What every committed survey leaves of its ladder."""
    vectors = committed(ctx)["surveys"]
    return WALLS.agree("surveys", {vector["id"]: survey(vector) for vector in vectors}, vectors)


def spills(ctx):
    """🤸️ What every committed toppling ladder does to its climber."""
    vectors = committed(ctx)["spills"]
    return WALLS.agree("spills", {vector["id"]: spill(vector) for vector in vectors}, vectors)


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter():
    """🔮️ Oracle role only: numpy is the reference, the TypeScript and Rust twins are judged against it."""
    return Adapter("python").oracle("constants", constants).oracle("placements", placements).oracle("leans", leans).oracle("climbs", climbs).oracle("surveys", surveys).oracle("spills", spills)


# endregion 🔖️Registration

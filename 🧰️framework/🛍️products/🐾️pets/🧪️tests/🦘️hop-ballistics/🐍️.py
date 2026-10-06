#!/usr/bin/env python3
"""🦘️ Oracle of the pets product's falls, landings and hops (design §4.5), in Python.

The subjects integrate flight tick by tick with semi-implicit Euler (``dt = 1/64`` s, the velocity first).
Below the terminal speed those ticks are samples of one continuous ballistic arc: the arc under the same
gravity whose launch velocity is half a tick's gain larger, ``vy + GRAVITY × dt ÷ 2``.
``scipy.integrate.solve_ivp`` integrates that arc and is read at the tick times; at the terminal speed
the fall continues as a straight line, and ``numpy.cumsum`` of the clamped speeds must reach the same
heights. A hop is derived from its definition only: the apex rise from the clearance and the steepness,
the flight time as the positive root of the continuous arc with that apex (``numpy.roots``), rounded to
whole ticks, and the launch velocity as the root (``scipy.optimize.brentq``) of the height the
integrated arc misses the target by after those ticks.

Every number the oracle answers is judged by those libraries within 1e-9 — and answered as the plain
binary64 value of the stated tick, because the harness compares on a decimal grid of 1e-9 on which the
exact binary fractions of a flight (ten fractional bits) are ties: a number that is right to 1e-13 would
round to the other side. The stated ticks are written here from the design text; they are refused
unless the integration, the roots and the sums confirm them.

Landings come from sweeps that are not the subjects' and use the integrated flight alone: the distinct
perch heights are walked from the top (``numpy.unique``), and a flight is laid against every perch at
once in one boolean matrix of ticks × perches.

@see https://docs.scipy.org/doc/scipy/reference/generated/scipy.integrate.solve_ivp.html
@see https://docs.scipy.org/doc/scipy/reference/generated/scipy.optimize.brentq.html
@see https://gafferongames.com/post/integration_basics/
@see ../../🧫️fixtures/🦘️hop-ballistics/🔣️.json
"""

# region 🔖️Imports
import json
import math

import numpy
import scipy.integrate
import scipy.optimize

from semio_repo_test import Adapter, Outcome

# endregion 🔖️Imports


# region 🔖️Reference
VECTORS = "shared://🦘️hop-ballistics/🔣️.json"
TICKS_PER_SECOND = 64
GRAVITY = 1800.0
FALL_SPEED = 900.0
HOP_CLEARANCE = 12.0
HOP_STEEPNESS = 0.375
HOP_HEIGHT = 84.0
HOP_DISTANCE = 160.0
HOP_TICKS = 48
CONSTANTS = {"gravity": GRAVITY, "fallSpeed": FALL_SPEED, "hopClearance": HOP_CLEARANCE, "hopSteepness": HOP_STEEPNESS, "hopHeight": HOP_HEIGHT, "hopDistance": HOP_DISTANCE, "hopTicks": HOP_TICKS}
AGREEMENT = 1e-12
STEPPING = 1e-9
MARGIN = 1e-6


def confirmed(label, stated, judged):
    """🤝️ Refuses stated numbers the libraries do not reach within 1e-9."""
    stated, judged = numpy.asarray(stated, dtype=float), numpy.asarray(judged, dtype=float)
    if stated.shape != judged.shape or (stated.size and float(numpy.abs(stated - judged).max()) > STEPPING):
        raise AssertionError("%s: the stated ticks give %r, the libraries %r" % (label, stated.tolist(), judged.tolist()))


def arc(x, y, vx, vy, ticks):
    """🌠️ The feet after each of ``ticks`` ticks of free flight below the terminal speed: the continuous arc launched with ``vy + GRAVITY ÷ 128``, integrated by scipy and read at the tick times."""
    if ticks == 0:
        return numpy.zeros((0, 2))
    times = numpy.arange(1, ticks + 1) / TICKS_PER_SECOND
    solution = scipy.integrate.solve_ivp(lambda _time, state: [state[2], state[3], 0.0, GRAVITY], (0.0, float(times[-1])), [x, y, vx, vy + GRAVITY / TICKS_PER_SECOND / 2], method="DOP853", t_eval=times, rtol=1e-12, atol=1e-12)
    if not solution.success:
        raise AssertionError("solve_ivp failed: %s" % solution.message)
    return numpy.stack([solution.y[0], solution.y[1]], axis=1)


def integrated(y, vy, ticks):
    """🪂️ The heights and speeds after each of ``ticks`` ticks of falling without a single step: the integrated arc until the speed reaches the terminal speed, a straight line from then on; the sum of the clamped speeds must reach the same heights."""
    speeds = numpy.minimum(vy + numpy.arange(1, ticks + 1) * (GRAVITY / TICKS_PER_SECOND), FALL_SPEED)
    free = int(numpy.count_nonzero(speeds < FALL_SPEED))
    risen = arc(0.0, y, 0.0, vy, free)[:, 1]
    last = float(risen[-1]) if free else y
    heights = numpy.concatenate([risen, last + numpy.arange(1, ticks - free + 1) * (FALL_SPEED / TICKS_PER_SECOND)])
    confirmed("the integrated fall against the sum of its speeds", y + numpy.cumsum(speeds) / TICKS_PER_SECOND, heights)
    return heights, speeds


def fall_step(y, vy):
    """⏬️ One stated tick of falling: the velocity first, clamped to the terminal speed, then the height with the new velocity."""
    speed = min(vy + GRAVITY / TICKS_PER_SECOND, FALL_SPEED)
    return y + speed / TICKS_PER_SECOND, speed


def fall(y, vy, ticks):
    """🍃️ The heights and speeds after each of ``ticks`` stated ticks, confirmed by the integrated fall."""
    judged_heights, judged_speeds = integrated(y, vy, ticks)
    heights, speeds = [], []
    for _ in range(ticks):
        y, vy = fall_step(y, vy)
        heights.append(y)
        speeds.append(vy)
    confirmed("the heights of a fall", heights, judged_heights)
    confirmed("the speeds of a fall", speeds, judged_speeds)
    return heights, speeds


def landing(perches, x, from_y, to_y):
    """🛬️ The index of the highest perch crossed at ``x`` between two heights, found by walking the distinct perch heights from the top; ``None`` when none is crossed."""
    levels = numpy.unique(numpy.array([perch["y"] for perch in perches], dtype=float))
    for level in levels[(levels >= from_y) & (levels <= to_y)]:
        for index, perch in enumerate(perches):
            if perch["y"] == level and perch["x0"] <= x <= perch["x1"]:
                return index
    return None


def crossings(perches, xs, heights, aside, above):
    """🧱️ Which perch every tick of a flight crosses, as a boolean matrix of ticks × perches; the slack per sample (``aside`` for its x, ``above`` for its height) widens every comparison that sample takes part in, or narrows it when negative."""
    x0, x1, level = (numpy.array([perch[key] for perch in perches], dtype=float) for key in ("x0", "x1", "y"))
    inside = (x0[None, :] - aside[1:, None] <= xs[1:, None]) & (xs[1:, None] <= x1[None, :] + aside[1:, None])
    return inside & (heights[:-1, None] - above[:-1, None] <= level[None, :]) & (level[None, :] <= heights[1:, None] + above[1:, None])


def touchdown(label, perches, xs, heights, steered, flown):
    """🛩️ The first tick of a flight that crosses a perch and the index of the highest perch crossed then; ``(None, None)`` when the flight crosses nothing.

    ``xs`` and ``heights`` start with the launch. ``steered`` and ``flown`` mark the x and the heights that were integrated rather than given: no crossing may hang on one of them by less than the margin, so a vector whose landing rounding could move is refused."""
    if not perches:
        return None, None
    still = numpy.zeros(heights.size)
    crossed = crossings(perches, xs, heights, still, still)
    if not numpy.array_equal(crossings(perches, xs, heights, steered * MARGIN, flown * MARGIN), crossings(perches, xs, heights, steered * -MARGIN, flown * -MARGIN)):
        raise AssertionError("%s: a computed sample lies within %r of a perch height or end it has to clear, so the landing is not robust" % (label, MARGIN))
    ticks = numpy.flatnonzero(crossed.any(axis=1))
    if ticks.size == 0:
        return None, None
    tick = int(ticks[0])
    level = numpy.array([perch["y"] for perch in perches], dtype=float)
    return tick + 1, int(numpy.argmin(numpy.where(crossed[tick], level, numpy.inf)))


def drop(label, perches, start, limit):
    """⬇️ A fall from ``start`` until it lands or ``limit`` ticks have passed: the perch it lands on, the ticks it took and the height it ends at — the height of the perch, or of the last stated tick when nothing was met."""
    heights = numpy.concatenate([[start["y"]], integrated(start["y"], start["vy"], limit)[0]])
    tick, index = touchdown(label, perches, numpy.full(heights.size, float(start["x"])), heights, numpy.zeros(heights.size, dtype=bool), numpy.arange(heights.size) > 0)
    if tick is None:
        return {"perch": None, "ticks": limit, "y": fall(start["y"], start["vy"], limit)[0][-1] if limit else start["y"]}
    return {"perch": index, "ticks": tick, "y": perches[index]["y"]}


def hop(start, end):
    """🚀️ The hop between two points and why there is none: ``(hop, verdict)`` with the verdict ``granted``, ``too-high``, ``too-far``, ``too-long`` or ``too-fast``.

    The duration is the positive root of the continuous arc with the stated apex, the launch velocity the root of the height the integrated arc misses the target by; the stated closed forms must reach both."""
    dx, dy = end["x"] - start["x"], end["y"] - start["y"]
    rise = max(HOP_CLEARANCE - min(dy, 0.0), abs(dx) * HOP_STEEPNESS)
    if rise > HOP_HEIGHT:
        return None, "too-high"
    if abs(dx) > HOP_DISTANCE:
        return None, "too-far"
    roots = numpy.roots([GRAVITY / 2, -math.sqrt(2 * GRAVITY * rise), -dy])
    edge = float(max(root.real for root in roots if abs(root.imag) < STEPPING)) * TICKS_PER_SECOND + 0.5
    if abs(edge - round(edge)) < MARGIN:
        raise AssertionError("the flight of %r → %r lasts %r ticks, on the edge of two tick counts" % (start, end, edge - 0.5))
    ticks = math.floor(edge)
    stated = math.floor((math.sqrt(2 * rise / GRAVITY) + math.sqrt(2 * (rise + dy) / GRAVITY)) * TICKS_PER_SECOND + 0.5)
    if stated != ticks:
        raise AssertionError("the flight of %r → %r lasts %d ticks by its root and %d as stated" % (start, end, ticks, stated))
    if ticks > HOP_TICKS:
        return None, "too-long"
    vx, vy = dx * TICKS_PER_SECOND / ticks, (dy - GRAVITY * ticks * (ticks + 1) / (2 * TICKS_PER_SECOND * TICKS_PER_SECOND)) * TICKS_PER_SECOND / ticks
    confirmed("the launch velocity of %r → %r" % (start, end), [vy], [scipy.optimize.brentq(lambda speed: float(arc(0.0, 0.0, 0.0, speed, ticks)[-1, 1]) - dy, -4 * FALL_SPEED, 4 * FALL_SPEED, xtol=1e-13, rtol=1e-15)])
    landed = vy + ticks * (GRAVITY / TICKS_PER_SECOND)
    if abs(landed - FALL_SPEED) < MARGIN:
        raise AssertionError("the hop %r → %r lands at %r px/s, on the edge of the terminal speed" % (start, end, landed))
    if landed > FALL_SPEED:
        return None, "too-fast"
    return {"vx": vx, "vy": vy, "ticks": ticks}, "granted"


def flight(start, end):
    """🕊️ The feet of a granted hop after every stated tick, the last tick on the target itself, and its apex; refused unless the integrated arc reaches the same feet and ends on the target, and the flight rises first and tops both ends."""
    launch, verdict = hop(start, end)
    if launch is None:
        raise AssertionError("%r → %r is out of reach (%s) and has no flight" % (start, end, verdict))
    feet = arc(start["x"], start["y"], launch["vx"], launch["vy"], launch["ticks"])
    confirmed("the end of the arc %r → %r" % (start, end), [end["x"], end["y"]], feet[-1])
    path, x, y, vy = [], start["x"], start["y"], launch["vy"]
    for _ in range(launch["ticks"] - 1):
        y, vy = fall_step(y, vy)
        x = x + launch["vx"] / TICKS_PER_SECOND
        path.append([x, y])
    confirmed("the arc %r → %r" % (start, end), path, feet[:-1])
    path.append([end["x"], end["y"]])
    apex = min(y for _x, y in path)
    if not (path[0][1] < start["y"] and apex < min(start["y"], end["y"]) - HOP_CLEARANCE / 2 and path[-2][1] < end["y"]):
        raise AssertionError("the arc of %r → %r does not rise above both of its ends" % (start, end))
    return {"ticks": launch["ticks"], "path": path, "apex": apex}


def route(vector):
    """🗺️ The index of the perch a granted hop really ends on: the first perch its integrated flight crosses on the way down, ``None`` when it crosses none."""
    launch, verdict = hop(vector["from"], vector["to"])
    if launch is None:
        raise AssertionError("routes/%s is out of reach (%s)" % (vector["id"], verdict))
    feet = arc(vector["from"]["x"], vector["from"]["y"], launch["vx"], launch["vy"], launch["ticks"])[:-1]
    xs = numpy.concatenate([[vector["from"]["x"]], feet[:, 0], [vector["to"]["x"]]])
    heights = numpy.concatenate([[vector["from"]["y"]], feet[:, 1], [vector["to"]["y"]]])
    ticks = numpy.arange(heights.size)
    arced = (ticks > 0) & (ticks < heights.size - 1)
    _tick, index = touchdown("routes/%s" % vector["id"], vector["perches"], xs, heights, arced, arced)
    return index


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


def constants(ctx):
    """🎚️ The tuning constants the vectors were generated with; they must be the ones of this reference."""
    document = committed(ctx)
    if document["constants"] != CONSTANTS:
        raise AssertionError("constants: the reference is tuned to %r, the committed vectors to %r" % (CONSTANTS, document["constants"]))
    return Outcome(document["constants"])


def falls(ctx):
    """🍂️ Every committed fall, tick by tick."""
    produced = {}
    vectors = committed(ctx)["falls"]
    for vector in vectors:
        heights, speeds = fall(vector["y"], vector["vy"], vector["ticks"])
        produced[vector["id"]] = {"heights": heights, "speeds": speeds}
    return agree("falls", produced, vectors)


def landings(ctx):
    """🪜️ The perch crossed by every committed sweep."""
    vectors = committed(ctx)["landings"]
    return agree("landings", {vector["id"]: [landing(vector["perches"], sweep["x"], sweep["fromY"], sweep["toY"]) for sweep in vector["sweeps"]] for vector in vectors}, vectors)


def drops(ctx):
    """🪨️ Every committed fall onto perches."""
    vectors = committed(ctx)["drops"]
    return agree("drops", {vector["id"]: drop("drops/%s" % vector["id"], vector["perches"], vector["start"], vector["limit"]) for vector in vectors}, vectors)


def hops(ctx):
    """🦗️ The hop of every committed pair of points, ``None`` when out of reach — for the committed reason."""
    produced = {}
    vectors = committed(ctx)["hops"]
    for vector in vectors:
        produced[vector["id"]], verdict = hop(vector["from"], vector["to"])
        if verdict != vector["verdict"]:
            raise AssertionError("hops/%s: the reference says %s, the committed vector says %s" % (vector["id"], verdict, vector["verdict"]))
    return agree("hops", produced, vectors)


def flights(ctx):
    """🎈️ The flight of every committed granted hop."""
    vectors = committed(ctx)["flights"]
    return agree("flights", {vector["id"]: flight(vector["from"], vector["to"]) for vector in vectors}, vectors)


def routes(ctx):
    """🎯️ The perch every committed hop really ends on."""
    vectors = committed(ctx)["routes"]
    return agree("routes", {vector["id"]: route(vector) for vector in vectors}, vectors)


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter():
    """🧭️ Oracle role only: scipy is the reference, the TypeScript and Rust twins are judged against it."""
    return Adapter("python").oracle("constants", constants).oracle("falls", falls).oracle("landings", landings).oracle("drops", drops).oracle("hops", hops).oracle("flights", flights).oracle("routes", routes)


# endregion 🔖️Registration

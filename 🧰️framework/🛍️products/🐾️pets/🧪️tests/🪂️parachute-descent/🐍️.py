#!/usr/bin/env python3
"""🪂️ Oracle of the parachute of the pets product (MECH §2): when it opens, how a canopy descends, steers, sways and flares, and how hard a pet lands with and without it, in Python.

The predicted impact speed is the closed form of a fall, ``sqrt(vy² + 2·g·H)`` held at the terminal speed,
which ``scipy.integrate.solve_ivp`` reaches with an event at the landing. Under an open canopy the descent
is a first-order lag towards the terminal speed, ``v' = −(v − vt) ÷ τ`` with ``τ = 0.18`` s: its tick
samples are ``vt + (v0 − vt)·Fᵏ`` with ``F = exp(−1 ÷ (64·τ))`` exactly (``numpy.exp``, ``numpy.power``,
and ``solve_ivp`` on the equation itself), the heights their running sum (``numpy.cumsum``). The sideways
steering is linear while it is not at its limit (``numpy.linalg.matrix_power`` of its one-tick matrix)
and a lag towards the limit while it is (``numpy.power``). The pet below the canopy is the pendulum of
case 🪢️swing-dynamics with a pivot in uniform motion, integrated by ``solve_ivp``; the wind is
``numpy.sin``; the flare is ``numpy.interp``.

A whole drop — fall, decision, reflex, canopy, flare, touch — is integrated once more as one continuous
system with events (the fall as the arc whose tick samples the semi-implicit steps are, the descent as
the lag with the flare), and the restated drop must land within a tick of it and at its speed within
the stated tolerance. As everywhere in this owner the oracle projects its restatement of the subjects'
arithmetic once the libraries have confirmed it, because the comparison grid of 1e-9 is finer than the
agreement between a tick-wise step and a differential equation.

@see https://docs.scipy.org/doc/scipy/reference/generated/scipy.integrate.solve_ivp.html
@see https://numpy.org/doc/stable/reference/generated/numpy.interp.html
@see ../🪢️swing-dynamics/🐍️.py
@see ../📐️turn-trigonometry/🐍️.py
@see ../../🧫️fixtures/🪂️parachute-descent/🔣️.json
"""

# region 🔖️Imports
import importlib.util
import json
import math
import os

import numpy
import scipy.integrate

from semio_repo_test import Adapter, Outcome

# endregion 🔖️Imports


# region 🔖️Constants
VECTORS = "shared://🪂️parachute-descent/🔣️.json"
TICK = 1.0 / 64.0
GRAVITY = 1800.0
FALL_SPEED = 900.0
HARD_LANDING = 600.0
CHUTE_OPENING = 240.0
CHUTE_HEADROOM = 56.0
CHUTE_REFLEX = 6
CHUTE_FACTOR = 0.9168553557320289
CHUTE_DESCENT = 2.0
CHUTE_STEER_GAIN = 1.2
CHUTE_STEER_SPEED = 1.2
CHUTE_STEER_EASE = 0.06
CHUTE_ROD = 0.9
CHUTE_GRAVITY = 900.0
CHUTE_DAMPING = 0.97
CHUTE_FLARE = 0.25
CHUTE_WIND = 10.0
CHUTE_WIND_RATE = 0.35
LAG_SECONDS = 0.18
CONSTANTS = {
    "gravity": GRAVITY,
    "fallSpeed": FALL_SPEED,
    "hardLanding": HARD_LANDING,
    "chuteOpening": CHUTE_OPENING,
    "chuteHeadroom": CHUTE_HEADROOM,
    "chuteReflex": CHUTE_REFLEX,
    "chuteFactor": CHUTE_FACTOR,
    "chuteDescent": CHUTE_DESCENT,
    "chuteSteerGain": CHUTE_STEER_GAIN,
    "chuteSteerSpeed": CHUTE_STEER_SPEED,
    "chuteSteerEase": CHUTE_STEER_EASE,
    "chuteRod": CHUTE_ROD,
    "chuteGravity": CHUTE_GRAVITY,
    "chuteDamping": CHUTE_DAMPING,
    "chuteFlare": CHUTE_FLARE,
    "chuteWind": CHUTE_WIND,
    "chuteWindRate": CHUTE_WIND_RATE,
}
AGREEMENT = 1e-12
STEPPING = 1e-9
MARGIN = 1e-6
# endregion 🔖️Constants


# region 🔖️Restatement
def neighbour(case):
    """🚪️ The oracle adapter of another case of this owner, loaded from its file."""
    spec = importlib.util.spec_from_file_location("pets_" + case.encode("ascii", "ignore").decode("ascii").replace("-", "_"), os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", case, "🐍️.py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


SWING = neighbour("🪢️swing-dynamics")
TRIGONOMETRY = SWING.TRIGONOMETRY


def impact_speed(vy, height):
    """☄️ ``impactSpeed`` restated: the root of ``vy² + 2·g·height`` with a height that is not below counted as 0, held at the terminal speed."""
    speed = math.sqrt(vy * vy + 2 * GRAVITY * (height if height > 0 else 0))
    return speed if speed < FALL_SPEED else FALL_SPEED


def chute_opens(vy, height):
    """🎟️ ``chuteOpens`` restated: fast enough, high enough, and a hard landing ahead."""
    return vy >= CHUTE_OPENING and height >= CHUTE_HEADROOM and impact_speed(vy, height) > HARD_LANDING


def chute_of(height):
    """🎒️ ``chuteOf`` restated: the four measures as shares of the body height."""
    return {"terminal": CHUTE_DESCENT * height, "reach": CHUTE_STEER_SPEED * height, "flare": CHUTE_FLARE * height, "length": CHUTE_ROD * height}


def canopy_of(feet, vx, vy, chute):
    """🌂️ ``canopyOf`` restated: the canopy the cords' length above the feet, everything moving as the pet does."""
    return {"x": float(feet["x"]), "y": feet["y"] - chute["length"], "vx": float(vx), "vy": float(vy), "bob": SWING.point(feet["x"], feet["y"]), "previous": SWING.point(feet["x"] - vx / 64, feet["y"] - vy / 64)}


def flare_of(remaining, flare):
    """🦅️ ``flareOf`` restated: all of the speed at the flare height and above, half at the touch and below, evenly in between."""
    if remaining >= flare:
        return 1
    if remaining <= 0:
        return 0.5
    return 0.5 + (0.5 * remaining) / flare


def chute_wind(ticks, phase):
    """🌬️ ``chuteWind`` restated: the restated sine of the wind's turns."""
    return CHUTE_WIND * TRIGONOMETRY.sin_turns((CHUTE_WIND_RATE * ticks) / 64 + phase)


def chute_step(canopy, chute, target, remaining, wind):
    """🕊️ ``chuteStep`` restated: the lag of the descent, the eased steering, the move of the canopy with wind and flare, the swing of the pet below."""
    vy = chute["terminal"] + (canopy["vy"] - chute["terminal"]) * CHUTE_FACTOR
    pull = CHUTE_STEER_GAIN * (target - canopy["x"])
    wanted = 0 - chute["reach"] if pull < 0 - chute["reach"] else chute["reach"] if pull > chute["reach"] else pull
    vx = canopy["vx"] + (wanted - canopy["vx"]) * CHUTE_STEER_EASE
    x = canopy["x"] + (vx + wind) / 64
    y = canopy["y"] + (vy * flare_of(remaining, chute["flare"])) / 64
    return {"x": x, "y": y, "vx": vx, "vy": vy, "bob": SWING.swing_step(canopy, {"x": x, "y": y}, canopy["bob"], canopy["previous"], chute["length"], CHUTE_GRAVITY, CHUTE_DAMPING, False), "previous": canopy["bob"]}


def fall_step(y, vy):
    """🍂️ ``fallStep`` of the terrain restated: the speed first, held at the terminal speed, then the height with the new speed."""
    speed = min(vy + GRAVITY / 64, FALL_SPEED)
    return y + speed / 64, speed


def dropped(vector):
    """🪨️ A whole drop restated, tick by tick as the stage takes it: fall and ask ``chuteOpens``; ``CHUTE_REFLEX`` ticks after the decision open the canopy and descend under it; stop at the tick the feet reach the landing."""
    chute = chute_of(vector["height"])
    y, vy, opened, canopy, tick, touch, track = 0.0, float(vector["vy"]), 0, None, 0, 0.0, []
    while True:
        tick += 1
        remaining = vector["drop"] - y
        if canopy is None:
            if opened == 0 and vector["chute"] and chute_opens(vy, remaining):
                opened = tick
            if opened != 0 and tick - opened >= CHUTE_REFLEX:
                canopy = canopy_of(SWING.point(0, y), 0, vy, chute)
        if canopy is None:
            y, vy = fall_step(y, vy)
            touch = vy
        else:
            moved = chute_step(canopy, chute, 0, remaining, 0)
            touch = (moved["y"] - canopy["y"]) * 64
            canopy, y, vy = moved, moved["bob"]["y"], moved["vy"]
        track.append((y, vy, canopy is not None))
        if y >= vector["drop"]:
            return {"opened": opened, "landed": tick, "touch": touch, "glide": vy, "plain": impact_speed(vector["vy"], vector["drop"])}, track


# endregion 🔖️Restatement


# region 🔖️Reference
def struck(vy, height):
    """🎯️ The speed at which a fall from ``vy`` reaches a landing ``height`` below, by ``solve_ivp`` with an event at the landing, held at the terminal speed."""
    if height <= 0:
        return min(abs(float(vy)), FALL_SPEED)
    landing = lambda _time, state: state[0] - height
    landing.terminal, landing.direction = True, 1.0
    solution = scipy.integrate.solve_ivp(lambda _time, state: [state[1], GRAVITY], (0.0, 60.0), [0.0, float(vy)], method="DOP853", events=landing, rtol=1e-12, atol=1e-12)
    if not solution.success or solution.t_events[0].size != 1:
        raise AssertionError("solve_ivp did not find the landing of a fall of %r px from %r px/s" % (height, vy))
    return min(float(solution.y_events[0][0][1]), FALL_SPEED)


def lagged(start, terminal, ticks):
    """📉️ The descent speed after each of ``ticks`` ticks under an open canopy: the closed form of the lag with ``numpy.exp``, which ``solve_ivp`` on ``v' = −(v − vt) ÷ τ`` must reach."""
    counts = numpy.arange(1, ticks + 1)
    closed = terminal + (start - terminal) * numpy.power(numpy.exp(-1.0 / (64.0 * LAG_SECONDS)), counts)
    solution = scipy.integrate.solve_ivp(lambda _time, speed: [-(speed[0] - terminal) / LAG_SECONDS], (0.0, ticks * TICK), [float(start)], method="DOP853", t_eval=counts * TICK, rtol=1e-12, atol=1e-12)
    if not solution.success or float(numpy.max(numpy.abs(solution.y[0] - closed))) > 1e-8 * max(1.0, abs(start)):
        raise AssertionError("the closed form of the lag from %r px/s leaves solve_ivp" % start)
    return closed


def steered(vector, chute, ticks):
    """🧭️ The sideways place and speed of a canopy after each of ``ticks`` ticks without wind: the matrix power of the steering step while it never reaches its limit, the lag towards the limit while it never leaves it; a committed descent must stay in one of the two."""
    offset, speed, gain, ease, reach = vector["feet"]["x"] - vector["target"], float(vector["vx"]), CHUTE_STEER_GAIN, CHUTE_STEER_EASE, chute["reach"]
    matrix = numpy.array([[1.0 - gain * ease / 64.0, (1.0 - ease) / 64.0], [-gain * ease, 1.0 - ease]])
    free = numpy.array([numpy.linalg.matrix_power(matrix, count) @ [offset, speed] for count in range(ticks + 1)])
    if float(numpy.max(numpy.abs(gain * free[:-1, 0]))) < reach - MARGIN:
        return vector["target"] + free[1:, 0], free[1:, 1]
    limit = reach if offset < 0 else -reach
    speeds = limit + (speed - limit) * numpy.power(1.0 - ease, numpy.arange(1, ticks + 1))
    places = vector["feet"]["x"] + numpy.cumsum(speeds) / 64.0
    if float(numpy.min(numpy.abs(gain * (numpy.concatenate([[vector["feet"]["x"]], places[:-1]]) - vector["target"])))) <= reach + MARGIN:
        raise AssertionError("canopy-descents/%s: the steering leaves its limit on the way, so neither closed form holds" % vector["id"])
    return places, speeds


def descended(vector):
    """🌥️ The restated states of a canopy at the committed ticks, each number confirmed: the descent speed by the lag, the height by the running sum of speed times flare, the sideways motion by its closed form (without wind), and the pet at the cords' length below the canopy."""
    chute = chute_of(vector["height"])
    canopy = canopy_of(vector["feet"], vector["vx"], vector["vy"], chute)
    ticks = max(vector["ticks"])
    states, flares, winds = [], [], []
    for tick in range(1, ticks + 1):
        remaining = vector["remaining"] - (canopy["bob"]["y"] - vector["feet"]["y"])
        wind = 0 if vector["phase"] is None else chute_wind(tick, vector["phase"])
        flares.append(float(numpy.interp(remaining, [0.0, chute["flare"]], [0.5, 1.0])))
        winds.append(wind)
        canopy = chute_step(canopy, chute, vector["target"], remaining, wind)
        states.append({"x": canopy["x"], "y": canopy["y"], "vx": canopy["vx"], "vy": canopy["vy"], "bob": canopy["bob"]})
    speeds = lagged(vector["vy"], chute["terminal"], ticks)
    SWING.confirmed("canopy-descents/%s: the descent speeds" % vector["id"], [state["vy"] for state in states], speeds)
    SWING.confirmed("canopy-descents/%s: the heights" % vector["id"], [state["y"] for state in states], vector["feet"]["y"] - chute["length"] + numpy.cumsum(speeds * flares) / 64.0)
    if vector["phase"] is None:
        places, drifts = steered(vector, chute, ticks)
        SWING.confirmed("canopy-descents/%s: the sideways places" % vector["id"], [state["x"] for state in states], places)
        SWING.confirmed("canopy-descents/%s: the sideways speeds" % vector["id"], [state["vx"] for state in states], drifts)
    else:
        blown = numpy.sin(2.0 * numpy.pi * (CHUTE_WIND_RATE * numpy.arange(1, ticks + 1) / 64.0 + vector["phase"])) * CHUTE_WIND
        SWING.confirmed("canopy-descents/%s: the wind" % vector["id"], winds, blown)
        SWING.confirmed("canopy-descents/%s: the sideways places" % vector["id"], [state["x"] for state in states], vector["feet"]["x"] + numpy.cumsum(numpy.array([state["vx"] for state in states]) + blown) / 64.0)
    cords = numpy.linalg.norm([[state["bob"]["x"] - state["x"], state["bob"]["y"] - state["y"]] for state in states], axis=1)
    if float(numpy.max(numpy.abs(cords - chute["length"]))) > STEPPING * chute["length"]:
        raise AssertionError("canopy-descents/%s: the pet leaves the length of its cords: %r" % (vector["id"], cords.tolist()))
    return [states[tick - 1] for tick in vector["ticks"]]


def sway_start(vector):
    """🌱️ Where a committed swaying pet is at tick 0 and one tick before it, read off the integrated pendulum below a canopy in uniform motion."""
    chute = chute_of(vector["height"])
    gravity, drag = SWING.felt(CHUTE_GRAVITY, CHUTE_DAMPING)
    pivot = lambda _time: (float(vector["vx"]), float(chute["terminal"]), 0.0, 0.0)
    before = SWING.pendulum(math.radians(vector["angle"]), 0.0, gravity, drag, lambda _time: (chute["length"], 0.0), pivot, [-TICK])[0]
    top = SWING.point(vector["canopy"]["x"], vector["canopy"]["y"])
    earlier = SWING.point(top["x"] - vector["vx"] * TICK, top["y"] - chute["terminal"] * TICK)
    return SWING.hung(top, chute["length"], math.radians(vector["angle"])), SWING.hung(earlier, chute["length"], before)


def swayed(vector):
    """🎐️ The restated feet of a pet below a canopy in uniform motion at the committed ticks, and how far its sway leaves the integrated pendulum with that moving pivot at most, in degrees."""
    chute = chute_of(vector["height"])
    bob, previous = sway_start(vector)
    SWING.confirmed("canopy-sways/%s: the committed start" % vector["id"], [vector["bob"]["x"], vector["bob"]["y"], vector["previous"]["x"], vector["previous"]["y"]], [bob["x"], bob["y"], previous["x"], previous["y"]])
    canopy = {"x": float(vector["canopy"]["x"]), "y": float(vector["canopy"]["y"]), "vx": float(vector["vx"]), "vy": float(chute["terminal"]), "bob": SWING.point(**vector["bob"]), "previous": SWING.point(**vector["previous"])}
    ticks = max(vector["ticks"])
    feet, tops = [], []
    for _ in range(ticks):
        canopy = chute_step(canopy, chute, vector["target"], vector["remaining"], 0)
        feet.append(canopy["bob"])
        tops.append(SWING.point(canopy["x"], canopy["y"]))
    gravity, drag = SWING.felt(CHUTE_GRAVITY, CHUTE_DAMPING)
    pivot = lambda _time: (float(vector["vx"]), float(chute["terminal"]), 0.0, 0.0)
    reference = numpy.degrees(SWING.pendulum(math.radians(vector["angle"]), 0.0, gravity, drag, lambda _time: (chute["length"], 0.0), pivot, numpy.arange(1, ticks + 1) * TICK))
    SWING.confirmed("canopy-sways/%s: the uniform motion of the canopy" % vector["id"], [[top["x"], top["y"]] for top in tops], [[vector["canopy"]["x"] + vector["vx"] * tick * TICK, vector["canopy"]["y"] + chute["terminal"] * tick * TICK] for tick in range(1, ticks + 1)])
    swung = SWING.degrees_of(feet, tops)
    return [feet[tick - 1] for tick in vector["ticks"]], SWING.apart(swung, reference), float(swung[-1])


def trailing(vector):
    """🪁️ The angle in degrees at which a pet trails behind a canopy in uniform motion once its sway has died: where the pull of gravity and the drag of the air balance, ``atan(γ·vx ÷ (g − γ·vy))`` behind the motion."""
    chute = chute_of(vector["height"])
    gravity, drag = SWING.felt(CHUTE_GRAVITY, CHUTE_DAMPING)
    return float(numpy.degrees(numpy.arctan2(-drag * vector["vx"], gravity - drag * chute["terminal"])))


def flowed(vector, restated, track):
    """🌊️ The landing of a committed drop as one continuous system: the tick it lands at and the descent speed it lands with.

    The fall is the arc whose tick samples the semi-implicit steps are (launched half a tick's gain faster), which
    the restated fall must lie on exactly. Without a canopy the landing is the first tick at or after the event of
    the landing on that arc. With one, the arc ends at the tick the canopy opens at (taken from the restated
    decision, which is discrete by nature); from there the speed lags towards the terminal speed and the height
    grows by the speed times the flare, until the event of the landing, whose moment is answered in ticks. A step
    moves the canopy by the speed it has just reached, so the height follows the lag half a tick ahead of the
    speed; the continuous system does the same."""
    chute = chute_of(vector["height"])
    landing = lambda _time, state: state[0] - vector["drop"]
    landing.terminal, landing.direction = True, 1.0
    falling = lambda _time, state: [state[1], GRAVITY]
    fallen = restated["landed"] if restated["opened"] == 0 else restated["opened"] + CHUTE_REFLEX - 1
    if max(speed for _height, speed, _open in track[:fallen]) > FALL_SPEED - MARGIN:
        raise AssertionError("drops/%s: the fall reaches the terminal speed, where its ticks are no samples of one arc" % vector["id"])
    solution = scipy.integrate.solve_ivp(falling, (0.0, fallen * TICK), [0.0, vector["vy"] + GRAVITY * TICK / 2], method="DOP853", rtol=1e-12, atol=1e-12, max_step=TICK)
    height, speed = float(solution.y[0][-1]), float(solution.y[1][-1]) - GRAVITY * TICK / 2
    SWING.confirmed("drops/%s: the fall" % vector["id"], [track[fallen - 1][0], track[fallen - 1][1]], [height, speed])
    if restated["opened"] == 0:
        solution = scipy.integrate.solve_ivp(falling, (0.0, 60.0), [0.0, vector["vy"] + GRAVITY * TICK / 2], method="DOP853", events=landing, rtol=1e-12, atol=1e-12, max_step=TICK)
        return math.ceil(float(solution.t_events[0][0]) / TICK - STEPPING), speed
    ahead = float(numpy.exp(-TICK / (2.0 * LAG_SECONDS)))
    gliding = lambda _time, state: [state[1] * float(numpy.interp(vector["drop"] - state[0], [0.0, chute["flare"]], [0.5, 1.0])), -(state[1] - chute["terminal"]) / LAG_SECONDS]
    solution = scipy.integrate.solve_ivp(gliding, (0.0, 60.0), [height, chute["terminal"] + (speed - chute["terminal"]) * ahead], method="DOP853", events=landing, rtol=1e-10, atol=1e-10, max_step=TICK)
    if not solution.success or solution.t_events[0].size != 1:
        raise AssertionError("drops/%s: solve_ivp did not find the landing" % vector["id"])
    return fallen + float(solution.t_events[0][0]) / TICK, chute["terminal"] + (float(solution.y_events[0][0][1]) - chute["terminal"]) / ahead


def judged_drop(vector):
    """🛬️ A restated drop and how far it lands from the continuous system: in ticks and in pixels per second of descent speed."""
    restated, track = dropped(vector)
    plain = struck(vector["vy"], vector["drop"])
    SWING.confirmed("drops/%s: the impact without a parachute" % vector["id"], [restated["plain"]], [plain])
    moment, speed = flowed(vector, restated, track)
    if restated["touch"] > restated["glide"] + STEPPING or restated["touch"] < 0.5 * restated["glide"] - STEPPING:
        raise AssertionError("drops/%s: the touch at %r px/s is not between half and all of the descent speed %r" % (vector["id"], restated["touch"], restated["glide"]))
    return restated, abs(restated["landed"] - moment), abs(restated["glide"] - speed)


# endregion 🔖️Reference


# region 🔖️Handlers
def committed(ctx):
    """🧫️ The committed vectors."""
    return json.loads(ctx.fixture_bytes(VECTORS))


def constants(ctx):
    """🎛️ The tuning constants the vectors were generated with; they must be the ones of this reference, the factor must be the exponential of its time constant, and a hard landing must be what a fall of 100 px from rest ends in."""
    document = committed(ctx)
    if document["constants"] != CONSTANTS:
        raise AssertionError("constants: the reference is tuned to %r, the committed vectors to %r" % (CONSTANTS, document["constants"]))
    if abs(float(numpy.exp(-1.0 / (64.0 * LAG_SECONDS))) - CHUTE_FACTOR) > 1e-16 or CHUTE_GRAVITY != GRAVITY / 2 or abs(struck(0, 100) - HARD_LANDING) > STEPPING:
        raise AssertionError("constants: the factor is not exp(−1 ÷ (64 × 0.18)), the sway gravity not half the gravity of a fall, or a hard landing not the end of a fall of 100 px")
    return Outcome(document["constants"])


def impact_speeds(ctx):
    """💥️ The predicted impact speed of every committed fall, confirmed by ``solve_ivp`` with an event at the landing."""
    produced = {}
    vectors = committed(ctx)["impacts"]
    for vector in vectors:
        produced[vector["id"]] = impact_speed(vector["vy"], vector["height"])
        SWING.confirmed("impact-speeds/%s" % vector["id"], [produced[vector["id"]]], [struck(vector["vy"], vector["height"])])
    return SWING.agree("impact-speeds", produced, vectors)


def chute_triggers(ctx):
    """🔔️ Whether a parachute opens for every committed fall, confirmed by numpy's three comparisons; a vector on the edge of the hard landing is refused unless it sits on it exactly."""
    produced = {}
    vectors = committed(ctx)["triggers"]
    for vector in vectors:
        produced[vector["id"]] = chute_opens(vector["vy"], vector["height"])
        ahead = float(numpy.minimum(numpy.sqrt(numpy.float64(vector["vy"]) ** 2 + 2.0 * GRAVITY * numpy.float64(vector["height"])), FALL_SPEED))
        if 0 < abs(ahead - HARD_LANDING) < MARGIN:
            raise AssertionError("chute-triggers/%s: the predicted impact of %r px/s lies on the edge of a hard landing" % (vector["id"], ahead))
        if produced[vector["id"]] != bool(numpy.all([vector["vy"] >= CHUTE_OPENING, vector["height"] >= CHUTE_HEADROOM, ahead > HARD_LANDING])):
            raise AssertionError("chute-triggers/%s: the restatement says %r, numpy otherwise" % (vector["id"], produced[vector["id"]]))
    return SWING.agree("chute-triggers", produced, vectors)


def chute_measures(ctx):
    """📏️ The measures of the parachute of every committed body height, confirmed by numpy's products."""
    produced = {}
    vectors = committed(ctx)["measures"]
    for vector in vectors:
        produced[vector["id"]] = chute_of(vector["height"])
        shares = numpy.array([CHUTE_DESCENT, CHUTE_STEER_SPEED, CHUTE_FLARE, CHUTE_ROD]) * numpy.float64(vector["height"])
        SWING.confirmed("chute-measures/%s" % vector["id"], [produced[vector["id"]][key] for key in ("terminal", "reach", "flare", "length")], shares)
    return SWING.agree("chute-measures", produced, vectors)


def flares(ctx):
    """🎚️ The share of its speed a canopy keeps at every committed height, confirmed by ``numpy.interp``."""
    produced = {}
    vectors = committed(ctx)["flares"]
    for vector in vectors:
        produced[vector["id"]] = flare_of(vector["remaining"], vector["flare"])
        SWING.confirmed("flares/%s" % vector["id"], [produced[vector["id"]]], [float(numpy.interp(vector["remaining"], [0.0, vector["flare"]], [0.5, 1.0])) if vector["flare"] > 0 else (1.0 if vector["remaining"] >= vector["flare"] else 0.5)])
    return SWING.agree("flares", produced, vectors)


def winds(ctx):
    """💨️ The push of the wind at every committed tick and phase, confirmed by ``numpy.sin``."""
    produced = {}
    vectors = committed(ctx)["winds"]
    for vector in vectors:
        produced[vector["id"]] = chute_wind(vector["ticks"], vector["phase"])
        SWING.confirmed("winds/%s" % vector["id"], [produced[vector["id"]]], [float(numpy.sin(2.0 * numpy.pi * (CHUTE_WIND_RATE * vector["ticks"] / 64.0 + vector["phase"]))) * CHUTE_WIND])
    return SWING.agree("winds", produced, vectors)


def canopy_descents(ctx):
    """⛱️ Every committed descent under an open canopy, confirmed number by number."""
    vectors = committed(ctx)["descents"]
    return SWING.agree("canopy-descents", {vector["id"]: descended(vector) for vector in vectors}, vectors)


def canopy_sways(ctx):
    """🎏️ Every committed sway below a canopy in uniform motion, judged by the integrated pendulum; a sway that has died must trail where drag and gravity balance."""
    produced = {}
    vectors = committed(ctx)["sways"]
    for vector in vectors:
        produced[vector["id"]], deviation, last = swayed(vector)
        if deviation > vector["tolerance"]:
            raise AssertionError("canopy-sways/%s: the sway leaves the integrated pendulum by %r°, more than the stated %r°" % (vector["id"], deviation, vector["tolerance"]))
        if vector["settles"] and abs(last - trailing(vector)) > vector["tolerance"]:
            raise AssertionError("canopy-sways/%s: the pet ends at %r°, not where drag and gravity balance, at %r°" % (vector["id"], last, trailing(vector)))
    return SWING.agree("canopy-sways", produced, vectors)


def drops(ctx):
    """🛩️ Every committed drop from the decision to the touch, judged by the continuous system."""
    produced = {}
    vectors = committed(ctx)["drops"]
    for vector in vectors:
        produced[vector["id"]], late, apart = judged_drop(vector)
        if late > vector["ticks"] or apart > vector["tolerance"]:
            raise AssertionError("drops/%s: lands %r ticks and %r px/s from the continuous system, more than the stated %r ticks and %r px/s" % (vector["id"], late, apart, vector["ticks"], vector["tolerance"]))
    return SWING.agree("drops", produced, vectors)


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter():
    """🔮️ Oracle role only: scipy and numpy are the reference, the TypeScript and Rust twins are judged against it."""
    return (
        Adapter("python")
        .oracle("constants", constants)
        .oracle("impact-speeds", impact_speeds)
        .oracle("chute-triggers", chute_triggers)
        .oracle("chute-measures", chute_measures)
        .oracle("flares", flares)
        .oracle("winds", winds)
        .oracle("canopy-descents", canopy_descents)
        .oracle("canopy-sways", canopy_sways)
        .oracle("drops", drops)
    )


# endregion 🔖️Registration

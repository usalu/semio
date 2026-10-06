#!/usr/bin/env python3
"""🪢️ Oracle of everything that hangs in the pets product (MECH §1, §3.3): the constrained swing step, the cone, the follow spring, the lean, the release velocity and the reel, in Python.

The subjects advance a point on a rod or a rope tick by tick (1/64 s) with one constrained Verlet step
(SHAKE). What that step approximates is a pendulum, and ``scipy.integrate.solve_ivp`` (DOP853, relative
and absolute tolerance 1e-12) integrates the pendulum itself: with the angle ``θ`` measured from straight
down towards the positive x axis, a length ``r(t)``, a pivot that moves with velocity ``p'`` and
acceleration ``a``, and a drag ``γ`` on the speed over the stage,

    r·θ'' = −g·sin θ − a_x·cos θ + a_y·sin θ − γ·(r·θ' + p'_x·cos θ − p'_y·sin θ) − 2·r'·θ'.

The step damps the displacement of the tick before, half a tick earlier than a continuous drag acts, so
a per-tick damping factor ``d`` is the drag ``γ = 128·(1 − d) ÷ (1 + d)`` together with a gravity that is
``2 ÷ (1 + d)`` times as strong (a centred difference of exactly that equation; 3.28 per second and
2.6 % for the 0.95 of a held pet). Every committed swing states its
own tolerance in degrees; the oracle refuses the vector when its restated step leaves the integrated
pendulum by more, and otherwise projects the restated positions — the approximation is far coarser than
the comparison tolerance of 1e-9, so the subjects are held to this restatement, which in turn is held to
scipy. The same holds for the amplitude of a free swing over a minute, where a negative control proves
that the judgement discriminates: the usual shortcut (step freely, pull the point back onto the circle)
is run from the same start and must lose more than its stated share, or the oracle refuses to answer.

Around the swing: the cone is judged by ``numpy.arctan2``, sine and cosine of the edge angle; the follow
spring by ``numpy.linalg.matrix_power`` of its one-tick matrix and ``scipy.linalg.expm`` of the continuous
spring it discretises; the lean by ``numpy.arctan2`` within the 2e-6 turns of ``atanTurns``; the release
velocity by ``numpy.polyfit(deg=2)`` over the same seven samples, whose slope at the newest sample the
integer weights must reproduce (and ``fractions.Fraction`` solves the normal equations to show that the
weights are exactly those of the fit); a slack rope by the closed form of a free flight; a reeled rope
by the pendulum with a prescribed length.

One scenario is a supplement, not third-party evidence: ``bit-patterns`` projects the 64-bit patterns of
single restated steps, so the twins are held to each other and to this third implementation bit for bit.

@see https://docs.scipy.org/doc/scipy/reference/generated/scipy.integrate.solve_ivp.html
@see https://docs.scipy.org/doc/scipy/reference/generated/scipy.linalg.expm.html
@see https://numpy.org/doc/stable/reference/generated/numpy.polyfit.html
@see https://numpy.org/doc/stable/reference/generated/numpy.linalg.matrix_power.html
@see ../📐️turn-trigonometry/🐍️.py
@see ../../🧫️fixtures/🪢️swing-dynamics/🔣️.json
"""

# region 🔖️Imports
import fractions
import importlib.util
import json
import math
import os
import struct

import numpy
import scipy.integrate
import scipy.linalg

from semio_repo_test import Adapter, Outcome

# endregion 🔖️Imports


# region 🔖️Constants
VECTORS = "shared://🪢️swing-dynamics/🔣️.json"
TICK = 1.0 / 64.0
GRAVITY = 1800.0
HANG_ROD = 0.8
HANG_GRAVITY = 7200.0
HANG_DAMPING = 0.95
HANG_CONE = 0.5
FOLLOW_STIFFNESS = 534.0
FOLLOW_DAMPING = 46.0
RELEASE_WEIGHTS = (7, -2, -7, -8, -5, 2, 13)
RELEASE_DIVISOR = 28
RELEASE_STALE = 3
THROW_SHARE = 0.5
THROW_LEAST = 70.0
THROW_MOST = 640.0
THROW_RISE = 520.0
REEL_SPEED = 60.0
REEL_RAMP = 10
REEL_LEAST = 0.9
REEL_CAP = 520.0
REEL_DAMPING = 0.9965
TAUT = 1.000001
CONSTANTS = {
    "hangRod": HANG_ROD,
    "hangGravity": HANG_GRAVITY,
    "hangDamping": HANG_DAMPING,
    "hangCone": HANG_CONE,
    "followStiffness": FOLLOW_STIFFNESS,
    "followDamping": FOLLOW_DAMPING,
    "releaseWeights": list(RELEASE_WEIGHTS),
    "releaseDivisor": RELEASE_DIVISOR,
    "releaseStale": RELEASE_STALE,
    "throwShare": THROW_SHARE,
    "throwLeast": THROW_LEAST,
    "throwMost": THROW_MOST,
    "throwRise": THROW_RISE,
    "reelSpeed": REEL_SPEED,
    "reelRamp": REEL_RAMP,
    "reelLeast": REEL_LEAST,
    "reelCap": REEL_CAP,
    "reelDamping": REEL_DAMPING,
}
AGREEMENT = 1e-12
STEPPING = 1e-9
MARGIN = 1e-6
LEAN_BOUND = 2e-6
SETTLED = 1.0 / 120.0
# endregion 🔖️Constants


# region 🔖️Restatement
def neighbour(case):
    """🚪️ The oracle adapter of another case of this owner, loaded from its file."""
    spec = importlib.util.spec_from_file_location("pets_" + case.encode("ascii", "ignore").decode("ascii").replace("-", "_"), os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", case, "🐍️.py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


TRIGONOMETRY = neighbour("📐️turn-trigonometry")


def point(x, y):
    """📍️ A point of two doubles."""
    return {"x": float(x), "y": float(y)}


def bits(value):
    """🧱️ The 64-bit IEEE pattern of a double as sixteen hexadecimal digits."""
    return struct.pack(">d", float(value)).hex()


def swing_step(before, now, bob, previous, length, gravity, damping, rope):
    """⛓️ ``swingStep`` restated: the free Verlet step, then the pull along the radius of the tick before by the smaller root of the quadratic; a slack rope pulls nothing."""
    free_x = bob["x"] + (bob["x"] - previous["x"]) * damping
    free_y = bob["y"] + (bob["y"] - previous["y"]) * damping + gravity / 4096
    radius_x = bob["x"] - before["x"]
    radius_y = bob["y"] - before["y"]
    reach_x = free_x - now["x"]
    reach_y = free_y - now["y"]
    radius = radius_x * radius_x + radius_y * radius_y
    along = reach_x * radius_x + reach_y * radius_y
    reach = reach_x * reach_x + reach_y * reach_y
    if rope and reach <= length * length:
        return point(free_x, free_y)
    root = along * along - radius * (reach - length * length)
    pull = (along - math.sqrt(root if root > 0 else 0.0)) / (radius if radius > 1e-9 else 1e-9)
    return point(free_x - pull * radius_x, free_y - pull * radius_y)


def naive_step(now, bob, previous, length, gravity, damping):
    """🧨️ The shortcut the product does not use, for the negative control: the free Verlet step, then the point pulled back onto the circle along its new radius."""
    free_x = bob["x"] + (bob["x"] - previous["x"]) * damping
    free_y = bob["y"] + (bob["y"] - previous["y"]) * damping + gravity / 4096
    dx, dy = free_x - now["x"], free_y - now["y"]
    distance = math.sqrt(dx * dx + dy * dy)
    return point(now["x"] + dx * length / distance, now["y"] + dy * length / distance)


def cone_clamp(anchor, bob, length):
    """🔻️ ``coneClamp`` restated: onto the edge of the cone on its own side when the direction leans beyond it, drawn in when farther than the rod, else untouched."""
    dx = bob["x"] - anchor["x"]
    dy = bob["y"] - anchor["y"]
    span = dx * dx + dy * dy
    if dy < 0 or dy * dy < HANG_CONE * HANG_CONE * span:
        drop = HANG_CONE * length
        side = math.sqrt(length * length - drop * drop)
        return point(anchor["x"] - side if dx < 0 else anchor["x"] + side, anchor["y"] + drop)
    if span <= length * length * TAUT:
        return point(bob["x"], bob["y"])
    taut = length / math.sqrt(span)
    return point(anchor["x"] + dx * taut, anchor["y"] + dy * taut)


def spring_step(position, velocity, target, stiffness, damping):
    """🪀️ ``springStep`` restated: the velocity first, then the position with the new velocity."""
    quickened = velocity + (stiffness * (target - position) - damping * velocity) * 0.015625
    return position + quickened * 0.015625, quickened


def follow_step(grip, target):
    """👣️ ``followStep`` restated: the follow spring on each axis."""
    x, vx = spring_step(grip["x"], grip["vx"], target["x"], FOLLOW_STIFFNESS, FOLLOW_DAMPING)
    y, vy = spring_step(grip["y"], grip["vy"], target["y"], FOLLOW_STIFFNESS, FOLLOW_DAMPING)
    return {"x": x, "y": y, "vx": vx, "vy": vy}


def hang_of(feet, length):
    """🪝️ ``hangOf`` restated: gripped ``length`` above the feet, at rest."""
    return {"grip": {"x": float(feet["x"]), "y": float(feet["y"]) - length, "vx": 0.0, "vy": 0.0}, "bob": point(feet["x"], feet["y"]), "previous": point(feet["x"], feet["y"])}


def hang_step(hang, target, length):
    """🦧️ ``hangStep`` restated: follow, swing under the held gravity and damping, clamp."""
    grip = follow_step(hang["grip"], target)
    swung = swing_step(hang["grip"], grip, hang["bob"], hang["previous"], length, HANG_GRAVITY, HANG_DAMPING, False)
    return {"grip": grip, "bob": cone_clamp(grip, swung, length), "previous": hang["bob"]}


def lean_of(anchor, bob, length):
    """🗼️ ``leanOf`` restated: the restated ``atanTurns`` of the unit axis from the anchor to the bob, mirrored into the sense of the rig."""
    return TRIGONOMETRY.atan_turns((anchor["x"] - bob["x"]) / length, (bob["y"] - anchor["y"]) / length)


def stale(samples):
    """🧊️ Whether the last three samples are one point."""
    if len(samples) < RELEASE_STALE:
        return False
    newest = samples[-1]
    return all(samples[-back]["x"] == newest["x"] and samples[-back]["y"] == newest["y"] for back in range(2, RELEASE_STALE + 1))


def ring_velocity(samples):
    """💍️ ``ringVelocity`` restated: the weighted sum of the last seven samples relative to the newest, oldest first, times 64 over 28; the oldest stands in for missing ones."""
    count = len(samples)
    if count == 0:
        return point(0, 0)
    newest = samples[-1]
    first = count - len(RELEASE_WEIGHTS)
    x, y = 0.0, 0.0
    for index in range(len(RELEASE_WEIGHTS) - 1):
        sample = samples[0 if first + index < 0 else first + index]
        x = x + RELEASE_WEIGHTS[index] * (float(sample["x"]) - float(newest["x"]))
        y = y + RELEASE_WEIGHTS[index] * (float(sample["y"]) - float(newest["y"]))
    return point((x * 64) / RELEASE_DIVISOR, (y * 64) / RELEASE_DIVISOR)


def throw_of(velocity):
    """🤾️ ``throwOf`` restated: nothing below the least speed, slowed to the most along its direction, the rise cut."""
    speed = math.sqrt(velocity["x"] * velocity["x"] + velocity["y"] * velocity["y"])
    if speed < THROW_LEAST:
        return point(0, 0)
    scale = THROW_MOST / speed if speed > THROW_MOST else 1
    y = velocity["y"] * scale
    return point(velocity["x"] * scale, 0 - THROW_RISE if y < 0 - THROW_RISE else y)


def release_velocity(samples):
    """🖐️ ``releaseVelocity`` restated: the throw of the pointer alone, nothing once it stopped."""
    return point(0, 0) if stale(samples) else throw_of(ring_velocity(samples))


def throw_velocity(hang, samples):
    """🥏️ ``throwVelocity`` restated: the feet's own velocity plus half of what the grip has not caught up with."""
    ring = point(0, 0) if stale(samples) else ring_velocity(samples)
    return throw_of(point((hang["bob"]["x"] - hang["previous"]["x"]) * 64 + THROW_SHARE * (ring["x"] - hang["grip"]["vx"]), (hang["bob"]["y"] - hang["previous"]["y"]) * 64 + THROW_SHARE * (ring["y"] - hang["grip"]["vy"])))


def reel_step(anchor, bob, previous, length, least, age):
    """🎣️ ``reelStep`` restated: the rope shortened by the ramped reel speed down to its least, the swing on it, the step cut to the cap and drawn in to the rope."""
    speed = (REEL_SPEED * (age + 1)) / REEL_RAMP if age + 1 < REEL_RAMP else REEL_SPEED
    floor = length if length < least else least
    wound = length - speed / 64
    rope = floor if wound < floor else wound
    swung = swing_step(anchor, anchor, bob, previous, rope, GRAVITY, REEL_DAMPING, True)
    dx = swung["x"] - bob["x"]
    dy = swung["y"] - bob["y"]
    step = dx * dx + dy * dy
    cap = REEL_CAP / 64
    if step <= cap * cap:
        return {"bob": swung, "length": rope}
    slowed = cap / math.sqrt(step)
    x = bob["x"] + dx * slowed
    y = bob["y"] + dy * slowed
    span = (x - anchor["x"]) * (x - anchor["x"]) + (y - anchor["y"]) * (y - anchor["y"])
    if span <= rope * rope:
        return {"bob": point(x, y), "length": rope}
    taut = rope / math.sqrt(span)
    return {"bob": point(anchor["x"] + (x - anchor["x"]) * taut, anchor["y"] + (y - anchor["y"]) * taut), "length": rope}


# endregion 🔖️Restatement


# region 🔖️Reference
def still(_time):
    """🗿️ A pivot that does not move: no velocity, no acceleration."""
    return 0.0, 0.0, 0.0, 0.0


def felt(gravity, damping):
    """🍯️ The gravity and the drag (in 1/s) of the pendulum a step with the per-tick damping factor ``d`` follows: ``g·2 ÷ (1 + d)`` and ``128·(1 − d) ÷ (1 + d)``."""
    return gravity * 2.0 / (1.0 + damping), 2.0 * (1.0 - damping) / ((1.0 + damping) * TICK)


def pendulum(angle, speed, gravity, drag, length, pivot, times):
    """🕰️ The angle of the pendulum at each of ``times`` (seconds, any sign) from ``angle`` and ``speed`` at time 0, integrated by scipy; ``length(t)`` yields the length and its rate, ``pivot(t)`` the pivot's velocity and acceleration."""

    def rate(time, state):
        radius, reeling = length(time)
        vx, vy, ax, ay = pivot(time)
        sine, cosine = math.sin(state[0]), math.cos(state[0])
        return [state[1], (-gravity * sine - ax * cosine + ay * sine - drag * (radius * state[1] + vx * cosine - vy * sine) - 2.0 * reeling * state[1]) / radius]

    times = numpy.asarray(times, dtype=float)
    angles = numpy.empty(times.shape)
    for forward in (True, False):
        chosen = numpy.flatnonzero(times > 0) if forward else numpy.flatnonzero(times < 0)
        if chosen.size == 0:
            continue
        ordered = chosen[numpy.argsort(times[chosen] if forward else -times[chosen])]
        solution = scipy.integrate.solve_ivp(rate, (0.0, float(times[ordered[-1]])), [angle, speed], method="DOP853", t_eval=times[ordered], rtol=1e-12, atol=1e-12, max_step=TICK / 4)
        if not solution.success:
            raise AssertionError("solve_ivp failed: %s" % solution.message)
        angles[ordered] = solution.y[0]
    angles[times == 0] = angle
    return angles


def hung(anchor, length, angle):
    """🎐️ The point ``length`` from ``anchor`` at ``angle`` radians from straight down."""
    return point(anchor["x"] + length * math.sin(angle), anchor["y"] + length * math.cos(angle))


def degrees_of(path, anchors):
    """📐️ The angle in degrees of every point of a path seen from its anchor, from straight down towards the positive x axis, by ``numpy.arctan2``."""
    return numpy.degrees(numpy.arctan2([place["x"] - anchor["x"] for place, anchor in zip(path, anchors)], [place["y"] - anchor["y"] for place, anchor in zip(path, anchors)]))


def apart(left, right):
    """⭕️ How far two lists of angles in degrees lie apart at most, around the circle."""
    return float(numpy.max(numpy.abs((numpy.asarray(left) - numpy.asarray(right) + 180.0) % 360.0 - 180.0)))


def fixed(vector):
    """📌️ The length of a rod that keeps it, as a function of time."""
    return lambda _time: (float(vector["length"]), 0.0)


def rod_start(vector):
    """🌱️ Where a committed swing is at tick 0 and one tick before it, read off the integrated pendulum."""
    angle, speed = math.radians(vector["angle"]), math.radians(vector.get("speed", 0))
    before = pendulum(angle, speed, *felt(vector["gravity"], vector["damping"]), fixed(vector), still, [-TICK])[0]
    return hung(vector["anchor"], vector["length"], angle), hung(vector["anchor"], vector["length"], before)


def rod_run(vector, ticks, step=swing_step):
    """🏃️ The restated positions of a committed swing after each of ``ticks`` ticks."""
    bob, previous = point(**vector["bob"]), point(**vector["previous"])
    path = []
    for _ in range(ticks):
        if step is swing_step:
            bob, previous = swing_step(vector["anchor"], vector["anchor"], bob, previous, vector["length"], vector["gravity"], vector["damping"], False), bob
        else:
            bob, previous = naive_step(vector["anchor"], bob, previous, vector["length"], vector["gravity"], vector["damping"]), bob
        path.append(bob)
    return path


def rod_swing(vector):
    """🎢️ The restated positions of a committed swing at its ticks and how far its angle leaves the integrated pendulum at most, in degrees."""
    bob, previous = rod_start(vector)
    confirmed("rod-swings/%s: the committed start" % vector["id"], [vector["bob"]["x"], vector["bob"]["y"], vector["previous"]["x"], vector["previous"]["y"]], [bob["x"], bob["y"], previous["x"], previous["y"]])
    ticks = max(vector["ticks"])
    path = rod_run(vector, ticks)
    reference = numpy.degrees(pendulum(math.radians(vector["angle"]), math.radians(vector.get("speed", 0)), *felt(vector["gravity"], vector["damping"]), fixed(vector), still, numpy.arange(1, ticks + 1) * TICK))
    return [path[tick - 1] for tick in vector["ticks"]], apart(degrees_of(path, [vector["anchor"]] * ticks), reference)


def peaks(path, anchor):
    """⛰️ The amplitude in degrees of every swing of a path: around each sample that is farther out than both neighbours, the top of the quartic ``numpy.polyfit`` lays through that sample and two on either side (``numpy.roots`` of its derivative)."""
    angles = numpy.abs(numpy.unwrap(numpy.radians(degrees_of(path, [anchor] * len(path)))))
    tops = []
    for index in range(2, len(angles) - 2):
        if angles[index] >= angles[index - 1] and angles[index] > angles[index + 1]:
            curve = numpy.polyfit([-2.0, -1.0, 0.0, 1.0, 2.0], angles[index - 2 : index + 3], 4)
            turning = [root.real for root in numpy.roots(numpy.polyder(curve)) if abs(root.imag) < 1e-9 and abs(root.real) <= 1.0]
            tops.append(math.degrees(max(float(numpy.polyval(curve, moment)) for moment in turning)))
    return tops


def lowest(path, anchor, window):
    """🪜️ The least height of the path below its anchor in every window of ticks — the top of the swing as the subjects can state it without an arctangent."""
    heights = [place["y"] - anchor["y"] for place in path]
    return [min(heights[start : start + window]) for start in range(0, len(heights), window)]


def drift(vector):
    """🧭️ The summary of a committed free swing over its ticks, how far the amplitude of the restated step drifts from the start, and how far the amplitude of the shortcut does."""
    bob, previous = rod_start(vector)
    confirmed("amplitude-drift/%s: the committed start" % vector["id"], [vector["bob"]["x"], vector["bob"]["y"], vector["previous"]["x"], vector["previous"]["y"]], [bob["x"], bob["y"], previous["x"], previous["y"]])
    times = numpy.arange(0, vector["ticks"] + 1) * TICK
    reference = pendulum(math.radians(vector["angle"]), 0.0, vector["gravity"], 0.0, fixed(vector), still, times)
    integrated = peaks([hung(vector["anchor"], vector["length"], angle) for angle in reference], vector["anchor"])
    if not integrated or max(abs(top - vector["angle"]) for top in integrated) > 1e-3:
        raise AssertionError("amplitude-drift/%s: the integrated pendulum itself does not keep its amplitude: %r" % (vector["id"], integrated[:3]))
    path = [point(**vector["bob"])] + rod_run(vector, vector["ticks"])
    kept = max(abs(top - vector["angle"]) for top in peaks(path, vector["anchor"]))
    shortcut = peaks([point(**vector["bob"])] + rod_run(vector, vector["ticks"], naive_step), vector["anchor"])
    lost = vector["angle"] - shortcut[-1]
    return {"lowest": lowest(path[1:], vector["anchor"], vector["window"]), "end": path[-1]}, kept, lost


def glide(vector):
    """🛫️ A pivot that travels ``move`` from ``from`` within ``duration`` seconds on the minimum-jerk profile ``10s³ − 15s⁴ + 6s⁵``: its place, velocity and acceleration at a time."""
    duration = float(vector["duration"])

    def state(time):
        share = min(max(time / duration, 0.0), 1.0)
        place = 10 * share**3 - 15 * share**4 + 6 * share**5
        speed = (30 * share**2 - 60 * share**3 + 30 * share**4) / duration
        thrust = (60 * share - 180 * share**2 + 120 * share**3) / duration**2
        return place, speed, thrust

    return state


def glide_path(vector, ticks):
    """🛤️ The places of a gliding pivot at ticks 0 … ``ticks``."""
    state = glide(vector)
    return [point(vector["from"]["x"] + vector["move"]["x"] * state(tick * TICK)[0], vector["from"]["y"] + vector["move"]["y"] * state(tick * TICK)[0]) for tick in range(ticks + 1)]


def anchored(vector):
    """🚚️ The restated positions of a rod below a gliding pivot at its ticks, and how far its angle leaves the integrated pendulum at most, in degrees."""
    ticks = max(vector["ticks"])
    path = glide_path(vector, ticks)
    confirmed("moving-anchors/%s: the committed path" % vector["id"], [[place["x"], place["y"]] for place in vector["path"]], [[place["x"], place["y"]] for place in path])
    state = glide(vector)
    pivot = lambda time: (vector["move"]["x"] * state(time)[1], vector["move"]["y"] * state(time)[1], vector["move"]["x"] * state(time)[2], vector["move"]["y"] * state(time)[2])
    reference = numpy.degrees(pendulum(0.0, 0.0, vector["gravity"], 0.0, fixed(vector), pivot, numpy.arange(1, ticks + 1) * TICK))
    anchors = [point(**place) for place in vector["path"]]
    bob = previous = point(anchors[0]["x"], anchors[0]["y"] + vector["length"])
    swung = []
    for tick in range(1, ticks + 1):
        bob, previous = swing_step(anchors[tick - 1], anchors[tick], bob, previous, vector["length"], vector["gravity"], 1, False), bob
        swung.append(bob)
    return [swung[tick - 1] for tick in vector["ticks"]], apart(degrees_of(swung, anchors[1:]), reference)


def slack(vector):
    """🪁️ The restated positions of a point on a rope after every tick, and the first tick at which the rope pulls (0 when it never does): until then the path must be the closed form of a free flight, afterwards the point never leaves the rope's reach."""
    anchor, length = vector["anchor"], vector["length"]
    bob, previous = point(**vector["bob"]), point(**vector["previous"])
    stride = (bob["x"] - previous["x"], bob["y"] - previous["y"])
    ticks = numpy.arange(1, vector["ticks"] + 1)
    flight = numpy.stack([bob["x"] + ticks * stride[0], bob["y"] + ticks * stride[1] + vector["gravity"] * ticks * (ticks + 1) / 2.0 / 4096.0], axis=1)
    beyond = numpy.flatnonzero(numpy.linalg.norm(flight - [anchor["x"], anchor["y"]], axis=1) > length)
    taut = int(beyond[0]) + 1 if beyond.size else 0
    if beyond.size and abs(float(numpy.linalg.norm(flight[beyond[0]] - [anchor["x"], anchor["y"]])) - length) < MARGIN:
        raise AssertionError("slack-ropes/%s: the flight grazes the reach of the rope, so the tick it pulls at is not robust" % vector["id"])
    path = []
    for _ in range(vector["ticks"]):
        bob, previous = swing_step(anchor, anchor, bob, previous, length, vector["gravity"], 1, True), bob
        path.append(bob)
    free = (taut - 1) if taut else vector["ticks"]
    if free:
        confirmed("slack-ropes/%s: the free flight" % vector["id"], [[place["x"], place["y"]] for place in path[:free]], flight[:free])
    reaches = numpy.linalg.norm([[place["x"] - anchor["x"], place["y"] - anchor["y"]] for place in path], axis=1)
    if float(reaches.max()) > length + STEPPING or (taut and abs(float(reaches[taut - 1]) - length) > STEPPING):
        raise AssertionError("slack-ropes/%s: the point leaves the reach of its rope: %r" % (vector["id"], reaches.tolist()))
    return path, taut


def reeled(vector):
    """🧶️ The restated positions of a swing on a rope that shortens by ``rate`` pixels per second, at its ticks, and how far its angle leaves the pendulum with that prescribed length at most, in degrees."""
    winding = lambda time: (vector["length"] - vector["rate"] * time, -float(vector["rate"]))
    angle = math.radians(vector["angle"])
    before = pendulum(angle, 0.0, vector["gravity"], 0.0, winding, still, [-TICK])[0]
    start, earlier = hung(vector["anchor"], vector["length"], angle), hung(vector["anchor"], winding(-TICK)[0], before)
    confirmed("reeled-swings/%s: the committed start" % vector["id"], [vector["bob"]["x"], vector["bob"]["y"], vector["previous"]["x"], vector["previous"]["y"]], [start["x"], start["y"], earlier["x"], earlier["y"]])
    ticks = max(vector["ticks"])
    reference = numpy.degrees(pendulum(angle, 0.0, vector["gravity"], 0.0, winding, still, numpy.arange(1, ticks + 1) * TICK))
    bob, previous = point(**vector["bob"]), point(**vector["previous"])
    path = []
    for tick in range(1, ticks + 1):
        bob, previous = swing_step(vector["anchor"], vector["anchor"], bob, previous, vector["length"] - (vector["rate"] * tick) / 64, vector["gravity"], 1, vector["rope"]), bob
        path.append(bob)
    return [path[tick - 1] for tick in vector["ticks"]], apart(degrees_of(path, [vector["anchor"]] * ticks), reference)


def reel_lengths(vector, ticks):
    """📏️ The length of a reeled rope after each of ``ticks`` ticks by ``numpy.cumsum`` of the ramped reel speed, held at its least."""
    speeds = REEL_SPEED * numpy.minimum(numpy.arange(1, ticks + 1), REEL_RAMP) / REEL_RAMP
    return numpy.maximum(vector["length"] - numpy.cumsum(speeds) / 64.0, min(vector["length"], vector["least"]))


def guarded(vector):
    """🚦️ The restated states of a reeled swing at its ticks and its fastest step in pixels per second; the rope must follow the ramp down to its least, the pet never leaves the rope's reach, and no step may exceed the cap by more than what the reel takes in. Without the cap the same swing must run away exactly when the vector says the cap acts."""
    ticks = max(vector["ticks"])
    bob, previous, length = point(**vector["bob"]), point(**vector["previous"]), float(vector["length"])
    free, earlier = bob, previous
    states, fastest, wildest = [], 0.0, 0.0
    lengths = reel_lengths(vector, ticks)
    for age in range(ticks):
        reel = reel_step(vector["anchor"], bob, previous, length, vector["least"], age)
        fastest = max(fastest, math.sqrt((reel["bob"]["x"] - bob["x"]) ** 2 + (reel["bob"]["y"] - bob["y"]) ** 2) * 64)
        bob, previous, length = reel["bob"], bob, reel["length"]
        states.append({"x": bob["x"], "y": bob["y"], "length": length})
        loose = swing_step(vector["anchor"], vector["anchor"], free, earlier, float(lengths[age]), GRAVITY, REEL_DAMPING, True)
        wildest = max(wildest, math.sqrt((loose["x"] - free["x"]) ** 2 + (loose["y"] - free["y"]) ** 2) * 64)
        free, earlier = loose, free
    confirmed("reel-guards/%s: the lengths of the rope" % vector["id"], [state["length"] for state in states], lengths)
    beyond = float(numpy.max(numpy.linalg.norm([[state["x"] - vector["anchor"]["x"], state["y"] - vector["anchor"]["y"]] for state in states], axis=1) - lengths))
    if fastest > REEL_CAP + REEL_SPEED or beyond > STEPPING:
        raise AssertionError("reel-guards/%s: fastest step %r px/s, farthest beyond the rope %r px" % (vector["id"], fastest, beyond))
    if abs(wildest - REEL_CAP) < MARGIN or (wildest > REEL_CAP) != vector["capped"]:
        raise AssertionError("reel-guards/%s: without the cap the swing reaches %r px/s, the committed vector says the cap %s" % (vector["id"], wildest, "acts" if vector["capped"] else "never acts"))
    return {"states": [states[tick - 1] for tick in vector["ticks"]], "fastest": fastest}


def edge(anchor, bob, length):
    """🍦️ Where numpy puts a point that must stay on its rod and inside the cone: by ``numpy.arctan2`` the lean, by sine and cosine the edge of the cone, by ``numpy.linalg.norm`` the draw towards the anchor."""
    dx, dy = bob["x"] - anchor["x"], bob["y"] - anchor["y"]
    lean = float(numpy.arctan2(dx, dy))
    widest = float(numpy.arccos(HANG_CONE))
    if abs(abs(lean) - widest) < MARGIN or abs(float(numpy.linalg.norm([dx, dy])) - length * math.sqrt(TAUT)) < 1e-7 * length:
        raise AssertionError("a committed point lies on the edge of the cone or of the rod's slack, so its clamp is not robust")
    if abs(lean) > widest:
        return point(anchor["x"] + length * float(numpy.sin(-widest if dx < 0 else widest)), anchor["y"] + length * float(numpy.cos(widest)))
    distance = float(numpy.linalg.norm([dx, dy]))
    if distance <= length * math.sqrt(TAUT):
        return point(bob["x"], bob["y"])
    return point(anchor["x"] + dx / distance * length, anchor["y"] + dy / distance * length)


def clamped(vector):
    """🍧️ The restated clamp of one committed point, confirmed by numpy's."""
    held = cone_clamp(vector["anchor"], vector["bob"], vector["length"])
    judged = edge(vector["anchor"], vector["bob"], vector["length"])
    confirmed("cone-clamps/%s" % vector["id"], [held["x"], held["y"]], [judged["x"], judged["y"]])
    return held


def step_matrix(stiffness, damping):
    """🧮️ One tick of the spring on ``(position − target, velocity)``."""
    kept = 1.0 - damping * TICK
    return numpy.array([[1.0 - stiffness * TICK * TICK, TICK * kept], [-stiffness * TICK, kept]], dtype=float)


def followed(vector):
    """🧲️ The grip after each committed number of ticks: the matrix power of the one-tick spring on each axis, which the single restated steps must reach."""
    matrix = step_matrix(FOLLOW_STIFFNESS, FOLLOW_DAMPING)
    grip = {key: float(value) for key, value in vector["grip"].items()}
    states, reached = [], 0
    for ticks in vector["ticks"]:
        while reached < ticks:
            grip = follow_step(grip, vector["target"])
            reached += 1
        power = numpy.linalg.matrix_power(matrix, ticks)
        x = power @ [vector["grip"]["x"] - vector["target"]["x"], vector["grip"]["vx"]]
        y = power @ [vector["grip"]["y"] - vector["target"]["y"], vector["grip"]["vy"]]
        confirmed("follow-springs/%s after %d ticks" % (vector["id"], ticks), [grip["x"], grip["vx"], grip["y"], grip["vy"]], [vector["target"]["x"] + x[0], x[1], vector["target"]["y"] + y[0], y[1]])
        states.append(dict(grip))
    return states


def follow_facts():
    """🛋️ What the follow spring promises, judged by numpy and scipy: no overshoot, half of a jump after 4 ticks, a steady lag of 70 ms, and never more than 13 % of a jump from the critically damped spring of a half-life of 0.06 s that ``scipy.linalg.expm`` advances (the tick-wise spring leads it)."""
    matrix = step_matrix(FOLLOW_STIFFNESS, FOLLOW_DAMPING)
    roots = numpy.linalg.eigvals(matrix)
    if numpy.max(numpy.abs(roots.imag)) > 0 or numpy.min(roots.real) <= 0 or numpy.max(roots.real) >= 1:
        raise AssertionError("the follow spring would ring or overshoot: eigenvalues %r" % roots.tolist())
    rate = 2.0 * math.log(2.0) / 0.06
    if abs(FOLLOW_STIFFNESS - rate * rate) > 0.01 * rate * rate or abs(FOLLOW_DAMPING - 2.0 * rate) > 0.01 * 2.0 * rate:
        raise AssertionError("the follow spring is not the critically damped spring of a half-life of 0.06 s: %r, %r" % (rate * rate, 2.0 * rate))
    continuous = scipy.linalg.expm(numpy.array([[0.0, 1.0], [-rate * rate, -2.0 * rate]]) * TICK)
    stepped, flowed, widest, half = numpy.array([-1.0, 0.0]), numpy.array([-1.0, 0.0]), 0.0, 0
    for tick in range(1, 65):
        stepped, flowed = matrix @ stepped, continuous @ flowed
        widest = max(widest, abs(float(stepped[0] - flowed[0])))
        if half == 0 and stepped[0] >= -0.5:
            half = tick
    grip = {"x": 0.0, "y": 0.0, "vx": 0.0, "vy": 0.0}
    for tick in range(1, 513):
        grip = follow_step(grip, point(640.0 * tick * TICK, 0.0))
    lag = (640.0 * 512 * TICK - grip["x"]) / 640.0
    if half != 4 or widest > 0.13 or abs(lag - (FOLLOW_DAMPING / FOLLOW_STIFFNESS - TICK)) > STEPPING or abs(lag - 0.0705) > 0.001:
        raise AssertionError("the follow spring: half way at tick %d, %r of a jump from the continuous spring, a lag of %r s" % (half, widest, lag))
    return {"half": half, "widest": widest, "lag": lag}


def weights_exact():
    """⚖️ The slope at the newest of seven evenly spaced samples of the parabola fitted by least squares, as exact fractions of each sample: the normal equations solved in rational arithmetic."""
    ticks = [fractions.Fraction(tick) for tick in range(-6, 1)]
    moments = [sum(tick**power for tick in ticks) for power in range(5)]
    rows = [[moments[row + column] for column in range(3)] + [fractions.Fraction(int(row == 1))] for row in range(3)]
    for pivot in range(3):
        rows[pivot] = [entry / rows[pivot][pivot] for entry in rows[pivot]]
        for other in range(3):
            if other != pivot:
                rows[other] = [entry - rows[other][pivot] * kept for entry, kept in zip(rows[other], rows[pivot])]
    return [sum(rows[power][3] * tick**power for power in range(3)) for tick in ticks]


def padded(samples):
    """🧺️ The last seven samples as rows, the oldest standing in for missing ones."""
    rows = [[float(sample["x"]), float(sample["y"])] for sample in samples[-7:]]
    return numpy.array([rows[0]] * (7 - len(rows)) + rows)


def fitted(samples):
    """📈️ The velocity of the pointer at its newest sample in pixels per second: the derivative of ``numpy.polyfit(deg=2)`` over the last seven samples."""
    if not samples:
        return point(0, 0)
    rows = padded(samples)
    slopes = [float(numpy.polyfit(numpy.arange(-6.0, 1.0), rows[:, axis] - rows[-1, axis], 2)[1]) * 64.0 for axis in (0, 1)]
    return point(slopes[0], slopes[1])


def limited(velocity):
    """🚧️ The throw numpy makes of a velocity: nothing below the least speed, the direction kept and the speed clipped to the most, the rise clipped; refused on the edge of either speed."""
    speed = float(numpy.linalg.norm([velocity["x"], velocity["y"]]))
    if abs(speed - THROW_LEAST) < MARGIN or abs(speed - THROW_MOST) < MARGIN:
        raise AssertionError("a committed release lies on the edge of the least or the most speed, so its throw is not robust")
    if speed < THROW_LEAST:
        return point(0, 0)
    kept = numpy.array([velocity["x"], velocity["y"]]) / speed * float(numpy.clip(speed, None, THROW_MOST))
    return point(kept[0], float(numpy.clip(kept[1], -THROW_RISE, None)))


def stopped(samples):
    """🥶️ Whether numpy finds the last three samples equal."""
    return len(samples) >= RELEASE_STALE and bool(numpy.all(padded(samples)[-RELEASE_STALE:] == padded(samples)[-1]))


def released(vector):
    """🎯️ The restated velocity of the pointer and its throw for one committed ring of samples, each confirmed by numpy's fit and limits."""
    ring = ring_velocity(vector["samples"])
    confirmed("release-velocities/%s: the slope of the fit" % vector["id"], [ring["x"], ring["y"]], [fitted(vector["samples"])["x"], fitted(vector["samples"])["y"]])
    release = release_velocity(vector["samples"])
    judged = point(0, 0) if stopped(vector["samples"]) else limited(fitted(vector["samples"]))
    confirmed("release-velocities/%s: the throw" % vector["id"], [release["x"], release["y"]], [judged["x"], judged["y"]])
    return {"ring": ring, "release": release}


def thrown(vector):
    """🤝️ The restated throw of a held pet, confirmed by numpy: the feet's velocity plus half of the fitted pointer velocity the grip lacks, through the limits."""
    hang = vector["hang"]
    ring = point(0, 0) if stopped(vector["samples"]) else fitted(vector["samples"])
    own = (numpy.array([hang["bob"]["x"], hang["bob"]["y"]], dtype=float) - [hang["previous"]["x"], hang["previous"]["y"]]) * 64.0
    blend = own + THROW_SHARE * (numpy.array([ring["x"], ring["y"]]) - [hang["grip"]["vx"], hang["grip"]["vy"]])
    judged = limited(point(blend[0], blend[1]))
    throw = throw_velocity(hang, vector["samples"])
    confirmed("throw-velocities/%s" % vector["id"], [throw["x"], throw["y"]], [judged["x"], judged["y"]])
    return throw


def dragged(vector):
    """🖱️ The restated leans of a held pet whose pointer glides, at its ticks, their peak and the tick from which the lean stays below 3°; every lean is confirmed by ``numpy.arctan2`` within 2e-6 turns."""
    state = glide(vector)
    pointer = [point(vector["from"]["x"] + vector["move"]["x"] * state(tick * TICK)[0], vector["from"]["y"] + vector["move"]["y"] * state(tick * TICK)[0]) for tick in range(1, vector["span"] + 1)]
    confirmed("drag-leans/%s: the committed pointer" % vector["id"], [[place["x"], place["y"]] for place in vector["pointer"]], [[round(place["x"], 2), round(place["y"], 2)] for place in pointer])
    hang = hang_of(vector["feet"], vector["length"])
    leans, peak, settled = [], 0.0, 1
    for tick in range(1, vector["span"] + 1):
        hang = hang_step(hang, vector["pointer"][tick - 1], vector["length"])
        lean = lean_of(hang["grip"], hang["bob"], vector["length"])
        judged = float(numpy.arctan2(hang["grip"]["x"] - hang["bob"]["x"], hang["bob"]["y"] - hang["grip"]["y"]) / (2.0 * numpy.pi))
        if abs(lean - judged) > LEAN_BOUND:
            raise AssertionError("drag-leans/%s: tick %d leans %r turns, numpy says %r" % (vector["id"], tick, lean, judged))
        reach = math.sqrt((hang["bob"]["x"] - hang["grip"]["x"]) ** 2 + (hang["bob"]["y"] - hang["grip"]["y"]) ** 2)
        if abs(reach - vector["length"]) > MARGIN * vector["length"] or abs(lean) > 1.0 / 6.0 + LEAN_BOUND:
            raise AssertionError("drag-leans/%s: tick %d leaves the rod or the cone: reach %r, lean %r" % (vector["id"], tick, reach, lean))
        leans.append(lean)
        peak = max(peak, abs(lean))
        if abs(lean) >= SETTLED:
            settled = tick + 1
    return {"leans": [leans[tick - 1] for tick in vector["ticks"]], "peak": peak, "settled": settled}


def flowing(vector):
    """🌊️ The peak lean in degrees of the continuous counterpart of a drag that never reaches the cone: the pointer glides, the grip follows it as a spring, and the pendulum below feels the grip's acceleration and the drag of the held damping — one system for ``solve_ivp``."""
    state = glide(vector)
    length = float(vector["length"])
    gravity, drag = felt(HANG_GRAVITY, HANG_DAMPING)

    def rate(time, values):
        place, speed, angle, turning = values
        thrust = FOLLOW_STIFFNESS * (vector["move"]["x"] * state(time)[0] - place) - FOLLOW_DAMPING * speed
        return [speed, thrust, turning, (-gravity * math.sin(angle) - thrust * math.cos(angle) - drag * (length * turning + speed * math.cos(angle))) / length]

    times = numpy.arange(1, vector["span"] + 1) * TICK
    solution = scipy.integrate.solve_ivp(rate, (0.0, float(times[-1])), [0.0, 0.0, 0.0, 0.0], method="DOP853", t_eval=times, rtol=1e-12, atol=1e-12, max_step=TICK / 4)
    if not solution.success:
        raise AssertionError("solve_ivp failed: %s" % solution.message)
    return float(numpy.degrees(numpy.max(numpy.abs(solution.y[2]))))


def stepped(vector):
    """🔬️ The bit patterns of one restated step; where the step can reach the rod's length, ``numpy.linalg.norm`` must find it there."""
    new = swing_step(vector["before"], vector["now"], vector["bob"], vector["previous"], vector["length"], vector["gravity"], vector["damping"], vector["rope"])
    reach = float(numpy.linalg.norm([new["x"] - vector["now"]["x"], new["y"] - vector["now"]["y"]]))
    if vector["held"] != (abs(reach - vector["length"]) <= STEPPING * max(1.0, vector["length"])):
        raise AssertionError("bit-patterns/%s: the step ends %r from its anchor on a rod of %r, the committed vector says it %s" % (vector["id"], reach, vector["length"], "holds" if vector["held"] else "does not hold"))
    return {"x": bits(new["x"]), "y": bits(new["y"])}


# endregion 🔖️Reference


# region 🔖️Handlers
def committed(ctx):
    """🧫️ The committed vectors."""
    return json.loads(ctx.input_bytes(VECTORS))


def confirmed(label, stated, judged):
    """🤞️ Refuses restated numbers the libraries do not reach within 1e-9 (relative beyond 1)."""
    stated, judged = numpy.asarray(stated, dtype=float), numpy.asarray(judged, dtype=float)
    if stated.shape != judged.shape or (stated.size and float(numpy.max(numpy.abs(stated - judged) / numpy.maximum(1.0, numpy.abs(judged)))) > STEPPING):
        raise AssertionError("%s: the restatement gives %r, the libraries %r" % (label, stated.tolist(), judged.tolist()))


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
    """🧾️ Holds every produced answer to the committed one — the vectors may never drift from the reference."""
    for vector in vectors:
        if not close(produced[vector["id"]], vector["expected"]):
            raise AssertionError("%s/%s: the reference answers %r, the committed vector says %r" % (scenario, vector["id"], produced[vector["id"]], vector["expected"]))
    return Outcome(produced)


def within(scenario, vector, deviation):
    """🎚️ Refuses a swing that leaves the integrated pendulum by more than the degrees its vector states."""
    if deviation > vector["tolerance"]:
        raise AssertionError("%s/%s: the step leaves the integrated pendulum by %r°, more than the stated %r°" % (scenario, vector["id"], deviation, vector["tolerance"]))


def constants(ctx):
    """🎛️ The tuning constants the vectors were generated with; they must be the ones of this reference, the weights must be the exact ones of the fit, and the follow spring must keep its promises."""
    document = committed(ctx)
    if document["constants"] != CONSTANTS:
        raise AssertionError("constants: the reference is tuned to %r, the committed vectors to %r" % (CONSTANTS, document["constants"]))
    if weights_exact() != [fractions.Fraction(weight, RELEASE_DIVISOR) for weight in RELEASE_WEIGHTS] or sum(RELEASE_WEIGHTS) != 0:
        raise AssertionError("constants: the least-squares weights are %r" % weights_exact())
    if HANG_GRAVITY != 4 * GRAVITY or abs(math.degrees(math.acos(HANG_CONE)) - 60.0) > 1e-9:
        raise AssertionError("constants: the held gravity is not four times the gravity of a fall or the cone is not 60° wide")
    follow_facts()
    return Outcome(document["constants"])


def rod_swings(ctx):
    """🕯️ Every committed swing on a rod below a fixed anchor, judged by the integrated pendulum."""
    produced = {}
    vectors = committed(ctx)["rods"]
    for vector in vectors:
        produced[vector["id"]], deviation = rod_swing(vector)
        within("rod-swings", vector, deviation)
    return agree("rod-swings", produced, vectors)


def amplitude_drift(ctx):
    """🏔️ Every committed free swing keeps its amplitude over a minute, and the shortcut does not."""
    produced = {}
    vectors = committed(ctx)["drifts"]
    for vector in vectors:
        produced[vector["id"]], kept, lost = drift(vector)
        if kept > vector["bound"]:
            raise AssertionError("amplitude-drift/%s: the amplitude drifts by %r°, more than the stated %r°" % (vector["id"], kept, vector["bound"]))
        if lost < vector["control"]:
            raise AssertionError("amplitude-drift/%s: the shortcut loses only %r° of its amplitude, less than the stated %r° — this judgement cannot tell the two steps apart" % (vector["id"], lost, vector["control"]))
    return agree("amplitude-drift", produced, vectors)


def moving_anchors(ctx):
    """🚃️ Every committed swing below a gliding anchor, judged by the pendulum with an accelerated pivot."""
    produced = {}
    vectors = committed(ctx)["anchors"]
    for vector in vectors:
        produced[vector["id"]], deviation = anchored(vector)
        within("moving-anchors", vector, deviation)
    return agree("moving-anchors", produced, vectors)


def slack_ropes(ctx):
    """🪢️ Every committed flight on a rope: free until the rope pulls, within its reach afterwards."""
    produced = {}
    vectors = committed(ctx)["ropes"]
    for vector in vectors:
        produced[vector["id"]], taut = slack(vector)
        if taut != vector["taut"]:
            raise AssertionError("slack-ropes/%s: the rope pulls first at tick %d, the committed vector says %d" % (vector["id"], taut, vector["taut"]))
    return agree("slack-ropes", produced, vectors)


def reeled_swings(ctx):
    """🎏️ Every committed swing on a shortening rope, judged by the pendulum with that prescribed length."""
    produced = {}
    vectors = committed(ctx)["reels"]
    for vector in vectors:
        produced[vector["id"]], deviation = reeled(vector)
        within("reeled-swings", vector, deviation)
    return agree("reeled-swings", produced, vectors)


def reel_guards(ctx):
    """🛡️ Every committed reeled swing keeps its rope at or above its least and its speed at or below the cap."""
    vectors = committed(ctx)["guards"]
    return agree("reel-guards", {vector["id"]: guarded(vector) for vector in vectors}, vectors)


def cone_clamps(ctx):
    """🚥️ Every committed point clamped to its rod and cone, confirmed by numpy."""
    vectors = committed(ctx)["cones"]
    return agree("cone-clamps", {vector["id"]: clamped(vector) for vector in vectors}, vectors)


def follow_springs(ctx):
    """🐕️ Every committed grip on its way to its target, confirmed by the matrix power."""
    vectors = committed(ctx)["follows"]
    return agree("follow-springs", {vector["id"]: followed(vector) for vector in vectors}, vectors)


def drag_leans(ctx):
    """🤹️ Every committed drag of a held pet: its leans, their peak and when it settles; a drag that stays off the cone must peak where the continuous system does, within its stated degrees, and one that reaches it must peak at 60°."""
    produced = {}
    vectors = committed(ctx)["drags"]
    for vector in vectors:
        produced[vector["id"]] = dragged(vector)
        peak = produced[vector["id"]]["peak"] * 360.0
        if vector["tolerance"] is None:
            if abs(peak - 60.0) > 360.0 * LEAN_BOUND:
                raise AssertionError("drag-leans/%s: the lean peaks at %r°, not at the cone" % (vector["id"], peak))
        elif abs(peak - flowing(vector)) > vector["tolerance"]:
            raise AssertionError("drag-leans/%s: the lean peaks at %r°, the continuous system at %r°, more than the stated %r° apart" % (vector["id"], peak, flowing(vector), vector["tolerance"]))
    return agree("drag-leans", produced, vectors)


def release_velocities(ctx):
    """🏓️ The pointer velocity and the throw of every committed ring of samples, confirmed by ``numpy.polyfit``."""
    vectors = committed(ctx)["releases"]
    return agree("release-velocities", {vector["id"]: released(vector) for vector in vectors}, vectors)


def throw_velocities(ctx):
    """🎳️ The throw of every committed held pet, confirmed by numpy."""
    vectors = committed(ctx)["throws"]
    return agree("throw-velocities", {vector["id"]: thrown(vector) for vector in vectors}, vectors)


def bit_patterns(ctx):
    """🧬️ The 64-bit patterns of every committed single step, held to the committed ones exactly."""
    vectors = committed(ctx)["steps"]
    produced = {vector["id"]: stepped(vector) for vector in vectors}
    for vector in vectors:
        if produced[vector["id"]] != vector["expected"]:
            raise AssertionError("bit-patterns/%s: the restatement answers %r, the committed vector says %r" % (vector["id"], produced[vector["id"]], vector["expected"]))
    return Outcome(produced)


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter():
    """🔮️ Oracle role only: scipy and numpy are the reference, the TypeScript and Rust twins are judged against it."""
    return (
        Adapter("python")
        .oracle("constants", constants)
        .oracle("rod-swings", rod_swings)
        .oracle("amplitude-drift", amplitude_drift)
        .oracle("moving-anchors", moving_anchors)
        .oracle("slack-ropes", slack_ropes)
        .oracle("reeled-swings", reeled_swings)
        .oracle("reel-guards", reel_guards)
        .oracle("cone-clamps", cone_clamps)
        .oracle("follow-springs", follow_springs)
        .oracle("drag-leans", drag_leans)
        .oracle("release-velocities", release_velocities)
        .oracle("throw-velocities", throw_velocities)
        .oracle("bit-patterns", bit_patterns)
    )


# endregion 🔖️Registration

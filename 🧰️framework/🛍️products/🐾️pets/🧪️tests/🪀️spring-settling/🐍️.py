#!/usr/bin/env python3
"""🪀️ Oracle of the pets product's one-tick spring (design §4.4), in Python.

One tick of semi-implicit Euler with ``dt = 1/64`` — the velocity first, the position with the new
velocity — is a linear map of ``(position − target, velocity)``. The oracle writes that map as the 2×2
matrix ``[[1 − k·dt², dt·(1 − c·dt)], [−k·dt, 1 − c·dt]]`` and lets ``numpy.linalg.matrix_power`` take
``n`` ticks at once (by repeated squaring), which the subjects reach by ``n`` single steps; before an
answer is projected, the oracle's own ``n`` single matrix products must land on the same state.
``numpy.linalg.eigvals`` judges the stated stability region: the spectral radius of the step matrix is
below 1 inside it and at least 1 outside it. The settling properties of the gaze spring are read off
the matrix-power trajectory.

@see https://numpy.org/doc/stable/reference/generated/numpy.linalg.matrix_power.html
@see https://numpy.org/doc/stable/reference/generated/numpy.linalg.eigvals.html
@see https://gafferongames.com/post/integration_basics/
@see ../../🧫️fixtures/🪀️spring-settling/🔣️.json
"""

# region 🔖️Imports
import json
import math

import numpy

from semio_repo_test import Adapter, Outcome

# endregion 🔖️Imports


# region 🔖️Reference
VECTORS = "shared://🪀️spring-settling/🔣️.json"
TICK_SECONDS = 1.0 / 64.0
AGREEMENT = 1e-12
STEPPING = 1e-9
MARGIN = 1e-6


def step_matrix(stiffness, damping):
    """🧮️ One tick on ``(position − target, velocity)``."""
    kept = 1.0 - damping * TICK_SECONDS
    return numpy.array([[1.0 - stiffness * TICK_SECONDS * TICK_SECONDS, TICK_SECONDS * kept], [-stiffness * TICK_SECONDS, kept]], dtype=float)


def after(spring, ticks):
    """⏭️ The spring ``ticks`` later by one matrix power; the same number of single products must agree."""
    matrix = step_matrix(spring["stiffness"], spring["damping"])
    start = numpy.array([spring["position"] - spring["target"], spring["velocity"]], dtype=float)
    state = numpy.linalg.matrix_power(matrix, ticks) @ start
    stepped = start
    for _ in range(ticks):
        stepped = matrix @ stepped
    if not numpy.allclose(state, stepped, rtol=STEPPING, atol=STEPPING):
        raise AssertionError("%d ticks at once reach %r, %d single ticks reach %r" % (ticks, state.tolist(), ticks, stepped.tolist()))
    return {"position": float(spring["target"] + state[0]), "velocity": float(state[1])}


def spectral_radius(stiffness, damping):
    """📡️ The largest magnitude among the eigenvalues of the step matrix: below 1 every motion dies out."""
    return float(numpy.max(numpy.abs(numpy.linalg.eigvals(step_matrix(stiffness, damping)))))


def inside(stiffness, damping):
    """🗺️ The stated stability region: ``stiffness > 0``, ``damping > 0`` and ``stiffness ÷ 4096 + damping ÷ 32 < 4``."""
    return stiffness > 0 and damping > 0 and stiffness / 4096 + damping / 32 < 4


def settling(gaze, jump):
    """🎯️ From which tick a pupil at rest stays within the band around its new target, and how far it overshoots, both as shares of the way."""
    travel = jump["target"] - jump["position"]
    spring = {"position": jump["position"], "velocity": 0.0, "target": jump["target"], "stiffness": gaze["stiffness"], "damping": gaze["damping"]}
    shares = [(after(spring, ticks)["position"] - jump["target"]) / travel for ticks in range(1, gaze["horizon"] + 1)]
    for share in shares:
        if abs(abs(share) - gaze["band"]) < MARGIN:
            raise AssertionError("gaze-settling/%s: a sample lies on the edge of the band, so the settling tick is not robust" % jump["id"])
    outside = [index + 1 for index, share in enumerate(shares) if abs(share) > gaze["band"]]
    settled = (outside[-1] + 1) if outside else 1
    overshoot = max(0.0, max(shares))
    if settled > gaze["settleTicks"]:
        raise AssertionError("gaze-settling/%s: settled at tick %d, later than the stated %d" % (jump["id"], settled, gaze["settleTicks"]))
    if overshoot > gaze["overshootBound"]:
        raise AssertionError("gaze-settling/%s: overshoots by %r of the way, beyond the stated %r" % (jump["id"], overshoot, gaze["overshootBound"]))
    return {"settled": settled, "overshoot": overshoot}


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


def single_steps(ctx):
    """👣️ ``springStep`` once for every committed spring."""
    vectors = committed(ctx)["steps"]
    return agree("single-steps", {vector["id"]: after(vector, 1) for vector in vectors}, vectors)


def tick_runs(ctx):
    """🏃️ Every committed spring after each committed number of ticks."""
    vectors = committed(ctx)["runs"]
    return agree("tick-runs", {vector["id"]: [after(vector, ticks) for ticks in vector["ticks"]] for vector in vectors}, vectors)


def resting(ctx):
    """🛌️ A spring at its target without velocity stays there exactly, however long."""
    vectors = committed(ctx)["rests"]
    produced = {vector["id"]: after({"position": vector["target"], "velocity": 0.0, "target": vector["target"], "stiffness": vector["stiffness"], "damping": vector["damping"]}, vector["ticks"]) for vector in vectors}
    for vector in vectors:
        if produced[vector["id"]] != {"position": vector["target"], "velocity": 0.0}:
            raise AssertionError("resting/%s: the spring left its target: %r" % (vector["id"], produced[vector["id"]]))
    return agree("resting", produced, vectors)


def stable_springs(ctx):
    """🧘️ Springs inside the stated region die out (spectral radius below 1), springs outside it do not; the inside ones are projected after their ticks."""
    document = committed(ctx)
    for vector in document["stables"]:
        radius = spectral_radius(vector["stiffness"], vector["damping"])
        if not inside(vector["stiffness"], vector["damping"]) or radius >= 1 - STEPPING:
            raise AssertionError("stable-springs/%s: spectral radius %r, expected a spring inside the stable region" % (vector["id"], radius))
    for vector in document["unstables"]:
        radius = spectral_radius(vector["stiffness"], vector["damping"])
        if inside(vector["stiffness"], vector["damping"]) or radius < 1 - STEPPING:
            raise AssertionError("stable-springs/%s: spectral radius %r, expected a spring outside the stable region" % (vector["id"], radius))
    produced = {vector["id"]: after(vector, vector["ticks"]) for vector in document["stables"]}
    for vector in document["stables"]:
        start = math.hypot(vector["position"] - vector["target"], vector["velocity"] * TICK_SECONDS)
        end = math.hypot(produced[vector["id"]]["position"] - vector["target"], produced[vector["id"]]["velocity"] * TICK_SECONDS)
        if end >= start:
            raise AssertionError("stable-springs/%s: the motion grew from %r to %r" % (vector["id"], start, end))
    return agree("stable-springs", produced, document["stables"])


def gaze_settling(ctx):
    """👀️ The gaze spring settles within the stated ticks and overshoot for every committed jump."""
    gaze = committed(ctx)["gaze"]
    produced = {"stiffness": gaze["stiffness"], "damping": gaze["damping"], "jumps": {jump["id"]: settling(gaze, jump) for jump in gaze["jumps"]}}
    for jump in gaze["jumps"]:
        if not close(produced["jumps"][jump["id"]], jump["expected"]):
            raise AssertionError("gaze-settling/%s: the reference answers %r, the committed vector says %r" % (jump["id"], produced["jumps"][jump["id"]], jump["expected"]))
    return Outcome(produced)


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter():
    """🧭️ Oracle role only: numpy is the reference, the TypeScript and Rust twins are judged against it."""
    return Adapter("python").oracle("single-steps", single_steps).oracle("tick-runs", tick_runs).oracle("resting", resting).oracle("stable-springs", stable_springs).oracle("gaze-settling", gaze_settling)


# endregion 🔖️Registration

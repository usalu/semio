#!/usr/bin/env python3
"""📐️ Oracle of the pets trigonometry in turns and of its scalar blends (design §4.1), in Python.

Sine and cosine are numpy's own ``numpy.sin`` and ``numpy.cos`` of ``2π·t``; an angle is committed as the
ratio of two integers, so every language forms the same double with one division. ``clamp`` is
``numpy.clip``, ``lerp`` the degree-one polynomial ``numpy.polynomial.polynomial.polyval`` evaluates
(corroborated by ``numpy.interp`` wherever the amount lies inside [0, 1]) and ``smoothstep`` the cubic
``3t² − 2t³`` of the clipped amount through the same ``polyval``. Nothing of the subject's reduction or
of its kernels enters those answers.

One scenario is a supplement, not third-party evidence: ``bit-patterns`` restates the reduction and the
two kernels of design §4.1 in Python's IEEE doubles, operation by operation, and projects the 64-bit
pattern of every result, so the twins are held to each other and to this third implementation bit for
bit — the tolerance of the other scenarios would hide a last-bit difference between languages. The
restatement is itself held to numpy within 1e-12 before anything is projected.

The way back and the rational decay are approximations by design (MECH §0.1), so numpy judges them
within their stated error and the restated value is what is projected: ``atanTurns(y, x)`` must lie
within 2e-6 turns of ``numpy.arctan2(y, x) ÷ 2π`` (compared around the circle, so ½ and −½ are one
direction), ``fastNegExp(x)`` within 1.9e-2 of ``numpy.exp(−x)`` everywhere and within 6e-4 up to
``x = 1``. An answer outside its bound is refused; inside it, the restated double and its 64-bit
pattern are the projection, which holds the twins to each other bit for bit.

@see https://numpy.org/doc/stable/reference/generated/numpy.sin.html
@see https://numpy.org/doc/stable/reference/generated/numpy.arctan2.html
@see https://numpy.org/doc/stable/reference/generated/numpy.exp.html
@see https://www.netlib.org/fdlibm/k_sin.c
@see https://www.netlib.org/fdlibm/k_cos.c
@see https://numpy.org/doc/stable/reference/generated/numpy.clip.html
@see https://numpy.org/doc/stable/reference/generated/numpy.polynomial.polynomial.polyval.html
@see ../../🧫️fixtures/📐️turn-trigonometry/🔣️.json
"""

# region 🔖️Imports
import json
import math
import struct

import numpy
from numpy.polynomial import polynomial

from semio_repo_test import Adapter, Outcome

# endregion 🔖️Imports


# region 🔖️Reference
VECTORS = "shared://📐️turn-trigonometry/🔣️.json"
TOLERANCE = 1e-12


def turns_of(numerator, denominator):
    """➗️ The angle in turns: one correctly rounded division of two integers."""
    return numerator / denominator


def sine(turns):
    """🌀️ ``numpy.sin`` of the angle in radians."""
    return float(numpy.sin(2.0 * numpy.pi * numpy.float64(turns)))


def cosine(turns):
    """🧭️ ``numpy.cos`` of the angle in radians."""
    return float(numpy.cos(2.0 * numpy.pi * numpy.float64(turns)))


def sweep(vector):
    """🧹️ Sines and cosines of ``n ÷ denominator`` turns for every ``n`` from ``first`` to ``last``."""
    turns = numpy.arange(vector["first"], vector["last"] + 1, dtype=numpy.float64) / numpy.float64(vector["denominator"])
    return {"sines": [float(value) for value in numpy.sin(2.0 * numpy.pi * turns)], "cosines": [float(value) for value in numpy.cos(2.0 * numpy.pi * turns)]}


def clamp(value, low, high):
    """🗜️ ``numpy.clip``: raised to ``low``, then lowered to ``high``."""
    return float(numpy.clip(numpy.float64(value), numpy.float64(low), numpy.float64(high)))


def lerp(start, end, amount):
    """↔️ The line through ``(0, start)`` and ``(1, end)`` at ``amount``; ``numpy.interp`` must agree inside [0, 1]."""
    value = float(polynomial.polyval(numpy.float64(amount), [numpy.float64(start), numpy.float64(end) - numpy.float64(start)]))
    if 0.0 <= amount <= 1.0:
        interpolated = float(numpy.interp(amount, [0.0, 1.0], [start, end]))
        if abs(interpolated - value) > TOLERANCE * max(1.0, abs(value)):
            raise AssertionError("lerp(%r, %r, %r): polyval answers %r, numpy.interp %r" % (start, end, amount, value, interpolated))
    return value


def smoothstep(amount):
    """🛝️ ``3t² − 2t³`` of the amount clipped to [0, 1]."""
    return float(polynomial.polyval(numpy.clip(numpy.float64(amount), 0.0, 1.0), [0.0, 0.0, 3.0, -2.0]))


ARC_BOUND = 2e-6
DECAY_BOUND = 1.9e-2
DECAY_NEAR_BOUND = 6e-4


def direction(y, x):
    """🎯️ ``numpy.arctan2`` of the point in turns, a vanished direction as a positive zero."""
    return float(numpy.arctan2(numpy.float64(y), numpy.float64(x)) / (2.0 * numpy.pi)) + 0.0


def around(left, right):
    """⭕️ How far two directions in turns lie apart around the circle."""
    return abs((left - right + 0.5) % 1.0 - 0.5)


def decay(x):
    """📉️ ``numpy.exp`` of the negated argument, 1 below 0."""
    return float(numpy.exp(-numpy.clip(numpy.float64(x), 0.0, None)))


# endregion 🔖️Reference


# region 🔖️Restatement
TAU = 6.283185307179586
SINE = (-1.66666666666666324348e-1, 8.33333333332248946124e-3, -1.98412698298579493134e-4, 2.75573137070700676789e-6, -2.50507602534068634195e-8, 1.58969099521155010221e-10)
COSINE = (4.16666666666666019037e-2, -1.38888888888741095749e-3, 2.48015872894767294178e-5, -2.75573143513906633035e-7, 2.0875723212981748279e-9, -1.13596475577881948265e-11)


def sine_kernel(angle):
    """〰️ The fdlibm sine polynomial on [−π/4, π/4], in Horner form."""
    square = angle * angle
    tail = SINE[1] + square * (SINE[2] + square * (SINE[3] + square * (SINE[4] + square * SINE[5])))
    return angle + square * angle * (SINE[0] + square * tail)


def cosine_kernel(angle):
    """🏔️ The fdlibm cosine polynomial on [−π/4, π/4], in Horner form."""
    square = angle * angle
    tail = COSINE[0] + square * (COSINE[1] + square * (COSINE[2] + square * (COSINE[3] + square * (COSINE[4] + square * COSINE[5]))))
    return 1 - (0.5 * square - square * (square * tail))


def reduced(turns):
    """🪓️ The nearest quarter turn of the angle's magnitude (0 … 4) and the rest in radians, a residue in [−π/4, π/4)."""
    magnitude = abs(turns)
    phase = magnitude - math.floor(magnitude)
    quarter = math.floor((math.floor(phase * 8) + 1) * 0.5)
    return quarter, (phase - quarter * 0.25) * TAU


def sin_turns(turns):
    """🎠️ ``sinTurns`` restated: the kernel and sign of the quarter, negated for a negative angle."""
    quarter, angle = reduced(turns)
    value = cosine_kernel(angle) if quarter == 1 else 0 - sine_kernel(angle) if quarter == 2 else 0 - cosine_kernel(angle) if quarter == 3 else sine_kernel(angle)
    return 0 - value if turns < 0 else value


def cos_turns(turns):
    """🎪️ ``cosTurns`` restated: the kernel and sign of the quarter."""
    quarter, angle = reduced(turns)
    return 0 - sine_kernel(angle) if quarter == 1 else 0 - cosine_kernel(angle) if quarter == 2 else sine_kernel(angle) if quarter == 3 else cosine_kernel(angle)


def bits(value):
    """🧱️ The 64-bit IEEE pattern of a double as sixteen hexadecimal digits."""
    return struct.pack(">d", float(value)).hex()


ARC = (0.9998660, -0.3302995, 0.1801410, -0.0851330, 0.0208351)
DECAY = (0.48, 0.235)


def arc_kernel(ratio):
    """🏹️ The arctangent polynomial of Abramowitz and Stegun 4.4.49 on [0, 1], in Horner form."""
    square = ratio * ratio
    return ratio * (ARC[0] + square * (ARC[1] + square * (ARC[2] + square * (ARC[3] + square * ARC[4]))))


def atan_turns(y, x):
    """🛰️ ``atanTurns`` restated: the polynomial of the smaller magnitude over the larger, a quarter turn minus it past the diagonal, mirrored for a negative ``x``, negated for a negative ``y``, and ½ where that rounds to −½."""
    rise, run = abs(float(y)), abs(float(x))
    if rise == 0 and run == 0:
        return 0.0
    octant = arc_kernel(rise / run) / TAU if rise <= run else 0.25 - arc_kernel(run / rise) / TAU
    half = 0.5 - octant if x < 0 else octant
    turns = 0 - half if y < 0 else half
    return 0.5 if turns <= -0.5 else turns


def fast_neg_exp(x):
    """🕯️ ``fastNegExp`` restated: one over the cubic ``1 + x·(1 + x·(0.48 + 0.235·x))`` of the argument raised to 0."""
    held = float(x) if x > 0 else 0.0
    return 1 / (1 + held * (1 + held * (DECAY[0] + DECAY[1] * held)))


def judged_direction(label, y, x):
    """🧿️ The restated direction of a point, refused when it leaves ``numpy.arctan2`` by more than the stated 2e-6 turns."""
    turns = atan_turns(y, x)
    if not -0.5 < turns <= 0.5 or around(turns, direction(y, x)) > ARC_BOUND:
        raise AssertionError("%s: the restatement answers %r turns for (%r, %r), numpy %r" % (label, turns, y, x, direction(y, x)))
    return turns


def judged_decay(label, x):
    """🧯️ The restated decay of an argument, refused when it leaves ``numpy.exp`` by more than the stated bound of its range."""
    value = fast_neg_exp(x)
    if abs(value - decay(x)) > (DECAY_NEAR_BOUND if x <= 1 else DECAY_BOUND):
        raise AssertionError("%s: the restatement answers %r for %r, numpy %r" % (label, value, x, decay(x)))
    return value


def lattice(vector):
    """🕸️ The directions of every point ``(column × step, row × step)`` of a square lattice, row by row from ``−span`` to ``span``."""
    span, step = vector["span"], vector["step"]
    return [judged_direction("arctangent-grids/%s" % vector["id"], row * step, column * step) for row in range(-span, span + 1) for column in range(-span, span + 1)]


# endregion 🔖️Restatement


# region 🔖️Handlers
def committed(ctx):
    """🧫️ The committed vectors."""
    return json.loads(ctx.input_bytes(VECTORS))


def close(produced, expected):
    """🤏️ Structural equality with a tolerance of 1e-12 on numbers: numpy's sine may differ by an ulp between builds."""
    if isinstance(produced, bool) or isinstance(expected, bool):
        return produced == expected
    if isinstance(produced, (int, float)) and isinstance(expected, (int, float)):
        return abs(produced - expected) <= TOLERANCE
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


def angles(ctx):
    """🎡️ Sine and cosine of every committed angle."""
    vectors = committed(ctx)["angles"]
    return agree("angles", {vector["id"]: {"sine": sine(turns_of(vector["numerator"], vector["denominator"])), "cosine": cosine(turns_of(vector["numerator"], vector["denominator"]))} for vector in vectors}, vectors)


def angle_bits(vector):
    """🔬️ The bit patterns of the restated sine and cosine of one committed angle, refused when the restatement leaves numpy by more than 1e-12 (1e-9 beyond eight turns, where the reference's own ``2π·t`` is coarser)."""
    turns = turns_of(vector["numerator"], vector["denominator"])
    slack = TOLERANCE if abs(turns) <= 8 else 1e-9
    if abs(sin_turns(turns) - sine(turns)) > slack or abs(cos_turns(turns) - cosine(turns)) > slack:
        raise AssertionError("bit-patterns/%s: the restatement answers %r and %r, numpy %r and %r" % (vector["id"], sin_turns(turns), cos_turns(turns), sine(turns), cosine(turns)))
    return {"sine": bits(sin_turns(turns)), "cosine": bits(cos_turns(turns))}


def bit_patterns(ctx):
    """🧬️ The 64-bit patterns of ``sinTurns`` and ``cosTurns`` of every committed angle, held to the committed ones exactly."""
    vectors = committed(ctx)["angles"]
    produced = {vector["id"]: angle_bits(vector) for vector in vectors}
    for vector in vectors:
        if produced[vector["id"]] != vector["bits"]:
            raise AssertionError("bit-patterns/%s: the restatement answers %r, the committed vector says %r" % (vector["id"], produced[vector["id"]], vector["bits"]))
    return Outcome(produced)


def sweeps(ctx):
    """🌊️ Sines and cosines along every committed sweep."""
    vectors = committed(ctx)["sweeps"]
    return agree("sweeps", {vector["id"]: sweep(vector) for vector in vectors}, vectors)


def clamps(ctx):
    """🚧️ ``clamp(value, low, high)`` for every committed triple."""
    vectors = committed(ctx)["clamps"]
    return agree("clamps", {vector["id"]: clamp(vector["value"], vector["low"], vector["high"]) for vector in vectors}, vectors)


def lerps(ctx):
    """📏️ ``lerp(from, to, amount)`` for every committed triple."""
    vectors = committed(ctx)["lerps"]
    return agree("lerps", {vector["id"]: lerp(vector["from"], vector["to"], vector["amount"]) for vector in vectors}, vectors)


def smoothsteps(ctx):
    """🎢️ ``smoothstep(amount)`` for every committed amount."""
    vectors = committed(ctx)["smoothsteps"]
    return agree("smoothsteps", {vector["id"]: smoothstep(vector["amount"]) for vector in vectors}, vectors)


def referred(scenario, vectors, reference):
    """🪞️ Holds the numpy answer every vector carries beside its expectation to numpy's answer of this run."""
    for vector in vectors:
        if not close(reference(vector), vector["reference"]):
            raise AssertionError("%s/%s: numpy answers %r, the committed vector says %r" % (scenario, vector["id"], reference(vector), vector["reference"]))


def arctangent(vector):
    """🪃️ The judged direction of one committed point and its bit pattern."""
    turns = judged_direction("arctangents/%s" % vector["id"], vector["y"], vector["x"])
    return {"turns": turns, "bits": bits(turns)}


def arctangents(ctx):
    """🗼️ ``atanTurns(y, x)`` of every committed point, judged by ``numpy.arctan2``."""
    vectors = committed(ctx)["arctangents"]
    referred("arctangents", vectors, lambda vector: direction(vector["y"], vector["x"]))
    return agree("arctangents", {vector["id"]: arctangent(vector) for vector in vectors}, vectors)


def arctangent_grids(ctx):
    """🏁️ ``atanTurns`` over every committed lattice of points, judged by ``numpy.arctan2`` point by point."""
    vectors = committed(ctx)["arctangentGrids"]
    return agree("arctangent-grids", {vector["id"]: lattice(vector) for vector in vectors}, vectors)


def faded(vector):
    """🌫️ The judged decay of one committed argument and its bit pattern."""
    value = judged_decay("decays/%s" % vector["id"], vector["x"])
    return {"value": value, "bits": bits(value)}


def decays(ctx):
    """🧊️ ``fastNegExp(x)`` of every committed argument, judged by ``numpy.exp``."""
    vectors = committed(ctx)["decays"]
    referred("decays", vectors, lambda vector: decay(vector["x"]))
    return agree("decays", {vector["id"]: faded(vector) for vector in vectors}, vectors)


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter():
    """🔮️ Oracle role only: numpy is the reference, the TypeScript and Rust twins are judged against it."""
    return (
        Adapter("python")
        .oracle("angles", angles)
        .oracle("bit-patterns", bit_patterns)
        .oracle("sweeps", sweeps)
        .oracle("clamps", clamps)
        .oracle("lerps", lerps)
        .oracle("smoothsteps", smoothsteps)
        .oracle("arctangents", arctangents)
        .oracle("arctangent-grids", arctangent_grids)
        .oracle("decays", decays)
    )


# endregion 🔖️Registration

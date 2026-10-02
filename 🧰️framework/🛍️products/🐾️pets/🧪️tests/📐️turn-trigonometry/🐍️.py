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

@see https://numpy.org/doc/stable/reference/generated/numpy.sin.html
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


# endregion 🔖️Restatement


# region 🔖️Handlers
def committed(ctx):
    """🧫️ The committed vectors."""
    return json.loads(ctx.fixture_bytes(VECTORS))


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


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter():
    """🔮️ Oracle role only: numpy is the reference, the TypeScript and Rust twins are judged against it."""
    return Adapter("python").oracle("angles", angles).oracle("bit-patterns", bit_patterns).oracle("sweeps", sweeps).oracle("clamps", clamps).oracle("lerps", lerps).oracle("smoothsteps", smoothsteps)


# endregion 🔖️Registration

#!/usr/bin/env python3
"""🎞️ Oracle of the pets product's keyframe animation (design §4.4), in Python.

The easing of a key is the CSS cubic Bézier through (0, 0), (x1, y1), (x2, y2), (1, 1). The oracle writes
it in Bernstein form and lets ``scipy.optimize.brentq`` find the parameter at which the curve's ``x``
equals the phase — a root finder and a polynomial form the subjects do not use (they bisect a Horner
form 48 times). Linear tracks are sampled by ``numpy.interp`` over the whole track; tracks with eased
keys go through the same segment search (``numpy.searchsorted``) with the eased local phase, and the
segment arithmetic itself is held to ``numpy.interp`` on every track with its eases removed. The lid of
a blink is the piecewise cubic Hermite spline ``scipy.interpolate.CubicHermiteSpline`` through
(0, 0), (4, 1), (5, 1), (12, 0) with flat tangents. Clip lengths are rounded half up by ``numpy.floor``
and confirmed in exact rational arithmetic; poses are blended by numpy in the convex form
``(1 − a)·from + a·to``.

@see https://docs.scipy.org/doc/scipy/reference/generated/scipy.optimize.brentq.html
@see https://numpy.org/doc/stable/reference/generated/numpy.interp.html
@see https://docs.scipy.org/doc/scipy/reference/generated/scipy.interpolate.CubicHermiteSpline.html
@see https://www.w3.org/TR/css-easing-1/#cubic-bezier-easing-functions
@see ../../🧫️fixtures/🎞️animation-sampling/🔣️.json
"""

# region 🔖️Imports
import json
import math
from fractions import Fraction

import numpy
from scipy.interpolate import CubicHermiteSpline
from scipy.optimize import brentq

from semio_repo_test import Adapter, Outcome

# endregion 🔖️Imports


# region 🔖️Reference
VECTORS = "shared://🎞️animation-sampling/🔣️.json"
TICKS_PER_SECOND = 64
BLINK_TICKS = 12
CHANNELS = ["x", "y", "rotation", "scaleX", "scaleY"]
REST = {"x": 0.0, "y": 0.0, "rotation": 0.0, "scaleX": 1.0, "scaleY": 1.0}
LID = CubicHermiteSpline([0.0, 4.0, 5.0, 12.0], [0.0, 1.0, 1.0, 0.0], [0.0, 0.0, 0.0, 0.0])
AGREEMENT = 1e-12


def bezier(first, second, parameter):
    """🪜️ One coordinate of the cubic Bézier through 0, ``first``, ``second``, 1 in Bernstein form."""
    rest = 1.0 - parameter
    return 3.0 * rest * rest * parameter * first + 3.0 * rest * parameter * parameter * second + parameter * parameter * parameter


def ease_bezier(ease, amount):
    """🎢️ CSS ``cubic-bezier(x1, y1, x2, y2)`` at ``amount``: 0 at and below 0, 1 at and above 1, else ``y`` where brentq finds ``x = amount``."""
    if amount <= 0:
        return 0.0
    if amount >= 1:
        return 1.0
    x1, y1, x2, y2 = ease
    parameter = brentq(lambda value: bezier(x1, x2, value) - amount, 0.0, 1.0, xtol=1e-15, rtol=4 * numpy.finfo(float).eps, maxiter=500)
    return float(bezier(y1, y2, parameter))


def segment_value(track, phase, eased):
    """🧮️ The value of a track by segment search; ``eased`` switches the eases of its keys on."""
    keys = track["keys"]
    ats = numpy.array([key["at"] for key in keys], dtype=float)
    values = numpy.array([key["value"] for key in keys], dtype=float)
    if phase <= ats[0]:
        return float(values[0])
    if phase >= ats[-1]:
        return float(values[-1])
    index = int(numpy.searchsorted(ats, phase, side="right")) - 1
    local = (phase - ats[index]) / (ats[index + 1] - ats[index])
    if eased and "ease" in keys[index]:
        local = ease_bezier(keys[index]["ease"], local)
    return float(values[index] + (values[index + 1] - values[index]) * local)


def sample_track(track, phase):
    """🛤️ The value of a track at ``phase``: ``numpy.interp`` when no key is eased, the eased segment otherwise; the segment arithmetic must reproduce ``numpy.interp`` with the eases removed."""
    keys = track["keys"]
    linear = float(numpy.interp(phase, [key["at"] for key in keys], [key["value"] for key in keys]))
    plain = segment_value(track, phase, False)
    if not math.isclose(plain, linear, rel_tol=AGREEMENT, abs_tol=AGREEMENT):
        raise AssertionError("segment arithmetic answers %r where numpy.interp answers %r at phase %r" % (plain, linear, phase))
    if all("ease" not in key for key in keys):
        return linear
    return segment_value(track, phase, True)


def clip_ticks(seconds):
    """⏱️ ``floor(seconds × 64 + 0.5)``, at least 1 — confirmed as round-half-up of the exact rational product."""
    ticks = int(numpy.floor(numpy.float64(seconds) * TICKS_PER_SECOND + 0.5))
    exact = math.floor(Fraction(seconds) * TICKS_PER_SECOND + Fraction(1, 2))
    if ticks != exact:
        raise AssertionError("%r seconds round to %d ticks in floating point and to %d exactly" % (seconds, ticks, exact))
    return max(1, ticks)


def sample_clip(species, clip, ticks):
    """🎬️ The pose ``ticks`` after a clip began: looping clips wrap, others hold their last key; channels without a track stay at rest."""
    length = clip_ticks(clip["seconds"])
    elapsed = max(ticks, 0)
    phase = (int(numpy.mod(elapsed, length)) if clip["loop"] else min(elapsed, length)) / length
    index = {bone["id"]: position for position, bone in enumerate(species["bones"])}
    pose = [dict(REST) for _ in species["bones"]]
    for track in clip["tracks"]:
        if track["bone"] in index:
            pose[index[track["bone"]]][track["channel"]] = sample_track(track, phase)
    return pose


def blend_pose(start, end, amount):
    """🌗️ The pose ``amount`` of the way between two poses: the ends themselves outside (0, 1), numpy's convex combination inside."""
    if amount <= 0:
        return start
    if amount >= 1:
        return end
    matrix = (1.0 - amount) * numpy.array([[bone[channel] for channel in CHANNELS] for bone in start], dtype=float) + amount * numpy.array([[bone[channel] for channel in CHANNELS] for bone in end], dtype=float)
    return [{channel: float(value) for channel, value in zip(CHANNELS, row)} for row in matrix]


def lid_at(ticks):
    """👁️ The closure of a blink ``ticks`` after it began: the Hermite spline inside the blink, 0 outside."""
    if ticks <= 0 or ticks >= BLINK_TICKS:
        return 0.0
    return float(LID(ticks))


# endregion 🔖️Reference


# region 🔖️Handlers
def committed(ctx):
    """🧫️ The committed vectors."""
    return json.loads(ctx.input_bytes(VECTORS))


def close(produced, expected):
    """🔍️ Deep equality with floats compared within 1e-12 — a root finder may differ in the last bits between library builds."""
    if isinstance(produced, bool) or isinstance(expected, bool) or isinstance(produced, str) or isinstance(expected, str):
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


def easings(ctx):
    """📈️ ``easeBezier(ease, amount)`` for every committed easing and amount."""
    vectors = committed(ctx)["easings"]
    return agree("bezier-easings", {vector["id"]: [ease_bezier(vector["ease"], amount) for amount in vector["amounts"]] for vector in vectors}, vectors)


def tracks(ctx):
    """📉️ ``sampleTrack(track, phase)`` for every committed track and phase."""
    vectors = committed(ctx)["tracks"]
    return agree("track-samples", {vector["id"]: [sample_track(vector["track"], phase) for phase in vector["phases"]] for vector in vectors}, vectors)


def lengths(ctx):
    """📏️ ``clipTicks`` for every committed clip length in seconds."""
    vectors = committed(ctx)["lengths"]
    return agree("clip-lengths", {vector["id"]: clip_ticks(vector["seconds"]) for vector in vectors}, vectors)


def poses(ctx):
    """🤸️ ``sampleClip(species, clip, ticks)`` for every committed clip of the committed species at every committed tick."""
    document = committed(ctx)
    clips = {clip["id"]: clip for clip in document["species"]["clips"]}
    vectors = document["poses"]
    return agree("clip-poses", {vector["id"]: [sample_clip(document["species"], clips[vector["clip"]], ticks) for ticks in vector["ticks"]] for vector in vectors}, vectors)


def blends(ctx):
    """🎚️ ``blendPose(from, to, amount)`` for every committed pair of poses and amount."""
    vectors = committed(ctx)["blends"]
    return agree("pose-blends", {vector["id"]: [blend_pose(vector["from"], vector["to"], amount) for amount in vector["amounts"]] for vector in vectors}, vectors)


def lids(ctx):
    """😉️ ``BLINK_TICKS`` and ``lidAt(ticks)`` for every committed tick."""
    document = committed(ctx)["lids"]
    produced = {"blinkTicks": BLINK_TICKS, "closures": [lid_at(ticks) for ticks in document["ticks"]]}
    if not close(produced, document["expected"]):
        raise AssertionError("blink-lids: the reference answers %r, the committed vector says %r" % (produced, document["expected"]))
    return Outcome(produced)


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter():
    """🧭️ Oracle role only: scipy and numpy are the reference, the TypeScript and Rust twins are judged against them."""
    return Adapter("python").oracle("bezier-easings", easings).oracle("track-samples", tracks).oracle("clip-lengths", lengths).oracle("clip-poses", poses).oracle("pose-blends", blends).oracle("blink-lids", lids)


# endregion 🔖️Registration

#!/usr/bin/env python3
"""👀️ Oracle of where a pet's pupils are drawn towards (design §4.3 ``lookOffset``, §5.2 gaze), in Python.

The distance from the eye to what it looks at is ``numpy.linalg.norm`` of their difference; the offset
is the unit direction (the difference divided by that norm) scaled to the length ``d ÷ (d + reach)``,
and ``(0, 0)`` when the points coincide, so it always lies inside the unit disc. An eye that sits on a
posed bone is first carried into the pet's frame by the bone's matrix, lifted to numpy's 3×3
homogeneous form. The spring that moves the pupils towards this offset is specified by the
``🪀️spring-settling`` case.

One scenario is a supplement, not third-party evidence: ``bit-patterns`` restates ``lookOffset`` in
Python's IEEE doubles (with the restated ``transform`` of the ``🦴️rig-solving`` oracle) and projects the
64-bit pattern of every number, so the twins are held to each other and to this third implementation
bit for bit. Every restated answer is held to numpy's within 1e-12 before it is projected.

@see https://numpy.org/doc/stable/reference/generated/numpy.linalg.norm.html
@see ../🦴️rig-solving/🐍️.py
@see ../../🧫️fixtures/👀️gaze-tracking/🔣️.json
"""

# region 🔖️Imports
import importlib.util
import json
import math
import os

import numpy

from semio_repo_test import Adapter, Outcome

# endregion 🔖️Imports


# region 🔖️Reference
VECTORS = "shared://👀️gaze-tracking/🔣️.json"
TOLERANCE = 1e-12


def look_offset(eye, target, reach):
    """🔭️ The unit direction from ``eye`` to ``target`` scaled to ``d ÷ (d + reach)``; a reach that is not positive counts as 0."""
    difference = numpy.array([target["x"] - eye["x"], target["y"] - eye["y"]], dtype=numpy.float64)
    distance = float(numpy.linalg.norm(difference))
    if distance == 0.0:
        return {"x": 0.0, "y": 0.0}
    offset = difference / distance * (distance / (distance + max(float(reach), 0.0)))
    return {"x": float(offset[0]) + 0.0, "y": float(offset[1]) + 0.0}


def carried(affine, x, y):
    """📌️ The point ``matrix @ [x, y, 1]`` of the SVG matrix ``[a, b, c, d, e, f]``."""
    a, b, c, d, e, f = affine
    point = numpy.array([[a, c, e], [b, d, f], [0.0, 0.0, 1.0]], dtype=numpy.float64) @ numpy.array([x, y, 1.0], dtype=numpy.float64)
    return {"x": float(point[0]) + 0.0, "y": float(point[1]) + 0.0}


def eye_look(vector):
    """👁️ Where an eye on a posed bone sits in the pet's frame, and the offset of its pupil towards the target."""
    eye = carried(vector["bone"], vector["eye"]["x"], vector["eye"]["y"])
    return {"eye": eye, "offset": look_offset(eye, vector["target"], vector["reach"])}


# endregion 🔖️Reference


# region 🔖️Restatement
def neighbour(case):
    """🚪️ The oracle adapter of another case of this owner, loaded from its file."""
    spec = importlib.util.spec_from_file_location("pets_" + case.encode("ascii", "ignore").decode("ascii").replace("-", "_"), os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", case, "🐍️.py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


RIG = neighbour("🦴️rig-solving")


def restated_offset(eye, target, reach):
    """🪡️ ``lookOffset`` restated: the difference divided by the distance plus the reach, in the subject's order."""
    dx = float(target["x"]) - float(eye["x"])
    dy = float(target["y"]) - float(eye["y"])
    distance = math.sqrt(dx * dx + dy * dy)
    if distance == 0:
        return {"x": 0.0, "y": 0.0}
    span = distance + (float(reach) if reach > 0 else 0.0)
    return {"x": dx / span, "y": dy / span}


def restated_eye(vector):
    """🔍️ The eye carried by the restated ``transform`` and its restated offset."""
    eye = RIG.transform(vector["bone"], vector["eye"]["x"], vector["eye"]["y"])
    return {"eye": eye, "offset": restated_offset(eye, vector["target"], vector["reach"])}


# endregion 🔖️Restatement


# region 🔖️Handlers
def committed(ctx):
    """🧫️ The committed vectors."""
    return json.loads(ctx.input_bytes(VECTORS))


def close(produced, expected):
    """🤏️ Structural equality with a tolerance of 1e-12 (relative beyond 1) on numbers."""
    if isinstance(produced, (int, float)) and isinstance(expected, (int, float)):
        return abs(produced - expected) <= TOLERANCE * max(1.0, abs(expected))
    if isinstance(produced, dict) and isinstance(expected, dict):
        return produced.keys() == expected.keys() and all(close(produced[key], expected[key]) for key in produced)
    return produced == expected


def agree(scenario, produced, vectors):
    """⚖️ Holds every produced answer to the committed one — the vectors may never drift from the reference."""
    for vector in vectors:
        if not close(produced[vector["id"]], vector["expected"]):
            raise AssertionError("%s/%s: the reference answers %r, the committed vector says %r" % (scenario, vector["id"], produced[vector["id"]], vector["expected"]))
    return Outcome(produced)


def inside(scenario, identifier, offset):
    """⭕️ Refuses an offset that leaves the unit disc."""
    if float(numpy.linalg.norm(numpy.array([offset["x"], offset["y"]], dtype=numpy.float64))) > 1.0 + TOLERANCE:
        raise AssertionError("%s/%s: the offset %r lies outside the unit disc" % (scenario, identifier, offset))


def offsets(ctx):
    """🎯️ ``lookOffset(eye, target, reach)`` for every committed pair of points."""
    vectors = committed(ctx)["offsets"]
    produced = {vector["id"]: look_offset(vector["eye"], vector["target"], vector["reach"]) for vector in vectors}
    for vector in vectors:
        inside("offsets", vector["id"], produced[vector["id"]])
    return agree("offsets", produced, vectors)


def eyes(ctx):
    """🧿️ The carried eye and its offset for every committed eye on a posed bone."""
    vectors = committed(ctx)["eyes"]
    produced = {vector["id"]: eye_look(vector) for vector in vectors}
    for vector in vectors:
        inside("eyes", vector["id"], produced[vector["id"]]["offset"])
    return agree("eyes", produced, vectors)


def bit_patterns(ctx):
    """🧬️ The 64-bit patterns of every restated offset and carried eye, each held to numpy first and to its committed pattern exactly."""
    document = committed(ctx)
    produced = {
        "offsets": {vector["id"]: RIG.held("bit-patterns/offsets", vector["id"], restated_offset(vector["eye"], vector["target"], vector["reach"]), look_offset(vector["eye"], vector["target"], vector["reach"])) for vector in document["offsets"]},
        "eyes": {vector["id"]: RIG.held("bit-patterns/eyes", vector["id"], restated_eye(vector), eye_look(vector)) for vector in document["eyes"]},
    }
    for group, answers in produced.items():
        for vector in document[group]:
            if answers[vector["id"]] != vector["bits"]:
                raise AssertionError("bit-patterns/%s/%s: the restatement answers %r, the committed vector says %r" % (group, vector["id"], answers[vector["id"]], vector["bits"]))
    return Outcome(produced)


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter():
    """🔮️ Oracle role only: numpy is the reference, the TypeScript and Rust twins are judged against it."""
    return Adapter("python").oracle("offsets", offsets).oracle("eyes", eyes).oracle("bit-patterns", bit_patterns)


# endregion 🔖️Registration

#!/usr/bin/env python3
"""🦴️ Oracle of the pets rig (design §4.3): affine products, inverses, carried points and solved skeletons, in Python.

Every matrix ``[a, b, c, d, e, f]`` (SVG ``matrix(a b c d e f)``) is lifted to numpy's 3×3 homogeneous
form ``[[a, c, e], [b, d, f], [0, 0, 1]]``; products are numpy's ``@``, inverses ``numpy.linalg.inv``
(where numpy raises ``LinAlgError`` because the matrix is singular, the documented answer is the
identity), and a skeleton is the chain ``world = parent world @ translate @ rotate @ scale`` with the
rotation built from ``numpy.cos`` and ``numpy.sin`` of ``numpy.deg2rad`` of the authored degrees.
Nothing of the subject's expressions enters those answers.

One scenario is a supplement, not third-party evidence: ``bit-patterns`` restates the expressions of
design §4.3 in Python's IEEE doubles, term for term (with the restated sine and cosine of the
``📐️turn-trigonometry`` oracle), and projects the 64-bit pattern of every number, so the twins are held
to each other and to this third implementation bit for bit — the tolerance of the other scenarios
would hide a last-bit difference between languages. Every restated answer is held to numpy's within
1e-12 before it is projected.

@see https://numpy.org/doc/stable/reference/generated/numpy.linalg.inv.html
@see https://www.w3.org/TR/css-transforms-1/#mathematical-description
@see ../📐️turn-trigonometry/🐍️.py
@see ../../🧫️fixtures/🦴️rig-solving/🔣️.json
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
VECTORS = "shared://🦴️rig-solving/🔣️.json"
TOLERANCE = 1e-12
REST = {"x": 0, "y": 0, "rotation": 0, "scaleX": 1, "scaleY": 1}


def lifted(affine):
    """🔳️ The 3×3 homogeneous matrix of ``[a, b, c, d, e, f]``."""
    a, b, c, d, e, f = affine
    return numpy.array([[a, c, e], [b, d, f], [0.0, 0.0, 1.0]], dtype=numpy.float64)


def lowered(matrix):
    """🔻️ ``[a, b, c, d, e, f]`` of a 3×3 homogeneous matrix; a negative zero is written as zero."""
    return [float(value) + 0.0 for value in (matrix[0, 0], matrix[1, 0], matrix[0, 1], matrix[1, 1], matrix[0, 2], matrix[1, 2])]


def product(parent, local):
    """✖️ ``parent @ local``."""
    return lowered(lifted(parent) @ lifted(local))


def inverse(affine):
    """↩️ ``numpy.linalg.inv``; the identity where numpy finds the matrix singular."""
    try:
        return lowered(numpy.linalg.inv(lifted(affine)))
    except numpy.linalg.LinAlgError:
        return [1.0, 0.0, 0.0, 1.0, 0.0, 0.0]


def carried(affine, x, y):
    """📌️ The point ``matrix @ [x, y, 1]``."""
    point = lifted(affine) @ numpy.array([x, y, 1.0], dtype=numpy.float64)
    return {"x": float(point[0]) + 0.0, "y": float(point[1]) + 0.0}


def local_of(bone, posed):
    """🦿️ ``translate(bone + offset) @ rotate(rest + offset degrees) @ scale(factors)`` of one bone."""
    angle = numpy.deg2rad(numpy.float64(bone.get("rotation", 0) + posed["rotation"]))
    translate = numpy.array([[1.0, 0.0, bone["x"] + posed["x"]], [0.0, 1.0, bone["y"] + posed["y"]], [0.0, 0.0, 1.0]], dtype=numpy.float64)
    rotate = numpy.array([[numpy.cos(angle), -numpy.sin(angle), 0.0], [numpy.sin(angle), numpy.cos(angle), 0.0], [0.0, 0.0, 1.0]], dtype=numpy.float64)
    scale = numpy.diag(numpy.array([posed["scaleX"], posed["scaleY"], 1.0], dtype=numpy.float64))
    return translate @ rotate @ scale


def solved(species, pose):
    """🩻️ Six numbers per bone in rig order: every bone's world matrix relative to the feet origin."""
    worlds = {}
    numbers = []
    for index, bone in enumerate(species["bones"]):
        local = local_of(bone, pose[index] if index < len(pose) else REST)
        worlds[bone["id"]] = worlds[bone["parent"]] @ local if "parent" in bone else local
        numbers.extend(lowered(worlds[bone["id"]]))
    return numbers


def rest_pose(species):
    """🛌️ One rest entry per bone."""
    return [dict(REST) for _ in species["bones"]]


# endregion 🔖️Reference


# region 🔖️Restatement
def neighbour(case):
    """🚪️ The oracle adapter of another case of this owner, loaded from its file."""
    spec = importlib.util.spec_from_file_location("pets_" + case.encode("ascii", "ignore").decode("ascii").replace("-", "_"), os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", case, "🐍️.py"))
    module = importlib.util.module_from_spec(spec)
    spec.loader.exec_module(module)
    return module


TRIGONOMETRY = neighbour("📐️turn-trigonometry")
IDENTITY = [1.0, 0.0, 0.0, 1.0, 0.0, 0.0]


def doubles(values):
    """🫧️ The numbers as doubles: a JSON integer must multiply like the double the subjects read, signed zeros included."""
    return [float(value) for value in values]


def compose(parent, local):
    """🧷️ ``compose`` restated: ``parent × local``, six sums of products in the subject's order."""
    parent, local = doubles(parent), doubles(local)
    return [
        parent[0] * local[0] + parent[2] * local[1],
        parent[1] * local[0] + parent[3] * local[1],
        parent[0] * local[2] + parent[2] * local[3],
        parent[1] * local[2] + parent[3] * local[3],
        parent[0] * local[4] + parent[2] * local[5] + parent[4],
        parent[1] * local[4] + parent[3] * local[5] + parent[5],
    ]


def invert(matrix):
    """🔁️ ``invert`` restated: the adjugate divided by the determinant, the identity for a determinant of 0."""
    matrix = doubles(matrix)
    determinant = matrix[0] * matrix[3] - matrix[1] * matrix[2]
    if determinant == 0:
        return list(IDENTITY)
    return [
        matrix[3] / determinant,
        (0 - matrix[1]) / determinant,
        (0 - matrix[2]) / determinant,
        matrix[0] / determinant,
        (matrix[2] * matrix[5] - matrix[3] * matrix[4]) / determinant,
        (matrix[1] * matrix[4] - matrix[0] * matrix[5]) / determinant,
    ]


def transform(matrix, x, y):
    """🧲️ ``transform`` restated: the carried point."""
    matrix, x, y = doubles(matrix), float(x), float(y)
    return {"x": matrix[0] * x + matrix[2] * y + matrix[4], "y": matrix[1] * x + matrix[3] * y + matrix[5]}


def solve_rig(species, pose):
    """🦾️ ``solveRig`` restated: per bone the local matrix from the restated sine and cosine, multiplied onto the world matrix of its parent."""
    bones = species["bones"]
    world = []
    for index, bone in enumerate(bones):
        posed = pose[index] if index < len(pose) else REST
        turns = (float(bone.get("rotation", 0)) + float(posed["rotation"])) / 360
        sine = TRIGONOMETRY.sin_turns(turns)
        cosine = TRIGONOMETRY.cos_turns(turns)
        local = [cosine * float(posed["scaleX"]), sine * float(posed["scaleX"]), (0 - sine) * float(posed["scaleY"]), cosine * float(posed["scaleY"]), float(bone["x"]) + float(posed["x"]), float(bone["y"]) + float(posed["y"])]
        parent = next((candidate for candidate in range(index - 1, -1, -1) if bones[candidate]["id"] == bone.get("parent")), -1)
        world.extend(local if parent < 0 else compose(world[parent * 6 : parent * 6 + 6], local))
    return world


def hexed(value):
    """🧱️ The 64-bit patterns of a number, a point or a list of numbers."""
    if isinstance(value, dict):
        return {key: hexed(entry) for key, entry in value.items()}
    if isinstance(value, list):
        return [hexed(entry) for entry in value]
    return TRIGONOMETRY.bits(value)


def numbers(value):
    """🧺️ Every number of a point or a list, in order."""
    if isinstance(value, dict):
        return [number for entry in value.values() for number in numbers(entry)]
    if isinstance(value, list):
        return [number for entry in value for number in numbers(entry)]
    return [float(value)]


def held(scenario, identifier, restated, reference):
    """🪢️ Refuses a restated answer that is not finite or leaves numpy's by more than 1e-12 (relative beyond 1), then returns its bit patterns."""
    mine = numbers(restated)
    theirs = numbers(reference)
    if len(mine) != len(theirs) or any(not math.isfinite(left) or abs(left - right) > TOLERANCE * max(1.0, abs(right)) for left, right in zip(mine, theirs)):
        raise AssertionError("%s/%s: the restatement answers %r, numpy %r" % (scenario, identifier, restated, reference))
    return hexed(restated)


# endregion 🔖️Restatement


# region 🔖️Handlers
def committed(ctx):
    """🧫️ The committed vectors."""
    return json.loads(ctx.fixture_bytes(VECTORS))


def close(produced, expected):
    """🤏️ Structural equality with a tolerance of 1e-12 (relative beyond 1) on numbers: LAPACK and numpy's sine may differ by an ulp between builds."""
    if isinstance(produced, bool) or isinstance(expected, bool):
        return produced == expected
    if isinstance(produced, (int, float)) and isinstance(expected, (int, float)):
        return abs(produced - expected) <= TOLERANCE * max(1.0, abs(expected))
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


def products(ctx):
    """🧮️ ``compose(parent, local)`` for every committed pair."""
    vectors = committed(ctx)["products"]
    return agree("products", {vector["id"]: product(vector["parent"], vector["local"]) for vector in vectors}, vectors)


def inverses(ctx):
    """🔄️ ``invert(matrix)`` for every committed matrix; a product with the original that is not the identity is refused."""
    vectors = committed(ctx)["inverses"]
    produced = {vector["id"]: inverse(vector["matrix"]) for vector in vectors}
    for vector in vectors:
        if vector["singular"]:
            continue
        if not numpy.allclose(lifted(vector["matrix"]) @ lifted(produced[vector["id"]]), numpy.eye(3), rtol=0.0, atol=1e-9):
            raise AssertionError("inverses/%s: matrix @ inverse is not the identity" % vector["id"])
    return agree("inverses", produced, vectors)


def points(ctx):
    """📍️ ``transform(matrix, x, y)`` for every committed point."""
    vectors = committed(ctx)["points"]
    return agree("points", {vector["id"]: carried(vector["matrix"], vector["x"], vector["y"]) for vector in vectors}, vectors)


def rest_poses(ctx):
    """🧍️ ``restPose(species)`` and the skeleton it solves to, for every committed species."""
    document = committed(ctx)
    vectors = document["restPoses"]
    species = {entry["id"]: entry for entry in document["species"]}
    return agree("rest-poses", {vector["id"]: {"pose": rest_pose(species[vector["species"]]), "bones": solved(species[vector["species"]], rest_pose(species[vector["species"]]))} for vector in vectors}, vectors)


def skeletons(ctx):
    """💀️ ``solveRig(species, pose)`` for every committed pose."""
    document = committed(ctx)
    vectors = document["skeletons"]
    species = {entry["id"]: entry for entry in document["species"]}
    return agree("skeletons", {vector["id"]: solved(species[vector["species"]], vector["pose"]) for vector in vectors}, vectors)


def bit_patterns(ctx):
    """🧬️ The 64-bit patterns of every restated product, inverse, carried point and solved skeleton, each held to numpy first and to its committed pattern exactly."""
    document = committed(ctx)
    species = {entry["id"]: entry for entry in document["species"]}
    produced = {
        "products": {vector["id"]: held("bit-patterns/products", vector["id"], compose(vector["parent"], vector["local"]), product(vector["parent"], vector["local"])) for vector in document["products"]},
        "inverses": {vector["id"]: held("bit-patterns/inverses", vector["id"], invert(vector["matrix"]), inverse(vector["matrix"])) for vector in document["inverses"]},
        "points": {vector["id"]: held("bit-patterns/points", vector["id"], transform(vector["matrix"], vector["x"], vector["y"]), carried(vector["matrix"], vector["x"], vector["y"])) for vector in document["points"]},
        "skeletons": {vector["id"]: held("bit-patterns/skeletons", vector["id"], solve_rig(species[vector["species"]], vector["pose"]), solved(species[vector["species"]], vector["pose"])) for vector in document["skeletons"]},
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
    return Adapter("python").oracle("products", products).oracle("inverses", inverses).oracle("points", points).oracle("rest-poses", rest_poses).oracle("skeletons", skeletons).oracle("bit-patterns", bit_patterns)


# endregion 🔖️Registration

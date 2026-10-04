#!/usr/bin/env python3
"""🕸️ Oracle of the classification credit (design §6): exact hits, profile-similarity partial credit, in Python.

A wrong category earns ``max(0, 1 − d(assigned, correct)/d_max)`` when both categories carry a profile:
``d`` is the Euclidean distance over the task's axes in definition order of the values normalised to
``(v − min)/(max − min)``, and ``d_max`` the largest such distance between any two profiled categories of
the task. The credit is computed by the loops below, written from the design text, and every distance
and every ``d_max`` they use is recomputed by ``scipy.spatial.distance`` (``euclidean`` per pair, ``pdist``
for the maximum) and must agree before the result is projected. The credit is always taken from the
task's own axes and profiles, whatever the sheet task shows of them (challenge design §3.2 hides the
axis numbers and normalises the profiles where the keys are hidden). On a timed sheet task (one that
carries ``seconds``, challenge design §3.4) items may be left unassigned and the answer may be absent:
such an item earns 0 and carries no assigned category, and the score stays the mean over every sheet
item — recomputed with ``numpy.mean``.

@see https://docs.scipy.org/doc/scipy/reference/generated/scipy.spatial.distance.pdist.html
@see ../../🧫️fixtures/🕸️profile-similarity/🔣️.json
"""

# region 🔖️Imports
import json
import math

import numpy
from scipy.spatial import distance

from semio_repo_test import Adapter, Outcome

# endregion 🔖️Imports


# region 🔖️Reference
VECTORS = "shared://🕸️profile-similarity/🔣️.json"
TOLERANCE = 1e-12


def normalised(task, profile):
    """📐️ A profile as ``(v − min)/(max − min)`` per axis, in axis definition order."""
    return [(profile[axis["id"]] - axis["min"]) / (axis["max"] - axis["min"]) for axis in task.get("axes", [])]


def euclidean(first, second):
    """📏️ The Euclidean distance, summed in axis definition order."""
    total = 0.0
    for left, right in zip(first, second):
        difference = left - right
        total += difference * difference
    return math.sqrt(total)


def profile_of(task, category):
    """🧩️ A category's profile when it covers every axis of the task, else ``None`` — an incomplete profile is not profiled."""
    profile = category.get("profile")
    return profile if profile is not None and all(axis["id"] in profile for axis in task.get("axes", [])) else None


def largest_distance(task):
    """🔭️ ``d_max``: the largest distance between two profiled categories, pairs in definition order."""
    profiled = [category for category in task["categories"] if profile_of(task, category) is not None]
    largest = 0.0
    for first in range(len(profiled)):
        for second in range(first + 1, len(profiled)):
            largest = max(largest, euclidean(normalised(task, profiled[first]["profile"]), normalised(task, profiled[second]["profile"])))
    return largest


def credit(task, assigned, correct):
    """🎖️ One item's credit: 1 on a hit, profile similarity between two profiled categories, 0 otherwise."""
    if assigned == correct:
        return 1
    categories = {category["id"]: category for category in task["categories"]}
    if profile_of(task, categories[assigned]) is None or profile_of(task, categories[correct]) is None:
        return 0
    largest = largest_distance(task)
    if largest == 0:
        return 0
    return max(0, 1 - euclidean(normalised(task, categories[assigned]["profile"]), normalised(task, categories[correct]["profile"])) / largest)


def scorable(task, sheet_task, answer):
    """🚦️ Whether the answer can be scored for this resolved sheet task: valid and complete, or on a timed sheet task (one that carries ``seconds``) merely valid or absent — otherwise it scores none."""
    items = {item["id"] for item in sheet_task["items"]}
    categories = {category["id"] for category in task["categories"]}
    if not (task["kind"] == "classification" == sheet_task["kind"] and task["id"] == sheet_task["id"] and items <= {item["id"] for item in task["items"]}):
        return False
    if answer is None:
        return "seconds" in sheet_task
    return (
        answer["kind"] == "classification"
        and all(item in items and category in {category["id"] for category in sheet_task["categories"]} and category in categories for item, category in answer["assignments"].items())
        and ("seconds" in sheet_task or all(item in answer["assignments"] for item in items))
    )


def classification_result(task, sheet_task, answer):
    """🧷️ ``scoreTask`` for a classification task: items in sheet order, the score their mean credit (0 without items), or ``None``. An item left unassigned on a timed sheet task earns 0 and carries no assigned category."""
    if not scorable(task, sheet_task, answer):
        return None
    items = {item["id"]: item for item in task["items"]}
    assignments = answer["assignments"] if answer is not None else {}
    results = []
    total = 0.0
    for sheet_item in sheet_task["items"]:
        item = items[sheet_item["id"]]
        if item["id"] not in assignments:
            results.append({"item": item["id"], "correct": item["category"], "credit": 0, **({"explanation": item["explanation"]} if "explanation" in item else {})})
            continue
        earned = credit(task, assignments[item["id"]], item["category"])
        total += earned
        result = {"item": item["id"], "assigned": assignments[item["id"]], "correct": item["category"], "credit": earned}
        if "explanation" in item:
            result["explanation"] = item["explanation"]
        results.append(result)
    return {"kind": "classification", "task": task["id"], "score": total / len(results) if results else 0, "items": results}


# endregion 🔖️Reference


# region 🔖️ThirdParty
def corroborate(vector, task, answer):
    """🔍️ scipy must reproduce every distance and the ``d_max`` the credit loop relies on."""
    profiled = [category for category in task["categories"] if profile_of(task, category) is not None]
    if len(profiled) >= 2 and task.get("axes"):
        matrix = numpy.array([normalised(task, category["profile"]) for category in profiled], dtype=float)
        scipy_largest = float(distance.pdist(matrix, "euclidean").max())
        if abs(scipy_largest - largest_distance(task)) > TOLERANCE:
            raise AssertionError("%s: the reference d_max is %r, scipy's pdist says %r" % (vector, largest_distance(task), scipy_largest))
    categories = {category["id"]: category for category in task["categories"]}
    items = {item["id"]: item for item in task["items"]}
    for identifier, assigned in answer["assignments"].items():
        correct = items[identifier]["category"]
        if profile_of(task, categories[assigned]) is not None and profile_of(task, categories[correct]) is not None and task.get("axes"):
            left, right = normalised(task, categories[assigned]["profile"]), normalised(task, categories[correct]["profile"])
            if abs(distance.euclidean(left, right) - euclidean(left, right)) > TOLERANCE:
                raise AssertionError("%s/%s: the reference distance is %r, scipy's euclidean says %r" % (vector, identifier, euclidean(left, right), distance.euclidean(left, right)))


# endregion 🔖️ThirdParty


# region 🔖️Handlers
def close(produced, committed):
    """⚖️ Structural equality with the §6 parity tolerance on numbers."""
    if isinstance(produced, bool) or isinstance(committed, bool):
        return produced == committed
    if isinstance(produced, (int, float)) and isinstance(committed, (int, float)):
        return abs(produced - committed) <= TOLERANCE
    if isinstance(produced, dict) and isinstance(committed, dict):
        return produced.keys() == committed.keys() and all(close(produced[key], committed[key]) for key in produced)
    if isinstance(produced, list) and isinstance(committed, list):
        return len(produced) == len(committed) and all(close(left, right) for left, right in zip(produced, committed))
    return produced == committed


def credits(ctx):
    """🗃️ Every committed classification vector, credited, corroborated and held to its committed result."""
    vectors = json.loads(ctx.fixture_bytes(VECTORS))
    tasks = {task["id"]: task for task in vectors["tasks"]}
    produced = {}
    for vector in vectors["vectors"]:
        task = tasks[vector["task"]]
        corroborate(vector["id"], task, vector["answer"])
        produced[vector["id"]] = classification_result(task, vector["sheetTask"], vector["answer"])
        if not close(produced[vector["id"]], vector["expected"]):
            raise AssertionError("credits/%s: the reference result %r differs from the committed %r" % (vector["id"], produced[vector["id"]], vector["expected"]))
    return Outcome(produced)


def timed(ctx):
    """⏳️ Every committed answer to a timed sheet task — complete, partial or absent — credited and held to its committed result: an unassigned item earns 0 and names no category, and the score is numpy's mean over every sheet item."""
    vectors = json.loads(ctx.fixture_bytes(VECTORS))
    tasks = {task["id"]: task for task in vectors["tasks"]}
    produced = {}
    for vector in vectors["timed"]:
        task, answer = tasks[vector["task"]], vector.get("answer")
        if answer is not None:
            corroborate(vector["id"], task, answer)
        result = classification_result(task, vector["sheetTask"], answer)
        left = [entry["item"] for entry in result["items"] if "assigned" not in entry]
        if left != [item["id"] for item in vector["sheetTask"]["items"] if answer is None or item["id"] not in answer["assignments"]] or any(entry["credit"] != 0 for entry in result["items"] if "assigned" not in entry):
            raise AssertionError("timed/%s: the unassigned items %r do not all earn 0" % (vector["id"], left))
        if abs(float(numpy.mean([entry["credit"] for entry in result["items"]])) - result["score"]) > TOLERANCE:
            raise AssertionError("timed/%s: the score %r is not numpy's mean credit over every sheet item" % (vector["id"], result["score"]))
        produced[vector["id"]] = result
        if not close(result, vector["expected"]):
            raise AssertionError("timed/%s: the reference result %r differs from the committed %r" % (vector["id"], result, vector["expected"]))
    return Outcome(produced)


def degraded(ctx):
    """🩹️ Every committed input that bypasses validation degrades to the committed result or to none — never a throw, never NaN."""
    produced = {}
    for vector in json.loads(ctx.fixture_bytes(VECTORS))["degraded"]:
        produced[vector["id"]] = classification_result(vector["task"], vector["sheetTask"], vector.get("answer"))
        if not close(produced[vector["id"]], vector["expected"]):
            raise AssertionError("degraded/%s: the reference result %r differs from the committed %r" % (vector["id"], produced[vector["id"]], vector["expected"]))
    return Outcome(produced)


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter():
    """🧭️ Oracle role only."""
    return Adapter("python").oracle("credits", credits).oracle("timed", timed).oracle("degraded", degraded)


# endregion 🔖️Registration

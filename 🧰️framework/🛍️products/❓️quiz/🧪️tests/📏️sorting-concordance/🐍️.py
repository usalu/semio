#!/usr/bin/env python3
"""📏️ Oracle of the sorting score (design §6): magnitude-weighted pair concordance, in Python.

The score is computed three ways and they must agree before anything is projected:

* the pair loop of §6, written here from the design text (the projected answer);
* numpy, vectorised over the full pair matrix (``subtract.outer`` weights, ``greater.outer``
  discordance, numpy's own pairwise summation) — the same number by an unrelated evaluation order;
* scipy's Kendall τ for the unit-weight reading of the same engine: with ``w = 1`` for every pair of
  distinct values the score is ``(1 + τ_a)/2`` over the untied pairs, and ``τ_a`` follows from
  ``scipy.stats.kendalltau``'s ``τ_b`` and the tie count. This is the third-party check of the
  concordance direction; the magnitude weights themselves are this product's policy;
* scipy's Spearman ρ for equally spaced values: the magnitude weights are then rank differences, and
  the score of every order is ``(1 + ρ)/2`` (``scipy.stats.spearmanr``) — the rank reading of the same
  weighted engine, checked on its own vector group.

@see https://docs.scipy.org/doc/scipy/reference/generated/scipy.stats.kendalltau.html
@see https://docs.scipy.org/doc/scipy/reference/generated/scipy.stats.spearmanr.html
@see ../../🧫️fixtures/📏️sorting-concordance/🔣️.json
"""

# region 🔖️Imports
import json
import math

import numpy
from scipy import stats

from semio_repo_test import Adapter, Outcome

# endregion 🔖️Imports


# region 🔖️Reference
VECTORS = "shared://📏️sorting-concordance/🔣️.json"
TOLERANCE = 1e-12


def scaled(value, scale):
    """📐️ ``s(v)``: the value itself, or its decadic logarithm on a logarithmic scale."""
    return value if scale == "linear" else math.log10(value)


def concordance(values, weight):
    """⚖️ The §6 pair loop over values in the learner's order: ``1 − discordant/total``, or 1 without weight."""
    total = 0.0
    discordant = 0.0
    for first in range(len(values)):
        for second in range(first + 1, len(values)):
            pair = weight(values[first], values[second])
            total += pair
            if values[first] > values[second]:
                discordant += pair
    return 1 - discordant / total if total > 0 else 1


def scorable(task, sheet_task, answer):
    """🚦️ Whether §5 holds the answer valid for this resolved sheet task — a permutation of its items — otherwise it scores none."""
    sheet = [item["id"] for item in sheet_task["items"]]
    return answer["kind"] == "sorting" == task["kind"] == sheet_task["kind"] and task["id"] == sheet_task["id"] and set(sheet) <= {item["id"] for item in task["items"]} and sorted(answer["order"]) == sorted(sheet)


def sorting_result(task, sheet_task, answer):
    """🪜️ ``scoreTask`` for a sorting task: the score and the learner-ordered item results, or ``None``."""
    if not scorable(task, sheet_task, answer):
        return None
    items = {item["id"]: item for item in task["items"]}
    index = {item["id"]: position for position, item in enumerate(task["items"])}
    scale = task["quantity"]["scale"]
    values = [items[identifier]["value"] for identifier in answer["order"]]
    score = concordance(values, lambda first, second: abs(scaled(first, scale) - scaled(second, scale)))
    ranks = {identifier: rank for rank, identifier in enumerate(sorted((item["id"] for item in sheet_task["items"]), key=lambda identifier: (items[identifier]["value"], index[identifier])))}
    results = []
    for position, identifier in enumerate(answer["order"]):
        result = {"item": identifier, "value": items[identifier]["value"], "position": position, "rank": ranks[identifier]}
        if "explanation" in items[identifier]:
            result["explanation"] = items[identifier]["explanation"]
        results.append(result)
    return {"kind": "sorting", "task": task["id"], "score": score, "items": results}


# endregion 🔖️Reference


# region 🔖️ThirdParty
def numpy_score(values, scale):
    """🧮️ The weighted score over the whole pair matrix, summed by numpy."""
    array = numpy.array(values, dtype=float)
    transformed = array if scale == "linear" else numpy.log10(array)
    upper = numpy.triu(numpy.ones((len(values), len(values)), dtype=bool), 1)
    weights = numpy.abs(numpy.subtract.outer(transformed, transformed))[upper]
    discordant = numpy.greater.outer(array, array)[upper]
    total = weights.sum()
    return 1 - weights[discordant].sum() / total if total > 0 else 1


def kendall_score(values):
    """📊️ ``(1 + τ_a)/2`` over the pairs of distinct values, from scipy's ``τ_b`` and the tie count."""
    pairs = len(values) * (len(values) - 1) // 2
    tied = sum(count * (count - 1) // 2 for count in numpy.unique(numpy.array(values, dtype=float), return_counts=True)[1])
    if pairs - tied == 0:
        return 1
    tau_b = stats.kendalltau(numpy.arange(len(values)), numpy.array(values, dtype=float)).statistic
    return (1 + tau_b * math.sqrt(pairs / (pairs - tied))) / 2


def corroborate(vector, task, answer, result):
    """🔍️ numpy and scipy must reproduce what the pair loop says, or the reference is not trusted."""
    items = {item["id"]: item["value"] for item in task["items"]}
    values = [items[identifier] for identifier in answer["order"]]
    vectorised = numpy_score(values, task["quantity"]["scale"])
    if abs(vectorised - result["score"]) > TOLERANCE:
        raise AssertionError("%s: the pair loop scores %r, numpy's pair matrix %r" % (vector, result["score"], vectorised))
    unit = concordance(values, lambda first, second: 1.0 if first != second else 0.0)
    kendall = kendall_score(values)
    if abs(unit - kendall) > TOLERANCE:
        raise AssertionError("%s: the unit-weight pair loop scores %r, scipy's Kendall τ implies %r" % (vector, unit, kendall))


def spearman(vector, task, answer, result):
    """📈️ With equally spaced values the weighted score must be ``(1 + ρ)/2`` with scipy's Spearman ρ."""
    items = {item["id"]: item["value"] for item in task["items"]}
    values = [items[identifier] for identifier in answer["order"]]
    steps = numpy.diff(numpy.sort(numpy.array(values, dtype=float)))
    if task["quantity"]["scale"] != "linear" or len(values) < 2 or not numpy.allclose(steps, steps[0], rtol=0, atol=TOLERANCE) or steps[0] == 0:
        raise AssertionError("%s: a rank-weight vector needs distinct, equally spaced values on a linear scale, got %r" % (vector, values))
    rho = stats.spearmanr(numpy.arange(len(values)), numpy.array(values, dtype=float)).statistic if len(values) > 2 else (1.0 if values[0] < values[1] else -1.0)
    if abs(result["score"] - (1 + rho) / 2) > TOLERANCE:
        raise AssertionError("%s: the weighted score is %r, scipy's Spearman ρ implies %r" % (vector, result["score"], (1 + rho) / 2))


# endregion 🔖️ThirdParty


# region 🔖️Handlers
def close(produced, committed):
    """🤏️ Structural equality with the §6 parity tolerance on numbers."""
    if isinstance(produced, bool) or isinstance(committed, bool):
        return produced == committed
    if isinstance(produced, (int, float)) and isinstance(committed, (int, float)):
        return abs(produced - committed) <= TOLERANCE
    if isinstance(produced, dict) and isinstance(committed, dict):
        return produced.keys() == committed.keys() and all(close(produced[key], committed[key]) for key in produced)
    if isinstance(produced, list) and isinstance(committed, list):
        return len(produced) == len(committed) and all(close(left, right) for left, right in zip(produced, committed))
    return produced == committed


def scores(ctx):
    """🗃️ Every committed sorting vector, scored, corroborated and held to its committed result."""
    vectors = json.loads(ctx.fixture_bytes(VECTORS))
    tasks = {task["id"]: task for task in vectors["tasks"]}
    produced = {}
    for vector in vectors["vectors"]:
        task = tasks[vector["task"]]
        produced[vector["id"]] = sorting_result(task, vector["sheetTask"], vector["answer"])
        corroborate(vector["id"], task, vector["answer"], produced[vector["id"]])
        if not close(produced[vector["id"]], vector["expected"]):
            raise AssertionError("scores/%s: the reference result %r differs from the committed %r" % (vector["id"], produced[vector["id"]], vector["expected"]))
    return Outcome(produced)


def rank_weights(ctx):
    """📶️ Every committed equally spaced order, scored, checked against Spearman's ρ and held to its committed result."""
    vectors = json.loads(ctx.fixture_bytes(VECTORS))
    tasks = {task["id"]: task for task in vectors["tasks"]}
    produced = {}
    for vector in vectors["rankVectors"]:
        task = tasks[vector["task"]]
        produced[vector["id"]] = sorting_result(task, vector["sheetTask"], vector["answer"])
        corroborate(vector["id"], task, vector["answer"], produced[vector["id"]])
        spearman(vector["id"], task, vector["answer"], produced[vector["id"]])
        if not close(produced[vector["id"]], vector["expected"]):
            raise AssertionError("rank-weights/%s: the reference result %r differs from the committed %r" % (vector["id"], produced[vector["id"]], vector["expected"]))
    return Outcome(produced)


def degraded(ctx):
    """🩹️ Every committed input that bypasses validation degrades to the committed result or to none — never a throw, never NaN."""
    produced = {}
    for vector in json.loads(ctx.fixture_bytes(VECTORS))["degraded"]:
        produced[vector["id"]] = sorting_result(vector["task"], vector["sheetTask"], vector["answer"])
        if not close(produced[vector["id"]], vector["expected"]):
            raise AssertionError("degraded/%s: the reference result %r differs from the committed %r" % (vector["id"], produced[vector["id"]], vector["expected"]))
    return Outcome(produced)


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter():
    """🧭️ Oracle role only."""
    return Adapter("python").oracle("scores", scores).oracle("rank-weights", rank_weights).oracle("degraded", degraded)


# endregion 🔖️Registration

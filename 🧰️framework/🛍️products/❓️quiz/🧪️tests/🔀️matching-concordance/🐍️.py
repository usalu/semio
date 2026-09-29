#!/usr/bin/env python3
"""🔀️ Oracle of the matching score (design §6): per-dimension weighted concordance with half-credit ties, in Python.

The dimension score is computed by the pair loop of §6, written here from the design text, and
corroborated twice before it is projected:

* numpy over the full pair matrix — ``sign`` products for discordance, equality masks for the
  half-weighted ties the learner creates by picking two equal cards, numpy's own summation;
* scipy's Kendall τ for the unit-weight reading: with ``w = 1`` for every pair of distinct true values
  the dimension score is ``(C + T/2)/(C + D + T)``, i.e. ``(1 + τ_a)/2`` over the pairs the truth
  orders, and ``τ_a`` follows from ``scipy.stats.kendalltau``'s ``τ_b`` and the two tie counts.

@see https://docs.scipy.org/doc/scipy/reference/generated/scipy.stats.kendalltau.html
@see ../../🧫️fixtures/🔀️matching-concordance/🔣️.json
"""

# region 🔖️Imports
import json
import math

import numpy
from scipy import stats

from semio_repo_test import Adapter, Outcome

# endregion 🔖️Imports


# region 🔖️Reference
VECTORS = "shared://🔀️matching-concordance/🔣️.json"
TOLERANCE = 1e-12


def scaled(value, scale):
    """📐️ ``s(v)``: the value itself, or its decadic logarithm on a logarithmic scale."""
    return value if scale == "linear" else math.log10(value)


def sign(value):
    """➕️ The sign of a difference: −1, 0 or 1."""
    return (value > 0) - (value < 0)


def dimension_score(truth, assigned, weight):
    """⚖️ The §6 pair loop: discordant pairs cost their weight, ties the learner created cost half of it."""
    total = 0.0
    discordant = 0.0
    for first in range(len(truth)):
        for second in range(first + 1, len(truth)):
            pair = weight(truth[first], truth[second])
            total += pair
            if sign(truth[first] - truth[second]) * sign(assigned[first] - assigned[second]) < 0:
                discordant += pair
            elif assigned[first] == assigned[second] and truth[first] != truth[second]:
                discordant += pair / 2
    return 1 - discordant / total if total > 0 else 1


def scorable(task, sheet_task, answer):
    """🚦️ Whether §5 holds the answer valid and complete for this resolved sheet task — otherwise it scores none."""
    items = {item["id"] for item in sheet_task["items"]}
    cards = {dimension["id"]: len(dimension["cards"]) for dimension in sheet_task["dimensions"]}
    if answer["kind"] != "matching" or task["kind"] != "matching" or sheet_task["kind"] != "matching" or task["id"] != sheet_task["id"]:
        return False
    if not items <= {item["id"] for item in task["items"]} or not {dimension["id"] for dimension in task["dimensions"]} <= set(cards):
        return False
    for dimension, assignments in answer["assignments"].items():
        indices = list(assignments.values())
        if dimension not in cards or len(set(indices)) != len(indices) or any(item not in items for item in assignments) or any(not 0 <= index < cards[dimension] for index in indices):
            return False
    return all(dimension in answer["assignments"] and all(item in answer["assignments"][dimension] for item in items) for dimension in cards)


def matching_result(task, sheet_task, answer):
    """🧷️ ``scoreTask`` for a matching task: dimensions in definition order, items in sheet order, the score their mean (0 without dimensions), or ``None``."""
    if not scorable(task, sheet_task, answer):
        return None
    items = {item["id"]: item for item in task["items"]}
    order = [item["id"] for item in sheet_task["items"]]
    cards = {dimension["id"]: dimension["cards"] for dimension in sheet_task["dimensions"]}
    dimensions = []
    for dimension in task["dimensions"]:
        scale = dimension["quantity"]["scale"]
        truth = [items[identifier]["values"][dimension["id"]] for identifier in order]
        assigned = [cards[dimension["id"]][answer["assignments"][dimension["id"]][identifier]] for identifier in order]
        score = dimension_score(truth, assigned, lambda first, second: abs(scaled(first, scale) - scaled(second, scale)))
        results = []
        for identifier, value, correct in zip(order, assigned, truth):
            result = {"item": identifier, "assigned": value, "correct": correct}
            if "explanation" in items[identifier]:
                result["explanation"] = items[identifier]["explanation"]
            results.append(result)
        dimensions.append({"dimension": dimension["id"], "score": score, "items": results})
    total = 0.0
    for dimension in dimensions:
        total += dimension["score"]
    return {"kind": "matching", "task": task["id"], "score": total / len(dimensions) if dimensions else 0, "dimensions": dimensions}


# endregion 🔖️Reference


# region 🔖️ThirdParty
def numpy_score(truth, assigned, scale):
    """🧮️ The weighted dimension score over the whole pair matrix, summed by numpy."""
    true = numpy.array(truth, dtype=float)
    given = numpy.array(assigned, dtype=float)
    transformed = true if scale == "linear" else numpy.log10(true)
    upper = numpy.triu(numpy.ones((len(truth), len(truth)), dtype=bool), 1)
    weights = numpy.abs(numpy.subtract.outer(transformed, transformed))[upper]
    true_sign = numpy.sign(numpy.subtract.outer(true, true))[upper]
    given_sign = numpy.sign(numpy.subtract.outer(given, given))[upper]
    discordant = true_sign * given_sign < 0
    tied = (given_sign == 0) & (true_sign != 0)
    total = weights.sum()
    return 1 - (weights[discordant].sum() + weights[tied].sum() / 2) / total if total > 0 else 1


def tied_pairs(values):
    """🔗️ How many pairs of a sample share a value."""
    return int(sum(count * (count - 1) // 2 for count in numpy.unique(numpy.array(values, dtype=float), return_counts=True)[1]))


def kendall_score(truth, assigned):
    """📊️ ``(1 + τ_a)/2`` over the pairs the truth orders, from scipy's ``τ_b`` and both tie counts."""
    pairs = len(truth) * (len(truth) - 1) // 2
    truth_ties = tied_pairs(truth)
    assigned_ties = tied_pairs(assigned)
    if pairs - truth_ties == 0:
        return 1
    if pairs - assigned_ties == 0:
        return 0.5
    tau_b = stats.kendalltau(numpy.array(truth, dtype=float), numpy.array(assigned, dtype=float)).statistic
    return (1 + tau_b * math.sqrt((pairs - assigned_ties) / (pairs - truth_ties))) / 2


def corroborate(vector, task, sheet_task, answer, result):
    """🔍️ numpy and scipy must reproduce every dimension the pair loop scored."""
    items = {item["id"]: item for item in task["items"]}
    order = [item["id"] for item in sheet_task["items"]]
    cards = {dimension["id"]: dimension["cards"] for dimension in sheet_task["dimensions"]}
    for dimension, scored in zip(task["dimensions"], result["dimensions"]):
        truth = [items[identifier]["values"][dimension["id"]] for identifier in order]
        assigned = [cards[dimension["id"]][answer["assignments"][dimension["id"]][identifier]] for identifier in order]
        vectorised = numpy_score(truth, assigned, dimension["quantity"]["scale"])
        if abs(vectorised - scored["score"]) > TOLERANCE:
            raise AssertionError("%s/%s: the pair loop scores %r, numpy's pair matrix %r" % (vector, dimension["id"], scored["score"], vectorised))
        unit = dimension_score(truth, assigned, lambda first, second: 1.0 if first != second else 0.0)
        kendall = kendall_score(truth, assigned)
        if abs(unit - kendall) > TOLERANCE:
            raise AssertionError("%s/%s: the unit-weight pair loop scores %r, scipy's Kendall τ implies %r" % (vector, dimension["id"], unit, kendall))


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
    """🗃️ Every committed matching vector, scored, corroborated and held to its committed result."""
    vectors = json.loads(ctx.fixture_bytes(VECTORS))
    tasks = {task["id"]: task for task in vectors["tasks"]}
    produced = {}
    for vector in vectors["vectors"]:
        task = tasks[vector["task"]]
        produced[vector["id"]] = matching_result(task, vector["sheetTask"], vector["answer"])
        corroborate(vector["id"], task, vector["sheetTask"], vector["answer"], produced[vector["id"]])
        if not close(produced[vector["id"]], vector["expected"]):
            raise AssertionError("scores/%s: the reference result %r differs from the committed %r" % (vector["id"], produced[vector["id"]], vector["expected"]))
    return Outcome(produced)


def degraded(ctx):
    """🩹️ Every committed input that bypasses validation degrades to the committed result or to none — never a throw, never NaN."""
    produced = {}
    for vector in json.loads(ctx.fixture_bytes(VECTORS))["degraded"]:
        produced[vector["id"]] = matching_result(vector["task"], vector["sheetTask"], vector["answer"])
        if not close(produced[vector["id"]], vector["expected"]):
            raise AssertionError("degraded/%s: the reference result %r differs from the committed %r" % (vector["id"], produced[vector["id"]], vector["expected"]))
    return Outcome(produced)


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter():
    """🧭️ Oracle role only."""
    return Adapter("python").oracle("scores", scores).oracle("degraded", degraded)


# endregion 🔖️Registration

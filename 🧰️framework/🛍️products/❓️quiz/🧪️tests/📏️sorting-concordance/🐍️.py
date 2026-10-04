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

Where the sheet task hides its keys (challenge design §3.4) the answer is the learner's guesses: an item
misses without a guess or with a guess farther from its value than the reach of the presented values
(on a logarithmic scale a factor: ``sqrt(hi / lo)``, at most 1000 and widened by ``1e-9`` of itself, compared with ``max / min`` of guess
and value; on a linear scale a distance: half the spread), a pair is also discordant when either of its
items misses, and a task without weight scores 0 with a miss and 1 without. On a timed sheet task (one
that carries ``seconds``) the answer may be partly guessed or absent — absent, the items stand in sheet
order. numpy recomputes the reach (``ptp``, ``sqrt``, ``minimum``), the misses (``maximum`` over
``minimum``) and the score over the pair matrix, and an answer without a miss must score exactly what its
order scores where the keys show, which is the reading scipy's Kendall τ judges.

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
REACH_FACTOR = 1000
REACH_SLACK = 1e-9


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


def reach(values, scale):
    """🎯️ A factor on a logarithmic scale — ``sqrt(hi / lo)``, at most 1000, 1000 where the values do not spread — and a distance on a linear one — ``(hi − lo) / 2``, unbounded where they do not spread."""
    lo, hi = (min(values), max(values)) if values else (math.inf, -math.inf)
    if scale == "linear":
        return (hi - lo) / 2 if hi > lo else math.inf
    return min(float(REACH_FACTOR), math.sqrt(hi / lo)) if hi > lo else float(REACH_FACTOR)


def misses(value, truth, scale, bound):
    """🙈️ Whether a value lies off the truth by more than the reach widened by ``REACH_SLACK``: the larger over the smaller on a logarithmic scale, the distance on a linear one."""
    widened = bound * (1 + REACH_SLACK)
    if scale == "linear":
        return abs(value - truth) > widened
    return max(value, truth) / min(value, truth) > widened


def guessed_concordance(values, missed, weight):
    """🎲️ The pair loop where the keys are hidden: a pair is also discordant when either of its items misses; without weight a miss scores 0 and none 1."""
    total = 0.0
    discordant = 0.0
    for first in range(len(values)):
        for second in range(first + 1, len(values)):
            pair = weight(values[first], values[second])
            total += pair
            if values[first] > values[second] or missed[first] or missed[second]:
                discordant += pair
    return 1 - discordant / total if total > 0 else (0 if any(missed) else 1)


def guesses_fit(sheet_task, answer):
    """🔢️ Whether the guesses of an answer name sheet items only, are numbers (positive on a logarithmic quantity) and stand in non-decreasing order along the order."""
    guesses = answer.get("guesses", {})
    sheet = {item["id"] for item in sheet_task["items"]}
    logarithmic = sheet_task["quantity"]["scale"] == "logarithmic"
    if any(item not in sheet or isinstance(guess, bool) or not isinstance(guess, (int, float)) or not math.isfinite(guess) or (logarithmic and guess <= 0) for item, guess in guesses.items()):
        return False
    guessed = [guesses[item] for item in answer["order"] if item in guesses]
    return all(before <= after for before, after in zip(guessed, guessed[1:]))


def scorable(task, sheet_task, answer):
    """🚦️ Whether the answer can be scored for this resolved sheet task: valid — a permutation of its items, guesses exactly where the keys are hidden — and complete, or on a timed sheet task merely valid or absent; otherwise it scores none."""
    sheet = [item["id"] for item in sheet_task["items"]]
    if not (task["kind"] == "sorting" == sheet_task["kind"] and task["id"] == sheet_task["id"] and set(sheet) <= {item["id"] for item in task["items"]}):
        return False
    if answer is None:
        return "seconds" in sheet_task
    if answer["kind"] != "sorting" or sorted(answer["order"]) != sorted(sheet):
        return False
    if "keys" in sheet_task:
        return "guesses" not in answer
    return guesses_fit(sheet_task, answer) and ("seconds" in sheet_task or all(item in answer.get("guesses", {}) for item in sheet))


def sorting_result(task, sheet_task, answer):
    """🪜️ ``scoreTask`` for a sorting task: the score and the learner-ordered item results, or ``None``. Where the keys are hidden an item misses without a guess or with a guess beyond the reach of the presented values, and the results carry the guess and the miss; without an answer the items stand in sheet order."""
    if not scorable(task, sheet_task, answer):
        return None
    items = {item["id"]: item for item in task["items"]}
    index = {item["id"]: position for position, item in enumerate(task["items"])}
    scale = task["quantity"]["scale"]
    sheet = [item["id"] for item in sheet_task["items"]]
    order = answer["order"] if answer is not None else sheet
    values = [items[identifier]["value"] for identifier in order]
    weight = lambda first, second: abs(scaled(first, scale) - scaled(second, scale))
    hidden = "keys" not in sheet_task
    guesses = answer.get("guesses", {}) if answer is not None else {}
    if hidden or answer is None:
        bound = reach([items[identifier]["value"] for identifier in sheet], scale)
        missed = [answer is None or identifier not in guesses or misses(guesses[identifier], items[identifier]["value"], scale, bound) for identifier in order]
        score = guessed_concordance(values, missed, weight)
    else:
        score = concordance(values, weight)
    ranks = {identifier: rank for rank, identifier in enumerate(sorted(sheet, key=lambda identifier: (items[identifier]["value"], index[identifier])))}
    results = []
    for position, identifier in enumerate(order):
        result = {"item": identifier, "value": items[identifier]["value"], "position": position, "rank": ranks[identifier]}
        if hidden:
            if identifier in guesses:
                result["guess"] = guesses[identifier]
            result["miss"] = missed[position]
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


def numpy_guessed(task, sheet_task, answer):
    """🕶️ The guessed score and the misses over the whole pair matrix: numpy's ``ptp`` for the spread, ``sqrt`` and ``minimum`` for the reach, one vectorised ratio or distance comparison for the misses and a boolean pair matrix for the discordance."""
    items = {item["id"]: item["value"] for item in task["items"]}
    scale = task["quantity"]["scale"]
    transform = (lambda array: array) if scale == "linear" else numpy.log10
    order = answer["order"] if answer is not None else [item["id"] for item in sheet_task["items"]]
    guesses = answer.get("guesses", {}) if answer is not None else {}
    presented = numpy.array([items[item["id"]] for item in sheet_task["items"]], dtype=float)
    spread = numpy.ptp(presented) if len(presented) else numpy.float64(0)
    if scale == "linear":
        bound = float(spread / numpy.float64(2)) if spread > 0 else float(numpy.inf)
    else:
        bound = float(numpy.minimum(numpy.float64(REACH_FACTOR), numpy.sqrt(numpy.amax(presented) / numpy.amin(presented)))) if spread > 0 else float(REACH_FACTOR)
    values = numpy.array([items[identifier] for identifier in order], dtype=float)
    guessed = numpy.array([identifier in guesses for identifier in order], dtype=bool)
    given = numpy.array([guesses.get(identifier, items[identifier]) for identifier in order], dtype=float)
    off = numpy.abs(given - values) if scale == "linear" else numpy.maximum(given, values) / numpy.minimum(given, values)
    missed = ~guessed | (off > numpy.multiply(numpy.float64(bound), numpy.add(numpy.float64(1), numpy.float64(REACH_SLACK))))
    upper = numpy.triu(numpy.ones((len(order), len(order)), dtype=bool), 1)
    weights = numpy.abs(numpy.subtract.outer(transform(values), transform(values)))[upper]
    discordant = (numpy.greater.outer(values, values) | numpy.logical_or.outer(missed, missed))[upper]
    total = weights.sum()
    return (1 - weights[discordant].sum() / total if total > 0 else (0 if missed.any() else 1)), [bool(miss) for miss in missed]


def corroborate_guessed(vector, task, sheet_task, answer, result):
    """🔦️ numpy must reproduce the guessed score and every miss, and an answer without a miss must score what its order scores where the keys show — the reading scipy's Kendall τ judges."""
    vectorised, missed = numpy_guessed(task, sheet_task, answer)
    if abs(vectorised - result["score"]) > TOLERANCE or missed != [entry["miss"] for entry in result["items"]]:
        raise AssertionError("%s: the pair loop scores %r with misses %r, numpy's pair matrix %r with %r" % (vector, result["score"], [entry["miss"] for entry in result["items"]], vectorised, missed))
    if not any(missed):
        shown = {"kind": "sorting", "order": [entry["item"] for entry in result["items"]]}
        keyed = sorting_result(task, {**{member: value for member, value in sheet_task.items() if member != "seconds"}, "keys": sorted(entry["value"] for entry in result["items"])}, shown)
        corroborate(vector, task, shown, keyed)
        if abs(keyed["score"] - result["score"]) > TOLERANCE:
            raise AssertionError("%s: without a miss the guesses score %r, their order %r" % (vector, result["score"], keyed["score"]))


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


def guessed(ctx):
    """🔮️ Every committed answer to a sheet task that hides its keys — guessed, partly guessed or absent — scored, corroborated and held to its committed result."""
    vectors = json.loads(ctx.fixture_bytes(VECTORS))
    tasks = {task["id"]: task for task in vectors["tasks"]}
    produced = {}
    for vector in vectors["guessed"]:
        task = tasks[vector["task"]]
        produced[vector["id"]] = sorting_result(task, vector["sheetTask"], vector.get("answer"))
        corroborate_guessed(vector["id"], task, vector["sheetTask"], vector.get("answer"), produced[vector["id"]])
        if not close(produced[vector["id"]], vector["expected"]):
            raise AssertionError("guessed/%s: the reference result %r differs from the committed %r" % (vector["id"], produced[vector["id"]], vector["expected"]))
    return Outcome(produced)


def degraded(ctx):
    """🩹️ Every committed input that bypasses validation degrades to the committed result or to none — never a throw, never NaN."""
    produced = {}
    for vector in json.loads(ctx.fixture_bytes(VECTORS))["degraded"]:
        produced[vector["id"]] = sorting_result(vector["task"], vector["sheetTask"], vector.get("answer"))
        if not close(produced[vector["id"]], vector["expected"]):
            raise AssertionError("degraded/%s: the reference result %r differs from the committed %r" % (vector["id"], produced[vector["id"]], vector["expected"]))
    return Outcome(produced)


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter():
    """🧭️ Oracle role only."""
    return Adapter("python").oracle("scores", scores).oracle("rank-weights", rank_weights).oracle("guessed", guessed).oracle("degraded", degraded)


# endregion 🔖️Registration

#!/usr/bin/env python3
"""🔀️ Oracle of the matching score (design §6): per-dimension weighted concordance with half-credit ties, in Python.

The dimension score is computed by the pair loop of §6, written here from the design text, and
corroborated twice before it is projected:

* numpy over the full pair matrix — ``sign`` products for discordance, equality masks for the
  half-weighted ties the learner creates by picking two equal cards, numpy's own summation;
* scipy's Kendall τ for the unit-weight reading: with ``w = 1`` for every pair of distinct true values
  the dimension score is ``(C + T/2)/(C + D + T)``, i.e. ``(1 + τ_a)/2`` over the pairs the truth
  orders, and ``τ_a`` follows from ``scipy.stats.kendalltau``'s ``τ_b`` and the two tie counts.

Where the sheet task hides its cards (challenge design §3.4) the assigned value of an item is the
learner's guess: an item misses without a guess or with a guess farther from its true value than the
reach of the presented true values of the dimension (on a logarithmic scale a factor: ``sqrt(hi / lo)``,
at most 1000 and widened by ``1e-9`` of itself, compared with ``max / min`` of guess and value; on a linear scale a distance: half the
spread); a pair costs its whole weight when either of its items misses, else as before, and a dimension
without weight scores 0 with a miss and 1 without. On a timed sheet task (one that carries ``seconds``)
items may be left out, also where the cards show: a left-out item misses. numpy recomputes the reach
(``ptp``, ``sqrt``, ``minimum``), the misses (``maximum`` over ``minimum``) and the score over the pair
matrix, and guesses without a miss must score what cards of the same values score — the reading
scipy's Kendall τ judges.

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
REACH_FACTOR = 1000
REACH_SLACK = 1e-9


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


def missed_score(truth, assigned, missed, weight):
    """🎲️ The pair loop with misses: a pair costs its whole weight when either of its items misses, else as the plain loop; without weight a miss scores 0 and none 1."""
    total = 0.0
    discordant = 0.0
    for first in range(len(truth)):
        for second in range(first + 1, len(truth)):
            pair = weight(truth[first], truth[second])
            total += pair
            if missed[first] or missed[second]:
                discordant += pair
            elif sign(truth[first] - truth[second]) * sign(assigned[first] - assigned[second]) < 0:
                discordant += pair
            elif assigned[first] == assigned[second] and truth[first] != truth[second]:
                discordant += pair / 2
    return 1 - discordant / total if total > 0 else (0 if any(missed) else 1)


def keys_hidden(sheet_task):
    """🫣️ Whether the sheet task hides its keys: a dimension without cards."""
    return any("cards" not in dimension for dimension in sheet_task["dimensions"])


def valid(sheet_task, answer):
    """🛂️ Whether the answer fits the sheet task: assignments exactly where the cards show, each index in range and used once; guesses exactly where they are hidden, numbers over sheet dimensions and items, positive on a logarithmic quantity."""
    items = {item["id"] for item in sheet_task["items"]}
    dimensions = {dimension["id"]: dimension for dimension in sheet_task["dimensions"]}
    if answer["kind"] != "matching":
        return False
    if keys_hidden(sheet_task):
        if "assignments" in answer:
            return False
        for dimension, guesses in answer.get("guesses", {}).items():
            if dimension not in dimensions or any(item not in items or isinstance(guess, bool) or not isinstance(guess, (int, float)) or not math.isfinite(guess) or (dimensions[dimension]["quantity"]["scale"] == "logarithmic" and guess <= 0) for item, guess in guesses.items()):
                return False
        return True
    if "guesses" in answer or "assignments" not in answer:
        return False
    for dimension, assignments in answer["assignments"].items():
        indices = list(assignments.values())
        if dimension not in dimensions or len(set(indices)) != len(indices) or any(item not in items for item in assignments) or any(not 0 <= index < len(dimensions[dimension]["cards"]) for index in indices):
            return False
    return True


def scorable(task, sheet_task, answer):
    """🚦️ Whether the answer can be scored for this resolved sheet task: valid and complete, or on a timed sheet task merely valid or absent — otherwise it scores none."""
    if task["kind"] != "matching" or sheet_task["kind"] != "matching" or task["id"] != sheet_task["id"]:
        return False
    items = {item["id"] for item in sheet_task["items"]}
    if not items <= {item["id"] for item in task["items"]} or not {dimension["id"] for dimension in task["dimensions"]} <= {dimension["id"] for dimension in sheet_task["dimensions"]}:
        return False
    if answer is None:
        return "seconds" in sheet_task
    if not valid(sheet_task, answer):
        return False
    chosen = answer.get("guesses" if keys_hidden(sheet_task) else "assignments", {})
    return "seconds" in sheet_task or all(dimension["id"] in chosen and all(item in chosen[dimension["id"]] for item in items) for dimension in sheet_task["dimensions"])


def matching_result(task, sheet_task, answer):
    """🧷️ ``scoreTask`` for a matching task: dimensions in definition order, items in sheet order, the score their mean (0 without dimensions), or ``None``. Where the cards are hidden the assigned value is the guess, an item misses without a guess or with one beyond the reach of the presented true values, and every item result says whether it misses; an item left unassigned on a timed sheet task misses too."""
    if not scorable(task, sheet_task, answer):
        return None
    items = {item["id"]: item for item in task["items"]}
    order = [item["id"] for item in sheet_task["items"]]
    presented = {dimension["id"]: dimension for dimension in sheet_task["dimensions"]}
    hidden = keys_hidden(sheet_task)
    chosen = {} if answer is None else answer.get("guesses" if hidden else "assignments", {})
    dimensions = []
    for dimension in task["dimensions"]:
        scale = dimension["quantity"]["scale"]
        weight = lambda first, second: abs(scaled(first, scale) - scaled(second, scale))
        truth = [items[identifier]["values"][dimension["id"]] for identifier in order]
        given = chosen.get(dimension["id"], {})
        assigned = [(given[identifier] if hidden else presented[dimension["id"]]["cards"][given[identifier]]) if identifier in given else None for identifier in order]
        bound = reach(truth, scale)
        missed = [value is None or (hidden and misses(value, correct, scale, bound)) for value, correct in zip(assigned, truth)]
        score = missed_score(truth, assigned, missed, weight) if hidden or any(missed) else dimension_score(truth, assigned, weight)
        results = []
        for identifier, value, correct, miss in zip(order, assigned, truth, missed):
            result = {"item": identifier}
            if value is not None:
                result["assigned"] = value
            result["correct"] = correct
            if hidden:
                result["miss"] = miss
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


def numpy_missed(task, sheet_task, answer, dimension):
    """🕶️ One dimension where items may miss, over the whole pair matrix: numpy's ``ptp`` for the spread, ``sqrt`` and ``minimum`` for the reach, one vectorised ratio or distance comparison for the misses and boolean pair matrices for the discordance and the ties."""
    items = {item["id"]: item for item in task["items"]}
    order = [item["id"] for item in sheet_task["items"]]
    scale = dimension["quantity"]["scale"]
    transform = (lambda array: array) if scale == "linear" else numpy.log10
    hidden = keys_hidden(sheet_task)
    given = {} if answer is None else answer.get("guesses" if hidden else "assignments", {}).get(dimension["id"], {})
    cards = next(entry for entry in sheet_task["dimensions"] if entry["id"] == dimension["id"]).get("cards", [])
    true = numpy.array([items[identifier]["values"][dimension["id"]] for identifier in order], dtype=float)
    answered = numpy.array([identifier in given for identifier in order], dtype=bool)
    assigned = numpy.array([(given[identifier] if hidden else cards[given[identifier]]) if identifier in given else items[identifier]["values"][dimension["id"]] for identifier in order], dtype=float)
    spread = numpy.ptp(true) if len(true) else numpy.float64(0)
    if scale == "linear":
        bound = float(spread / numpy.float64(2)) if spread > 0 else float(numpy.inf)
    else:
        bound = float(numpy.minimum(numpy.float64(REACH_FACTOR), numpy.sqrt(numpy.amax(true) / numpy.amin(true)))) if spread > 0 else float(REACH_FACTOR)
    off = numpy.abs(assigned - true) if scale == "linear" else numpy.maximum(assigned, true) / numpy.minimum(assigned, true)
    missed = ~answered | (off > numpy.multiply(numpy.float64(bound), numpy.add(numpy.float64(1), numpy.float64(REACH_SLACK))) if hidden else numpy.zeros(len(order), dtype=bool))
    upper = numpy.triu(numpy.ones((len(order), len(order)), dtype=bool), 1)
    weights = numpy.abs(numpy.subtract.outer(transform(true), transform(true)))[upper]
    true_sign = numpy.sign(numpy.subtract.outer(true, true))[upper]
    given_sign = numpy.sign(numpy.subtract.outer(assigned, assigned))[upper]
    lost = numpy.logical_or.outer(missed, missed)[upper]
    discordant = lost | (true_sign * given_sign < 0)
    tied = ~lost & (given_sign == 0) & (true_sign != 0)
    total = weights.sum()
    return (1 - (weights[discordant].sum() + weights[tied].sum() / 2) / total if total > 0 else (0 if missed.any() else 1)), [bool(miss) for miss in missed]


def corroborate_missed(vector, task, sheet_task, answer, result):
    """🔦️ numpy must reproduce every dimension score and every miss of an answer that guesses or leaves items out."""
    for dimension, scored in zip(task["dimensions"], result["dimensions"]):
        vectorised, missed = numpy_missed(task, sheet_task, answer, dimension)
        found = [entry.get("miss", "assigned" not in entry) for entry in scored["items"]]
        if abs(vectorised - scored["score"]) > TOLERANCE or missed != found:
            raise AssertionError("%s/%s: the pair loop scores %r with misses %r, numpy's pair matrix %r with %r" % (vector, dimension["id"], scored["score"], found, vectorised, missed))
        if not any(missed):
            truth = [entry["correct"] for entry in scored["items"]]
            assigned = [entry["assigned"] for entry in scored["items"]]
            unit = dimension_score(truth, assigned, lambda first, second: 1.0 if first != second else 0.0)
            if abs(unit - kendall_score(truth, assigned)) > TOLERANCE or abs(numpy_score(truth, assigned, dimension["quantity"]["scale"]) - scored["score"]) > TOLERANCE:
                raise AssertionError("%s/%s: without a miss the guesses must score as cards of their values do" % (vector, dimension["id"]))


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


def guessed(ctx):
    """🔮️ Every committed answer that guesses where the cards are hidden, or leaves items out on a timed sheet task, scored, corroborated and held to its committed result."""
    vectors = json.loads(ctx.fixture_bytes(VECTORS))
    tasks = {task["id"]: task for task in vectors["tasks"]}
    produced = {}
    for vector in vectors["guessed"]:
        task = tasks[vector["task"]]
        produced[vector["id"]] = matching_result(task, vector["sheetTask"], vector.get("answer"))
        corroborate_missed(vector["id"], task, vector["sheetTask"], vector.get("answer"), produced[vector["id"]])
        if not close(produced[vector["id"]], vector["expected"]):
            raise AssertionError("guessed/%s: the reference result %r differs from the committed %r" % (vector["id"], produced[vector["id"]], vector["expected"]))
    return Outcome(produced)


def degraded(ctx):
    """🩹️ Every committed input that bypasses validation degrades to the committed result or to none — never a throw, never NaN."""
    produced = {}
    for vector in json.loads(ctx.fixture_bytes(VECTORS))["degraded"]:
        produced[vector["id"]] = matching_result(vector["task"], vector["sheetTask"], vector.get("answer"))
        if not close(produced[vector["id"]], vector["expected"]):
            raise AssertionError("degraded/%s: the reference result %r differs from the committed %r" % (vector["id"], produced[vector["id"]], vector["expected"]))
    return Outcome(produced)


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter():
    """🧭️ Oracle role only."""
    return Adapter("python").oracle("scores", scores).oracle("guessed", guessed).oracle("degraded", degraded)


# endregion 🔖️Registration

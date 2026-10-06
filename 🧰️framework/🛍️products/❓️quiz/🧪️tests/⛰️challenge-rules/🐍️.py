#!/usr/bin/env python3
"""⛰️ Second implementation of the challenge rules (challenge design §3.1), in Python, corroborated by numpy.

Written from the design text alone — never from the TypeScript or Rust twins. A run is played at one of
four challenges, each the one before plus one step: ``easy`` shows the keys and hints, ``medium`` shows
the keys, ``hard`` hides them, ``expert`` hides them and runs every task against a clock; a run earns
``points = score × par`` with par 100, 200, 300 and 400. A value **misses** when it lies farther from the
truth than the **reach** of its set. On a logarithmic scale the reach is a factor — the square root of the
presented true values' max/min ratio, at most 1000, and 1000 itself where they do not spread — and a value
misses when the larger of value and truth divided by the smaller exceeds it. On a linear scale the reach is
a distance — half the spread, unbounded where the values do not spread — and a value misses when it lies
farther than that. Either reach is first widened by the relative slack ``1e-9`` (``reach × (1 + 1e-9)``), so a
decimal typed at exactly the reach — ``0.018`` against ``18`` — stays within. Only division, square root,
multiplication, addition, subtraction, absolute value, minimum, maximum and comparison enter, all exactly
rounded, so an exact factor of 1000 never misses. A timed task allows
``30 + per(kind) × items`` seconds (``× dimensions`` for a matching), the instant a learner acted is lowered
to five minutes (``300000`` ms) past the decider's clock and then raised to its floor.

A hint (challenge design §8) is a pure function of the task, its sheet task and the answer, and always questions
one concrete relation the learner's own answer claims between two items. A sorting item whose key on the ladder
misses its value, or a matching item whose card misses it, is compared with a reference item of the same task
(and dimension) that holds a key too: preferably one whose own key does not miss (an anchor), else any; among
those whose claimed relation to it is the most wrong (within the slack), one no earlier kept hint of the task
(and dimension) names yet, then a familiar item, then the smallest claim, then the first in sheet order. The claim is the ratio of the two keys on a logarithmic scale
(``factor``) or their difference on a linear one (``difference``), and the ``verdict`` says whether the claim
points to one side of 1 (of 0) and the truth to none or the other (``reversed``; a claim of exactly 1 or 0 points
nowhere and is never reversed), else whether the truth lies further out on the same side (``under``) or not (``over``). A misplaced classification item whose assigned and own categories both
carry profiles is questioned on the axis where the two profiles lie farthest apart relative to that axis's reach
(half the spread of the presented categories' values on it) — only when some axis lies beyond its widened reach —,
against an item placed in its own category whose value on that axis lies between the two (``other``, the side
the placement claims ``above``) where there is one; any other misplaced item is questioned together with an item
the learner put in the same category although it belongs elsewhere, else apart from an item of its own category
the learner put elsewhere, else against the assigned category itself. A task gives at most three hints, kept in
their order: compare hints by their largest error and profile hints by their relative gap, descending, first,
group and category hints after them; the references of the kept compare hints are chosen afterwards in that order.

Before anything is projected numpy recomputes every number by an unrelated route: the extremes with
``numpy.ptp`` and ``numpy.amin``, the reach with ``numpy.sqrt`` and ``numpy.minimum``, the misses by
vectorised ratios (``numpy.maximum`` over ``numpy.minimum``) or distances, the lowered and raised instant
with ``numpy.minimum`` and ``numpy.maximum`` over unsigned 64-bit integers, the seconds with ``numpy.prod``,
the ladder of keys with ``numpy.sort`` and the points with the exact rational product of ``fractions.Fraction``
rounded once. The hints are recomputed as arrays over the sheet order: every candidate's claimed and true
ratio or difference at once, the error by ``numpy.maximum`` over ``numpy.minimum`` or ``numpy.abs``, the verdict
by ``numpy.sign`` and boolean masks, the pool, the tie window, the references named so far (``numpy.isin``) and the
familiar items by boolean masks, the reference by ``numpy.argmin`` (whose first minimum is the first in sheet order), the profile reaches by
``numpy.ptp`` per axis, the profile anchor by ``numpy.argmax`` over masked distances, the group partners by
``numpy.flatnonzero`` and the cap by ``numpy.lexsort``, which is stable.
Every logarithmic reach below the cap is also held to exact rational arithmetic: it must be ``sqrt(hi / lo)``
correctly rounded, i.e. the squares of the midpoints to its neighbouring doubles must bracket ``hi / lo``.

@see ../../🧬️schema/🔣️.json
@see ../../🧫️fixtures/⛰️challenge-rules/🔣️.json
@see https://numpy.org/doc/stable/reference/generated/numpy.sqrt.html
@see https://numpy.org/doc/stable/reference/generated/numpy.maximum.html
@see https://numpy.org/doc/stable/reference/generated/numpy.argmax.html
"""

# region 🔖️Imports
import json
import math
from fractions import Fraction

import numpy

from semio_repo_test import Adapter, Outcome

# endregion 🔖️Imports


# region 🔖️Reference
VECTORS = "shared://⛰️challenge-rules/🔣️.json"
TOLERANCE = 1e-12
CHALLENGES = ["easy", "medium", "hard", "expert"]
CHALLENGE_RULES = {
    "easy": {"keys": True, "hints": True, "timed": False, "par": 100},
    "medium": {"keys": True, "hints": False, "timed": False, "par": 200},
    "hard": {"keys": False, "hints": False, "timed": False, "par": 300},
    "expert": {"keys": False, "hints": False, "timed": True, "par": 400},
}
REACH_FACTOR = 1000
REACH_SLACK = 1e-9
TASK_SECONDS = {"base": 30, "classification": 8, "sorting": 12, "matching": 12}
CLOCK_LEAD = 300000


def challenge_rank(challenge):
    """🪜️ The place of a challenge among the four, 0 the easiest."""
    return CHALLENGES.index(challenge)


def challenge_meets(challenge, least):
    """🚧️ Whether a challenge is at least as demanding as ``least``."""
    return challenge_rank(challenge) >= challenge_rank(least)


def points(score, challenge):
    """💯️ ``score × par`` of the challenge."""
    return score * CHALLENGE_RULES[challenge]["par"]


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


def task_seconds(kind, items, dimensions):
    """⏳️ The seconds a timed task allows: the base plus the kind's seconds per item, per dimension for a matching."""
    return TASK_SECONDS["base"] + TASK_SECONDS[kind] * items * (dimensions if kind == "matching" else 1)


def acted(at, floor, now):
    """⏱️ The instant a learner acted, lowered to the lead past the decider's clock, then raised to its floor."""
    return max(min(at, now + CLOCK_LEAD), floor)


HINTS_PER_TASK = 3


def verdict(scale, claimed, true):
    """⚖️ Where the truth lies against the claim, around the pivot 1 (logarithmic) or 0 (linear): ``reversed`` when the claim lies off the pivot and the truth at it or on its other side — a claim at the pivot (equal keys) names no direction and is never reversed —, else ``under`` when the truth lies beyond the claim, further from the pivot, else ``over``."""
    pivot = 0 if scale == "linear" else 1
    if (claimed > pivot and true <= pivot) or (claimed < pivot and true >= pivot):
        return "reversed"
    return "under" if (claimed >= pivot and true > claimed) or (claimed < pivot and true < claimed) else "over"


def relation(scale, key, truth, other_key, other_truth):
    """📐️ How wrong the relation of two items that the keys claim is, its verdict, and the claim as the hint carries it: the ratio of the keys on a logarithmic scale, their difference on a linear one."""
    if scale == "linear":
        claimed, true = key - other_key, truth - other_truth
        return abs(claimed - true), verdict(scale, claimed, true), {"difference": claimed}
    claimed, true = key / other_key, truth / other_truth
    return max(claimed, true) / min(claimed, true), verdict(scale, claimed, true), {"factor": claimed}


def oriented(claim):
    """🪃️ How far a claim lies from no relation at all: ``max(ρ, 1/ρ)`` for a factor, ``|δ|`` for a difference."""
    if "difference" in claim:
        return abs(claim["difference"])
    return max(claim["factor"], 1 / claim["factor"])


def compare_hints(hinted, keyed, scale, bound, familiar, dimension=None):
    """🔁️ Per hinted item whose key misses, in the given order, its relations to the other keyed items that are the most wrong, weighted by the largest error — anchors (keys that do not miss) first, every keyed item where none is left; errors within the relative ``REACH_SLACK`` of the largest tie, kept in sheet order with whether the item is familiar, for ``referenced`` to choose from after the cap; ``keyed`` lists ``(item, key, truth)`` in sheet order, ``familiar`` the items flagged so."""
    missing = {item for item, key, truth in keyed if misses(key, truth, scale, bound)}
    hints = []
    for item, key, truth in hinted:
        if item not in missing:
            continue
        candidates = [entry for entry in keyed if entry[0] != item]
        pool = [entry for entry in candidates if entry[0] not in missing] or candidates
        if not pool:
            continue
        related = [(other, *relation(scale, key, truth, other_key, other_truth), other in familiar) for other, other_key, other_truth in pool]
        largest = max(entry[1] for entry in related)
        hints.append((largest, {"item": item, "dimension": dimension, "tied": [entry for entry in related if entry[1] * (1 + REACH_SLACK) >= largest]}))
    return hints


def referenced(kept):
    """🪝️ The kept hints with the references of the compare hints chosen in their order: among the tied relations a reference not yet named by an earlier compare hint of the task (of the same dimension) wins, then a familiar one, then the smallest oriented claim, then the first in sheet order."""
    used, hints = set(), []
    for hint in kept:
        if "tied" not in hint:
            hints.append(hint)
            continue
        dimension = hint["dimension"]
        other, _, found, claim, _ = min(hint["tied"], key=lambda entry: ((dimension, entry[0]) in used, not entry[4], oriented(entry[3])))
        used.add((dimension, other))
        hints.append({"kind": "compare", "item": hint["item"], "other": other, **({} if dimension is None else {"dimension": dimension}), **claim, "verdict": found})
    return hints


def profile_axis(axes, presented, assigned, own):
    """🕸️ The axis to question when an item of profile ``own`` sits under profile ``assigned``, with its gap relative to its reach: the largest such ratio (reach = half the spread of the presented categories on the axis; axes without spread left out), first in axis order on ties — none when no gap lies beyond its widened reach."""
    chosen, largest, beyond = None, None, False
    for axis in axes:
        name = axis["id"]
        if name not in assigned or name not in own:
            continue
        values = [category["profile"][name] for category in presented if name in category.get("profile", {})]
        spread = (max(values) - min(values)) / 2
        if spread == 0:
            continue
        gap = abs(assigned[name] - own[name])
        beyond = beyond or gap > spread * (1 + REACH_SLACK)
        if largest is None or gap / spread > largest:
            chosen, largest = name, gap / spread
    return (chosen, largest) if beyond else None


def profile_anchor(placed, own, categories, item, axis, assigned, true):
    """📍️ The anchor a profile hint names: among the other items placed in their own category whose profile has a value on ``axis`` strictly between the item's assigned value and its true one (so the placement claims the item lies on the other side of it), the one farthest from the true value, the first in sheet order on ties; with the side the placement claims (``above``) — ``None`` without one."""
    found = None
    for other, chosen in placed:
        value = categories[chosen].get("profile", {}).get(axis)
        if other == item or chosen != own[other] or value is None or not (assigned > value > true or assigned < value < true):
            continue
        if found is None or abs(true - value) > found[1]:
            found = (other, abs(true - value), assigned > value)
    return None if found is None else {"other": found[0], "above": found[2]}


def misplaced_hints(task, sheet_task, assignments):
    """🧺️ Per misplaced item in sheet order, with its weight: the profile hint where both categories carry profiles (none for a near miss) weighted by its gap relative to the reach, naming an anchor where one lies between; else, unweighted, a partner the learner put in the same category though it belongs elsewhere, else one of its own category the learner put elsewhere, else the assigned category."""
    own = {item["id"]: item["category"] for item in task["items"]}
    categories = {category["id"]: category for category in task["categories"]}
    presented = [categories[category["id"]] for category in sheet_task["categories"]]
    placed = [(item["id"], assignments[item["id"]]) for item in sheet_task["items"] if item["id"] in assignments]
    hints = []
    for item, category in placed:
        if category == own[item]:
            continue
        if "profile" in categories[category] and "profile" in categories[own[item]]:
            found = profile_axis(task.get("axes", []), presented, categories[category]["profile"], categories[own[item]]["profile"])
            if found is not None:
                axis, ratio = found
                anchor = profile_anchor(placed, own, categories, item, axis, categories[category]["profile"][axis], categories[own[item]]["profile"][axis])
                hints.append((ratio, {"kind": "profile", "item": item, "category": category, "axis": axis, **(anchor or {})}))
            continue
        together = next((other for other, chosen in placed if other != item and chosen == category and own[other] != own[item]), None)
        apart = next((other for other, chosen in placed if own[other] == own[item] and chosen != category), None)
        if together is not None:
            hints.append((None, {"kind": "group", "item": item, "other": together, "together": True}))
        elif apart is not None:
            hints.append((None, {"kind": "group", "item": item, "other": apart, "together": False}))
        else:
            hints.append((None, {"kind": "category", "item": item, "category": category}))
    return hints


def capped(weighted):
    """✂️ At most ``HINTS_PER_TASK`` hints, kept in their given order: the weighted ones (largest compare errors, profile gaps relative to their reach) with the largest weights first, then the unweighted group and category hints, the earlier one on ties; the references of the kept compare hints are then chosen by ``referenced``."""
    ranked = sorted(range(len(weighted)), key=lambda index: (weighted[index][0] is None, 0 if weighted[index][0] is None else -weighted[index][0], index))
    kept = set(ranked[:HINTS_PER_TASK])
    return referenced([hint for index, (_, hint) in enumerate(weighted) if index in kept])


def hints_of(task, sheet_task, answer):
    """💡️ The hints of one recorded answer, at most ``HINTS_PER_TASK``: none without an answer or where the sheet task hides the keys of a sorting or matching; sorting in the learner's order, matching per dimension then item in sheet order, classification in sheet order; one task's compare hints prefer references not yet named (per dimension)."""
    if answer is None:
        return []
    items = {item["id"]: item for item in task["items"]}
    familiar = {item["id"] for item in task["items"] if item.get("familiar")}
    if task["kind"] == "sorting":
        if "keys" not in sheet_task:
            return []
        scale = task["quantity"]["scale"]
        key_of = {item: sheet_task["keys"][place] for place, item in enumerate(answer["order"])}
        keyed = [(item["id"], key_of[item["id"]], items[item["id"]]["value"]) for item in sheet_task["items"] if item["id"] in key_of]
        return capped(compare_hints([(item, key_of[item], items[item]["value"]) for item in answer["order"]], keyed, scale, reach(sheet_task["keys"], scale), familiar))
    if task["kind"] == "matching":
        scales = {dimension["id"]: dimension["quantity"]["scale"] for dimension in task["dimensions"]}
        hints = []
        for dimension in sheet_task["dimensions"]:
            if "cards" not in dimension:
                continue
            name, scale = dimension["id"], scales[dimension["id"]]
            chosen = answer.get("assignments", {}).get(name, {})
            keyed = [(item["id"], dimension["cards"][chosen[item["id"]]], items[item["id"]]["values"][name]) for item in sheet_task["items"] if item["id"] in chosen]
            hints += compare_hints(keyed, keyed, scale, reach([items[item["id"]]["values"][name] for item in sheet_task["items"]], scale), familiar, name)
        return capped(hints)
    return capped(misplaced_hints(task, sheet_task, answer["assignments"]))


def bounded(value):
    """♾️ A reach as JSON carries it: ``None`` where nothing misses."""
    return None if math.isinf(value) else value


# endregion 🔖️Reference


# region 🔖️ThirdParty
def numpy_reach(values, scale):
    """🧮️ The reach by numpy: ``ptp`` halved on a linear scale, ``sqrt`` of ``amax / amin`` clipped to the cap with ``minimum`` on a logarithmic one (``ptp`` tells whether the values spread)."""
    array = numpy.array(values, dtype=float)
    spread = numpy.ptp(array)
    if scale == "linear":
        return float(spread / numpy.float64(2)) if spread > 0 else float(numpy.inf)
    lowest = numpy.amin(array)
    return float(numpy.minimum(numpy.float64(REACH_FACTOR), numpy.sqrt(numpy.amax(array) / lowest))) if spread > 0 else float(REACH_FACTOR)


def numpy_misses(values, truths, scale, bound):
    """🔬️ Which of several values miss their truths, by one vectorised comparison of ratios or distances."""
    given, true = numpy.array(values, dtype=float), numpy.array(truths, dtype=float)
    off = numpy.abs(given - true) if scale == "linear" else numpy.maximum(given, true) / numpy.minimum(given, true)
    return [bool(found) for found in off > numpy.multiply(numpy.float64(bound), numpy.add(numpy.float64(1), numpy.float64(REACH_SLACK)))]


def exactly_rounded_root(vector, values, found):
    """📏️ A logarithmic reach below the cap must be ``sqrt(hi / lo)`` correctly rounded: the exact squares of the midpoints to its neighbouring doubles bracket the double ``hi / lo``."""
    quotient = Fraction(max(values) / min(values))
    below, above = Fraction(math.nextafter(found, 0.0)), Fraction(math.nextafter(found, math.inf))
    middle = Fraction(found)
    if not ((below + middle) / 2) ** 2 <= quotient <= ((middle + above) / 2) ** 2:
        raise AssertionError("%s: %r is not the correctly rounded square root of %r" % (vector, found, float(quotient)))


def corroborate_reach(vector, values, scale, found):
    """🔍️ numpy must reproduce a reach bit for bit, and a logarithmic one below the cap must be exactly rounded, or the reference is not trusted."""
    recomputed = numpy_reach(values, scale)
    if found != recomputed:
        raise AssertionError("%s: the reference reach is %r, numpy says %r" % (vector, found, recomputed))
    if scale == "logarithmic" and found < REACH_FACTOR and max(values) > min(values):
        exactly_rounded_root(vector, values, found)


def numpy_compare(ids, keys, truths, familiar, scale, bound, order, dimension=None):
    """🧲️ The compare hints over arrays in sheet order before their references are chosen, weighted by ``numpy.max`` of the pool's errors: the misses by one comparison, then per hinted item (in ``order``) every candidate's claimed and true ratio or difference at once, the verdict by ``numpy.sign`` (linear) or the sides of 1 (logarithmic) as masks, the pool as a mask and the members tied within the slack of the largest error as a second mask, for ``numpy_referenced``."""
    given, true, known = numpy.array(keys, dtype=float), numpy.array(truths, dtype=float), numpy.array(familiar, dtype=bool)
    missed = numpy.array(numpy_misses(keys, truths, scale, bound), dtype=bool)
    hints = []
    for item in order:
        index = ids.index(item)
        if not missed[index]:
            continue
        candidates = numpy.arange(len(ids)) != index
        pool = candidates & ~missed if numpy.any(candidates & ~missed) else candidates
        if not numpy.any(pool):
            continue
        if scale == "linear":
            claimed, actual = numpy.subtract(given[index], given), numpy.subtract(true[index], true)
            errors, side, distance = numpy.abs(numpy.subtract(claimed, actual)), numpy.float64(0), numpy.abs(claimed)
        else:
            claimed, actual = numpy.divide(given[index], given), numpy.divide(true[index], true)
            errors, side, distance = numpy.divide(numpy.maximum(claimed, actual), numpy.minimum(claimed, actual)), numpy.float64(1), numpy.maximum(claimed, numpy.divide(numpy.float64(1), claimed))
        pointing = numpy.sign(numpy.subtract(claimed, side))
        reversed_ = (pointing != 0) & (pointing != numpy.sign(numpy.subtract(actual, side)))
        under = numpy.logical_or(numpy.logical_and(claimed >= side, actual > claimed), numpy.logical_and(claimed < side, actual < claimed))
        verdicts = numpy.where(reversed_, "reversed", numpy.where(under, "under", "over"))
        largest = numpy.max(errors[pool])
        tied = pool & (numpy.multiply(errors, numpy.add(numpy.float64(1), numpy.float64(REACH_SLACK))) >= largest)
        if numpy.any(tied):
            hints.append((float(largest), {"item": item, "dimension": dimension, "ids": numpy.array(ids, dtype=object), "tied": tied, "known": known, "distance": distance, "claimed": claimed, "verdicts": verdicts, "member": "difference" if scale == "linear" else "factor"}))
    return hints


def numpy_referenced(kept):
    """🧷️ The references of the kept compare hints in their order, by masks: the tied references narrowed to those not yet named by an earlier compare hint of the dimension (``numpy.isin`` over the names used so far) where any is left, then to the familiar ones where any is left, and the reference by ``numpy.argmin`` of the oriented claims, whose first minimum is the first in sheet order."""
    used, hints = {}, []
    for hint in kept:
        if "tied" not in hint:
            hints.append(hint)
            continue
        dimension, tied = hint["dimension"], hint["tied"]
        unused = tied & ~numpy.isin(hint["ids"], numpy.array(used.get(dimension, []), dtype=object))
        fresh = unused if numpy.any(unused) else tied
        preferred = fresh & hint["known"] if numpy.any(fresh & hint["known"]) else fresh
        other = int(numpy.argmin(numpy.where(preferred, hint["distance"], numpy.inf)))
        used.setdefault(dimension, []).append(hint["ids"][other])
        hints.append({"kind": "compare", "item": hint["item"], "other": hint["ids"][other], **({} if dimension is None else {"dimension": dimension}), hint["member"]: float(hint["claimed"][other]), "verdict": str(hint["verdicts"][other])})
    return hints


def numpy_classification(task, sheet_task, assignments):
    """🧭️ The classification hints over arrays, with their weights: per misplaced item the profile reaches by ``numpy.ptp`` per axis, the gaps by ``numpy.abs``, the axis by ``numpy.argmax`` over the ratios (axes without spread masked out), the anchor by ``numpy.argmax`` over the distances to the true value of the items placed in their own category whose value lies between, and the partners by ``numpy.flatnonzero`` over boolean masks in sheet order."""
    own = {item["id"]: item["category"] for item in task["items"]}
    categories = {category["id"]: category for category in task["categories"]}
    presented = [categories[category["id"]] for category in sheet_task["categories"]]
    placed = [item["id"] for item in sheet_task["items"] if item["id"] in assignments]
    chosen = numpy.array([assignments[item] for item in placed], dtype=object)
    owns = numpy.array([own[item] for item in placed], dtype=object)
    hints = []
    for index, item in enumerate(placed):
        category = chosen[index]
        if category == owns[index]:
            continue
        others = numpy.arange(len(placed)) != index
        if "profile" in categories[category] and "profile" in categories[owns[index]]:
            axes = [axis["id"] for axis in task.get("axes", []) if axis["id"] in categories[category]["profile"] and axis["id"] in categories[owns[index]]["profile"]]
            reaches = numpy.array([numpy.ptp(numpy.array([entry["profile"][axis] for entry in presented if axis in entry.get("profile", {})], dtype=float)) / numpy.float64(2) for axis in axes], dtype=float)
            gaps = numpy.abs(numpy.array([categories[category]["profile"][axis] for axis in axes], dtype=float) - numpy.array([categories[owns[index]]["profile"][axis] for axis in axes], dtype=float))
            spread = reaches > 0
            ratios = numpy.divide(gaps, reaches, out=numpy.full(len(axes), -numpy.inf), where=spread)
            if numpy.any(spread & (gaps > numpy.multiply(reaches, numpy.add(numpy.float64(1), numpy.float64(REACH_SLACK))))):
                axis = axes[int(numpy.argmax(ratios))]
                assigned, true = numpy.float64(categories[category]["profile"][axis]), numpy.float64(categories[owns[index]]["profile"][axis])
                values = numpy.array([categories[owner].get("profile", {}).get(axis, numpy.nan) for owner in owns], dtype=float)
                between = others & (chosen == owns) & (((assigned > values) & (values > true)) | ((assigned < values) & (values < true)))
                anchor = int(numpy.argmax(numpy.where(between, numpy.abs(numpy.subtract(true, values)), -numpy.inf)))
                hints.append((float(numpy.max(ratios)), {"kind": "profile", "item": item, "category": category, "axis": axis, **({"other": placed[anchor], "above": bool(assigned > values[anchor])} if numpy.any(between) else {})}))
            continue
        together = numpy.flatnonzero(others & (chosen == category) & (owns != owns[index]))
        apart = numpy.flatnonzero((owns == owns[index]) & (chosen != category))
        if together.size:
            hints.append((None, {"kind": "group", "item": item, "other": placed[int(together[0])], "together": True}))
        elif apart.size:
            hints.append((None, {"kind": "group", "item": item, "other": placed[int(apart[0])], "together": False}))
        else:
            hints.append((None, {"kind": "category", "item": item, "category": category}))
    return hints


def numpy_capped(weighted):
    """🪚️ The cap by ``numpy.lexsort`` (stable): unweighted hints after weighted ones, weighted ones by descending weight; the first ``HINTS_PER_TASK`` of that ranking, returned in their given order by ``numpy.sort`` of their positions, with their references chosen by ``numpy_referenced``."""
    if not weighted:
        return []
    weights = numpy.array([0.0 if weight is None else weight for weight, _ in weighted], dtype=float)
    unweighted = numpy.array([weight is None for weight, _ in weighted], dtype=bool)
    return numpy_referenced([weighted[int(position)][1] for position in numpy.sort(numpy.lexsort((numpy.negative(weights), unweighted))[:HINTS_PER_TASK])])


def corroborate_hints(vector, task, sheet_task, answer, hints):
    """🕵️ numpy must recompute the hints of one answer, and a ladder of keys must be the ascending true values of the presented items."""
    items = {item["id"]: item for item in task["items"]}
    if answer is None:
        expected = []
    elif task["kind"] == "classification":
        expected = numpy_capped(numpy_classification(task, sheet_task, answer["assignments"]))
    elif task["kind"] == "sorting":
        expected = []
        if "keys" in sheet_task:
            if sheet_task["keys"] != numpy.sort(numpy.array([items[item["id"]]["value"] for item in sheet_task["items"]], dtype=float)).tolist():
                raise AssertionError("%s: the keys are not the ascending true values of the presented items" % vector)
            places = {item: place for place, item in enumerate(answer["order"])}
            ids = [item["id"] for item in sheet_task["items"] if item["id"] in places]
            scale = task["quantity"]["scale"]
            expected = numpy_capped(numpy_compare(ids, [sheet_task["keys"][places[item]] for item in ids], [items[item]["value"] for item in ids], [items[item].get("familiar", False) for item in ids], scale, numpy_reach(sheet_task["keys"], scale), answer["order"]))
    else:
        weighted = []
        for dimension in sheet_task["dimensions"]:
            chosen = answer.get("assignments", {}).get(dimension["id"], {})
            ids = [item["id"] for item in sheet_task["items"] if item["id"] in chosen]
            if "cards" not in dimension or not ids:
                continue
            scale = next(entry["quantity"]["scale"] for entry in task["dimensions"] if entry["id"] == dimension["id"])
            bound = numpy_reach([items[item["id"]]["values"][dimension["id"]] for item in sheet_task["items"]], scale)
            weighted += numpy_compare(ids, [dimension["cards"][chosen[item]] for item in ids], [items[item]["values"][dimension["id"]] for item in ids], [items[item].get("familiar", False) for item in ids], scale, bound, ids, dimension["id"])
        expected = numpy_capped(weighted)
    if hints != expected:
        raise AssertionError("%s: the reference hints %r, numpy implies %r" % (vector, hints, expected))


# endregion 🔖️ThirdParty


# region 🔖️Handlers
def close(produced, committed):
    """🤏️ Structural equality with the parity tolerance on numbers."""
    if isinstance(produced, bool) or isinstance(committed, bool):
        return produced == committed
    if isinstance(produced, (int, float)) and isinstance(committed, (int, float)):
        return abs(produced - committed) <= TOLERANCE
    if isinstance(produced, dict) and isinstance(committed, dict):
        return produced.keys() == committed.keys() and all(close(produced[key], committed[key]) for key in produced)
    if isinstance(produced, list) and isinstance(committed, list):
        return len(produced) == len(committed) and all(close(left, right) for left, right in zip(produced, committed))
    return produced == committed


def held_to(scenario, produced, expected):
    """⚖️ The reference answer must be the committed one."""
    for key, value in produced.items():
        if not close(value, expected[key]):
            raise AssertionError("%s/%s: the reference answers %r, the committed vector %r" % (scenario, key, value, expected[key]))
    return Outcome(produced)


def committed(ctx):
    """🧫️ The committed vectors."""
    return json.loads(ctx.input_bytes(VECTORS))


def rule_table(challenges):
    """📋️ The rule table, the ranks and who meets whom, over the committed challenges."""
    return {"table": {challenge: CHALLENGE_RULES[challenge] for challenge in challenges}, "ranks": {challenge: challenge_rank(challenge) for challenge in challenges}, "meets": {challenge: {least: challenge_meets(challenge, least) for least in challenges} for challenge in challenges}}


def scored_points(vector):
    """🧾️ The points of one committed score at one challenge, held to the exact rational product rounded once."""
    found = points(vector["score"], vector["challenge"])
    exact = float(Fraction(vector["score"]) * CHALLENGE_RULES[vector["challenge"]]["par"])
    if found != exact or found != float(numpy.float64(vector["score"]) * numpy.float64(CHALLENGE_RULES[vector["challenge"]]["par"])):
        raise AssertionError("points/%s: the reference says %r, the exact product %r" % (vector["id"], found, exact))
    return found


def rules(ctx):
    """🗂️ The rule table, the ranks, the least-challenge relation and the points of every committed score."""
    vectors = committed(ctx)
    table = rule_table(vectors["rules"]["challenges"])
    if [challenge for challenge in vectors["rules"]["challenges"]] != CHALLENGES or sorted(table["ranks"].values()) != [int(rank) for rank in numpy.arange(len(CHALLENGES))]:
        raise AssertionError("rules: the committed challenges are not the four in ascending order")
    if any(int(numpy.diff([CHALLENGE_RULES[challenge]["par"] for challenge in CHALLENGES])[index]) != 100 for index in range(3)):
        raise AssertionError("rules: every step up must add a par of 100")
    return held_to("rules", {**table, "points": {vector["id"]: scored_points(vector) for vector in vectors["points"]}}, {**vectors["rules"]["expected"], "points": {vector["id"]: vector["expected"] for vector in vectors["points"]}})


def reaches(ctx):
    """📡️ The reach of every committed set of values, and whether every committed value misses its truth."""
    vectors = committed(ctx)
    produced = {"reaches": {}, "misses": {}}
    for vector in vectors["reaches"]:
        found = reach(vector["values"], vector["scale"])
        corroborate_reach("reaches/%s" % vector["id"], vector["values"], vector["scale"], found)
        produced["reaches"][vector["id"]] = bounded(found)
    for vector in vectors["misses"]:
        found = reach(vector["values"], vector["scale"])
        corroborate_reach("misses/%s" % vector["id"], vector["values"], vector["scale"], found)
        miss = misses(vector["value"], vector["truth"], vector["scale"], found)
        if [miss] != numpy_misses([vector["value"]], [vector["truth"]], vector["scale"], numpy_reach(vector["values"], vector["scale"])):
            raise AssertionError("misses/%s: the reference says %r, numpy the opposite" % (vector["id"], miss))
        produced["misses"][vector["id"]] = {"reach": bounded(found), "miss": miss}
    return held_to("reach", produced, {"reaches": {vector["id"]: vector["expected"] for vector in vectors["reaches"]}, "misses": {vector["id"]: vector["expected"] for vector in vectors["misses"]}})


def clock(ctx):
    """🕰️ The seconds of every committed timed task and every committed instant raised to its floor."""
    vectors = committed(ctx)
    produced = {"seconds": {}, "instants": {}}
    for vector in vectors["seconds"]:
        found = task_seconds(vector["kind"], vector["items"], vector["dimensions"])
        if found != TASK_SECONDS["base"] + int(numpy.prod([TASK_SECONDS[vector["kind"]], vector["items"], vector["dimensions"] if vector["kind"] == "matching" else 1])):
            raise AssertionError("seconds/%s: numpy's product disagrees with %r" % (vector["id"], found))
        produced["seconds"][vector["id"]] = found
    for vector in vectors["instants"]:
        found = acted(vector["at"], vector["floor"], vector["now"])
        oracle = int(numpy.maximum(numpy.minimum(numpy.uint64(vector["at"]), numpy.add(numpy.uint64(vector["now"]), numpy.uint64(CLOCK_LEAD))), numpy.uint64(vector["floor"])))
        if found != oracle:
            raise AssertionError("instants/%s: the reference says %r, numpy %r" % (vector["id"], found, oracle))
        produced["instants"][vector["id"]] = found
    return held_to("clock", produced, {"seconds": {vector["id"]: vector["expected"] for vector in vectors["seconds"]}, "instants": {vector["id"]: vector["expected"] for vector in vectors["instants"]}})


def hinted(ctx, group):
    """🗃️ The hints of every committed task, sheet task and answer of one vector group, recomputed by numpy."""
    vectors = committed(ctx)
    tasks = {task["id"]: task for task in vectors["tasks"]}
    produced = {}
    for vector in vectors[group]:
        task = tasks[vector["task"]]
        produced[vector["id"]] = hints_of(task, vector["sheetTask"], vector.get("answer"))
        corroborate_hints("%s/%s" % (group, vector["id"]), task, vector["sheetTask"], vector.get("answer"), produced[vector["id"]])
    return held_to(group, produced, {vector["id"]: vector["expected"] for vector in vectors[group]})


def hints(ctx):
    """📈️ The compare hints of sortings and matchings and every case without a hint."""
    return hinted(ctx, "hints")


def classification_hints(ctx):
    """🏷️ The profile, group and category hints of classifications."""
    return hinted(ctx, "classificationHints")


def quiz_hints(ctx):
    """📚️ The hints of the tasks of the authored quizzes on sheets they deal."""
    return hinted(ctx, "quizHints")


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter():
    """🔌️ Oracle role only."""
    return Adapter("python").oracle("rules", rules).oracle("reach", reaches).oracle("clock", clock).oracle("hints", hints).oracle("classification-hints", classification_hints).oracle("quiz-hints", quiz_hints)


# endregion 🔖️Registration

#!/usr/bin/env python3
"""🧾️ Second implementation of the learner lifecycle (design §3–§8), in Python.

Written from the design text alone — never from the TypeScript or Rust twins. It composes everything a
proctor decides with: the id and slug shapes and the handle policy (the letters are derived from the
Unicode Character Database through ``unicodedata``, never from the cores' range tables), the handle
decider (one state per handle key; a claimed handle is refused, never recalled by a command), the
learner decider (anonymous registration, start-run, open-task, record-answer, submit-run) with the caps of
``Limits``, the sheet of §4 over numpy's MT19937, the answer rules of §5, the scoring of §6 and the
badges of §7, plus the folding ``evolve`` and the learner and run views. State is a plain dictionary
here; only decisions and schema views are projected, so no twin's private state shape is ever compared.

A run is played at one challenge (challenge design §3): chosen at ``start-run`` and fixed for the run —
starting the quiz again at the open run's challenge is ``run-open``, at another challenge it voids the
open run. The sheet, the answer rules, the scoring and the points follow the challenge: the keys show
or are guessed, an open easy run carries hints (challenge design §8: each a question about one relation the
learner's own answer claims between two items, about a profile axis or about a category), and on a timed run a task is opened once (``open-task``,
``run-untimed`` elsewhere, ``already-opened`` the second time) before it is answered (``task-unopened``)
within its seconds (``time-up``). ``start-run``, ``open-task`` and ``record-answer`` carry the instant the
learner acted, a timestamp of at most 2^53 − 1 (``id-invalid`` beyond): every claim is first lowered to
five minutes past the decider's clock; the run then starts at its claim, an opening or an answer is raised to
the run's start — the task's opening for an answer on a timed run — and the limit is applied to these instants
alone, so a device whose clock runs less than five minutes ahead is never lowered. The caps count every
run started, open, submitted or voided alike. A timed run is submitted with tasks unanswered or partly answered,
what is missing scoring as a miss; an untimed one stays ``run-incomplete``.

@see ../../🧫️fixtures/🧾️learner-lifecycle/🔣️.json
"""

# region 🔖️Imports
import copy
import json
import math
import os
import re
import unicodedata
from collections import Counter

import numpy

from semio_repo_test import Adapter, Outcome

# endregion 🔖️Imports


# region 🔖️Randomness
def fnv1a32(text):
    """#️⃣ FNV-1a 32-bit over the UTF-8 bytes of ``text`` — the seed of a run."""
    value = 2166136261
    for byte in text.encode("utf-8"):
        value = ((value ^ byte) * 16777619) & 0xFFFFFFFF
    return value


class Mt19937:
    """🌀️ numpy's MT19937 seeded with ``init_genrand(seed)``, read one tempered word at a time."""

    def __init__(self, seed):
        """🔑️ Seeds the generator with ``init_genrand(seed)`` through numpy's legacy seeding."""
        state = numpy.random.RandomState(seed).get_state()
        self.generator = numpy.random.MT19937()
        self.generator.state = {"bit_generator": "MT19937", "state": {"key": state[1], "pos": state[2]}}

    def next(self):
        """🔢️ The next tempered u32."""
        return int(self.generator.random_raw())


def uniform(generator, bound):
    """🎯️ Rejection-sampled index in ``[0, bound)``; ``bound == 1`` draws nothing."""
    if bound == 1:
        return 0
    limit = 2**32 - (2**32 % bound)
    drawn = generator.next()
    while drawn >= limit:
        drawn = generator.next()
    return drawn % bound


def shuffle(generator, items):
    """🔀️ Fisher–Yates from the end on a copy."""
    shuffled = list(items)
    for index in range(len(shuffled) - 1, 0, -1):
        other = uniform(generator, index + 1)
        shuffled[index], shuffled[other] = shuffled[other], shuffled[index]
    return shuffled


# endregion 🔖️Randomness


# region 🔖️Challenge
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


def challenge_meets(challenge, least):
    """🏔️ Whether a challenge is at least as demanding as ``least``."""
    return CHALLENGES.index(challenge) >= CHALLENGES.index(least)


def scaled(value, scale):
    """📐️ ``s(v)``: the value itself, or its decadic logarithm on a logarithmic scale."""
    return value if scale == "linear" else math.log10(value)


def reach(values, scale):
    """📡️ A factor on a logarithmic scale — ``sqrt(hi / lo)``, at most 1000, 1000 where the values do not spread — and a distance on a linear one — ``(hi − lo) / 2``, unbounded where they do not spread."""
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
    """🧑‍⚖️ Where the truth lies against the claim, around the pivot 1 (logarithmic) or 0 (linear): ``reversed`` when the claim lies off the pivot and the truth at it or on its other side — a claim at the pivot (equal keys) names no direction and is never reversed —, else ``under`` when the truth lies beyond the claim, further from the pivot, else ``over``."""
    pivot = 0 if scale == "linear" else 1
    if (claimed > pivot and true <= pivot) or (claimed < pivot and true >= pivot):
        return "reversed"
    return "under" if (claimed >= pivot and true > claimed) or (claimed < pivot and true < claimed) else "over"


def relation(scale, key, truth, other_key, other_truth):
    """🔗️ How wrong the relation of two items that the keys claim is, its verdict, and the claim as the hint carries it: the ratio of the keys on a logarithmic scale, their difference on a linear one."""
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
    """🧿️ The axis to question when an item of profile ``own`` sits under profile ``assigned``, with its gap relative to its reach: the largest such ratio (reach = half the spread of the presented categories on the axis; axes without spread left out), first in axis order on ties — none when no gap lies beyond its widened reach."""
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
    """📌️ The anchor a profile hint names: among the other items placed in their own category whose profile has a value on ``axis`` strictly between the item's assigned value and its true one, the one farthest from the true value, the first in sheet order on ties; with the side the placement claims (``above``) — ``None`` without one."""
    found = None
    for other, chosen in placed:
        value = categories[chosen].get("profile", {}).get(axis)
        if other == item or chosen != own[other] or value is None or not (assigned > value > true or assigned < value < true):
            continue
        if found is None or abs(true - value) > found[1]:
            found = (other, abs(true - value), assigned > value)
    return None if found is None else {"other": found[0], "above": found[2]}


def misplaced_hints(task, sheet_task_, assignments):
    """🧺️ Per misplaced item in sheet order, with its weight: the profile hint where both categories carry profiles (none for a near miss) weighted by its gap relative to the reach, naming an anchor where one lies between; else, unweighted, a partner the learner put in the same category though it belongs elsewhere, else one of its own category the learner put elsewhere, else the assigned category."""
    own = {item["id"]: item["category"] for item in task["items"]}
    categories = {category["id"]: category for category in task["categories"]}
    presented = [categories[category["id"]] for category in sheet_task_["categories"]]
    placed = [(item["id"], assignments[item["id"]]) for item in sheet_task_["items"] if item["id"] in assignments]
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


def hints_of(task, sheet_task_, answer):
    """💡️ The hints of one recorded answer (challenge design §8, §8.4a, §8.4b), at most ``HINTS_PER_TASK``: per sorting or matching item whose key misses, the most wrong relation its keys claim to another item, preferring references the task's earlier hints (of the dimension) do not name yet; per misplaced classification item, a profile, group or category question."""
    if answer is None:
        return []
    items = {item["id"]: item for item in task["items"]}
    familiar = {item["id"] for item in task["items"] if item.get("familiar")}
    if task["kind"] == "sorting":
        if "keys" not in sheet_task_:
            return []
        scale = task["quantity"]["scale"]
        key_of = {item: sheet_task_["keys"][place] for place, item in enumerate(answer["order"])}
        keyed = [(item["id"], key_of[item["id"]], items[item["id"]]["value"]) for item in sheet_task_["items"] if item["id"] in key_of]
        return capped(compare_hints([(item, key_of[item], items[item]["value"]) for item in answer["order"]], keyed, scale, reach(sheet_task_["keys"], scale), familiar))
    if task["kind"] == "matching":
        scales = {dimension["id"]: dimension["quantity"]["scale"] for dimension in task["dimensions"]}
        hints = []
        for dimension in sheet_task_["dimensions"]:
            if "cards" not in dimension:
                continue
            name, scale = dimension["id"], scales[dimension["id"]]
            chosen = answer.get("assignments", {}).get(name, {})
            keyed = [(item["id"], dimension["cards"][chosen[item["id"]]], items[item["id"]]["values"][name]) for item in sheet_task_["items"] if item["id"] in chosen]
            hints += compare_hints(keyed, keyed, scale, reach([items[item["id"]]["values"][name] for item in sheet_task_["items"]], scale), familiar, name)
        return capped(hints)
    return capped(misplaced_hints(task, sheet_task_, answer["assignments"]))


# endregion 🔖️Challenge


# region 🔖️Sheet
def ascending(task, items):
    """📈️ The true ascending order of sorting items: by value, ties by definition index."""
    index = {item["id"]: position for position, item in enumerate(task["items"])}
    return sorted(items, key=lambda item: (item["value"], index[item["id"]]))


def icon_of(source):
    """🖼️ The icon of a task, an item or a dimension as the optional member its presentation carries."""
    return {"icon": source["icon"]} if "icon" in source else {}


def short_of(source):
    """🩳️ The short label of an item as the optional member its presentation carries."""
    return {"short": source["short"]} if "short" in source else {}


def hidden_category(axes, category):
    """🫥️ A category where the keys are hidden: no description, and every profile value its share of the axis range."""
    ranges = {axis["id"]: axis for axis in axes}
    built = {member: value for member, value in category.items() if member not in ("description", "profile")}
    if "profile" in category:
        built["profile"] = {axis: (value - ranges[axis]["min"]) / (ranges[axis]["max"] - ranges[axis]["min"]) for axis, value in category["profile"].items() if axis in ranges}
    return built


def sheet_task(generator, task, rules):
    """🗒️ One task's sheet, consuming the stream in the same order at every challenge; the keys only where the challenge shows them, the seconds only where it is timed."""
    items = shuffle(generator, task["items"])
    if "draw" in task and task["draw"] < len(items):
        items = items[: task["draw"]]
    built = {"kind": task["kind"], "id": task["id"], "title": task["title"], "prompt": task["prompt"], **icon_of(task)}
    if task["kind"] == "classification":
        if "axes" in task:
            built["axes"] = task["axes"] if rules["keys"] else [{member: value for member, value in axis.items() if member in ("id", "label", "short")} for axis in task["axes"]]
        categories = shuffle(generator, task["categories"])
        built["categories"] = categories if rules["keys"] else [hidden_category(task.get("axes", []), category) for category in categories]
    elif task["kind"] == "sorting":
        built["quantity"] = task["quantity"]
        if len(items) >= 2 and [item["id"] for item in items] == [item["id"] for item in ascending(task, items)]:
            items = items[1:] + items[:1]
        if rules["keys"]:
            built["keys"] = sorted(item["value"] for item in items)
    else:
        built["dimensions"] = []
        for dimension in task["dimensions"]:
            cards = shuffle(generator, [item["values"][dimension["id"]] for item in items])
            built["dimensions"].append({"id": dimension["id"], "quantity": dimension["quantity"], **icon_of(dimension), **({"cards": cards} if rules["keys"] else {})})
    built["items"] = [{"id": item["id"], "label": item["label"], **short_of(item), **icon_of(item)} for item in items]
    if rules["timed"]:
        built["seconds"] = task_seconds(task["kind"], len(items), len(task.get("dimensions", [])))
    return built


def sheet_of(quiz, seed, challenge):
    """🃏️ ``sheet(quiz, seed, challenge)``: keep the first task first, shuffle the rest, then present tasks in definition order."""
    generator = Mt19937(seed)
    shuffled_order = shuffle(generator, range(len(quiz["tasks"])))
    order = [0, *[position for position in shuffled_order if position != 0]] if shuffled_order else []
    built = [sheet_task(generator, task, CHALLENGE_RULES[challenge]) for task in quiz["tasks"]]
    return {"quiz": quiz["id"], "seed": seed, "challenge": challenge, "title": quiz["title"], "description": quiz["description"], "tasks": [built[position] for position in order]}


# endregion 🔖️Sheet


# region 🔖️Answers
def keys_hidden(sheet_task_):
    """🫣️ Whether a sheet task hides its keys: a sorting without ``keys``, a matching with a dimension without ``cards``."""
    if sheet_task_["kind"] == "sorting":
        return "keys" not in sheet_task_
    return sheet_task_["kind"] == "matching" and any("cards" not in dimension for dimension in sheet_task_["dimensions"])


def numbers_fit(guesses, items, logarithmic):
    """🔣️ Whether guesses name sheet items only and are finite numbers, positive on a logarithmic quantity."""
    return all(item in items and not isinstance(guess, bool) and isinstance(guess, (int, float)) and math.isfinite(guess) and not (logarithmic and guess <= 0) for item, guess in guesses.items())


def answer_rejection(sheet_task_, answer):
    """🚫️ ``answer-invalid`` for an answer the sheet task cannot hold, ``None`` otherwise: assignments where the keys show, guesses where they are hidden."""
    if answer["kind"] != sheet_task_["kind"]:
        return "answer-invalid"
    items = {item["id"] for item in sheet_task_["items"]}
    if answer["kind"] == "classification":
        categories = {category["id"] for category in sheet_task_["categories"]}
        return None if all(item in items and category in categories for item, category in answer["assignments"].items()) else "answer-invalid"
    if answer["kind"] == "sorting":
        if Counter(answer["order"]) != Counter(item["id"] for item in sheet_task_["items"]):
            return "answer-invalid"
        if not keys_hidden(sheet_task_):
            return "answer-invalid" if "guesses" in answer else None
        guesses = answer.get("guesses", {})
        guessed = [guesses[item] for item in answer["order"] if item in guesses]
        fits = numbers_fit(guesses, items, sheet_task_["quantity"]["scale"] == "logarithmic") and all(before <= after for before, after in zip(guessed, guessed[1:]))
        return None if fits else "answer-invalid"
    dimensions = {dimension["id"]: dimension for dimension in sheet_task_["dimensions"]}
    if keys_hidden(sheet_task_):
        if "assignments" in answer:
            return "answer-invalid"
        fits = all(dimension in dimensions and numbers_fit(guesses, items, dimensions[dimension]["quantity"]["scale"] == "logarithmic") for dimension, guesses in answer.get("guesses", {}).items())
        return None if fits else "answer-invalid"
    if "guesses" in answer or "assignments" not in answer:
        return "answer-invalid"
    for dimension, assignments in answer["assignments"].items():
        if dimension not in dimensions:
            return "answer-invalid"
        indices = list(assignments.values())
        if len(set(indices)) != len(indices) or any(item not in items for item in assignments) or any(not 0 <= index < len(dimensions[dimension]["cards"]) for index in indices):
            return "answer-invalid"
    return None


def answer_complete(sheet_task_, answer):
    """☑️ Whether a valid answer, or its absence, lets an untimed run be submitted."""
    if answer is None:
        return False
    items = [item["id"] for item in sheet_task_["items"]]
    if answer["kind"] == "classification":
        return all(item in answer["assignments"] for item in items)
    if answer["kind"] == "sorting":
        return not keys_hidden(sheet_task_) or all(item in answer.get("guesses", {}) for item in items)
    chosen = answer.get("guesses" if keys_hidden(sheet_task_) else "assignments", {})
    return all(dimension["id"] in chosen and all(item in chosen[dimension["id"]] for item in items) for dimension in sheet_task_["dimensions"])


# endregion 🔖️Answers


# region 🔖️Scoring
def sign(value):
    """➕️ The sign of a difference: −1, 0 or 1."""
    return (value > 0) - (value < 0)


def explained(result, item):
    """💬️ Carries the item's explanation into its result when the item has one."""
    if "explanation" in item:
        result["explanation"] = item["explanation"]
    return result


def sorting_result(task, sheet_task_, answer):
    """📏️ Magnitude-weighted pair concordance over the learner's order. Where the keys are hidden a pair is also discordant when either item misses — no guess, or one beyond the reach of the presented values — and without an answer the items stand in sheet order, every one a miss."""
    items = {item["id"]: item for item in task["items"]}
    index = {item["id"]: position for position, item in enumerate(task["items"])}
    scale = task["quantity"]["scale"]
    sheet = [item["id"] for item in sheet_task_["items"]]
    hidden = keys_hidden(sheet_task_)
    order = answer["order"] if answer is not None else sheet
    guesses = answer.get("guesses", {}) if answer is not None else {}
    values = [items[identifier]["value"] for identifier in order]
    bound = reach([items[identifier]["value"] for identifier in sheet], scale)
    missed = [answer is None or (hidden and (identifier not in guesses or misses(guesses[identifier], items[identifier]["value"], scale, bound))) for identifier in order]
    total = 0.0
    discordant = 0.0
    for first in range(len(values)):
        for second in range(first + 1, len(values)):
            weight = abs(scaled(values[first], scale) - scaled(values[second], scale))
            total += weight
            if values[first] > values[second] or missed[first] or missed[second]:
                discordant += weight
    ranks = {identifier: rank for rank, identifier in enumerate(sorted(sheet, key=lambda identifier: (items[identifier]["value"], index[identifier])))}
    results = []
    for position, identifier in enumerate(order):
        result = {"item": identifier, "value": items[identifier]["value"], "position": position, "rank": ranks[identifier]}
        if hidden:
            result.update({"guess": guesses[identifier]} if identifier in guesses else {}, miss=missed[position])
        results.append(explained(result, items[identifier]))
    return {"kind": "sorting", "task": task["id"], "score": 1 - discordant / total if total > 0 else (0 if any(missed) else 1), "items": results}


def matching_result(task, sheet_task_, answer):
    """🧷️ Per-dimension weighted concordance with half-weighted learner-made ties, averaged over dimensions. Where the cards are hidden the assigned value is the guess and an item misses without one or with one beyond the reach of the presented true values; an item left out misses as well; a pair with a miss costs its whole weight."""
    items = {item["id"]: item for item in task["items"]}
    order = [item["id"] for item in sheet_task_["items"]]
    presented = {dimension["id"]: dimension for dimension in sheet_task_["dimensions"]}
    hidden = keys_hidden(sheet_task_)
    chosen = {} if answer is None else answer.get("guesses" if hidden else "assignments", {})
    dimensions = []
    for dimension in task["dimensions"]:
        scale = dimension["quantity"]["scale"]
        truth = [items[identifier]["values"][dimension["id"]] for identifier in order]
        given = chosen.get(dimension["id"], {})
        assigned = [(given[identifier] if hidden else presented[dimension["id"]]["cards"][given[identifier]]) if identifier in given else None for identifier in order]
        bound = reach(truth, scale)
        missed = [value is None or (hidden and misses(value, correct, scale, bound)) for value, correct in zip(assigned, truth)]
        total = 0.0
        discordant = 0.0
        for first in range(len(truth)):
            for second in range(first + 1, len(truth)):
                weight = abs(scaled(truth[first], scale) - scaled(truth[second], scale))
                total += weight
                if missed[first] or missed[second]:
                    discordant += weight
                elif sign(truth[first] - truth[second]) * sign(assigned[first] - assigned[second]) < 0:
                    discordant += weight
                elif assigned[first] == assigned[second] and truth[first] != truth[second]:
                    discordant += weight / 2
        results = []
        for identifier, value, correct, miss in zip(order, assigned, truth, missed):
            result = {"item": identifier, **({"assigned": value} if value is not None else {}), "correct": correct, **({"miss": miss} if hidden else {})}
            results.append(explained(result, items[identifier]))
        dimensions.append({"dimension": dimension["id"], "score": 1 - discordant / total if total > 0 else (0 if any(missed) else 1), "items": results})
    total = 0.0
    for dimension in dimensions:
        total += dimension["score"]
    return {"kind": "matching", "task": task["id"], "score": total / len(dimensions) if dimensions else 0, "dimensions": dimensions}


def normalised(task, profile):
    """🎚️ A profile as ``(v − min)/(max − min)`` per axis, in axis definition order."""
    return [(profile[axis["id"]] - axis["min"]) / (axis["max"] - axis["min"]) for axis in task.get("axes", [])]


def euclidean(first, second):
    """📍️ The Euclidean distance, summed in axis definition order."""
    total = 0.0
    for left, right in zip(first, second):
        difference = left - right
        total += difference * difference
    return math.sqrt(total)


def profile_of(task, category):
    """🧩️ A category's profile when it covers every axis of the task, else ``None`` — an incomplete profile is not profiled."""
    profile = category.get("profile")
    return profile if profile is not None and all(axis["id"] in profile for axis in task.get("axes", [])) else None


def credit(task, assigned, correct):
    """🎖️ 1 on a hit, profile similarity between two profiled categories, 0 otherwise."""
    if assigned == correct:
        return 1
    categories = {category["id"]: category for category in task["categories"]}
    if profile_of(task, categories[assigned]) is None or profile_of(task, categories[correct]) is None:
        return 0
    profiled = [category for category in task["categories"] if profile_of(task, category) is not None]
    largest = 0.0
    for first in range(len(profiled)):
        for second in range(first + 1, len(profiled)):
            largest = max(largest, euclidean(normalised(task, profiled[first]["profile"]), normalised(task, profiled[second]["profile"])))
    if largest == 0:
        return 0
    return max(0, 1 - euclidean(normalised(task, categories[assigned]["profile"]), normalised(task, categories[correct]["profile"])) / largest)


def classification_result(task, sheet_task_, answer):
    """🕸️ Mean credit over the sheet items; an item left unassigned earns 0 and names no category."""
    items = {item["id"]: item for item in task["items"]}
    assignments = answer["assignments"] if answer is not None else {}
    results = []
    total = 0.0
    for sheet_item in sheet_task_["items"]:
        item = items[sheet_item["id"]]
        if item["id"] not in assignments:
            results.append(explained({"item": item["id"], "correct": item["category"], "credit": 0}, item))
            continue
        earned = credit(task, assignments[item["id"]], item["category"])
        total += earned
        results.append(explained({"item": item["id"], "assigned": assignments[item["id"]], "correct": item["category"], "credit": earned}, item))
    return {"kind": "classification", "task": task["id"], "score": total / len(results) if results else 0, "items": results}


def resolvable(task, sheet_task_):
    """🔗️ Whether the sheet task is a presentation of this task: same kind and id, items and dimensions it knows."""
    known = {item["id"] for item in task["items"]}
    same = task["kind"] == sheet_task_["kind"] and task["id"] == sheet_task_["id"] and all(item["id"] in known for item in sheet_task_["items"])
    if same and task["kind"] == "classification":
        return all(category in {category["id"] for category in task["categories"]} for category in (category["id"] for category in sheet_task_["categories"]))
    if same and task["kind"] == "matching":
        return {dimension["id"] for dimension in task["dimensions"]} <= {dimension["id"] for dimension in sheet_task_["dimensions"]}
    return same


def score_task(task, sheet_task_, answer, timed):
    """🧮️ ``scoreTask`` dispatched on the task kind, or ``None`` for an answer held invalid, for a task the sheet cannot resolve and — unless the sheet is timed — for an incomplete or absent answer."""
    if not resolvable(task, sheet_task_) or (answer is not None and answer_rejection(sheet_task_, answer) is not None) or not (timed or answer_complete(sheet_task_, answer)):
        return None
    return {"classification": classification_result, "sorting": sorting_result, "matching": matching_result}[task["kind"]](task, sheet_task_, answer)


def score_run(quiz, sheet, answers):
    """🏁️ ``scoreRun``: task results in sheet order, the run score their mean (0 without tasks) and its points the score times the par of the sheet's challenge, or ``None`` when any task scores none; on a timed sheet a task may be answered partly or not at all."""
    rules = CHALLENGE_RULES[sheet["challenge"]]
    tasks = {task["id"]: task for task in quiz["tasks"]}
    results = [score_task(tasks[sheet_task_["id"]], sheet_task_, answers.get(sheet_task_["id"]), rules["timed"]) if sheet_task_["id"] in tasks else None for sheet_task_ in sheet["tasks"]]
    if any(result is None for result in results):
        return None
    total = 0.0
    for result in results:
        total += result["score"]
    score = total / len(results) if results else 0
    return {"quiz": quiz["id"], "challenge": sheet["challenge"], "score": score, "points": score * rules["par"], "tasks": results}


# endregion 🔖️Scoring


# region 🔖️Badges
def earns(rule, quizzes, results):
    """🧐️ Whether one badge rule holds over the learner's submitted results; a rule that names a challenge counts only the results played at least at it."""
    counted = [result for result in results if "challenge" not in rule or challenge_meets(result["challenge"], rule["challenge"])]
    if rule["kind"] == "perfect-quiz":
        return any(result["quiz"] == rule["quiz"] and result["score"] == 1 for result in counted)
    if rule["kind"] == "perfect-tasks":
        selected = [(quiz["id"], task["id"]) for quiz in quizzes for task in quiz["tasks"] if rule.get("taskKind", task["kind"]) == task["kind"] and rule.get("quiz", quiz["id"]) == quiz["id"]]
        return len(selected) > 0 and all(any(result["quiz"] == quiz and any(scored["task"] == task and scored["score"] == 1 for scored in result["tasks"]) for result in counted) for quiz, task in selected)
    return all(any(result["quiz"] == quiz["id"] for result in results) for quiz in quizzes)


def earned_badges(badges, quizzes, results, held):
    """🏅️ The ids of the badges newly earned, in catalog order."""
    return [badge["id"] for badge in badges if badge["id"] not in held and earns(badge["rule"], quizzes, results)]


# endregion 🔖️Badges


# region 🔖️Handles
WHITE_SPACE = frozenset([*range(0x09, 0x0E), 0x20, 0x85, 0xA0, 0x1680, *range(0x2000, 0x200B), 0x2028, 0x2029, 0x202F, 0x205F, 0x3000])
LATIN_BLOCKS = [(0x0000, 0x007F), (0x0080, 0x00FF), (0x0100, 0x017F), (0x0180, 0x024F), (0x1E00, 0x1EFF)]
HANDLE_MAX = 64
HANDLE_INPUT_MAX = 256
DEFAULT_LIMITS = {"learners": 100000, "runsPerQuiz": 200, "runs": 1000, "answersPerRun": 2000}


def handle_letter(character):
    """🔤️ Whether a character is a Latin upper- or lowercase letter of the five blocks without a compatibility decomposition."""
    point = ord(character)
    return any(low <= point <= high for low, high in LATIN_BLOCKS) and unicodedata.category(character) in ("Lu", "Ll") and unicodedata.normalize("NFKC", character) == character


def normalize_handle(handle):
    """🪪️ ``{display, key}`` of a handle, or ``None`` outside the policy: White_Space runs collapse to one space and are trimmed, U+2019 becomes the apostrophe, then 1…64 letters, ASCII digits, ``'._-`` and single spaces with at least one letter or digit; the key is the lowercase."""
    if len(handle) > HANDLE_INPUT_MAX:
        return None
    words = "".join(" " if ord(character) in WHITE_SPACE else character for character in handle).split(" ")
    display = " ".join(word for word in words if word).replace("’", "'")
    worded = [character for character in display if handle_letter(character) or character in "0123456789"]
    if not worded or len(display) > HANDLE_MAX or any(character not in worded and character not in " '._-" for character in display):
        return None
    return {"display": display, "key": display.lower()}


def handle_actor_id(key):
    """🗝️ The id of the stream of a handle key: the lowercase hex of its UTF-8 bytes."""
    return key.encode("utf-8").hex()


def is_id(value):
    """🆔️ Exactly 32 lowercase hex characters."""
    return isinstance(value, str) and re.fullmatch("[0-9a-f]{32}", value) is not None


def is_slug(value):
    """🐌️ Lowercase letters and digits in hyphen-separated words, at most 64 characters."""
    return isinstance(value, str) and len(value) <= 64 and re.fullmatch("[a-z0-9]+(?:-[a-z0-9]+)*", value) is not None


MAX_TIMESTAMP = 2**53 - 1


def is_timestamp(value):
    """🕰️ An integer from 0 to 2^53 − 1: the instants every language reads exactly."""
    return isinstance(value, int) and not isinstance(value, bool) and 0 <= value <= MAX_TIMESTAMP


def command_rejection(command):
    """🛃️ ``id-invalid`` unless the command, learner and run ids are ids, the quiz and task ids slugs, the challenge of a start one of the four and the ``at`` of a start, an opening or an answer a timestamp."""
    ids = all(is_id(command[member]) for member in ("id", "learner", "run") if member in command)
    slugs = all(is_slug(command[member]) for member in ("quiz", "task") if member in command)
    challenge = command["type"] != "start-run" or command.get("challenge") in CHALLENGES
    at = command["type"] not in ("start-run", "open-task", "record-answer") or is_timestamp(command.get("at"))
    return None if ids and slugs and challenge and at else "id-invalid"


def registration_rejection(learners, limits):
    """🚧️ ``roster-full`` once the proctor holds its cap of learners."""
    return "roster-full" if learners >= limits["learners"] else None


def empty_handle_state(key):
    """🌾️ A handle key nobody claimed yet."""
    return {"key": key, "holder": None}


def decide_handle(state, command, now):
    """🛂️ identify-learner under a pseudonym or name: a free handle of this key registers the learner, a claimed one is refused."""
    malformed = command_rejection(command)
    if malformed is not None:
        return {"rejection": malformed}
    identity = command["identity"]
    handle = normalize_handle(identity["handle"]) if identity["kind"] != "anonymous" else None
    if handle is None or handle["key"] != state["key"]:
        return {"rejection": "handle-invalid"}
    if state["holder"] is not None:
        return {"rejection": "handle-claimed"}
    return {"events": [{"type": "learner-registered", "learner": command["learner"], "identity": {"kind": identity["kind"], "handle": handle["display"]}, "at": now}]}


def evolve_handle(state, event):
    """🗳️ The learner registered under a pseudonym or name holds the handle from then on."""
    if event["type"] != "learner-registered" or event["identity"]["kind"] == "anonymous":
        return state
    return {**state, "holder": event["learner"]}


# endregion 🔖️Handles


# region 🔖️Learner
def empty_learner_state(learner):
    """🌱️ A learner nothing has happened to yet."""
    return {"learner": learner, "identity": None, "runs": [], "badges": [], "best": {}, "reachedAt": None}


def run_of(state, run):
    """🔎️ One run of the learner, or ``None``."""
    return next((candidate for candidate in state["runs"] if candidate["run"] == run), None)


def evolve_learner(state, event):
    """🗂️ Folds one learner event into the state."""
    state = copy.deepcopy(state)
    kind = event["type"]
    if kind == "learner-registered":
        state["identity"] = event["identity"]
    elif kind == "run-started":
        state["runs"].append({"run": event["run"], "quiz": event["quiz"], "challenge": event["challenge"], "revision": event["revision"], "seed": event["seed"], "status": "open", "answers": {}, "recorded": 0, "opened": {}, "result": None, "startedAt": event["at"], "submittedAt": None})
    elif kind == "run-voided":
        run_of(state, event["run"])["status"] = "voided"
    elif kind == "task-opened":
        run_of(state, event["run"])["opened"][event["task"]] = event["at"]
    elif kind == "answer-recorded":
        run = run_of(state, event["run"])
        run["answers"][event["task"]] = event["answer"]
        run["recorded"] += 1
    elif kind == "run-submitted":
        run = run_of(state, event["run"])
        run["status"], run["result"], run["submittedAt"] = "submitted", event["result"], event["at"]
        quiz = event["result"]["quiz"]
        if quiz not in state["best"] or event["result"]["points"] > state["best"][quiz]["points"]:
            state["best"][quiz] = {"challenge": event["result"]["challenge"], "score": event["result"]["score"], "points": event["result"]["points"]}
            state["reachedAt"] = event["at"]
    elif kind == "badge-awarded":
        state["badges"].append({"badge": event["badge"], "run": event["run"], "at": event["at"]})
    return state


def decide_learner(state, command, context):
    """🏛️ The registration of an anonymous learner, start-run, record-answer and submit-run against the learner's state, the current quizzes and the caps."""
    now, learner, limits = context["now"], state["learner"], context["limits"]
    malformed = command_rejection(command)
    if malformed is not None:
        return {"rejection": malformed}
    if command["learner"] != learner:
        return {"rejection": "unknown-learner"}
    if command["type"] == "identify-learner":
        if command["identity"]["kind"] != "anonymous":
            return {"rejection": "handle-invalid"}
        if state["identity"] is not None:
            return {"rejection": "learner-exists"}
        return {"events": [{"type": "learner-registered", "learner": learner, "identity": {"kind": "anonymous"}, "at": now}]}
    if command["type"] == "start-run":
        if state["identity"] is None:
            return {"rejection": "unknown-learner"}
        if command["quiz"] not in context["quizzes"]:
            return {"rejection": "unknown-quiz"}
        used = run_of(state, command["run"])
        if used is not None:
            return {"rejection": "run-open" if used["status"] == "open" else "run-closed"}
        current = context["quizzes"][command["quiz"]]
        started = {"type": "run-started", "learner": learner, "run": command["run"], "quiz": command["quiz"], "challenge": command["challenge"], "revision": current["revision"], "seed": fnv1a32(command["run"]), "at": acted(command["at"], 0, now)}
        open_run = next((run for run in state["runs"] if run["quiz"] == command["quiz"] and run["status"] == "open"), None)
        if open_run is not None and open_run["revision"] == current["revision"] and open_run["challenge"] == command["challenge"]:
            return {"rejection": "run-open"}
        if len(state["runs"]) >= limits["runs"] or sum(1 for run in state["runs"] if run["quiz"] == command["quiz"]) >= limits["runsPerQuiz"]:
            return {"rejection": "runs-exhausted"}
        if open_run is None:
            return {"events": [started]}
        return {"events": [{"type": "run-voided", "learner": learner, "run": open_run["run"], "at": now}, started]}
    run = run_of(state, command["run"])
    if run is None:
        return {"rejection": "unknown-run"}
    if run["status"] != "open":
        return {"rejection": "run-closed"}
    current = context["quizzes"].get(run["quiz"])
    stale = current is None or current["revision"] != run["revision"]
    timed = CHALLENGE_RULES[run["challenge"]]["timed"]
    if command["type"] == "open-task":
        if stale:
            return {"rejection": "quiz-revised"}
        if not timed:
            return {"rejection": "run-untimed"}
        if all(task["id"] != command["task"] for task in sheet_of(current["quiz"], run["seed"], run["challenge"])["tasks"]):
            return {"rejection": "unknown-task"}
        if command["task"] in run["opened"]:
            return {"rejection": "already-opened"}
        return {"events": [{"type": "task-opened", "learner": learner, "run": run["run"], "task": command["task"], "at": acted(command["at"], run["startedAt"], now)}]}
    if command["type"] == "record-answer":
        if stale:
            return {"rejection": "quiz-revised"}
        if run["recorded"] >= limits["answersPerRun"]:
            return {"rejection": "answers-exhausted"}
        sheet_task_ = next((task for task in sheet_of(current["quiz"], run["seed"], run["challenge"])["tasks"] if task["id"] == command["task"]), None)
        if sheet_task_ is None:
            return {"rejection": "unknown-task"}
        if timed:
            if command["task"] not in run["opened"]:
                return {"rejection": "task-unopened"}
            opened = run["opened"][command["task"]]
            at = acted(command["at"], opened, now)
            if at - opened > sheet_task_["seconds"] * 1000:
                return {"rejection": "time-up"}
        else:
            at = acted(command["at"], run["startedAt"], now)
        if answer_rejection(sheet_task_, command["answer"]) is not None:
            return {"rejection": "answer-invalid"}
        return {"events": [{"type": "answer-recorded", "learner": learner, "run": run["run"], "task": command["task"], "answer": command["answer"], "at": at}]}
    if stale:
        return {"events": [{"type": "run-voided", "learner": learner, "run": run["run"], "at": now}]}
    sheet = sheet_of(current["quiz"], run["seed"], run["challenge"])
    if not timed and not all(answer_complete(task, run["answers"].get(task["id"])) for task in sheet["tasks"]):
        return {"rejection": "run-incomplete"}
    result = score_run(current["quiz"], sheet, run["answers"])
    if result is None:
        return {"rejection": "run-incomplete"}
    results = [candidate["result"] for candidate in state["runs"] if candidate["status"] == "submitted"] + [result]
    held = [award["badge"] for award in state["badges"]]
    quizzes = [entry["quiz"] for entry in context["quizzes"].values()]
    awarded = [{"type": "badge-awarded", "learner": learner, "badge": badge, "run": run["run"], "at": now} for badge in earned_badges(context["catalog"]["badges"], quizzes, results, held)]
    return {"events": [{"type": "run-submitted", "learner": learner, "run": run["run"], "result": result, "at": now}] + awarded}


# endregion 🔖️Learner


# region 🔖️Views
def learner_view(state):
    """👤️ The learner's runs newest first — in reverse order of their starts, whatever instants the devices claimed — with their challenge, badges in award order, the best run per quiz — the one with the most points, the earliest of equals — and the total of their points."""
    runs = []
    for run in reversed(state["runs"]):
        summary = {"run": run["run"], "quiz": run["quiz"], "challenge": run["challenge"], "status": run["status"], "startedAt": run["startedAt"]}
        if run["status"] == "submitted":
            summary["score"], summary["points"], summary["submittedAt"] = run["result"]["score"], run["result"]["points"], run["submittedAt"]
        runs.append(summary)
    total = 0.0
    for best in state["best"].values():
        total += best["points"]
    return {"learner": state["learner"], "identity": state["identity"], "runs": runs, "badges": state["badges"], "best": state["best"], "total": total}


def run_view(state, run, quizzes):
    """🏃️ One run with its sheet at its challenge, latest answers and — once submitted — its result; when each task was opened on a timed run; the hints of an open run whose challenge hints, per task that has any."""
    found = run_of(state, run)
    quiz = quizzes[found["quiz"]]["quiz"]
    rules = CHALLENGE_RULES[found["challenge"]]
    sheet = sheet_of(quiz, found["seed"], found["challenge"])
    view = {"run": found["run"], "learner": state["learner"], "quiz": found["quiz"], "status": found["status"], "sheet": sheet, "answers": found["answers"], "startedAt": found["startedAt"]}
    if found["status"] == "submitted":
        view["result"], view["submittedAt"] = found["result"], found["submittedAt"]
    if rules["timed"]:
        view["opened"] = found["opened"]
    if rules["hints"] and found["status"] == "open":
        tasks = {task["id"]: task for task in quiz["tasks"]}
        hints = {presented["id"]: hints_of(tasks[presented["id"]], presented, found["answers"].get(presented["id"])) for presented in sheet["tasks"]}
        if any(hints.values()):
            view["hints"] = {task: found_hints for task, found_hints in hints.items() if found_hints}
    return view


# endregion 🔖️Views


# region 🔖️Replay
VECTORS = "shared://🧾️learner-lifecycle/🔣️.json"
TOLERANCE = 1e-12


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


def quizzes_at(vectors, revisions):
    """📚️ The decision context's quizzes: every catalog quiz with its revision at this point of the sequence."""
    return {quiz["id"]: {"quiz": quiz, "revision": revisions[quiz["id"]]} for quiz in vectors["quizzes"]}


def replay_learner(vectors, sequence):
    """🎞️ Folds the given events, then decides and folds every step under the sequence's caps; returns the decisions and the final state."""
    state = empty_learner_state(sequence["learner"])
    for event in sequence["given"]:
        state = evolve_learner(state, event)
    revisions = dict(vectors["revisions"])
    limits = sequence.get("limits", vectors["limits"])
    decisions = []
    for step in sequence["steps"]:
        revisions.update(step.get("revisions", {}))
        decision = decide_learner(state, step["command"], {"now": step["now"], "catalog": vectors["catalog"], "quizzes": quizzes_at(vectors, revisions), "limits": limits})
        decisions.append(decision)
        for event in decision.get("events", []):
            state = evolve_learner(state, event)
    return decisions, state, quizzes_at(vectors, revisions)


def replay_handle(sequence):
    """📼️ Decides and folds every identify-learner step addressed to one handle key, from the unclaimed key."""
    state = empty_handle_state(sequence["key"])
    decisions = []
    for step in sequence["steps"]:
        decision = decide_handle(state, step["command"], step["now"])
        decisions.append(decision)
        for event in decision.get("events", []):
            state = evolve_handle(state, event)
    return decisions


# endregion 🔖️Replay


# region 🔖️Site
def site_quizzes(repo_root, catalog_path):
    """🎓️ The catalog at a repository path and its quizzes in catalog order."""
    with open(os.path.join(repo_root, catalog_path), "r", encoding="utf-8") as handle:
        catalog = json.load(handle)
    quizzes = []
    for path in catalog["quizzes"]:
        with open(os.path.join(repo_root, os.path.dirname(catalog_path), path), "r", encoding="utf-8") as handle:
            quizzes.append(json.load(handle))
    return catalog, quizzes


def perfect_answer(task, sheet_task_):
    """💯️ The answer that scores a sheet task 1: every item in its category, ascending by value (ties in definition order), every item on a card of its own value — and, where the keys are hidden, every item guessed at its true value."""
    items = {item["id"]: item for item in task["items"]}
    index = {item["id"]: position for position, item in enumerate(task["items"])}
    presented = [item["id"] for item in sheet_task_["items"]]
    if task["kind"] == "classification":
        return {"kind": "classification", "assignments": {identifier: items[identifier]["category"] for identifier in presented}}
    if task["kind"] == "sorting":
        order = sorted(presented, key=lambda identifier: (items[identifier]["value"], index[identifier]))
        return {"kind": "sorting", "order": order, **({"guesses": {identifier: items[identifier]["value"] for identifier in order}} if keys_hidden(sheet_task_) else {})}
    if keys_hidden(sheet_task_):
        return {"kind": "matching", "guesses": {dimension["id"]: {identifier: items[identifier]["values"][dimension["id"]] for identifier in presented} for dimension in sheet_task_["dimensions"]}}
    assignments = {}
    for dimension in sheet_task_["dimensions"]:
        free = list(range(len(dimension["cards"])))
        assignments[dimension["id"]] = {}
        for identifier in presented:
            card = next(position for position in free if dimension["cards"][position] == items[identifier]["values"][dimension["id"]])
            free.remove(card)
            assignments[dimension["id"]][identifier] = card
    return {"kind": "matching", "assignments": assignments}


def flawed_answer(task, sheet_task_):
    """🩹️ The perfect answer with exactly one mistake: the first item in the first wrong category, the smallest and the largest item exchanged (with their guesses, where the keys are hidden), or the cards — the guesses, where they are hidden — of the first item and of the first later item of another value exchanged in the first dimension."""
    answer = perfect_answer(task, sheet_task_)
    items = {item["id"]: item for item in task["items"]}
    presented = [item["id"] for item in sheet_task_["items"]]
    if task["kind"] == "classification":
        first = presented[0]
        answer["assignments"][first] = next(category["id"] for category in sheet_task_["categories"] if category["id"] != items[first]["category"])
    elif task["kind"] == "sorting":
        smallest, largest = answer["order"][0], answer["order"][-1]
        answer["order"][0], answer["order"][-1] = largest, smallest
        if "guesses" in answer:
            answer["guesses"][smallest], answer["guesses"][largest] = answer["guesses"][largest], answer["guesses"][smallest]
    else:
        dimension = sheet_task_["dimensions"][0]["id"]
        first = presented[0]
        other = next(identifier for identifier in presented[1:] if items[identifier]["values"][dimension] != items[first]["values"][dimension])
        chosen = answer["guesses" if "guesses" in answer else "assignments"][dimension]
        chosen[first], chosen[other] = chosen[other], chosen[first]
    return answer


def play(catalog, quizzes, limits, scenario):
    """🎮️ One registered learner playing the scenario's runs, each at its challenge — a timed task is opened before it is answered, every command at the instant it is decided: per run its score and the badges it awards, and every badge held at the end."""
    loaded = {quiz["id"]: {"quiz": quiz, "revision": "0" * 64} for quiz in quizzes}
    state = evolve_learner(empty_learner_state(scenario["learner"]), {"type": "learner-registered", "learner": scenario["learner"], "identity": {"kind": "anonymous"}, "at": 0})
    now = 0
    scores = []
    awards = []

    def decided(command):
        nonlocal state, now
        now += 1
        decision = decide_learner(state, command, {"now": now, "catalog": catalog, "quizzes": loaded, "limits": limits})
        if "events" not in decision:
            raise AssertionError("site/%s: %s is refused with %s" % (scenario["id"], command["type"], decision["rejection"]))
        for event in decision["events"]:
            state = evolve_learner(state, event)
        return decision["events"]

    for number, run in enumerate(scenario["runs"]):
        quiz = loaded[run["quiz"]]["quiz"]
        tasks = {task["id"]: task for task in quiz["tasks"]}
        command = lambda step, **members: {"type": step, "id": "%032x" % (now + 1 + (number << 64)), "learner": scenario["learner"], "run": run["run"], **members}
        decided(command("start-run", quiz=run["quiz"], challenge=run["challenge"], at=now + 1))
        for sheet_task_ in sheet_of(quiz, fnv1a32(run["run"]), run["challenge"])["tasks"]:
            answer = (flawed_answer if sheet_task_["id"] == run.get("flaw") else perfect_answer)(tasks[sheet_task_["id"]], sheet_task_)
            if "seconds" in sheet_task_:
                decided(command("open-task", task=sheet_task_["id"], at=now + 1))
            decided(command("record-answer", task=sheet_task_["id"], answer=answer, at=now + 1))
        events = decided(command("submit-run"))
        scores.append(next(event["result"]["score"] for event in events if event["type"] == "run-submitted"))
        awards.append([event["badge"] for event in events if event["type"] == "badge-awarded"])
    return {"scores": scores, "awards": awards, "held": [award["badge"] for award in state["badges"]]}


def selects(rule, quiz, task):
    """🧗️ Whether a badge rule depends on one task of one quiz being perfect."""
    if rule["kind"] == "perfect-quiz":
        return rule["quiz"] == quiz["id"]
    return rule["kind"] == "perfect-tasks" and rule.get("quiz", quiz["id"]) == quiz["id"] and rule.get("taskKind", task["kind"]) == task["kind"]


# endregion 🔖️Site


# region 🔖️Handlers


def held_to(scenario, produced, expected):
    """⚖️ The reference answer must be the committed one."""
    for key, value in produced.items():
        if not close(value, expected[key]):
            raise AssertionError("%s/%s: the reference answers %r, the committed vector %r" % (scenario, key, value, expected[key]))
    return Outcome(produced)


def committed(ctx):
    """🧫️ The committed vectors."""
    return json.loads(ctx.fixture_bytes(VECTORS))


def claims(vectors):
    """🙋️ Every handle sequence: the well-formed ones and those whose commands carry malformed ids."""
    return vectors["registrations"] + vectors["malformed"]["registrations"]


def sequences(vectors):
    """🧑‍🎓️ Every learner sequence: the well-formed ones and those whose commands carry malformed ids."""
    return vectors["learners"] + vectors["malformed"]["learners"]


def registrations(ctx):
    """📇️ Every committed identify-learner sequence of a handle key, and every committed learner count against its cap."""
    vectors = committed(ctx)
    produced = {"handles": {sequence["id"]: replay_handle(sequence) for sequence in claims(vectors)}, "quotas": {vector["id"]: registration_rejection(vector["learners"], vector["limits"]) for vector in vectors["quotas"]}}
    expected = {"handles": {sequence["id"]: [step["expected"] for step in sequence["steps"]] for sequence in claims(vectors)}, "quotas": {vector["id"]: vector["expected"] for vector in vectors["quotas"]}}
    return held_to("registrations", produced, expected)


def site_catalog(ctx):
    """🥇️ Every committed play of the site catalog: perfect runs score 1 at every challenge and earn every badge whose least challenge they meet, a single mistake withholds exactly the badges that depend on its task."""
    site = committed(ctx)["site"]
    catalog, quizzes = site_quizzes(ctx.repo_root, site["catalog"])
    every = [badge["id"] for badge in catalog["badges"]]
    for named in site["named"]:
        if named not in every:
            raise AssertionError("site: the catalog has no badge %s" % named)
    produced = {}
    for scenario in site["plays"]:
        played = play(catalog, quizzes, DEFAULT_LIMITS, scenario)
        withheld = set()
        for run in scenario["runs"]:
            quiz = next(quiz for quiz in quizzes if quiz["id"] == run["quiz"])
            for task in quiz["tasks"]:
                below = lambda rule: "challenge" in rule and not challenge_meets(run["challenge"], rule["challenge"])
                withheld |= {badge["id"] for badge in catalog["badges"] if selects(badge["rule"], quiz, task) and (run.get("flaw") == task["id"] or below(badge["rule"]))}
        if not all(score == 1 for score, run in zip(played["scores"], scenario["runs"]) if "flaw" not in run) or any(score >= 1 for score, run in zip(played["scores"], scenario["runs"]) if "flaw" in run):
            raise AssertionError("site/%s: a perfect run must score 1 and a flawed run below it, got %r" % (scenario["id"], played["scores"]))
        if sorted(played["held"]) != sorted(set(every) - withheld):
            raise AssertionError("site/%s: holds %r, must hold every badge but %r" % (scenario["id"], played["held"], sorted(withheld)))
        if scenario["expected"] != {"awards": played["awards"], "held": played["held"]}:
            raise AssertionError("site/%s: the reference awards %r, the committed vector %r" % (scenario["id"], played["awards"], scenario["expected"]))
        produced[scenario["id"]] = played
    return Outcome(produced)


def learner_decisions(ctx):
    """📜️ Every committed learner sequence's decisions."""
    vectors = committed(ctx)
    return held_to("learner-decisions", {sequence["id"]: replay_learner(vectors, sequence)[0] for sequence in sequences(vectors)}, {sequence["id"]: [step["expected"] for step in sequence["steps"]] for sequence in sequences(vectors)})


def learner_views(ctx):
    """🪞️ The learner view and the current-revision run views after every committed sequence of a registered learner."""
    vectors = committed(ctx)
    viewed = [sequence for sequence in sequences(vectors) if "views" in sequence]
    produced = {}
    for sequence in viewed:
        _, state, quizzes = replay_learner(vectors, sequence)
        produced[sequence["id"]] = {"learner": learner_view(state), "runs": {run: run_view(state, run, quizzes) for run in sequence["views"]["runs"]}}
    return held_to("learner-views", produced, {sequence["id"]: sequence["views"]["expected"] for sequence in viewed})


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter():
    """🧭️ Oracle role only."""
    return Adapter("python").oracle("registrations", registrations).oracle("learner-decisions", learner_decisions).oracle("learner-views", learner_views).oracle("site-catalog", site_catalog)


# endregion 🔖️Registration

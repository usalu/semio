#!/usr/bin/env python3
"""📊️ Second implementation of the crowd view (design §17 and §20, schema ``CrowdView``), in Python.

What the learners answered and scored in the submitted runs of one quiz, aggregated per task — per
dimension for a matching — and per item, semantically by item id because every learner's sheet differs.
Written from the design text and the schema: only results of the viewed quiz count (``runs``); a result
counts for a task with its first task result of the task's id and kind, for a dimension with the first
dimension of that id in it, for an item with the first entry of that id; tasks and matching dimensions
follow the quiz's definition order, items the task's definition order, and an item nobody answered is left
out; a classification item counts the assigned categories and a matching item the assigned values, as
``{key, count}`` in ascending key order — keys are strings compared by code point, a value rendered in
JSON number syntax as JavaScript prints it (``120``, ``42.6``, ``0.027``) — and a sorting item carries the
mean of its normalized position ``position / (n − 1)`` over the learners who ordered it (0 smallest … 1
largest; a one-item order sits at 0) beside ``places``, one count per place a sheet of the task presents
(``m = min(draw, items)``): position ``i`` in an order of ``n`` items counts for ``i · (m − 1) / (n − 1)``
rounded half up, place 0 when ``n < 2``. ``scores`` holds ten bins of whole percents ``⌊score · 100 + ½⌋``
— ``[0, 10)``, … ``[80, 90)``, ``[90, 100]`` — of the run scores on the view, of the task scores on a
classification or sorting, of the dimension's scores on a matching's crowd task.

One crowd serves every challenge (challenge design §3.6). The score bins take the score — the accuracy —
of runs at any challenge. Where a sheet task hid its cards the assigned value of a matching item is a
guess (its result says whether it ``miss``es): it counts under the nearest value among all authored
items of the task for that dimension, the distance taken on the quantity's scale (``log10`` on a
logarithmic one) and the smaller value of two equally near. An item the learner left unanswered on a
timed run — a classification or matching item result without ``assigned`` — counts nowhere, and a sorting
result without any guess where the keys were hidden is a task without an answer, standing in sheet
order: it adds its score to the bins and nothing to the places.

Counts and places are recounted with ``collections.Counter``, every mean is recomputed with
``numpy.mean``, every score distribution with ``numpy.histogram`` over the whole percents (edges
0, 10, … 90, 101) and the authored value of every guess with ``numpy.argmin`` over ``numpy.unique`` of the
authored values before anything is projected.

@see ../../🧬️schema/🔣️.json
@see ../../🧫️fixtures/📊️crowd-view/🔣️.json
@see https://numpy.org/doc/stable/reference/generated/numpy.histogram.html
"""

# region 🔖️Imports
import json
import math
from collections import Counter
from fractions import Fraction

import numpy

from semio_repo_test import Adapter, Outcome

# endregion 🔖️Imports


# region 🔖️Reference
VECTORS = "shared://📊️crowd-view/🔣️.json"
TOLERANCE = 1e-12
SCORE_BINS = 10


def percent(score):
    """💯️ The whole percent of a score: ``⌊score · 100 + ½⌋``."""
    return math.floor(score * 100 + 0.5)


def score_bin(score):
    """🪣️ ``scoreBin(score)``: the tenth of the whole percent, the last bin closed (``[90, 100]``)."""
    return min(SCORE_BINS - 1, percent(score) // 10)


def binned(scores):
    """📉️ How many of the scores fall into each of the ten bins."""
    bins = [0] * SCORE_BINS
    for score in scores:
        bins[score_bin(score)] += 1
    return bins


def presented(task):
    """🎪️ ``presented(task)``: how many items a sheet of the task presents — ``draw`` of them, all without ``draw``."""
    return min(task.get("draw", len(task["items"])), len(task["items"]))


def place_bin(position, length, places):
    """🪑️ ``placeBin(i, n, m)``: ``i · (m − 1) / (n − 1)`` rounded half up as an exact fraction, place 0 for fewer than two items."""
    return 0 if length < 2 else math.floor(Fraction(position * (places - 1), length - 1) + Fraction(1, 2))


def first(entries, **wanted):
    """🥇️ The first entry with every wanted member, in a list of none or one."""
    return [entry for entry in entries if all(entry[name] == value for name, value in wanted.items())][:1]


def value_key(value):
    """🔢️ A value in JSON number syntax as JavaScript prints it: integral values without a fraction."""
    return str(int(value)) if float(value).is_integer() and abs(value) < 1e21 else repr(float(value))


def counted(keys):
    """🧮️ ``{key, count}`` per distinct key, keys ascending by code point."""
    tally = {}
    for key in keys:
        tally[key] = tally.get(key, 0) + 1
    return [{"key": key, "count": tally[key]} for key in sorted(tally)]


def normalized(position, length):
    """📐️ A learner's position of an item as 0 (smallest) … 1 (largest); a one-item order sits at 0."""
    return position / max(length - 1, 1)


def mean(values):
    """➗️ The mean in the order the values were given."""
    total = 0.0
    for value in values:
        total += value
    return total / len(values)


def scaled(value, scale):
    """📏️ ``s(v)``: the value itself, or its decadic logarithm on a logarithmic scale."""
    return value if scale == "linear" else math.log10(value)


def nearest(value, task, dimension):
    """🧲️ The authored value of a matching dimension a guess is tallied under: the nearest of all the task's items on the quantity's scale, the smaller one of two equally near."""
    scale = dimension["quantity"]["scale"]
    authored = sorted({item["values"][dimension["id"]] for item in task["items"]})
    return min(authored, key=lambda candidate: abs(scaled(candidate, scale) - scaled(value, scale)))


def tallied(entry, task, dimension):
    """🗂️ The value a matching item result counts under: the assigned card value itself, or — where the sheet task hid the cards, so the result says whether it misses — the authored value nearest to the guess."""
    return nearest(entry["assigned"], task, dimension) if "miss" in entry else entry["assigned"]


def ordered(scored_task):
    """🧍️ Whether a sorting result carries an order of the learner: always where the keys showed, and where they were hidden only when some item was guessed — a task without any answer stands in sheet order."""
    return any("miss" not in entry or "guess" in entry for entry in scored_task["items"])


def crowd_view(quiz, results):
    """🗳️ ``crowdView(quiz, results)``: the run scores, then every task (every dimension of a matching) with its scores and answered items."""
    runs = [result for result in results if result["quiz"] == quiz["id"]]
    tasks = []
    for task in quiz["tasks"]:
        scored = [found for result in runs for found in first(result["tasks"], task=task["id"], kind=task["kind"])]
        if task["kind"] == "matching":
            for dimension in task["dimensions"]:
                parts = [part for scored_task in scored for part in first(scored_task["dimensions"], dimension=dimension["id"])]
                items = []
                for item in task["items"]:
                    keys = [value_key(tallied(entry, task, dimension)) for part in parts for entry in first(part["items"], item=item["id"]) if "assigned" in entry]
                    if keys:
                        items.append({"item": item["id"], "answers": len(keys), "counts": counted(keys)})
                tasks.append({"task": task["id"], "kind": "matching", "dimension": dimension["id"], "scores": binned(part["score"] for part in parts), "items": items})
            continue
        items = []
        for item in task["items"]:
            if task["kind"] == "classification":
                keys = [entry["assigned"] for scored_task in scored for entry in first(scored_task["items"], item=item["id"]) if "assigned" in entry]
                if keys:
                    items.append({"item": item["id"], "answers": len(keys), "counts": counted(keys)})
            else:
                orders = [(entry["position"], len(scored_task["items"])) for scored_task in scored if ordered(scored_task) for entry in first(scored_task["items"], item=item["id"])]
                if orders:
                    places = [0] * presented(task)
                    for position, length in orders:
                        places[place_bin(position, length, len(places))] += 1
                    items.append({"item": item["id"], "answers": len(orders), "meanPosition": mean([normalized(position, length) for position, length in orders]), "places": places})
        tasks.append({"task": task["id"], "kind": task["kind"], "scores": binned(scored_task["score"] for scored_task in scored), "items": items})
    return {"quiz": quiz["id"], "runs": len(runs), "scores": binned(result["score"] for result in runs), "tasks": tasks}


# endregion 🔖️Reference


# region 🔖️Corroboration
EDGES = list(range(0, 10 * SCORE_BINS, 10)) + [101]


def histogram(vector, where, scores, bins):
    """📶️ ``numpy.histogram`` over the whole percents (edges 0, 10, … 90, 101) must recount a score distribution."""
    recounted = [int(count) for count in numpy.histogram([percent(score) for score in scores], bins=EDGES)[0]]
    if recounted != bins or sum(bins) != len(scores):
        raise AssertionError("%s/%s: the scores %r disagree with numpy.histogram %r of %r" % (vector, where, bins, recounted, scores))


def corroborate(vector, quiz, results, view):
    """🔍️ ``collections.Counter`` must recount every count and place, ``numpy.mean`` recompute every mean position and ``numpy.histogram`` every score distribution."""
    runs = [result for result in results if result["quiz"] == quiz["id"]]
    definitions = {task["id"]: task for task in quiz["tasks"]}
    histogram(vector, "runs", [result["score"] for result in runs], view["scores"])
    if sum(view["scores"]) != view["runs"]:
        raise AssertionError("%s: %r run scores for %r runs" % (vector, sum(view["scores"]), view["runs"]))
    for crowd in view["tasks"]:
        scored = [found for result in runs for found in first(result["tasks"], task=crowd["task"], kind=crowd["kind"])]
        if crowd["kind"] == "matching":
            scored = [part for scored_task in scored for part in first(scored_task["dimensions"], dimension=crowd["dimension"])]
        histogram(vector, "%s/%s" % (crowd["task"], crowd.get("dimension", "-")), [entry["score"] for entry in scored], crowd["scores"])
        definition = definitions[crowd["task"]]
        if crowd["kind"] == "sorting":
            scored = [scored_task for scored_task in scored if not all("miss" in found and "guess" not in found for found in scored_task["items"])]
        named = {entry["item"] for entry in crowd["items"]}
        for item in definition["items"]:
            if item["id"] not in named and any("assigned" in found or crowd["kind"] == "sorting" for scored_task in scored for found in first(scored_task["items"], item=item["id"])):
                raise AssertionError("%s/%s/%s: an answered item is left out" % (vector, crowd["task"], item["id"]))
        for entry in crowd["items"]:
            answered = [(found, len(scored_task["items"])) for scored_task in scored for found in first(scored_task["items"], item=entry["item"]) if crowd["kind"] == "sorting" or "assigned" in found]
            if crowd["kind"] == "matching":
                dimension = next(candidate for candidate in definition["dimensions"] if candidate["id"] == crowd["dimension"])
                authored = numpy.unique(numpy.array([item["values"][crowd["dimension"]] for item in definition["items"]], dtype=float))
                transformed = authored if dimension["quantity"]["scale"] == "linear" else numpy.log10(authored)
                snap = lambda value: float(authored[int(numpy.argmin(numpy.abs(transformed - (value if dimension["quantity"]["scale"] == "linear" else numpy.log10(value)))))])
                answered = [({**found, "assigned": snap(found["assigned"])} if "miss" in found else found, length) for found, length in answered]
            if crowd["kind"] == "sorting":
                positions = [normalized(found["position"], length) for found, length in answered]
                if abs(float(numpy.mean(positions)) - entry["meanPosition"]) > TOLERANCE:
                    raise AssertionError("%s/%s/%s: the mean position is %r, numpy says %r" % (vector, crowd["task"], entry["item"], entry["meanPosition"], float(numpy.mean(positions))))
                count = presented(definitions[crowd["task"]])
                places = Counter((2 * found["position"] * (count - 1) + (length - 1)) // (2 * (length - 1)) if length > 1 else 0 for found, length in answered)
                if len(entry["places"]) != count or Counter({place: given for place, given in enumerate(entry["places"]) if given}) != places or sum(entry["places"]) != entry["answers"]:
                    raise AssertionError("%s/%s/%s: the places %r disagree with Counter %r" % (vector, crowd["task"], entry["item"], entry["places"], places))
                continue
            keys = [found["assigned"] if crowd["kind"] == "classification" else value_key(found["assigned"]) for found, _ in answered]
            if Counter(keys) != Counter({count["key"]: count["count"] for count in entry["counts"]}) or sum(Counter(keys).values()) != entry["answers"]:
                raise AssertionError("%s/%s/%s: the counts %r disagree with Counter %r" % (vector, crowd["task"], entry["item"], entry["counts"], Counter(keys)))


# endregion 🔖️Corroboration


# region 🔖️Handlers
def close(produced, committed):
    """🤏️ Structural equality with the 1e-12 tolerance on numbers."""
    if isinstance(produced, bool) or isinstance(committed, bool):
        return produced == committed
    if isinstance(produced, (int, float)) and isinstance(committed, (int, float)):
        return abs(produced - committed) <= TOLERANCE
    if isinstance(produced, dict) and isinstance(committed, dict):
        return produced.keys() == committed.keys() and all(close(produced[key], committed[key]) for key in produced)
    if isinstance(produced, list) and isinstance(committed, list):
        return len(produced) == len(committed) and all(close(left, right) for left, right in zip(produced, committed))
    return produced == committed


def crowds(ctx):
    """🗃️ The crowd view of every committed quiz and set of results, corroborated and held to the committed view."""
    vectors = json.loads(ctx.fixture_bytes(VECTORS))
    quizzes = {quiz["id"]: quiz for quiz in vectors["quizzes"]}
    produced = {}
    for vector in vectors["vectors"]:
        quiz = quizzes[vector["quiz"]]
        produced[vector["id"]] = crowd_view(quiz, vector["results"])
        corroborate(vector["id"], quiz, vector["results"], produced[vector["id"]])
        if not close(produced[vector["id"]], vector["expected"]):
            raise AssertionError("crowds/%s: the reference view %r differs from the committed %r" % (vector["id"], produced[vector["id"]], vector["expected"]))
    return Outcome(produced)


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter():
    """🧭️ Oracle role only."""
    return Adapter("python").oracle("crowds", crowds)


# endregion 🔖️Registration

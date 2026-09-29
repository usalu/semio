#!/usr/bin/env python3
"""📊️ Second implementation of the crowd view (design §17, schema ``CrowdView``), in Python.

What the learners answered in the submitted runs of one quiz, aggregated per task — per dimension for a
matching — and per item, semantically by item id because every learner's sheet differs. Written from the
design text and the schema: only results of the viewed quiz count (``runs``); tasks and matching
dimensions follow the quiz's definition order, items the task's definition order, and an item nobody
answered is left out; a classification item counts the assigned categories and a matching item the
assigned values, as ``{key, count}`` in ascending key order — keys are strings compared by code point,
a value rendered in JSON number syntax as JavaScript prints it (``120``, ``42.6``, ``0.027``) — and a
sorting item carries the mean of its normalized position ``position / (n − 1)`` over the learners who
ordered it (0 smallest … 1 largest; a one-item order sits at 0). Counts are recounted with
``collections.Counter`` and every mean is recomputed with ``numpy.mean`` before anything is projected.

@see ../../🧬️schema/🔣️.json
@see ../../🧫️fixtures/📊️crowd-view/🔣️.json
"""

# region 🔖️Imports
import json
from collections import Counter

import numpy

from semio_repo_test import Adapter, Outcome

# endregion 🔖️Imports


# region 🔖️Reference
VECTORS = "shared://📊️crowd-view/🔣️.json"
TOLERANCE = 1e-12


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


def crowd_view(quiz, results):
    """🗳️ ``crowdView(quiz, results)``: every task (every dimension of a matching) with its answered items."""
    runs = [result for result in results if result["quiz"] == quiz["id"]]
    tasks = []
    for task in quiz["tasks"]:
        scored = [scored for result in runs for scored in result["tasks"] if scored["task"] == task["id"] and scored["kind"] == task["kind"]]
        if task["kind"] == "matching":
            for dimension in task["dimensions"]:
                items = []
                for item in task["items"]:
                    keys = [value_key(entry["assigned"]) for scored_task in scored for part in scored_task["dimensions"] if part["dimension"] == dimension["id"] for entry in part["items"] if entry["item"] == item["id"]]
                    if keys:
                        items.append({"item": item["id"], "answers": len(keys), "counts": counted(keys)})
                tasks.append({"task": task["id"], "kind": "matching", "dimension": dimension["id"], "items": items})
            continue
        items = []
        for item in task["items"]:
            if task["kind"] == "classification":
                keys = [entry["assigned"] for scored_task in scored for entry in scored_task["items"] if entry["item"] == item["id"]]
                if keys:
                    items.append({"item": item["id"], "answers": len(keys), "counts": counted(keys)})
            else:
                positions = [normalized(entry["position"], len(scored_task["items"])) for scored_task in scored for entry in scored_task["items"] if entry["item"] == item["id"]]
                if positions:
                    items.append({"item": item["id"], "answers": len(positions), "meanPosition": mean(positions)})
        tasks.append({"task": task["id"], "kind": task["kind"], "items": items})
    return {"quiz": quiz["id"], "runs": len(runs), "tasks": tasks}


# endregion 🔖️Reference


# region 🔖️Corroboration
def corroborate(vector, quiz, results, view):
    """🔍️ ``collections.Counter`` must recount every count and ``numpy.mean`` recompute every mean position."""
    runs = [result for result in results if result["quiz"] == quiz["id"]]
    for crowd in view["tasks"]:
        for entry in crowd["items"]:
            if crowd["kind"] == "sorting":
                positions = [normalized(item["position"], len(scored["items"])) for result in runs for scored in result["tasks"] if scored["task"] == crowd["task"] for item in scored["items"] if item["item"] == entry["item"]]
                if abs(float(numpy.mean(positions)) - entry["meanPosition"]) > TOLERANCE:
                    raise AssertionError("%s/%s/%s: the mean position is %r, numpy says %r" % (vector, crowd["task"], entry["item"], entry["meanPosition"], float(numpy.mean(positions))))
                continue
            if crowd["kind"] == "classification":
                keys = [item["assigned"] for result in runs for scored in result["tasks"] if scored["task"] == crowd["task"] for item in scored["items"] if item["item"] == entry["item"]]
            else:
                keys = [value_key(item["assigned"]) for result in runs for scored in result["tasks"] if scored["task"] == crowd["task"] for part in scored["dimensions"] if part["dimension"] == crowd["dimension"] for item in part["items"] if item["item"] == entry["item"]]
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

#!/usr/bin/env python3
"""✅️ Second implementation of answer validity and completeness (design §5), in Python.

Written from the design text alone. An answer is valid when its kind matches the sheet task, every
item, category and dimension it names exists in that sheet task, every card index is in range and used
at most once per dimension, and a sorting order is a permutation of the sheet items; partial
classification and matching answers stay valid. Completeness is asked only of valid answers: every
sheet item assigned (in every dimension, for matching), and a recorded sorting answer is always complete.

@see ../../🧫️fixtures/✅️answer-validation/🔣️.json
"""

# region 🔖️Imports
import json
from collections import Counter

from semio_repo_test import Adapter, Outcome

# endregion 🔖️Imports


# region 🔖️Reference
VECTORS = "shared://✅️answer-validation/🔣️.json"


def answer_rejection(sheet_task, answer):
    """🚫️ ``answer-invalid`` for an answer the sheet task cannot hold, ``None`` otherwise."""
    if answer["kind"] != sheet_task["kind"]:
        return "answer-invalid"
    items = {item["id"] for item in sheet_task["items"]}
    if answer["kind"] == "classification":
        categories = {category["id"] for category in sheet_task["categories"]}
        return None if all(item in items and category in categories for item, category in answer["assignments"].items()) else "answer-invalid"
    if answer["kind"] == "sorting":
        return None if Counter(answer["order"]) == Counter(item["id"] for item in sheet_task["items"]) else "answer-invalid"
    cards = {dimension["id"]: len(dimension["cards"]) for dimension in sheet_task["dimensions"]}
    for dimension, assignments in answer["assignments"].items():
        if dimension not in cards:
            return "answer-invalid"
        indices = list(assignments.values())
        if len(set(indices)) != len(indices) or any(item not in items for item in assignments) or any(not 0 <= index < cards[dimension] for index in indices):
            return "answer-invalid"
    return None


def answer_complete(sheet_task, answer):
    """☑️ Whether a (valid) answer, or its absence, lets the run be submitted."""
    if answer is None:
        return False
    items = [item["id"] for item in sheet_task["items"]]
    if answer["kind"] == "classification":
        return all(item in answer["assignments"] for item in items)
    if answer["kind"] == "sorting":
        return True
    return all(dimension["id"] in answer["assignments"] and all(item in answer["assignments"][dimension["id"]] for item in items) for dimension in sheet_task["dimensions"])


def verdict(sheet_task, answer):
    """⚖️ The projected verdict: the rejection, and completeness for an answer that is valid or absent."""
    rejection = None if answer is None else answer_rejection(sheet_task, answer)
    return {"rejection": rejection} if rejection is not None else {"rejection": None, "complete": answer_complete(sheet_task, answer)}


# endregion 🔖️Reference


# region 🔖️Handlers
def verdicts(ctx):
    """🗃️ Every committed (sheet task, answer) pair, judged and held to its committed verdict."""
    vectors = json.loads(ctx.fixture_bytes(VECTORS))
    tasks = {task["id"]: task for task in vectors["sheetTasks"]}
    produced = {}
    for vector in vectors["vectors"]:
        produced[vector["id"]] = verdict(tasks[vector["sheetTask"]], vector.get("answer"))
        if produced[vector["id"]] != vector["expected"]:
            raise AssertionError("verdicts/%s: the reference says %r, the committed vector %r" % (vector["id"], produced[vector["id"]], vector["expected"]))
    return Outcome(produced)


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter():
    """🧭️ Oracle role only."""
    return Adapter("python").oracle("verdicts", verdicts)


# endregion 🔖️Registration

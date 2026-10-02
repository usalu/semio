#!/usr/bin/env python3
"""✅️ Second implementation of answer validity and completeness (design §5), in Python.

Written from the design text alone. An answer is valid when its kind matches the sheet task, every
item, category and dimension it names exists in that sheet task, every card index is in range and used
at most once per dimension, and a sorting order is a permutation of the sheet items whose optional
numeric ``guesses`` (item id to guess in the quantity's base unit) name sheet items only, are finite
numbers (positive on a logarithmic quantity) and stand in non-decreasing guess order along the order,
ties allowed and unguessed items unconstrained; partial classification and matching answers stay
valid. Completeness is asked only of valid answers: every sheet item assigned (in every dimension,
for matching), and a recorded sorting answer is always complete.

@see ../../🧫️fixtures/✅️answer-validation/🔣️.json
"""

# region 🔖️Imports
import json
import math
from collections import Counter

from semio_repo_test import Adapter, Outcome

# endregion 🔖️Imports


# region 🔖️Reference
VECTORS = "shared://✅️answer-validation/🔣️.json"


def guesses_fit(sheet_task, answer):
    """🔢️ Whether the optional ``guesses`` of a sorting answer are an object of finite numbers over sheet items, in non-decreasing order along ``order``."""
    guesses = answer.get("guesses", {})
    if not isinstance(guesses, dict):
        return False
    items = {item["id"] for item in sheet_task["items"]}
    logarithmic = sheet_task["quantity"]["scale"] == "logarithmic"
    for item, guess in guesses.items():
        if item not in items or isinstance(guess, bool) or not isinstance(guess, (int, float)) or not math.isfinite(guess) or (logarithmic and guess <= 0):
            return False
    guessed = [guesses[item] for item in answer["order"] if item in guesses]
    return all(before <= after for before, after in zip(guessed, guessed[1:]))


def answer_rejection(sheet_task, answer):
    """🚫️ ``answer-invalid`` for an answer the sheet task cannot hold, ``None`` otherwise."""
    if answer["kind"] != sheet_task["kind"]:
        return "answer-invalid"
    items = {item["id"] for item in sheet_task["items"]}
    if answer["kind"] == "classification":
        categories = {category["id"] for category in sheet_task["categories"]}
        return None if all(item in items and category in categories for item, category in answer["assignments"].items()) else "answer-invalid"
    if answer["kind"] == "sorting":
        permutation = Counter(answer["order"]) == Counter(item["id"] for item in sheet_task["items"])
        return None if permutation and guesses_fit(sheet_task, answer) else "answer-invalid"
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
    for vector in vectors["vectors"] + vectors["malformed"]:
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

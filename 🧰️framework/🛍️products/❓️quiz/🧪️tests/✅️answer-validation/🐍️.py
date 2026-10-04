#!/usr/bin/env python3
"""✅️ Second implementation of answer validity and completeness (design §5, challenge design §3.3), in Python.

Written from the design text alone. An answer is valid when its kind matches the sheet task, every
item, category and dimension it names exists in that sheet task and a sorting order is a permutation of
the sheet items. What else it may carry follows the sheet task. Where the keys show (a sorting with
``keys``, a matching whose dimensions carry ``cards``) the answer assigns: a sorting is its order alone
and a matching its ``assignments`` — required, as they always were —, every card index in range and used
at most once per dimension; ``guesses`` are then invalid. Where the keys are hidden the guesses are the
answer — optional, an answer without them is valid and incomplete —: a sorting's
``guesses`` (item id to guess in the quantity's base unit) name sheet items only, are finite numbers
(positive on a logarithmic quantity) and stand in non-decreasing guess order along the order, ties
allowed and unguessed items unconstrained; a matching's ``guesses`` name sheet dimensions and items and
are finite numbers (positive on a logarithmic quantity), and ``assignments`` are then invalid. Partial
answers stay valid. Completeness is asked only of valid answers: every sheet item assigned or guessed
(in every dimension, for matching); a recorded sorting order is complete where the keys show.

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


def keys_hidden(sheet_task):
    """🙈️ Whether a sheet task hides its keys: a sorting without ``keys``, a matching with a dimension without ``cards``; a classification has no key to guess."""
    if sheet_task["kind"] == "sorting":
        return "keys" not in sheet_task
    return sheet_task["kind"] == "matching" and any("cards" not in dimension for dimension in sheet_task["dimensions"])


def numbers_fit(guesses, items, logarithmic):
    """🔢️ Whether guesses are an object of finite numbers over sheet items, positive on a logarithmic quantity."""
    if not isinstance(guesses, dict):
        return False
    return all(item in items and not isinstance(guess, bool) and isinstance(guess, (int, float)) and math.isfinite(guess) and not (logarithmic and guess <= 0) for item, guess in guesses.items())


def guesses_fit(sheet_task, answer):
    """🪜️ Whether the ``guesses`` of a sorting answer fit their sheet items and stand in non-decreasing order along ``order``."""
    guesses = answer.get("guesses", {})
    if not numbers_fit(guesses, {item["id"] for item in sheet_task["items"]}, sheet_task["quantity"]["scale"] == "logarithmic"):
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
        if Counter(answer["order"]) != Counter(item["id"] for item in sheet_task["items"]):
            return "answer-invalid"
        if not keys_hidden(sheet_task):
            return "answer-invalid" if "guesses" in answer else None
        return None if guesses_fit(sheet_task, answer) else "answer-invalid"
    dimensions = {dimension["id"]: dimension for dimension in sheet_task["dimensions"]}
    if keys_hidden(sheet_task):
        if "assignments" in answer or not isinstance(answer.get("guesses", {}), dict):
            return "answer-invalid"
        for dimension, guesses in answer.get("guesses", {}).items():
            if dimension not in dimensions or not numbers_fit(guesses, items, dimensions[dimension]["quantity"]["scale"] == "logarithmic"):
                return "answer-invalid"
        return None
    if "guesses" in answer or "assignments" not in answer:
        return "answer-invalid"
    for dimension, assignments in answer["assignments"].items():
        if dimension not in dimensions:
            return "answer-invalid"
        indices = list(assignments.values())
        if len(set(indices)) != len(indices) or any(item not in items for item in assignments) or any(not 0 <= index < len(dimensions[dimension]["cards"]) for index in indices):
            return "answer-invalid"
    return None


def answer_complete(sheet_task, answer):
    """☑️ Whether a (valid) answer, or its absence, lets an untimed run be submitted."""
    if answer is None:
        return False
    items = [item["id"] for item in sheet_task["items"]]
    if answer["kind"] == "classification":
        return all(item in answer["assignments"] for item in items)
    if answer["kind"] == "sorting":
        return not keys_hidden(sheet_task) or all(item in answer.get("guesses", {}) for item in items)
    chosen = answer.get("guesses" if keys_hidden(sheet_task) else "assignments", {})
    return all(dimension["id"] in chosen and all(item in chosen[dimension["id"]] for item in items) for dimension in sheet_task["dimensions"])


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

#!/usr/bin/env python3
"""🃏️ Second implementation of the quiz sheet (design §4), in Python, over numpy's MT19937.

Written from the design text alone — never from the TypeScript or Rust twins. The random stream is
numpy's ``MT19937`` seeded through its public legacy seeding (``init_genrand``); the task order, the
item draw, the category and card shuffles, the no-accidental-solution rotation of a sorting task and
the solution-free projection are this file's reading of §4, so a sheet both twins agree on is a sheet
two independent readings of the design agree on.

@see ../../🧬️schema/🔣️.json
@see ../../🧫️fixtures/🃏️sheet-assembly/🔣️.json
"""

# region 🔖️Imports
import json

import numpy

from semio_repo_test import Adapter, Outcome

# endregion 🔖️Imports


# region 🔖️Randomness
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


# region 🔖️Sheet
VECTORS = "shared://🃏️sheet-assembly/🔣️.json"


def ascending(task, items):
    """📈️ The true ascending order of drawn sorting items: by value, ties by definition index."""
    index = {item["id"]: position for position, item in enumerate(task["items"])}
    return sorted(items, key=lambda item: (item["value"], index[item["id"]]))


def sheet_task(generator, task):
    """🧾️ One task's solution-free sheet, consuming the stream in the order §4 fixes."""
    items = shuffle(generator, task["items"])
    if "draw" in task and task["draw"] < len(items):
        items = items[: task["draw"]]
    built = {"kind": task["kind"], "id": task["id"], "title": task["title"], "prompt": task["prompt"]}
    if task["kind"] == "classification":
        if "axes" in task:
            built["axes"] = task["axes"]
        built["categories"] = shuffle(generator, task["categories"])
    elif task["kind"] == "sorting":
        built["quantity"] = task["quantity"]
        if len(items) >= 2 and [item["id"] for item in items] == [item["id"] for item in ascending(task, items)]:
            items = items[1:] + items[:1]
    else:
        built["dimensions"] = [{"id": dimension["id"], "quantity": dimension["quantity"], "cards": shuffle(generator, [item["values"][dimension["id"]] for item in items])} for dimension in task["dimensions"]]
    built["items"] = [{"id": item["id"], "label": item["label"]} for item in items]
    return built


def sheet_of(quiz, seed):
    """📄️ ``sheet(quiz, seed)``: task order first, then every task in definition order."""
    generator = Mt19937(seed)
    order = shuffle(generator, range(len(quiz["tasks"])))
    built = [sheet_task(generator, task) for task in quiz["tasks"]]
    return {"quiz": quiz["id"], "seed": seed, "title": quiz["title"], "description": quiz["description"], "tasks": [built[position] for position in order]}


# endregion 🔖️Sheet


# region 🔖️Handlers
def sheets(ctx):
    """🗃️ Every committed (quiz, seed) pair, rebuilt and held to its committed sheet."""
    vectors = json.loads(ctx.fixture_bytes(VECTORS))
    quizzes = {quiz["id"]: quiz for quiz in vectors["quizzes"]}
    produced = {}
    for vector in vectors["sheets"]:
        produced[vector["id"]] = sheet_of(quizzes[vector["quiz"]], vector["seed"])
        if produced[vector["id"]] != vector["sheet"]:
            raise AssertionError("sheets/%s: the reference sheet differs from the committed one" % vector["id"])
    return Outcome(produced)


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter():
    """🧭️ Oracle role only: the second implementation is the reference the twins are judged against."""
    return Adapter("python").oracle("sheets", sheets)


# endregion 🔖️Registration

#!/usr/bin/env python3
"""🃏️ Second implementation of the quiz sheet (design §4, challenge design §3.2), in Python, over numpy's MT19937.

Written from the design text alone — never from the TypeScript or Rust twins. The random stream is
numpy's ``MT19937`` seeded through its public legacy seeding (``init_genrand``); the task order, the
item draw, the category and card shuffles, the no-accidental-solution rotation of a sorting task and
the projection are this file's reading of the design, so a sheet both twins agree on is a sheet
two independent readings of the design agree on. The sheet is a function of ``(quiz, seed, challenge)``:
the stream is consumed alike at every challenge, so one seed deals the same items in the same order;
where the challenge shows the keys a sorting carries the ascending true values of its presented items
and a matching its cards, where it hides them a sorting carries no keys, a matching no cards (the card
shuffle is still drawn), a category no description and an axis no numbers, every profile value being its
share of the axis range; a timed challenge gives every task its seconds. numpy recomputes the ladder of
keys (``numpy.sort``), the cards and the shares (``numpy.interp``) before a sheet is projected.

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
CHALLENGE_RULES = {
    "easy": {"keys": True, "hints": True, "timed": False, "par": 100},
    "medium": {"keys": True, "hints": False, "timed": False, "par": 200},
    "hard": {"keys": False, "hints": False, "timed": False, "par": 300},
    "expert": {"keys": False, "hints": False, "timed": True, "par": 400},
}
TASK_SECONDS = {"base": 30, "classification": 8, "sorting": 12, "matching": 12}


def ascending(task, items):
    """📈️ The true ascending order of drawn sorting items: by value, ties by definition index."""
    index = {item["id"]: position for position, item in enumerate(task["items"])}
    return sorted(items, key=lambda item: (item["value"], index[item["id"]]))


def icon_of(source):
    """🖼️ The icon of a task, an item or a dimension as the optional member its presentation carries."""
    return {"icon": source["icon"]} if "icon" in source else {}


def short_of(source):
    """🩳️ The short label of an item or an axis as the optional member its presentation carries."""
    return {"short": source["short"]} if "short" in source else {}


def hidden_category(axes, category):
    """🙈️ A category where the keys are hidden: no description, and every profile value its share of the axis range."""
    ranges = {axis["id"]: axis for axis in axes}
    built = {member: value for member, value in category.items() if member not in ("description", "profile")}
    if "profile" in category:
        built["profile"] = {axis: (value - ranges[axis]["min"]) / (ranges[axis]["max"] - ranges[axis]["min"]) for axis, value in category["profile"].items() if axis in ranges}
    return built


def task_seconds(kind, items, dimensions):
    """⏳️ The seconds a timed task allows: the base plus the kind's seconds per item, per dimension for a matching."""
    return TASK_SECONDS["base"] + TASK_SECONDS[kind] * items * (dimensions if kind == "matching" else 1)


def sheet_task(generator, task, rules):
    """🧾️ One task's sheet, consuming the stream in the same order at every challenge; the keys only where the challenge shows them, the seconds only where it is timed."""
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
    """📄️ ``sheet(quiz, seed, challenge)``: keep the first task first, shuffle the rest, then present tasks in definition order."""
    generator = Mt19937(seed)
    shuffled_order = shuffle(generator, range(len(quiz["tasks"])))
    order = [0, *[position for position in shuffled_order if position != 0]] if shuffled_order else []
    built = [sheet_task(generator, task, CHALLENGE_RULES[challenge]) for task in quiz["tasks"]]
    return {"quiz": quiz["id"], "seed": seed, "challenge": challenge, "title": quiz["title"], "description": quiz["description"], "tasks": [built[position] for position in order]}


# endregion 🔖️Sheet


# region 🔖️Corroboration
SOLUTIONS = ("value", "values", "category", "explanation", "familiar")


def dealt(sheet):
    """🂠️ What the random stream decides of a sheet: the task order, the items of every task in order, the category order and the dimensions."""
    return [(task["id"], [item["id"] for item in task["items"]], [category["id"] for category in task.get("categories", [])], [dimension["id"] for dimension in task.get("dimensions", [])]) for task in sheet["tasks"]]


def carried(value, members):
    """🔎️ Whether a JSON value carries one of the named members at any depth."""
    if isinstance(value, dict):
        return any(member in members for member in value) or any(carried(child, members) for child in value.values())
    return isinstance(value, list) and any(carried(child, members) for child in value)


def corroborate(vector, quiz, sheet, sheets):
    """🔍️ What the design promises of a sheet beyond its members: one deal per seed at every challenge, numpy's sort of the true values as the ladder of keys, the true values as the cards, no number where the keys are hidden and the seconds exactly on a timed sheet."""
    rules = CHALLENGE_RULES[sheet["challenge"]]
    for other in sheets:
        if dealt(other) != dealt(sheet):
            raise AssertionError("sheets/%s: the same seed deals other items at another challenge" % vector)
    tasks = {task["id"]: task for task in quiz["tasks"]}
    for presented in sheet["tasks"]:
        task = tasks[presented["id"]]
        items = {item["id"]: item for item in task["items"]}
        if carried(presented["items"], SOLUTIONS):
            raise AssertionError("sheets/%s/%s: a sheet item carries a solution" % (vector, presented["id"]))
        if ("seconds" in presented) != rules["timed"] or (rules["timed"] and presented["seconds"] != 30 + {"classification": 8, "sorting": 12, "matching": 12 * len(presented.get("dimensions", []))}[presented["kind"]] * len(presented["items"])):
            raise AssertionError("sheets/%s/%s: the seconds are not those of a timed task" % (vector, presented["id"]))
        if not rules["keys"] and ("keys" in presented or any("cards" in dimension for dimension in presented.get("dimensions", [])) or any(not set(axis) <= {"id", "label", "short"} for axis in presented.get("axes", []))):
            raise AssertionError("sheets/%s/%s: a sheet that hides the keys carries one" % (vector, presented["id"]))
        if presented["kind"] == "sorting" and rules["keys"] and presented["keys"] != numpy.sort(numpy.array([items[item["id"]]["value"] for item in presented["items"]], dtype=float)).tolist():
            raise AssertionError("sheets/%s/%s: the keys are not numpy's ascending sort of the presented values" % (vector, presented["id"]))
        if presented["kind"] == "matching" and rules["keys"]:
            for dimension in presented["dimensions"]:
                if numpy.sort(numpy.array(dimension["cards"], dtype=float)).tolist() != numpy.sort(numpy.array([items[item["id"]]["values"][dimension["id"]] for item in presented["items"]], dtype=float)).tolist():
                    raise AssertionError("sheets/%s/%s: the cards are not the presented values" % (vector, presented["id"]))
        if presented["kind"] == "classification" and not rules["keys"]:
            axes = {axis["id"]: axis for axis in task.get("axes", [])}
            for category in presented["categories"]:
                authored = next(entry for entry in task["categories"] if entry["id"] == category["id"])
                if "description" in category or ("profile" in category) != ("profile" in authored):
                    raise AssertionError("sheets/%s/%s: a hidden category keeps its description or loses its profile" % (vector, presented["id"]))
                for axis, share in category.get("profile", {}).items():
                    span = numpy.array([axes[axis]["min"], axes[axis]["max"]], dtype=float)
                    if abs(float(numpy.interp(authored["profile"][axis], span, [0.0, 1.0])) - share) > 1e-12:
                        raise AssertionError("sheets/%s/%s: %s is not its share of the axis range by numpy.interp" % (vector, presented["id"], axis))


# endregion 🔖️Corroboration


# region 🔖️Handlers
def sheets(ctx):
    """🗃️ Every committed (quiz, seed, challenge) triple, rebuilt, corroborated and held to its committed sheet."""
    vectors = json.loads(ctx.fixture_bytes(VECTORS))
    quizzes = {quiz["id"]: quiz for quiz in vectors["quizzes"]}
    produced = {}
    for vector in vectors["sheets"]:
        produced[vector["id"]] = sheet_of(quizzes[vector["quiz"]], vector["seed"], vector["challenge"])
        if produced[vector["id"]] != vector["sheet"]:
            raise AssertionError("sheets/%s: the reference sheet differs from the committed one" % vector["id"])
    for vector in vectors["sheets"]:
        corroborate(vector["id"], quizzes[vector["quiz"]], produced[vector["id"]], [produced[other["id"]] for other in vectors["sheets"] if (other["quiz"], other["seed"]) == (vector["quiz"], vector["seed"])])
    return Outcome(produced)


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter():
    """🧭️ Oracle role only: the second implementation is the reference the twins are judged against."""
    return Adapter("python").oracle("sheets", sheets)


# endregion 🔖️Registration

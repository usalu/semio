"""🧗️ Randomness, the challenge ladder with its hints and clock, and the sheet of a run."""

from __future__ import annotations

# region 🔖️Imports
import math
import numpy
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

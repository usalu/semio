"""✅️ The answer rules, the scoring of a run and the badges it earns."""

from __future__ import annotations

# region 🔖️Imports
import math
from collections import Counter

from semio_quiz_lifecycle_sheet import CHALLENGE_RULES, challenge_meets, scaled, reach, misses
# endregion 🔖️Imports


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

"""🎞️ The replay of committed sequences and the play of the site catalog."""

from __future__ import annotations

# region 🔖️Imports
import json
import os

from semio_quiz_lifecycle_sheet import fnv1a32, sheet_of
from semio_quiz_lifecycle_scoring import keys_hidden
from semio_quiz_lifecycle_learner import (
    empty_handle_state,
    decide_handle,
    evolve_handle,
    empty_learner_state,
    evolve_learner,
    decide_learner,
)
# endregion 🔖️Imports


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

#!/usr/bin/env python3
"""🧾️ Second implementation of the learner lifecycle (design §3–§8), in Python.

Written from the design text alone — never from the TypeScript or Rust twins. It composes everything a
proctor decides with: handle normalisation, the roster decider (identify-learner), the learner decider
(start-run, record-answer, submit-run) with the sheet of §4 over numpy's MT19937, the answer rules of
§5, the scoring of §6 and the badges of §7, plus the folding ``evolve`` and the learner and run views.
State is a plain dictionary here; only decisions and schema views are projected, so no twin's private
state shape is ever compared.

@see ../../🧫️fixtures/🧾️learner-lifecycle/🔣️.json
"""

# region 🔖️Imports
import copy
import json
import math
from collections import Counter

import numpy

from semio_repo_test import Adapter, Outcome

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


# region 🔖️Sheet
def ascending(task, items):
    """📈️ The true ascending order of sorting items: by value, ties by definition index."""
    index = {item["id"]: position for position, item in enumerate(task["items"])}
    return sorted(items, key=lambda item: (item["value"], index[item["id"]]))


def sheet_task(generator, task):
    """🗒️ One task's solution-free sheet, consuming the stream in the order §4 fixes."""
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
    """🃏️ ``sheet(quiz, seed)``: task order first, then every task in definition order."""
    generator = Mt19937(seed)
    order = shuffle(generator, range(len(quiz["tasks"])))
    built = [sheet_task(generator, task) for task in quiz["tasks"]]
    return {"quiz": quiz["id"], "seed": seed, "title": quiz["title"], "description": quiz["description"], "tasks": [built[position] for position in order]}


# endregion 🔖️Sheet


# region 🔖️Answers
def answer_rejection(sheet_task_, answer):
    """🚫️ ``answer-invalid`` for an answer the sheet task cannot hold, ``None`` otherwise."""
    if answer["kind"] != sheet_task_["kind"]:
        return "answer-invalid"
    items = {item["id"] for item in sheet_task_["items"]}
    if answer["kind"] == "classification":
        categories = {category["id"] for category in sheet_task_["categories"]}
        return None if all(item in items and category in categories for item, category in answer["assignments"].items()) else "answer-invalid"
    if answer["kind"] == "sorting":
        return None if Counter(answer["order"]) == Counter(item["id"] for item in sheet_task_["items"]) else "answer-invalid"
    cards = {dimension["id"]: len(dimension["cards"]) for dimension in sheet_task_["dimensions"]}
    for dimension, assignments in answer["assignments"].items():
        if dimension not in cards:
            return "answer-invalid"
        indices = list(assignments.values())
        if len(set(indices)) != len(indices) or any(item not in items for item in assignments) or any(not 0 <= index < cards[dimension] for index in indices):
            return "answer-invalid"
    return None


def answer_complete(sheet_task_, answer):
    """☑️ Whether a valid answer, or its absence, lets the run be submitted."""
    if answer is None:
        return False
    items = [item["id"] for item in sheet_task_["items"]]
    if answer["kind"] == "classification":
        return all(item in answer["assignments"] for item in items)
    if answer["kind"] == "sorting":
        return True
    return all(dimension["id"] in answer["assignments"] and all(item in answer["assignments"][dimension["id"]] for item in items) for dimension in sheet_task_["dimensions"])


# endregion 🔖️Answers


# region 🔖️Scoring
def scaled(value, scale):
    """📐️ ``s(v)``: the value itself, or its decadic logarithm on a logarithmic scale."""
    return value if scale == "linear" else math.log10(value)


def sign(value):
    """➕️ The sign of a difference: −1, 0 or 1."""
    return (value > 0) - (value < 0)


def explained(result, item):
    """💬️ Carries the item's explanation into its result when the item has one."""
    if "explanation" in item:
        result["explanation"] = item["explanation"]
    return result


def sorting_result(task, sheet_task_, answer):
    """📏️ Magnitude-weighted pair concordance over the learner's order."""
    items = {item["id"]: item for item in task["items"]}
    index = {item["id"]: position for position, item in enumerate(task["items"])}
    scale = task["quantity"]["scale"]
    values = [items[identifier]["value"] for identifier in answer["order"]]
    total = 0.0
    discordant = 0.0
    for first in range(len(values)):
        for second in range(first + 1, len(values)):
            weight = abs(scaled(values[first], scale) - scaled(values[second], scale))
            total += weight
            if values[first] > values[second]:
                discordant += weight
    ranks = {identifier: rank for rank, identifier in enumerate(sorted((item["id"] for item in sheet_task_["items"]), key=lambda identifier: (items[identifier]["value"], index[identifier])))}
    results = [explained({"item": identifier, "value": items[identifier]["value"], "position": position, "rank": ranks[identifier]}, items[identifier]) for position, identifier in enumerate(answer["order"])]
    return {"kind": "sorting", "task": task["id"], "score": 1 - discordant / total if total > 0 else 1, "items": results}


def matching_result(task, sheet_task_, answer):
    """🧷️ Per-dimension weighted concordance with half-weighted learner-made ties, averaged over dimensions."""
    items = {item["id"]: item for item in task["items"]}
    order = [item["id"] for item in sheet_task_["items"]]
    cards = {dimension["id"]: dimension["cards"] for dimension in sheet_task_["dimensions"]}
    dimensions = []
    for dimension in task["dimensions"]:
        scale = dimension["quantity"]["scale"]
        truth = [items[identifier]["values"][dimension["id"]] for identifier in order]
        assigned = [cards[dimension["id"]][answer["assignments"][dimension["id"]][identifier]] for identifier in order]
        total = 0.0
        discordant = 0.0
        for first in range(len(truth)):
            for second in range(first + 1, len(truth)):
                weight = abs(scaled(truth[first], scale) - scaled(truth[second], scale))
                total += weight
                if sign(truth[first] - truth[second]) * sign(assigned[first] - assigned[second]) < 0:
                    discordant += weight
                elif assigned[first] == assigned[second] and truth[first] != truth[second]:
                    discordant += weight / 2
        results = [explained({"item": identifier, "assigned": value, "correct": correct}, items[identifier]) for identifier, value, correct in zip(order, assigned, truth)]
        dimensions.append({"dimension": dimension["id"], "score": 1 - discordant / total if total > 0 else 1, "items": results})
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
    """🕸️ Mean credit over the sheet items."""
    items = {item["id"]: item for item in task["items"]}
    results = []
    total = 0.0
    for sheet_item in sheet_task_["items"]:
        item = items[sheet_item["id"]]
        earned = credit(task, answer["assignments"][item["id"]], item["category"])
        total += earned
        results.append(explained({"item": item["id"], "assigned": answer["assignments"][item["id"]], "correct": item["category"], "credit": earned}, item))
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


def score_task(task, sheet_task_, answer):
    """🧮️ ``scoreTask`` dispatched on the task kind, or ``None`` for an answer §5 holds invalid or incomplete or a task the sheet cannot resolve."""
    if not resolvable(task, sheet_task_) or answer_rejection(sheet_task_, answer) is not None or not answer_complete(sheet_task_, answer):
        return None
    return {"classification": classification_result, "sorting": sorting_result, "matching": matching_result}[task["kind"]](task, sheet_task_, answer)


def score_run(quiz, sheet, answers):
    """🏁️ ``scoreRun``: task results in sheet order, the run score their mean (0 without tasks), or ``None`` when any task scores none."""
    tasks = {task["id"]: task for task in quiz["tasks"]}
    results = [score_task(tasks[sheet_task_["id"]], sheet_task_, answers[sheet_task_["id"]]) if sheet_task_["id"] in tasks and sheet_task_["id"] in answers else None for sheet_task_ in sheet["tasks"]]
    if any(result is None for result in results):
        return None
    total = 0.0
    for result in results:
        total += result["score"]
    return {"quiz": quiz["id"], "score": total / len(results) if results else 0, "tasks": results}


# endregion 🔖️Scoring


# region 🔖️Badges
def earns(rule, quizzes, results):
    """🧐️ Whether one badge rule holds over the learner's submitted results."""
    if rule["kind"] == "perfect-quiz":
        return any(result["quiz"] == rule["quiz"] and result["score"] == 1 for result in results)
    if rule["kind"] == "perfect-tasks":
        selected = [(quiz["id"], task["id"]) for quiz in quizzes for task in quiz["tasks"] if rule.get("taskKind", task["kind"]) == task["kind"] and rule.get("quiz", quiz["id"]) == quiz["id"]]
        return len(selected) > 0 and all(any(result["quiz"] == quiz and any(scored["task"] == task and scored["score"] == 1 for scored in result["tasks"]) for result in results) for quiz, task in selected)
    return all(any(result["quiz"] == quiz["id"] for result in results) for quiz in quizzes)


def earned_badges(badges, quizzes, results, held):
    """🏅️ The ids of the badges newly earned, in catalog order."""
    return [badge["id"] for badge in badges if badge["id"] not in held and earns(badge["rule"], quizzes, results)]


# endregion 🔖️Badges


# region 🔖️Roster
def normalize_handle(handle):
    """🪪️ ``{display, key}`` of a handle — trimmed, inner whitespace runs collapsed, key lowercased — or ``None``."""
    display = " ".join(handle.split())
    return {"display": display, "key": display.lower()} if 1 <= len(display) <= 64 else None


def decide_roster(state, command, now):
    """🛂️ identify-learner: anonymous learners are always new, a claimed handle key recalls its learner."""
    identity = command["identity"]
    if identity["kind"] == "anonymous":
        return {"events": [{"type": "learner-registered", "learner": command["learner"], "identity": {"kind": "anonymous"}, "at": now}]}
    handle = normalize_handle(identity["handle"])
    if handle is None:
        return {"rejection": "handle-invalid"}
    if handle["key"] in state:
        return {"events": [{"type": "learner-recalled", "learner": state[handle["key"]], "at": now}]}
    return {"events": [{"type": "learner-registered", "learner": command["learner"], "identity": {"kind": identity["kind"], "handle": handle["display"]}, "at": now}]}


def evolve_roster(state, event):
    """🗳️ A registered pseudonym or name claims its handle key."""
    if event["type"] != "learner-registered" or event["identity"]["kind"] == "anonymous":
        return state
    return {**state, normalize_handle(event["identity"]["handle"])["key"]: event["learner"]}


# endregion 🔖️Roster


# region 🔖️Learner
def empty_learner_state(learner):
    """🌱️ A learner nothing has happened to yet."""
    return {"learner": learner, "identity": None, "runs": [], "badges": [], "best": {}, "reachedAt": None, "lastActivity": None}


def run_of(state, run):
    """🔎️ One run of the learner, or ``None``."""
    return next((candidate for candidate in state["runs"] if candidate["run"] == run), None)


def evolve_learner(state, event):
    """🗂️ Folds one learner event into the state."""
    state = copy.deepcopy(state)
    state["lastActivity"] = event["at"] if state["lastActivity"] is None else max(state["lastActivity"], event["at"])
    kind = event["type"]
    if kind == "learner-registered":
        state["identity"] = event["identity"]
    elif kind == "run-started":
        state["runs"].append({"run": event["run"], "quiz": event["quiz"], "revision": event["revision"], "seed": event["seed"], "status": "open", "answers": {}, "result": None, "startedAt": event["at"], "submittedAt": None})
    elif kind == "run-voided":
        run_of(state, event["run"])["status"] = "voided"
    elif kind == "answer-recorded":
        run_of(state, event["run"])["answers"][event["task"]] = event["answer"]
    elif kind == "run-submitted":
        run = run_of(state, event["run"])
        run["status"], run["result"], run["submittedAt"] = "submitted", event["result"], event["at"]
        quiz = event["result"]["quiz"]
        if quiz not in state["best"] or event["result"]["score"] > state["best"][quiz]:
            state["best"][quiz] = event["result"]["score"]
            state["reachedAt"] = event["at"]
    elif kind == "badge-awarded":
        state["badges"].append({"badge": event["badge"], "run": event["run"], "at": event["at"]})
    return state


def decide_learner(state, command, context):
    """🏛️ start-run, record-answer and submit-run against the learner's state and the current quizzes."""
    now, learner = context["now"], state["learner"]
    if command["type"] == "start-run":
        if state["identity"] is None:
            return {"rejection": "unknown-learner"}
        if command["quiz"] not in context["quizzes"]:
            return {"rejection": "unknown-quiz"}
        current = context["quizzes"][command["quiz"]]
        started = {"type": "run-started", "learner": learner, "run": command["run"], "quiz": command["quiz"], "revision": current["revision"], "seed": fnv1a32(command["run"]), "at": now}
        open_run = next((run for run in state["runs"] if run["quiz"] == command["quiz"] and run["status"] == "open"), None)
        if open_run is None:
            return {"events": [started]}
        if open_run["revision"] == current["revision"]:
            return {"rejection": "run-open"}
        return {"events": [{"type": "run-voided", "learner": learner, "run": open_run["run"], "at": now}, started]}
    run = run_of(state, command["run"])
    if run is None:
        return {"rejection": "unknown-run"}
    if run["status"] != "open":
        return {"rejection": "run-closed"}
    current = context["quizzes"].get(run["quiz"])
    stale = current is None or current["revision"] != run["revision"]
    if command["type"] == "record-answer":
        if stale:
            return {"rejection": "quiz-revised"}
        sheet_task_ = next((task for task in sheet_of(current["quiz"], run["seed"])["tasks"] if task["id"] == command["task"]), None)
        if sheet_task_ is None:
            return {"rejection": "unknown-task"}
        if answer_rejection(sheet_task_, command["answer"]) is not None:
            return {"rejection": "answer-invalid"}
        return {"events": [{"type": "answer-recorded", "learner": learner, "run": run["run"], "task": command["task"], "answer": command["answer"], "at": now}]}
    if stale:
        return {"events": [{"type": "run-voided", "learner": learner, "run": run["run"], "at": now}]}
    sheet = sheet_of(current["quiz"], run["seed"])
    if not all(answer_complete(task, run["answers"].get(task["id"])) for task in sheet["tasks"]):
        return {"rejection": "run-incomplete"}
    result = score_run(current["quiz"], sheet, run["answers"])
    if result is None:
        return {"rejection": "run-incomplete"}
    results = [candidate["result"] for candidate in state["runs"] if candidate["status"] == "submitted"] + [result]
    held = [award["badge"] for award in state["badges"]]
    quizzes = [entry["quiz"] for entry in context["quizzes"].values()]
    awarded = [{"type": "badge-awarded", "learner": learner, "badge": badge, "run": run["run"], "at": now} for badge in earned_badges(context["catalog"]["badges"], quizzes, results, held)]
    return {"events": [{"type": "run-submitted", "learner": learner, "run": run["run"], "result": result, "at": now}] + awarded}


# endregion 🔖️Learner


# region 🔖️Views
def learner_view(state):
    """👤️ The learner's runs newest first, badges in award order, best score per quiz and the total in points."""
    runs = []
    for run in sorted(state["runs"], key=lambda candidate: candidate["startedAt"], reverse=True):
        summary = {"run": run["run"], "quiz": run["quiz"], "status": run["status"], "startedAt": run["startedAt"]}
        if run["status"] == "submitted":
            summary["score"], summary["submittedAt"] = run["result"]["score"], run["submittedAt"]
        runs.append(summary)
    total = 0.0
    for score in state["best"].values():
        total += score * 100
    return {"learner": state["learner"], "identity": state["identity"], "runs": runs, "badges": state["badges"], "best": state["best"], "total": total}


def run_view(state, run, quizzes):
    """🏃️ One run with its sheet, latest answers and — once submitted — its result."""
    found = run_of(state, run)
    view = {"run": found["run"], "learner": state["learner"], "quiz": found["quiz"], "status": found["status"], "sheet": sheet_of(quizzes[found["quiz"]]["quiz"], found["seed"]), "answers": found["answers"], "startedAt": found["startedAt"]}
    if found["status"] == "submitted":
        view["result"], view["submittedAt"] = found["result"], found["submittedAt"]
    return view


# endregion 🔖️Views


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
    """🎞️ Folds the given events, then decides and folds every step; returns the decisions and the final state."""
    state = empty_learner_state(sequence["learner"])
    for event in sequence["given"]:
        state = evolve_learner(state, event)
    revisions = dict(vectors["revisions"])
    decisions = []
    for step in sequence["steps"]:
        revisions.update(step.get("revisions", {}))
        decision = decide_learner(state, step["command"], {"now": step["now"], "catalog": vectors["catalog"], "quizzes": quizzes_at(vectors, revisions)})
        decisions.append(decision)
        for event in decision.get("events", []):
            state = evolve_learner(state, event)
    return decisions, state, quizzes_at(vectors, revisions)


def replay_roster(sequence):
    """📼️ Decides and folds every identify-learner step from an empty roster."""
    state = {}
    decisions = []
    for step in sequence["steps"]:
        decision = decide_roster(state, step["command"], step["now"])
        decisions.append(decision)
        for event in decision.get("events", []):
            state = evolve_roster(state, event)
    return decisions


def held_to(scenario, produced, expected):
    """⚖️ The reference answer must be the committed one."""
    for key, value in produced.items():
        if not close(value, expected[key]):
            raise AssertionError("%s/%s: the reference answers %r, the committed vector %r" % (scenario, key, value, expected[key]))
    return Outcome(produced)


def committed(ctx):
    """🧫️ The committed vectors."""
    return json.loads(ctx.fixture_bytes(VECTORS))


def handles(ctx):
    """🏷️ ``normalizeHandle`` for every committed handle."""
    vectors = committed(ctx)["handles"]
    return held_to("handles", {vector["id"]: normalize_handle(vector["handle"]) for vector in vectors}, {vector["id"]: vector["expected"] for vector in vectors})


def roster(ctx):
    """📇️ Every committed identify-learner sequence."""
    vectors = committed(ctx)["roster"]
    return held_to("roster", {sequence["id"]: replay_roster(sequence) for sequence in vectors}, {sequence["id"]: [step["expected"] for step in sequence["steps"]] for sequence in vectors})


def learner_decisions(ctx):
    """📜️ Every committed learner sequence's decisions."""
    vectors = committed(ctx)
    return held_to("learner-decisions", {sequence["id"]: replay_learner(vectors, sequence)[0] for sequence in vectors["learners"]}, {sequence["id"]: [step["expected"] for step in sequence["steps"]] for sequence in vectors["learners"]})


def learner_views(ctx):
    """🪞️ The learner view and the current-revision run views after every committed sequence of a registered learner."""
    vectors = committed(ctx)
    viewed = [sequence for sequence in vectors["learners"] if "views" in sequence]
    produced = {}
    for sequence in viewed:
        _, state, quizzes = replay_learner(vectors, sequence)
        produced[sequence["id"]] = {"learner": learner_view(state), "runs": {run: run_view(state, run, quizzes) for run in sequence["views"]["runs"]}}
    return held_to("learner-views", produced, {sequence["id"]: sequence["views"]["expected"] for sequence in viewed})


# endregion 🔖️Replay


# region 🔖️Registration
def adapter():
    """🧭️ Oracle role only."""
    return Adapter("python").oracle("handles", handles).oracle("roster", roster).oracle("learner-decisions", learner_decisions).oracle("learner-views", learner_views)


# endregion 🔖️Registration

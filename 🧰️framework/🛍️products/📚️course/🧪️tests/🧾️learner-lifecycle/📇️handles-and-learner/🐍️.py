"""📇️ The handle decider, the learner decider with its folding evolve, and the learner and run views."""

from __future__ import annotations

# region 🔖️Imports
import copy
import re
import unicodedata

from semio_quiz_lifecycle_sheet import fnv1a32, CHALLENGES, CHALLENGE_RULES, acted, hints_of, sheet_of
from semio_quiz_lifecycle_scoring import answer_rejection, answer_complete, score_run, earned_badges
# endregion 🔖️Imports


# region 🔖️Handles
WHITE_SPACE = frozenset([*range(0x09, 0x0E), 0x20, 0x85, 0xA0, 0x1680, *range(0x2000, 0x200B), 0x2028, 0x2029, 0x202F, 0x205F, 0x3000])
LATIN_BLOCKS = [(0x0000, 0x007F), (0x0080, 0x00FF), (0x0100, 0x017F), (0x0180, 0x024F), (0x1E00, 0x1EFF)]
HANDLE_MAX = 64
HANDLE_INPUT_MAX = 256
DEFAULT_LIMITS = {"learners": 100000, "runsPerQuiz": 200, "runs": 1000, "answersPerRun": 2000}


def handle_letter(character):
    """🔤️ Whether a character is a Latin upper- or lowercase letter of the five blocks without a compatibility decomposition."""
    point = ord(character)
    return any(low <= point <= high for low, high in LATIN_BLOCKS) and unicodedata.category(character) in ("Lu", "Ll") and unicodedata.normalize("NFKC", character) == character


def normalize_handle(handle):
    """🪪️ ``{display, key}`` of a handle, or ``None`` outside the policy: White_Space runs collapse to one space and are trimmed, U+2019 becomes the apostrophe, then 1…64 letters, ASCII digits, ``'._-`` and single spaces with at least one letter or digit; the key is the lowercase."""
    if len(handle) > HANDLE_INPUT_MAX:
        return None
    words = "".join(" " if ord(character) in WHITE_SPACE else character for character in handle).split(" ")
    display = " ".join(word for word in words if word).replace("’", "'")
    worded = [character for character in display if handle_letter(character) or character in "0123456789"]
    if not worded or len(display) > HANDLE_MAX or any(character not in worded and character not in " '._-" for character in display):
        return None
    return {"display": display, "key": display.lower()}


def handle_actor_id(key):
    """🗝️ The id of the stream of a handle key: the lowercase hex of its UTF-8 bytes."""
    return key.encode("utf-8").hex()


def is_id(value):
    """🆔️ Exactly 32 lowercase hex characters."""
    return isinstance(value, str) and re.fullmatch("[0-9a-f]{32}", value) is not None


def is_slug(value):
    """🐌️ Lowercase letters and digits in hyphen-separated words, at most 64 characters."""
    return isinstance(value, str) and len(value) <= 64 and re.fullmatch("[a-z0-9]+(?:-[a-z0-9]+)*", value) is not None


MAX_TIMESTAMP = 2**53 - 1


def is_timestamp(value):
    """🕰️ An integer from 0 to 2^53 − 1: the instants every language reads exactly."""
    return isinstance(value, int) and not isinstance(value, bool) and 0 <= value <= MAX_TIMESTAMP


def command_rejection(command):
    """🛃️ ``id-invalid`` unless the command, learner and run ids are ids, the quiz and task ids slugs, the challenge of a start one of the four and the ``at`` of a start, an opening or an answer a timestamp."""
    ids = all(is_id(command[member]) for member in ("id", "learner", "run") if member in command)
    slugs = all(is_slug(command[member]) for member in ("quiz", "task") if member in command)
    challenge = command["type"] != "start-run" or command.get("challenge") in CHALLENGES
    at = command["type"] not in ("start-run", "open-task", "record-answer") or is_timestamp(command.get("at"))
    return None if ids and slugs and challenge and at else "id-invalid"


def registration_rejection(learners, limits):
    """🚧️ ``roster-full`` once the proctor holds its cap of learners."""
    return "roster-full" if learners >= limits["learners"] else None


def empty_handle_state(key):
    """🌾️ A handle key nobody claimed yet."""
    return {"key": key, "holder": None}


def decide_handle(state, command, now):
    """🛂️ identify-learner under a pseudonym or name: a free handle of this key registers the learner, a claimed one is refused."""
    malformed = command_rejection(command)
    if malformed is not None:
        return {"rejection": malformed}
    identity = command["identity"]
    handle = normalize_handle(identity["handle"]) if identity["kind"] != "anonymous" else None
    if handle is None or handle["key"] != state["key"]:
        return {"rejection": "handle-invalid"}
    if state["holder"] is not None:
        return {"rejection": "handle-claimed"}
    return {"events": [{"type": "learner-registered", "learner": command["learner"], "identity": {"kind": identity["kind"], "handle": handle["display"]}, "at": now}]}


def evolve_handle(state, event):
    """🗳️ The learner registered under a pseudonym or name holds the handle from then on."""
    if event["type"] != "learner-registered" or event["identity"]["kind"] == "anonymous":
        return state
    return {**state, "holder": event["learner"]}


# endregion 🔖️Handles


# region 🔖️Learner
def empty_learner_state(learner):
    """🌱️ A learner nothing has happened to yet."""
    return {"learner": learner, "identity": None, "runs": [], "badges": [], "best": {}, "reachedAt": None}


def run_of(state, run):
    """🔎️ One run of the learner, or ``None``."""
    return next((candidate for candidate in state["runs"] if candidate["run"] == run), None)


def evolve_learner(state, event):
    """🗂️ Folds one learner event into the state."""
    state = copy.deepcopy(state)
    kind = event["type"]
    if kind == "learner-registered":
        state["identity"] = event["identity"]
    elif kind == "run-started":
        state["runs"].append({"run": event["run"], "quiz": event["quiz"], "challenge": event["challenge"], "revision": event["revision"], "seed": event["seed"], "status": "open", "answers": {}, "recorded": 0, "opened": {}, "result": None, "startedAt": event["at"], "submittedAt": None})
    elif kind == "run-voided":
        run_of(state, event["run"])["status"] = "voided"
    elif kind == "task-opened":
        run_of(state, event["run"])["opened"][event["task"]] = event["at"]
    elif kind == "answer-recorded":
        run = run_of(state, event["run"])
        run["answers"][event["task"]] = event["answer"]
        run["recorded"] += 1
    elif kind == "run-submitted":
        run = run_of(state, event["run"])
        run["status"], run["result"], run["submittedAt"] = "submitted", event["result"], event["at"]
        quiz = event["result"]["quiz"]
        if quiz not in state["best"] or event["result"]["points"] > state["best"][quiz]["points"]:
            state["best"][quiz] = {"challenge": event["result"]["challenge"], "score": event["result"]["score"], "points": event["result"]["points"]}
            state["reachedAt"] = event["at"]
    elif kind == "badge-awarded":
        state["badges"].append({"badge": event["badge"], "run": event["run"], "at": event["at"]})
    return state


def decide_learner(state, command, context):
    """🏛️ The registration of an anonymous learner, start-run, record-answer and submit-run against the learner's state, the current quizzes and the caps."""
    now, learner, limits = context["now"], state["learner"], context["limits"]
    malformed = command_rejection(command)
    if malformed is not None:
        return {"rejection": malformed}
    if command["learner"] != learner:
        return {"rejection": "unknown-learner"}
    if command["type"] == "identify-learner":
        if command["identity"]["kind"] != "anonymous":
            return {"rejection": "handle-invalid"}
        if state["identity"] is not None:
            return {"rejection": "learner-exists"}
        return {"events": [{"type": "learner-registered", "learner": learner, "identity": {"kind": "anonymous"}, "at": now}]}
    if command["type"] == "start-run":
        if state["identity"] is None:
            return {"rejection": "unknown-learner"}
        if command["quiz"] not in context["quizzes"]:
            return {"rejection": "unknown-quiz"}
        used = run_of(state, command["run"])
        if used is not None:
            return {"rejection": "run-open" if used["status"] == "open" else "run-closed"}
        current = context["quizzes"][command["quiz"]]
        started = {"type": "run-started", "learner": learner, "run": command["run"], "quiz": command["quiz"], "challenge": command["challenge"], "revision": current["revision"], "seed": fnv1a32(command["run"]), "at": acted(command["at"], 0, now)}
        open_run = next((run for run in state["runs"] if run["quiz"] == command["quiz"] and run["status"] == "open"), None)
        if open_run is not None and open_run["revision"] == current["revision"] and open_run["challenge"] == command["challenge"]:
            return {"rejection": "run-open"}
        if len(state["runs"]) >= limits["runs"] or sum(1 for run in state["runs"] if run["quiz"] == command["quiz"]) >= limits["runsPerQuiz"]:
            return {"rejection": "runs-exhausted"}
        if open_run is None:
            return {"events": [started]}
        return {"events": [{"type": "run-voided", "learner": learner, "run": open_run["run"], "at": now}, started]}
    run = run_of(state, command["run"])
    if run is None:
        return {"rejection": "unknown-run"}
    if run["status"] != "open":
        return {"rejection": "run-closed"}
    current = context["quizzes"].get(run["quiz"])
    stale = current is None or current["revision"] != run["revision"]
    timed = CHALLENGE_RULES[run["challenge"]]["timed"]
    if command["type"] == "open-task":
        if stale:
            return {"rejection": "quiz-revised"}
        if not timed:
            return {"rejection": "run-untimed"}
        if all(task["id"] != command["task"] for task in sheet_of(current["quiz"], run["seed"], run["challenge"])["tasks"]):
            return {"rejection": "unknown-task"}
        if command["task"] in run["opened"]:
            return {"rejection": "already-opened"}
        return {"events": [{"type": "task-opened", "learner": learner, "run": run["run"], "task": command["task"], "at": acted(command["at"], run["startedAt"], now)}]}
    if command["type"] == "record-answer":
        if stale:
            return {"rejection": "quiz-revised"}
        if run["recorded"] >= limits["answersPerRun"]:
            return {"rejection": "answers-exhausted"}
        sheet_task_ = next((task for task in sheet_of(current["quiz"], run["seed"], run["challenge"])["tasks"] if task["id"] == command["task"]), None)
        if sheet_task_ is None:
            return {"rejection": "unknown-task"}
        if timed:
            if command["task"] not in run["opened"]:
                return {"rejection": "task-unopened"}
            opened = run["opened"][command["task"]]
            at = acted(command["at"], opened, now)
            if at - opened > sheet_task_["seconds"] * 1000:
                return {"rejection": "time-up"}
        else:
            at = acted(command["at"], run["startedAt"], now)
        if answer_rejection(sheet_task_, command["answer"]) is not None:
            return {"rejection": "answer-invalid"}
        return {"events": [{"type": "answer-recorded", "learner": learner, "run": run["run"], "task": command["task"], "answer": command["answer"], "at": at}]}
    if stale:
        return {"events": [{"type": "run-voided", "learner": learner, "run": run["run"], "at": now}]}
    sheet = sheet_of(current["quiz"], run["seed"], run["challenge"])
    result = score_run(current["quiz"], sheet, run["answers"])
    if result is None:
        return {"rejection": "run-incomplete"}
    results = [candidate["result"] for candidate in state["runs"] if candidate["status"] == "submitted"] + [result]
    held = [award["badge"] for award in state["badges"]]
    quizzes = [entry["quiz"] for entry in context["quizzes"].values()]
    awarded = [{"type": "badge-awarded", "learner": learner, "badge": badge, "run": run["run"], "at": now} for badge in earned_badges(context["course"]["badges"], quizzes, results, held)]
    return {"events": [{"type": "run-submitted", "learner": learner, "run": run["run"], "result": result, "at": now}] + awarded}


# endregion 🔖️Learner


# region 🔖️Views
def learner_view(state):
    """👤️ The learner's runs newest first — in reverse order of their starts, whatever instants the devices claimed — with their challenge, badges in award order, the best run per quiz — the one with the most points, the earliest of equals — and the total of their points."""
    runs = []
    for run in reversed(state["runs"]):
        summary = {"run": run["run"], "quiz": run["quiz"], "challenge": run["challenge"], "status": run["status"], "startedAt": run["startedAt"]}
        if run["status"] == "submitted":
            summary["score"], summary["points"], summary["submittedAt"] = run["result"]["score"], run["result"]["points"], run["submittedAt"]
        runs.append(summary)
    total = 0.0
    for best in state["best"].values():
        total += best["points"]
    return {"learner": state["learner"], "identity": state["identity"], "runs": runs, "badges": state["badges"], "best": state["best"], "total": total}


def run_view(state, run, quizzes):
    """🏃️ One run with its sheet at its challenge, latest answers and — once submitted — its result; when each task was opened on a timed run; the hints of an open run whose challenge hints, per task that has any."""
    found = run_of(state, run)
    quiz = quizzes[found["quiz"]]["quiz"]
    rules = CHALLENGE_RULES[found["challenge"]]
    sheet = sheet_of(quiz, found["seed"], found["challenge"])
    view = {"run": found["run"], "learner": state["learner"], "quiz": found["quiz"], "status": found["status"], "sheet": sheet, "answers": found["answers"], "startedAt": found["startedAt"]}
    if found["status"] == "submitted":
        view["result"], view["submittedAt"] = found["result"], found["submittedAt"]
    if rules["timed"]:
        view["opened"] = found["opened"]
    if rules["hints"] and found["status"] == "open":
        tasks = {task["id"]: task for task in quiz["tasks"]}
        hints = {presented["id"]: hints_of(tasks[presented["id"]], presented, found["answers"].get(presented["id"])) for presented in sheet["tasks"]}
        if any(hints.values()):
            view["hints"] = {task: found_hints for task, found_hints in hints.items() if found_hints}
    return view


# endregion 🔖️Views

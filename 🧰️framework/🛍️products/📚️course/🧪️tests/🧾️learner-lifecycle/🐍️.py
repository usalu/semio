#!/usr/bin/env python3
"""🧾️ Second implementation of the learner lifecycle (design §3–§8), in Python.

Written from the design text alone — never from the TypeScript or Rust twins. It composes everything a
proctor decides with: the id and slug shapes and the handle policy (the letters are derived from the
Unicode Character Database through ``unicodedata``, never from the cores' range tables), the handle
decider (one state per handle key; a claimed handle is refused, never recalled by a command), the
learner decider (anonymous registration, start-run, open-task, record-answer, submit-run) with the caps of
``Limits``, the sheet of §4 over numpy's MT19937, the answer rules of §5, the scoring of §6 and the
badges of §7, plus the folding ``evolve`` and the learner and run views. State is a plain dictionary
here; only decisions and schema views are projected, so no twin's private state shape is ever compared.

A run is played at one challenge (challenge design §3): chosen at ``start-run`` and fixed for the run —
starting the quiz again at the open run's challenge is ``run-open``, at another challenge it voids the
open run. The sheet, the answer rules, the scoring and the points follow the challenge: the keys show
or are guessed, an open easy run carries hints (challenge design §8: each a question about one relation the
learner's own answer claims between two items, about a profile axis or about a category), and on a timed run a task is opened once (``open-task``,
``run-untimed`` elsewhere, ``already-opened`` the second time) before it is answered (``task-unopened``)
within its seconds (``time-up``). ``start-run``, ``open-task`` and ``record-answer`` carry the instant the
learner acted, a timestamp of at most 2^53 − 1 (``id-invalid`` beyond): every claim is first lowered to
five minutes past the decider's clock; the run then starts at its claim, an opening or an answer is raised to
the run's start — the task's opening for an answer on a timed run — and the limit is applied to these instants
alone, so a device whose clock runs less than five minutes ahead is never lowered. The caps count every
run started, open, submitted or voided alike. A timed run is submitted with tasks unanswered or partly answered,
what is missing scoring as a miss, and an untimed run is submitted the same way: the points are those of the tasks already answered.

@see ../../🧫️fixtures/🧾️learner-lifecycle/🔣️.json
"""

# region 🔖️Imports
import importlib.util
import json
import os
import sys

from semio_repo_test import Adapter, Outcome


def _load(alias, folder):
    """🧩️ Loads one child module of this adapter from its folder beside this file and registers it under an importable name."""
    path = os.path.join(os.path.dirname(os.path.abspath(__file__)), folder, "🐍️.py")
    spec = importlib.util.spec_from_file_location(alias, path)
    module = importlib.util.module_from_spec(spec)
    sys.modules[alias] = module
    spec.loader.exec_module(module)
    return module


_load("semio_quiz_lifecycle_sheet", "🧗️challenge-and-sheet")
_load("semio_quiz_lifecycle_scoring", "✅️answers-and-scoring")
_load("semio_quiz_lifecycle_learner", "📇️handles-and-learner")
_load("semio_quiz_lifecycle_site", "🎞️replay-and-site")

from semio_quiz_lifecycle_sheet import challenge_meets
from semio_quiz_lifecycle_learner import DEFAULT_LIMITS, registration_rejection, learner_view, run_view
from semio_quiz_lifecycle_site import (
    VECTORS,
    close,
    replay_learner,
    replay_handle,
    site_quizzes,
    play,
    selects,
)
# endregion 🔖️Imports


# region 🔖️Handlers


def held_to(scenario, produced, expected):
    """⚖️ The reference answer must be the committed one."""
    for key, value in produced.items():
        if not close(value, expected[key]):
            raise AssertionError("%s/%s: the reference answers %r, the committed vector %r" % (scenario, key, value, expected[key]))
    return Outcome(produced)


def committed(ctx):
    """🧫️ The committed vectors."""
    return json.loads(ctx.input_bytes(VECTORS))


def claims(vectors):
    """🙋️ Every handle sequence: the well-formed ones and those whose commands carry malformed ids."""
    return vectors["registrations"] + vectors["malformed"]["registrations"]


def sequences(vectors):
    """🧑‍🎓️ Every learner sequence: the well-formed ones and those whose commands carry malformed ids."""
    return vectors["learners"] + vectors["malformed"]["learners"]


def registrations(ctx):
    """📇️ Every committed identify-learner sequence of a handle key, and every committed learner count against its cap."""
    vectors = committed(ctx)
    produced = {"handles": {sequence["id"]: replay_handle(sequence) for sequence in claims(vectors)}, "quotas": {vector["id"]: registration_rejection(vector["learners"], vector["limits"]) for vector in vectors["quotas"]}}
    expected = {"handles": {sequence["id"]: [step["expected"] for step in sequence["steps"]] for sequence in claims(vectors)}, "quotas": {vector["id"]: vector["expected"] for vector in vectors["quotas"]}}
    return held_to("registrations", produced, expected)


def site_course(ctx):
    """🥇️ Every committed play of the site course: perfect runs score 1 at every challenge and earn every badge whose least challenge they meet, a single mistake withholds exactly the badges that depend on its task."""
    site = committed(ctx)["site"]
    course, quizzes = site_quizzes(ctx.repo_root, site["course"])
    every = [badge["id"] for badge in course["badges"]]
    for named in site["named"]:
        if named not in every:
            raise AssertionError("site: the course has no badge %s" % named)
    produced = {}
    for scenario in site["plays"]:
        played = play(course, quizzes, DEFAULT_LIMITS, scenario)
        withheld = set()
        for run in scenario["runs"]:
            quiz = next(quiz for quiz in quizzes if quiz["id"] == run["quiz"])
            for task in quiz["tasks"]:
                below = lambda rule: "challenge" in rule and not challenge_meets(run["challenge"], rule["challenge"])
                withheld |= {badge["id"] for badge in course["badges"] if selects(badge["rule"], quiz, task) and (run.get("flaw") == task["id"] or below(badge["rule"]))}
        if not all(score == 1 for score, run in zip(played["scores"], scenario["runs"]) if "flaw" not in run) or any(score >= 1 for score, run in zip(played["scores"], scenario["runs"]) if "flaw" in run):
            raise AssertionError("site/%s: a perfect run must score 1 and a flawed run below it, got %r" % (scenario["id"], played["scores"]))
        if sorted(played["held"]) != sorted(set(every) - withheld):
            raise AssertionError("site/%s: holds %r, must hold every badge but %r" % (scenario["id"], played["held"], sorted(withheld)))
        if scenario["expected"] != {"awards": played["awards"], "held": played["held"]}:
            raise AssertionError("site/%s: the reference awards %r, the committed vector %r" % (scenario["id"], played["awards"], scenario["expected"]))
        produced[scenario["id"]] = played
    return Outcome(produced)


def learner_decisions(ctx):
    """📜️ Every committed learner sequence's decisions."""
    vectors = committed(ctx)
    return held_to("learner-decisions", {sequence["id"]: replay_learner(vectors, sequence)[0] for sequence in sequences(vectors)}, {sequence["id"]: [step["expected"] for step in sequence["steps"]] for sequence in sequences(vectors)})


def learner_views(ctx):
    """🪞️ The learner view and the current-revision run views after every committed sequence of a registered learner."""
    vectors = committed(ctx)
    viewed = [sequence for sequence in sequences(vectors) if "views" in sequence]
    produced = {}
    for sequence in viewed:
        _, state, quizzes = replay_learner(vectors, sequence)
        produced[sequence["id"]] = {"learner": learner_view(state), "runs": {run: run_view(state, run, quizzes) for run in sequence["views"]["runs"]}}
    return held_to("learner-views", produced, {sequence["id"]: sequence["views"]["expected"] for sequence in viewed})


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter():
    """🧭️ Oracle role only."""
    return Adapter("python").oracle("registrations", registrations).oracle("learner-decisions", learner_decisions).oracle("learner-views", learner_views).oracle("site-course", site_course)


# endregion 🔖️Registration

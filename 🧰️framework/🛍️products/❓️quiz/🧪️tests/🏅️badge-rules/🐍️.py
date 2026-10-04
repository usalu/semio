#!/usr/bin/env python3
"""🏅️ Second implementation of the badge rules (design §7), in Python.

Badges are this product's own policy, so no third party can judge them; this reading of §7 is the
reference the TypeScript and Rust twins are held to, and the two twins are held to each other.
Badges are evaluated in catalog order over every submitted result, held badges are skipped, and a
rule awards when: some result of its quiz scored exactly 1 (``perfect-quiz``); every catalog task the
selector matches scored exactly 1 in some result, a selector matching nothing never awarding
(``perfect-tasks``); every catalog quiz has a submitted result (``completed-quizzes``). A ``perfect-quiz``
or ``perfect-tasks`` rule may name a ``challenge`` (challenge design §3.6): the least challenge that
counts — only results played at that challenge or a more demanding one (easy < medium < hard < expert)
are looked at, every result without it.

@see ../../🧫️fixtures/🏅️badge-rules/🔣️.json
"""

# region 🔖️Imports
import json

from semio_repo_test import Adapter, Outcome

# endregion 🔖️Imports


# region 🔖️Reference
VECTORS = "shared://🏅️badge-rules/🔣️.json"
CHALLENGES = ["easy", "medium", "hard", "expert"]


def counting(rule, results):
    """🚧️ The results a rule counts: every one, or those played at a challenge at least as demanding as the rule's."""
    return [result for result in results if "challenge" not in rule or CHALLENGES.index(result["challenge"]) >= CHALLENGES.index(rule["challenge"])]


def perfect_task(results, quiz, task):
    """💯️ Whether some result of ``quiz`` scored ``task`` exactly 1."""
    return any(result["quiz"] == quiz and any(scored["task"] == task and scored["score"] == 1 for scored in result["tasks"]) for result in results)


def earns(rule, quizzes, results):
    """⚖️ Whether one rule holds over the learner's submitted results."""
    if rule["kind"] == "perfect-quiz":
        return any(result["quiz"] == rule["quiz"] and result["score"] == 1 for result in counting(rule, results))
    if rule["kind"] == "perfect-tasks":
        selected = [(quiz["id"], task["id"]) for quiz in quizzes for task in quiz["tasks"] if rule.get("taskKind", task["kind"]) == task["kind"] and rule.get("quiz", quiz["id"]) == quiz["id"]]
        return len(selected) > 0 and all(perfect_task(counting(rule, results), quiz, task) for quiz, task in selected)
    return all(any(result["quiz"] == quiz["id"] for result in results) for quiz in quizzes)


def earned_badges(badges, quizzes, results, held):
    """🎖️ The ids of the badges newly earned, in catalog order."""
    return [badge["id"] for badge in badges if badge["id"] not in held and earns(badge["rule"], quizzes, results)]


# endregion 🔖️Reference


# region 🔖️Handlers
def awards(ctx):
    """🗃️ Every committed (results, held) pair, judged and held to its committed awards."""
    vectors = json.loads(ctx.fixture_bytes(VECTORS))
    produced = {}
    for vector in vectors["vectors"]:
        produced[vector["id"]] = earned_badges(vectors["badges"], vectors["quizzes"], vector["results"], vector["held"])
        if produced[vector["id"]] != vector["expected"]:
            raise AssertionError("awards/%s: the reference awards %r, the committed vector %r" % (vector["id"], produced[vector["id"]], vector["expected"]))
    return Outcome(produced)


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter():
    """🧭️ Oracle role only."""
    return Adapter("python").oracle("awards", awards)


# endregion 🔖️Registration

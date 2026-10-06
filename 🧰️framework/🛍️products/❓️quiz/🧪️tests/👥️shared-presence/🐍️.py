#!/usr/bin/env python3
"""👥️ Oracle of the quiz's shared presence, cursors and thinking (design §15–§17): python-jsonschema plus the place rules, in Python.

A presence state (catalog-wide roster room) and a cursor state (the room of one place) are shared with
every other learner, so both cores refuse a state before the proctor relays it. The structure is judged
by python-jsonschema's Draft 7 validator against the normative ``PresenceState``/``CursorState``
definitions: 8-hex tags instead of learner ids, anchors — cards, ``item:<id>`` and ``category:<id>`` since
§17 —, cursor coordinates in [0, 1], a dragged item that is a slug, no undeclared member that could smuggle
a learner id. The place rules the schema cannot state are written here
from the design text: the page of a quiz, a run and its results name their quiz (``quiz-required``), no
other screen names a quiz (``quiz-not-allowed``), and only a run names the task on screen
(``task-without-run``). Room scopes follow §15 and §16: the roster is the catalog scope; ``introduction``,
``home``, ``leaderboard`` and ``badges`` each have a room ``<catalog>/<screen>``; the page, the run and the
results of one quiz share the room ``<catalog>/quiz/<quiz>``. The identity screen has none (a learner there
has no tag yet), the personal pages ``learner`` and ``preferences`` have none, and neither has a quiz page,
run or results place without its quiz. A learner's draft answers of an open run are shared in the thinking
room ``<catalog>/quiz/<quiz>/thinking`` as a ``ThinkingState``: tag and ``ThinkingAnswer`` per task id —
classification and sorting drafts as answers, matching drafts as the values assigned per dimension and
item, never card indices. It is refused when it breaks the schema (a non-finite value arrives as a text or
null, never as a number), when a sorting draft repeats an item (``duplicate-id``), or beyond the bounds
(``too-many``): at most 64 tasks, and at most 64 entries per order, assignment map, dimension map and
dimension — the ``THINKING_LIMIT`` both cores share, which the design text does not state.

@see ../../🧬️schema/🔣️.json
@see ../../🧫️fixtures/👥️shared-presence/🔣️.json
"""

# region 🔖️Imports
import json
import os

import jsonschema

from semio_repo_test import Adapter, Outcome

# endregion 🔖️Imports


# region 🔖️Reference
SCHEMA = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", "🧬️schema", "🔣️.json")
VECTORS = "shared://👥️shared-presence/🔣️.json"
ROOMS = ("introduction", "home", "leaderboard", "badges")
QUIZ_SCREENS = ("quiz", "run", "results")
THINKING_LIMIT = 64


def violations(definition, document):
    """💥️ The schema's complaints about one document, as the names of the failing keywords."""
    with open(SCHEMA, "r", encoding="utf-8") as handle:
        schema = json.load(handle)
    return sorted({error.validator for error in jsonschema.Draft7Validator({**schema, "$ref": "#/$defs/%s" % definition}).iter_errors(document)})


def place_problem(place):
    """📍️ The place rule a structurally valid place breaks, or ``None``."""
    if place["screen"] in QUIZ_SCREENS and "quiz" not in place:
        return "quiz-required"
    if place["screen"] not in QUIZ_SCREENS and "quiz" in place:
        return "quiz-not-allowed"
    if place["screen"] != "run" and "task" in place:
        return "task-without-run"
    return None


def presence_problem(state):
    """🟢️ Why a presence state may not be shared — a failing schema keyword or a place rule — or ``None``."""
    broken = violations("PresenceState", state)
    return broken[0] if broken else place_problem(state["place"])


def cursor_problem(state):
    """🖱️ Why a cursor state may not be shared — a failing schema keyword — or ``None``."""
    broken = violations("CursorState", state)
    return broken[0] if broken else None


def roster_scope(catalog):
    """📋️ The catalog-wide roster room."""
    return catalog


def room_scope(catalog, place):
    """🚪️ The room of a place, or ``None`` for a place learners do not share."""
    if place["screen"] in QUIZ_SCREENS:
        return "%s/quiz/%s" % (catalog, place["quiz"]) if "quiz" in place else None
    return "%s/%s" % (catalog, place["screen"]) if place["screen"] in ROOMS else None


def thinking_scope(catalog, quiz):
    """💭️ The thinking room of a quiz."""
    return "%s/quiz/%s/thinking" % (catalog, quiz)


def thinking_problem(state):
    """🧠️ Why a thinking state may not be shared — a failing schema keyword, ``duplicate-id`` or ``too-many`` — or ``None``."""
    broken = violations("ThinkingState", state)
    if broken:
        return broken[0]
    if len(state["answers"]) > THINKING_LIMIT:
        return "too-many"
    for answer in state["answers"].values():
        if answer["kind"] == "sorting":
            if len(answer["order"]) > THINKING_LIMIT or len(answer.get("guesses", {})) > THINKING_LIMIT:
                return "too-many"
            if len(set(answer["order"])) != len(answer["order"]):
                return "duplicate-id"
        elif answer["kind"] == "classification":
            if len(answer["assignments"]) > THINKING_LIMIT:
                return "too-many"
        elif len(answer["values"]) > THINKING_LIMIT or any(len(items) > THINKING_LIMIT for items in answer["values"].values()):
            return "too-many"
    return None


# endregion 🔖️Reference


# region 🔖️Handlers
def committed(ctx):
    """🧫️ The committed vectors."""
    return json.loads(ctx.input_bytes(VECTORS))


def held_to(scenario, produced, expected):
    """⚖️ The reference answer must be the committed one."""
    for key, value in produced.items():
        if value != expected[key]:
            raise AssertionError("%s/%s: the reference answers %r, the committed vector %r" % (scenario, key, value, expected[key]))
    return Outcome(produced)


def judged(scenario, vectors, problem, definition):
    """🔎️ Accepts or refuses every state and checks that a refused one breaks the rule its vector names."""
    produced = {}
    for vector in vectors:
        found = problem(vector["state"])
        if vector.get("rule") is not None and vector["rule"] not in violations(definition, vector["state"]) + [found]:
            raise AssertionError("%s/%s: expected the %s rule to refuse it, found %r" % (scenario, vector["id"], vector["rule"], found))
        produced[vector["id"]] = found is None
    return held_to(scenario, produced, {vector["id"]: vector["expected"] for vector in vectors})


def room_scopes(ctx):
    """🗺️ The roster scope and the room scope of every committed catalog and place."""
    vectors = committed(ctx)["scopes"]
    return held_to("room-scopes", {vector["id"]: {"roster": roster_scope(vector["catalog"]), "room": room_scope(vector["catalog"], vector["place"])} for vector in vectors}, {vector["id"]: vector["expected"] for vector in vectors})


def presence_states(ctx):
    """👀️ Every committed presence state, accepted or refused."""
    return judged("presence-states", committed(ctx)["presence"], presence_problem, "PresenceState")


def cursor_states(ctx):
    """🎯️ Every committed cursor state, accepted or refused."""
    return judged("cursor-states", committed(ctx)["cursors"], cursor_problem, "CursorState")


def thinking_scopes(ctx):
    """🗯️ The thinking room of every committed catalog and quiz."""
    vectors = committed(ctx)["thinkingScopes"]
    return held_to("thinking-scopes", {vector["id"]: thinking_scope(vector["catalog"], vector["quiz"]) for vector in vectors}, {vector["id"]: vector["expected"] for vector in vectors})


def thinking_states(ctx):
    """🤔️ Every committed thinking state, accepted or refused."""
    return judged("thinking-states", committed(ctx)["thinking"], thinking_problem, "ThinkingState")


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter():
    """🧭️ Oracle role only: jsonschema and the place rules are the reference, the owned validators are the subjects."""
    return Adapter("python").oracle("room-scopes", room_scopes).oracle("presence-states", presence_states).oracle("cursor-states", cursor_states).oracle("thinking-scopes", thinking_scopes).oracle("thinking-states", thinking_states)


# endregion 🔖️Registration

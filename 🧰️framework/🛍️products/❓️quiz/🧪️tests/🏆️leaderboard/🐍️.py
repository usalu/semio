#!/usr/bin/env python3
"""🏆️ Second implementation of the read views (schema ``CatalogView``/``LearnerView``/``Leaderboard``), in Python.

The views are this product's own policy, so no third party can judge them; this reading of the
contract is the reference the TypeScript and Rust twins are held to. The catalog view is the catalog
without quiz paths and badge rules, its quizzes without solutions (task id, kind and title only), in
catalog order. Every learner stream is folded from its committed events; the leaderboard then lists
every learner with a submitted run, by total descending, badge count descending, ``reachedAt``
ascending and learner id ascending, ranked from 1 — and publishes the learner only as its ``tag``, the
FNV-1a 32-bit hash of the learner id as 8 lowercase hex digits, never the id itself. The total is the
sum of the best score per quiz in points (score × 100); ``reachedAt`` is the submission that last
raised a best score (a first submission of a quiz raises it); ``lastActivity`` is the latest ``at`` of
the learner's stream.

@see ../../🧬️schema/🔣️.json
@see ../../🧫️fixtures/🏆️leaderboard/🔣️.json
"""

# region 🔖️Imports
import copy
import json

from semio_repo_test import Adapter, Outcome

# endregion 🔖️Imports


# region 🔖️Reference
VECTORS = "shared://🏆️leaderboard/🔣️.json"
TOLERANCE = 1e-12


def fnv1a32(text):
    """#️⃣ FNV-1a 32-bit over the UTF-8 bytes of ``text``."""
    value = 2166136261
    for byte in text.encode("utf-8"):
        value = ((value ^ byte) * 16777619) & 0xFFFFFFFF
    return value


def tag(learner):
    """🏷️ The public, non-reversible learner tag: ``fnv1a32`` of the id as 8 lowercase hex digits."""
    return "%08x" % fnv1a32(learner)


def catalog_view(catalog, quizzes):
    """📚️ The solution-free catalog: no quiz paths, no badge rules, quizzes reduced to their task ids, kinds and titles."""
    return {
        "id": catalog["id"],
        "title": catalog["title"],
        "introduction": catalog["introduction"],
        "quizzes": [{"id": quiz["id"], "title": quiz["title"], "description": quiz["description"], "tasks": [{"id": task["id"], "kind": task["kind"], "title": task["title"]} for task in quiz["tasks"]]} for quiz in quizzes],
        "badges": [{"id": badge["id"], "emoji": badge["emoji"], "label": badge["label"], "description": badge["description"]} for badge in catalog["badges"]],
    }


def empty_learner_state(learner):
    """🌱️ A learner nothing has happened to yet."""
    return {"learner": learner, "identity": None, "runs": [], "badges": [], "best": {}, "reachedAt": None, "lastActivity": None}


def run_of(state, run):
    """🔎️ One run of the learner, or ``None``."""
    return next((candidate for candidate in state["runs"] if candidate["run"] == run), None)


def evolve_learner(state, event):
    """🧵️ Folds one learner event into the state."""
    state = copy.deepcopy(state)
    state["lastActivity"] = event["at"] if state["lastActivity"] is None else max(state["lastActivity"], event["at"])
    kind = event["type"]
    if kind == "learner-registered":
        state["identity"] = event["identity"]
    elif kind == "run-started":
        state["runs"].append({"run": event["run"], "quiz": event["quiz"], "status": "open", "result": None, "startedAt": event["at"], "submittedAt": None})
    elif kind == "run-voided":
        run_of(state, event["run"])["status"] = "voided"
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


def total_of(state):
    """💯️ The sum of the best scores in points."""
    total = 0.0
    for score in state["best"].values():
        total += score * 100
    return total


def learner_view(state):
    """👤️ The learner's runs newest first, badges in award order, best score per quiz and the total in points."""
    runs = []
    for run in sorted(state["runs"], key=lambda candidate: candidate["startedAt"], reverse=True):
        summary = {"run": run["run"], "quiz": run["quiz"], "status": run["status"], "startedAt": run["startedAt"]}
        if run["status"] == "submitted":
            summary["score"], summary["submittedAt"] = run["result"]["score"], run["submittedAt"]
        runs.append(summary)
    return {"learner": state["learner"], "identity": state["identity"], "runs": runs, "badges": state["badges"], "best": state["best"], "total": total_of(state)}


def leaderboard(states):
    """🥇️ Every learner with a submitted run, ordered and ranked by the leaderboard's tie-break chain."""
    submitted = [state for state in states if any(run["status"] == "submitted" for run in state["runs"])]
    ordered = sorted(submitted, key=lambda state: (-total_of(state), -len(state["badges"]), state["reachedAt"], state["learner"]))
    return {
        "rows": [
            {
                "rank": rank,
                "tag": tag(state["learner"]),
                "identity": state["identity"],
                "total": total_of(state),
                "reachedAt": state["reachedAt"],
                "best": state["best"],
                "badges": [award["badge"] for award in state["badges"]],
                "runs": sum(1 for run in state["runs"] if run["status"] == "submitted"),
                "lastActivity": state["lastActivity"],
            }
            for rank, state in enumerate(ordered, start=1)
        ]
    }


# endregion 🔖️Reference


# region 🔖️Handlers
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


def folded(vector):
    """🗂️ Every learner of a vector folded from its committed events, in committed order."""
    states = []
    for learner in vector["learners"]:
        state = empty_learner_state(learner["learner"])
        for event in learner["events"]:
            state = evolve_learner(state, event)
        states.append(state)
    return states


def held_to(scenario, produced, expected):
    """⚖️ The reference answer must be the committed one."""
    for key, value in produced.items():
        if not close(value, expected[key]):
            raise AssertionError("%s/%s: the reference answers %r, the committed vector %r" % (scenario, key, value, expected[key]))
    return Outcome(produced)


def learner_views(ctx):
    """🪞️ The learner view of every committed learner stream."""
    vectors = json.loads(ctx.fixture_bytes(VECTORS))["vectors"]
    produced = {vector["id"]: {state["learner"]: learner_view(state) for state in folded(vector)} for vector in vectors}
    return held_to("learner-views", produced, {vector["id"]: vector["expected"]["learnerViews"] for vector in vectors})


def rankings(ctx):
    """🗃️ The leaderboard of every committed set of learner streams."""
    vectors = json.loads(ctx.fixture_bytes(VECTORS))["vectors"]
    produced = {vector["id"]: leaderboard(folded(vector)) for vector in vectors}
    return held_to("rankings", produced, {vector["id"]: vector["expected"]["leaderboard"] for vector in vectors})


def catalog_views(ctx):
    """🗺️ The catalog view of every committed catalog with its quizzes."""
    vectors = json.loads(ctx.fixture_bytes(VECTORS))["catalogs"]
    produced = {vector["id"]: catalog_view(vector["catalog"], vector["quizzes"]) for vector in vectors}
    return held_to("catalog-views", produced, {vector["id"]: vector["expected"] for vector in vectors})


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter():
    """🧭️ Oracle role only."""
    return Adapter("python").oracle("catalog-views", catalog_views).oracle("learner-views", learner_views).oracle("rankings", rankings)


# endregion 🔖️Registration

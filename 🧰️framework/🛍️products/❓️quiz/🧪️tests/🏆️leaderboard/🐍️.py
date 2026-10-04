#!/usr/bin/env python3
"""🏆️ Second implementation of the read views (schema ``CatalogView``/``LearnerView``/``Leaderboard``), in Python.

The views are this product's own policy, so no third party can judge them; this reading of the
contract is the reference the TypeScript and Rust twins are held to. The catalog view is the catalog
without quiz paths and badge rules, its quizzes without solutions (emoji, title, description, and task
id, kind and title only), in catalog order. Every learner stream is folded from its committed events
into a transcript — the submitted runs and the badges beside the learner id. A leaderboard has a
period and may name one quiz: it counts the runs submitted inside the period's window around the
asked instant — the UTC day, the ISO week from Monday, the month, or every run for all-time; the
windows are read off Python's ``datetime``, the third party the twins' calendar arithmetic is held to
— and of that quiz only. A standing is the learner's unranked row over the runs that count; the
leaderboard orders the standings by total descending, badge count descending, ``reachedAt`` ascending
and learner id ascending, ranks them from 1, answers the top 100 rows, the number of ranked learners,
the number of runs submitted in all and — when the caller is ranked — the caller's own row, and
publishes a learner only as its ``tag``, the FNV-1a 32-bit hash of the learner id as 8 lowercase hex
digits, never the id itself. A run earns points — its score times the par of its challenge, carried by
its result (challenge design §3.6) — and the best run of a quiz is the one with the most points, a later
run replacing it only with strictly more; ``best`` names it per quiz with its challenge, score and
points, and the total is the sum of those points. ``reachedAt`` is the submission that last raised a
best (a first submission of a quiz raises it); ``lastActivity`` is the last submission that counts; the
badges are those the counted runs earned.

@see ../../🧬️schema/🔣️.json
@see ../../🧫️fixtures/🏆️leaderboard/🔣️.json
"""

# region 🔖️Imports
import copy
import json
from datetime import datetime, timedelta, timezone

from semio_repo_test import Adapter, Outcome

# endregion 🔖️Imports


# region 🔖️Reference
VECTORS = "shared://🏆️leaderboard/🔣️.json"
TOLERANCE = 1e-12
PAR = {"easy": 100, "medium": 200, "hard": 300, "expert": 400}


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
    """📚️ The solution-free catalog: no quiz paths, no badge rules, quizzes reduced to their emoji, title, description and the id, kind, title and icon of their tasks."""
    return {
        "id": catalog["id"],
        "title": catalog["title"],
        "introduction": catalog["introduction"],
        "quizzes": [{"id": quiz["id"], "emoji": quiz["emoji"], "title": quiz["title"], "description": quiz["description"], "tasks": [{"id": task["id"], "kind": task["kind"], "title": task["title"], **({"icon": task["icon"]} if "icon" in task else {})} for task in quiz["tasks"]]} for quiz in quizzes],
        "badges": [{"id": badge["id"], "emoji": badge["emoji"], "label": badge["label"], "description": badge["description"]} for badge in catalog["badges"]],
    }


def empty_learner_state(learner):
    """🌱️ A learner nothing has happened to yet."""
    return {"learner": learner, "identity": None, "runs": [], "badges": [], "best": {}, "reachedAt": None}


def run_of(state, run):
    """🔎️ One run of the learner, or ``None``."""
    return next((candidate for candidate in state["runs"] if candidate["run"] == run), None)


def evolve_learner(state, event):
    """🧵️ Folds one learner event into the state."""
    state = copy.deepcopy(state)
    kind = event["type"]
    if kind == "learner-registered":
        state["identity"] = event["identity"]
    elif kind == "run-started":
        state["runs"].append({"run": event["run"], "quiz": event["quiz"], "challenge": event["challenge"], "status": "open", "result": None, "startedAt": event["at"], "submittedAt": None})
    elif kind == "run-voided":
        run_of(state, event["run"])["status"] = "voided"
    elif kind == "run-submitted":
        run = run_of(state, event["run"])
        run["status"], run["result"], run["submittedAt"] = "submitted", event["result"], event["at"]
        quiz = event["result"]["quiz"]
        if quiz not in state["best"] or event["result"]["points"] > state["best"][quiz]["points"]:
            state["best"][quiz] = best_of(event["result"])
            state["reachedAt"] = event["at"]
    elif kind == "badge-awarded":
        state["badges"].append({"badge": event["badge"], "run": event["run"], "at": event["at"]})
    return state


def best_of(scored):
    """🏵️ A scored run as the best of its quiz: its challenge, its score and its points — which must be the score times the par of the challenge."""
    if abs(scored["points"] - scored["score"] * PAR[scored["challenge"]]) > TOLERANCE:
        raise AssertionError("a %s run scored %r carries %r points" % (scored["challenge"], scored["score"], scored["points"]))
    return {"challenge": scored["challenge"], "score": scored["score"], "points": scored["points"]}


def total_of(state):
    """💯️ The sum of the points of the best runs."""
    total = 0.0
    for best in state["best"].values():
        total += best["points"]
    return total


def learner_view(state):
    """👤️ The learner's runs newest first with their challenge, badges in award order, the best run per quiz — the one with the most points, the earliest of equals — and the total of their points."""
    runs = []
    for run in sorted(state["runs"], key=lambda candidate: candidate["startedAt"], reverse=True):
        summary = {"run": run["run"], "quiz": run["quiz"], "challenge": run["challenge"], "status": run["status"], "startedAt": run["startedAt"]}
        if run["status"] == "submitted":
            summary["score"], summary["points"], summary["submittedAt"] = run["result"]["score"], run["result"]["points"], run["submittedAt"]
        runs.append(summary)
    return {"learner": state["learner"], "identity": state["identity"], "runs": runs, "badges": state["badges"], "best": state["best"], "total": total_of(state)}


TOP = 100
PERIODS = ["daily", "weekly", "monthly", "all-time"]
EPOCH = datetime(1970, 1, 1, tzinfo=timezone.utc)


def transcript(state):
    """📜️ What every standing of a learner is made of — the submitted runs in submission order and the badges with the quiz of the run that earned them — or ``None`` before registration or the first submission."""
    runs = [{"quiz": run["quiz"], "challenge": run["result"]["challenge"], "score": run["result"]["score"], "points": run["result"]["points"], "at": run["submittedAt"]} for run in sorted((run for run in state["runs"] if run["status"] == "submitted"), key=lambda run: run["submittedAt"])]
    if state["identity"] is None or not runs:
        return None
    badges = [{"badge": award["badge"], "quiz": run_of(state, award["run"])["quiz"], "at": award["at"]} for award in state["badges"] if run_of(state, award["run"]) is not None]
    return {"learner": state["learner"], "tag": tag(state["learner"]), "identity": state["identity"], "runs": runs, "badges": badges}


def milliseconds(moment):
    """⏱️ A calendar moment as milliseconds since the Unix epoch, none before it."""
    return max(0, (moment - EPOCH) // timedelta(milliseconds=1))


def period_window(period, at):
    """🪟️ The window of a period around an instant, read off Python's ``datetime`` calendar: the UTC day, the ISO week from its Monday, the month; ``None`` for all-time."""
    if period == "all-time":
        return None
    day = (EPOCH + timedelta(milliseconds=at)).replace(hour=0, minute=0, second=0, microsecond=0)
    if period == "daily":
        start, end = day, day + timedelta(days=1)
    elif period == "weekly":
        start = day - timedelta(days=day.weekday())
        end = start + timedelta(days=7)
    else:
        start = day.replace(day=1)
        end = start.replace(year=start.year + 1, month=1) if start.month == 12 else start.replace(month=start.month + 1)
    return {"from": milliseconds(start), "until": milliseconds(end)}


def standing(record, window, quiz):
    """🧍️ The unranked row of a transcript over its runs inside the window and of the quiz, beside the learner id; ``None`` when no run counts."""
    counts = lambda entry: (quiz is None or entry["quiz"] == quiz) and (window is None or window["from"] <= entry["at"] < window["until"])
    runs = [run for run in record["runs"] if counts(run)]
    if not runs:
        return None
    best, reached = {}, None
    for run in runs:
        if run["quiz"] not in best or run["points"] > best[run["quiz"]]["points"]:
            best[run["quiz"]], reached = best_of(run), run["at"]
    total = 0.0
    for entry in best.values():
        total += entry["points"]
    return {
        "learner": record["learner"],
        "tag": record["tag"],
        "identity": record["identity"],
        "total": total,
        "reachedAt": reached,
        "best": best,
        "badges": [award["badge"] for award in record["badges"] if counts(award)],
        "runs": len(runs),
        "lastActivity": max(run["at"] for run in runs),
    }


def leaderboard(transcripts, period, quiz, at, caller):
    """🥇️ One leaderboard at an instant: the top rows of the standings in scope in the tie-break order, how many learners are ranked, how many runs were submitted in all, and the caller's own row when the caller is ranked."""
    window = period_window(period, at)
    standings = [entry for entry in (standing(record, window, quiz) for record in transcripts) if entry is not None]
    ordered = sorted(standings, key=lambda entry: (-entry["total"], -len(entry["badges"]), entry["reachedAt"], entry["learner"]))
    ranked = [(entry["learner"], {"rank": rank, **{member: value for member, value in entry.items() if member != "learner"}}) for rank, entry in enumerate(ordered, start=1)]
    board = {"period": period}
    if quiz is not None:
        board["quiz"] = quiz
    if window is not None:
        board["window"] = window
    board.update({"rows": [row for _, row in ranked[:TOP]], "learners": len(ranked), "submissions": sum(len(record["runs"]) for record in transcripts)})
    own = next((row for learner, row in ranked if learner == caller), None) if caller is not None else None
    if own is not None:
        board["own"] = own
    return board


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


def transcripts(vector):
    """📚️ The transcript of every ranked learner of a vector, in committed order."""
    return [record for record in map(transcript, folded(vector)) if record is not None]


def asked(records, board, callers):
    """🙋️ One leaderboard as every committed caller is answered, keyed by the caller's label."""
    return {caller["id"]: leaderboard(records, board["period"], board.get("quiz"), board["at"], caller.get("learner")) for caller in callers}


def boards(records, vector):
    """🗂️ Every committed board of a vector as every committed caller is answered, keyed by the board's label."""
    return {board["id"]: asked(records, board, vector["callers"]) for board in vector["boards"]}


def rankings(ctx):
    """🗃️ Every committed leaderboard of every committed set of learner streams, as every committed caller is answered."""
    vectors = json.loads(ctx.fixture_bytes(VECTORS))["vectors"]
    produced = {vector["id"]: boards(transcripts(vector), vector) for vector in vectors}
    return held_to("rankings", produced, {vector["id"]: vector["expected"]["leaderboards"] for vector in vectors})


def windows(ctx):
    """🪟️ The window of every period around every committed instant; ``None`` for all-time."""
    vectors = json.loads(ctx.fixture_bytes(VECTORS))["windows"]
    produced = {vector["id"]: {period: period_window(period, vector["at"]) for period in PERIODS} for vector in vectors}
    for identifier, answered in produced.items():
        at = next(vector["at"] for vector in vectors if vector["id"] == identifier)
        if answered["all-time"] is not None or any(not answered[period]["from"] <= at < answered[period]["until"] for period in PERIODS[:3]):
            raise AssertionError("windows/%s: a window does not contain its instant" % identifier)
    return held_to("windows", produced, {vector["id"]: vector["expected"] for vector in vectors})


def outline(board):
    """🪧️ A leaderboard reduced to what a cut decides: ``rank:tag`` of every row in order, the number of ranked learners and ``rank:tag`` of the caller's own row."""
    place = lambda row: "%d:%s" % (row["rank"], row["tag"])
    return {"rows": [place(row) for row in board["rows"]], "learners": board["learners"], "own": place(board["own"]) if "own" in board else None}


def cuts(ctx):
    """🪜️ The outline of the committed board over the first transcripts of the committed crowd, per committed cut and caller: the top 100, the count of ranked learners and the caller's own row."""
    crowd = json.loads(ctx.fixture_bytes(VECTORS))["crowd"]
    produced = {}
    for vector in crowd["cuts"]:
        records = crowd["transcripts"][: vector["learners"]]
        produced[vector["id"]] = {caller: outline(board) for caller, board in asked(records, crowd["board"], vector["callers"]).items()}
        for board in produced[vector["id"]].values():
            if len(board["rows"]) != min(TOP, len(records)) or board["learners"] != len(records) or [row.split(":")[0] for row in board["rows"]] != [str(rank) for rank in range(1, len(board["rows"]) + 1)]:
                raise AssertionError("cuts/%s: the top is not the first %d ranks of %d learners" % (vector["id"], TOP, len(records)))
    return held_to("cuts", produced, {vector["id"]: vector["expected"] for vector in crowd["cuts"]})


def catalog_views(ctx):
    """🗺️ The catalog view of every committed catalog with its quizzes."""
    vectors = json.loads(ctx.fixture_bytes(VECTORS))["catalogs"]
    produced = {vector["id"]: catalog_view(vector["catalog"], vector["quizzes"]) for vector in vectors}
    return held_to("catalog-views", produced, {vector["id"]: vector["expected"] for vector in vectors})


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter():
    """🧭️ Oracle role only."""
    return Adapter("python").oracle("catalog-views", catalog_views).oracle("learner-views", learner_views).oracle("windows", windows).oracle("rankings", rankings).oracle("cuts", cuts)


# endregion 🔖️Registration

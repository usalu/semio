#!/usr/bin/env python3
"""🧠️ Oracle of the pets behaviour choice (design §4.6, §5.2, §5.4–§5.6), in Python.

A pet chooses by weights. The choice itself is numpy's: the index ``numpy.searchsorted`` finds for ``unit × total``
in ``numpy.cumsum`` of the positive weights (side ``right``: the first running sum that exceeds the mark), with the
unit of a keyed draw taken from ``numpy.random.SeedSequence(key).generate_state(1)``. Which activity may follow
which is a directed graph; ``scipy.sparse.csgraph`` decides what is reachable from where and that the graph is one
strongly connected component, so no activity is a dead end. The rotation of a cast is ``numpy.roll``.

The tables the stage is tuned with (the limits of the modes, the weights, the dwells, the moods, the shares of an
encounter, the graph) are restated here from the design text with numpy arithmetic — a second reading that holds the
two cores to one set of numbers; the third-party evidence is the choice, the reachability and the keyed draw.

@see https://numpy.org/doc/stable/reference/generated/numpy.searchsorted.html
@see https://docs.scipy.org/doc/scipy/reference/sparse.csgraph.html
@see ../../🧫️fixtures/🧠️behavior-choice/🔣️.json
"""

# region 🔖️Imports
import json

import numpy
from scipy.sparse import csr_matrix
from scipy.sparse.csgraph import breadth_first_order, connected_components

from semio_repo_test import Adapter, Outcome

# endregion 🔖️Imports


# region 🔖️Reference
VECTORS = "shared://🧠️behavior-choice/🔣️.json"
ACTIVITIES = ["idle", "fidget", "walk", "hop", "fall", "land", "sleep", "greet", "cuddle", "squabble", "sulk"]
ENCOUNTERS = ["greet", "cuddle", "squabble"]
LIMITS = {
    "still": {"movers": 0, "fidgeters": 0, "idleLow": 0, "idleHigh": 0, "fidget": 0, "walk": 0, "hop": 0, "sleep": 0, "stroll": 0, "encounterGap": 0, "encounterRate": 0},
    "calm": {"movers": 1, "fidgeters": 1, "idleLow": 384, "idleHigh": 1280, "fidget": 0.4, "walk": 0.2, "hop": 0.12, "sleep": 6, "stroll": 4, "encounterGap": 5760, "encounterRate": 0.04},
    "lively": {"movers": 2, "fidgeters": 2, "idleLow": 192, "idleHigh": 640, "fidget": 1, "walk": 0.6, "hop": 0.3, "sleep": 3, "stroll": 6, "encounterGap": 1920, "encounterRate": 0.12},
}
DWELL_LOW = [0, 96, 1920, 96, 640, 19, 1280, 128, 128, 128, 192]
DWELL_HIGH = [0, 96, 1920, 96, 640, 19, 3840, 320, 320, 320, 384]
MOODS = [0.3, 0.5, 0.4, 0.5, -0.2, 0.2, 0.1, 0.7, 1, -0.8, -0.6]
FOLLOWERS = {
    "idle": ["idle", "fidget", "walk", "hop", "fall", "sleep", "greet", "cuddle", "squabble"],
    "fidget": ["idle", "walk", "fall", "greet"],
    "walk": ["idle", "fall", "greet", "cuddle", "squabble"],
    "hop": ["idle", "fall", "land"],
    "fall": ["idle", "land"],
    "land": ["idle", "walk", "fall", "greet"],
    "sleep": ["idle", "walk", "fall", "greet"],
    "greet": ["idle", "walk", "fall"],
    "cuddle": ["idle", "walk", "fall"],
    "squabble": ["idle", "walk", "fall", "sulk"],
    "sulk": ["idle", "walk", "fall", "greet"],
}
CAST_STREAM = 0xFFFFFFFE
QUIET_DROWSE = 3
DROWSY = 0.6


def unit(key):
    """🎯️ The first word numpy's seed sequence generates for the key, divided by 2³²."""
    return int(numpy.random.SeedSequence([int(part) for part in key]).generate_state(1)[0]) / 4294967296.0


def pick(weights, mark):
    """🎰️ The index whose running sum of positive weights first exceeds ``mark × total``; −1 without a positive weight."""
    amounts = numpy.array(weights, dtype=numpy.float64)
    if len(amounts) == 0:
        return -1
    running = numpy.cumsum(numpy.where(amounts > 0, amounts, 0.0))
    if not float(running[-1]) > 0:
        return -1
    index = int(numpy.searchsorted(running, numpy.float64(mark) * running[-1], side="right"))
    return index if index < len(weights) else int(numpy.flatnonzero(amounts > 0)[-1])


def weights(vector):
    """🏋️ How much an idle actor feels like each activity, in ``ACTIVITIES`` order (design §5.6 with the tuned limits)."""
    limits = LIMITS[vector["mode"]]
    energy = numpy.float64(vector["needs"]["energy"])
    curiosity = numpy.float64(vector["needs"]["curiosity"])
    drive = 0.5 + 0.5 * energy
    urge = 0.25 + curiosity
    tired = numpy.clip((DROWSY - energy) / DROWSY, 0, 1)
    awake = not vector["quiet"]
    free = vector["movers"] < limits["movers"]
    fidget = limits["fidget"] * drive * (0.5 + curiosity) if awake and vector["fidgeters"] < limits["fidgeters"] and vector["fidgets"] else 0.0
    walk = limits["walk"] * drive * urge if awake and free and vector["roam"] else 0.0
    hop = limits["hop"] * energy * urge * (1 + vector["crowd"]) if awake and free and vector["hops"] else 0.0
    sleep = 0.0 if vector["watched"] else limits["sleep"] * tired * tired * (QUIET_DROWSE if vector["quiet"] else 1)
    return [float(value) for value in [1, fidget, walk, hop, 0, 0, sleep, 0, 0, 0, 0]]


def dwell(activity, mode, mark):
    """⏳️ ``low + floor((high − low) × unit)`` ticks; ``idle`` takes its range from the mode."""
    index = ACTIVITIES.index(activity)
    low = LIMITS[mode]["idleLow"] if activity == "idle" else DWELL_LOW[index]
    high = LIMITS[mode]["idleHigh"] if activity == "idle" else DWELL_HIGH[index]
    return int(low + numpy.floor((high - low) * numpy.float64(mark)))


def shares(affinity):
    """🥧️ The chances ``[greet, cuddle, squabble]`` of an encounter at an affinity."""
    affinity = numpy.float64(affinity)
    cuddle = 0.5 + 0.4 * affinity if affinity >= 0.4 else (0.5 * affinity if affinity > 0 else 0.0)
    squabble = numpy.clip(0.55 - 0.5 * affinity, 0, 0.95) if affinity <= -0.3 else numpy.clip(0.08 - 0.5 * affinity, 0, 0.95)
    return [float(1 - cuddle - squabble), float(cuddle), float(squabble)]


def reach(followers):
    """🕸️ Per activity everything reachable from it (itself included), by scipy's breadth-first search, and the number of strongly connected components."""
    size = len(ACTIVITIES)
    rows = [ACTIVITIES.index(activity) for activity in ACTIVITIES for _ in followers[activity]]
    columns = [ACTIVITIES.index(follower) for activity in ACTIVITIES for follower in followers[activity]]
    graph = csr_matrix((numpy.ones(len(rows), dtype=numpy.int8), (rows, columns)), shape=(size, size))
    reachable = {}
    for index, activity in enumerate(ACTIVITIES):
        order = breadth_first_order(graph, index, directed=True, return_predecessors=False)
        reachable[activity] = [ACTIVITIES[int(node)] for node in sorted(int(node) for node in order)]
    components, _ = connected_components(graph, directed=True, connection="strong")
    return {"reachable": reachable, "components": int(components)}


def turns(ring, seats, word, epoch):
    """🎠️ The members of a ring that are on at an epoch, ``seats`` of them at most: the ring rolled by the word's start plus the epoch."""
    if len(ring) == 0:
        return []
    start = word % len(ring) + epoch % len(ring)
    rolled = [str(species) for species in numpy.roll(numpy.array(ring, dtype=object), -start)]
    return rolled[: max(seats, 0)]


def cast(vector, epoch):
    """🎟️ The rotation visits: it gets the seats the core leaves free, and one seat whenever the stage holds at least two. The core gets the other seats: all of it while it fits, otherwise it takes turns itself. Both rings are rolled by a start of the seed's (word 0 for the core, word 1 for the rotation) plus the epoch."""
    room = max(int(numpy.floor(vector["capacity"])), 0)
    core = []
    for species in vector["cast"]["core"]:
        if species not in core:
            core.append(species)
    pool = []
    for species in vector["cast"]["rotation"]:
        if species not in core and species not in pool:
            pool.append(species)
    guests = min(len(pool), max(room - len(core), 1 if room >= 2 else 0))
    seats = room - guests
    words = numpy.random.SeedSequence([vector["seed"], CAST_STREAM, 0]).generate_state(2)
    troupe = list(core) if len(core) <= seats else turns(core, seats, int(words[0]), epoch)
    return troupe + turns(pool, guests, int(words[1]), epoch)


# endregion 🔖️Reference


# region 🔖️Handlers
def committed(ctx):
    """🧫️ The committed vectors."""
    return json.loads(ctx.fixture_bytes(VECTORS))


def close(produced, expected):
    """📏️ Whether two answers agree: numbers within 1e-9, everything else exactly."""
    if isinstance(produced, bool) or isinstance(expected, bool) or isinstance(produced, str) or isinstance(expected, str) or produced is None or expected is None:
        return produced == expected
    if isinstance(produced, (int, float)) and isinstance(expected, (int, float)):
        return abs(produced - expected) <= 1e-9
    if isinstance(produced, list) and isinstance(expected, list):
        return len(produced) == len(expected) and all(close(left, right) for left, right in zip(produced, expected))
    if isinstance(produced, dict) and isinstance(expected, dict):
        return produced.keys() == expected.keys() and all(close(produced[key], expected[key]) for key in produced)
    return False


def agree(scenario, produced, vectors):
    """⚖️ Holds every produced answer to the committed one — the vectors may never drift from the reference."""
    for vector in vectors:
        if not close(produced[vector["id"]], vector["expected"]):
            raise AssertionError("%s/%s: the reference answers %r, the committed vector says %r" % (scenario, vector["id"], produced[vector["id"]], vector["expected"]))
    return Outcome(produced)


def limits(ctx):
    """🚦️ The limits of every mode, and the order the design gives them: still allows nothing, lively allows more than calm and waits less."""
    vectors = committed(ctx)["limits"]
    still, calm, lively = LIMITS["still"], LIMITS["calm"], LIMITS["lively"]
    if bool(numpy.any(numpy.array(list(still.values())) != 0)):
        raise AssertionError("limits: still must allow nothing")
    if not (calm["movers"] == 1 and lively["movers"] == 2 and calm["idleLow"] == 6 * 64 and calm["idleHigh"] == 20 * 64 and lively["idleLow"] == 3 * 64 and lively["idleHigh"] == 10 * 64):
        raise AssertionError("limits: movers and idle dwells must be those of design §5.6")
    if not (calm["encounterGap"] >= 90 * 64 and lively["encounterGap"] >= 30 * 64 and calm["walk"] < lively["walk"] and calm["fidget"] < lively["fidget"]):
        raise AssertionError("limits: calm must be calmer than lively")
    return agree("limits", {vector["id"]: LIMITS[vector["id"]] for vector in vectors}, vectors)


def weight_lists(ctx):
    """🧮️ ``activityWeights`` of every committed situation."""
    vectors = committed(ctx)["weights"]
    return agree("weights", {vector["id"]: weights(vector) for vector in vectors}, vectors)


def picks(ctx):
    """🗳️ ``weightedIndex(weights, unit)`` for every committed unit of every committed weight list."""
    vectors = committed(ctx)["picks"]
    return agree("picks", {vector["id"]: [pick(vector["weights"], mark) for mark in vector["units"]] for vector in vectors}, vectors)


def decisions(ctx):
    """🙋️ What an actor decides, counter after counter: the activity ``randomPick([seed, stream, counter], weights)`` names, and how often each one is named."""
    vectors = committed(ctx)["decisions"]
    produced = {}
    for vector in vectors:
        amounts = weights(vector)
        chosen = [ACTIVITIES[pick(amounts, unit([vector["seed"], vector["stream"], counter]))] for counter in range(vector["count"])]
        tally = numpy.bincount([ACTIVITIES.index(activity) for activity in chosen], minlength=len(ACTIVITIES))
        produced[vector["id"]] = {"activities": chosen, "counts": {activity: int(tally[index]) for index, activity in enumerate(ACTIVITIES)}}
    return agree("decisions", produced, vectors)


def dwells(ctx):
    """⏱️ ``dwellOf(activity, mode, unit)`` for every committed unit."""
    vectors = committed(ctx)["dwells"]
    return agree("dwells", {vector["id"]: [dwell(vector["activity"], vector["mode"], mark) for mark in vector["units"]] for vector in vectors}, vectors)


def moods(ctx):
    """🙂️ ``moodOf(activity)`` for every activity."""
    vectors = committed(ctx)["moods"]
    return agree("moods", {vector["id"]: MOODS[ACTIVITIES.index(vector["id"])] for vector in vectors}, vectors)


def encounters(ctx):
    """🎭️ ``encounterShares(affinity)`` and ``encounterOf(affinity, unit)`` for every committed unit."""
    vectors = committed(ctx)["encounters"]
    produced = {}
    for vector in vectors:
        parts = shares(vector["affinity"])
        produced[vector["id"]] = {"shares": parts, "kinds": [ENCOUNTERS[pick(parts, mark)] for mark in vector["units"]]}
    return agree("encounters", produced, vectors)


def reachability(ctx):
    """🧭️ The followers of every activity, what scipy reaches from each one and the number of strongly connected components."""
    vectors = committed(ctx)["graph"]
    return agree("reachability", {vector["id"]: {"followers": FOLLOWERS, **reach(FOLLOWERS)} for vector in vectors}, vectors)


def casts(ctx):
    """🎪️ ``castOf(cast, capacity, epoch, seed)`` for every committed epoch."""
    vectors = committed(ctx)["casts"]
    return agree("casts", {vector["id"]: [cast(vector, epoch) for epoch in vector["epochs"]] for vector in vectors}, vectors)


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter():
    """🔮️ Oracle role only: numpy and scipy are the reference, the TypeScript and Rust twins are judged against them."""
    return (
        Adapter("python")
        .oracle("limits", limits)
        .oracle("weights", weight_lists)
        .oracle("picks", picks)
        .oracle("decisions", decisions)
        .oracle("dwells", dwells)
        .oracle("moods", moods)
        .oracle("encounters", encounters)
        .oracle("reachability", reachability)
        .oracle("casts", casts)
    )


# endregion 🔖️Registration

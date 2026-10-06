#!/usr/bin/env python3
"""🤝️ Oracle of the pets bond and need dynamics (design §4.6, §5.2, §5.5), in Python.

How two species feel about each other is an authored affinity plus the drift of their shared history, held inside
``[−0.6, 1]``; the drift takes steps with every encounter, stays inside ``[−0.5, 0.5]`` and fades linearly back to 0
(a tenth in ten minutes). The drives of an actor move linearly with what it does and stay inside ``[0, 1]``. All of
it is clipped linear arithmetic, which numpy evaluates here on its own (``numpy.clip``, ``numpy.sign``,
``numpy.maximum``, ``numpy.abs``), written from the design text.

@see https://numpy.org/doc/stable/reference/generated/numpy.clip.html
@see ../../🧫️fixtures/🤝️bond-dynamics/🔣️.json
"""

# region 🔖️Imports
import json

import numpy

from semio_repo_test import Adapter, Outcome

# endregion 🔖️Imports


# region 🔖️Reference
VECTORS = "shared://🤝️bond-dynamics/🔣️.json"
EPISODES = ["hang", "tumble", "glide", "aim", "reel", "climb", "mantle", "slide", "carry", "trick", "purr", "dizzy", "shrug", "scoot", "push"]
ACTIVITIES = ["idle", "fidget", "walk", "hop", "fall", "land", "sleep", "greet", "cuddle", "squabble", "sulk"] + EPISODES
AFFINITY_FLOOR = -0.6
RAPPORT_SPAN = 0.5
RAPPORT_STEPS = [0, 0, 0, 0, 0, 0, 0, 0.05, 0.1, -0.15, 0.1] + [0] * len(EPISODES)
RAPPORT_FADE_STEP = 0.1
RAPPORT_FADE_TICKS = 38400
ENERGY_RATES = [-0.001, -0.01, -0.012, -0.02, 0, 0, 0.02, -0.006, -0.004, -0.012, -0.001] + [0, 0, 0, -0.004, -0.012, -0.016, -0.016, -0.002, -0.014, -0.01, 0.004, -0.002, -0.001, -0.012, -0.014]
SOCIABILITY_RATES = [0.002, 0.002, 0.002, 0.002, 0, 0.002, 0.002, -0.12, -0.12, -0.12, -0.02] + [0, 0, 0.002, 0.002, 0.002, 0.002, 0.002, 0.002, 0.002, 0.002, -0.06, 0.002, 0.002, 0.002, 0.002]
CURIOSITY_RATES = [0.01, -0.08, -0.06, -0.1, 0, 0.01, 0.01, 0, 0, 0, 0.01] + [0, 0, -0.04, -0.06, -0.08, -0.08, -0.08, -0.04, -0.06, -0.08, 0.01, 0, 0.01, -0.02, -0.1]


def joined(pair, a, b):
    """🔗️ Whether a pair joins the two species, in either order."""
    return (pair[0] == a and pair[1] == b) or (pair[0] == b and pair[1] == a)


def affinity(bonds, rapports, a, b):
    """💞️ Authored affinity plus drift, clipped to ``[−0.6, 1]``; 0 for a species and itself."""
    if a == b:
        return 0.0
    authored = next((bond["affinity"] for bond in bonds if joined(bond["between"], a, b)), 0.0)
    drift = next((rapport["drift"] for rapport in rapports if joined(rapport["between"], a, b)), 0.0)
    return float(numpy.clip(numpy.float64(authored) + numpy.float64(drift), AFFINITY_FLOOR, 1))


def stepped(drift, activity):
    """🪢️ The drift after an encounter, clipped to ``[−0.5, 0.5]``."""
    return float(numpy.clip(numpy.float64(drift) + RAPPORT_STEPS[ACTIVITIES.index(activity)], -RAPPORT_SPAN, RAPPORT_SPAN))


def faded(drift, ticks):
    """🍂️ The drift ``ticks`` later: its magnitude less ``0.1 × ticks ÷ 38400``, never below 0, with its sign."""
    drift = numpy.float64(drift)
    step = (RAPPORT_FADE_STEP * ticks) / RAPPORT_FADE_TICKS
    return float(numpy.sign(drift) * numpy.maximum(numpy.abs(drift) - step, 0.0) + 0.0)


def arrived(temperament):
    """🌱️ The drives a pet arrives with."""
    return {"energy": float(0.5 + 0.5 * numpy.float64(temperament["energy"])), "sociability": temperament["sociability"], "curiosity": temperament["curiosity"]}


def spent(needs, activity, ticks, temperament):
    """🔋️ The drives after ``ticks`` of an activity, each clipped to ``[0, 1]``."""
    index = ACTIVITIES.index(activity)
    seconds = numpy.float64(ticks) / 64
    energy = ENERGY_RATES[index] * (1.5 - temperament["energy"]) if ENERGY_RATES[index] < 0 else ENERGY_RATES[index]
    sociability = SOCIABILITY_RATES[index] * (0.5 + temperament["sociability"]) if SOCIABILITY_RATES[index] > 0 else SOCIABILITY_RATES[index]
    curiosity = CURIOSITY_RATES[index] * (0.5 + temperament["curiosity"]) if CURIOSITY_RATES[index] > 0 else CURIOSITY_RATES[index]
    return {
        "energy": float(numpy.clip(needs["energy"] + energy * seconds, 0, 1)),
        "sociability": float(numpy.clip(needs["sociability"] + sociability * seconds, 0, 1)),
        "curiosity": float(numpy.clip(needs["curiosity"] + curiosity * seconds, 0, 1)),
    }


# endregion 🔖️Reference


# region 🔖️Handlers
def committed(ctx):
    """🧫️ The committed vectors."""
    return json.loads(ctx.input_bytes(VECTORS))


def close(produced, expected):
    """📏️ Whether two answers agree: numbers within 1e-9, everything else exactly."""
    if isinstance(produced, (int, float)) and isinstance(expected, (int, float)) and not isinstance(produced, bool) and not isinstance(expected, bool):
        return abs(produced - expected) <= 1e-9
    if isinstance(produced, list) and isinstance(expected, list):
        return len(produced) == len(expected) and all(close(left, right) for left, right in zip(produced, expected))
    if isinstance(produced, dict) and isinstance(expected, dict):
        return produced.keys() == expected.keys() and all(close(produced[key], expected[key]) for key in produced)
    return produced == expected


def agree(scenario, produced, vectors):
    """⚖️ Holds every produced answer to the committed one — the vectors may never drift from the reference."""
    for vector in vectors:
        if not close(produced[vector["id"]], vector["expected"]):
            raise AssertionError("%s/%s: the reference answers %r, the committed vector says %r" % (scenario, vector["id"], produced[vector["id"]], vector["expected"]))
    return Outcome(produced)


def affinities(ctx):
    """💗️ ``affinityOf`` of every committed pair over the committed bonds and rapports."""
    vectors = committed(ctx)["affinities"]
    return agree("affinities", {vector["id"]: [affinity(vector["bonds"], vector["rapports"], pair[0], pair[1]) for pair in vector["pairs"]] for vector in vectors}, vectors)


def rapport_steps(ctx):
    """👣️ ``rapportAfter(drift, activity)`` for every activity."""
    vectors = committed(ctx)["rapportSteps"]
    return agree("rapport-steps", {vector["id"]: {activity: stepped(vector["drift"], activity) for activity in ACTIVITIES} for vector in vectors}, vectors)


def rapport_fading(ctx):
    """⌛️ ``rapportFaded(drift, ticks)`` for every committed span."""
    vectors = committed(ctx)["rapportFading"]
    return agree("rapport-fading", {vector["id"]: [faded(vector["drift"], ticks) for ticks in vector["ticks"]] for vector in vectors}, vectors)


def histories(ctx):
    """📖️ The shared history of a pair: before every event the drift fades over the ticks since the last one, then it takes the step of the event; the affinity is the authored one plus the drift, and never falls below −0.6."""
    vectors = committed(ctx)["histories"]
    produced = {}
    for vector in vectors:
        drift = 0.0
        drifts = []
        feelings = []
        for event in vector["events"]:
            drift = stepped(faded(drift, event["after"]), event["activity"])
            drifts.append(drift)
            feelings.append(affinity([{"between": ["a", "b"], "affinity": vector["affinity"]}], [{"between": ["a", "b"], "drift": drift}], "b", "a"))
        if len(feelings) > 0 and float(numpy.min(feelings)) < AFFINITY_FLOOR:
            raise AssertionError("histories/%s: an affinity fell below the floor" % vector["id"])
        produced[vector["id"]] = {"drifts": drifts, "affinities": feelings}
    return agree("histories", produced, vectors)


def needs(ctx):
    """🪫️ ``needsAfter(needs, activity, ticks, temperament)`` for every committed span."""
    vectors = committed(ctx)["needs"]
    return agree("needs", {vector["id"]: spent(vector["needs"], vector["activity"], vector["ticks"], vector["temperament"]) for vector in vectors}, vectors)


def days(ctx):
    """🌗️ A day in the life of a temperament: from ``needsOf`` through one span after the other."""
    vectors = committed(ctx)["days"]
    produced = {}
    for vector in vectors:
        state = arrived(vector["temperament"])
        states = [state]
        for span in vector["spans"]:
            state = spent(state, span["activity"], span["ticks"], vector["temperament"])
            states.append(state)
        produced[vector["id"]] = states
    return agree("days", produced, vectors)


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter():
    """🔮️ Oracle role only: numpy is the reference, the TypeScript and Rust twins are judged against it."""
    return Adapter("python").oracle("affinities", affinities).oracle("rapport-steps", rapport_steps).oracle("rapport-fading", rapport_fading).oracle("histories", histories).oracle("needs", needs).oracle("days", days)


# endregion 🔖️Registration

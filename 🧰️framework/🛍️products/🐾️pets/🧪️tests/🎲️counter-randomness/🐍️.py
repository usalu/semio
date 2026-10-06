#!/usr/bin/env python3
"""🎲️ Oracle of the pets counter-based randomness (design §2.4, §4.2), in Python.

The words are numpy's own: ``numpy.random.SeedSequence(key).generate_state(count)`` for a key of
unsigned 32-bit integers — the library hashes the key into its pool of four words and reads the pool
out, and nothing of that hash is restated here. A unit draw is the first word divided by 2³², a ranged
draw ``low + (high − low) × unit``, and a weighted pick the index ``numpy.searchsorted`` finds for
``unit × total`` in ``numpy.cumsum`` of the positive weights (side ``right``: the first running sum
that exceeds the mark); −1 says no weight is positive.

@see https://numpy.org/doc/stable/reference/random/bit_generators/generated/numpy.random.SeedSequence.html
@see https://numpy.org/doc/stable/reference/generated/numpy.searchsorted.html
@see ../../🧫️fixtures/🎲️counter-randomness/🔣️.json
"""

# region 🔖️Imports
import json

import numpy

from semio_repo_test import Adapter, Outcome

# endregion 🔖️Imports


# region 🔖️Reference
VECTORS = "shared://🎲️counter-randomness/🔣️.json"


def words(key, count):
    """🔢️ The first ``count`` words numpy's seed sequence generates for the key."""
    return [int(word) for word in numpy.random.SeedSequence([int(part) for part in key]).generate_state(count)]


def unit(key):
    """🎯️ The first word divided by 2³²: a multiple of 2⁻³² in [0, 1)."""
    return words(key, 1)[0] / 4294967296.0


def between(key, low, high):
    """📏️ ``low + (high − low) × unit``."""
    return float(numpy.float64(low) + (numpy.float64(high) - numpy.float64(low)) * numpy.float64(unit(key)))


def pick(key, weights):
    """🎰️ The index whose running sum of positive weights first exceeds ``unit × total``; −1 without a positive weight."""
    if len(weights) == 0:
        return -1
    amounts = numpy.array(weights, dtype=numpy.float64)
    running = numpy.cumsum(numpy.where(amounts > 0, amounts, 0.0))
    total = float(running[-1])
    if not total > 0:
        return -1
    index = int(numpy.searchsorted(running, numpy.float64(unit(key)) * running[-1], side="right"))
    return index if index < len(weights) else int(numpy.flatnonzero(amounts > 0)[-1])


# endregion 🔖️Reference


# region 🔖️Handlers
def committed(ctx):
    """🧫️ The committed vectors."""
    return json.loads(ctx.input_bytes(VECTORS))


def agree(scenario, produced, vectors):
    """⚖️ Holds every produced answer to the committed one, exactly — the vectors may never drift from the reference."""
    for vector in vectors:
        if produced[vector["id"]] != vector["expected"]:
            raise AssertionError("%s/%s: the reference answers %r, the committed vector says %r" % (scenario, vector["id"], produced[vector["id"]], vector["expected"]))
    return Outcome(produced)


def word_lists(ctx):
    """🧮️ ``randomWords(key, count)`` for every committed key."""
    vectors = committed(ctx)["words"]
    return agree("words", {vector["id"]: words(vector["key"], vector["count"]) for vector in vectors}, vectors)


def units(ctx):
    """🪙️ ``randomUnit(key)`` for every committed key."""
    vectors = committed(ctx)["units"]
    return agree("units", {vector["id"]: unit(vector["key"]) for vector in vectors}, vectors)


def streams(ctx):
    """🧵️ The unit draws of the keys ``[seed, stream, 0 … count − 1]``: what one actor draws, decision after decision."""
    vectors = committed(ctx)["streams"]
    return agree("streams", {vector["id"]: [unit([vector["seed"], vector["stream"], counter]) for counter in range(vector["count"])] for vector in vectors}, vectors)


def ranges(ctx):
    """📐️ ``randomBetween(key, low, high)`` for every committed key and range."""
    vectors = committed(ctx)["ranges"]
    return agree("ranges", {vector["id"]: between(vector["key"], vector["low"], vector["high"]) for vector in vectors}, vectors)


def picks(ctx):
    """🗳️ ``randomPick([seed, stream, counter], weights)`` for the counters ``0 … count − 1`` of every committed weight list."""
    vectors = committed(ctx)["picks"]
    return agree("picks", {vector["id"]: [pick([vector["seed"], vector["stream"], counter], vector["weights"]) for counter in range(vector["count"])] for vector in vectors}, vectors)


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter():
    """🔮️ Oracle role only: numpy is the reference, the TypeScript and Rust twins are judged against it."""
    return Adapter("python").oracle("words", word_lists).oracle("units", units).oracle("streams", streams).oracle("ranges", ranges).oracle("picks", picks)


# endregion 🔖️Registration

#!/usr/bin/env python3
"""🎲️ Oracle of the quiz product's seeded randomness (design §3), in Python.

The generator is numpy's own ``MT19937`` bit generator, seeded through numpy's public legacy seeding
(``RandomState(seed)`` runs ``init_genrand`` for a scalar seed) and read through ``random_raw``, so the
tempered 32-bit stream is produced by a library that has never seen this repository. CPython's
``random.Random`` — a second, unrelated MT19937 implementation — is loaded with the same 624-word state
and must emit the same words, and the C++ standard's own check value (the 10000th output of a
default-seeded ``std::mt19937`` is 4123659995, [rand.predef]) is held as a committed vector.

FNV-1a, the rejection-sampled ``uniform`` and the Fisher–Yates ``shuffle`` are written here from the
design text only; they consume the library's stream and nothing else.

@see https://numpy.org/doc/stable/reference/random/bit_generators/mt19937.html
@see https://eel.is/c++draft/rand.predef
@see ../../🧫️fixtures/🎲️seeded-randomness/🔣️.json
"""

# region 🔖️Imports
import json
import random

import numpy

from semio_repo_test import Adapter, Outcome

# endregion 🔖️Imports


# region 🔖️Reference
VECTORS = "shared://🎲️seeded-randomness/🔣️.json"


def fnv1a32(text):
    """#️⃣ FNV-1a 32-bit over the UTF-8 bytes of ``text``."""
    value = 2166136261
    for byte in text.encode("utf-8"):
        value = ((value ^ byte) * 16777619) & 0xFFFFFFFF
    return value


class Mt19937:
    """🌀️ numpy's MT19937 seeded with ``init_genrand(seed)``, read one tempered word at a time."""

    def __init__(self, seed):
        """🔑️ Seeds the generator with ``init_genrand(seed)`` through numpy's legacy seeding."""
        state = numpy.random.RandomState(seed).get_state()
        self.key = [int(word) for word in state[1]]
        self.position = int(state[2])
        self.generator = numpy.random.MT19937()
        self.generator.state = {"bit_generator": "MT19937", "state": {"key": state[1], "pos": state[2]}}

    def next(self):
        """🔢️ The next tempered u32."""
        return int(self.generator.random_raw())

    def cpython_twin(self):
        """🐍️ CPython's own MT19937 loaded with the same initial state — an independent generator."""
        twin = random.Random()
        twin.setstate((3, tuple(self.key) + (self.position,), None))
        return twin


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
    """🃏️ Fisher–Yates from the end on a copy."""
    shuffled = list(items)
    for index in range(len(shuffled) - 1, 0, -1):
        other = uniform(generator, index + 1)
        shuffled[index], shuffled[other] = shuffled[other], shuffled[index]
    return shuffled


# endregion 🔖️Reference


# region 🔖️Handlers
def committed(ctx):
    """🧫️ The committed vectors."""
    return json.loads(ctx.fixture_bytes(VECTORS))


def agree(scenario, produced, vectors, field):
    """⚖️ Holds every produced answer to the committed one — the vectors may never drift from the reference."""
    for vector in vectors:
        if produced[vector["id"]] != vector[field]:
            raise AssertionError("%s/%s: the reference answers %r, the committed vector says %r" % (scenario, vector["id"], produced[vector["id"]], vector[field]))
    return Outcome(produced)


def hashes(ctx):
    """🔣️ ``fnv1a32(text)`` for every committed text."""
    vectors = committed(ctx)["hashes"]
    return agree("hashes", {vector["id"]: fnv1a32(vector["text"]) for vector in vectors}, vectors, "hash")


def run_seeds(ctx):
    """🌱️ ``runSeed(run)`` for every committed run id."""
    vectors = committed(ctx)["runSeeds"]
    return agree("run-seeds", {vector["id"]: fnv1a32(vector["run"]) for vector in vectors}, vectors, "seed")


def raw_outputs(ctx):
    """🌊️ The first words after skipping, cross-checked against CPython's generator loaded with the same state."""
    produced = {}
    vectors = committed(ctx)["rawOutputs"]
    for vector in vectors:
        generator = Mt19937(vector["seed"])
        twin = generator.cpython_twin()
        words = [generator.next() for _ in range(vector["skip"] + vector["count"])][vector["skip"] :]
        twin_words = [twin.getrandbits(32) for _ in range(vector["skip"] + vector["count"])][vector["skip"] :]
        if words != twin_words:
            raise AssertionError("raw-outputs/%s: numpy emits %r, CPython's MT19937 emits %r from the same state" % (vector["id"], words, twin_words))
        produced[vector["id"]] = words
    return agree("raw-outputs", produced, vectors, "outputs")


def uniform_draws(ctx):
    """🎰️ Sequential ``uniform(bound)`` draws, then the next raw word — which pins how many words the draws consumed."""
    produced = {}
    vectors = committed(ctx)["uniformDraws"]
    for vector in vectors:
        generator = Mt19937(vector["seed"])
        draws = [uniform(generator, bound) for bound in vector["bounds"]]
        produced[vector["id"]] = {"draws": draws, "next": generator.next()}
    return agree("uniform-draws", produced, vectors, "expected")


def shuffles(ctx):
    """🔀️ ``shuffle([0 … length−1])``, then the next raw word."""
    produced = {}
    vectors = committed(ctx)["shuffles"]
    for vector in vectors:
        generator = Mt19937(vector["seed"])
        produced[vector["id"]] = {"permutation": shuffle(generator, range(vector["length"])), "next": generator.next()}
    return agree("shuffles", produced, vectors, "expected")


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter():
    """🧭️ Oracle role only: numpy is the reference, the TypeScript and Rust twins are judged against it."""
    return Adapter("python").oracle("hashes", hashes).oracle("run-seeds", run_seeds).oracle("raw-outputs", raw_outputs).oracle("uniform-draws", uniform_draws).oracle("shuffles", shuffles)


# endregion 🔖️Registration

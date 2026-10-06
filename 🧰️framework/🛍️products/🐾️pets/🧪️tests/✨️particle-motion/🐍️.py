#!/usr/bin/env python3
"""✨️ Oracle of the pets particles (design-v2 §19, research2 §9), in Python.

The hash is evaluated in numpy's ``uint32`` arithmetic, which wraps at 32 bits by itself: ``lowbias32``
(xor-shift 16, multiply by 0x7feb352d, xor-shift 15, multiply by 0x846ca68b, xor-shift 16) and
``mix(a, b) = lowbias32((a xor 0x9e3779b9) + (b + 1)·0x85ebca6b)``; the five published words of the research
report are checked before anything is projected. A unit is a word divided by 2³².

Which particles are alive is found the slow way: a simulation walks from the start of the emitter tick by tick,
puts every particle into a pool at the tick the birth rule names, takes it out when its life is over and drops
the eldest when the pool is fuller than the emitter allows — the subject looks at a window of indices and
stores nothing. Where the particles are is the same formulas evaluated for all of them at once with
``numpy.sin`` and ``numpy.cos`` of ``2π·t``, ``numpy.clip`` for the eases and the rational decay
``1 ÷ (1 + x + 0.48x² + 0.235x³)``, which is held to ``numpy.exp(−x)`` within the bound its author states (0.02)
wherever it is used — and with it the path of every thrown particle to the path under a true exponential drag.

The cap is ``numpy.argsort`` (stable) of the ages; the end of an emitter is its closed form, held against the
simulation: nothing may be alive from that tick on.

@see https://nullprogram.com/blog/2018/07/31/
@see https://numpy.org/doc/stable/user/basics.types.html
@see https://numpy.org/doc/stable/reference/generated/numpy.argsort.html
@see ../../🧫️fixtures/✨️particle-motion/🔣️.json
"""

# region 🔖️Imports
import json

import numpy

from semio_repo_test import Adapter, Outcome

# endregion 🔖️Imports


# region 🔖️Constants
VECTORS = "shared://✨️particle-motion/🔣️.json"
TOLERANCE = 1e-12
TICKS_PER_SECOND = 64
EMITTER_CAP = 32
AHEAD = 0.0
DOWN = 0.25
UP = 0.75
LANE_BIRTH = 0
LANE_HEADING = 1
LANE_PACE = 2
LANE_PHASE = 3
LANE_LOOK = 4
FALL_SWAY = 3.0
FALL_SWAY_SPEED = 60.0
FALL_SWAY_RATE = 0.5
FALL_FADE_IN = 4.0
RISE_WANDER = 24.0
RISE_WANDER_RATE = 0.35
RISE_ROCK = 0.1
BURST_DRAG = 0.4
BURST_GRAVITY = 120.0
ORBIT_SQUASH = 0.35
ORBIT_DEPTH = 0.15
ORBIT_FADE = 8
DRIFT_MEANDER = 3.0
DRIFT_RATE = 0.5
DECAY_BOUND = 0.02
BINS = 16
GOLDEN = (("low", (1,), 0x688990C0), ("low", (0xDEADBEEF,), 0xE628C683), ("mix", (0, 0), 0xE577F3AA), ("mix", (1, 2), 0xB111E030), ("mix", (7, 3, 5), 0x5AB36A78))
# endregion 🔖️Constants


# region 🔖️Hash
def words_of(values):
    """🔢️ Values as an array of unsigned 32-bit words (a single value becomes an array of one)."""
    return numpy.atleast_1d(numpy.asarray(values, dtype=numpy.uint64) & numpy.uint64(0xFFFFFFFF)).astype(numpy.uint32)


def lowbias(words):
    """🧂️ ``lowbias32`` of every word, in ``uint32`` arithmetic."""
    stirred = words_of(words).copy()
    stirred ^= stirred >> numpy.uint32(16)
    stirred *= numpy.uint32(0x7FEB352D)
    stirred ^= stirred >> numpy.uint32(15)
    stirred *= numpy.uint32(0x846CA68B)
    stirred ^= stirred >> numpy.uint32(16)
    return stirred


def mix(first, second):
    """🥣️ ``lowbias32((a xor 0x9e3779b9) + (b + 1)·0x85ebca6b)`` of every pair, wrapping at 32 bits."""
    return lowbias((words_of(first) ^ numpy.uint32(0x9E3779B9)) + (words_of(second) + numpy.uint32(1)) * numpy.uint32(0x85EBCA6B))


def chained(chain):
    """⛓️ ``mix`` folded over a chain of words from the left: ``[a, b, c]`` is ``mix(mix(a, b), c)``."""
    word = words_of(chain[0])
    for link in chain[1:]:
        word = mix(word, link)
    return int(word[0])


def unit(words):
    """🪙️ Words as numbers in [0, 1): each divided by 2³²."""
    return words_of(words).astype(numpy.float64) / 4294967296.0


def scattered(key, indices, lane):
    """🎲️ What the particles ``indices`` of the emitter keyed ``key`` read from their lane ``lane``."""
    return unit(mix(mix(key, indices), lane))


def golden():
    """🏅️ Refuses to answer anything unless numpy reproduces the five published words of the hash."""
    for kind, words, expected in GOLDEN:
        produced = int(lowbias(words[0])[0]) if kind == "low" else chained(words)
        if produced != expected:
            raise AssertionError("golden %s%r: numpy answers %#010x, the research report says %#010x" % (kind, words, produced, expected))


def spread_of(vector):
    """📊️ The sum of the words and the histogram of the units of one committed run of indices over sixteen equal bins, held to what a uniform lane must look like."""
    indices = numpy.arange(vector["first"], vector["first"] + vector["count"], dtype=numpy.uint64)
    words = mix(mix(vector["key"], indices), vector["lane"])
    units = unit(words)
    bins = numpy.bincount(numpy.floor(units * BINS).astype(numpy.int64), minlength=BINS)
    count = vector["count"]
    if count >= 4096:
        if abs(float(units.mean()) - 0.5) > 4.0 * numpy.sqrt(1.0 / 12.0 / count):
            raise AssertionError("uniformity/%s: the mean %r is not that of a uniform lane" % (vector["id"], float(units.mean())))
        if abs(float(units.var()) - 1.0 / 12.0) > 4.0 * numpy.sqrt(1.0 / 180.0 / count):
            raise AssertionError("uniformity/%s: the variance %r is not that of a uniform lane" % (vector["id"], float(units.var())))
        chi = float(numpy.sum((bins - count / BINS) ** 2 / (count / BINS)))
        if chi > 45.0:
            raise AssertionError("uniformity/%s: chi-square %r over %d bins" % (vector["id"], chi, BINS))
    return {"total": int(words.astype(numpy.uint64).sum()), "bins": [int(value) for value in bins]}


# endregion 🔖️Hash


# region 🔖️Births
def life_ticks(emitter):
    """⏳️ The ticks a particle lives: ``⌊life × 64 + ½⌋``, at least one."""
    return max(1, int(numpy.floor(numpy.float64(emitter["life"]) * TICKS_PER_SECOND + 0.5)))


def swarm_of(emitter):
    """👥️ How many particles the emitter keeps alive at most."""
    return int(numpy.clip(numpy.floor(emitter["count"]), 0, EMITTER_CAP))


def period_of(emitter):
    """🥁️ The ticks between two births: ``⌈life ÷ count⌉``, at least one."""
    return max(1, -(-life_ticks(emitter) // max(1, swarm_of(emitter))))


def births(emitter, since, key, count):
    """🐣️ The ticks the first ``count`` particles of a continuous emitter are born."""
    period = period_of(emitter)
    indices = numpy.arange(count, dtype=numpy.uint64)
    return since + indices.astype(numpy.int64) * period + numpy.floor(scattered(key, indices, LANE_BIRTH) * period).astype(numpy.int64)


def simulated(emitter, since, until, key, ticks):
    """🕹️ Who is alive at every asked tick, eldest first, by walking every tick from the start of the emitter with a pool of particles: ``{tick: [(index, age), …]}``."""
    life = life_ticks(emitter)
    swarm = swarm_of(emitter)
    record = {tick: [] for tick in ticks}
    if swarm < 1 or not ticks or max(ticks) < since:
        return record
    horizon = max(ticks)
    wanted = set(ticks)
    if emitter["motion"] == "burst":
        for tick in range(since, horizon + 1):
            if tick in wanted and tick - since < life:
                record[tick] = [(index, tick - since) for index in range(swarm)]
        return record
    if emitter["motion"] == "orbit":
        for tick in range(since, horizon + 1):
            if tick in wanted and (until is None or tick < until + ORBIT_FADE):
                record[tick] = [(index, tick - since) for index in range(swarm)]
        return record
    born = births(emitter, since, key, (horizon - since) // period_of(emitter) + 2)
    slots = {}
    for index, tick in enumerate(born):
        slots.setdefault(int(tick), []).append(index)
    pool = []
    for tick in range(since, horizon + 1):
        pool = [index for index in pool if tick - int(born[index]) < life]
        if until is None or tick < until:
            pool.extend(slots.get(tick, []))
        if len(pool) > swarm:
            pool = pool[len(pool) - swarm :]
        if tick in wanted:
            record[tick] = [(index, tick - int(born[index])) for index in pool]
    return record


# endregion 🔖️Births


# region 🔖️Motions
def sine(turns):
    """🌀️ ``numpy.sin`` of an angle in turns."""
    return numpy.sin(2.0 * numpy.pi * numpy.asarray(turns, dtype=numpy.float64))


def cosine(turns):
    """🧭️ ``numpy.cos`` of an angle in turns."""
    return numpy.cos(2.0 * numpy.pi * numpy.asarray(turns, dtype=numpy.float64))


def smoothstep(amounts):
    """🛝️ ``3t² − 2t³`` of the amounts clipped to [0, 1]."""
    held = numpy.clip(numpy.asarray(amounts, dtype=numpy.float64), 0.0, 1.0)
    return held * held * (3.0 - 2.0 * held)


def decay(amounts):
    """📉️ The rational decay ``1 ÷ (1 + x·(1 + x·(0.48 + 0.235·x)))``, refused when it leaves ``numpy.exp(−x)`` by more than its stated bound."""
    held = numpy.maximum(numpy.asarray(amounts, dtype=numpy.float64), 0.0)
    rational = 1.0 / (1.0 + held * (1.0 + held * (0.48 + 0.235 * held)))
    if numpy.any(numpy.abs(rational - numpy.exp(-held)) > DECAY_BOUND):
        raise AssertionError("the rational decay leaves numpy.exp by more than %r" % DECAY_BOUND)
    return rational


def placed(emitter, origin, facing, since, until, key, tick, alive):
    """🎇️ The particles alive at one tick as drawn — x, y, scale, rotation, opacity, age —, all of them evaluated at once."""
    if not alive:
        return []
    indices = numpy.array([index for index, _ in alive], dtype=numpy.float64)
    ages = numpy.array([age for _, age in alive], dtype=numpy.float64)
    life = float(life_ticks(emitter))
    swarm = float(swarm_of(emitter))
    speed = numpy.float64(emitter["speed"])
    spread = numpy.float64(emitter["spread"])
    seconds = ages / TICKS_PER_SECOND
    share = ages / life
    lane = lambda number: scattered(key, indices.astype(numpy.uint64), number)
    motion = emitter["motion"]
    if motion == "fall":
        heading = DOWN + (lane(LANE_HEADING) - 0.5) * spread
        pace = speed * (0.9 + 0.2 * lane(LANE_PACE))
        sway = FALL_SWAY * decay(pace / FALL_SWAY_SPEED) * sine(FALL_SWAY_RATE * seconds + lane(LANE_PHASE))
        x = origin["x"] + facing * (pace * cosine(heading) * seconds + sway)
        y = origin["y"] + pace * sine(heading) * seconds
        scale = 0.8 + 0.4 * lane(LANE_LOOK)
        rotation = facing * (heading - DOWN)
        opacity = numpy.minimum(1.0, ages / FALL_FADE_IN) * (0.55 + 0.35 * lane(LANE_LOOK)) * (1.0 - smoothstep((share - 0.75) / 0.25))
    elif motion == "rise":
        heading = UP + (lane(LANE_HEADING) - 0.5) * spread
        pace = speed * (0.7 + 0.3 * lane(LANE_PACE))
        swing = RISE_WANDER_RATE * seconds + lane(LANE_PHASE)
        x = origin["x"] + facing * (pace * cosine(heading) * seconds + RISE_WANDER * spread * sine(swing) * (0.3 + share))
        y = origin["y"] + pace * sine(heading) * seconds
        scale = 0.5 + 0.5 * smoothstep(ages / 8.0)
        rotation = facing * RISE_ROCK * spread * cosine(swing)
        opacity = smoothstep(ages / 6.0) * (1.0 - smoothstep((share - 0.6) / 0.4))
    elif motion == "burst":
        heading = UP + ((indices + lane(LANE_HEADING)) / swarm - 0.5) * spread
        pace = speed * (0.45 + 0.55 * lane(LANE_PACE))
        reach = BURST_DRAG * (1.0 - decay(seconds / BURST_DRAG))
        x = origin["x"] + facing * (pace * cosine(heading) * reach)
        y = origin["y"] + pace * sine(heading) * reach + 0.5 * BURST_GRAVITY * seconds * seconds
        true_reach = BURST_DRAG * (1.0 - numpy.exp(-seconds / BURST_DRAG))
        if numpy.any(numpy.abs(pace * (reach - true_reach)) > DECAY_BOUND * float(speed) * BURST_DRAG + 1e-9):
            raise AssertionError("a thrown particle leaves the path under an exponential drag by more than %r of its reach" % DECAY_BOUND)
        scale = 1.0 - 0.6 * share
        rotation = heading if facing > 0 else 0.5 - heading
        opacity = 1.0 - share * share
    elif motion == "orbit":
        radius = speed * (life / TICKS_PER_SECOND) / (2.0 * numpy.pi)
        turned = (indices / swarm) * spread + ages / life
        angle = turned if facing > 0 else 0.5 - turned
        x = origin["x"] + radius * cosine(angle)
        y = origin["y"] + radius * ORBIT_SQUASH * sine(angle)
        scale = 1.0 + ORBIT_DEPTH * sine(angle)
        rotation = numpy.zeros_like(ages)
        opacity = smoothstep(ages / ORBIT_FADE) * (1.0 if until is None else 1.0 - smoothstep((tick - until) / ORBIT_FADE))
    elif motion == "drift":
        heading = AHEAD + (lane(LANE_HEADING) - 0.5) * spread
        pace = speed * (0.5 + 0.5 * lane(LANE_PACE))
        meander = DRIFT_MEANDER * sine(DRIFT_RATE * seconds + lane(LANE_PHASE))
        x = origin["x"] + facing * (pace * cosine(heading) * seconds - meander * sine(heading))
        y = origin["y"] + pace * sine(heading) * seconds + meander * cosine(heading)
        scale = 0.6 + 0.6 * lane(LANE_LOOK)
        rotation = heading if facing > 0 else 0.5 - heading
        opacity = smoothstep(share / 0.3) * (1.0 - smoothstep((share - 0.7) / 0.3)) * (0.6 + 0.4 * lane(LANE_LOOK))
    else:
        raise AssertionError("unknown motion %r" % motion)
    columns = numpy.broadcast_arrays(x, y, scale, rotation, opacity, ages)
    return [{"x": float(a), "y": float(b), "scale": float(c), "rotation": float(d), "opacity": float(e), "age": int(f)} for a, b, c, d, e, f in zip(*columns)]


def flight(vector):
    """🎬️ The particles of one committed emitter at each of its committed ticks."""
    record = simulated(vector["emitter"], vector["since"], vector["until"], vector["key"], vector["ticks"])
    return [placed(vector["emitter"], vector["origin"], vector["facing"], vector["since"], vector["until"], vector["key"], tick, record[tick]) for tick in vector["ticks"]]


def lifetimes(vector):
    """🗓️ The births of the first committed particles and, tick by tick, the ages of those alive, eldest first."""
    emitter = vector["emitter"]
    record = simulated(emitter, vector["since"], vector["until"], vector["key"], list(range(vector["first"], vector["last"] + 1)))
    born = [int(tick) for tick in births(emitter, vector["since"], vector["key"], vector["births"])]
    if any(later <= earlier for earlier, later in zip(born, born[1:])):
        raise AssertionError("births/%s: the births are not in the order of their indices" % vector["id"])
    return {"life": life_ticks(emitter), "swarm": swarm_of(emitter), "period": period_of(emitter), "born": born, "ages": [[age for _, age in record[tick]] for tick in range(vector["first"], vector["last"] + 1)]}


# endregion 🔖️Motions


# region 🔖️Caps
def kept(vector):
    """✂️ The positions that survive a cap: the youngest by ``numpy.argsort`` (stable, so the earlier of two of one age wins), back in their order."""
    ages = numpy.array(vector["ages"], dtype=numpy.int64)
    room = max(0, int(numpy.floor(vector["cap"])))
    return sorted(int(position) for position in numpy.argsort(ages, kind="stable")[:room])


def ending(vector):
    """🏁️ The first tick from which nothing of the emitter is alive, or ``None`` while it has no end; held against the simulation over the committed keys."""
    emitter = vector["emitter"]
    since = vector["since"]
    until = vector["until"]
    life = life_ticks(emitter)
    if swarm_of(emitter) < 1:
        end = since
    elif emitter["motion"] == "burst":
        end = since + life
    elif until is None:
        end = None
    elif emitter["motion"] == "orbit":
        end = max(since, until + ORBIT_FADE)
    else:
        end = since if until <= since else until + life - 1
    if end is not None:
        after = list(range(end, end + 2 * life + 4))
        before = list(range(since, end))
        for key in vector["keys"]:
            record = simulated(emitter, since, until, key, before + after)
            if any(record[tick] for tick in after):
                raise AssertionError("ends/%s: something is alive at or after tick %d" % (vector["id"], end))
            if emitter["motion"] in ("burst", "orbit") and before and swarm_of(emitter) > 0 and not record[end - 1]:
                raise AssertionError("ends/%s: nothing is alive at tick %d, one before the end" % (vector["id"], end - 1))
    return end


# endregion 🔖️Caps


# region 🔖️Handlers
def committed(ctx):
    """🧫️ The committed vectors."""
    return json.loads(ctx.input_bytes(VECTORS))


def close(produced, expected):
    """🤏️ Structural equality with a tolerance of 1e-12 on fractions (numpy's sine may differ by an ulp between builds); whole numbers compare exactly."""
    if isinstance(produced, bool) or isinstance(expected, bool) or produced is None or expected is None:
        return produced == expected and type(produced) == type(expected)
    if isinstance(produced, int) and isinstance(expected, int):
        return produced == expected
    if isinstance(produced, (int, float)) and isinstance(expected, (int, float)):
        return abs(produced - expected) <= TOLERANCE * max(1.0, abs(expected))
    if isinstance(produced, dict) and isinstance(expected, dict):
        return produced.keys() == expected.keys() and all(close(produced[key], expected[key]) for key in produced)
    if isinstance(produced, list) and isinstance(expected, list):
        return len(produced) == len(expected) and all(close(left, right) for left, right in zip(produced, expected))
    return produced == expected


def agree(scenario, produced, vectors):
    """⚖️ Holds every produced answer to the committed one — the vectors may never drift from the reference."""
    for vector in vectors:
        if not close(produced[vector["id"]], vector["expected"]):
            raise AssertionError("%s/%s: the reference answers %r, the committed vector says %r" % (scenario, vector["id"], produced[vector["id"]], vector["expected"]))
    return Outcome(produced)


def hashes(ctx):
    """🧮️ ``lowbias32`` of every committed word and ``mix`` folded over every committed chain."""
    golden()
    vectors = committed(ctx)["hashes"]
    return agree("hashes", {vector["id"]: int(lowbias(vector["low"])[0]) if "low" in vector else chained(vector["chain"]) for vector in vectors}, vectors)


def uniformity(ctx):
    """🎰️ The sum and the histogram of every committed run of a lane."""
    golden()
    vectors = committed(ctx)["uniformity"]
    return agree("uniformity", {vector["id"]: spread_of(vector) for vector in vectors}, vectors)


def births_and_lives(ctx):
    """👶️ Births and, tick by tick, the ages of the living of every committed emitter."""
    golden()
    vectors = committed(ctx)["births"]
    return agree("births", {vector["id"]: lifetimes(vector) for vector in vectors}, vectors)


def motions(ctx):
    """💃️ The particles of every committed emitter at its committed ticks."""
    golden()
    vectors = committed(ctx)["motions"]
    return agree("motions", {vector["id"]: flight(vector) for vector in vectors}, vectors)


def caps(ctx):
    """🧢️ Who survives every committed cap."""
    vectors = committed(ctx)["caps"]
    return agree("caps", {vector["id"]: kept(vector) for vector in vectors}, vectors)


def ends(ctx):
    """🔚️ The end of every committed emitter."""
    golden()
    vectors = committed(ctx)["ends"]
    return agree("ends", {vector["id"]: ending(vector) for vector in vectors}, vectors)


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter():
    """🔮️ Oracle role only: numpy is the reference, the TypeScript and Rust twins are judged against it."""
    return Adapter("python").oracle("hashes", hashes).oracle("uniformity", uniformity).oracle("births", births_and_lives).oracle("motions", motions).oracle("caps", caps).oracle("ends", ends)


# endregion 🔖️Registration

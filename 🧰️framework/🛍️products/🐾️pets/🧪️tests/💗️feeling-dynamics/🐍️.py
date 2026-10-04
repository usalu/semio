#!/usr/bin/env python3
"""💗️ Oracle of the pets mood dynamics (design-v2 §19, mechanics §8.2–§8.3, content §A1), in Python.

A feeling is a mood with an intensity and the tick it was stirred. It holds for two seconds, fades linearly and
gives way to the resting mood of the species, which rises back to its rest level; impulses reinforce, replace by
rank or are lost; moods travel between neighbours without ever growing on the way; the face and the wishes of a pet
are the face and the wishes of content moved towards those of its mood by the intensity.

numpy evaluates all of it on its own, written from the design text: the trajectory of a feeling over a whole array
of ticks as one closed form (``numpy.maximum``, ``numpy.minimum``, ``numpy.where``), the tick a mood fades by
``numpy.argmax`` over the sampled line (never by the division the subjects use), the tick a feeling settles by the
first sample at rest, the steps between consecutive ticks by ``numpy.diff``; the ranks as a boolean matrix whose
transitivity is a matrix product and whose tiers are ``numpy.argsort`` of its column sums; the crowd of a contagion
vectorised over everybody who catches, with the laws "nobody ends above the strongest of its mood" and "every
intensity stays in [0, 1]" asserted beat by beat; faces and weights by ``numpy.interp``; proneness by ``numpy.dot``.

@see https://numpy.org/doc/stable/reference/generated/numpy.interp.html
@see https://numpy.org/doc/stable/reference/generated/numpy.argmax.html
@see ../../🧫️fixtures/💗️feeling-dynamics/🔣️.json
"""

# region 🔖️Imports
import json

import numpy

from semio_repo_test import Adapter, Outcome

# endregion 🔖️Imports


# region 🔖️Tables
VECTORS = "shared://💗️feeling-dynamics/🔣️.json"
MOODS = ["content", "happy", "playful", "curious", "proud", "sleepy", "grumpy", "sad", "scared"]
ACTIVITIES = ["idle", "fidget", "walk", "hop", "fall", "land", "sleep", "greet", "cuddle", "squabble", "sulk", "hang", "tumble", "glide", "aim", "reel", "climb", "mantle", "slide", "carry", "trick", "purr", "dizzy", "shrug", "scoot", "push"]
TICKS_PER_SECOND = 64
PRIORITIES = [0, 3, 3, 2, 3, 1, 5, 4, 6]
TIERS = [["content"], ["sleepy"], ["curious"], ["happy", "playful", "proud"], ["sad"], ["grumpy"], ["scared"]]
HOLD = 128
FAINT = 0.05
REST = 0.25
RISE = 0.03125
OVERRIDE = 1.2
DECAYS = [0.03125, 0.015625, 0.03125, 0.0625, 0.03125, 0.0078125, 0.015625, 0.0078125, 0.15625]
PRONE_DECAY = 0.5
VALENCES = [0.3, 0.8, 0.6, 0.2, 0.6, 0.1, -0.5, -0.8, -0.4]
LIDS = [0.1, 0.12, 0, 0, 0.15, 0.55, 0.35, 0.25, 0]
SLANTS = [0, 0, -10, -10, 6, 4, 12, -10, -14]
DROPS = [0, -1.5, 0, -1, -1.5, 1.5, 0, 2, 2]
OCCASIONS = ["greeted", "tricked", "purred", "lifted", "dangled", "shaken", "dropped", "floated", "landed", "trampled", "welcomed", "cuddled", "squabbled", "mended", "drenched", "pestered", "evicted", "watched", "startled", "woken", "entertained", "climbed", "missed", "bonked"]
APPRAISALS = [
    ["greeted", "content", 0.2],
    ["greeted", "happy", 0.3],
    ["tricked", "playful", 0.4],
    ["purred", "content", 0.5],
    ["purred", "happy", 0.3],
    ["lifted", "scared", 0.3],
    ["dangled", "curious", 0.4],
    ["shaken", "scared", 0.5],
    ["dropped", "grumpy", 0.4],
    ["floated", "content", 1],
    ["floated", "proud", 0.4],
    ["landed", "content", 0.5],
    ["landed", "happy", 0.3],
    ["trampled", "grumpy", 0.3],
    ["welcomed", "happy", 0.3],
    ["cuddled", "content", 1],
    ["cuddled", "happy", 0.5],
    ["squabbled", "grumpy", 0.5],
    ["mended", "content", 0.3],
    ["drenched", "sad", 0.4],
    ["pestered", "grumpy", 0.4],
    ["evicted", "scared", 0.4],
    ["watched", "curious", 0.4],
    ["startled", "scared", 0.4],
    ["woken", "content", 1],
    ["entertained", "happy", 0.3],
    ["climbed", "content", 0.3],
    ["climbed", "proud", 0.4],
    ["missed", "grumpy", 0.3],
    ["bonked", "grumpy", 0.3],
]
PRONENESS = [[1, 0, 0, 0], [0.5, 0, 1, 0], [0.5, 1, 0, 0], [0.5, 0, 0, 1], [1, 0, 0, 0], [1.5, -1, 0, 0], [1.5, 0, -1, 0], [0.5, 0, 1, 0], [1, 0, 0, 0]]
PRONE_GAIN = 1.5
TRICK_AMOUNT = 0.4
DROWSY_ENERGY = 0.6
WEIGHTS = [
    [1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1],
    [1, 1.5, 1.25, 1.25, 1, 1, 0.5, 1, 1, 1, 1, 1, 1, 1, 1.25, 1, 1.25, 1, 1, 1.25, 1.5, 1, 1, 1, 1, 1.25],
    [1, 2, 1.5, 2, 1, 1, 0.25, 1, 1, 1, 1, 1, 1, 1, 1.5, 1, 1.5, 1, 1, 1.5, 3, 1, 1, 1, 1, 2],
    [1, 1, 2, 1.5, 1, 1, 0.25, 1, 1, 1, 1, 1, 1, 1, 2, 1, 2, 1, 1, 1.5, 1, 1, 1, 1, 1, 1.5],
    [1, 1.5, 1, 1, 1, 1, 0.5, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 2, 1, 1, 1, 1, 1],
    [1, 0.25, 0.5, 0.25, 1, 1, 4, 1, 1, 1, 1, 1, 1, 1, 0.25, 1, 0.25, 1, 1, 0.25, 0.25, 1, 1, 1, 1, 0.25],
    [1, 1, 1.5, 0.5, 1, 1, 0.5, 1, 1, 1, 1, 1, 1, 1, 0.5, 1, 0.5, 1, 1, 0.5, 0.25, 1, 1, 1, 1, 0.5],
    [1, 0.25, 0.5, 0.25, 1, 1, 1.5, 1, 1, 1, 1, 1, 1, 1, 0.5, 1, 0.5, 1, 1, 0.5, 0.25, 1, 1, 1, 1, 0.25],
    [1, 0, 0.25, 0.25, 1, 1, 0, 1, 1, 1, 1, 1, 1, 1, 0, 1, 0, 1, 1, 0, 0, 1, 1, 1, 1, 0],
]
AFFINITIES = [0, 0.15, 0.1, 0.05, 0.05, -0.05, -0.25, -0.1, -0.05]
SHARES = [[1, 1, 1], [1.25, 1.25, 1], [1.25, 1.25, 1], [1.25, 1, 1], [1.5, 1, 1], [1, 1, 0.5], [0.5, 0.5, 1.5], [1, 2, 0.5], [1, 2, 0.25]]
SPREADS = [0, 0.5, 0.6, 0.4, 0, 0.8, 0.2, 0, 0.3]
CONTAGION_BEAT = 32
CONTAGION_REACH = 3
TABLES = {
    "priorities": PRIORITIES,
    "hold": HOLD,
    "faint": FAINT,
    "rest": REST,
    "rise": RISE,
    "override": OVERRIDE,
    "decays": DECAYS,
    "proneDecay": PRONE_DECAY,
    "valences": VALENCES,
    "lids": LIDS,
    "slants": SLANTS,
    "drops": DROPS,
    "occasions": OCCASIONS,
    "appraisals": [{"occasion": row[0], "mood": row[1], "amount": row[2]} for row in APPRAISALS],
    "proneness": PRONENESS,
    "proneGain": PRONE_GAIN,
    "trickAmount": TRICK_AMOUNT,
    "drowsyEnergy": DROWSY_ENERGY,
    "weights": WEIGHTS,
    "affinities": AFFINITIES,
    "shares": SHARES,
    "spreads": SPREADS,
    "contagionBeat": CONTAGION_BEAT,
    "contagionReach": CONTAGION_REACH,
}

# endregion 🔖️Tables


# region 🔖️Reference
def felt(mood, intensity, since):
    """🫀️ One feeling as the vectors carry it."""
    return {"mood": mood, "intensity": float(intensity), "since": int(since)}


def fade_tick(feeling):
    """🕯️ The first tick at which a mood that is not the resting one is no longer felt: found on the sampled line of what is left after the hold, never by solving for it."""
    if feeling["intensity"] < FAINT:
        return int(feeling["since"])
    rate = DECAYS[MOODS.index(feeling["mood"])]
    waits = numpy.arange(1, int(numpy.ceil(TICKS_PER_SECOND / rate)) + 3)
    left = feeling["intensity"] - (rate * waits) / TICKS_PER_SECOND
    return int(feeling["since"] + HOLD + waits[numpy.argmax(left < FAINT)])


def course(feeling, resting, ticks):
    """📈️ The feeling at every tick of an array, as one closed form: ``(own, intensities, sinces)`` — whether it is still in its own mood (the resting one otherwise), how strongly, and the anchor before a feeling at rest is given the tick it came to rest."""
    intensity = numpy.float64(feeling["intensity"])
    since = int(feeling["since"])
    rate = DECAYS[MOODS.index(feeling["mood"])]
    if feeling["mood"] == resting:
        if intensity > REST:
            wait = ticks - since - HOLD
            left = intensity - (rate * PRONE_DECAY * numpy.maximum(wait, 0)) / TICKS_PER_SECOND
            return numpy.ones(len(ticks), dtype=bool), numpy.maximum(left, REST), numpy.where(wait <= 0, since, ticks - HOLD)
        span = ticks - since
        still = (span <= 0) | (intensity == REST)
        risen = numpy.minimum(intensity + (RISE * numpy.maximum(span, 0)) / TICKS_PER_SECOND, REST)
        return numpy.ones(len(ticks), dtype=bool), numpy.where(still, intensity, risen), numpy.where(still, since, ticks)
    wait = ticks - since - HOLD
    left = intensity - (rate * numpy.maximum(wait, 0)) / TICKS_PER_SECOND
    alive = left >= FAINT
    risen = numpy.clip((RISE * (ticks - fade_tick(feeling))) / TICKS_PER_SECOND, 0, REST)
    return alive, numpy.where(alive, left, risen), numpy.where(alive, numpy.where(wait > 0, ticks - HOLD, since), ticks)


def settles(feeling, resting):
    """🏁️ The tick from which the feeling is the resting mood at its rest level: the first sample of its course that is, over a span longer than any mood lasts."""
    start = int(feeling["since"])
    ticks = numpy.arange(start, start + HOLD + 2 * 8192 + 512 + 8)
    own, intensities, _ = course(feeling, resting, ticks)
    rested = (intensities == REST) & (own if feeling["mood"] == resting else ~own)
    if not rested.any():
        raise AssertionError("a feeling never came to rest: %r" % feeling)
    return int(ticks[numpy.argmax(rested)])


def trajectory(feeling, resting, ticks):
    """🍂️ The feeling at every tick of an array: ``(moods, intensities, sinces)``; from the tick it came to rest it is anchored there."""
    ticks = numpy.asarray(ticks, dtype=numpy.int64)
    own, intensities, sinces = course(feeling, resting, ticks)
    rested = settles(feeling, resting)
    return [feeling["mood"] if lasting else resting for lasting in own], intensities, numpy.where(ticks >= rested, rested, sinces)


def settle(feeling, resting, tick):
    """🍁️ The feeling as it stands at one tick."""
    moods, intensities, sinces = trajectory(feeling, resting, [tick])
    return felt(moods[0], intensities[0], sinces[0])


def rank(mood):
    """🏅️ The rank of a mood."""
    return PRIORITIES[MOODS.index(mood)]


def impulse(feeling, mood, amount, tick):
    """⚡️ The feeling after an impulse: reinforced, soothed, replaced by rank, or left as it is."""
    if not amount > 0:
        return feeling
    if mood == feeling["mood"]:
        return felt(mood, numpy.minimum(numpy.float64(feeling["intensity"]) + amount, 1), tick)
    if mood == "content":
        if not VALENCES[MOODS.index(feeling["mood"])] < VALENCES[0]:
            return feeling
        eased = numpy.float64(feeling["intensity"]) - amount
        return felt(feeling["mood"], eased, feeling["since"]) if eased >= FAINT else felt(feeling["mood"], 0, tick)
    if amount < FAINT:
        return feeling
    fresh = felt(mood, numpy.minimum(amount, 1), tick)
    if feeling["intensity"] <= REST or rank(mood) > rank(feeling["mood"]):
        return fresh
    if tick - feeling["since"] < HOLD:
        return feeling
    return fresh if rank(mood) == rank(feeling["mood"]) or amount > OVERRIDE * feeling["intensity"] else feeling


def proneness(character, mood):
    """🎚️ How strongly a species takes an impulse of a mood: the dot product of its row with ``[1, energy, sociability, curiosity]``, half again for its resting mood."""
    temperament = character["temperament"]
    lean = float(numpy.dot(PRONENESS[MOODS.index(mood)], [1, temperament["energy"], temperament["sociability"], temperament["curiosity"]]))
    return lean * PRONE_GAIN if character["mood"] == mood else lean


def stir(feeling, mood, amount, character, tick):
    """🌊️ An impulse as a species takes it."""
    return impulse(feeling, mood, float(numpy.clip(amount * proneness(character, mood), 0, 1)), tick)


def appraise(feeling, occasion, character, tick):
    """🧐️ The feeling after an occasion: every impulse of its rows, in table order."""
    for row in APPRAISALS:
        if row[0] == occasion:
            feeling = stir(feeling, row[1], row[2], character, tick)
    return feeling


def perform(feeling, mood, character, tick):
    """🎉️ The feeling after a trick that leaves a mood, or none (``None``: the ``tricked`` row)."""
    return appraise(feeling, "tricked", character, tick) if mood is None else stir(feeling, mood, TRICK_AMOUNT, character, tick)


def drowse(feeling, energy, tick):
    """💤️ Sleepy follows the energy need."""
    tired = float(numpy.clip((DROWSY_ENERGY - energy) / DROWSY_ENERGY, 0, 1))
    present = feeling["intensity"] if feeling["mood"] == "sleepy" else 0
    return impulse(feeling, "sleepy", tired - present, tick) if tired > present else feeling


def face(feeling):
    """🎭️ The face of a feeling: ``numpy.interp`` from the face of content to the face of the mood."""
    index = MOODS.index(feeling["mood"])
    return {name: float(numpy.interp(feeling["intensity"], [0, 1], [table[0], table[index]])) for name, table in (("bend", VALENCES), ("lid", LIDS), ("slant", SLANTS), ("drop", DROPS))}


def weights(mood, intensity):
    """🏋️ The multipliers of a mood on the weights of the activities: ``numpy.interp`` from 1 to its row."""
    return [float(numpy.interp(intensity, [0, 1], [1, full])) for full in WEIGHTS[MOODS.index(mood)]]


def bias(first, second):
    """🤝️ How an encounter of two feelings leans."""
    one, other = MOODS.index(first["mood"]), MOODS.index(second["mood"])
    shares =[float(numpy.interp(first["intensity"], [0, 1], [1, SHARES[one][index]]) * numpy.interp(second["intensity"], [0, 1], [1, SHARES[other][index]])) for index in range(3)]
    vain = first["mood"] == "proud" and first["intensity"] > REST
    boastful = second["mood"] == "proud" and second["intensity"] > REST
    show = 0 if vain and (not boastful or first["intensity"] >= second["intensity"]) else 1 if boastful else -1
    return {"affinity": float(numpy.mean([AFFINITIES[one] * first["intensity"], AFFINITIES[other] * second["intensity"]])), "shares": shares, "show": show}


def crowd(vector):
    """🦠️ A crowd through its beats: before every beat everybody is settled, then everybody catches from each neighbour in order — vectorised over those who catch, each neighbour as it stood when the beat began. Asserts that every intensity stays in [0, 1] and that nobody ends a beat above the strongest of its mood at the beginning of the beat."""
    restings = vector["restings"]
    sociability = numpy.asarray(vector["sociabilities"], dtype=numpy.float64)
    affinity = numpy.asarray(vector["affinities"], dtype=numpy.float64)
    near = numpy.asarray(vector["near"], dtype=bool)
    feelings = [dict(feeling) for feeling in vector["feelings"]]
    count = len(feelings)
    beats = []
    for beat in range(1, vector["beats"] + 1):
        tick = beat * CONTAGION_BEAT
        present = [settle(feeling, resting, tick) for feeling, resting in zip(feelings, restings)]
        before_mood = numpy.array([MOODS.index(feeling["mood"]) for feeling in present])
        before = numpy.array([feeling["intensity"] for feeling in present], dtype=numpy.float64)
        mood = before_mood.copy()
        intensity = before.copy()
        since = numpy.array([feeling["since"] for feeling in present], dtype=numpy.int64)
        for giver in range(count):
            if not before[giver] > REST:
                continue
            gain = SPREADS[before_mood[giver]] * sociability * (0.5 + 0.5 * affinity[giver])
            reached = near[giver] & (numpy.arange(count) != giver) & (gain > 0)
            same = reached & (mood == before_mood[giver])
            lift = gain * (before[giver] - intensity)
            intensity = numpy.where(same & (lift > 0), intensity + lift, intensity)
            adopted = gain * before[giver]
            adopting = reached & (mood != before_mood[giver]) & (intensity <= REST) & (adopted >= FAINT)
            mood = numpy.where(adopting, before_mood[giver], mood)
            intensity = numpy.where(adopting, adopted, intensity)
            since = numpy.where(adopting, tick, since)
        if intensity.min() < 0 or intensity.max() > 1:
            raise AssertionError("contagion/%s: an intensity left [0, 1] in beat %d" % (vector["id"], beat))
        for index in range(len(MOODS)):
            after = intensity[mood == index]
            strongest = before[before_mood == index].max() if (before_mood == index).any() else 0.0
            if after.size > 0 and after.max() > strongest + 1e-12:
                raise AssertionError("contagion/%s: %s grew on the way in beat %d" % (vector["id"], MOODS[index], beat))
        feelings = [felt(MOODS[int(mood[index])], intensity[index], since[index]) for index in range(count)]
        beats.append(feelings)
    return beats


def contest():
    """🥇️ Which impulse replaces which strong mood: ``during`` its hold (only a higher rank does) and ``after`` it (the same rank does too). Asserts with numpy that outranking is a strict order in tiers — transitive, never mutual — and that the tiers are those of the design."""
    ranks = numpy.asarray(PRIORITIES)
    stirring = numpy.array([mood != "content" for mood in MOODS])
    during = (ranks[None, :] > ranks[:, None]) & stirring[None, :]
    after = (ranks[None, :] >= ranks[:, None]) & stirring[None, :] & ~numpy.eye(len(MOODS), dtype=bool)
    beats = ranks[None, :] > ranks[:, None]
    if (beats & beats.T).any() or ((beats.astype(int) @ beats.astype(int) > 0) & ~beats).any():
        raise AssertionError("priorities: outranking is not a strict order")
    order = numpy.argsort(beats.sum(axis=0), kind="stable")
    tiers = []
    for index in order:
        if tiers and ranks[index] == ranks[MOODS.index(tiers[-1][0])]:
            tiers[-1].append(MOODS[index])
        else:
            tiers.append([MOODS[index]])
    if tiers != TIERS:
        raise AssertionError("priorities: the tiers are %r, the design says %r" % (tiers, TIERS))
    return {"during": during.astype(int).tolist(), "after": after.astype(int).tolist()}


# endregion 🔖️Reference


# region 🔖️Handlers
def committed(ctx):
    """🧫️ The committed vectors."""
    return json.loads(ctx.fixture_bytes(VECTORS))


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


def tables(ctx):
    """📋️ Every table and constant of the module."""
    return agree("tables", {vector["id"]: TABLES for vector in committed(ctx)["tables"]}, committed(ctx)["tables"])


def priorities(ctx):
    """🏆️ Which impulse replaces which mood, during and after its hold."""
    return agree("priorities", {vector["id"]: contest() for vector in committed(ctx)["priorities"]}, committed(ctx)["priorities"])


def story(vector):
    """📖️ A feeling through its events: settled at the tick of each, then stirred by its impulse."""
    feeling = vector["feeling"]
    feelings = []
    for event in vector["events"]:
        feeling = impulse(settle(feeling, vector["resting"], event["tick"]), event["mood"], event["amount"], event["tick"])
        feelings.append(feeling)
    return feelings


def impulses(ctx):
    """📚️ Every committed story of impulses."""
    vectors = committed(ctx)["impulses"]
    return agree("impulses", {vector["id"]: story(vector) for vector in vectors}, vectors)


def decay(vector):
    """📉️ A feeling at every committed tick and the tick it settles. Asserts that between consecutive ticks a mood only ever holds, loses its rate, or rises by the rate of the rest."""
    moods, intensities, sinces = trajectory(vector["feeling"], vector["resting"], vector["ticks"])
    start = int(vector["feeling"]["since"])
    run = numpy.arange(start, start + 2 * HOLD + 1024)
    _, line, _ = trajectory(vector["feeling"], vector["resting"], run)
    steps = numpy.diff(line) * TICKS_PER_SECOND
    rate = DECAYS[MOODS.index(vector["feeling"]["mood"])] * (PRONE_DECAY if vector["feeling"]["mood"] == vector["resting"] else 1)
    smooth = numpy.isclose(steps, 0, atol=1e-12) | numpy.isclose(steps, -rate, atol=1e-12) | numpy.isclose(steps, RISE, atol=1e-12)
    if (~smooth).sum() > 2:
        raise AssertionError("decays/%s: the trajectory is not piecewise linear at its rates" % vector["id"])
    return {"views": [felt(moods[index], intensities[index], sinces[index]) for index in range(len(vector["ticks"]))], "settles": settles(vector["feeling"], vector["resting"])}


def decays(ctx):
    """🌇️ Every committed feeling over its ticks."""
    vectors = committed(ctx)["decays"]
    return agree("decays", {vector["id"]: decay(vector) for vector in vectors}, vectors)


def jump(vector):
    """🦘️ A feeling settled through every cut in turn and in one step; the two must be the same feeling."""
    stepped = vector["feeling"]
    for cut in vector["cuts"]:
        stepped = settle(stepped, vector["resting"], cut)
    direct = settle(vector["feeling"], vector["resting"], vector["cuts"][-1])
    if stepped != direct:
        raise AssertionError("jumps/%s: settling in steps gives %r, in one step %r" % (vector["id"], stepped, direct))
    if settle(direct, vector["resting"], vector["cuts"][-1]) != direct:
        raise AssertionError("jumps/%s: settling twice at one tick changes the feeling" % vector["id"])
    return {"stepped": stepped, "direct": direct}


def jumps(ctx):
    """⏭️ However time is cut, the same feeling comes out."""
    vectors = committed(ctx)["jumps"]
    return agree("jumps", {vector["id"]: jump(vector) for vector in vectors}, vectors)


def contagion(ctx):
    """🤧️ Every committed crowd through its beats."""
    vectors = committed(ctx)["contagion"]
    return agree("contagion", {vector["id"]: crowd(vector) for vector in vectors}, vectors)


def faces(ctx):
    """🖼️ The valence, the spirits and the face of every committed feeling."""
    vectors = committed(ctx)["faces"]
    return agree("faces", {vector["id"]: {"valence": VALENCES[MOODS.index(vector["feeling"]["mood"])], "spirits": face(vector["feeling"])["bend"], "face": face(vector["feeling"])} for vector in vectors}, vectors)


def appraisals(ctx):
    """🧾️ The proneness of every committed character and its feeling after every committed occasion, trick and look at its energy."""
    vectors = committed(ctx)["appraisals"]
    produced = {}
    for vector in vectors:
        character = vector["character"]
        produced[vector["id"]] = {
            "proneness": [proneness(character, mood) for mood in MOODS],
            "occasions": {occasion: [appraise(feeling, occasion, character, vector["tick"]) for feeling in vector["feelings"]] for occasion in OCCASIONS},
            "tricks": {mood if mood is not None else "none": [perform(feeling, mood, character, vector["tick"]) for feeling in vector["feelings"]] for mood in [None] + MOODS},
            "drowsy": [[drowse(feeling, energy, vector["tick"]) for energy in vector["energies"]] for feeling in vector["feelings"]],
        }
    return agree("appraisals", produced, vectors)


def wishes(ctx):
    """🎛️ The multipliers of every mood at every committed intensity."""
    vectors = committed(ctx)["wishes"]
    return agree("wishes", {vector["id"]: {mood: weights(mood, vector["intensity"]) for mood in MOODS} for vector in vectors}, vectors)


def leanings(ctx):
    """💑️ How the encounter of every committed pair of feelings leans, and what that does to the committed shares."""
    vectors = committed(ctx)["leanings"]
    produced = {}
    for vector in vectors:
        leaning = bias(vector["first"], vector["second"])
        produced[vector["id"]] = {"leaning": leaning, "swayed": [float(numpy.multiply(vector["shares"], leaning["shares"])[index]) for index in range(3)]}
    return agree("leanings", produced, vectors)


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter():
    """🔮️ Oracle role only: numpy is the reference, the TypeScript and Rust twins are judged against it."""
    return (
        Adapter("python")
        .oracle("tables", tables)
        .oracle("priorities", priorities)
        .oracle("impulses", impulses)
        .oracle("decays", decays)
        .oracle("jumps", jumps)
        .oracle("contagion", contagion)
        .oracle("faces", faces)
        .oracle("appraisals", appraisals)
        .oracle("wishes", wishes)
        .oracle("leanings", leanings)
    )


# endregion 🔖️Registration

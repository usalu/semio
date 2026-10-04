#!/usr/bin/env python3
"""⚗️ Oracle of the pets states, tricks and chemistry (design-v2 §19, §21, content §7–§8), in Python.

A species has lasting states — some give way to the next after a while —, tricks that a cue sets off and that may
leave it in another state, and a menagerie has reactions: when an actor that matches ``when`` is within ``within``
pixels of one that matches ``near``, seen from it as ``where`` says, while the affinity of the two lies within
``affinity`` and no third actor that matches ``unless`` is that near the second, the effects of ``then`` happen, at
most once in ``every`` seconds and with the probability ``chance``. A side that names no species takes anyone.

numpy and scipy judge all of it on their own, written from the design text. The state at a tick is read off the
unrolled chain of states by ``numpy.cumsum`` and ``numpy.searchsorted`` (the subjects take whole laps off a circle
instead). A weighted pick is ``numpy.searchsorted`` in ``numpy.cumsum`` of the weights. Which state can be reached
from the resting one — by tricks, by time, by reactions — is a directed graph, decided by
``scipy.sparse.csgraph.breadth_first_order``. The matching of reactions is vectorised over all ordered pairs of
actors at once: bodies become boxes, gaps and overlaps become matrices (``numpy.maximum``, broadcasting), the traits
become masks, affinities a matrix clipped by ``numpy.clip``, the third parties a matrix summed per second actor, and
``numpy.argwhere`` lists what is due in pair order. The fold of a beat (chance, cooling, nothing applied twice) is
restated from the text.

@see https://docs.scipy.org/doc/scipy/reference/generated/scipy.sparse.csgraph.breadth_first_order.html
@see https://numpy.org/doc/stable/reference/generated/numpy.argwhere.html
@see ../../🧫️fixtures/⚗️chemistry-rules/🔣️.json
@see ../../🧫️fixtures/🧬️schema-conformance/🔣️.json
"""

# region 🔖️Imports
import json

import numpy
from scipy.sparse import csr_matrix
from scipy.sparse.csgraph import breadth_first_order

from semio_repo_test import Adapter, Outcome

# endregion 🔖️Imports


# region 🔖️Reference
VECTORS = "shared://⚗️chemistry-rules/🔣️.json"
SAMPLE = "shared://🧬️schema-conformance/🔣️.json"
MOODS = ["content", "happy", "playful", "curious", "proud", "sleepy", "grumpy", "sad", "scared"]
CUES = ["click", "circle", "countercircle", "stroke", "shake", "whim", "show"]
WHERES = ["above", "below", "beside", "any"]
WILLING = [True, True, True, True, True, False, False, False, False]
TICKS_PER_SECOND = 64
WHIM_FAVOR = 3
EFFECT_AMOUNT = 0.6
REST = 0.25
CALM = {"mood": "content", "intensity": REST, "since": 0}


def state_of(species, name):
    """🔦️ The state of a species by its id, or ``None``."""
    return next((state for state in species["states"] if state["id"] == name), None)


def lasting(state):
    """⏱️ For how many ticks a state lasts; 0 says until something changes it."""
    return 0 if "lasts" not in state else max(1, int(numpy.floor(state["lasts"] * TICKS_PER_SECOND + 0.5)))


def follower(species, state):
    """➡️ The state a lasting state gives way to: its ``then``, or the resting state when that names none of the species."""
    return state["then"] if "then" in state and state_of(species, state["then"]) is not None else species["states"][0]["id"]


def standing(species, name, since, tick):
    """🕰️ The state at a tick: the chain of states is unrolled until it has outlasted the time that passed, and the place in it is ``numpy.searchsorted`` in ``numpy.cumsum`` of how long each lasts."""
    current = state_of(species, name) or species["states"][0]
    chain = [current]
    spans = []
    while sum(spans) <= tick - since:
        span = lasting(current)
        if span == 0 or follower(species, current) == current["id"]:
            break
        spans.append(span)
        current = state_of(species, follower(species, current))
        chain.append(current)
    if len(spans) == 0:
        return {"state": chain[0]["id"], "since": int(since)}
    ends = numpy.cumsum(spans)
    passed = int(numpy.searchsorted(ends, tick - since, side="right"))
    return {"state": chain[passed]["id"], "since": int(since + (ends[passed - 1] if passed > 0 else 0))}


def ending(species, name, since):
    """🔚️ The tick at which a state gives way, or ``None``."""
    state = state_of(species, name)
    if state is None or lasting(state) == 0 or follower(species, state) == state["id"]:
        return None
    return int(since + lasting(state))


def ladder(species):
    """🪜️ The states of a species in authored order."""
    return [state["id"] for state in species["states"]]


def rungs(species, trick):
    """🧗️ The rungs a trick steps along: its ``from`` where the species has those states, the whole ladder otherwise."""
    listed = [name for name in trick.get("from", []) if state_of(species, name) is not None]
    return listed if len(listed) > 0 else ladder(species)


def step(steps, name, direction):
    """👣️ One step along rungs, held at both ends."""
    if name not in steps or direction == 0:
        return name
    index = int(numpy.clip(steps.index(name) + numpy.sign(direction), 0, len(steps) - 1))
    return steps[index]


def after(species, name, trick):
    """🎬️ The state a trick leaves a pet in."""
    if "to" in trick:
        return trick["to"] if state_of(species, trick["to"]) is not None else name
    direction = 1 if "circle" in trick["cues"] else -1 if "countercircle" in trick["cues"] else 0
    return step(rungs(species, trick), name, direction)


def shown(feeling):
    """🪞️ The mood a pet shows: its mood while that is stirred above the rest level, ``content`` while it is calm."""
    return feeling["mood"] if feeling["intensity"] > REST else "content"


def offered(species, cue, name, feeling):
    """🎪️ The tricks a cue can set off in a state and a feeling, in authored order."""
    if cue in ("whim", "show") and not WILLING[MOODS.index(shown(feeling))]:
        return []
    return [trick for trick in species["tricks"] if cue in trick["cues"] and ("from" not in trick or name in trick["from"])]


def clicked(species, name, feeling, index):
    """👆️ The trick of the n-th click, round and round."""
    tricks = offered(species, "click", name, feeling)
    return None if len(tricks) == 0 else tricks[int(numpy.floor(index)) % len(tricks)]["id"]


def picked(species, cue, name, feeling, mark):
    """🎲️ The trick a unit draw picks: ``numpy.searchsorted`` of ``unit × total`` in ``numpy.cumsum`` of the weights."""
    tricks = offered(species, cue, name, feeling)
    if len(tricks) == 0:
        return None
    sums = numpy.cumsum([WHIM_FAVOR if trick.get("mood") == shown(feeling) else 1 for trick in tricks])
    return tricks[min(int(numpy.searchsorted(sums, mark * sums[-1], side="right")), len(tricks) - 1)]["id"]


def edges(menagerie, species):
    """🕸️ Every way from one state of a species into another: by a trick on offer there, by time, by a reaction that puts the species into a state (from the state its trait asks for, or from any)."""
    names = ladder(species)
    found = set()
    for trick in species["tricks"]:
        for name in names:
            if "from" not in trick or name in trick["from"]:
                found.add((name, after(species, name, trick)))
    for name in names:
        if ending(species, name, 0) is not None:
            found.add((name, follower(species, state_of(species, name))))
    for reaction in menagerie.get("chemistry", []):
        for effect in reaction["then"]:
            trait = reaction[effect["on"]]
            if trait.get("species", species["id"]) != species["id"] or "state" not in effect or state_of(species, effect["state"]) is None:
                continue
            for name in [trait["state"]] if "state" in trait else names:
                if name in names:
                    found.add((name, effect["state"]))
    return sorted(([start, end] for start, end in found if start != end), key=lambda edge: (names.index(edge[0]), names.index(edge[1])))


def reach(menagerie, species):
    """🧭️ The edges between the states of a species and what scipy's breadth-first search reaches from the resting state, in authored order."""
    names = ladder(species)
    ways = edges(menagerie, species)
    rows = [names.index(start) for start, _ in ways]
    columns = [names.index(end) for _, end in ways]
    graph = csr_matrix((numpy.ones(len(ways), dtype=numpy.int8), (rows, columns)), shape=(len(names), len(names)))
    order = breadth_first_order(graph, 0, directed=True, return_predecessors=False)
    reached = sorted(int(node) for node in order)
    return {"edges": ways, "reachable": [names[index] for index in reached], "unreachable": [name for index, name in enumerate(names) if index not in reached]}


def geometry(sightings):
    """📐️ The bodies of a stage as boxes and what every ordered pair is to each other, as matrices ``[first, second]``: the squared gap between the boxes, and whether the first is above, below or beside the second."""
    x = numpy.array([sighting["x"] for sighting in sightings], dtype=numpy.float64)
    y = numpy.array([sighting["y"] for sighting in sightings], dtype=numpy.float64)
    width = numpy.array([sighting["width"] for sighting in sightings], dtype=numpy.float64)
    height = numpy.array([sighting["height"] for sighting in sightings], dtype=numpy.float64)
    left, right, top, bottom = x - width / 2, x + width / 2, y - height, y
    across = numpy.maximum(0, numpy.maximum(left[:, None] - right[None, :], left[None, :] - right[:, None]))
    upright = numpy.maximum(0, numpy.maximum(top[:, None] - bottom[None, :], top[None, :] - bottom[:, None]))
    column = (left[:, None] < right[None, :]) & (left[None, :] < right[:, None])
    row = (top[:, None] < bottom[None, :]) & (top[None, :] < bottom[:, None])
    middle = y - height / 2
    return {
        "gap": across * across + upright * upright,
        "above": column & (middle[:, None] < middle[None, :]),
        "below": column & (middle[:, None] > middle[None, :]),
        "beside": row & ~column,
        "any": numpy.ones((len(sightings), len(sightings)), dtype=bool),
    }


def held_ticks(held):
    """⏲️ For how many ticks a trait asks a state to have been held: ``floor(held × 64 + 0.5)``, at least 1."""
    return max(1, int(numpy.floor(held * TICKS_PER_SECOND + 0.5)))


def trait_mask(trait, sightings):
    """🔎️ Which actors are what one side of a reaction asks for: the species (anyone when it names none), the state held long enough, the activity, the trick and the mood an actor shows."""
    return numpy.array(
        [
            ("species" not in trait or trait["species"] == sighting["species"])
            and all(key not in trait or trait[key] == sighting[key] for key in ("state", "activity", "trick"))
            and ("held" not in trait or sighting["held"] >= held_ticks(trait["held"]))
            and ("mood" not in trait or trait["mood"] == shown(sighting))
            for sighting in sightings
        ],
        dtype=bool,
    )


def every(reaction):
    """🔁️ For how many ticks a reaction cools after its turn."""
    return max(1, int(numpy.floor(reaction["every"] * TICKS_PER_SECOND + 0.5)))


def affinities(menagerie, arranged, rapports):
    """💞️ The affinity of every ordered pair of actors as a matrix: the authored bond of their species (0 when unlisted) plus the drift of their rapport (0 when none), held inside ``[−0.6, 1]`` by ``numpy.clip``; 0 for a species and itself."""
    def lookup(entries, key, first, second):
        return next((entry[key] for entry in entries if sorted(entry["between"]) == sorted([first, second])), 0)

    bonds = numpy.array([[lookup(menagerie.get("bonds", []), "affinity", one["species"], other["species"]) for other in arranged] for one in arranged], dtype=numpy.float64)
    drifts = numpy.array([[lookup(rapports, "drift", one["species"], other["species"]) for other in arranged] for one in arranged], dtype=numpy.float64)
    same = numpy.array([[one["species"] == other["species"] for other in arranged] for one in arranged], dtype=bool)
    return numpy.where(same, 0.0, numpy.clip(bonds + drifts, -0.6, 1))


def trials(menagerie, sightings, coolings, tick, rapports):
    """🧫️ The reactions that are due, in pair order: one boolean array over ``[first, second, reaction]`` with the actors arranged by the place of their species in the menagerie, listed by ``numpy.argwhere``. An ``affinity`` is a range test on the matrix of affinities; ``unless`` counts, per second actor, the thirds near it that match, by a matrix sum without the pair itself."""
    names = [species["id"] for species in menagerie["species"]]
    known = [index for index, sighting in enumerate(sightings) if sighting["species"] in names]
    order = [known[index] for index in numpy.argsort([names.index(sightings[index]["species"]) for index in known], kind="stable")] if len(known) > 0 else []
    chemistry = menagerie.get("chemistry", [])
    if len(order) < 2 or len(chemistry) == 0:
        return []
    arranged = [sightings[index] for index in order]
    shapes = geometry(arranged)
    bound = affinities(menagerie, arranged, rapports)
    count = len(arranged)
    due = numpy.zeros((count, count, len(chemistry)), dtype=bool)
    for index, reaction in enumerate(chemistry):
        reach = reaction["within"] * reaction["within"]
        mask = trait_mask(reaction["when"], arranged)[:, None] & trait_mask(reaction["near"], arranged)[None, :]
        mask &= shapes["gap"] <= reach
        mask &= shapes[reaction.get("where", "any")]
        mask &= ~numpy.eye(count, dtype=bool)
        if "affinity" in reaction:
            mask &= (bound >= reaction["affinity"][0]) & (bound <= reaction["affinity"][1])
        if "unless" in reaction:
            third = trait_mask(reaction["unless"], arranged)[:, None] & (shapes["gap"] <= reach)
            thirds = third.sum(axis=0)[None, :] - third.astype(int) - numpy.diag(third).astype(int)[None, :]
            mask &= thirds <= 0
        for cooling in coolings:
            if cooling["reaction"] == reaction["id"] and cooling["until"] > tick:
                mask &= ~(numpy.array([sighting["species"] == cooling["when"] for sighting in arranged])[:, None] & numpy.array([sighting["species"] == cooling["near"] for sighting in arranged])[None, :])
        due[:, :, index] = mask
    return [{"reaction": int(index), "when": int(order[first]), "near": int(order[second]), "chancy": "chance" in chemistry[index]} for first, second, index in numpy.argwhere(due)]


def react(menagerie, sightings, tick, coolings, units, rapports):
    """🔥️ One beat of chemistry: every due reaction has its turn and cools; the effects of those that happen become consequences, everybody judged as it stood, nothing applied twice."""
    species = {entry["id"]: entry for entry in menagerie["species"]}
    kept = [cooling for cooling in coolings if cooling["until"] > tick]
    consequences = []
    stated, moved, tricked, started, shifted, promised = set(), set(), set(), set(), set(), set()
    drawn = 0
    for trial in trials(menagerie, sightings, coolings, tick, rapports):
        reaction = menagerie["chemistry"][trial["reaction"]]
        first, second = sightings[trial["when"]], sightings[trial["near"]]
        kept.append({"reaction": reaction["id"], "when": first["species"], "near": second["species"], "until": int(tick + every(reaction))})
        if "chance" in reaction:
            mark = units[drawn] if drawn < len(units) else 1
            drawn += 1
            if not mark < reaction["chance"]:
                continue
        for effect in reaction["then"]:
            actor, other = (first, second) if effect["on"] == "when" else (second, first)
            kind = species[actor["species"]]
            pair = frozenset((actor["species"], other["species"]))
            consequence = {"reaction": reaction["id"], "on": actor["species"], "other": other["species"], "state": None, "mood": None, "amount": 0, "rapport": 0, "encounter": None, "trick": None, "activity": None}
            if "state" in effect and state_of(kind, effect["state"]) is not None and actor["species"] not in stated:
                consequence["state"] = effect["state"]
                stated.add(actor["species"])
            if "mood" in effect and actor["species"] not in moved:
                consequence["mood"] = effect["mood"]
                consequence["amount"] = effect.get("amount", EFFECT_AMOUNT)
                moved.add(actor["species"])
            if effect.get("rapport", 0) != 0 and pair not in shifted:
                consequence["rapport"] = effect["rapport"]
                shifted.add(pair)
            if "encounter" in effect and pair not in promised:
                consequence["encounter"] = effect["encounter"]
                promised.add(pair)
            if "trick" in effect and actor["species"] not in tricked and any(trick["id"] == effect["trick"] and ("from" not in trick or actor["state"] in trick["from"]) for trick in kind["tricks"]):
                consequence["trick"] = effect["trick"]
                tricked.add(actor["species"])
            if "activity" in effect and actor["species"] not in started:
                consequence["activity"] = effect["activity"]
                started.add(actor["species"])
            if any(consequence[key] is not None for key in ("state", "mood", "encounter", "trick", "activity")) or consequence["rapport"] != 0:
                consequences.append(consequence)
    return {"consequences": consequences, "coolings": kept, "drawn": drawn}


def cast(reaction, names):
    """🎟️ The species that stand for the two sides of a reaction in a rehearsal: the one a side names, else the first of ``names`` that is not the other side's."""
    when = reaction["when"].get("species", next((name for name in names if name != reaction["near"].get("species")), None))
    near = reaction["near"].get("species", next((name for name in names if name != when), None))
    return when, near


def staged(menagerie, reaction, first_species, second_species):
    """🧍️ Two actors of the given species that stand as a reaction asks: each in the state its trait names (its resting state otherwise), held as long as it asks, in the mood, activity and trick it names (its resting mood, ``idle`` and no trick otherwise — performing when it names a trick), the first above, below or beside the second with half the reach between their bodies."""
    species = {entry["id"]: entry for entry in menagerie["species"]}

    def actor(trait, name):
        kind = species[name]
        return {
            "species": kind["id"],
            "state": trait.get("state", kind["states"][0]["id"]),
            "held": held_ticks(trait["held"]) if "held" in trait else 0,
            "mood": trait.get("mood", kind["mood"]),
            "intensity": 0.5,
            "activity": trait.get("activity", "trick" if "trick" in trait else "idle"),
            "trick": trait.get("trick"),
            "x": 400,
            "y": 400,
            "width": kind["size"]["width"],
            "height": kind["size"]["height"],
        }

    first, second = actor(reaction["when"], first_species), actor(reaction["near"], second_species)
    gap = reaction["within"] / 2
    where = reaction.get("where", "any")
    if where == "above":
        first["y"] = second["y"] - second["height"] - gap
    elif where == "below":
        first["y"] = second["y"] + first["height"] + gap
    else:
        first["x"] = second["x"] + (first["width"] + second["width"]) / 2 + gap
    return [first, second]


def rehearsal(menagerie):
    """🎭️ What a whole menagerie says about itself: per species its ladder and what can be reached, per reaction what happens when two of its species stand as it asks (a side that names no species is played by the first species the other side is not) with their authored bond and every draw lucky."""
    kinds = [species for species in menagerie.get("species", []) if len(species.get("states", [])) > 0 and "tricks" in species and "mood" in species]
    names = [species["id"] for species in kinds]
    plays = {}
    for reaction in menagerie.get("chemistry", []):
        first, second = cast(reaction, names)
        if first not in names or second not in names or first == second:
            continue
        sightings = staged(menagerie, reaction, first, second)
        plays[reaction["id"]] = {"due": [menagerie["chemistry"][trial["reaction"]]["id"] for trial in trials(menagerie, sightings, [], 0, [])], "beat": react(menagerie, sightings, 0, [], [0] * len(menagerie["chemistry"]), [])}
    return {"species": {species["id"]: {"ladder": ladder(species), **reach(menagerie, species)} for species in kinds}, "reactions": plays}


# endregion 🔖️Reference


# region 🔖️Handlers
def committed(ctx):
    """📥️ The committed vectors."""
    return json.loads(ctx.fixture_bytes(VECTORS))


def kind_of(document, name):
    """🧬️ One species of the menagerie the vectors carry."""
    return next(species for species in document["menagerie"]["species"] if species["id"] == name)


def close(produced, expected):
    """📏️ Whether two answers agree: numbers within 1e-9, everything else exactly."""
    if isinstance(produced, bool) or isinstance(expected, bool):
        return produced is expected
    if isinstance(produced, (int, float)) and isinstance(expected, (int, float)):
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


def relation(vector):
    """🔭️ What the first of two bodies is to the second: within each committed reach or not, and above, below, beside."""
    shapes = geometry([vector["first"], vector["second"]])
    return {"nearby": [bool(shapes["gap"][0, 1] <= reach_ * reach_) for reach_ in vector["reaches"]], **{where: bool(shapes[where][0, 1]) for where in WHERES}}


def relations(ctx):
    """🗺️ Every committed pair of bodies."""
    vectors = committed(ctx)["relations"]
    return agree("relations", {vector["id"]: relation(vector) for vector in vectors}, vectors)


def states(ctx):
    """⌛️ The state of every committed species at every committed tick, when it gives way and how long it lasts."""
    document = committed(ctx)
    produced = {}
    for vector in document["states"]:
        species = kind_of(document, vector["species"])
        state = state_of(species, vector["state"])
        produced[vector["id"]] = {"standings": [standing(species, vector["state"], vector["since"], tick) for tick in vector["ticks"]], "ends": ending(species, vector["state"], vector["since"]), "lasts": 0 if state is None else lasting(state)}
    return agree("states", produced, document["states"])


def ladders(ctx):
    """🧱️ The ladder of every committed species, the rungs of every trick, every step up and down and the state every trick leaves."""
    document = committed(ctx)
    produced = {}
    for vector in document["ladders"]:
        species = kind_of(document, vector["id"])
        names = ladder(species)
        produced[vector["id"]] = {
            "ladder": names,
            "rungs": {trick["id"]: rungs(species, trick) for trick in species["tricks"]},
            "steps": {name: {"up": step(names, name, 1), "down": step(names, name, -1), "stay": step(names, name, 0)} for name in names + ["nowhere"]},
            "after": {trick["id"]: {name: after(species, name, trick) for name in names} for trick in species["tricks"]},
        }
    return agree("ladders", produced, document["ladders"])


def tricks(ctx):
    """🃏️ The tricks on offer per cue, state and feeling, the trick of every committed click and of every committed draw."""
    document = committed(ctx)
    produced = {}
    for vector in document["tricks"]:
        species = kind_of(document, vector["id"])
        names = ladder(species)
        produced[vector["id"]] = {
            "offers": {cue: {name: {feeling["id"]: [trick["id"] for trick in offered(species, cue, name, feeling)] for feeling in vector["feelings"]} for name in names} for cue in CUES},
            "clicks": {name: [clicked(species, name, CALM, index) for index in vector["clicks"]] for name in names},
            "whims": {name: {feeling["id"]: [picked(species, "whim", name, feeling, mark) for mark in vector["units"]] for feeling in vector["feelings"]} for name in names},
            "shows": {name: {feeling["id"]: [picked(species, "show", name, feeling, mark) for mark in vector["units"]] for feeling in vector["feelings"]} for name in names},
        }
    return agree("tricks", produced, document["tricks"])


def reachability(ctx):
    """🗝️ What every committed species can reach from its resting state; a state that cannot be reached is an error."""
    document = committed(ctx)
    produced = {}
    for vector in document["reachability"]:
        produced[vector["id"]] = reach(document["menagerie"], kind_of(document, vector["id"]))
        if len(produced[vector["id"]]["unreachable"]) > 0:
            raise AssertionError("reachability/%s: nothing leads to %r" % (vector["id"], produced[vector["id"]]["unreachable"]))
    return agree("reachability", produced, document["reachability"])


def matching(ctx):
    """🧩️ The reactions that are due on every committed stage."""
    document = committed(ctx)
    return agree("matching", {vector["id"]: trials(document["menagerie"], vector["sightings"], vector["coolings"], vector["tick"], vector["rapports"]) for vector in document["matching"]}, document["matching"])


def reactions(ctx):
    """🎞️ Every committed story beat by beat, the coolings carried from one beat to the next."""
    document = committed(ctx)
    produced = {}
    for vector in document["reactions"]:
        coolings = []
        beats = []
        for beat in vector["beats"]:
            outcome = react(document["menagerie"], beat["sightings"], beat["tick"], coolings, beat["units"], beat["rapports"])
            coolings = outcome["coolings"]
            beats.append(outcome)
        produced[vector["id"]] = beats
    return agree("reactions", produced, document["reactions"])


def sample(ctx):
    """🧸️ The sample menagerie of the product judged as it is committed right now: ladders, reachability and every reaction staged."""
    return Outcome({"sample": rehearsal(json.loads(ctx.fixture_bytes(SAMPLE))["menagerie"])})


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter():
    """🔮️ Oracle role only: numpy and scipy are the reference, the TypeScript and Rust twins are judged against them."""
    return (
        Adapter("python")
        .oracle("relations", relations)
        .oracle("states", states)
        .oracle("ladders", ladders)
        .oracle("tricks", tricks)
        .oracle("reachability", reachability)
        .oracle("matching", matching)
        .oracle("reactions", reactions)
        .oracle("sample", sample)
    )


# endregion 🔖️Registration

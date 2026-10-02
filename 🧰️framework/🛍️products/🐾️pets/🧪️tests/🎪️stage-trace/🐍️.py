#!/usr/bin/env python3
"""🎪️ Keeper of the committed stage traces (design §5, §10), in Python.

No third-party library simulates the pets stage, and a second engine written to be compared with it would be the
same model again (recorded no-oracle decision ``pets-stage-trace``). This file therefore simulates nothing: it reads
the COMMITTED trace of every script and checks what can be checked on it without the stage — that every recorded
actor stands on a surveyed surface at the height of that surface (minus its hover), that the box of its body touches
no keep-out, that no two actors of one surface stand in each other, that no more actors walk or hop than the mode
allows, that at most one pair has partners, that opacities are whole and that every law flag the recording carries is
true. The digests are the subject's own; the
TypeScript and the Rust twin must reproduce them bit for bit.

The checks run when the vectors are generated (``generate_behavior_vectors.py`` refuses to write a trace that breaks
them) and whenever a harness dispatches this adapter: its handlers answer with the committed projections.

@see ../../🧫️fixtures/🎪️stage-trace/🔣️.json
@see ./🟦️.ts — the replay, the digest and the per-tick laws
"""

# region 🔖️Imports
import json

import numpy

from semio_repo_test import Adapter, Outcome

# endregion 🔖️Imports


# region 🔖️Laws
VECTORS = "shared://🎪️stage-trace/🔣️.json"
MOVERS = {"still": 0, "calm": 1, "lively": 2}
HOP_TICKS = 48
LAWS = ["perched", "clear", "apart", "paced", "paired", "graphed", "whole"]
CONTACT = 0.0078125


def hover_of(species):
    """🎈️ How high a species floats above its perch."""
    locomotion = species["locomotion"]
    return locomotion.get("hover", 0) if locomotion["gait"] == "float" else 0


def setting(script, tick):
    """🗺️ The survey in force after ``tick`` ticks and the tick of the last change of mode: events at tick ``t`` are folded before tick ``t + 1`` passes."""
    survey = None
    tuned = -HOP_TICKS - 2
    for step in script["steps"]:
        if step["at"] >= tick:
            continue
        for event in step["events"]:
            if event["kind"] == "surveyed":
                survey = event
            if event["kind"] == "tuned":
                tuned = max(tuned, step["at"])
    return survey, tuned


def overlaps(left, top, right, bottom, keepout):
    """⛔️ Whether a box and a keep-out share an area."""
    if keepout["width"] <= 0 or keepout["height"] <= 0:
        return False
    return bool(numpy.maximum(left, keepout["x"]) < numpy.minimum(right, keepout["x"] + keepout["width"]) and numpy.maximum(top, keepout["y"]) < numpy.minimum(bottom, keepout["y"] + keepout["height"]))


def verify_checkpoint(menagerie, script, checkpoint):
    """📍️ The laws one recorded checkpoint must keep."""
    where = "%s@%d" % (script["id"], checkpoint["tick"])
    kinds = {species["id"]: species for species in menagerie["species"]}
    survey, tuned = setting(script, checkpoint["tick"])
    actors = checkpoint["actors"]
    order = [kind["id"] for kind in menagerie["species"]]
    listed = [actor["species"] for actor in actors]
    if listed != [species for species in order if species in listed] or len(set(listed)) != len(listed):
        raise AssertionError("%s: actors are not listed once each in menagerie order: %r" % (where, listed))
    movers = 0
    partners = 0
    for actor in actors:
        kind = kinds[actor["species"]]
        if not 0 <= actor["opacity"] <= 1:
            raise AssertionError("%s: %s has opacity %r" % (where, actor["species"], actor["opacity"]))
        if actor["activity"] in ("walk", "hop") and not actor["leaving"]:
            movers += 1
        if actor["partner"] is not None:
            partners += 1
            if actor["partner"] not in listed:
                raise AssertionError("%s: %s has a partner that is not on stage" % (where, actor["species"]))
        if actor["perch"] is None:
            continue
        if survey is None:
            raise AssertionError("%s: %s stands on %s before anything was surveyed" % (where, actor["species"], actor["perch"]))
        surfaces = [surface for surface in survey["surfaces"] if surface["id"] == actor["perch"]]
        if len(surfaces) != 1:
            raise AssertionError("%s: %s stands on %s, which is not surveyed" % (where, actor["species"], actor["perch"]))
        surface = surfaces[0]
        half = kind["size"]["width"] / 2
        if not (max(surface["x0"], 0) + half <= actor["x"] <= min(surface["x1"], survey["width"]) - half):
            raise AssertionError("%s: %s at x=%r does not stand inside %s" % (where, actor["species"], actor["x"], actor["perch"]))
        if actor["y"] != surface["y"] - hover_of(kind):
            raise AssertionError("%s: %s at y=%r is not on %s" % (where, actor["species"], actor["y"], actor["perch"]))
        for keepout in survey["keepouts"]:
            if overlaps(actor["x"] - half, actor["y"] - kind["size"]["height"], actor["x"] + half, actor["y"], keepout):
                raise AssertionError("%s: %s stands inside a keep-out" % (where, actor["species"]))
    grounded = [actor for actor in actors if actor["perch"] is not None]
    for index, one in enumerate(grounded):
        for two in grounded[index + 1 :]:
            if one["perch"] != two["perch"]:
                continue
            shoulders = (kinds[one["species"]]["size"]["width"] + kinds[two["species"]]["size"]["width"]) / 2
            if float(numpy.abs(one["x"] - two["x"])) < shoulders - CONTACT:
                raise AssertionError("%s: %s and %s stand in each other on %s" % (where, one["species"], two["species"], one["perch"]))
    if movers > MOVERS[checkpoint["mode"]] and checkpoint["tick"] - tuned > HOP_TICKS + 1:
        raise AssertionError("%s: %d actors move in mode %s" % (where, movers, checkpoint["mode"]))
    if partners > 2:
        raise AssertionError("%s: %d actors have partners" % (where, partners))


def verify(document):
    """🧾️ Checks every committed trace; raises on the first broken law."""
    menagerie = document["menagerie"]
    seen = set()
    for script in document["scripts"]:
        if script["id"] in seen:
            raise AssertionError("duplicate script id %s" % script["id"])
        seen.add(script["id"])
        expected = script["expected"]
        if expected["frames"] != script["ticks"]:
            raise AssertionError("%s: %d frames for %d ticks" % (script["id"], expected["frames"], script["ticks"]))
        ticks = [checkpoint["tick"] for checkpoint in expected["checkpoints"]]
        wanted = [tick for tick in range(1, script["ticks"] + 1) if tick % script["every"] == 0 or tick == script["ticks"]]
        if ticks != wanted:
            raise AssertionError("%s: checkpoints at %r, expected %r" % (script["id"], ticks, wanted))
        if expected["checkpoints"][-1]["digest"] != expected["digest"]:
            raise AssertionError("%s: the last checkpoint does not carry the digest of the trace" % script["id"])
        digests = numpy.array([checkpoint["digest"] for checkpoint in expected["checkpoints"]], dtype=numpy.int64)
        if bool(numpy.any(digests < 0)) or bool(numpy.any(digests > 4294967295)):
            raise AssertionError("%s: a digest is not an unsigned 32-bit integer" % script["id"])
        for law in LAWS:
            if expected["laws"].get(law) is not True:
                raise AssertionError("%s: the recorded trace breaks the law %s" % (script["id"], law))
        for checkpoint in expected["checkpoints"]:
            verify_checkpoint(menagerie, script, checkpoint)
    return document


# endregion 🔖️Laws


# region 🔖️Handlers
def committed(ctx):
    """🧫️ The committed vectors, verified."""
    return verify(json.loads(ctx.fixture_bytes(VECTORS)))


def traces(ctx):
    """🎞️ The committed trace of every script."""
    return Outcome({script["id"]: script["expected"] for script in committed(ctx)["scripts"]})


def laws(ctx):
    """⚖️ The law flags of every committed trace: all true, or ``verify`` has raised."""
    return Outcome({script["id"]: script["expected"]["laws"] for script in committed(ctx)["scripts"]})


def determinism(ctx):
    """🔁️ What a deterministic stage owes every script: the same trace again, another one for another seed, the same end however time is cut."""
    return Outcome({script["id"]: {"repeatable": True, "seeded": True, "chunked": True} for script in committed(ctx)["scripts"]})


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter():
    """🧭️ Keeper of the specification vectors: answers with the committed projections after checking their laws."""
    return Adapter("python").oracle("traces", traces).oracle("laws", laws).oracle("determinism", determinism)


# endregion 🔖️Registration

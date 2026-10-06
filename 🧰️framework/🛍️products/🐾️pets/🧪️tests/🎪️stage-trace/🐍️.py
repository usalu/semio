#!/usr/bin/env python3
"""🎪️ Keeper of the committed stage traces (design §5, §10), in Python.

No third-party library simulates the pets stage, and a second engine written to be compared with it would be the
same model again (recorded no-oracle decision ``pets-stage-trace``). This file therefore simulates nothing: it reads
the COMMITTED trace of every script and checks what can be checked on it without the stage — that every recorded
actor on a perch stands on a surveyed surface at the height of that surface (minus its hover; one that scoots to a
new seat at most two of its widths off it), that an actor clings to a surveyed wall exactly while it is on a wall, that the box of its body touches no keep-out (but on a scooter's way out
of one), that an upright body is the size box, the hover and a margin of 4 px, that the recorded bodies of no two
actors overlap anywhere (the shared area of every pair of boxes, by broadcasting), that no more actors walk or hop
than the mode allows, that at most one pair has partners, that opacities are whole, that the frame of a checkpoint
shows at most 160 particles and one lifted copy (none, and nobody held, on a still stage) and holds only a pet that is
in the learner's hand, and that every law flag the recording carries is true. The digests are the subject's own; the
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
MARGIN = 4
STAGE_CAP = 160


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
        if (actor["footing"] == "perch") != (actor["perch"] is not None):
            raise AssertionError("%s: %s has footing %s and perch %r" % (where, actor["species"], actor["footing"], actor["perch"]))
        if (actor["footing"] == "wall") != (actor["wall"] is not None):
            raise AssertionError("%s: %s has footing %s and wall %r" % (where, actor["species"], actor["footing"], actor["wall"]))
        if actor["wall"] is not None and (survey is None or actor["wall"] not in [wall["id"] for wall in survey["walls"]]):
            raise AssertionError("%s: %s clings to %s, which is not surveyed" % (where, actor["species"], actor["wall"]))
        half = kind["size"]["width"] / 2
        if actor["footing"] in ("perch", "head"):
            upright = numpy.array([actor["x"] - half - MARGIN, actor["y"] - kind["size"]["height"] - MARGIN, actor["x"] + half + MARGIN, actor["y"] + hover_of(kind) + MARGIN])
            if not numpy.allclose(numpy.array([actor["body"][edge] for edge in ("x0", "y0", "x1", "y1")]), upright, rtol=0, atol=1e-9):
                raise AssertionError("%s: the body of %s is not its size box, hover and margin" % (where, actor["species"]))
        if actor["perch"] is None:
            continue
        if survey is None:
            raise AssertionError("%s: %s stands on %s before anything was surveyed" % (where, actor["species"], actor["perch"]))
        surfaces = [surface for surface in survey["surfaces"] if surface["id"] == actor["perch"]]
        if len(surfaces) != 1:
            raise AssertionError("%s: %s stands on %s, which is not surveyed" % (where, actor["species"], actor["perch"]))
        surface = surfaces[0]
        slack = 2 * kind["size"]["width"] if actor["activity"] == "scoot" else 0
        if not (max(surface["x0"], 0) + half - slack <= actor["x"] <= min(surface["x1"], survey["width"]) - half + slack):
            raise AssertionError("%s: %s at x=%r does not stand inside %s" % (where, actor["species"], actor["x"], actor["perch"]))
        if actor["y"] != surface["y"] - hover_of(kind):
            raise AssertionError("%s: %s at y=%r is not on %s" % (where, actor["species"], actor["y"], actor["perch"]))
        for keepout in survey["keepouts"]:
            if actor["activity"] != "scoot" and overlaps(actor["x"] - half, actor["y"] - kind["size"]["height"], actor["x"] + half, actor["y"], keepout):
                raise AssertionError("%s: %s stands inside a keep-out" % (where, actor["species"]))
    if actors:
        boxes = numpy.array([[actor["body"][edge] for edge in ("x0", "y0", "x1", "y1")] for actor in actors])
        shared = numpy.minimum(boxes[:, None, 2], boxes[None, :, 2]) - numpy.maximum(boxes[:, None, 0], boxes[None, :, 0])
        tall = numpy.minimum(boxes[:, None, 3], boxes[None, :, 3]) - numpy.maximum(boxes[:, None, 1], boxes[None, :, 1])
        clash = numpy.argwhere(numpy.triu((shared > 0) & (tall > 0), 1))
        if clash.size:
            one, two = clash[0]
            raise AssertionError("%s: the bodies of %s and %s overlap" % (where, actors[one]["species"], actors[two]["species"]))
    if movers > MOVERS[checkpoint["mode"]] and checkpoint["tick"] - tuned > HOP_TICKS + 1:
        raise AssertionError("%s: %d actors move in mode %s" % (where, movers, checkpoint["mode"]))
    if partners > 2:
        raise AssertionError("%s: %d actors have partners" % (where, partners))
    verify_scenery(where, checkpoint, listed)


def verify_scenery(where, checkpoint, listed):
    """🎆️ What the frame of a checkpoint shows beside the actors: whole counts, at most ``STAGE_CAP`` particles and one lifted copy, nothing of the kind (and no dust) on a still stage, and a held pet that is on stage in the learner's hand."""
    drawn = checkpoint["drawn"]
    counts = numpy.array([drawn["particles"], drawn["ladders"], drawn["lifts"], drawn["puffs"]])
    if counts.dtype.kind != "i" or bool(numpy.any(counts < 0)):
        raise AssertionError("%s: the scenery counts %r are no whole numbers" % (where, drawn))
    if drawn["particles"] > STAGE_CAP or drawn["lifts"] > 1:
        raise AssertionError("%s: %d particles and %d lifted copies" % (where, drawn["particles"], drawn["lifts"]))
    if checkpoint["mode"] == "still" and (drawn["particles"] > 0 or drawn["lifts"] > 0 or drawn["puffs"] > 0 or drawn["held"] is not None):
        raise AssertionError("%s: a still stage shows particles, a lifted copy, dust or a held pet" % where)
    held = drawn["held"]
    if held is not None and (held not in listed or [actor for actor in checkpoint["actors"] if actor["species"] == held][0]["footing"] != "hand"):
        raise AssertionError("%s: %s is held but not in the learner's hand on stage" % (where, held))


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
    return verify(json.loads(ctx.input_bytes(VECTORS)))


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

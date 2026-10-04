#!/usr/bin/env python3
"""🧬️ Oracle of the pets contract's conformance: ``jsonschema`` judges every committed menagerie, species and ensemble.

``jsonschema``'s Draft 7 validator reads the normative ``🧬️schema/🔣️.json`` and judges three populations of the
committed vectors: the accepted documents, which must conform; the structurally broken documents, each of which
must break the schema at the keyword and JSON pointer its vector names; and the documents that break a rule of
the design (§3), which the schema either cannot see at all — then they must conform — or sees as the keyword
their vector names.

The rules beyond the schema (unique ids, resolved references, bone order, key order, loop seams, gait clips,
hover, bonds, casts, and of the second round: states, tricks, purrs, emitters, the clips a gear and every species
owe, a floater's gear, grip and reach inside the size box, and the chemistry between species) have no schema
keyword and no library: ``findings`` below restates them from the design text for documents whose structure is
sound. That second implementation is a supplement that holds both cores to one reading of the rules; the
third-party evidence of this case is the schema verdict.

@see https://python-jsonschema.readthedocs.io/en/stable/validate/
@see ../../🧬️schema/🔣️.json
@see ../../🧫️fixtures/🧬️schema-conformance/🔣️.json
@see ../../README.md — the table of codes and the pointer each one is reported at
"""

# region 🔖️Imports
import json
import math
import os
import re

import jsonschema

from semio_repo_test import Adapter, Outcome

# endregion 🔖️Imports


# region 🔖️Schema
SCHEMA = os.path.join(os.path.dirname(os.path.abspath(__file__)), "..", "..", "🧬️schema", "🔣️.json")
VECTORS = "shared://🧬️schema-conformance/🔣️.json"
CODES = {"type": "type-invalid", "required": "required", "additionalProperties": "property-unknown", "enum": "value-invalid", "const": "value-invalid", "minItems": "length-invalid", "maxItems": "length-invalid"}


def validator(definition):
    """🛡️ A Draft 7 validator of one ``$defs`` entry, resolving references inside the normative schema."""
    with open(SCHEMA, "r", encoding="utf-8") as handle:
        schema = json.load(handle)
    return jsonschema.Draft7Validator({**schema, "$ref": "#/$defs/%s" % definition})


def at(base, key):
    """🧷️ A JSON pointer one step below ``base``, escaping ``~`` and ``/``."""
    return "%s/%s" % (base, str(key).replace("~", "~0").replace("/", "~1"))


def pointer(parts):
    """📍️ The JSON pointer of a path of keys and indices."""
    reached = ""
    for part in parts:
        reached = at(reached, part)
    return reached


def reported(error):
    """💥️ What one schema error and the errors nested in its ``oneOf`` branches report, as ``(keyword, pointer)``.

    ``required`` and ``additionalProperties`` are reported at the property concerned, every other keyword at
    the value it judges.
    """
    here = pointer(error.absolute_path)
    if error.validator == "required":
        found = [("required", at(here, name)) for name in error.validator_value if name not in error.instance]
    elif error.validator == "additionalProperties":
        found = [("additionalProperties", at(here, name)) for name in error.instance if name not in error.schema.get("properties", {})]
    else:
        found = [(error.validator, here)]
    for nested in error.context or []:
        found += reported(nested)
    return found


def complaints(definition, document):
    """📣️ Everything the schema reports about one document, as sorted ``(keyword, pointer)`` pairs; empty when it conforms."""
    return sorted({pair for error in validator(definition).iter_errors(document) for pair in reported(error)})


def resolve(vectors, vector):
    """🔎️ The document of a vector: inline, or the value its JSON pointer reaches in the vectors."""
    if "document" in vector:
        return vector["document"]
    value = vectors
    for part in vector["pointer"].split("/")[1:]:
        key = part.replace("~1", "/").replace("~0", "~")
        value = value[int(key)] if isinstance(value, list) else value[key]
    return value


# endregion 🔖️Schema


# region 🔖️Rules
SLUG = re.compile(r"[a-z0-9]+(?:-[a-z0-9]+)*")
COLOR = re.compile(r"#[0-9a-f]{6}")
ACTIVITIES = ["idle", "fidget", "walk", "hop", "fall", "land", "sleep", "greet", "cuddle", "squabble", "sulk", "hang", "tumble", "glide", "aim", "reel", "climb", "mantle", "slide", "carry", "trick", "purr", "dizzy", "shrug", "scoot", "push"]
LANGUAGES = ["en", "de"]
POSITIVE = {"ellipse": ["rx", "ry"], "rect": ["width", "height"]}
GEAR_ACTIVITIES = {"climb": ["climb", "mantle", "slide"], "ladder": ["carry", "climb"], "grapple": ["aim", "reel"], "parachute": ["glide"]}
GROUND_GEARS = ["climb", "ladder", "grapple"]
OWED_ACTIVITIES = ["hang", "tumble", "purr", "dizzy", "shrug", "push"]


def slug(value, path, out):
    """🏷️ The value when it is a slug of 1…64 characters; ``slug-invalid`` otherwise."""
    if len(value) <= 64 and SLUG.fullmatch(value):
        return value
    out.append((path, "slug-invalid"))
    return None


def reference(value, path, out, known):
    """🔗️ A slug that must be one of ``known`` (when that is given); ``unknown-reference`` otherwise."""
    valid = slug(value, path, out)
    if valid is not None and known is not None and valid not in known:
        out.append((path, "unknown-reference"))
    return valid


def positive(json_object, key, path, out, or_zero=False):
    """📏️ A number above zero (``or_zero``: zero or above) when the object carries it; hands it on when valid."""
    if key not in json_object:
        return None
    value = json_object[key]
    if value < 0 if or_zero else value <= 0:
        out.append((at(path, key), "out-of-range"))
        return None
    return value


def text(value, path, out):
    """🌍️ A text whose every language holds at least one character."""
    for language in LANGUAGES:
        if len(value[language]) < 1:
            out.append((at(path, language), "length-invalid"))


def unique(ids, path, out, key=None, code="duplicate-id"):
    """👯️ ``code`` (``duplicate-id`` unless told otherwise) at every entry that repeats an earlier one."""
    seen = set()
    for index, value in enumerate(ids):
        if value is None:
            continue
        if value in seen:
            out.append((at(path, index) if key is None else at(at(path, index), key), code))
        seen.add(value)


def between(json_object, key, path, out, low, high):
    """🎚️ A number in ``[low, high]`` when the object carries it; ``out-of-range`` otherwise."""
    if key in json_object and (json_object[key] < low or json_object[key] > high):
        out.append((at(path, key), "out-of-range"))


def shape(value, path, out):
    """🔷️ The geometry of a part, an emitter or a canopy: a path that is not empty, positive extents, a corner radius of zero or more."""
    if value["kind"] == "path" and len(value["d"]) < 1:
        out.append((at(path, "d"), "length-invalid"))
    for key in POSITIVE.get(value["kind"], []):
        positive(value, key, path, out)
    if value["kind"] == "rect":
        positive(value, "radius", path, out, or_zero=True)


def bones(entries, path, out):
    """🦴️ Parents first, exactly the first bone without a parent; hands the bone ids on."""
    ids = [slug(bone["id"], at(at(path, index), "id"), out) for index, bone in enumerate(entries)]
    if len(entries) < 1:
        out.append((path, "items-too-few"))
    unique(ids, path, out, "id")
    for index, bone in enumerate(entries):
        if "parent" not in bone:
            if index > 0:
                out.append((at(path, index), "bone-order"))
            continue
        parent = slug(bone["parent"], at(at(path, index), "parent"), out)
        if parent is None:
            continue
        first = ids.index(parent) if parent in ids else -1
        if first < 0:
            out.append((at(at(path, index), "parent"), "unknown-reference"))
        if index == 0 or first >= index:
            out.append((at(at(path, index), "parent"), "bone-order"))
    return {value for value in ids if value is not None}


def parts(entries, path, out, bone_ids):
    """🧩️ Every part on a bone of the rig, with positive extents; hands the part ids on."""
    ids = []
    for index, part in enumerate(entries):
        here = at(path, index)
        reference(part["bone"], at(here, "bone"), out, bone_ids)
        shape(part["shape"], at(here, "shape"), out)
        positive(part, "strokeWidth", here, out)
        ids.append(slug(part["id"], at(here, "id"), out))
    unique(ids, path, out, "id")
    return {value for value in ids if value is not None}


def face(value, path, out, bone_ids, part_ids):
    """🙂️ Eyes whose pupil is smaller than their white, an optional mouth, and the part the face is drawn above."""
    ids = []
    for index, eye in enumerate(value["eyes"]):
        here = at(at(path, "eyes"), index)
        reference(eye["bone"], at(here, "bone"), out, bone_ids)
        radius = positive(eye, "radius", here, out)
        pupil = positive(eye, "pupil", here, out)
        if radius is not None and pupil is not None and pupil >= radius:
            out.append((at(here, "pupil"), "out-of-range"))
        ids.append(slug(eye["id"], at(here, "id"), out))
    unique(ids, at(path, "eyes"), out, "id")
    if "mouth" in value:
        reference(value["mouth"]["bone"], at(at(path, "mouth"), "bone"), out, bone_ids)
        positive(value["mouth"], "width", at(path, "mouth"), out)
    if "above" in value:
        reference(value["above"], at(path, "above"), out, part_ids)


def track(value, path, out, bone_ids, loop):
    """🛤️ At least two keys in strictly ascending phase from 0 to 1, eases inside [0, 1] and, in a looping clip, a seamless end."""
    reference(value["bone"], at(path, "bone"), out, bone_ids)
    keys = value["keys"]
    here = at(path, "keys")
    for index, key in enumerate(keys):
        for abscissa in [0, 2]:
            if "ease" in key and (key["ease"][abscissa] < 0 or key["ease"][abscissa] > 1):
                out.append((at(at(at(here, index), "ease"), abscissa), "ease-range"))
        if key["at"] < 0 or key["at"] > 1:
            out.append((at(at(here, index), "at"), "key-order"))
    if len(keys) < 2:
        out.append((here, "key-order"))
        return
    last = len(keys) - 1
    for index, key in enumerate(keys):
        if (index == 0 and key["at"] != 0) or (index == last and key["at"] != 1) or (index > 0 and key["at"] <= keys[index - 1]["at"]):
            out.append((at(at(here, index), "at"), "key-order"))
    start, end = keys[0]["value"], keys[last]["value"]
    if not loop or start == end:
        return
    revolutions = (end - start) / 360
    if value["channel"] != "rotation" or abs(revolutions - math.floor(revolutions + 0.5)) > 1e-9:
        out.append((at(at(here, last), "value"), "loop-seam"))


def clips(entries, path, out, bone_ids):
    """🎞️ Every clip with a positive length and sound tracks; hands the clip ids on."""
    ids = []
    for index, clip in enumerate(entries):
        here = at(path, index)
        positive(clip, "seconds", here, out)
        for number, member in enumerate(clip["tracks"]):
            track(member, at(at(here, "tracks"), number), out, bone_ids, clip["loop"])
        ids.append(slug(clip["id"], at(here, "id"), out))
    unique(ids, path, out, "id")
    return {value for value in ids if value is not None}


def emitters(entries, path, out, bone_ids):
    """✨️ Every emitter on a bone of the rig, with 1…32 particles, a positive life, a speed of zero or more and a spread in [0, 1]; hands the emitter ids on."""
    ids = []
    for index, emitter in enumerate(entries):
        here = at(path, index)
        reference(emitter["bone"], at(here, "bone"), out, bone_ids)
        shape(emitter["shape"], at(here, "shape"), out)
        positive(emitter, "strokeWidth", here, out)
        between(emitter, "count", here, out, 1, 32)
        positive(emitter, "life", here, out)
        positive(emitter, "speed", here, out, or_zero=True)
        between(emitter, "spread", here, out, 0, 1)
        ids.append(slug(emitter["id"], at(here, "id"), out))
    unique(ids, path, out, "id")
    return {value for value in ids if value is not None}


def states(entries, path, out, clip_ids, emitter_ids):
    """🔦️ At least one state; tints in lowercase colours, the overlay clip and the emitter of the species, and a state that lasts names the state it gives way to; hands the state ids on."""
    ids = []
    for index, state in enumerate(entries):
        here = at(path, index)
        text(state["name"], at(here, "name"), out)
        for name, color in state.get("tint", {}).items():
            if not COLOR.fullmatch(color):
                out.append((at(at(here, "tint"), name), "out-of-range"))
        if "clip" in state:
            reference(state["clip"], at(here, "clip"), out, clip_ids)
        if "emitter" in state:
            reference(state["emitter"], at(here, "emitter"), out, emitter_ids)
        positive(state, "lasts", here, out)
        if "lasts" in state and "then" not in state:
            out.append((at(here, "then"), "lasts-then"))
        ids.append(slug(state["id"], at(here, "id"), out))
    if len(entries) < 1:
        out.append((path, "items-too-few"))
    unique(ids, path, out, "id")
    known = {value for value in ids if value is not None}
    for index, state in enumerate(entries):
        if "then" in state:
            reference(state["then"], at(at(path, index), "then"), out, known)
    return known


def tricks(entries, path, out, clip_ids, emitter_ids, state_ids):
    """🪄️ Every trick with a clip of the species, distinct cues, an optional emitter and states of the species; hands the trick ids on."""
    ids = []
    for index, trick in enumerate(entries):
        here = at(path, index)
        text(trick["name"], at(here, "name"), out)
        reference(trick["clip"], at(here, "clip"), out, clip_ids)
        unique(trick["cues"], at(here, "cues"), out, code="duplicate-entry")
        if "emitter" in trick:
            reference(trick["emitter"], at(here, "emitter"), out, emitter_ids)
        if "from" in trick:
            unique([reference(state, at(at(here, "from"), number), out, state_ids) for number, state in enumerate(trick["from"])], at(here, "from"), out, code="duplicate-entry")
        if "to" in trick:
            reference(trick["to"], at(here, "to"), out, state_ids)
        ids.append(slug(trick["id"], at(here, "id"), out))
    unique(ids, path, out, "id")
    return {value for value in ids if value is not None}


def species(value, path, out):
    """🐾️ One species; hands its id, its state ids and its trick ids on when the id is a slug."""
    text(value["name"], at(path, "name"), out)
    text(value["thing"], at(path, "thing"), out)
    for index, ground in enumerate(value["grounds"]):
        if len(ground) < 1:
            out.append((at(at(path, "grounds"), index), "length-invalid"))
    for side in ["width", "height"]:
        positive(value["size"], side, at(path, "size"), out)
    for name in ["body", "accent", "detail"]:
        if not COLOR.fullmatch(value["palette"][name]):
            out.append((at(at(path, "palette"), name), "out-of-range"))
    bone_ids = bones(value["bones"], at(path, "bones"), out)
    part_ids = parts(value["parts"], at(path, "parts"), out, bone_ids)
    face(value["face"], at(path, "face"), out, bone_ids, part_ids)
    clip_ids = clips(value["clips"], at(path, "clips"), out, bone_ids)
    played = set()
    for activity in ACTIVITIES:
        listed = value["repertoire"].get(activity)
        if listed is None:
            continue
        for index, clip in enumerate(listed):
            reference(clip, at(at(at(path, "repertoire"), activity), index), out, clip_ids)
        if len(listed) > 0:
            played.add(activity)
    locomotion = value["locomotion"]
    positive(locomotion, "speed", at(path, "locomotion"), out)
    positive(locomotion, "hover", at(path, "locomotion"), out)
    if (locomotion["gait"] == "float") != ("hover" in locomotion):
        out.append((at(at(path, "locomotion"), "hover"), "float-hover"))
    if locomotion["gait"] != "float" and locomotion["gait"] not in played:
        out.append((at(at(path, "locomotion"), "gait"), "missing-gait-clip"))
    for trait in ["energy", "sociability", "curiosity"]:
        if value["temperament"][trait] < 0 or value["temperament"][trait] > 1:
            out.append((at(at(path, "temperament"), trait), "out-of-range"))
    for activity in OWED_ACTIVITIES:
        if activity not in played:
            out.append((at(at(path, "repertoire"), activity), "missing-activity-clip"))
    emitter_ids = emitters(value["emitters"], at(path, "emitters"), out, bone_ids)
    state_ids = states(value["states"], at(path, "states"), out, clip_ids, emitter_ids)
    trick_ids = tricks(value["tricks"], at(path, "tricks"), out, clip_ids, emitter_ids, state_ids)
    reference(value["purr"]["clip"], at(at(path, "purr"), "clip"), out, clip_ids)
    if "emitter" in value["purr"]:
        reference(value["purr"]["emitter"], at(at(path, "purr"), "emitter"), out, emitter_ids)
    unique(value["gear"], at(path, "gear"), out, code="duplicate-entry")
    for index, gear in enumerate(value["gear"]):
        if locomotion["gait"] == "float" and gear in GROUND_GEARS:
            out.append((at(at(path, "gear"), index), "floater-gear"))
        if any(activity not in played for activity in GEAR_ACTIVITIES[gear]):
            out.append((at(at(path, "gear"), index), "missing-gear-clip"))
    if positive(value, "grip", path, out) is not None and 0 < value["size"]["height"] < value["grip"]:
        out.append((at(path, "grip"), "out-of-range"))
    if positive(value, "reach", path, out, or_zero=True) is not None and 0 < value["size"]["width"] < value["reach"]:
        out.append((at(path, "reach"), "out-of-range"))
    if "canopy" in value:
        shape(value["canopy"], at(path, "canopy"), out)
    identifier = slug(value["id"], at(path, "id"), out)
    return None if identifier is None else {"id": identifier, "states": state_ids, "tricks": trick_ids}


def bonds(entries, path, out, known):
    """🤝️ Two different species per bond, every unordered pair at most once, an affinity in [−1, 1]."""
    pairs = set()
    for index, bond in enumerate(entries):
        here = at(path, index)
        if bond["affinity"] < -1 or bond["affinity"] > 1:
            out.append((at(here, "affinity"), "out-of-range"))
        left, right = [reference(end, at(at(here, "between"), number), out, known) for number, end in enumerate(bond["between"])]
        if left is None or right is None:
            continue
        if left == right:
            out.append((at(here, "between"), "self-bond"))
            continue
        pair = tuple(sorted([left, right]))
        if pair in pairs:
            out.append((at(here, "between"), "duplicate-bond"))
        pairs.add(pair)


def casts(entries, path, out, known):
    """🎟️ Unique scenes, each with at least one core species."""
    scenes = []
    for index, cast in enumerate(entries):
        here = at(path, index)
        for members in ["core", "rotation"]:
            for number, member in enumerate(cast[members]):
                reference(member, at(at(here, members), number), out, known)
        if len(cast["core"]) < 1:
            out.append((at(here, "core"), "empty-cast"))
        scenes.append(slug(cast["scene"], at(here, "scene"), out))
    seen = set()
    for index, scene in enumerate(scenes):
        if scene is None:
            continue
        if scene in seen:
            out.append((at(at(path, index), "scene"), "duplicate-scene"))
        seen.add(scene)


def trait(value, path, out, kinds, anyone):
    """🕵️ One side of a reaction: a species of the menagerie when it names one, a state and a trick of that species (of some species of the menagerie when it names none) and a positive time held; hands on the species it names when the menagerie knows it, ``anyone`` when it names none."""
    if "species" in value:
        named = reference(value["species"], at(path, "species"), out, None if kinds is None else set(kinds))
        kind = None if named is None or kinds is None else kinds.get(named)
    else:
        kind = anyone
    if "state" in value:
        reference(value["state"], at(path, "state"), out, None if kind is None else kind["states"])
    if "trick" in value:
        reference(value["trick"], at(path, "trick"), out, None if kind is None else kind["tricks"])
    positive(value, "held", path, out)
    return kind


def chemistry(entries, path, out, kinds):
    """⚗️ Unique reaction ids, sides that resolve (a side that names no species stands for anyone), a third side ``unless`` like them, affinity bounds in [−1, 1] with the low one first, positive distances and periods, a chance in [0, 1] and at least one effect whose state and trick belong to the species of its side."""
    ids = []
    anyone = None if kinds is None else {"id": "", "states": {state for kind in kinds.values() for state in kind["states"]}, "tricks": {trick for kind in kinds.values() for trick in kind["tricks"]}}
    for index, reaction in enumerate(entries):
        here = at(path, index)
        sides = {party: trait(reaction[party], at(here, party), out, kinds, anyone) for party in ["when", "near"]}
        if "unless" in reaction:
            trait(reaction["unless"], at(here, "unless"), out, kinds, anyone)
        if "affinity" in reaction:
            ends = reaction["affinity"]
            for number, end in enumerate(ends):
                if end < -1 or end > 1:
                    out.append((at(at(here, "affinity"), number), "out-of-range"))
            if len(ends) != 2:
                out.append((at(here, "affinity"), "length-invalid"))
            elif ends[0] > ends[1]:
                out.append((at(here, "affinity"), "out-of-range"))
        positive(reaction, "within", here, out)
        positive(reaction, "every", here, out)
        between(reaction, "chance", here, out, 0, 1)
        for number, effect in enumerate(reaction["then"]):
            there = at(at(here, "then"), number)
            kind = sides[effect["on"]]
            if "state" in effect:
                reference(effect["state"], at(there, "state"), out, None if kind is None else kind["states"])
            if "trick" in effect:
                reference(effect["trick"], at(there, "trick"), out, None if kind is None else kind["tricks"])
            between(effect, "amount", there, out, 0, 1)
            between(effect, "rapport", there, out, -1, 1)
        if len(reaction["then"]) < 1:
            out.append((at(here, "then"), "items-too-few"))
        ids.append(slug(reaction["id"], at(here, "id"), out))
    unique(ids, path, out, "id")


def findings(definition, document):
    """🩺️ The rule findings of a structurally sound document, as ``{path, code}`` sorted by path, then code, in code point order."""
    out = []
    if definition == "Species":
        species(document, "", out)
    else:
        slug(document["id"], "/id", out)
        text(document["title"], "/title", out)
        known = None
        kinds = None
        if definition == "Menagerie":
            found = [species(member, at("/species", index), out) for index, member in enumerate(document["species"])]
            unique([None if kind is None else kind["id"] for kind in found], "/species", out, "id")
            kinds = {}
            for kind in found:
                if kind is not None:
                    kinds.setdefault(kind["id"], kind)
            known = set(kinds)
        else:
            paths = []
            for index, path in enumerate(document["species"]):
                if len(path) < 1:
                    out.append((at("/species", index), "length-invalid"))
                paths.append(path if len(path) >= 1 else None)
            unique(paths, "/species", out)
        bonds(document["bonds"], "/bonds", out, known)
        casts(document["casts"], "/casts", out, known)
        chemistry(document["chemistry"], "/chemistry", out, kinds)
    return [{"path": path, "code": code} for path, code in sorted(set(out))]


# endregion 🔖️Rules


# region 🔖️Handlers
def vectors(ctx):
    """🧫️ The committed vectors."""
    return json.loads(ctx.fixture_bytes(VECTORS))


def accepted_documents(ctx):
    """✅️ Every accepted document conforms to the schema and breaks no rule."""
    committed = vectors(ctx)
    produced = {}
    for vector in committed["accepted"]:
        document = resolve(committed, vector)
        broken = complaints(vector["definition"], document)
        if broken:
            raise AssertionError("accepted-documents/%s: not a %s — %r" % (vector["id"], vector["definition"], broken))
        found = findings(vector["definition"], document)
        if found:
            raise AssertionError("accepted-documents/%s: the rule reference finds %r" % (vector["id"], found))
        produced[vector["id"]] = True
    return Outcome(produced)


def structural_rejections(ctx):
    """🚫️ Every structurally broken document breaks the schema at the keyword and pointer its vector names."""
    produced = {}
    for vector in vectors(ctx)["structural"]:
        broken = complaints(vector["definition"], vector["document"])
        named = (vector["violates"]["keyword"], vector["violates"]["at"])
        if named not in broken:
            raise AssertionError("structural-rejections/%s: expected %r, the schema reports %r" % (vector["id"], named, broken))
        if vector["issues"] != [{"path": named[1], "code": CODES[named[0]]}]:
            raise AssertionError("structural-rejections/%s: the committed finding %r does not restate %r" % (vector["id"], vector["issues"], named))
        produced[vector["id"]] = False
    return Outcome(produced)


def rule_violations(ctx):
    """⚖️ Every rule document yields the committed findings; the schema conforms or names the committed keyword."""
    produced = {}
    for vector in vectors(ctx)["rules"]:
        found = findings(vector["definition"], vector["document"])
        if not found or found != vector["issues"]:
            raise AssertionError("rule-violations/%s: committed %r, the reference finds %r" % (vector["id"], vector["issues"], found))
        broken = complaints(vector["definition"], vector["document"])
        if vector["violates"] is None and broken:
            raise AssertionError("rule-violations/%s: the schema should not see this rule, it reports %r" % (vector["id"], broken))
        if vector["violates"] is not None and (vector["violates"]["keyword"], vector["violates"]["at"]) not in broken:
            raise AssertionError("rule-violations/%s: expected %r, the schema reports %r" % (vector["id"], vector["violates"], broken))
        produced[vector["id"]] = found
    return Outcome(produced)


# endregion 🔖️Handlers


# region 🔖️Registration
def adapter():
    """🧭️ Oracle role only: ``jsonschema`` is the reference, the owned validators are the subjects."""
    return Adapter("python").oracle("accepted-documents", accepted_documents).oracle("structural-rejections", structural_rejections).oracle("rule-violations", rule_violations)


# endregion 🔖️Registration

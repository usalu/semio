#!/usr/bin/env python3
"""🖐️ An INDEPENDENT second implementation of the `s.block.5d` part-kind document and all forty-one
of its typed mutations, in Python, serving as this case's differential oracle.

**Why a second implementation and not a third-party library.** A `block5d` document is a KIND
DEFINITION carrying BOTH presentations of one part at once: a 2d facet (shape, extent, colour, icon
kind), a 3d facet (orientation quaternion and scale), the mesh representations it offers, the grip
kinds it declares, and the grips placed on it in BOTH spaces — each grip carrying a 2d polar
placement (`angle`, `radius2d`) and a 3d placement (`position`, `direction`, `radius3d`) at the same
time. Nothing outside this repository models one record placed in two spaces at once, and none of
them reads `.dsl.semio`. The sibling `🧱️mutate-block-2d-1` settled the same question the same way over
the same carrier, and this file follows it.

**What it was written from.**

* ``🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json`` — the thirteen members of
  `Block5dSnapshot`.
* ``…/🧬️schema/🧬️mutations/🔣️.json`` — the forty-one verbs and their argument lists.
* rules 1, 2 and 7 of
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` — the
  part-kind scalars, the rule-1 `update-` exception for the two inseparable facets, the four id-keyed
  collections, and absolute `move`/`resize` per space.
* the forty-one committed `(before, mutation, diff, outcome, after)` quintets, which are the only
  statement of the one thing this vocabulary does not share with its siblings: that `update-part-2d`
  REBUILDS the 2d facet from its six arguments in their declared order and DROPS every member whose
  argument is `null` — its vector turns a `{shape, radius, color, iconKind}` circle into a
  `{shape, width, height, color}` rectangle, so the facet loses two members and gains two.

**No Rust was read to write this.** `🦀️.rs` beside this file registers the SUBJECT half
only. Unlike its `🧊️3d` sibling this subset holds its whole grip-kind vocabulary LOCALLY — there is
no composed catalogue child — so all forty-one kinds are adjudicated and none is refused.
"""

# region 🔖️Imports
import copy
import json

from semio_repo_test import Adapter, Outcome

# endregion 🔖️Imports


# region 🔖️Vocabulary
MEMBERS = ("schema", "partKind", "2d", "3d", "representations", "gripKinds", "grips", "compatibility", "attributes", "authors", "camera2d", "camera3d", "meta")
"""🗂️ The thirteen members `Block5dSnapshot` declares — and the cross-language projection."""

PART_KIND_FIELDS = {"rename-part-kind": ("name", "newName"), "change-part-kind-label": ("label", "newLabel"), "change-part-kind-variant": ("variant", "newVariant"), "change-part-kind-description": ("description", "newDescription"), "change-part-kind-icon": ("icon", "newIcon"), "change-part-kind-unit": ("unit", "newUnit")}
"""✏️ The six kind-level scalar setters."""

PART_2D_FIELDS = (("shape", "newShape"), ("radius", "newRadius"), ("width", "newWidth"), ("height", "newHeight"), ("color", "newColor"), ("iconKind", "newIconKind"))
"""🖌️ `update-part-2d` rebuilds the 2d facet from these six arguments, in this order, dropping every
one whose argument is `null` — which is how its committed vector turns a circle with a radius and an
icon kind into a rectangle with a width and a height and neither."""

PART_3D_FIELDS = (("orientation", "newOrientation"), ("scale", "newScale"))
"""🧊 `update-part-3d` rebuilds the 3d facet from both of its arguments; neither is optional."""

REPRESENTATION_FIELDS = {"rename-representation": ("name", "newName"), "change-representation-mesh-url": ("meshUrl", "newMeshUrl"), "change-representation-lod": ("lod", "newLod"), "change-representation-description": ("description", "newDescription")}
"""✒️ The four representation setters, all addressed by `id`."""

GRIP_KIND_FIELDS = {"rename-grip-kind": ("name", "newName"), "change-grip-kind-label": ("label", "newLabel"), "change-grip-kind-color": ("color", "newColor"), "change-grip-kind-default-rope-kind": ("defaultRopeKind", "newDefaultRopeKind")}
"""🎨 The four grip-kind setters. All four stay in this document: unlike `s.block.3d`, this subset
declares no composed catalogue child, so `rename-grip-kind` is an ordinary local setter."""

GRIP_FIELDS = {"change-grip-grip-kind": ("gripKind", "newGripKind"), "resize-grip-3d": ("radius3d", "newRadius3d")}
"""📍 The two single-field grip setters."""

COLLECTIONS = {"create-representation": ("representations", "representation"), "create-grip-kind": ("gripKinds", "gripKind"), "create-grip": ("grips", "grip"), "add-compatibility-rule": ("compatibility", "rule"), "add-attribute": ("attributes", "attribute"), "add-author": ("authors", "author")}
"""🌱 The six append verbs: which member each writes and what its whole-record payload is called."""

REMOVALS = {"delete-representation": "representations", "delete-grip-kind": "gripKinds", "delete-grip": "grips", "remove-compatibility-rule": "compatibility", "remove-author": "authors"}
"""🗑️ The five id-addressed removals. `remove-attribute` is addressed by `key` and is handled on its
own, because `attributes` is the one table in this document that is not id-keyed."""

KINDS = (
    "rename-part-kind",
    "change-part-kind-label",
    "change-part-kind-variant",
    "change-part-kind-description",
    "change-part-kind-icon",
    "change-part-kind-unit",
    "update-part-2d",
    "update-part-3d",
    "create-representation",
    "delete-representation",
    "rename-representation",
    "change-representation-mesh-url",
    "change-representation-lod",
    "change-representation-description",
    "add-representation-tag",
    "remove-representation-tag",
    "add-representation-attribute",
    "remove-representation-attribute",
    "create-grip-kind",
    "delete-grip-kind",
    "rename-grip-kind",
    "change-grip-kind-label",
    "change-grip-kind-color",
    "change-grip-kind-default-rope-kind",
    "create-grip",
    "delete-grip",
    "move-grip-2d",
    "move-grip-3d",
    "resize-grip-3d",
    "change-grip-grip-kind",
    "add-compatibility-rule",
    "remove-compatibility-rule",
    "add-attribute",
    "remove-attribute",
    "add-author",
    "remove-author",
    "move-camera2d",
    "scale-camera2d",
    "move-camera3d",
    "scale-camera3d",
    "change-meta-description",
)
"""🏷️ Every kind the catalog declares, in its declared order."""


def tag_of(kind):
    """🔤️ The internally tagged `mutation` discriminator of a kind — lowerCamelCase of its words."""
    head, *rest = kind.split("-")
    return head + "".join(word[:1].upper() + word[1:] for word in rest)


TAGS = {kind: tag_of(kind) for kind in KINDS}

RECORDS = {"representations": {"id", "name", "meshUrl", "tags", "lod", "description", "attributes"}, "gripKinds": {"id", "name", "label", "color", "defaultRopeKind"}, "grips": {"id", "gripKind", "angle", "radius2d", "position", "direction", "radius3d"}, "compatibility": {"id", "source", "target", "bidirectional"}}
"""🧱️ The members each id-keyed record carries, as the committed vectors spell them."""

WHOLE = {"partKind": {"id", "name", "label", "variant", "description", "icon", "unit"}, "camera2d": {"x", "y", "zoom"}, "camera3d": {"position", "target", "zoom"}, "meta": {"description"}}
"""📐️ The members each fixed whole-value member carries. The two facets `2d` and `3d` are NOT here:
`update-part-2d` legitimately changes which members the 2d facet holds."""
# endregion 🔖️Vocabulary


# region 🔖️Document
def validate(document, where):
    """✅️ Holds the document to the shape the committed vectors agree on, and to id uniqueness."""
    if set(document) != set(MEMBERS):
        raise AssertionError("%s: a block5d document must carry exactly %r, found %r" % (where, sorted(MEMBERS), sorted(document)))
    for member, expected in WHOLE.items():
        if set(document[member]) != expected:
            raise AssertionError("%s: %s must carry exactly %r, found %r" % (where, member, sorted(expected), sorted(document[member])))
    if document["2d"].get("shape") not in ("circle", "rectangle"):
        raise AssertionError("%s: the 2d facet must declare a circle or a rectangle, found %r" % (where, document["2d"].get("shape")))
    if set(document["3d"]) != {"orientation", "scale"}:
        raise AssertionError("%s: the 3d facet must carry exactly an orientation and a scale, found %r" % (where, sorted(document["3d"])))
    for name, expected in RECORDS.items():
        identifiers = []
        for record in document[name]:
            if set(record) != expected:
                raise AssertionError("%s: a %s record must carry exactly %r, found %r" % (where, name, sorted(expected), sorted(record)))
            identifiers.append(record["id"])
        if len(set(identifiers)) != len(identifiers):
            raise AssertionError("%s: %s carries a duplicate id in %r" % (where, name, identifiers))
    keys = [entry["key"] for entry in document["attributes"]]
    if len(set(keys)) != len(keys):
        raise AssertionError("%s: attributes carries a duplicate key in %r" % (where, keys))


def located(rows, identity, kind, where):
    """🔎️ The index of the record this kind addresses; an absent id is an error, never a no-op —
    every committed vector of this subset declares `status: applied`."""
    for at, record in enumerate(rows):
        if record.get("id") == identity:
            return at
    raise AssertionError("%s-%s: the committed vector addresses %r, which the before-snapshot does not hold" % (where, kind, identity))


def rebuilt(fields, payload):
    """🖌️ A rule-1 `update-` facet: rebuilt from its arguments in their declared order, dropping every
    member whose argument is `null`."""
    return {member: copy.deepcopy(payload[argument]) for member, argument in fields if payload.get(argument) is not None}


def arguments_for(fields, facet):
    """↩️ The arguments that rebuild a facet exactly as it stands — the inverse of `rebuilt`."""
    return {argument: copy.deepcopy(facet[member]) if member in facet else None for member, argument in fields}
# endregion 🔖️Document


# region 🔖️Verbs
def apply_mutation(document, kind, payload):
    """🦠️ Applies one kind. Every committed vector of this subset declares `status: applied`, so an
    address the document does not hold is an error rather than a rejection outcome."""
    document = copy.deepcopy(document)
    if kind in PART_KIND_FIELDS:
        field, argument = PART_KIND_FIELDS[kind]
        document["partKind"][field] = payload[argument]
    elif kind == "update-part-2d":
        document["2d"] = rebuilt(PART_2D_FIELDS, payload)
    elif kind == "update-part-3d":
        document["3d"] = rebuilt(PART_3D_FIELDS, payload)
    elif kind in COLLECTIONS:
        member, argument = COLLECTIONS[kind]
        document[member].append(copy.deepcopy(payload[argument]))
    elif kind in REMOVALS:
        member = REMOVALS[kind]
        document[member].pop(located(document[member], payload["id"], kind, "mutate"))
    elif kind in REPRESENTATION_FIELDS:
        field, argument = REPRESENTATION_FIELDS[kind]
        document["representations"][located(document["representations"], payload["id"], kind, "mutate")][field] = payload[argument]
    elif kind == "add-representation-tag":
        document["representations"][located(document["representations"], payload["id"], kind, "mutate")]["tags"].append(payload["tag"])
    elif kind == "remove-representation-tag":
        tags = document["representations"][located(document["representations"], payload["id"], kind, "mutate")]["tags"]
        if payload["tag"] not in tags:
            raise AssertionError("mutate-%s: the committed vector removes the tag %r, which the representation does not carry" % (kind, payload["tag"]))
        tags.remove(payload["tag"])
    elif kind == "add-representation-attribute":
        document["representations"][located(document["representations"], payload["id"], kind, "mutate")]["attributes"].append(copy.deepcopy(payload["attribute"]))
    elif kind == "remove-representation-attribute":
        record = document["representations"][located(document["representations"], payload["id"], kind, "mutate")]
        record["attributes"] = [entry for entry in record["attributes"] if entry["key"] != payload["key"]]
    elif kind in GRIP_KIND_FIELDS:
        field, argument = GRIP_KIND_FIELDS[kind]
        document["gripKinds"][located(document["gripKinds"], payload["id"], kind, "mutate")][field] = payload[argument]
    elif kind == "move-grip-2d":
        record = document["grips"][located(document["grips"], payload["id"], kind, "mutate")]
        record["angle"] = payload["newAngle"]
        record["radius2d"] = payload["newRadius2d"]
    elif kind == "move-grip-3d":
        record = document["grips"][located(document["grips"], payload["id"], kind, "mutate")]
        record["position"] = copy.deepcopy(payload["newPosition"])
        record["direction"] = copy.deepcopy(payload["newDirection"])
    elif kind in GRIP_FIELDS:
        field, argument = GRIP_FIELDS[kind]
        document["grips"][located(document["grips"], payload["id"], kind, "mutate")][field] = payload[argument]
    elif kind == "remove-attribute":
        document["attributes"] = [entry for entry in document["attributes"] if entry["key"] != payload["key"]]
    elif kind == "move-camera2d":
        document["camera2d"]["x"] = payload["newX"]
        document["camera2d"]["y"] = payload["newY"]
    elif kind == "scale-camera2d":
        document["camera2d"]["zoom"] = payload["newZoom"]
    elif kind == "move-camera3d":
        document["camera3d"]["position"] = copy.deepcopy(payload["newPosition"])
        document["camera3d"]["target"] = copy.deepcopy(payload["newTarget"])
    elif kind == "scale-camera3d":
        document["camera3d"]["zoom"] = payload["newZoom"]
    elif kind == "change-meta-description":
        document["meta"]["description"] = payload["newDescription"]
    else:
        raise AssertionError("mutate-%s: this implementation declares no verb for that kind" % kind)
    return document


def inverse_mutation(document, kind, payload):
    """↩️ The kind's OWN inverse, expressed in this same closed vocabulary, computed against the
    pre-mutation document. `create`/`add` inverts to `delete`/`remove`, which is exact only for a
    TRAILING record because no `create-`/`add-` verb here carries an index — a property of the closed
    schema, shared by both implementations."""
    if kind in PART_KIND_FIELDS:
        field, argument = PART_KIND_FIELDS[kind]
        return [(kind, {argument: document["partKind"][field]})]
    if kind == "update-part-2d":
        return [(kind, arguments_for(PART_2D_FIELDS, document["2d"]))]
    if kind == "update-part-3d":
        return [(kind, arguments_for(PART_3D_FIELDS, document["3d"]))]
    if kind in COLLECTIONS:
        member, argument = COLLECTIONS[kind]
        record = payload[argument]
        if member == "attributes":
            return [("remove-attribute", {"key": record["key"]})]
        undo = {"representations": "delete-representation", "gripKinds": "delete-grip-kind", "grips": "delete-grip", "compatibility": "remove-compatibility-rule", "authors": "remove-author"}[member]
        return [(undo, {"id": record["id"]})]
    if kind in REMOVALS:
        member = REMOVALS[kind]
        record = document[member][located(document[member], payload["id"], kind, "inverse")]
        redo = {"representations": ("create-representation", "representation"), "gripKinds": ("create-grip-kind", "gripKind"), "grips": ("create-grip", "grip"), "compatibility": ("add-compatibility-rule", "rule"), "authors": ("add-author", "author")}[member]
        return [(redo[0], {redo[1]: copy.deepcopy(record)})]
    if kind in REPRESENTATION_FIELDS:
        field, argument = REPRESENTATION_FIELDS[kind]
        return [(kind, {"id": payload["id"], argument: document["representations"][located(document["representations"], payload["id"], kind, "inverse")][field]})]
    if kind == "add-representation-tag":
        return [("remove-representation-tag", {"id": payload["id"], "tag": payload["tag"]})]
    if kind == "remove-representation-tag":
        return [("add-representation-tag", {"id": payload["id"], "tag": payload["tag"]})]
    if kind == "add-representation-attribute":
        return [("remove-representation-attribute", {"id": payload["id"], "key": payload["attribute"]["key"]})]
    if kind == "remove-representation-attribute":
        record = document["representations"][located(document["representations"], payload["id"], kind, "inverse")]
        held = next(entry for entry in record["attributes"] if entry["key"] == payload["key"])
        return [("add-representation-attribute", {"id": payload["id"], "attribute": copy.deepcopy(held)})]
    if kind in GRIP_KIND_FIELDS:
        field, argument = GRIP_KIND_FIELDS[kind]
        return [(kind, {"id": payload["id"], argument: document["gripKinds"][located(document["gripKinds"], payload["id"], kind, "inverse")][field]})]
    if kind == "move-grip-2d":
        record = document["grips"][located(document["grips"], payload["id"], kind, "inverse")]
        return [(kind, {"id": payload["id"], "newAngle": record["angle"], "newRadius2d": record["radius2d"]})]
    if kind == "move-grip-3d":
        record = document["grips"][located(document["grips"], payload["id"], kind, "inverse")]
        return [(kind, {"id": payload["id"], "newPosition": copy.deepcopy(record["position"]), "newDirection": copy.deepcopy(record["direction"])})]
    if kind in GRIP_FIELDS:
        field, argument = GRIP_FIELDS[kind]
        return [(kind, {"id": payload["id"], argument: document["grips"][located(document["grips"], payload["id"], kind, "inverse")][field]})]
    if kind == "remove-attribute":
        held = next(entry for entry in document["attributes"] if entry["key"] == payload["key"])
        return [("add-attribute", {"attribute": copy.deepcopy(held)})]
    if kind == "move-camera2d":
        return [(kind, {"newX": document["camera2d"]["x"], "newY": document["camera2d"]["y"]})]
    if kind == "scale-camera2d":
        return [(kind, {"newZoom": document["camera2d"]["zoom"]})]
    if kind == "move-camera3d":
        return [(kind, {"newPosition": copy.deepcopy(document["camera3d"]["position"]), "newTarget": copy.deepcopy(document["camera3d"]["target"])})]
    if kind == "scale-camera3d":
        return [(kind, {"newZoom": document["camera3d"]["zoom"]})]
    if kind == "change-meta-description":
        return [(kind, {"newDescription": document["meta"]["description"]})]
    raise AssertionError("inverse-%s: this implementation declares no inverse for that kind" % kind)
# endregion 🔖️Verbs


# region 🔖️Laws
def equals_committed(kind, produced, committed):
    """🎯️ The committed after-snapshot claim, member by member, with no tolerance and no ignored key."""
    for member in MEMBERS:
        if produced[member] != committed[member]:
            raise AssertionError("mutate-%s: %s is %s, the committed after-snapshot says %s" % (kind, member, json.dumps(produced[member], sort_keys=True)[:300], json.dumps(committed[member], sort_keys=True)[:300]))


def observable(kind, before, after):
    """👁️ Every committed vector of this subset declares `status: applied`, so every one must move
    the compared projection. There is no exemption list here, and none is needed."""
    if before == after:
        raise AssertionError("mutate-%s: the committed vector declares this kind applied, yet the document did not move" % kind)


def touches_one(kind, before, after):
    """🎯️ Every kind in this vocabulary writes exactly ONE of the thirteen members — the check an
    after-snapshot comparison cannot make on its own."""
    moved = [member for member in MEMBERS if before[member] != after[member]]
    if len(moved) != 1:
        raise AssertionError("mutate-%s: moved %r; every kind in this vocabulary writes exactly one member" % (kind, moved))


def declares(diff, member):
    """🔺️ Whether the committed diff really declares a member: every arm is always on the wire, so
    `null` and `[]` declare nothing while a whole-value replacement carrying an empty list does."""
    held = diff.get(member)
    if held is None:
        return False
    if isinstance(held, list):
        return len(held) > 0
    return True


DIFF_ALIASES = {"2d": "part2d", "3d": "part3d"}
"""🔗️ The two members whose diff arm cannot be named after them, because a field name may not start
with a digit."""


def footprint(kind, before, after, diff):
    """⚖️ Footprint completeness: before and after differ on exactly the members the committed diff
    declares. Restated by hand here because the Python host exposes no shared `law` module."""
    changed = [member for member in MEMBERS if before[member] != after[member]]
    for member in changed:
        if not declares(diff, DIFF_ALIASES.get(member, member)):
            raise AssertionError("inverse-%s: the snapshot member %r moved without the committed diff declaring it, so an undo built from that diff would not restore it" % (kind, member))
    for member in MEMBERS:
        if declares(diff, DIFF_ALIASES.get(member, member)) and member not in changed:
            raise AssertionError("inverse-%s: the committed diff declares %r, yet it is identical in both committed snapshots" % (kind, member))


def restores(kind, restored, original):
    """↩️ The full inverse law — stronger than the footprint law the subject half asserts, and
    available here because this implementation really applies the verbs: applying the kind and then
    its OWN computed inverse must land back on the committed before-snapshot, member for member and
    index for index."""
    for member in MEMBERS:
        if restored[member] != original[member]:
            raise AssertionError("inverse-%s: %s came back as %s, not %s" % (kind, member, json.dumps(restored[member], sort_keys=True)[:300], json.dumps(original[member], sort_keys=True)[:300]))
# endregion 🔖️Laws


# region 🔖️Plan
def doc_json(ctx):
    """📜️ The scenario's doc string — the Python `Context` has no accessor of its own."""
    for step in ctx.scenario["steps"]:
        if step.get("docString"):
            return json.loads(step["docString"])
    raise AssertionError("scenario %s carries no doc string" % ctx.scenario["id"])


def leaf(ctx, spec, name):
    """🧫️ One committed leaf of the vector the doc string addresses."""
    return json.loads(ctx.fixture_bytes(spec[name]).decode("utf-8"))


def uri_in(ctx, needle):
    """🧫️ The one declared fixture URI of this scenario's steps containing `needle`."""
    for step in ctx.scenario["steps"]:
        for token in step["text"].split():
            if token.startswith(("asset://", "local://", "shared://")) and needle in token:
                return token
    raise AssertionError("scenario %s declares no fixture URI containing %r" % (ctx.scenario["id"], needle))


def payload_of(spec_mutation, kind):
    """🦠️ The committed payload, checked to carry this kind's own internally tagged discriminator."""
    if spec_mutation.get("mutation") != TAGS[kind]:
        raise AssertionError("mutate-%s: the committed vector carries a %r payload" % (kind, spec_mutation.get("mutation")))
    return {key: value for key, value in spec_mutation.items() if key != "mutation"}


def outcome_of(payload):
    """📤️ Wraps a projection with its own compact serialization as the raw artifact."""
    return Outcome(payload, raw=json.dumps(payload, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))
# endregion 🔖️Plan


# region 🔖️Handlers
def mutate_handler(kind):
    """🎯️ Applies one kind to its committed before-snapshot and asserts, in role, the committed
    after-snapshot, the declared status, observability and the single-member footprint."""

    def handler(ctx):
        spec = doc_json(ctx)
        if spec["kind"] != kind:
            raise AssertionError("mutate-%s: the feature's doc string states %r" % (kind, spec["kind"]))
        before = leaf(ctx, spec, "before")
        after = leaf(ctx, spec, "after")
        outcome = leaf(ctx, spec, "outcome")
        if outcome.get("status") != "applied":
            raise AssertionError("mutate-%s: the committed outcome declares %r; this feature replays applied vectors only" % (kind, outcome.get("status")))
        validate(before, "mutate-%s" % kind)
        applied = apply_mutation(before, kind, payload_of(leaf(ctx, spec, "mutation"), kind))
        validate(applied, "mutate-%s" % kind)
        equals_committed(kind, applied, after)
        observable(kind, before, applied)
        touches_one(kind, before, applied)
        return outcome_of(applied)

    return handler


def inverse_handler(kind):
    """↩️ Applies one kind and then its OWN computed inverse, requires the committed before-snapshot
    back, and additionally holds the committed diff to the footprint law the subject half asserts."""

    def handler(ctx):
        spec = doc_json(ctx)
        if spec["kind"] != kind:
            raise AssertionError("inverse-%s: the feature's doc string states %r" % (kind, spec["kind"]))
        before = leaf(ctx, spec, "before")
        after = leaf(ctx, spec, "after")
        payload = payload_of(leaf(ctx, spec, "mutation"), kind)
        validate(before, "inverse-%s" % kind)
        current = apply_mutation(before, kind, payload)
        for step_kind, step_payload in inverse_mutation(before, kind, payload):
            current = apply_mutation(current, step_kind, step_payload)
        restores(kind, current, before)
        footprint(kind, before, after, leaf(ctx, spec, "diff"))
        return outcome_of(current)

    return handler


def identity_handler(ctx):
    """🔁️ Reads the one committed 5d document that carries both presentations at once, and answers
    with the whole document. This implementation additionally requires, in role, that it really is
    that document: a 2d facet, a 3d facet, and at least one grip placed in BOTH spaces and bound to a
    grip kind the document declares. A codec that dropped one space could not satisfy it."""
    uri = uri_in(ctx, "⬅️before")
    committed = ctx.fixture_bytes(uri)
    document = json.loads(committed.decode("utf-8"))
    validate(document, "identity-round-trip")
    if not document["grips"] or not document["gripKinds"] or not document["representations"]:
        raise AssertionError("identity-round-trip: the committed round-trip snapshot must carry grip kinds, a placed grip and a representation")
    declared = {record["id"] for record in document["gripKinds"]}
    for grip in document["grips"]:
        if grip["gripKind"] not in declared:
            raise AssertionError("identity-round-trip: grip %r is bound to %r, which the document does not declare" % (grip["id"], grip["gripKind"]))
        for space in ("angle", "radius2d", "position", "direction", "radius3d"):
            if grip.get(space) is None:
                raise AssertionError("identity-round-trip: grip %r carries no %s, so it is not placed in both spaces" % (grip["id"], space))
    reserialized = json.dumps(document, separators=(",", ":"), ensure_ascii=False).encode("utf-8")
    if reserialized == committed:
        raise AssertionError("identity-round-trip: the committed file is pretty-printed and this writer is compact, so reproducing its bytes exactly would mean the handler returned the input unread")
    reparsed = json.loads(reserialized.decode("utf-8"))
    if reparsed != document:
        raise AssertionError("identity-round-trip: serializing and re-reading the document moved it")
    return Outcome(reparsed, raw=reserialized)
# endregion 🔖️Handlers


# region 🔖️Registration
def adapter():
    """🧭️ Registration by FULL expanded scenario id, in the ORACLE role only — registering these
    handlers as subjects too would make the reference its own subject and manufacture a green
    self-comparison."""
    built = Adapter("python")
    for kind in KINDS:
        built = built.oracle("mutate-%s" % kind, mutate_handler(kind))
        built = built.oracle("inverse-%s" % kind, inverse_handler(kind))
    return built.oracle("identity-round-trip", identity_handler)
# endregion 🔖️Registration

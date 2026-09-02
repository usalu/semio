#!/usr/bin/env python3
"""🧩️ An INDEPENDENT second implementation of the `s.puzzle.3d` scene document and its thirty-five
typed mutations, in Python, serving as this case's differential oracle.

**Why a second implementation and not a third-party library.** A `puzzle3d` document is a SCENE whose
connectivity is not spatial: objects carry their own VORTICES, and an attraction joins two of them by
a two-part address `"<objectId>:<vortexId>"` rather than by a transform. Beside the objects sit target
volumes and image references, each with its own placement, and a metadata block holding a
kind-compatibility relation and an optional kind catalogue. No scene-graph interchange format —
glTF, USD, IFC — models a joint whose endpoints are named ports owned by two nodes, and none of them
reads `.dsl.semio`. What a reference genuinely can adjudicate is this document's own algebra, and that
it IS adjudicable was settled in this same wave by `mutate-fem3d-1` and `mutate-gisterrain-1`, which
took Python second implementations over this same carrier.

**What it was written from.**

* ``🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json`` — the seven members of
  `Puzzle3dSnapshot`.
* rules 2, 4 and 7 of
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md`.
* the thirty-five committed `(before, mutation, diff, outcome, after)` quintets, for the verbs and
  their argument lists and for the three things only they state: that a vortex list is INSIDE the
  object, so removing a vortex is an object mutation that CASCADES into the attractions addressed to
  it; that deleting an object severs every attraction naming any of its vortices; and that `scale` is
  a union — `scale-object` writes a per-axis triple over a uniform scalar and `scale-target-volume`
  writes a uniform scalar over a per-axis triple.

The verbs were NOT read from ``…/🧬️schema/🧬️mutations/🔣️.json``: that file is titled
`Puzzle3dMutation` but declares the SNAPSHOT's members, the pre-migration whole-snapshot-shaped
generic schema `s.architect.program`'s own mutation schema records itself as superseding. It was never
replaced here.

**No Rust was read to write this.** `🦀️.rs` beside this file registers the SUBJECT half
only.

**One kind this implementation REFUSES, by clause rather than by absence.** See `UNDERDETERMINED`.
"""

# region 🔖️Imports
import copy
import json

from semio_repo_test import Adapter, Outcome

# endregion 🔖️Imports


# region 🔖️Vocabulary
MEMBERS = ("schema", "domain", "meta", "objects", "attractions", "targetVolumes", "references")
"""🗂️ The seven members `Puzzle3dSnapshot` declares — and the cross-language projection."""

OBJECT_FIELDS = {"move-object": ("origin", "newOrigin"), "rotate-object": ("orientation", "newOrientation"), "scale-object": ("scale", "newScale"), "change-object-mesh": ("meshUrl", "newMeshUrl"), "edit-object-label": ("label", "newLabel"), "change-object-kind": ("objectKind", "newObjectKind"), "change-object-anchor": ("anchor", "newAnchor"), "change-object-hidden": ("hidden", "newHidden"), "change-object-locked": ("locked", "newLocked")}
"""✏️ The nine single-field object setters."""

VOLUME_FIELDS = {"move-target-volume": ("origin", "newOrigin"), "rotate-target-volume": ("orientation", "newOrientation"), "scale-target-volume": ("scale", "newScale"), "change-target-volume-hidden": ("hidden", "newHidden"), "change-target-volume-locked": ("locked", "newLocked")}
"""📦 The five single-field target-volume setters."""

REFERENCE_FIELDS = {"move-reference": ("origin", "newOrigin"), "resize-reference": ("widthWorld", "newWidthWorld"), "replace-reference-source": ("source", "newSource"), "change-reference-hidden": ("hidden", "newHidden"), "change-reference-locked": ("locked", "newLocked")}
"""🖼️ The five single-field reference setters."""

COLLECTIONS = {"create-object": ("objects", "object"), "create-target-volume": ("targetVolumes", "targetVolume"), "create-reference": ("references", "reference")}
"""🌱 The three indexed append verbs: which member each writes and what its whole-record payload is
called."""

REMOVALS = {"delete-object": "objects", "delete-target-volume": "targetVolumes", "delete-reference": "references"}
"""🗑️ The three id-addressed removals. `delete-object` additionally severs attractions."""

ATTRACTION_GEOMETRY = ("gap", "shift", "rise", "rotation", "turn", "tilt", "x", "y")
"""🧮 The eight geometry members of an attraction. `replace-attraction-geometry` names them with a
`new` prefix; `connect-vortices` names them bare, the one place in this vocabulary where the same
eight values are addressed under two spellings."""

COMPATIBILITY_FIELDS = ("source", "target", "bidirectional", "important", "specificity")
"""🤝 The members of one kind-compatibility record, in the order `connect-kind-compatibility` names
them."""

UNDERDETERMINED = {"replace-object-vortex"}
"""🚧️ The one kind this implementation refuses to state — see `UNDERDETERMINED_REASON`."""

UNDERDETERMINED_REASON = (
    "this implementation refuses this kind rather than guessing it. Its single committed vector supplies a genuinely different vortex — `vortex-1` "
    "moves from `vortex-kind-a` to `vortex-kind-c` — and yet the committed outcome declares `mutation.no-op` and the after-snapshot is identical to "
    "the before-snapshot. At least three rules produce exactly that and no committed document distinguishes them: the verb is unimplemented; it "
    "refuses a vortex an attraction is addressed to, which `vortex-1` is; or it refuses a vortex kind the `kindCompatibility` relation does not admit, "
    "which `vortex-kind-c` is. `📓️derivation-rules.md` rule 2 says `replace-<singular>-<member>` replaces the addressed record, so a second "
    "implementation written from the specification would move the document. ONE more committed vector, on an unattracted vortex, decides it. Its "
    "sibling `mutate-puzzle-2d-1` reports the identical gap over `replace-node-handle`."
)

KINDS = (
    "create-object",
    "delete-object",
    "move-object",
    "rotate-object",
    "scale-object",
    "change-object-mesh",
    "edit-object-label",
    "change-object-kind",
    "change-object-anchor",
    "change-object-hidden",
    "change-object-locked",
    "add-object-vortex",
    "remove-object-vortex",
    "replace-object-vortex",
    "connect-vortices",
    "disconnect-vortices",
    "replace-attraction-geometry",
    "create-target-volume",
    "delete-target-volume",
    "move-target-volume",
    "rotate-target-volume",
    "scale-target-volume",
    "change-target-volume-hidden",
    "change-target-volume-locked",
    "create-reference",
    "delete-reference",
    "move-reference",
    "resize-reference",
    "replace-reference-source",
    "change-reference-hidden",
    "change-reference-locked",
    "change-domain",
    "connect-kind-compatibility",
    "disconnect-kind-compatibility",
    "replace-kind-catalogs",
)
"""🏷️ Every kind the catalog declares, in its declared order."""


def tag_of(kind):
    """🔤️ The internally tagged `mutation` discriminator of a kind — lowerCamelCase of its words."""
    head, *rest = kind.split("-")
    return head + "".join(word[:1].upper() + word[1:] for word in rest)


TAGS = {kind: tag_of(kind) for kind in KINDS}
# endregion 🔖️Vocabulary


# region 🔖️Document
def validate(document, where):
    """✅️ Holds the document to the shape the committed vectors agree on: seven members, unique
    object, vortex, attraction, volume and reference ids, and every attraction addressed to a
    `"<objectId>:<vortexId>"` pair the scene really holds."""
    if set(document) != set(MEMBERS):
        raise AssertionError("%s: a puzzle3d document must carry exactly %r, found %r" % (where, sorted(MEMBERS), sorted(document)))
    if "kindCompatibility" not in document["meta"]:
        raise AssertionError("%s: meta must carry a kindCompatibility relation, found %r" % (where, sorted(document["meta"])))
    ports = set()
    for member in ("objects", "targetVolumes", "references"):
        identifiers = [record["id"] for record in document[member]]
        if len(set(identifiers)) != len(identifiers):
            raise AssertionError("%s: %s carries a duplicate id in %r" % (where, member, identifiers))
    for record in document["objects"]:
        for vortex in record["vortices"]:
            port = "%s:%s" % (record["id"], vortex["id"])
            if port in ports:
                raise AssertionError("%s: the port %r is declared twice" % (where, port))
            ports.add(port)
    for attraction in document["attractions"]:
        for end in ("attracting", "attracted"):
            if attraction[end] not in ports:
                raise AssertionError("%s: attraction %r names %s port %r, which this scene does not hold" % (where, attraction["id"], end, attraction[end]))


def record_at(document, member, identity, kind, where):
    """🔎️ The index of the record this kind addresses; an absent id is an error, never a no-op."""
    for at, record in enumerate(document[member]):
        if record["id"] == identity:
            return at
    raise AssertionError("%s-%s: the committed vector addresses %s %r, which the before-snapshot does not hold" % (where, kind, member, identity))


def ports_of(record):
    """🔌 The two-part addresses one object's vortices answer to."""
    return {"%s:%s" % (record["id"], vortex["id"]) for vortex in record["vortices"]}


def attached_to(document, ports):
    """✂️ The attractions addressed to any of these ports, in scene order — what a removal severs."""
    return [held for held in document["attractions"] if held["attracting"] in ports or held["attracted"] in ports]
# endregion 🔖️Document


# region 🔖️Verbs
def apply_mutation(document, kind, payload):
    """🦠️ Applies one kind. Every committed vector of this subset declares `status: applied`, so an
    address the scene does not hold is an error rather than a rejection outcome."""
    if kind in UNDERDETERMINED:
        raise AssertionError("mutate-%s: %s" % (kind, UNDERDETERMINED_REASON))
    document = copy.deepcopy(document)
    if kind in COLLECTIONS:
        member, argument = COLLECTIONS[kind]
        index = payload.get("index")
        document[member].insert(len(document[member]) if index is None else index, copy.deepcopy(payload[argument]))
    elif kind in REMOVALS:
        member = REMOVALS[kind]
        at = record_at(document, member, payload["id"], kind, "mutate")
        if member == "objects":
            severed = ports_of(document["objects"][at])
            document["attractions"] = [held for held in document["attractions"] if held["attracting"] not in severed and held["attracted"] not in severed]
        document[member].pop(at)
    elif kind in OBJECT_FIELDS:
        member, argument = OBJECT_FIELDS[kind]
        document["objects"][record_at(document, "objects", payload["id"], kind, "mutate")][member] = copy.deepcopy(payload[argument])
    elif kind in VOLUME_FIELDS:
        member, argument = VOLUME_FIELDS[kind]
        document["targetVolumes"][record_at(document, "targetVolumes", payload["id"], kind, "mutate")][member] = copy.deepcopy(payload[argument])
    elif kind in REFERENCE_FIELDS:
        member, argument = REFERENCE_FIELDS[kind]
        document["references"][record_at(document, "references", payload["id"], kind, "mutate")][member] = copy.deepcopy(payload[argument])
    elif kind == "add-object-vortex":
        record = document["objects"][record_at(document, "objects", payload["objectId"], kind, "mutate")]
        index = payload.get("index")
        record["vortices"].insert(len(record["vortices"]) if index is None else index, copy.deepcopy(payload["vortex"]))
    elif kind == "remove-object-vortex":
        record = document["objects"][record_at(document, "objects", payload["objectId"], kind, "mutate")]
        if not any(vortex["id"] == payload["vortexId"] for vortex in record["vortices"]):
            raise AssertionError("mutate-%s: object %r declares no vortex %r" % (kind, payload["objectId"], payload["vortexId"]))
        port = "%s:%s" % (payload["objectId"], payload["vortexId"])
        record["vortices"] = [vortex for vortex in record["vortices"] if vortex["id"] != payload["vortexId"]]
        document["attractions"] = [held for held in document["attractions"] if held["attracting"] != port and held["attracted"] != port]
    elif kind == "connect-vortices":
        attraction = {"id": payload["id"], "attracting": payload["attracting"], "attracted": payload["attracted"]}
        for member in ATTRACTION_GEOMETRY:
            attraction[member] = payload[member]
        document["attractions"].append(attraction)
    elif kind == "disconnect-vortices":
        document["attractions"].pop(record_at(document, "attractions", payload["id"], kind, "mutate"))
    elif kind == "replace-attraction-geometry":
        attraction = document["attractions"][record_at(document, "attractions", payload["id"], kind, "mutate")]
        for member in ATTRACTION_GEOMETRY:
            attraction[member] = payload["new" + member[:1].upper() + member[1:]]
    elif kind == "change-domain":
        document["domain"] = payload["newDomain"]
    elif kind == "connect-kind-compatibility":
        document["meta"]["kindCompatibility"].append({member: payload[member] for member in COMPATIBILITY_FIELDS})
    elif kind == "disconnect-kind-compatibility":
        held = [rule for rule in document["meta"]["kindCompatibility"] if rule["source"] == payload["source"] and rule["target"] == payload["target"]]
        if not held:
            raise AssertionError("mutate-%s: the relation declares no %r to %r rule" % (kind, payload["source"], payload["target"]))
        document["meta"]["kindCompatibility"] = [rule for rule in document["meta"]["kindCompatibility"] if rule not in held]
    elif kind == "replace-kind-catalogs":
        document["meta"]["kindCatalogs"] = copy.deepcopy(payload["newCatalogs"])
    else:
        raise AssertionError("mutate-%s: this implementation declares no verb for that kind" % kind)
    return document


def reconnect(attraction):
    """🔗 The `connect-vortices` arguments that rebuild one attraction exactly as it stands."""
    payload = {"id": attraction["id"], "attracting": attraction["attracting"], "attracted": attraction["attracted"]}
    for member in ATTRACTION_GEOMETRY:
        payload[member] = attraction[member]
    return payload


def inverse_mutation(document, kind, payload):
    """↩️ The kind's OWN inverse, expressed in this same closed vocabulary, computed against the
    pre-mutation scene. `delete-object` and `remove-object-vortex` invert to SEVERAL steps, because
    they sever attractions: the object or vortex is put back at its own index first and every severed
    attraction is reconnected after it, in scene order."""
    if kind in UNDERDETERMINED:
        raise AssertionError("inverse-%s: %s" % (kind, UNDERDETERMINED_REASON))
    if kind in COLLECTIONS:
        member, argument = COLLECTIONS[kind]
        undo = {"objects": "delete-object", "targetVolumes": "delete-target-volume", "references": "delete-reference"}[member]
        return [(undo, {"id": payload[argument]["id"]})]
    if kind in REMOVALS:
        member = REMOVALS[kind]
        at = record_at(document, member, payload["id"], kind, "inverse")
        record = document[member][at]
        redo = {"objects": ("create-object", "object"), "targetVolumes": ("create-target-volume", "targetVolume"), "references": ("create-reference", "reference")}[member]
        steps = [(redo[0], {redo[1]: copy.deepcopy(record), "index": at})]
        if member == "objects":
            steps += [("connect-vortices", reconnect(held)) for held in attached_to(document, ports_of(record))]
        return steps
    if kind in OBJECT_FIELDS:
        member, argument = OBJECT_FIELDS[kind]
        return [(kind, {"id": payload["id"], argument: copy.deepcopy(document["objects"][record_at(document, "objects", payload["id"], kind, "inverse")][member])})]
    if kind in VOLUME_FIELDS:
        member, argument = VOLUME_FIELDS[kind]
        return [(kind, {"id": payload["id"], argument: copy.deepcopy(document["targetVolumes"][record_at(document, "targetVolumes", payload["id"], kind, "inverse")][member])})]
    if kind in REFERENCE_FIELDS:
        member, argument = REFERENCE_FIELDS[kind]
        return [(kind, {"id": payload["id"], argument: copy.deepcopy(document["references"][record_at(document, "references", payload["id"], kind, "inverse")][member])})]
    if kind == "add-object-vortex":
        return [("remove-object-vortex", {"objectId": payload["objectId"], "vortexId": payload["vortex"]["id"]})]
    if kind == "remove-object-vortex":
        record = document["objects"][record_at(document, "objects", payload["objectId"], kind, "inverse")]
        at = next(index for index, vortex in enumerate(record["vortices"]) if vortex["id"] == payload["vortexId"])
        port = "%s:%s" % (payload["objectId"], payload["vortexId"])
        steps = [("add-object-vortex", {"objectId": payload["objectId"], "vortex": copy.deepcopy(record["vortices"][at]), "index": at})]
        return steps + [("connect-vortices", reconnect(held)) for held in attached_to(document, {port})]
    if kind == "connect-vortices":
        return [("disconnect-vortices", {"id": payload["id"]})]
    if kind == "disconnect-vortices":
        return [("connect-vortices", reconnect(document["attractions"][record_at(document, "attractions", payload["id"], kind, "inverse")]))]
    if kind == "replace-attraction-geometry":
        attraction = document["attractions"][record_at(document, "attractions", payload["id"], kind, "inverse")]
        return [(kind, dict({"id": payload["id"]}, **{"new" + member[:1].upper() + member[1:]: attraction[member] for member in ATTRACTION_GEOMETRY}))]
    if kind == "change-domain":
        return [(kind, {"newDomain": document["domain"]})]
    if kind == "connect-kind-compatibility":
        return [("disconnect-kind-compatibility", {"source": payload["source"], "target": payload["target"]})]
    if kind == "disconnect-kind-compatibility":
        held = next(rule for rule in document["meta"]["kindCompatibility"] if rule["source"] == payload["source"] and rule["target"] == payload["target"])
        return [("connect-kind-compatibility", copy.deepcopy(held))]
    if kind == "replace-kind-catalogs":
        held = document["meta"].get("kindCatalogs")
        if held is None:
            raise AssertionError(
                "inverse-%s: this implementation refuses to guess this inverse. The committed vector INSTALLS a catalogue where the before-snapshot "
                "carried none, so undoing it requires REMOVING the member — and no verb in this closed vocabulary can express that. The sibling "
                "`mutate-puzzle-5d-1` commits the deciding evidence: its `null-catalogs-is-noop` vector shows that `replace-kind-catalogs` with a "
                "NULL argument is accepted and is a NO-OP, not a removal. So the gap is in the vocabulary, not in this implementation, and it is "
                "invisible to the subject half of this case, which asserts only that the committed diff and the committed snapshots agree on a "
                "footprint and never applies an inverse at all." % kind
            )
        return [(kind, {"newCatalogs": copy.deepcopy(held)})]
    raise AssertionError("inverse-%s: this implementation declares no inverse for that kind" % kind)
# endregion 🔖️Verbs


# region 🔖️Laws
def equals_committed(kind, produced, committed):
    """🎯️ The committed after-snapshot claim, member by member, with no tolerance and no ignored key."""
    for member in MEMBERS:
        if produced[member] != committed[member]:
            raise AssertionError("mutate-%s: %s is %s, the committed after-snapshot says %s" % (kind, member, json.dumps(produced[member], sort_keys=True)[:400], json.dumps(committed[member], sort_keys=True)[:400]))


def observable(kind, before, after, no_op):
    """👁️ A vector whose committed outcome does NOT declare `mutation.no-op` must move the compared
    projection; one that does must move nothing. The exemption is read off the committed outcome."""
    if no_op and before != after:
        raise AssertionError("mutate-%s: the committed outcome declares mutation.no-op, yet the scene moved" % kind)
    if not no_op and before == after:
        raise AssertionError("mutate-%s: the committed vector declares this kind applied, yet the scene did not move" % kind)


def restores(kind, restored, original):
    """↩️ The full inverse law: applying the kind and then its OWN computed inverse must land back on
    the committed before-snapshot, member for member and index for index."""
    for member in MEMBERS:
        if restored[member] != original[member]:
            raise AssertionError("inverse-%s: %s came back as %s, not %s" % (kind, member, json.dumps(restored[member], sort_keys=True)[:400], json.dumps(original[member], sort_keys=True)[:400]))
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


def declares_no_op(outcome):
    """🚦️ Whether the committed outcome itself records that the mutation had nothing to do."""
    return any(message.get("code") == "mutation.no-op" for message in outcome.get("messages", []))


def outcome_of(payload):
    """📤️ Wraps a projection with its own compact serialization as the raw artifact."""
    return Outcome(payload, raw=json.dumps(payload, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))
# endregion 🔖️Plan


# region 🔖️Handlers
def mutate_handler(kind):
    """🎯️ Applies one kind to its committed before-snapshot and asserts, in role, the committed
    after-snapshot and the observability the committed outcome implies."""

    def handler(ctx):
        spec = doc_json(ctx)
        if spec["kind"] != kind:
            raise AssertionError("mutate-%s: the feature's doc string states %r" % (kind, spec["kind"]))
        before = leaf(ctx, spec, "before")
        after = leaf(ctx, spec, "after")
        outcome = leaf(ctx, spec, "outcome")
        validate(before, "mutate-%s" % kind)
        applied = apply_mutation(before, kind, payload_of(leaf(ctx, spec, "mutation"), kind))
        validate(applied, "mutate-%s" % kind)
        equals_committed(kind, applied, after)
        observable(kind, before, applied, declares_no_op(outcome))
        return outcome_of(applied)

    return handler


def inverse_handler(kind):
    """↩️ Applies one kind and then its OWN computed inverse and requires the committed before-snapshot
    back — the full inverse law, which the subject half of this case cannot assert because it never
    applies anything."""

    def handler(ctx):
        spec = doc_json(ctx)
        if spec["kind"] != kind:
            raise AssertionError("inverse-%s: the feature's doc string states %r" % (kind, spec["kind"]))
        before = leaf(ctx, spec, "before")
        payload = payload_of(leaf(ctx, spec, "mutation"), kind)
        validate(before, "inverse-%s" % kind)
        current = apply_mutation(before, kind, payload)
        for step_kind, step_payload in inverse_mutation(before, kind, payload):
            current = apply_mutation(current, step_kind, step_payload)
        restores(kind, current, before)
        return outcome_of(current)

    return handler


def identity_handler(ctx):
    """🔁️ Reads the committed scene and answers with the whole document. This implementation
    additionally requires, in role, that it really is this scene and not a graph: two objects owning
    vortices, an attraction whose two endpoints are `"<objectId>:<vortexId>"` ports rather than object
    ids, a target volume and an image reference."""
    uri = uri_in(ctx, "⬅️before")
    committed = ctx.fixture_bytes(uri)
    document = json.loads(committed.decode("utf-8"))
    validate(document, "identity-round-trip")
    object_ids = {record["id"] for record in document["objects"]}
    if len(document["objects"]) < 2 or not document["attractions"] or not document["targetVolumes"] or not document["references"]:
        raise AssertionError("identity-round-trip: the committed scene must carry two objects, an attraction, a target volume and a reference")
    for attraction in document["attractions"]:
        for end in ("attracting", "attracted"):
            if ":" not in attraction[end] or attraction[end] in object_ids:
                raise AssertionError("identity-round-trip: attraction %r names %r as its %s; in this scene an attraction joins two \"<objectId>:<vortexId>\" ports" % (attraction["id"], attraction[end], end))
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

#!/usr/bin/env python3
"""🖐️ An INDEPENDENT second implementation of the `s.puzzle.5d` assembly document and its twenty-eight
typed mutations, in Python, serving as this case's differential oracle.

**Why a second implementation and not a third-party library.** A `puzzle5d` document is an ASSEMBLY
whose every element is placed in TWO spaces at once: each part carries a `2d` facet and a `3d` facet,
each grip carries a `2d` angle and a `3d` position, and a fastener joins two grips by a two-part
address `"<partId>:<gripId>"` rather than by a transform. No assembly or scene interchange format —
STEP AP214, glTF, USD, IFC — carries one element placed in a diagram and in a model simultaneously,
and none of them reads `.dsl.semio`. That this algebra IS adjudicable was settled in this same wave by
`mutate-fem2d-1`, `mutate-fem3d-1` and `mutate-gismap-1`, which took Python second implementations
over this same carrier.

**What it was written from.**

* ``🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️component.json`` — the seven members of
  `Puzzle5dSnapshot`. Note that `kindCompatibility` is a TOP-LEVEL member here, where both siblings
  file it inside `meta`.
* rules 1, 2, 4 and 7 of
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md`.
* the twenty-eight committed `(before, mutation, diff, outcome, after)` quintets, for the verbs and
  their argument lists and for the four things only they state: that a grip list is INSIDE the part,
  so removing a grip is a part mutation that CASCADES into the fasteners addressed to it; that
  deleting a part severs every fastener naming any of its grips; that `replace-part2d-geometry`
  rebuilds shape and extent from four arguments, dropping every member whose argument is `null`, while
  leaving the 2d facet's placement and text alone; and that `replace-kind-catalogs` with a NULL
  argument is a NO-OP rather than a removal.

The verbs were NOT read from ``…/🧬️schema/🧬️mutations/🔣️component.json``: like both siblings, that
file is titled for the mutation but declares the SNAPSHOT's members — the pre-migration
whole-snapshot-shaped generic schema `s.architect.program`'s own mutation schema records itself as
superseding, never replaced here.

**No Rust was read to write this.** `🦀️component.rs` beside this file registers the SUBJECT half
only.

**All twenty-eight kinds are adjudicated and none is refused** — and this subset is the one that
settles a question its siblings leave open. `mutate-puzzle-2d-1` and `mutate-puzzle-3d-1` each commit
exactly one `replace-<container>-<port>` vector and each declares it `mutation.no-op` with an
unchanged after-snapshot. Here the corresponding `replace-part-grip` vector really does rekind
`grip-1`, on a grip a fastener IS attached to, and the document moves. That is evidence the verb is
implemented and that being attached does not block it — which narrows, without closing, the three
readings those two siblings report.
"""

# region 🔖️Imports
import copy
import json

from semio_repo_test import Adapter, Outcome

# endregion 🔖️Imports


# region 🔖️Vocabulary
MEMBERS = ("schema", "domain", "label", "meta", "kindCompatibility", "parts", "fasteners")
"""🗂️ The seven members `Puzzle5dSnapshot` declares — and the cross-language projection."""

DEFAULTS = {"hidden": False, "locked": False, "anchor": "fixed", "scale": 1.0}
"""🫥 The four members a committed snapshot OMITS when they hold their default. No committed document
states these values; the vectors state them from both sides — each appears in an after-snapshot only
when a verb sets it to the other value, and the inverse scenario then requires it to disappear again."""

PART_2D_FIELDS = {"edit-part2d-text": "text", "change-part2d-icon": "iconKind", "change-part2d-hidden": "hidden", "change-part2d-locked": "locked"}
"""✏️ The four single-field setters that write inside a part's 2d facet."""

PART_3D_FIELDS = {"move-part3d": "origin", "rotate-part3d": "orientation", "scale-part3d": "scale", "change-part3d-mesh": "meshUrl", "edit-part3d-label": "label"}
"""🧊 The five single-field setters that write inside a part's 3d facet."""

PART_FIELDS = {"change-part-kind": "partKind", "change-part-anchor": "anchor"}
"""🧩 The two setters that write on the part itself rather than on one of its facets."""

ARGUMENT_OF = {"edit-part2d-text": "newText", "change-part2d-icon": "newIconKind", "change-part2d-hidden": "newHidden", "change-part2d-locked": "newLocked", "move-part3d": "newOrigin", "rotate-part3d": "newOrientation", "scale-part3d": "newScale", "change-part3d-mesh": "newMeshUrl", "edit-part3d-label": "newLabel", "change-part-kind": "newPartKind", "change-part-anchor": "newAnchor"}
"""🔤️ What each single-field setter calls its argument."""

PART_2D_GEOMETRY = (("shape", "newShape"), ("radius", "newRadius"), ("width", "newWidth"), ("height", "newHeight"))
"""🧮 `replace-part2d-geometry` rebuilds these four members of the 2d facet in this order, dropping
every one whose argument is `null`, and leaves the facet's placement, text, icon and flags alone."""

FASTENER_GEOMETRY = ("gap", "shift", "rise", "rotation", "turn", "tilt", "x", "y")
"""🔩 The eight geometry members of a fastener. `replace-fastener-geometry` names them with a `new`
prefix; `connect-grips` names them bare."""

COMPATIBILITY_FIELDS = ("source", "target", "bidirectional", "important", "specificity")
"""🤝 The members of one kind-compatibility record, in the order `connect-kind-compatibility` names
them."""

DOCUMENT_FIELDS = {"rename-puzzle5d": ("label", "newLabel"), "change-domain": ("domain", "newDomain")}
"""📄️ The two document-level scalar setters of rule 1. `change-description` writes inside `meta` and
is handled on its own."""

KINDS = (
    "create-part",
    "delete-part",
    "move-part2d",
    "replace-part2d-geometry",
    "edit-part2d-text",
    "change-part2d-icon",
    "change-part2d-hidden",
    "change-part2d-locked",
    "move-part3d",
    "rotate-part3d",
    "scale-part3d",
    "change-part3d-mesh",
    "edit-part3d-label",
    "change-part-kind",
    "change-part-anchor",
    "add-part-grip",
    "remove-part-grip",
    "replace-part-grip",
    "connect-grips",
    "disconnect-grips",
    "replace-fastener-geometry",
    "change-fastener-kind",
    "rename-puzzle5d",
    "change-domain",
    "change-description",
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
    """✅️ Holds the document to the shape the committed vectors agree on: seven members, unique part,
    grip and fastener ids, every part placed in BOTH spaces, every fastener addressed to a
    `"<partId>:<gripId>"` pair the assembly really holds, and no member left standing at its default."""
    if set(document) != set(MEMBERS):
        raise AssertionError("%s: a puzzle5d document must carry exactly %r, found %r" % (where, sorted(MEMBERS), sorted(document)))
    ports = set()
    identifiers = [part["id"] for part in document["parts"]]
    if len(set(identifiers)) != len(identifiers):
        raise AssertionError("%s: parts carries a duplicate id in %r" % (where, identifiers))
    for part in document["parts"]:
        for space in ("2d", "3d"):
            if space not in part:
                raise AssertionError("%s: part %r carries no %s facet, so it is not placed in both spaces" % (where, part["id"], space))
        for member, default in DEFAULTS.items():
            for holder in (part, part["2d"], part["3d"]):
                if member in holder and holder[member] == default:
                    raise AssertionError("%s: part %r writes %s at its default %r, which a committed snapshot omits" % (where, part["id"], member, default))
        for grip in part["grips"]:
            port = "%s:%s" % (part["id"], grip["id"])
            if port in ports:
                raise AssertionError("%s: the port %r is declared twice" % (where, port))
            ports.add(port)
    for fastener in document["fasteners"]:
        for end in ("source", "target"):
            if fastener[end] not in ports:
                raise AssertionError("%s: fastener %r names %s port %r, which this assembly does not hold" % (where, fastener["id"], end, fastener[end]))


def part_at(document, identity, kind, where):
    """🔎️ The index of the part this kind addresses; an absent id is an error, never a no-op."""
    for at, part in enumerate(document["parts"]):
        if part["id"] == identity:
            return at
    raise AssertionError("%s-%s: the committed vector addresses part %r, which the before-snapshot does not hold" % (where, kind, identity))


def fastener_at(document, identity, kind, where):
    """🔎️ The index of the fastener this kind addresses."""
    for at, fastener in enumerate(document["fasteners"]):
        if fastener["id"] == identity:
            return at
    raise AssertionError("%s-%s: the committed vector addresses fastener %r, which the before-snapshot does not hold" % (where, kind, identity))


def holder_of(kind, part):
    """📍 Which record a single-field setter writes into: the part, its 2d facet or its 3d facet."""
    if kind in PART_2D_FIELDS:
        return part["2d"], PART_2D_FIELDS[kind]
    if kind in PART_3D_FIELDS:
        return part["3d"], PART_3D_FIELDS[kind]
    return part, PART_FIELDS[kind]


def written(record, member, value):
    """🫥 Writes a member, or REMOVES it when the value is the one a committed snapshot omits."""
    if member in DEFAULTS and value == DEFAULTS[member]:
        record.pop(member, None)
    else:
        record[member] = copy.deepcopy(value)


def ports_of(part):
    """🔌 The two-part addresses one part's grips answer to."""
    return {"%s:%s" % (part["id"], grip["id"]) for grip in part["grips"]}


def attached_to(document, ports):
    """✂️ The fasteners addressed to any of these ports, in assembly order — what a removal severs."""
    return [held for held in document["fasteners"] if held["source"] in ports or held["target"] in ports]
# endregion 🔖️Document


# region 🔖️Verbs
def apply_mutation(document, kind, payload):
    """🦠️ Applies one kind, and answers whether it moved anything."""
    document = copy.deepcopy(document)
    if kind == "create-part":
        index = payload.get("index")
        document["parts"].insert(len(document["parts"]) if index is None else index, copy.deepcopy(payload["part"]))
    elif kind == "delete-part":
        at = part_at(document, payload["id"], kind, "mutate")
        severed = ports_of(document["parts"][at])
        document["fasteners"] = [held for held in document["fasteners"] if held["source"] not in severed and held["target"] not in severed]
        document["parts"].pop(at)
    elif kind == "move-part2d":
        facet = document["parts"][part_at(document, payload["id"], kind, "mutate")]["2d"]
        facet["x"] = payload["newX"]
        facet["y"] = payload["newY"]
    elif kind == "replace-part2d-geometry":
        facet = document["parts"][part_at(document, payload["id"], kind, "mutate")]["2d"]
        for member, argument in PART_2D_GEOMETRY:
            facet.pop(member, None)
            if payload.get(argument) is not None:
                facet[member] = copy.deepcopy(payload[argument])
    elif kind in ARGUMENT_OF:
        holder, member = holder_of(kind, document["parts"][part_at(document, payload["id"], kind, "mutate")])
        written(holder, member, payload[ARGUMENT_OF[kind]])
    elif kind == "add-part-grip":
        part = document["parts"][part_at(document, payload["partId"], kind, "mutate")]
        index = payload.get("index")
        part["grips"].insert(len(part["grips"]) if index is None else index, copy.deepcopy(payload["grip"]))
    elif kind == "remove-part-grip":
        part = document["parts"][part_at(document, payload["partId"], kind, "mutate")]
        if not any(grip["id"] == payload["gripId"] for grip in part["grips"]):
            raise AssertionError("mutate-%s: part %r declares no grip %r" % (kind, payload["partId"], payload["gripId"]))
        port = "%s:%s" % (payload["partId"], payload["gripId"])
        part["grips"] = [grip for grip in part["grips"] if grip["id"] != payload["gripId"]]
        document["fasteners"] = [held for held in document["fasteners"] if held["source"] != port and held["target"] != port]
    elif kind == "replace-part-grip":
        part = document["parts"][part_at(document, payload["partId"], kind, "mutate")]
        at = next((index for index, grip in enumerate(part["grips"]) if grip["id"] == payload["gripId"]), None)
        if at is None:
            raise AssertionError("mutate-%s: part %r declares no grip %r" % (kind, payload["partId"], payload["gripId"]))
        part["grips"][at] = copy.deepcopy(payload["newGrip"])
    elif kind == "connect-grips":
        fastener = {"id": payload["id"], "source": payload["source"], "target": payload["target"], "fastenerKind": payload["fastenerKind"]}
        for member in FASTENER_GEOMETRY:
            fastener[member] = payload[member]
        document["fasteners"].append(fastener)
    elif kind == "disconnect-grips":
        document["fasteners"].pop(fastener_at(document, payload["id"], kind, "mutate"))
    elif kind == "replace-fastener-geometry":
        fastener = document["fasteners"][fastener_at(document, payload["id"], kind, "mutate")]
        for member in FASTENER_GEOMETRY:
            fastener[member] = payload["new" + member[:1].upper() + member[1:]]
    elif kind == "change-fastener-kind":
        document["fasteners"][fastener_at(document, payload["id"], kind, "mutate")]["fastenerKind"] = payload["newFastenerKind"]
    elif kind in DOCUMENT_FIELDS:
        member, argument = DOCUMENT_FIELDS[kind]
        document[member] = payload[argument]
    elif kind == "change-description":
        document["meta"]["description"] = payload["newDescription"]
    elif kind == "connect-kind-compatibility":
        document["kindCompatibility"].append({member: payload[member] for member in COMPATIBILITY_FIELDS})
    elif kind == "disconnect-kind-compatibility":
        held = [rule for rule in document["kindCompatibility"] if rule["source"] == payload["source"] and rule["target"] == payload["target"]]
        if not held:
            raise AssertionError("mutate-%s: the relation declares no %r to %r rule" % (kind, payload["source"], payload["target"]))
        document["kindCompatibility"] = [rule for rule in document["kindCompatibility"] if rule not in held]
    elif kind == "replace-kind-catalogs":
        if payload["newCatalogs"] is not None:
            document["meta"]["kindCatalogs"] = copy.deepcopy(payload["newCatalogs"])
    else:
        raise AssertionError("mutate-%s: this implementation declares no verb for that kind" % kind)
    return document


def reconnect(fastener):
    """🔗 The `connect-grips` arguments that rebuild one fastener exactly as it stands."""
    payload = {"id": fastener["id"], "source": fastener["source"], "target": fastener["target"], "fastenerKind": fastener["fastenerKind"]}
    for member in FASTENER_GEOMETRY:
        payload[member] = fastener[member]
    return payload


def inverse_mutation(document, kind, payload):
    """↩️ The kind's OWN inverse, expressed in this same closed vocabulary, computed against the
    pre-mutation assembly. `delete-part` and `remove-part-grip` invert to SEVERAL steps, because they
    sever fasteners: the part or grip is put back at its own index first and every severed fastener is
    reconnected after it, in assembly order."""
    if kind == "create-part":
        return [("delete-part", {"id": payload["part"]["id"]})]
    if kind == "delete-part":
        at = part_at(document, payload["id"], kind, "inverse")
        part = document["parts"][at]
        steps = [("create-part", {"part": copy.deepcopy(part), "index": at})]
        return steps + [("connect-grips", reconnect(held)) for held in attached_to(document, ports_of(part))]
    if kind == "move-part2d":
        facet = document["parts"][part_at(document, payload["id"], kind, "inverse")]["2d"]
        return [(kind, {"id": payload["id"], "newX": facet["x"], "newY": facet["y"]})]
    if kind == "replace-part2d-geometry":
        facet = document["parts"][part_at(document, payload["id"], kind, "inverse")]["2d"]
        return [(kind, dict({"id": payload["id"]}, **{argument: copy.deepcopy(facet[member]) if member in facet else None for member, argument in PART_2D_GEOMETRY}))]
    if kind in ARGUMENT_OF:
        holder, member = holder_of(kind, document["parts"][part_at(document, payload["id"], kind, "inverse")])
        return [(kind, {"id": payload["id"], ARGUMENT_OF[kind]: copy.deepcopy(holder.get(member, DEFAULTS.get(member)))})]
    if kind == "add-part-grip":
        return [("remove-part-grip", {"partId": payload["partId"], "gripId": payload["grip"]["id"]})]
    if kind == "remove-part-grip":
        part = document["parts"][part_at(document, payload["partId"], kind, "inverse")]
        at = next(index for index, grip in enumerate(part["grips"]) if grip["id"] == payload["gripId"])
        port = "%s:%s" % (payload["partId"], payload["gripId"])
        steps = [("add-part-grip", {"partId": payload["partId"], "grip": copy.deepcopy(part["grips"][at]), "index": at})]
        return steps + [("connect-grips", reconnect(held)) for held in attached_to(document, {port})]
    if kind == "replace-part-grip":
        part = document["parts"][part_at(document, payload["partId"], kind, "inverse")]
        held = next(grip for grip in part["grips"] if grip["id"] == payload["gripId"])
        return [(kind, {"partId": payload["partId"], "gripId": payload["gripId"], "newGrip": copy.deepcopy(held)})]
    if kind == "connect-grips":
        return [("disconnect-grips", {"id": payload["id"]})]
    if kind == "disconnect-grips":
        return [("connect-grips", reconnect(document["fasteners"][fastener_at(document, payload["id"], kind, "inverse")]))]
    if kind == "replace-fastener-geometry":
        fastener = document["fasteners"][fastener_at(document, payload["id"], kind, "inverse")]
        return [(kind, dict({"id": payload["id"]}, **{"new" + member[:1].upper() + member[1:]: fastener[member] for member in FASTENER_GEOMETRY}))]
    if kind == "change-fastener-kind":
        return [(kind, {"id": payload["id"], "newFastenerKind": document["fasteners"][fastener_at(document, payload["id"], kind, "inverse")]["fastenerKind"]})]
    if kind in DOCUMENT_FIELDS:
        member, argument = DOCUMENT_FIELDS[kind]
        return [(kind, {argument: document[member]})]
    if kind == "change-description":
        return [(kind, {"newDescription": document["meta"]["description"]})]
    if kind == "connect-kind-compatibility":
        return [("disconnect-kind-compatibility", {"source": payload["source"], "target": payload["target"]})]
    if kind == "disconnect-kind-compatibility":
        held = next(rule for rule in document["kindCompatibility"] if rule["source"] == payload["source"] and rule["target"] == payload["target"])
        return [("connect-kind-compatibility", copy.deepcopy(held))]
    if kind == "replace-kind-catalogs":
        if payload["newCatalogs"] is None:
            return []
        held = document["meta"].get("kindCatalogs")
        if held is None:
            raise AssertionError(
                "inverse-%s: this implementation refuses to guess this inverse. Undoing an INSTALL where the before-snapshot carried no catalogue "
                "requires REMOVING the member, and this subset's own committed `null-catalogs-is-noop` vector shows that a null argument is a NO-OP "
                "rather than a removal — so no verb in this closed vocabulary can express it." % kind
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
        raise AssertionError("mutate-%s: the committed outcome declares mutation.no-op, yet the assembly moved" % kind)
    if not no_op and before == after:
        raise AssertionError("mutate-%s: the committed vector declares this kind applied, yet the assembly did not move" % kind)


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
    """🔁️ Reads the committed assembly and answers with the whole document. This implementation
    additionally requires, in role, that it really is this assembly: two parts each placed in BOTH
    spaces, grips placed in both spaces, and a fastener whose endpoints are `"<partId>:<gripId>"` ports
    rather than part ids."""
    uri = uri_in(ctx, "⬅️before")
    committed = ctx.fixture_bytes(uri)
    document = json.loads(committed.decode("utf-8"))
    validate(document, "identity-round-trip")
    part_ids = {part["id"] for part in document["parts"]}
    if len(document["parts"]) < 2 or not document["fasteners"]:
        raise AssertionError("identity-round-trip: the committed assembly must carry two parts and a fastener")
    for part in document["parts"]:
        for grip in part["grips"]:
            for space in ("2d", "3d"):
                if space not in grip:
                    raise AssertionError("identity-round-trip: grip %r on part %r carries no %s placement" % (grip["id"], part["id"], space))
    for fastener in document["fasteners"]:
        for end in ("source", "target"):
            if ":" not in fastener[end] or fastener[end] in part_ids:
                raise AssertionError("identity-round-trip: fastener %r names %r as its %s; in this assembly a fastener joins two \"<partId>:<gripId>\" ports" % (fastener["id"], fastener[end], end))
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

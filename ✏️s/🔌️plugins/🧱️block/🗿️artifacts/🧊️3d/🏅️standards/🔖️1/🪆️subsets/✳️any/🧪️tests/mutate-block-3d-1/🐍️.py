#!/usr/bin/env python3
"""🧊️ An INDEPENDENT second implementation of the `s.block.3d` object-kind document and all
thirty-seven of its typed mutations, in Python, serving as this case's differential oracle.

**Why a second implementation and not a third-party library.** A `block3d` document is a KIND
DEFINITION, not an instance: one object kind's identity, the mesh representations it offers at
several levels of detail, the vortex kinds it may expose, the vortices placed on it in 3-space, the
compatibility relation between vortex kinds, its attribute table, its authors, its editor camera and
its metadata. A component-library or symbol-library format (KiCad, Modelica, IFC property sets)
carries pins or ports but has no notion of a kind-level compatibility relation, of a vortex radius,
or of a vortex-kind vocabulary SPLIT across a shared catalogue child and a local extras table — and
none of them reads `.dsl.semio`. The sibling `mutate-block-2d-1` settled the same question the same
way over the same carrier, and this file follows it.

**What it was written from.**

* ``🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json`` — the eleven members of
  `Block3dSnapshot`.
* ``…/🧬️schema/🧬️mutations/🔣️.json`` — the thirty-seven verbs and their argument lists.
* rules 1, 2 and 7 of
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` — the
  object-kind scalars, the four id-keyed collections, and absolute `move`/`resize` for the spatial
  fields.
* the thirty-seven committed `(before, mutation, diff, outcome, after)` quintets, which are the only
  statement of two things: that `create-vortex-kind` writes the local extras record WITHOUT the
  `name` its payload carries (the name belongs to the catalogue child alone), and that an
  `attribute` is addressed by `key` while every other record is addressed by `id`.

**No Rust was read to write this.** `🦀️.rs` beside this file registers the SUBJECT half
only.

**Three kinds this implementation REFUSES, by clause rather than by absence.** `create-vortex-kind`,
`delete-vortex-kind` and `rename-vortex-kind` all rewrite `catalog`, which a committed snapshot
carries as a COMPOSED CHILD HANDLE — `{"childId": "catalog-a602bbe51a39cd44", "target": {…}}`. Their
committed after-snapshots carry a NEW `childId` (`catalog-69f2059178f5dfa4`,
`catalog-9dc5de0f33c9568d`, `catalog-e76534bc13e6b5a6`), which is a content address of the child
`s.stdio.semio@v1/kit` document after the vocabulary moved. No document in this repository states the
addressing function or the child's canonical encoding, so a second implementation cannot reproduce
it. This is the same blocker `mutate-program-1` reports over `knowledge`/`benchmarks` and
`mutate-en1990-1` reports over its composed child slot: publishing the child-addressing rule closes
all three, and no comparison profile moves.
"""

# region 🔖️Imports
import copy
import json

from semio_repo_test import Adapter, Outcome

# endregion 🔖️Imports


# region 🔖️Vocabulary
MEMBERS = ("schema", "objectKind", "representations", "catalog", "vortexKindExtra", "vortices", "compatibility", "attributes", "authors", "camera3d", "meta")
"""🗂️ The eleven members `Block3dSnapshot` declares — and the cross-language projection."""

OBJECT_KIND_FIELDS = {"rename-object-kind": ("name", "newName"), "change-object-kind-label": ("label", "newLabel"), "change-object-kind-variant": ("variant", "newVariant"), "change-object-kind-description": ("description", "newDescription"), "change-object-kind-icon": ("icon", "newIcon"), "change-object-kind-unit": ("unit", "newUnit")}
"""✏️ The six kind-level scalar setters: which member each writes and what its argument is called."""

REPRESENTATION_FIELDS = {"rename-representation": ("name", "newName"), "change-representation-mesh-url": ("meshUrl", "newMeshUrl"), "change-representation-lod": ("lod", "newLod"), "change-representation-description": ("description", "newDescription")}
"""✒️ The four representation setters, all addressed by `id`."""

VORTEX_KIND_FIELDS = {"change-vortex-kind-label": ("label", "newLabel"), "change-vortex-kind-color": ("color", "newColor"), "change-vortex-kind-default-cable-kind": ("defaultCableKind", "newDefaultCableKind")}
"""🎨 The three LOCAL vortex-kind setters — the three that stay inside `vortexKindExtra` and never
touch the catalogue child."""

VORTEX_FIELDS = {"change-vortex-vortex-kind": ("vortexKind", "newVortexKind"), "change-vortex-label": ("label", "newLabel"), "resize-vortex": ("radius", "newRadius")}
"""📍 The three single-field vortex setters. `move-vortex` writes two fields at once and is handled
on its own, because `📓️derivation-rules.md` rule 7 makes position and direction one absolute gesture."""

CATALOG_KINDS = ("create-vortex-kind", "delete-vortex-kind", "rename-vortex-kind")
"""🧷 The three kinds that rewrite the composed catalogue child — the ones this implementation
refuses."""

KINDS = (
    "rename-object-kind",
    "change-object-kind-label",
    "change-object-kind-variant",
    "change-object-kind-description",
    "change-object-kind-icon",
    "change-object-kind-unit",
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
    "create-vortex-kind",
    "delete-vortex-kind",
    "rename-vortex-kind",
    "change-vortex-kind-label",
    "change-vortex-kind-color",
    "change-vortex-kind-default-cable-kind",
    "create-vortex",
    "delete-vortex",
    "move-vortex",
    "resize-vortex",
    "change-vortex-vortex-kind",
    "change-vortex-label",
    "add-compatibility-rule",
    "remove-compatibility-rule",
    "add-attribute",
    "remove-attribute",
    "add-author",
    "remove-author",
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

RECORDS = {"representations": {"id", "name", "meshUrl", "tags", "lod", "description", "attributes"}, "vortexKindExtra": {"id", "label", "color", "defaultCableKind"}, "vortices": {"id", "vortexKind", "position", "direction", "radius", "label"}, "compatibility": {"id", "source", "target", "bidirectional"}}
"""🧱️ The members each id-keyed record carries, as the committed vectors spell them."""

MEASURE_MEMBERS = {"camera3d": {"position", "target", "zoom"}, "meta": {"description"}, "objectKind": {"id", "name", "label", "variant", "description", "icon", "unit"}}
"""📐️ The members each whole-value member carries."""
# endregion 🔖️Vocabulary


# region 🔖️Document
def validate(document, where):
    """✅️ Holds the document to the shape the committed vectors agree on, and to id uniqueness."""
    if set(document) != set(MEMBERS):
        raise AssertionError("%s: a block3d document must carry exactly %r, found %r" % (where, sorted(MEMBERS), sorted(document)))
    for member, expected in MEASURE_MEMBERS.items():
        if set(document[member]) != expected:
            raise AssertionError("%s: %s must carry exactly %r, found %r" % (where, member, sorted(expected), sorted(document[member])))
    if set(document["catalog"]) != {"childId", "target"}:
        raise AssertionError("%s: the composed catalogue child must carry exactly childId and target, found %r" % (where, sorted(document["catalog"])))
    for name, expected in RECORDS.items():
        identifiers = []
        for record in document[name]:
            if set(record) != expected:
                raise AssertionError("%s: a %s record must carry exactly %r, found %r" % (where, name, sorted(expected), sorted(record)))
            identifiers.append(record["id"])
        if len(set(identifiers)) != len(identifiers):
            raise AssertionError("%s: %s carries a duplicate id in %r" % (where, name, identifiers))
    for table in ("attributes",):
        keys = [entry["key"] for entry in document[table]]
        if len(set(keys)) != len(keys):
            raise AssertionError("%s: %s carries a duplicate key in %r" % (where, table, keys))


def located(rows, identity, kind, where):
    """🔎️ The index of the record this kind addresses; an absent id is an error, never a no-op —
    every committed vector of this subset declares `status: applied`."""
    for at, record in enumerate(rows):
        if record.get("id") == identity:
            return at
    raise AssertionError("%s-%s: the committed vector addresses %r, which the before-snapshot does not hold" % (where, kind, identity))
# endregion 🔖️Document


# region 🔖️Verbs
def refuse_catalog(kind):
    """🚧️ The refusal for the three kinds that rewrite the composed catalogue child."""
    raise AssertionError(
        "%s: this implementation refuses this kind rather than guessing it. It rewrites `catalog`, which a committed snapshot carries as a COMPOSED "
        "CHILD HANDLE ({childId, target}); the committed after-snapshot's new `childId` is a content address of the child `s.stdio.semio@v1/kit` "
        "document after the vortex-kind vocabulary moved, and no document in this repository states the addressing function or the child's canonical "
        "encoding. The local `vortexKindExtra` half of this kind IS stateable and is implemented above; the catalogue half is not. Publishing the "
        "child-addressing rule closes it, and no comparison profile moves." % kind
    )


def apply_mutation(document, kind, payload):
    """🦠️ Applies one kind. Every committed vector of this subset declares `status: applied`, so an
    address the document does not hold is an error rather than a rejection outcome."""
    document = copy.deepcopy(document)
    if kind in CATALOG_KINDS:
        refuse_catalog(kind)
    if kind in OBJECT_KIND_FIELDS:
        field, argument = OBJECT_KIND_FIELDS[kind]
        document["objectKind"][field] = payload[argument]
    elif kind == "create-representation":
        document["representations"].append(copy.deepcopy(payload["representation"]))
    elif kind == "delete-representation":
        document["representations"].pop(located(document["representations"], payload["id"], kind, "mutate"))
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
    elif kind in VORTEX_KIND_FIELDS:
        field, argument = VORTEX_KIND_FIELDS[kind]
        document["vortexKindExtra"][located(document["vortexKindExtra"], payload["id"], kind, "mutate")][field] = payload[argument]
    elif kind == "create-vortex":
        document["vortices"].append(copy.deepcopy(payload["vortex"]))
    elif kind == "delete-vortex":
        document["vortices"].pop(located(document["vortices"], payload["id"], kind, "mutate"))
    elif kind == "move-vortex":
        record = document["vortices"][located(document["vortices"], payload["id"], kind, "mutate")]
        record["position"] = copy.deepcopy(payload["newPosition"])
        record["direction"] = copy.deepcopy(payload["newDirection"])
    elif kind in VORTEX_FIELDS:
        field, argument = VORTEX_FIELDS[kind]
        document["vortices"][located(document["vortices"], payload["id"], kind, "mutate")][field] = payload[argument]
    elif kind == "add-compatibility-rule":
        document["compatibility"].append(copy.deepcopy(payload["rule"]))
    elif kind == "remove-compatibility-rule":
        document["compatibility"].pop(located(document["compatibility"], payload["id"], kind, "mutate"))
    elif kind == "add-attribute":
        document["attributes"].append(copy.deepcopy(payload["attribute"]))
    elif kind == "remove-attribute":
        document["attributes"] = [entry for entry in document["attributes"] if entry["key"] != payload["key"]]
    elif kind == "add-author":
        document["authors"].append(copy.deepcopy(payload["author"]))
    elif kind == "remove-author":
        document["authors"].pop(located(document["authors"], payload["id"], kind, "mutate"))
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
    if kind in CATALOG_KINDS:
        refuse_catalog(kind)
    if kind in OBJECT_KIND_FIELDS:
        field, argument = OBJECT_KIND_FIELDS[kind]
        return [(kind, {argument: document["objectKind"][field]})]
    if kind == "create-representation":
        return [("delete-representation", {"id": payload["representation"]["id"]})]
    if kind == "delete-representation":
        return [("create-representation", {"representation": copy.deepcopy(document["representations"][located(document["representations"], payload["id"], kind, "inverse")])})]
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
    if kind in VORTEX_KIND_FIELDS:
        field, argument = VORTEX_KIND_FIELDS[kind]
        return [(kind, {"id": payload["id"], argument: document["vortexKindExtra"][located(document["vortexKindExtra"], payload["id"], kind, "inverse")][field]})]
    if kind == "create-vortex":
        return [("delete-vortex", {"id": payload["vortex"]["id"]})]
    if kind == "delete-vortex":
        return [("create-vortex", {"vortex": copy.deepcopy(document["vortices"][located(document["vortices"], payload["id"], kind, "inverse")])})]
    if kind == "move-vortex":
        record = document["vortices"][located(document["vortices"], payload["id"], kind, "inverse")]
        return [(kind, {"id": payload["id"], "newPosition": copy.deepcopy(record["position"]), "newDirection": copy.deepcopy(record["direction"])})]
    if kind in VORTEX_FIELDS:
        field, argument = VORTEX_FIELDS[kind]
        return [(kind, {"id": payload["id"], argument: document["vortices"][located(document["vortices"], payload["id"], kind, "inverse")][field]})]
    if kind == "add-compatibility-rule":
        return [("remove-compatibility-rule", {"id": payload["rule"]["id"]})]
    if kind == "remove-compatibility-rule":
        return [("add-compatibility-rule", {"rule": copy.deepcopy(document["compatibility"][located(document["compatibility"], payload["id"], kind, "inverse")])})]
    if kind == "add-attribute":
        return [("remove-attribute", {"key": payload["attribute"]["key"]})]
    if kind == "remove-attribute":
        held = next(entry for entry in document["attributes"] if entry["key"] == payload["key"])
        return [("add-attribute", {"attribute": copy.deepcopy(held)})]
    if kind == "add-author":
        return [("remove-author", {"id": payload["author"]["id"]})]
    if kind == "remove-author":
        return [("add-author", {"author": copy.deepcopy(document["authors"][located(document["authors"], payload["id"], kind, "inverse")])})]
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


def declares(diff, member):
    """🔺️ Whether the committed diff really declares a member: every arm is always on the wire, so
    `null` and `[]` declare nothing while a whole-value replacement carrying an empty list does."""
    held = diff.get(member)
    if held is None:
        return False
    if isinstance(held, list):
        return len(held) > 0
    return True


DIFF_ALIASES = {"catalog": "vortexKinds", "vortexKindExtra": "vortexKinds"}
"""🔗️ The many-to-one arm this subset alone needs: the vocabulary lives in TWO snapshot members and
the diff declares BOTH through one `vortexKinds` field."""


def footprint(kind, before, after, diff):
    """⚖️ Footprint completeness: before and after differ on exactly the members the committed diff
    declares. Restated by hand here because the Python host exposes no shared `law` module."""
    changed = [member for member in MEMBERS if before[member] != after[member]]
    for member in changed:
        if not declares(diff, DIFF_ALIASES.get(member, member)):
            raise AssertionError("inverse-%s: the snapshot member %r moved without the committed diff declaring it, so an undo built from that diff would not restore it" % (kind, member))
    for member in MEMBERS:
        name = DIFF_ALIASES.get(member, member)
        if declares(diff, name) and not any(DIFF_ALIASES.get(other, other) == name and other in changed for other in MEMBERS):
            raise AssertionError("inverse-%s: the committed diff declares %r, yet every snapshot member it governs is identical in both committed snapshots" % (kind, name))


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
    after-snapshot, the declared status and observability."""

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
    """🔁️ Reads the one committed 3d document that carries both a catalogue child handle and local
    extras, and answers with the whole document. This implementation additionally requires, in role,
    that it really is that document: a catalogue child, at least two local vortex kinds, a placed
    vortex bound to a kind the document declares, and a representation."""
    uri = uri_in(ctx, "⬅️before")
    committed = ctx.fixture_bytes(uri)
    document = json.loads(committed.decode("utf-8"))
    validate(document, "identity-round-trip")
    if len(document["vortexKindExtra"]) < 2 or not document["vortices"] or not document["representations"]:
        raise AssertionError("identity-round-trip: the committed round-trip snapshot must carry a catalogue child, two local vortex kinds, a placed vortex and a representation")
    declared = {record["id"] for record in document["vortexKindExtra"]}
    for vortex in document["vortices"]:
        if vortex["vortexKind"] not in declared:
            raise AssertionError("identity-round-trip: vortex %r is bound to %r, which is not among the document's local vortex kinds — the catalogue child would have to be opened to resolve it" % (vortex["id"], vortex["vortexKind"]))
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

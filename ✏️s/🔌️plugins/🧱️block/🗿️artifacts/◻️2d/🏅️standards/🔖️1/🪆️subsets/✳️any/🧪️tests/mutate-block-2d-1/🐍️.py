#!/usr/bin/env python3
"""🧱️ An INDEPENDENT second implementation of the `s.block.block2d` node-kind document and all
twenty-six of its typed mutations, in Python, serving as this case's differential oracle.

**Why a second implementation and not a third-party library.** A `block2d` document is a KIND
DEFINITION, not an instance: one node kind's identity and presentation, the handle kinds it declares,
the handles placed on its rim by polar coordinate, the compatibility rules between handle kinds, its
attribute table, its authors, its editor camera and its metadata. Nothing outside this repository
models that — a component-library or symbol-library format (KiCad, Modelica, IFC's property sets)
carries pins or ports but has no notion of a kind-level compatibility relation between handle kinds,
of a rim angle in radians, or of the presentation variant this vocabulary switches with a single
verb; and none of them reads `.dsl.semio`. What a reference genuinely can adjudicate is this
document's own algebra — six scalar setters on the kind, a whole-value presentation swap, create and
delete plus three setters over the handle kinds, create and delete plus two setters over the handles,
and add/remove over three id- or key-keyed tables — and that is what this file implements, from the
specification, in another language.

**What it was written from.**

* ``🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json`` — the ten members of
  `Block2dSnapshot`.
* ``…/🧬️schema/🧬️mutations/🔣️.json`` and ``…/🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio``
  — the twenty-six verbs and their argument lists.
* the twenty-six committed `(before, mutation, after, outcome)` specification vectors, which give the
  internally tagged wire form of each verb and the two things only they state: that
  `update-presentation` REBUILDS the presentation from its six arguments, dropping the members whose
  argument is `null` (its vector turns a `{shape, radius, color, iconKind}` circle into a
  `{shape, width, height, color}` rectangle), and that an `author` keeps only the members it was
  given (`email` is optional).

**No Rust was read to write this.** `🦀️.rs` beside this file registers the SUBJECT half
only.
"""

# region 🔖️Imports
import copy
import json

from semio_repo_test import Adapter, Context, Outcome

# endregion 🔖️Imports


# region 🔖️Vocabulary
MEMBERS = ("schema", "nodeKind", "presentation", "handleKinds", "handles", "compatibility", "attributes", "authors", "camera2d", "meta")
"""🗂️ The ten members `Block2dSnapshot` declares — and the cross-language projection."""

KINDS = (
    "rename-node-kind",
    "change-node-kind-label",
    "change-node-kind-variant",
    "change-node-kind-description",
    "change-node-kind-icon",
    "change-node-kind-unit",
    "update-presentation",
    "create-handle-kind",
    "delete-handle-kind",
    "rename-handle-kind",
    "change-handle-kind-label",
    "change-handle-kind-color",
    "change-handle-kind-default-wire-kind",
    "create-handle",
    "delete-handle",
    "move-handle",
    "change-handle-handle-kind",
    "add-compatibility-rule",
    "remove-compatibility-rule",
    "add-attribute",
    "remove-attribute",
    "add-author",
    "remove-author",
    "move-camera2d",
    "scale-camera2d",
    "change-meta-description",
)
"""🏷️ Every kind the catalog declares, in its declared order."""


def tag_of(kind):
    """🔤️ The internally tagged `mutation` discriminator of a kind — lowerCamelCase of its words."""
    head, *rest = kind.split("-")
    return head + "".join(word[:1].upper() + word[1:] for word in rest)


TAGS = {kind: tag_of(kind) for kind in KINDS}

NODE_KIND_FIELDS = {"rename-node-kind": ("name", "newName"), "change-node-kind-label": ("label", "newLabel"), "change-node-kind-variant": ("variant", "newVariant"), "change-node-kind-description": ("description", "newDescription"), "change-node-kind-icon": ("icon", "newIcon"), "change-node-kind-unit": ("unit", "newUnit")}
"""✏️ The six kind-level scalar setters: which member each writes and what its argument is called."""

HANDLE_KIND_FIELDS = {"rename-handle-kind": ("name", "newName"), "change-handle-kind-label": ("label", "newLabel"), "change-handle-kind-color": ("color", "newColor"), "change-handle-kind-default-wire-kind": ("defaultWireKind", "newDefaultWireKind")}
"""✒️ The four handle-kind setters, all addressed by `id`."""

PRESENTATION_FIELDS = (("shape", "newShape"), ("radius", "newRadius"), ("width", "newWidth"), ("height", "newHeight"), ("color", "newColor"), ("iconKind", "newIconKind"))
"""🖌️ `update-presentation` rebuilds the presentation from these six arguments, in this order,
dropping every one whose argument is `null` — which is how its committed vector turns a circle with a
radius and an icon kind into a rectangle with a width and a height and neither."""

RECORDS = {"handleKinds": {"id", "name", "label", "color", "defaultWireKind"}, "handles": {"id", "handleKind", "angle", "radius"}, "compatibility": {"id", "source", "target", "bidirectional"}}
"""🧱️ The members each id-keyed record carries, as the committed vectors spell them."""

# endregion 🔖️Vocabulary


# region 🔖️Document
def validate(document):
    """✅️ Holds the document to the shape the committed vectors agree on, and to id uniqueness."""
    if set(document) != set(MEMBERS):
        raise AssertionError("a block2d document must carry exactly %r, found %r" % (sorted(MEMBERS), sorted(document)))
    if set(document["nodeKind"]) != {"id", "name", "label", "variant", "description", "icon", "unit"}:
        raise AssertionError("nodeKind must carry exactly its seven declared members, found %r" % sorted(document["nodeKind"]))
    if document["presentation"].get("shape") not in ("circle", "rectangle"):
        raise AssertionError("presentation must declare a circle or a rectangle, found %r" % document["presentation"].get("shape"))
    if set(document["camera2d"]) != {"x", "y", "zoom"} or set(document["meta"]) != {"description"}:
        raise AssertionError("camera2d and meta must carry exactly their declared members")
    for name, expected in RECORDS.items():
        identifiers = []
        for record in document[name]:
            if set(record) != expected:
                raise AssertionError("a %s record must carry exactly %r, found %r" % (name, sorted(expected), sorted(record)))
            identifiers.append(record["id"])
        if len(set(identifiers)) != len(identifiers):
            raise AssertionError("%s carries a duplicate id: %r" % (name, identifiers))
    for entry in document["attributes"]:
        if set(entry) - {"key", "value", "definition"} or "key" not in entry:
            raise AssertionError("an attribute must carry a key and a value, found %r" % entry)
    keys = [entry["key"] for entry in document["attributes"]]
    if len(set(keys)) != len(keys):
        raise AssertionError("attributes carries a duplicate key: %r" % keys)
    for author in document["authors"]:
        if set(author) - {"id", "name", "email"} or "id" not in author:
            raise AssertionError("an author must carry at least an id, found %r" % author)


def document_of(payload):
    """📥️ Reads a block2d document out of a snapshot JSON value."""
    document = copy.deepcopy(payload)
    validate(document)
    return document


def find(items, key, identifier):
    """🔎️ The index of a record whose `key` member equals `identifier`, or `None`."""
    for at, item in enumerate(items):
        if item.get(key) == identifier:
            return at
    return None


# endregion 🔖️Document


# region 🔖️Mutations
def kind_of(mutation):
    """🏷️ The kind an internally tagged mutation payload names."""
    if not isinstance(mutation, dict) or "mutation" not in mutation:
        raise AssertionError("a mutation carries an internally tagged `mutation` member, found %r" % mutation)
    for kind, tag in TAGS.items():
        if tag == mutation["mutation"]:
            return kind
    raise AssertionError("unknown mutation variant %r" % mutation["mutation"])


def at_or_reject(items, key, identifier, kind, what):
    """🔎️ One record's index, or a rejection — a mutation that addressed nothing is never a no-op."""
    at = find(items, key, identifier)
    if at is None:
        raise AssertionError("%s: no %s %r in the document" % (kind, what, identifier))
    return at


def apply_mutation(document, mutation):
    """🧬️ Applies one typed mutation, returning the resulting document."""
    kind = kind_of(mutation)
    result = copy.deepcopy(document)
    if kind in NODE_KIND_FIELDS:
        member, argument = NODE_KIND_FIELDS[kind]
        result["nodeKind"][member] = mutation[argument]
    elif kind == "update-presentation":
        result["presentation"] = {member: mutation[argument] for member, argument in PRESENTATION_FIELDS if mutation.get(argument) is not None}
    elif kind in HANDLE_KIND_FIELDS:
        member, argument = HANDLE_KIND_FIELDS[kind]
        result["handleKinds"][at_or_reject(result["handleKinds"], "id", mutation["id"], kind, "handle kind")][member] = mutation[argument]
    elif kind == "create-handle-kind":
        record = copy.deepcopy(mutation["handleKind"])
        if find(result["handleKinds"], "id", record["id"]) is not None:
            raise AssertionError("%s: the document already declares a handle kind %r" % (kind, record["id"]))
        result["handleKinds"].append(record)
    elif kind == "delete-handle-kind":
        result["handleKinds"].pop(at_or_reject(result["handleKinds"], "id", mutation["id"], kind, "handle kind"))
    elif kind == "create-handle":
        record = copy.deepcopy(mutation["handle"])
        if find(result["handles"], "id", record["id"]) is not None:
            raise AssertionError("%s: the document already carries a handle %r" % (kind, record["id"]))
        result["handles"].append(record)
    elif kind == "delete-handle":
        result["handles"].pop(at_or_reject(result["handles"], "id", mutation["id"], kind, "handle"))
    elif kind == "move-handle":
        handle = result["handles"][at_or_reject(result["handles"], "id", mutation["id"], kind, "handle")]
        handle["angle"], handle["radius"] = float(mutation["newAngle"]), float(mutation["newRadius"])
    elif kind == "change-handle-handle-kind":
        result["handles"][at_or_reject(result["handles"], "id", mutation["id"], kind, "handle")]["handleKind"] = mutation["newHandleKind"]
    elif kind == "add-compatibility-rule":
        rule = copy.deepcopy(mutation["rule"])
        if find(result["compatibility"], "id", rule["id"]) is not None:
            raise AssertionError("%s: the document already carries a rule %r" % (kind, rule["id"]))
        result["compatibility"].append(rule)
    elif kind == "remove-compatibility-rule":
        result["compatibility"].pop(at_or_reject(result["compatibility"], "id", mutation["id"], kind, "compatibility rule"))
    elif kind == "add-attribute":
        entry = copy.deepcopy(mutation["attribute"])
        if find(result["attributes"], "key", entry["key"]) is not None:
            raise AssertionError("%s: the document already carries an attribute %r" % (kind, entry["key"]))
        result["attributes"].append(entry)
    elif kind == "remove-attribute":
        result["attributes"].pop(at_or_reject(result["attributes"], "key", mutation["key"], kind, "attribute"))
    elif kind == "add-author":
        author = copy.deepcopy(mutation["author"])
        if find(result["authors"], "id", author["id"]) is not None:
            raise AssertionError("%s: the document already credits %r" % (kind, author["id"]))
        result["authors"].append(author)
    elif kind == "remove-author":
        result["authors"].pop(at_or_reject(result["authors"], "id", mutation["id"], kind, "author"))
    elif kind == "move-camera2d":
        result["camera2d"]["x"], result["camera2d"]["y"] = float(mutation["newX"]), float(mutation["newY"])
    elif kind == "scale-camera2d":
        result["camera2d"]["zoom"] = float(mutation["newZoom"])
    else:
        result["meta"]["description"] = mutation["newDescription"]
    validate(result)
    return result


def inverse_mutation(document, mutation):
    """↩️ The mutation that undoes one application, computed against the document it applies to.

    No `create-`/`add-` verb in this vocabulary carries an index, so the inverse of a delete is exact
    only for a TRAILING record — the feature's rows are chosen accordingly and say so.
    """
    kind = kind_of(mutation)
    if kind in NODE_KIND_FIELDS:
        member, argument = NODE_KIND_FIELDS[kind]
        return {"mutation": TAGS[kind], argument: document["nodeKind"][member]}
    if kind == "update-presentation":
        held = document["presentation"]
        payload = {"mutation": TAGS[kind]}
        for member, argument in PRESENTATION_FIELDS:
            payload[argument] = held.get(member)
        return payload
    if kind in HANDLE_KIND_FIELDS:
        member, argument = HANDLE_KIND_FIELDS[kind]
        at = at_or_reject(document["handleKinds"], "id", mutation["id"], "inverse of %s" % kind, "handle kind")
        return {"mutation": TAGS[kind], "id": mutation["id"], argument: document["handleKinds"][at][member]}
    if kind == "create-handle-kind":
        return {"mutation": TAGS["delete-handle-kind"], "id": mutation["handleKind"]["id"]}
    if kind == "delete-handle-kind":
        at = at_or_reject(document["handleKinds"], "id", mutation["id"], "inverse of %s" % kind, "handle kind")
        return {"mutation": TAGS["create-handle-kind"], "handleKind": copy.deepcopy(document["handleKinds"][at])}
    if kind == "create-handle":
        return {"mutation": TAGS["delete-handle"], "id": mutation["handle"]["id"]}
    if kind == "delete-handle":
        at = at_or_reject(document["handles"], "id", mutation["id"], "inverse of %s" % kind, "handle")
        return {"mutation": TAGS["create-handle"], "handle": copy.deepcopy(document["handles"][at])}
    if kind == "move-handle":
        handle = document["handles"][at_or_reject(document["handles"], "id", mutation["id"], "inverse of %s" % kind, "handle")]
        return {"mutation": TAGS[kind], "id": mutation["id"], "newAngle": handle["angle"], "newRadius": handle["radius"]}
    if kind == "change-handle-handle-kind":
        handle = document["handles"][at_or_reject(document["handles"], "id", mutation["id"], "inverse of %s" % kind, "handle")]
        return {"mutation": TAGS[kind], "id": mutation["id"], "newHandleKind": handle["handleKind"]}
    if kind == "add-compatibility-rule":
        return {"mutation": TAGS["remove-compatibility-rule"], "id": mutation["rule"]["id"]}
    if kind == "remove-compatibility-rule":
        at = at_or_reject(document["compatibility"], "id", mutation["id"], "inverse of %s" % kind, "compatibility rule")
        return {"mutation": TAGS["add-compatibility-rule"], "rule": copy.deepcopy(document["compatibility"][at])}
    if kind == "add-attribute":
        return {"mutation": TAGS["remove-attribute"], "key": mutation["attribute"]["key"]}
    if kind == "remove-attribute":
        at = at_or_reject(document["attributes"], "key", mutation["key"], "inverse of %s" % kind, "attribute")
        return {"mutation": TAGS["add-attribute"], "attribute": copy.deepcopy(document["attributes"][at])}
    if kind == "add-author":
        return {"mutation": TAGS["remove-author"], "id": mutation["author"]["id"]}
    if kind == "remove-author":
        at = at_or_reject(document["authors"], "id", mutation["id"], "inverse of %s" % kind, "author")
        return {"mutation": TAGS["add-author"], "author": copy.deepcopy(document["authors"][at])}
    if kind == "move-camera2d":
        return {"mutation": TAGS[kind], "newX": document["camera2d"]["x"], "newY": document["camera2d"]["y"]}
    if kind == "scale-camera2d":
        return {"mutation": TAGS[kind], "newZoom": document["camera2d"]["zoom"]}
    return {"mutation": TAGS[kind], "newDescription": document["meta"]["description"]}


# endregion 🔖️Mutations


# region 🔖️Laws
WRITES = {
    "rename-node-kind": "nodeKind",
    "change-node-kind-label": "nodeKind",
    "change-node-kind-variant": "nodeKind",
    "change-node-kind-description": "nodeKind",
    "change-node-kind-icon": "nodeKind",
    "change-node-kind-unit": "nodeKind",
    "update-presentation": "presentation",
    "create-handle-kind": "handleKinds",
    "delete-handle-kind": "handleKinds",
    "rename-handle-kind": "handleKinds",
    "change-handle-kind-label": "handleKinds",
    "change-handle-kind-color": "handleKinds",
    "change-handle-kind-default-wire-kind": "handleKinds",
    "create-handle": "handles",
    "delete-handle": "handles",
    "move-handle": "handles",
    "change-handle-handle-kind": "handles",
    "add-compatibility-rule": "compatibility",
    "remove-compatibility-rule": "compatibility",
    "add-attribute": "attributes",
    "remove-attribute": "attributes",
    "add-author": "authors",
    "remove-author": "authors",
    "move-camera2d": "camera2d",
    "scale-camera2d": "camera2d",
    "change-meta-description": "meta",
}
"""🗂️ The one member each verb writes, spelled out rather than derived from the kind's suffix: three
of the twenty-six share a suffix with a kind that writes a different member —
`change-handle-handle-kind` writes `handles`, not `handleKinds` — and a suffix rule that got one of
them wrong would quietly weaken the very check this table exists to make."""


def written_member(kind):
    """🗂️ The one member a verb writes."""
    return WRITES[kind]


def observable(scenario, before, after):
    """👁️ Every row below writes a value the document does not already hold, so a forward application
    must move it. A setter that quietly did nothing would otherwise pass by agreeing."""
    if before == after:
        raise AssertionError("%s: the forward mutation left the document untouched, so nothing was proved" % scenario)


def touches_one(scenario, kind, before, after):
    """🔀️ Each verb writes exactly ONE of the ten members. That is the check an after-snapshot
    comparison cannot make on its own: an implementation that re-derived a sibling table on every
    edit — renumbering handles, re-sorting handle kinds — would still land on the right value for the
    member it meant to write."""
    written = written_member(kind)
    moved = [name for name in MEMBERS if before[name] != after[name]]
    if moved != [written]:
        raise AssertionError("%s: this verb writes %s and nothing else, but %r moved" % (scenario, written, moved))


def restores(kind, restored, original):
    """↩️ The metamorphic inverse law, reported by the member and index that failed to come back."""
    if restored == original:
        return
    for name in MEMBERS:
        if restored[name] == original[name]:
            continue
        if not isinstance(original[name], list):
            raise AssertionError("inverse-%s: %s came back as %s, not %s" % (kind, name, json.dumps(restored[name], sort_keys=True), json.dumps(original[name], sort_keys=True)))
        key = "key" if name == "attributes" else "id"
        was = [record[key] for record in original[name]]
        now = [record[key] for record in restored[name]]
        if was != now:
            raise AssertionError("inverse-%s: %s came back as %r, not %r" % (kind, name, now, was))
        for at, (left, right) in enumerate(zip(original[name], restored[name])):
            if left != right:
                raise AssertionError("inverse-%s: %s[%d] (%s) came back as %s, not %s" % (kind, name, at, left[key], json.dumps(right, sort_keys=True), json.dumps(left, sort_keys=True)))


def equals_committed(kind, produced, committed):
    """🎯️ The committed after-snapshot claim, member by member."""
    for name in MEMBERS:
        if produced[name] != committed[name]:
            raise AssertionError("spec-vector-%s: %s is %s, the committed after-snapshot says %s" % (kind, name, json.dumps(produced[name], sort_keys=True)[:300], json.dumps(committed[name], sort_keys=True)[:300]))


# endregion 🔖️Laws


# region 🔖️Plan
def doc_string(ctx):
    """📜️ The scenario's doc string — the Python `Context` has no accessor of its own."""
    for step in ctx.scenario["steps"]:
        if step.get("docString"):
            return step["docString"]
    raise AssertionError("scenario %s carries no doc string" % ctx.scenario["id"])


def uri_in(ctx, needle):
    """🧫️ The one declared fixture URI of this scenario's steps containing `needle`."""
    for step in ctx.scenario["steps"]:
        for token in step["text"].split():
            if token.startswith(("asset://", "local://", "shared://")) and needle in token:
                return token
    raise AssertionError("scenario %s declares no fixture URI containing %r" % (ctx.scenario["id"], needle))


def json_fixture(ctx, needle):
    """🧫️ The declared JSON fixture this scenario names."""
    return json.loads(ctx.fixture_bytes(uri_in(ctx, needle)).decode("utf-8"))


def outcome_of(payload):
    """📤️ Wraps a projection with its own compact serialization as the raw artifact."""
    return Outcome(payload, raw=json.dumps(payload, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))


# endregion 🔖️Plan


# region 🔖️Handlers
def mutate_handler(kind):
    """🎯️ Applies one kind to the real derived Hexagonal Cut Concrete Forest Left node kind."""

    def handler(ctx):
        document = document_of(json_fixture(ctx, "hexagonal-cut-concrete-forest-left"))
        mutation = json.loads(doc_string(ctx))
        if kind_of(mutation) != kind:
            raise AssertionError("mutate-%s: the feature states a %s payload" % (kind, kind_of(mutation)))
        applied = apply_mutation(document, mutation)
        observable("mutate-%s" % kind, document, applied)
        touches_one("mutate-%s" % kind, kind, document, applied)
        return outcome_of(applied)

    return handler


def inverse_handler(kind):
    """↩️ Applies one kind to the real derived document and then its OWN computed inverse.

    The projection carries BOTH documents; projecting only the restored one would make all
    twenty-six rows project the same value and the differential would be vacuous.
    """

    def handler(ctx):
        document = document_of(json_fixture(ctx, "hexagonal-cut-concrete-forest-left"))
        mutation = json.loads(doc_string(ctx))
        if kind_of(mutation) != kind:
            raise AssertionError("inverse-%s: the feature states a %s payload" % (kind, kind_of(mutation)))
        applied = apply_mutation(document, mutation)
        observable("inverse-%s" % kind, document, applied)
        restored = apply_mutation(applied, inverse_mutation(document, mutation))
        restores(kind, restored, document)
        return outcome_of({"mutated": applied, "restored": restored})

    return handler


def spec_vector_handler(kind):
    """📐️ Replays the committed handcrafted `(before, mutation, after)` triple for one kind."""

    def handler(ctx):
        before = document_of(json_fixture(ctx, "⬅️before"))
        mutation = json_fixture(ctx, "🦠️mutation")
        after = document_of(json_fixture(ctx, "➡️after"))
        if kind_of(mutation) != kind:
            raise AssertionError("spec-vector-%s: the committed vector carries a %s payload" % (kind, kind_of(mutation)))
        applied = apply_mutation(before, mutation)
        equals_committed(kind, applied, after)
        observable("spec-vector-%s" % kind, before, applied)
        touches_one("spec-vector-%s" % kind, kind, before, applied)
        restores(kind, apply_mutation(applied, inverse_mutation(before, mutation)), before)
        return outcome_of(applied)

    return handler


def identity_handler(ctx):
    """🔁️ Reads the derived real node kind and answers with the whole document.

    This implementation additionally requires, in role, that the document really is the committed
    kind: eleven handles all at the same 0.36 rim radius, every one of them bound to a handle kind the
    document declares, and every compatibility rule naming declared kinds on both sides. A codec that
    dropped a table could not satisfy it.
    """
    document = document_of(json_fixture(ctx, "hexagonal-cut-concrete-forest-left"))
    kinds = {record["id"] for record in document["handleKinds"]}
    if len(document["handles"]) != 11:
        raise AssertionError("identity-round-trip: the committed kind carries eleven handles, read %d" % len(document["handles"]))
    radii = {handle["radius"] for handle in document["handles"]}
    if radii != {0.36}:
        raise AssertionError("identity-round-trip: every committed handle sits on the same 0.36 rim, found radii %r" % sorted(radii))
    for handle in document["handles"]:
        if handle["handleKind"] not in kinds:
            raise AssertionError("identity-round-trip: handle %r is bound to %r, which the document does not declare" % (handle["id"], handle["handleKind"]))
    for rule in document["compatibility"]:
        for end in ("source", "target"):
            if rule[end] not in kinds:
                raise AssertionError("identity-round-trip: rule %r names %s %r, which the document does not declare" % (rule["id"], end, rule[end]))
    reread = document_of(json.loads(json.dumps(document)))
    if reread != document:
        raise AssertionError("identity-round-trip: serializing and re-reading the document moved it")
    return outcome_of(document)


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
        built = built.oracle("spec-vector-%s" % kind, spec_vector_handler(kind))
    return built.oracle("identity-round-trip", identity_handler)


# endregion 🔖️Registration

#!/usr/bin/env python3
"""🖨️ An INDEPENDENT second implementation of the `s.raster.raster` layered document and all twelve
of its typed mutations, in Python, serving as this case's differential oracle.

**Why a second implementation and not a third-party library.** `s.raster.raster` is a LAYERED
DOCUMENT, not an image file. Its pixels live behind ids in a root `assets` pool and its vocabulary
edits the layer TREE around them. `png`, `image` and `tiff` — the raster crates this repository
already links — read a different artifact entirely, and Pillow, which really does read the pixel
payloads, has nothing whatsoever to say about a group's `children`, a layer's blend mode or a
reorder that lifts a node out of a group. Registering one of them here would be a category error
rather than an oracle. What a reference can adjudicate is the tree algebra — insert-under-parent-at-
index, delete-with-subtree, move-between-parents, per-node field edits, and the root asset pool —
and that is what this file implements, from the specification, in another language.

**What it was written from.**

* ``🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🧬️mutations/🔣️.json`` — all twelve variants,
  their INTERNALLY tagged wire form (`{"mutation": "createLayer", "parentId": …, "index": …,
  "layer": …}`) and each one's required members, `additionalProperties: false`.
* the twelve committed `(before, mutation, after, outcome)` specification vectors, which are where
  the document's own shape is actually written down: a document is `{schema, id, title, layers,
  assets}`; a layer node is one of three kinds over a shared base of `id`, `name`, `visible`,
  `opacity`, `blendMode` and `transform` — `group` adds `mask` and `children`, `pixel` adds `mask`,
  `width`, `height` and `imageKey`, and `adjustment` adds `adjustmentKind` and `params` and carries
  no mask at all; and `assets` maps an asset id to a composed `s.stdio.semio@v1/image` child handle.
* ``…/🧬️schema/🧬️mutations/📝️text/📖️.grammar.semio`` — the twelve verbs and their
  positional argument lists.

**A defect in the specification, found while writing this and reported rather than worked around.**
``…/🧬️schema/📸️snapshot/🔣️.json`` does NOT describe this artifact: it is a verbatim copy of
`s.stdio.json`'s `JsonSnapshot` schema, `{schema, value}`, with the wrong `$id`. The mutation schema
points at it for `RasterLayerNode` and therefore points at nothing. The document shape above was
read off the twelve committed vectors instead, which agree with each other on every field.

**No Rust was read to write this.** `🦀️.rs` beside this file registers the SUBJECT half
only.
"""

# region 🔖️Imports
import copy
import json

from semio_repo_test import Adapter, Context, Outcome

# endregion 🔖️Imports


# region 🔖️Vocabulary
MEMBERS = ("schema", "id", "title", "layers", "assets")
"""🗂️ The five members of a raster document — and the cross-language projection."""

KINDS = (
    "create-layer",
    "delete-layer",
    "reorder-layers",
    "rename-layer",
    "change-layer-visible",
    "change-layer-opacity",
    "change-layer-blend-mode",
    "move-layer",
    "resize-layer",
    "change-layer-adjustment-kind",
    "add-layer-asset",
    "remove-layer-asset",
)
"""🏷️ Every kind the catalog declares."""

TAGS = {
    "create-layer": "createLayer",
    "delete-layer": "deleteLayer",
    "reorder-layers": "reorderLayers",
    "rename-layer": "renameLayer",
    "change-layer-visible": "changeLayerVisible",
    "change-layer-opacity": "changeLayerOpacity",
    "change-layer-blend-mode": "changeLayerBlendMode",
    "move-layer": "moveLayer",
    "resize-layer": "resizeLayer",
    "change-layer-adjustment-kind": "changeLayerAdjustmentKind",
    "add-layer-asset": "addLayerAsset",
    "remove-layer-asset": "removeLayerAsset",
}
"""🔤️ The internally tagged `mutation` discriminator of each kind, as the committed schema spells it."""

BASE = ("kind", "id", "name", "visible", "opacity", "blendMode", "transform")
"""🧱️ The members every layer node carries, whatever its kind."""

EXTRA = {"group": ("mask", "children"), "pixel": ("mask", "width", "height", "imageKey"), "adjustment": ("adjustmentKind", "params")}
"""🧱️ What each node kind adds on top of the shared base."""

# endregion 🔖️Vocabulary


# region 🔖️Document
def validate_node(node, path):
    """✅️ Holds one layer node to the shape the twelve committed vectors agree on."""
    if not isinstance(node, dict) or node.get("kind") not in EXTRA:
        raise AssertionError("%s must be one of %r, found %r" % (path, sorted(EXTRA), node.get("kind") if isinstance(node, dict) else node))
    expected = set(BASE) | set(EXTRA[node["kind"]])
    if set(node) != expected:
        raise AssertionError("%s must carry exactly %r, found %r" % (path, sorted(expected), sorted(node)))
    if set(node["transform"]) != {"x", "y", "scaleX", "scaleY", "rotation"}:
        raise AssertionError("%s.transform must carry exactly the five declared members, found %r" % (path, sorted(node["transform"])))
    if node["kind"] == "group":
        for at, child in enumerate(node["children"]):
            validate_node(child, "%s.children[%d]" % (path, at))


def validate(document):
    """✅️ Holds the whole document to that shape, and to unique layer ids across the whole tree."""
    if set(document) != set(MEMBERS):
        raise AssertionError("a raster document must carry exactly %r, found %r" % (sorted(MEMBERS), sorted(document)))
    for at, node in enumerate(document["layers"]):
        validate_node(node, "layers[%d]" % at)
    identifiers = [node["id"] for node, _parent, _at in walk(document)]
    if len(set(identifiers)) != len(identifiers):
        raise AssertionError("the layer tree carries a duplicate id: %r" % identifiers)
    for key, handle in document["assets"].items():
        if set(handle) != {"childId", "target"}:
            raise AssertionError("asset %r must be a composed child handle, found %r" % (key, sorted(handle)))


def document_of(payload):
    """📥️ Reads a raster document out of a snapshot JSON value.

    An ABSENT `assets` member is the empty pool: ten of the twelve committed vectors omit it
    entirely and the two that exercise the pool spell it out, which is how the vectors say the
    member is defaulted rather than required.
    """
    document = copy.deepcopy(payload)
    document.setdefault("assets", {})
    validate(document)
    return document


def siblings(document, parent_id):
    """🌳️ The child list a `parentId` names — the root layer list when it is `null`."""
    if parent_id is None:
        return document["layers"]
    for node, _parent, _at in walk(document):
        if node["id"] == parent_id:
            if node["kind"] != "group":
                raise AssertionError("%r is a %s layer and cannot hold children" % (parent_id, node["kind"]))
            return node["children"]
    raise AssertionError("no layer %r to hold children" % parent_id)


def walk(document, nodes=None, parent=None):
    """🌳️ Every node of the tree as `(node, parentId, index)`, parents before children."""
    for at, node in enumerate(document["layers"] if nodes is None else nodes):
        yield node, parent, at
        if node["kind"] == "group":
            for found in walk(document, node["children"], node["id"]):
                yield found


def locate(document, layer_id):
    """🔎️ The `(node, parentId, index)` of one layer, or `None` when the tree does not hold it."""
    for node, parent, at in walk(document):
        if node["id"] == layer_id:
            return node, parent, at
    return None


def node_of(document, layer_id, kind):
    """🔎️ One layer, or a rejection — a mutation that addressed nothing must never be a silent no-op."""
    found = locate(document, layer_id)
    if found is None:
        raise AssertionError("%s: no layer %r in the document" % (kind, layer_id))
    return found


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


def apply_mutation(document, mutation):
    """🧬️ Applies one typed mutation, returning the resulting document.

    Attaching an asset the document ALREADY holds is a no-op that returns the document untouched,
    not a rejection: the committed `add-layer-asset` vector declares `status: "applied"` with a
    `mutation.no-op` warning for exactly that case. Removing one the document never held IS a
    rejection — its committed vector declares `status: "rejected"`, `mutation.target-missing`. Every
    other rejection below is an error rather than a silent no-op.
    """
    kind = kind_of(mutation)
    result = copy.deepcopy(document)
    if kind == "create-layer":
        node = copy.deepcopy(mutation["layer"])
        if locate(result, node["id"]) is not None:
            raise AssertionError("%s: %r is already in the layer tree" % (kind, node["id"]))
        into = siblings(result, mutation["parentId"])
        at = int(mutation["index"])
        if at < 0 or at > len(into):
            raise AssertionError("%s: index %d is outside 0..=%d" % (kind, at, len(into)))
        into.insert(at, node)
    elif kind == "delete-layer":
        node, parent, at = node_of(result, mutation["layerId"], kind)
        siblings(result, parent).pop(at)
    elif kind == "reorder-layers":
        node, parent, at = node_of(result, mutation["layerId"], kind)
        target = mutation.get("parentId")
        if target is not None and locate(result, target) is None:
            raise AssertionError("%s: no layer %r to move into" % (kind, target))
        if node["kind"] == "group" and target is not None and any(inner["id"] == target for inner, _p, _a in walk(result, node["children"], node["id"])):
            raise AssertionError("%s: %r cannot be moved inside its own subtree" % (kind, node["id"]))
        siblings(result, parent).pop(at)
        into = siblings(result, target)
        to = int(mutation["index"])
        if to < 0 or to > len(into):
            raise AssertionError("%s: index %d is outside 0..=%d" % (kind, to, len(into)))
        into.insert(to, node)
    elif kind == "add-layer-asset":
        key = mutation["assetId"]
        if key in result["assets"]:
            return result
        result["assets"][key] = copy.deepcopy(mutation["asset"])
    elif kind == "remove-layer-asset":
        key = mutation["assetId"]
        if key not in result["assets"]:
            raise AssertionError("%s: %r is not attached to the document" % (kind, key))
        del result["assets"][key]
    else:
        node, _parent, _at = node_of(result, mutation["layerId"], kind)
        if kind == "rename-layer":
            node["name"] = mutation["newName"]
        elif kind == "change-layer-visible":
            node["visible"] = mutation["newVisible"]
        elif kind == "change-layer-opacity":
            node["opacity"] = float(mutation["newOpacity"])
        elif kind == "change-layer-blend-mode":
            node["blendMode"] = mutation["newBlendMode"]
        elif kind == "move-layer":
            node["transform"]["x"] = float(mutation["newX"])
            node["transform"]["y"] = float(mutation["newY"])
        elif kind == "resize-layer":
            if node["kind"] != "pixel":
                raise AssertionError("%s: %r is a %s layer and carries no size" % (kind, node["id"], node["kind"]))
            node["width"] = int(mutation["newWidth"])
            node["height"] = int(mutation["newHeight"])
        else:
            if node["kind"] != "adjustment":
                raise AssertionError("%s: %r is a %s layer and carries no adjustment kind" % (kind, node["id"], node["kind"]))
            node["adjustmentKind"] = mutation["newAdjustmentKind"]
    validate(result)
    return result


def inverse_mutation(document, mutation):
    """↩️ The mutation that undoes one application, computed against the document it applies to."""
    kind = kind_of(mutation)
    if kind == "create-layer":
        return {"mutation": TAGS["delete-layer"], "layerId": mutation["layer"]["id"]}
    if kind == "delete-layer":
        node, parent, at = node_of(document, mutation["layerId"], "inverse of %s" % kind)
        return {"mutation": TAGS["create-layer"], "parentId": parent, "index": at, "layer": copy.deepcopy(node)}
    if kind == "reorder-layers":
        node, parent, at = node_of(document, mutation["layerId"], "inverse of %s" % kind)
        return {"mutation": TAGS["reorder-layers"], "layerId": node["id"], "parentId": parent, "index": at}
    if kind == "add-layer-asset":
        return {"mutation": TAGS["remove-layer-asset"], "assetId": mutation["assetId"]}
    if kind == "remove-layer-asset":
        key = mutation["assetId"]
        if key not in document["assets"]:
            raise AssertionError("inverse of %s: %r is not attached to the document" % (kind, key))
        return {"mutation": TAGS["add-layer-asset"], "assetId": key, "asset": copy.deepcopy(document["assets"][key])}
    node, _parent, _at = node_of(document, mutation["layerId"], "inverse of %s" % kind)
    if kind == "rename-layer":
        return {"mutation": TAGS[kind], "layerId": node["id"], "newName": node["name"]}
    if kind == "change-layer-visible":
        return {"mutation": TAGS[kind], "layerId": node["id"], "newVisible": node["visible"]}
    if kind == "change-layer-opacity":
        return {"mutation": TAGS[kind], "layerId": node["id"], "newOpacity": node["opacity"]}
    if kind == "change-layer-blend-mode":
        return {"mutation": TAGS[kind], "layerId": node["id"], "newBlendMode": node["blendMode"]}
    if kind == "move-layer":
        return {"mutation": TAGS[kind], "layerId": node["id"], "newX": node["transform"]["x"], "newY": node["transform"]["y"]}
    if kind == "resize-layer":
        return {"mutation": TAGS[kind], "layerId": node["id"], "newWidth": node["width"], "newHeight": node["height"]}
    return {"mutation": TAGS[kind], "layerId": node["id"], "newAdjustmentKind": node["adjustmentKind"]}


# endregion 🔖️Mutations


# region 🔖️Laws
def observable(scenario, before, after):
    """👁️ Every kind here edits the tree or the asset pool, so a forward application must move the
    document. A mutation that quietly did nothing would otherwise agree with an unchanged one."""
    if before == after:
        raise AssertionError("%s: the forward mutation left the document untouched, so nothing was proved" % scenario)


def restores(kind, restored, original):
    """↩️ The metamorphic inverse law, reported by the first member and layer address that diverges."""
    if restored == original:
        return
    for name in MEMBERS:
        if restored[name] == original[name]:
            continue
        if name != "layers":
            raise AssertionError("inverse-%s: %s came back as %s, not %s" % (kind, name, json.dumps(restored[name], sort_keys=True), json.dumps(original[name], sort_keys=True)))
        was = [(node["id"], parent, at) for node, parent, at in walk(original)]
        now = [(node["id"], parent, at) for node, parent, at in walk(restored)]
        if was != now:
            raise AssertionError("inverse-%s: the layer tree came back as %r, not %r" % (kind, now, was))
        raise AssertionError("inverse-%s: the layer tree kept its shape but a node's fields did not come back" % kind)


def equals_committed(kind, produced, committed):
    """🎯️ The committed after-document claim, member by member."""
    for name in MEMBERS:
        if produced[name] != committed[name]:
            raise AssertionError("spec-vector-%s: %s is %s, the committed after-document says %s" % (kind, name, json.dumps(produced[name], sort_keys=True), json.dumps(committed[name], sort_keys=True)))


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
    """🎯️ Applies one kind to the real derived Semio demo board."""

    def handler(ctx):
        document = document_of(json_fixture(ctx, "🔣️.snapshot.json"))
        mutation = json.loads(doc_string(ctx))
        if kind_of(mutation) != kind:
            raise AssertionError("mutate-%s: the feature states a %s payload" % (kind, kind_of(mutation)))
        applied = apply_mutation(document, mutation)
        observable("mutate-%s" % kind, document, applied)
        return outcome_of(applied)

    return handler


def inverse_handler(kind):
    """↩️ Applies one kind to the real derived board and then its OWN computed inverse.

    The projection carries BOTH documents; projecting only the restored one would make all twelve
    rows project the same value and the differential would be vacuous.
    """

    def handler(ctx):
        document = document_of(json_fixture(ctx, "🔣️.snapshot.json"))
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
    """📐️ Replays the committed handcrafted `(before, mutation, after)` triple for one kind.

    Two of the twelve committed vectors are NEGATIVE — `add-layer-asset` re-attaches an asset the
    document already holds, which its outcome declares an `applied` NO-OP, and `remove-layer-asset`
    names one it never attached, which its outcome declares REJECTED. The feature's `verdict` column
    states which of the three answers each vector commits to, and this handler requires exactly that:
    an `applied` vector must reach the committed after-document and move it, a `noop` vector must
    reach it without moving it, and a `refused` one must be refused and leave the document alone.
    """

    def handler(ctx):
        before = document_of(json_fixture(ctx, "⬅️before"))
        mutation = json_fixture(ctx, "🦠️mutation")
        after = document_of(json_fixture(ctx, "➡️after"))
        verdict = json.loads(doc_string(ctx))["verdict"]
        if kind_of(mutation) != kind:
            raise AssertionError("spec-vector-%s: the committed vector carries a %s payload" % (kind, kind_of(mutation)))
        if verdict == "refused":
            try:
                apply_mutation(before, mutation)
            except AssertionError:
                equals_committed(kind, before, after)
                return outcome_of(before)
            raise AssertionError("spec-vector-%s: the committed vector declares a refusal, but the mutation applied" % kind)
        applied = apply_mutation(before, mutation)
        equals_committed(kind, applied, after)
        if verdict == "noop":
            if applied != before:
                raise AssertionError("spec-vector-%s: the committed vector declares a no-op, but the document moved" % kind)
            return outcome_of(applied)
        observable("spec-vector-%s" % kind, before, applied)
        restores(kind, apply_mutation(applied, inverse_mutation(before, mutation)), before)
        return outcome_of(applied)

    return handler


def identity_handler(ctx):
    """🔁️ Reads the derived real board and answers with the whole document.

    This implementation additionally requires, in role, that the board really is the committed demo:
    the 1024×1024 `backdrop` layer bound to the `semio-emblem` key, the `brightnessContrast`
    adjustment with its committed parameters, and an asset pool whose every key is bound by some
    pixel layer — a document a codec that dropped the pool or the tree could not satisfy. The
    `.dsl.semio` carrier's own laws are asserted in role on the Rust side, against the artifact's
    committed example.
    """
    document = document_of(json_fixture(ctx, "🔣️.snapshot.json"))
    by_id = {node["id"]: node for node, _parent, _at in walk(document)}
    backdrop = by_id.get("backdrop")
    if backdrop is None or (backdrop["width"], backdrop["height"], backdrop["imageKey"]) != (1024, 1024, "semio-emblem"):
        raise AssertionError("identity-round-trip: the committed demo binds a 1024×1024 backdrop to `semio-emblem`, found %r" % backdrop)
    brighten = by_id.get("brighten")
    if brighten is None or brighten["adjustmentKind"] != "brightnessContrast" or sorted(brighten["params"]) != ["brightness", "contrast"]:
        raise AssertionError("identity-round-trip: the committed demo carries a brightnessContrast adjustment, found %r" % brighten)
    bound = {node.get("imageKey") for node, _parent, _at in walk(document)}
    dangling = [key for key in document["assets"] if key not in bound]
    if dangling:
        raise AssertionError("identity-round-trip: the asset pool holds %r, which no layer binds" % dangling)
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

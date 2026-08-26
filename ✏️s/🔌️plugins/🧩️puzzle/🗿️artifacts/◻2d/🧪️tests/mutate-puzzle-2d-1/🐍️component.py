#!/usr/bin/env python3
"""🧩️ An INDEPENDENT second implementation of the `s.puzzle.2d` board document and its twenty-six
typed mutations, in Python, serving as this case's differential oracle.

**Why a second implementation and not a third-party library.** A `puzzle2d` document is a BOARD: nodes
carrying their own handles, edges that join two HANDLES rather than two nodes, and a metadata block
holding a manifest id, a kind-compatibility relation and an optional kind catalogue. No graph
interchange format models an edge whose endpoints are ports OWNED BY a node — GraphML, DOT and
GEXF all join node to node — and none of them reads `.dsl.semio`. What a reference genuinely can
adjudicate is this document's own algebra, and that the algebra IS adjudicable was settled in this
same wave by `mutate-fem2d-1` and `mutate-gismap-1`, which took Python second implementations over
this same carrier.

**What it was written from.**

* ``🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️component.json`` — the five members of
  `Puzzle2dSnapshot`.
* the twenty-six committed payloads themselves, for the verbs and their argument lists — NOT
  ``…/🧬️schema/🧬️mutations/🔣️component.json``, which despite its title `Puzzle2dMutation` is a copy of
  the SNAPSHOT schema (`{schema, camera, nodes, edges, meta}`) and declares no mutation at all. That
  file is the pre-migration whole-snapshot-shaped generic schema `s.architect.program`'s own mutation
  schema records itself as superseding; here it was never replaced.
* rules 2, 4 and 7 of
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md` — the
  id-keyed collections, `connect`/`disconnect` for the edge collection and for the compatibility
  relation, and absolute `move`/`scale`.
* the twenty-six committed `(before, mutation, diff, outcome, after)` quintets, which are the only
  statement of four things: that a node's handle list is INSIDE the node rather than a register of its
  own, so removing a handle is a node mutation that CASCADES into the edges attached to it; that
  deleting a node severs every edge attached to any of ITS handles; that `replace-node-geometry`
  rebuilds shape and extent from its four arguments and drops every member whose argument is `null`;
  and that a member equal to its default is OMITTED from the carrier rather than written.

**No Rust was read to write this.** `🦀️component.rs` beside this file registers the SUBJECT half
only.

**One kind this implementation REFUSES, by clause rather than by absence.** See `UNDERDETERMINED`.
"""

# region 🔖️Imports
import copy
import json

from semio_repo_test import Adapter, Outcome

# endregion 🔖️Imports


# region 🔖️Vocabulary
MEMBERS = ("schema", "camera", "nodes", "edges", "meta")
"""🗂️ The five members `Puzzle2dSnapshot` declares — and the cross-language projection."""

DEFAULTS = {"visible": True, "locked": False, "root": False, "scale": 1.0}
"""🫥 The four members a committed snapshot OMITS when they hold their default. No committed document
states these values; the vectors state them indirectly and completely — each of the four appears in an
after-snapshot only when a verb sets it to the OTHER value, and the inverse scenario then requires the
member to disappear again, which pins the default from both sides."""

NODE_FIELDS = {"change-node-kind": ("nodeKind", "newNodeKind"), "edit-node-text": ("text", "newText"), "change-node-icon": ("iconKind", "newIconKind"), "change-node-anchor": ("anchor", "newAnchor"), "change-node-visible": ("visible", "newVisible"), "change-node-locked": ("locked", "newLocked"), "change-node-root": ("root", "newRoot"), "scale-node": ("scale", "newScale")}
"""✏️ The eight single-field node setters."""

EDGE_FIELDS = {"change-edge-kind": ("edgeKind", "newEdgeKind"), "change-edge-visible": ("visible", "newVisible"), "change-edge-locked": ("locked", "newLocked")}
"""🔗 The three single-field edge setters."""

NODE_GEOMETRY = (("shape", "newShape"), ("radius", "newRadius"), ("width", "newWidth"), ("height", "newHeight"))
"""🧊 `replace-node-geometry` rebuilds these four members in this order, dropping every one whose
argument is `null` — which is how its committed vector turns a circle with a radius into a rectangle
with a width and a height."""

EDGE_GEOMETRY = (("gap", "newGap"), ("shift", "newShift"), ("rise", "newRise"), ("rotation", "newRotation"), ("turn", "newTurn"), ("tilt", "newTilt"), ("x", "newX"), ("y", "newY"))
"""🧮 The eight geometry members of an edge. `replace-edge-geometry` names them with a `new` prefix;
`connect-handles` names them bare, which is the one place in this vocabulary where the same eight
values are addressed under two spellings."""

EDGE_TIPS = ("sourceTip", "targetTip")
"""🖇 The two optional edge tips, which `connect-handles` drops when its argument is `null`."""

COMPATIBILITY_FIELDS = ("source", "target", "bidirectional", "important", "specificity")
"""🤝 The members of one kind-compatibility record, in the order `connect-kind-compatibility` names
them."""

UNDERDETERMINED = {"replace-node-handle"}
"""🚧️ `replace-node-handle` is the one kind this implementation refuses to state. Its single committed
vector supplies a genuinely different handle — `handle-1` moves from `handle-kind-a` to
`handle-kind-c` — and yet its committed outcome declares `mutation.no-op` and its after-snapshot is
byte-identical to its before-snapshot. At least three different rules produce exactly that outcome and
no committed document distinguishes them: the verb may be unimplemented; it may refuse a handle an
edge is attached to, which `handle-1` is; or it may refuse a handle kind the `kindCompatibility`
relation does not admit, which `handle-kind-c` is. `📓️derivation-rules.md` rule 2 says
`replace-<singular>-<member>` REPLACES the addressed record, so a second implementation written from
the specification would move the document — and this implementation declines to pick one of the three
rules and call it agreement. ONE more committed vector, on an UNCONNECTED handle, decides it."""

KINDS = (
    "create-node",
    "delete-node",
    "move-node",
    "replace-node-geometry",
    "change-node-kind",
    "edit-node-text",
    "change-node-icon",
    "scale-node",
    "change-node-visible",
    "change-node-locked",
    "change-node-root",
    "change-node-anchor",
    "add-node-handle",
    "remove-node-handle",
    "replace-node-handle",
    "connect-handles",
    "disconnect-handles",
    "replace-edge-geometry",
    "change-edge-kind",
    "change-edge-tips",
    "change-edge-visible",
    "change-edge-locked",
    "change-manifest-id",
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
    """✅️ Holds the document to the shape the committed vectors agree on: five members, unique node,
    handle and edge ids, every edge attached to handles the board really holds, and no member left
    standing at its own default."""
    if set(document) != set(MEMBERS):
        raise AssertionError("%s: a puzzle2d document must carry exactly %r, found %r" % (where, sorted(MEMBERS), sorted(document)))
    if set(document["camera"]) != {"x", "y", "zoom"}:
        raise AssertionError("%s: camera must carry exactly x, y and zoom, found %r" % (where, sorted(document["camera"])))
    handles = {}
    node_ids = []
    for node in document["nodes"]:
        node_ids.append(node["id"])
        if node.get("shape") not in ("circle", "rectangle"):
            raise AssertionError("%s: node %r must declare a circle or a rectangle, found %r" % (where, node["id"], node.get("shape")))
        for member, default in DEFAULTS.items():
            if member in node and node[member] == default:
                raise AssertionError("%s: node %r writes %s at its default %r, which a committed snapshot omits" % (where, node["id"], member, default))
        for handle in node["handles"]:
            if handle["id"] in handles:
                raise AssertionError("%s: handle %r is declared on two nodes" % (where, handle["id"]))
            handles[handle["id"]] = node["id"]
    if len(set(node_ids)) != len(node_ids):
        raise AssertionError("%s: nodes carries a duplicate id in %r" % (where, node_ids))
    edge_ids = []
    for edge in document["edges"]:
        edge_ids.append(edge["id"])
        for end in ("source", "target"):
            if edge[end] not in handles:
                raise AssertionError("%s: edge %r names %s handle %r, which no node on this board declares" % (where, edge["id"], end, edge[end]))
    if len(set(edge_ids)) != len(edge_ids):
        raise AssertionError("%s: edges carries a duplicate id in %r" % (where, edge_ids))
    if "manifestId" not in document["meta"] or "kindCompatibility" not in document["meta"]:
        raise AssertionError("%s: meta must carry a manifestId and a kindCompatibility relation, found %r" % (where, sorted(document["meta"])))


def node_at(document, identity, kind, where):
    """🔎️ The index of the node this kind addresses; an absent id is an error, never a no-op."""
    for at, node in enumerate(document["nodes"]):
        if node["id"] == identity:
            return at
    raise AssertionError("%s-%s: the committed vector addresses node %r, which the before-snapshot does not hold" % (where, kind, identity))


def edge_at(document, identity, kind, where):
    """🔎️ The index of the edge this kind addresses."""
    for at, edge in enumerate(document["edges"]):
        if edge["id"] == identity:
            return at
    raise AssertionError("%s-%s: the committed vector addresses edge %r, which the before-snapshot does not hold" % (where, kind, identity))


def written(record, member, value):
    """🫥 Writes a member, or REMOVES it when the value is the one a committed snapshot omits."""
    if member in DEFAULTS and value == DEFAULTS[member]:
        record.pop(member, None)
    else:
        record[member] = value


def attached_to(document, handle_ids):
    """✂️ The edges attached to any of these handles, in board order — what a node or handle removal
    severs."""
    return [edge for edge in document["edges"] if edge["source"] in handle_ids or edge["target"] in handle_ids]
# endregion 🔖️Document


# region 🔖️Verbs
def apply_mutation(document, kind, payload):
    """🦠️ Applies one kind, answering the new document and the diagnostic codes it raised."""
    if kind in UNDERDETERMINED:
        raise AssertionError("mutate-%s: %s" % (kind, UNDERDETERMINED_REASON))
    document = copy.deepcopy(document)
    if kind == "create-node":
        index = payload.get("index")
        document["nodes"].insert(len(document["nodes"]) if index is None else index, copy.deepcopy(payload["node"]))
    elif kind == "delete-node":
        at = node_at(document, payload["id"], kind, "mutate")
        severed = {handle["id"] for handle in document["nodes"][at]["handles"]}
        document["nodes"].pop(at)
        document["edges"] = [edge for edge in document["edges"] if edge["source"] not in severed and edge["target"] not in severed]
    elif kind == "move-node":
        node = document["nodes"][node_at(document, payload["id"], kind, "mutate")]
        node["x"] = payload["newX"]
        node["y"] = payload["newY"]
    elif kind == "replace-node-geometry":
        node = document["nodes"][node_at(document, payload["id"], kind, "mutate")]
        for member, argument in NODE_GEOMETRY:
            node.pop(member, None)
            if payload.get(argument) is not None:
                node[member] = copy.deepcopy(payload[argument])
    elif kind in NODE_FIELDS:
        member, argument = NODE_FIELDS[kind]
        written(document["nodes"][node_at(document, payload["id"], kind, "mutate")], member, payload[argument])
    elif kind == "add-node-handle":
        node = document["nodes"][node_at(document, payload["nodeId"], kind, "mutate")]
        index = payload.get("index")
        node["handles"].insert(len(node["handles"]) if index is None else index, copy.deepcopy(payload["handle"]))
    elif kind == "remove-node-handle":
        node = document["nodes"][node_at(document, payload["nodeId"], kind, "mutate")]
        if not any(handle["id"] == payload["handleId"] for handle in node["handles"]):
            raise AssertionError("mutate-%s: node %r declares no handle %r" % (kind, payload["nodeId"], payload["handleId"]))
        node["handles"] = [handle for handle in node["handles"] if handle["id"] != payload["handleId"]]
        document["edges"] = [edge for edge in document["edges"] if edge["source"] != payload["handleId"] and edge["target"] != payload["handleId"]]
    elif kind == "connect-handles":
        edge = {"id": payload["id"], "source": payload["source"], "target": payload["target"], "edgeKind": payload["edgeKind"]}
        for member, _argument in EDGE_GEOMETRY:
            edge[member] = payload[member]
        for tip in EDGE_TIPS:
            if payload.get(tip) is not None:
                edge[tip] = payload[tip]
        document["edges"].append(edge)
    elif kind == "disconnect-handles":
        document["edges"].pop(edge_at(document, payload["id"], kind, "mutate"))
    elif kind == "replace-edge-geometry":
        edge = document["edges"][edge_at(document, payload["id"], kind, "mutate")]
        for member, argument in EDGE_GEOMETRY:
            edge[member] = payload[argument]
    elif kind == "change-edge-tips":
        edge = document["edges"][edge_at(document, payload["id"], kind, "mutate")]
        for tip, argument in (("sourceTip", "newSourceTip"), ("targetTip", "newTargetTip")):
            if payload.get(argument) is None:
                edge.pop(tip, None)
            else:
                edge[tip] = payload[argument]
    elif kind in EDGE_FIELDS:
        member, argument = EDGE_FIELDS[kind]
        written(document["edges"][edge_at(document, payload["id"], kind, "mutate")], member, payload[argument])
    elif kind == "change-manifest-id":
        document["meta"]["manifestId"] = payload["newManifestId"]
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


UNDERDETERMINED_REASON = (
    "this implementation refuses this kind rather than guessing it. Its single committed vector supplies a genuinely different handle — `handle-1` "
    "moves from `handle-kind-a` to `handle-kind-c` — and yet the committed outcome declares `mutation.no-op` and the after-snapshot is identical to "
    "the before-snapshot. At least three rules produce exactly that and no committed document distinguishes them: the verb is unimplemented; it "
    "refuses a handle an edge is attached to, which `handle-1` is; or it refuses a handle kind the `kindCompatibility` relation does not admit, which "
    "`handle-kind-c` is. `📓️derivation-rules.md` rule 2 says `replace-<singular>-<member>` replaces the addressed record, so a second implementation "
    "written from the specification would move the document. ONE more committed vector, on an unconnected handle, decides it."
)


def inverse_mutation(document, kind, payload):
    """↩️ The kind's OWN inverse, expressed in this same closed vocabulary, computed against the
    pre-mutation document. `delete-node` and `remove-node-handle` invert to SEVERAL steps, because
    they sever edges: the node or handle is put back first and every severed edge is reconnected after
    it, in board order. A `create`/`add` inverts to a single removal, which is exact only for a
    TRAILING record unless the verb carries an index — both of these do."""
    if kind in UNDERDETERMINED:
        raise AssertionError("inverse-%s: %s" % (kind, UNDERDETERMINED_REASON))
    if kind == "create-node":
        return [("delete-node", {"id": payload["node"]["id"]})]
    if kind == "delete-node":
        at = node_at(document, payload["id"], kind, "inverse")
        node = document["nodes"][at]
        severed = attached_to(document, {handle["id"] for handle in node["handles"]})
        steps = [("create-node", {"node": copy.deepcopy(node), "index": at})]
        return steps + [("connect-handles", reconnect(edge)) for edge in severed]
    if kind == "move-node":
        node = document["nodes"][node_at(document, payload["id"], kind, "inverse")]
        return [(kind, {"id": payload["id"], "newX": node["x"], "newY": node["y"]})]
    if kind == "replace-node-geometry":
        node = document["nodes"][node_at(document, payload["id"], kind, "inverse")]
        return [(kind, dict({"id": payload["id"]}, **{argument: copy.deepcopy(node[member]) if member in node else None for member, argument in NODE_GEOMETRY}))]
    if kind in NODE_FIELDS:
        member, argument = NODE_FIELDS[kind]
        node = document["nodes"][node_at(document, payload["id"], kind, "inverse")]
        return [(kind, {"id": payload["id"], argument: node.get(member, DEFAULTS.get(member))})]
    if kind == "add-node-handle":
        return [("remove-node-handle", {"nodeId": payload["nodeId"], "handleId": payload["handle"]["id"]})]
    if kind == "remove-node-handle":
        node = document["nodes"][node_at(document, payload["nodeId"], kind, "inverse")]
        at = next(index for index, handle in enumerate(node["handles"]) if handle["id"] == payload["handleId"])
        severed = attached_to(document, {payload["handleId"]})
        steps = [("add-node-handle", {"nodeId": payload["nodeId"], "handle": copy.deepcopy(node["handles"][at]), "index": at})]
        return steps + [("connect-handles", reconnect(edge)) for edge in severed]
    if kind == "connect-handles":
        return [("disconnect-handles", {"id": payload["id"]})]
    if kind == "disconnect-handles":
        return [("connect-handles", reconnect(document["edges"][edge_at(document, payload["id"], kind, "inverse")]))]
    if kind == "replace-edge-geometry":
        edge = document["edges"][edge_at(document, payload["id"], kind, "inverse")]
        return [(kind, dict({"id": payload["id"]}, **{argument: edge[member] for member, argument in EDGE_GEOMETRY}))]
    if kind == "change-edge-tips":
        edge = document["edges"][edge_at(document, payload["id"], kind, "inverse")]
        return [(kind, {"id": payload["id"], "newSourceTip": edge.get("sourceTip"), "newTargetTip": edge.get("targetTip")})]
    if kind in EDGE_FIELDS:
        member, argument = EDGE_FIELDS[kind]
        edge = document["edges"][edge_at(document, payload["id"], kind, "inverse")]
        return [(kind, {"id": payload["id"], argument: edge.get(member, DEFAULTS.get(member))})]
    if kind == "change-manifest-id":
        return [(kind, {"newManifestId": document["meta"]["manifestId"]})]
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


def reconnect(edge):
    """🔗 The `connect-handles` arguments that rebuild one edge exactly as it stands."""
    payload = {"id": edge["id"], "source": edge["source"], "target": edge["target"], "edgeKind": edge["edgeKind"]}
    for member, _argument in EDGE_GEOMETRY:
        payload[member] = edge[member]
    for tip in EDGE_TIPS:
        payload[tip] = edge.get(tip)
    return payload
# endregion 🔖️Verbs


# region 🔖️Laws
def equals_committed(kind, produced, committed):
    """🎯️ The committed after-snapshot claim, member by member, with no tolerance and no ignored key."""
    for member in MEMBERS:
        if produced[member] != committed[member]:
            raise AssertionError("mutate-%s: %s is %s, the committed after-snapshot says %s" % (kind, member, json.dumps(produced[member], sort_keys=True)[:400], json.dumps(committed[member], sort_keys=True)[:400]))


def observable(kind, before, after, no_op):
    """👁️ A vector whose committed outcome does NOT declare `mutation.no-op` must move the compared
    projection; one that does must move nothing. The exemption is read off the committed outcome, not
    off a list this file keeps."""
    if no_op and before != after:
        raise AssertionError("mutate-%s: the committed outcome declares mutation.no-op, yet the document moved" % kind)
    if not no_op and before == after:
        raise AssertionError("mutate-%s: the committed vector declares this kind applied, yet the document did not move" % kind)


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
    """🔁️ Reads the committed board and answers with the whole document. This implementation
    additionally requires, in role, that it really is a board and not a graph: two nodes, at least
    three handles OWNED BY those nodes, and an edge whose two endpoints are handles rather than nodes."""
    uri = uri_in(ctx, "⬅️before")
    committed = ctx.fixture_bytes(uri)
    document = json.loads(committed.decode("utf-8"))
    validate(document, "identity-round-trip")
    handles = [handle["id"] for node in document["nodes"] for handle in node["handles"]]
    node_ids = {node["id"] for node in document["nodes"]}
    if len(document["nodes"]) < 2 or len(handles) < 3 or not document["edges"]:
        raise AssertionError("identity-round-trip: the committed board must carry two nodes, three handles and an edge, found %d/%d/%d" % (len(document["nodes"]), len(handles), len(document["edges"])))
    for edge in document["edges"]:
        for end in ("source", "target"):
            if edge[end] in node_ids:
                raise AssertionError("identity-round-trip: edge %r names the NODE %r as its %s; on this board an edge joins two handles" % (edge["id"], edge[end], end))
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

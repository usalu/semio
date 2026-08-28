#!/usr/bin/env python3
"""🔌 An INDEPENDENT second implementation of the `s.trinity.jack` assembly scene and all eight of
its typed mutations, in Python, serving as this case's differential oracle.

**Why a second implementation and not a third-party library.** A jack scene is a labelled property
graph whose nodes carry PORTS and whose edges address `node@port` endpoints, over a `manifest` that
declares which node, edge and port kinds exist. Graph libraries — `networkx`, `igraph`, `petgraph` —
model vertices and edges but have no notion of a port-addressed endpoint, of a manifest that closes
the kind vocabulary, or of the untyped `properties` bag that `change-data-property` and
`remove-data-property` edit; and none of them reads `.dsl.semio`. What a reference genuinely can
adjudicate is this vocabulary's own algebra — append-node, append-edge, delete-by-id, the two
in-place node edits, and the two property-bag edits, each with its own rejection rule — and that is
what this file implements, from the specification, in another language.

**What it was written from.**

* ``🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️component.json`` — `JackSnapshot` is
  `schema`, `name`, `manifestId`, `manifest`, `camera`, `nodes`, `edges` and `rootNodeId`; a `Node`
  is `{id, kind, name, x, y, width, height, properties, ports}` and a `Port` is
  `{id, kind, direction, properties}`; an `Edge` is `{id, kind, source, target, properties}`; both
  property bags are open objects, everything else is `additionalProperties: false`.
* ``…/🧬️schema/🧬️mutations/📝️text/📖️component.grammar.semio`` — the eight verbs and their positional
  argument lists, including `entity = "node" ":" id / "edge" ":" id` for the two property verbs.
* the eight committed specification vectors, which give the INTERNALLY tagged wire form of each verb
  and — this is all they give — its REJECTION rule.

**A defect in the specification, found while writing this and reported rather than worked around.**
``…/🧬️schema/🧬️mutations/🔣️component.json`` does not describe the mutations at all: it is a verbatim
copy of the snapshot schema, `title` changed to `JackMutation` and nothing else. The wire form was
therefore read off the committed vectors, which spell it internally tagged and — inconsistently —
mix camelCase discriminators with snake_case arguments (`new_name`, `new_value`).

**A gap in the evidence, likewise reported.** ALL EIGHT committed vectors are NEGATIVE: three
rejections and five accepted no-ops. Not one of them exercises a mutation that actually changes the
scene, so the accepting direction of this entire vocabulary had no committed evidence before this
case's real-document scenarios.

**What is inferred rather than read, and on what grounds.** Nothing states whether deleting a node
also deletes the edges that name it. The document HAS the invariant — `create-edge`'s committed
vector is rejected with `mutation.invariant` precisely because its endpoints are absent — so a
delete that left a dangling edge behind would produce a document the format refuses to construct.
This implementation therefore cascades, and says so; the feature's `delete-node` row nevertheless
addresses a node no edge names, so the cross-language comparison never rests on the inference.

**No Rust was read to write this.** `🦀️component.rs` beside this file registers the SUBJECT half
only.
"""

# region 🔖️Imports
import copy
import json

from semio_repo_test import Adapter, Context, Outcome, digest

# endregion 🔖️Imports


# region 🔖️Vocabulary
MEMBERS = ("schema", "name", "camera", "nodes", "edges", "rootNodeId")
"""🗂️ The cross-language projection: the members BOTH committed serializations of a jack scene carry.

The committed carrier writes `schema`, `name`, `manifestId`, `camera`, `nodes`, `edges` and
`rootNodeId`; the committed specification vectors write `schema`, `name`, `manifest`, `camera` and a
composed `content` child, and omit `nodes` and `edges` entirely because theirs are empty. Neither
`manifest`/`manifestId` — the kind vocabulary, which no mutation in this catalog edits — nor
`content` is therefore comparable across both, and `content` is not even declared by the snapshot
schema. What both forms do carry, once an absent `nodes`/`edges`/`rootNodeId` is read as empty, is
the six members above.
"""

KINDS = ("create-node", "delete-node", "create-edge", "delete-edge", "rename-node", "move-node", "change-data-property", "remove-data-property")
"""🏷️ Every kind the catalog declares."""

TAGS = {
    "create-node": "createNode",
    "delete-node": "deleteNode",
    "create-edge": "createEdge",
    "delete-edge": "deleteEdge",
    "rename-node": "renameNode",
    "move-node": "moveNode",
    "change-data-property": "changeDataProperty",
    "remove-data-property": "removeDataProperty",
}
"""🔤️ The internally tagged `mutation` discriminator of each kind, as the committed vectors spell it."""

NODE_MEMBERS = {"id", "kind", "name", "x", "y", "width", "height", "properties", "ports"}
PORT_MEMBERS = {"id", "kind", "direction", "properties"}
EDGE_MEMBERS = {"id", "kind", "source", "target", "properties"}

# endregion 🔖️Vocabulary


# region 🔖️Carrier
BANNER = "semio trinity.jack.dsl v1"

HEX_JSON = ("camera", "nodes", "edges")
"""#⃣ The members the carrier writes as hex of compact JSON; the rest are hex of plain UTF-8 text."""


def parse_carrier(text):
    """📖️ Reads a `trinity.jack.dsl v1` document into `(document, member-order)`.

    Every member is the hex of its UTF-8 bytes; `camera`, `nodes` and `edges` are the hex of their
    compact JSON. The order the members appear in is kept so the document can be printed back
    exactly.
    """
    lines = text.split("\n")
    if not lines or lines[0] != BANNER:
        raise AssertionError("carrier banner must be %r, found %r" % (BANNER, lines[0] if lines else ""))
    document, order = {}, []
    for line in lines[1:]:
        if line == "":
            continue
        key, separator, value = line.partition("=")
        if separator != "=":
            raise AssertionError("carrier line %r is not a key=value member" % line)
        decoded = bytes.fromhex(value).decode("utf-8")
        document[key] = json.loads(decoded) if key in HEX_JSON else decoded
        order.append(key)
    validate(document)
    return document, order


def print_carrier(document, order):
    """🖨️ Prints a document back in the member order the carrier used."""
    rendered = [BANNER]
    for key in order:
        value = json.dumps(document[key], separators=(",", ":"), ensure_ascii=False) if key in HEX_JSON else document[key]
        rendered.append("%s=%s" % (key, value.encode("utf-8").hex()))
    return "\n".join(rendered) + "\n"


# endregion 🔖️Carrier


# region 🔖️Document
def normalize(payload):
    """📥️ Reads a jack scene out of either committed serialization, defaulting the members the one
    that omits them leaves out — an absent `nodes`, `edges` or `rootNodeId` is the empty one."""
    document = copy.deepcopy(payload)
    document.setdefault("nodes", [])
    document.setdefault("edges", [])
    document.setdefault("rootNodeId", "")
    validate(document)
    return document


def validate(document):
    """✅️ Holds the document to the committed JSON Schema, and to the invariant `create-edge`'s own
    committed rejection establishes: every edge endpoint must name a node the scene holds."""
    for name in ("schema", "name"):
        if not isinstance(document.get(name), str):
            raise AssertionError("%s must be a string, found %r" % (name, document.get(name)))
    if set(document.get("camera", {})) != {"x", "y", "zoom"}:
        raise AssertionError("camera must carry exactly x, y and zoom, found %r" % sorted(document.get("camera", {})))
    identifiers = set()
    for node in document["nodes"]:
        if set(node) != NODE_MEMBERS:
            raise AssertionError("a node must carry exactly %r, found %r" % (sorted(NODE_MEMBERS), sorted(node)))
        if node["id"] in identifiers:
            raise AssertionError("the scene carries a duplicate node id %r" % node["id"])
        identifiers.add(node["id"])
        for port in node["ports"]:
            if set(port) != PORT_MEMBERS:
                raise AssertionError("a port must carry exactly %r, found %r" % (sorted(PORT_MEMBERS), sorted(port)))
    seen = set()
    for edge in document["edges"]:
        if set(edge) != EDGE_MEMBERS:
            raise AssertionError("an edge must carry exactly %r, found %r" % (sorted(EDGE_MEMBERS), sorted(edge)))
        if edge["id"] in seen:
            raise AssertionError("the scene carries a duplicate edge id %r" % edge["id"])
        seen.add(edge["id"])
        for end in ("source", "target"):
            if endpoint_node(edge[end]) not in identifiers:
                raise AssertionError("edge %r names %s %r, which is not a node in the scene" % (edge["id"], end, edge[end]))


def endpoint_node(endpoint):
    """🔌 The node half of a `node@port` endpoint."""
    return endpoint.split("@", 1)[0]


def find(items, identifier):
    """🔎️ The index of an id in a member list, or `None`."""
    for at, item in enumerate(items):
        if item["id"] == identifier:
            return at
    return None


def bag(document, entity, kind):
    """🎒️ The property bag one `entity` addresses, or `None` when it names nothing.

    An ABSENT target is not a rejection for the property verbs: two committed vectors declare
    `status: "applied"` with a `mutation.no-op` warning against a scene that holds neither the node
    nor the edge they name. Only the two `delete-` verbs reject an absent target, and their own
    committed vectors say so.
    """
    items = document["nodes"] if entity["entity"] == "node" else document["edges"] if entity["entity"] == "edge" else None
    if items is None:
        raise AssertionError("%s: %r is neither a node nor an edge" % (kind, entity["entity"]))
    at = find(items, entity["id"])
    return None if at is None else items[at]["properties"]


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
    """🧬️ Applies one typed mutation, returning `(document, no-op)`.

    Five of the eight verbs have a committed vector showing that writing the value the scene already
    holds — or naming a target the scene does not hold at all — is an accepted NO-OP rather than a
    rejection, so this returns whether the application was one. The three structural verbs reject
    instead, each with the code its own committed vector names.
    """
    kind = kind_of(mutation)
    result = copy.deepcopy(document)
    if kind == "create-node":
        node = copy.deepcopy(mutation["node"])
        if find(result["nodes"], node["id"]) is not None:
            raise AssertionError("%s: the scene already holds a node %r" % (kind, node["id"]))
        result["nodes"].append(node)
    elif kind == "delete-node":
        at = find(result["nodes"], mutation["id"])
        if at is None:
            raise AssertionError("%s: no node %r in the scene" % (kind, mutation["id"]))
        result["nodes"].pop(at)
        result["edges"] = [edge for edge in result["edges"] if mutation["id"] not in (endpoint_node(edge["source"]), endpoint_node(edge["target"]))]
    elif kind == "create-edge":
        edge = copy.deepcopy(mutation["edge"])
        if find(result["edges"], edge["id"]) is not None:
            raise AssertionError("%s: the scene already holds an edge %r" % (kind, edge["id"]))
        for end in ("source", "target"):
            if find(result["nodes"], endpoint_node(edge[end])) is None:
                raise AssertionError("%s: the edge's %s names %r, which is not a node in the scene" % (kind, end, edge[end]))
        result["edges"].append(edge)
    elif kind == "delete-edge":
        at = find(result["edges"], mutation["id"])
        if at is None:
            raise AssertionError("%s: no edge %r in the scene" % (kind, mutation["id"]))
        result["edges"].pop(at)
    elif kind == "rename-node":
        at = find(result["nodes"], mutation["id"])
        if at is None:
            return result, True
        if result["nodes"][at]["name"] == mutation["new_name"]:
            return result, True
        result["nodes"][at]["name"] = mutation["new_name"]
    elif kind == "move-node":
        at = find(result["nodes"], mutation["id"])
        if at is None:
            return result, True
        node = result["nodes"][at]
        if (node["x"], node["y"]) == (mutation["x"], mutation["y"]):
            return result, True
        node["x"], node["y"] = float(mutation["x"]), float(mutation["y"])
    elif kind == "change-data-property":
        properties = bag(result, mutation["entity"], kind)
        if properties is None or (mutation["key"] in properties and properties[mutation["key"]] == mutation["new_value"]):
            return result, True
        properties[mutation["key"]] = copy.deepcopy(mutation["new_value"])
    else:
        properties = bag(result, mutation["entity"], kind)
        if properties is None or mutation["key"] not in properties:
            return result, True
        del properties[mutation["key"]]
    validate(result)
    return result, False


def inverse_mutation(document, mutation):
    """↩️ The mutation that undoes one application, computed against the document it applies to."""
    kind = kind_of(mutation)
    if kind == "create-node":
        return {"mutation": TAGS["delete-node"], "id": mutation["node"]["id"]}
    if kind == "delete-node":
        at = find(document["nodes"], mutation["id"])
        if at is None:
            raise AssertionError("inverse of %s: no node %r in the scene" % (kind, mutation["id"]))
        return {"mutation": TAGS["create-node"], "node": copy.deepcopy(document["nodes"][at])}
    if kind == "create-edge":
        return {"mutation": TAGS["delete-edge"], "id": mutation["edge"]["id"]}
    if kind == "delete-edge":
        at = find(document["edges"], mutation["id"])
        if at is None:
            raise AssertionError("inverse of %s: no edge %r in the scene" % (kind, mutation["id"]))
        return {"mutation": TAGS["create-edge"], "edge": copy.deepcopy(document["edges"][at])}
    if kind == "rename-node":
        at = find(document["nodes"], mutation["id"])
        return {"mutation": TAGS[kind], "id": mutation["id"], "new_name": document["nodes"][at]["name"]}
    if kind == "move-node":
        at = find(document["nodes"], mutation["id"])
        node = document["nodes"][at]
        return {"mutation": TAGS[kind], "id": mutation["id"], "x": node["x"], "y": node["y"]}
    properties = bag(document, mutation["entity"], "inverse of %s" % kind)
    if properties is None:
        raise AssertionError("inverse of %s: %s %r is not in the scene" % (kind, mutation["entity"]["entity"], mutation["entity"]["id"]))
    if kind == "change-data-property":
        if mutation["key"] not in properties:
            return {"mutation": TAGS["remove-data-property"], "entity": copy.deepcopy(mutation["entity"]), "key": mutation["key"]}
        return {"mutation": TAGS[kind], "entity": copy.deepcopy(mutation["entity"]), "key": mutation["key"], "new_value": copy.deepcopy(properties[mutation["key"]])}
    if mutation["key"] not in properties:
        return {"mutation": TAGS["remove-data-property"], "entity": copy.deepcopy(mutation["entity"]), "key": mutation["key"]}
    return {"mutation": TAGS["change-data-property"], "entity": copy.deepcopy(mutation["entity"]), "key": mutation["key"], "new_value": copy.deepcopy(properties[mutation["key"]])}


# endregion 🔖️Mutations


# region 🔖️Laws
def observable(scenario, before, after):
    """👁️ A real-document row must move the scene. A mutation that quietly did nothing would agree
    with an unchanged one and report a pass having proved nothing."""
    if before == after:
        raise AssertionError("%s: the forward mutation left the scene untouched, so nothing was proved" % scenario)


def restores(kind, restored, original):
    """↩️ The metamorphic inverse law, reported by the first member and index that diverges."""
    if restored == original:
        return
    for name in MEMBERS:
        if restored.get(name) == original.get(name):
            continue
        if name in ("nodes", "edges"):
            was = [item["id"] for item in original[name]]
            now = [item["id"] for item in restored[name]]
            if was != now:
                raise AssertionError("inverse-%s: %s came back as %r, not %r" % (kind, name, now, was))
            for at, (left, right) in enumerate(zip(original[name], restored[name])):
                if left != right:
                    raise AssertionError("inverse-%s: %s[%d] (%s) came back as %s, not %s" % (kind, name, at, left["id"], json.dumps(right, sort_keys=True), json.dumps(left, sort_keys=True)))
        raise AssertionError("inverse-%s: %s came back as %s, not %s" % (kind, name, json.dumps(restored.get(name), sort_keys=True), json.dumps(original.get(name), sort_keys=True)))


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


def tower(ctx):
    """🗼️ The real committed Nakagin Capsule Tower scene, read through its own carrier."""
    return parse_carrier(ctx.fixture_bytes(uri_in(ctx, "📚️examples")).decode("utf-8"))


def projection_of(document):
    """📤️ What parity compares: the six members both committed serializations carry."""
    return {name: document.get(name, [] if name in ("nodes", "edges") else "") for name in MEMBERS}


def outcome_of(payload):
    """📤️ Wraps a projection with its own compact serialization as the raw artifact."""
    return Outcome(payload, raw=json.dumps(payload, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))


# endregion 🔖️Plan


# region 🔖️Handlers
def mutate_handler(kind):
    """🎯️ Applies one kind to the REAL committed Nakagin Capsule Tower scene."""

    def handler(ctx):
        document, _order = tower(ctx)
        mutation = json.loads(doc_string(ctx))
        if kind_of(mutation) != kind:
            raise AssertionError("mutate-%s: the feature states a %s payload" % (kind, kind_of(mutation)))
        applied, noop = apply_mutation(document, mutation)
        if noop:
            raise AssertionError("mutate-%s: the feature's parameters were an accepted no-op, so nothing was proved" % kind)
        observable("mutate-%s" % kind, document, applied)
        return outcome_of(projection_of(applied))

    return handler


def inverse_handler(kind):
    """↩️ Applies one kind to the REAL tower scene and then its OWN computed inverse.

    The projection carries BOTH scenes; projecting only the restored one would make all eight rows
    project the same value and the differential would be vacuous.
    """

    def handler(ctx):
        document, _order = tower(ctx)
        mutation = json.loads(doc_string(ctx))
        if kind_of(mutation) != kind:
            raise AssertionError("inverse-%s: the feature states a %s payload" % (kind, kind_of(mutation)))
        applied, noop = apply_mutation(document, mutation)
        if noop:
            raise AssertionError("inverse-%s: the feature's parameters were an accepted no-op, so nothing was proved" % kind)
        observable("inverse-%s" % kind, document, applied)
        restored, _noop = apply_mutation(applied, inverse_mutation(document, mutation))
        restores(kind, restored, document)
        return outcome_of({"mutated": projection_of(applied), "restored": projection_of(restored)})

    return handler


def spec_vector_handler(kind):
    """📐️ Replays one committed handcrafted vector. All eight are NEGATIVE, so the feature's
    `verdict` column states which of the two refusals each one commits to: `refused` must be refused
    outright, and `noop` must be accepted while leaving the scene exactly where it was."""

    def handler(ctx):
        before = normalize(json_fixture(ctx, "⬅️before"))
        mutation = json_fixture(ctx, "🦠️mutation")
        after = normalize(json_fixture(ctx, "➡️after"))
        verdict = json.loads(doc_string(ctx))["verdict"]
        if kind_of(mutation) != kind:
            raise AssertionError("spec-vector-%s: the committed vector carries a %s payload" % (kind, kind_of(mutation)))
        if verdict == "refused":
            try:
                apply_mutation(before, mutation)
            except AssertionError:
                if projection_of(before) != projection_of(after):
                    raise AssertionError("spec-vector-%s: a refused vector must commit the same before- and after-scene" % kind)
                return outcome_of(projection_of(before))
            raise AssertionError("spec-vector-%s: the committed vector declares a refusal, but the mutation applied" % kind)
        applied, noop = apply_mutation(before, mutation)
        if not noop:
            raise AssertionError("spec-vector-%s: the committed vector declares an accepted no-op, but the mutation moved the scene" % kind)
        if projection_of(applied) != projection_of(after):
            raise AssertionError("spec-vector-%s: the applied scene is not the committed after-scene" % kind)
        return outcome_of(projection_of(applied))

    return handler


def identity_handler(ctx):
    """🔁️ Reads the artifact's own committed real example and prints it back byte for byte.

    Byte exactness is asserted here, in role: this implementation and the committed file are two
    independent productions of one carrier layout, so anything short of identity is a misreading.
    The projection is the scene itself, which is what lets the two languages be compared on what
    they each read out of the same real bytes.
    """
    text = ctx.fixture_bytes(uri_in(ctx, "📚️examples")).decode("utf-8")
    document, order = parse_carrier(text)
    if document["name"] != "Nakagin Capsule Tower" or len(document["nodes"]) != 9 or len(document["edges"]) != 6:
        raise AssertionError("identity-round-trip: the committed example is the nine-node, six-edge Nakagin tower, read %r with %d node(s) and %d edge(s)" % (document.get("name"), len(document["nodes"]), len(document["edges"])))
    printed = print_carrier(document, order)
    if printed != text:
        at = next((offset for offset, (left, right) in enumerate(zip(printed, text)) if left != right), min(len(printed), len(text)))
        raise AssertionError("identity-round-trip: re-encoding the committed example produced %d byte(s) against %d, first difference at %d" % (len(printed.encode("utf-8")), len(text.encode("utf-8")), at))
    reparsed, _order = parse_carrier(printed)
    if reparsed != document:
        raise AssertionError("identity-round-trip: printing and reparsing moved the scene")
    return Outcome(
        projection_of(document),
        raw=printed.encode("utf-8"),
        diagnostics=[{"severity": "info", "message": "committed example reproduced byte for byte: %d bytes, digest %s" % (len(printed.encode("utf-8")), digest(printed.encode("utf-8")))}],
    )


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

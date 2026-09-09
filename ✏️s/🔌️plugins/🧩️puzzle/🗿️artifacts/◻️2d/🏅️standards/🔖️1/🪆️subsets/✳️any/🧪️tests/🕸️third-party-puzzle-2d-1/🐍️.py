#!/usr/bin/env python3
"""🕸️ Third-party oracle for `s.puzzle.2d@1`, in Python, over four libraries that have never seen
this repository.

**Why this exists next to `◻️mutate-puzzle-2d-1`.** That case's reference is a SECOND IMPLEMENTATION
written inside this repository, and a second implementation is useful evidence but never independent
evidence. This case adds the independent half. It does not re-implement a single verb. For every
committed vector it rebuilds the same fact in a DIFFERENT data structure and lets a third-party
library state the answer:

* ``networkx`` — the board becomes a ``MultiDiGraph`` whose vertices are NODES and HANDLES, with
  ``owns`` edges (node → handle) and ``wire`` edges (handle → handle, keyed by the board's edge id).
  Cascade-on-delete is then not something this file computes: it is what ``Graph.remove_node`` does
  to a vertex's incident edges, and ``networkx.utils.graphs_equal`` — the library's own comparison —
  decides whether the committed after-snapshot agrees. This is the exact objection the earlier
  survey raised against GraphML/DOT/GEXF (``none of them can express an edge whose endpoints are
  ports OWNED BY a node``) answered rather than repeated: the port is a VERTEX, so the carrier's
  two-level connectivity becomes ordinary graph incidence and the library's deletion semantics
  become the oracle.
* ``shapely`` — a node's footprint becomes a real ``Point.buffer`` or ``box`` and the geometry verbs
  become ``affinity.translate`` / ``affinity.scale(origin=…)``. Position, extent and the
  null-dropping circle→rectangle rebuild are then adjudicated by a geometry engine's own affine
  algebra and its own ``area`` / ``bounds`` / ``centroid`` / ``equals_exact``, on numbers this
  repository never handed it in that form.
* ``jsonschema`` — every committed ``🦠️mutation`` payload is validated against its own leaf
  ``🧬️schema/🔣️.json`` by an independent draft-07 validator, and every leaf schema is additionally
  probed with a member it does not declare, so an accepted payload proves the validator ran rather
  than that the schema was permissive.
* ``jsonpatch``, corroborated by ``deepdiff`` — the committed ``🔺️diff`` is a TYPED ``Puzzle2dDiff``
  (per-collection ``added``/``removed``/``patched``), not RFC 6902. So an RFC 6902 patch is derived
  from ``(before, after)`` by ``jsonpatch.make_patch``, round-tripped through ``apply`` to reproduce
  the after-snapshot exactly, and its op PATHS are then held against the typed diff's own
  added/removed/patched sets. Two libraries with unrelated algorithms must agree on whether the
  document moved at all.

**Vectors are discovered, never listed.** The committed scenario set under
``🧬️schema/🧬️mutations/<leaf>/`` grows and is renamed; a hard-coded Examples table went stale
for four of twenty-six rows before this file was written. The one fixture this case declares is the
mutation vocabulary's own ``🔣️.json``; every vector beneath its directory is found at run time, so a
scenario added tomorrow is checked without touching this file.

**What these libraries do NOT adjudicate**, stated rather than implied: the anchor ENUM, the handle
`angle` (no library here has a polar-on-owner concept), which of three refusal rules makes
`replace-node-handle` a no-op, and the meaning of a kind label. Those stay with the second
implementation and the committed vectors.

@see ../◻️mutate-puzzle-2d-1/🐍️.py
@see ../../🔮️oracle/🔣️.json
"""

# region 🔖️Imports
import json
import math
import os

import jsonpatch
import jsonschema
import networkx as nx
from deepdiff import DeepDiff
from shapely import affinity
from shapely.geometry import Point, box

from semio_repo_test import Adapter, Outcome

# endregion 🔖️Imports


# region 🔖️Vocabulary
VECTOR_ROOT_URI = "shared://🧬️mutations/🔣️.json"
"""🧫️ The declared fixture. Its DIRECTORY is the mutation vocabulary, and every committed vector is
found beneath it at run time."""

SNAPSHOT_BEFORE = ("📸️snapshot", "⬅️before", "🔣️.json")
SNAPSHOT_AFTER = ("📸️snapshot", "➡️after", "🔣️.json")
MUTATION_LEAF = ("🦠️mutation", "🔣️.json")
DIFF_LEAF = ("🔺️diff", "🔣️.json")
OUTCOME_LEAF = ("🎯️outcome", "🔣️.json")
SCENARIOS_DIR = "🧪️tests"
LEAF_SCHEMA = "🧬️schema/🔣️.json"

GRAPH_KINDS = ("create-node", "delete-node", "add-node-handle", "remove-node-handle", "replace-node-handle", "connect-handles", "disconnect-handles")
"""🕸️ The seven kinds whose correctness is topological — the only ones a graph library can speak to."""

RELATION_KINDS = ("connect-kind-compatibility", "disconnect-kind-compatibility")
"""🤝 The two kinds that edit the kind-compatibility relation, which is itself a directed graph over
kind labels."""

GEOMETRY_KINDS = ("move-node", "scale-node", "change-node-anchor", "replace-node-geometry")
"""📐 The four node-geometry kinds. `replace-edge-geometry` and `connect-handles` carry an edge x/y
too and are handled beside them."""

COMPATIBILITY_ATTRS = ("bidirectional", "important", "specificity")
"""🤝 The three attributes a compatibility record carries beyond its two endpoints."""

MEMBERS = ("schema", "camera", "nodes", "edges", "meta")
"""🗂️ The five members of `Puzzle2dSnapshot` — and the five collections the typed diff keys on."""

CIRCLE_SEGMENTS = 512
"""⭕️ Quadrant segments shapely polygonises a circle with. High enough that `area` lands on πr² to
better than one part in a million, which is what the extent assertions compare against."""

TOLERANCE = 1e-9
"""📏 Absolute tolerance for coordinate comparisons; the committed vectors are exact decimals."""


def kind_of(leaf_name):
    """🏷️ The mutation kind a vocabulary directory names, with its identity emoji stripped."""
    for at, character in enumerate(leaf_name):
        if character.isascii() and (character.isalpha() or character.isdigit()):
            return leaf_name[at:]
    return leaf_name


def tag_of(kind):
    """🔤️ The internally tagged `mutation` discriminator of a kind — lowerCamelCase of its words."""
    head, *rest = kind.split("-")
    return head + "".join(word[:1].upper() + word[1:] for word in rest)
# endregion 🔖️Vocabulary


# region 🔖️Discovery
def read_json(*parts):
    """📜️ One committed JSON leaf, or `None` when the vector deliberately does not carry it."""
    path = os.path.join(*parts)
    if not os.path.exists(path):
        return None
    with open(path, "r", encoding="utf-8") as handle:
        return json.load(handle)


DISCOVERED = {}
"""🗃️ Per-process vector cache. Five scenarios read the same tree, and re-reading it five times is
the difference between a two-second case and a twenty-second one."""


def vectors(ctx):
    """🧫️ Every committed vector under the declared vocabulary root, in a stable order.

    Discovery rather than enumeration is deliberate: this artifact's scenario directories are being
    authored and renamed continuously, and the sibling case's hand-written Examples table pointed at
    four directories that no longer existed."""
    root = os.path.dirname(ctx.fixture(VECTOR_ROOT_URI))
    if root in DISCOVERED:
        return DISCOVERED[root]
    found = []
    for leaf in sorted(os.listdir(root)):
        scenarios = os.path.join(root, leaf, SCENARIOS_DIR)
        if not os.path.isdir(scenarios):
            continue
        schema = read_json(root, leaf, LEAF_SCHEMA)
        for scenario in sorted(os.listdir(scenarios)):
            directory = os.path.join(scenarios, scenario)
            if not os.path.isdir(directory):
                continue
            found.append(
                {
                    "id": "%s/%s" % (leaf, scenario),
                    "kind": kind_of(leaf),
                    "schema": schema,
                    "before": read_json(directory, *SNAPSHOT_BEFORE),
                    "after": read_json(directory, *SNAPSHOT_AFTER),
                    "mutation": read_json(directory, *MUTATION_LEAF),
                    "diff": read_json(directory, *DIFF_LEAF),
                    "outcome": read_json(directory, *OUTCOME_LEAF),
                }
            )
    if not found:
        raise AssertionError("no committed vector was discovered under %s — the mutation vocabulary cannot be empty" % root)
    DISCOVERED[root] = found
    return found


def payload_of(vector):
    """🦠️ The committed payload's arguments — the discriminator removed, what the handlers consume."""
    return {key: value for key, value in vector["mutation"].items() if key != "mutation"}


def declares_no_op(vector):
    """🚦️ Whether the committed outcome itself records that the mutation had nothing to do."""
    outcome = vector["outcome"] or {}
    return any(message.get("code") == "mutation.no-op" for message in outcome.get("messages", []))


def moves_document(vector):
    """🚦️ Whether the committed outcome says this vector moves the board at all.

    A vector is a rejection (`status` other than `applied`, with a `code` naming why) or an applied
    no-op or an applied change; only the third may move the document, and the two libraries below
    must reproduce nothing at all for the other two."""
    outcome = vector["outcome"] or {}
    return outcome.get("status") == "applied" and not declares_no_op(vector)


def report(scenario, rows, failures):
    """📤️ One scenario's answer: an ordered, per-vector record plus the count it actually checked."""
    if failures:
        raise AssertionError("%s: %d disagreement(s)\n%s" % (scenario, len(failures), "\n".join(failures)))
    return Outcome({"scenario": scenario, "checked": sum(row["checks"] for row in rows), "vectors": rows})
# endregion 🔖️Discovery


# region 🔖️Graph
def board_graph(document):
    """🕸️ The board as a `networkx.MultiDiGraph` over NODE and HANDLE vertices.

    An `owns` edge carries a node's handle list as graph INCIDENCE instead of array membership, and a
    `wire` edge joins two HANDLE vertices, so an edge's endpoints really are ports owned by a node —
    the shape no graph interchange FORMAT can express, expressed. Nothing about this structure is a
    transliteration of the subject: the subject nests handles inside nodes and keeps edges in a
    sibling list."""
    graph = nx.MultiDiGraph()
    for node in document["nodes"]:
        graph.add_node("node:%s" % node["id"], **{key: value for key, value in node.items() if key != "handles"})
        for handle in node["handles"]:
            graph.add_node("handle:%s" % handle["id"], **handle)
            graph.add_edge("node:%s" % node["id"], "handle:%s" % handle["id"], key="owns:%s" % handle["id"], rel="owns")
    for edge in document["edges"]:
        graph.add_edge("handle:%s" % edge["source"], "handle:%s" % edge["target"], key="wire:%s" % edge["id"], rel="wire", **edge)
    return graph


def owned_handles(graph, node_id):
    """🔗 The handle vertices a node owns, read off graph incidence rather than the nested array."""
    return [target for _, target, data in graph.out_edges("node:%s" % node_id, data=True) if data["rel"] == "owns"]


def ownership_invariant(graph, where, failures):
    """🧭️ Every handle vertex is owned by exactly one node, and no wire dangles.

    Both facts are read out of the graph's own degree bookkeeping; neither is computed by scanning
    the carrier."""
    for vertex in graph.nodes:
        if not vertex.startswith("handle:"):
            continue
        owners = [source for source, _, data in graph.in_edges(vertex, data=True) if data["rel"] == "owns"]
        if len(owners) != 1:
            failures.append("%s: handle vertex %s is owned by %d nodes" % (where, vertex, len(owners)))
    for source, target, data in graph.edges(data=True):
        if data["rel"] != "wire":
            continue
        if not (source.startswith("handle:") and target.startswith("handle:")):
            failures.append("%s: wire %s → %s does not join two handle vertices" % (where, source, target))


def apply_topology(graph, vector):
    """🕸️ Applies one topological kind to the graph USING THE LIBRARY'S OWN vertex and edge removal.

    The cascades this artifact is defined by — a removed handle severs its wire, a deleted node
    severs every wire on any of its handles — are never written out here. `remove_node` drops a
    vertex's incident edges because that is what a graph is, and that behaviour IS the assertion."""
    kind = vector["kind"]
    payload = payload_of(vector)
    if kind == "create-node":
        node = payload["node"]
        graph.add_node("node:%s" % node["id"], **{key: value for key, value in node.items() if key != "handles"})
        for handle in node["handles"]:
            graph.add_node("handle:%s" % handle["id"], **handle)
            graph.add_edge("node:%s" % node["id"], "handle:%s" % handle["id"], key="owns:%s" % handle["id"], rel="owns")
        return True
    if kind == "delete-node":
        graph.remove_nodes_from(owned_handles(graph, payload["id"]) + ["node:%s" % payload["id"]])
        return True
    if kind == "add-node-handle":
        handle = payload["handle"]
        graph.add_node("handle:%s" % handle["id"], **handle)
        graph.add_edge("node:%s" % payload["nodeId"], "handle:%s" % handle["id"], key="owns:%s" % handle["id"], rel="owns")
        return True
    if kind == "remove-node-handle":
        graph.remove_node("handle:%s" % payload["handleId"])
        return True
    if kind == "replace-node-handle":
        old = "handle:%s" % payload["handleId"]
        if old not in graph:
            return False
        handle = payload["newHandle"]
        new = "handle:%s" % handle["id"]
        owner = next(source for source, _, data in graph.in_edges(old, data=True) if data["rel"] == "owns")
        if new != old:
            # 🔁️A re-identified port is a VERTEX RELABEL, and `relabel_nodes` carries every incident
            # wire across with it — the fact this verb has to get right, stated by the library.
            nx.relabel_nodes(graph, {old: new}, copy=False)
            graph.remove_edge(owner, new, key="owns:%s" % payload["handleId"])
            graph.add_edge(owner, new, key="owns:%s" % handle["id"], rel="owns")
        graph.nodes[new].clear()
        graph.nodes[new].update(handle)
        return True
    if kind == "connect-handles":
        # 🫥️A JSON carrier writes no member for an absent optional, so a null argument becomes an
        # absent attribute rather than a null one. That is a carrier fact, not a verb rule.
        edge = {key: value for key, value in payload.items() if value is not None}
        graph.add_edge("handle:%s" % payload["source"], "handle:%s" % payload["target"], key="wire:%s" % payload["id"], rel="wire", **edge)
        return True
    if kind == "disconnect-handles":
        for source, target, key in list(graph.edges(keys=True)):
            if key == "wire:%s" % payload["id"]:
                graph.remove_edge(source, target, key=key)
        return True
    return False


def graph_cascade(ctx):
    """🕸️ networkx answers the seven topological kinds, and the ownership invariant on every board."""
    rows = []
    failures = []
    for vector in vectors(ctx):
        checks = 0
        if vector["before"] is None:
            rows.append({"id": vector["id"], "kind": vector["kind"], "checks": 0, "note": "no before-snapshot"})
            continue
        before = board_graph(vector["before"])
        ownership_invariant(before, "%s before" % vector["id"], failures)
        checks += 1
        state = "invariant-only"
        if vector["kind"] in GRAPH_KINDS and vector["after"] is not None:
            after = board_graph(vector["after"])
            ownership_invariant(after, "%s after" % vector["id"], failures)
            checks += 2
            if not moves_document(vector):
                state = "declared-%s" % (vector["outcome"] or {}).get("status", "unstated")
                if not nx.utils.graphs_equal(before, after):
                    failures.append("%s: the committed outcome is %r, yet the board's graph moved" % (vector["id"], vector["outcome"]))
            elif apply_topology(before, vector):
                state = "topology-reproduced"
                if not nx.utils.graphs_equal(before, after):
                    failures.append("%s: the graph networkx computed from the before-snapshot does not equal the graph of the committed after-snapshot (nodes %d/%d, edges %d/%d)" % (vector["id"], before.number_of_nodes(), after.number_of_nodes(), before.number_of_edges(), after.number_of_edges()))
            else:
                failures.append("%s: kind %s is topological and declared applied, and this oracle applied nothing" % (vector["id"], vector["kind"]))
        rows.append({"id": vector["id"], "kind": vector["kind"], "checks": checks, "state": state, "handles": sum(1 for vertex in before.nodes if vertex.startswith("handle:")), "wires": sum(1 for _, _, data in before.edges(data=True) if data["rel"] == "wire")})
    return report("graph-cascade", rows, failures)


def relation_graph(document):
    """🤝 The kind-compatibility relation as a `networkx.DiGraph` over KIND LABELS.

    A `DiGraph` admits one edge per ordered pair, so the relation's own uniqueness is enforced by the
    library rather than asserted here: a duplicated record silently collapses and the edge count
    stops matching the record count."""
    graph = nx.DiGraph()
    for record in document["meta"].get("kindCompatibility", []) or []:
        graph.add_edge(record["source"], record["target"], **{key: record[key] for key in COMPATIBILITY_ATTRS if key in record})
    return graph


def kind_compatibility(ctx):
    """🤝 networkx answers the compatibility relation: uniqueness, and the two relation verbs."""
    rows = []
    failures = []
    for vector in vectors(ctx):
        if vector["before"] is None:
            continue
        checks = 0
        before = relation_graph(vector["before"])
        records = vector["before"]["meta"].get("kindCompatibility", []) or []
        checks += 1
        if before.number_of_edges() != len(records):
            failures.append("%s: the before-snapshot declares %d compatibility records but only %d distinct ordered pairs" % (vector["id"], len(records), before.number_of_edges()))
        if vector["after"] is not None:
            after = relation_graph(vector["after"])
            checks += 1
            payload = payload_of(vector) if moves_document(vector) else {}
            if vector["kind"] == "connect-kind-compatibility" and payload:
                before.add_edge(payload["source"], payload["target"], **{key: payload[key] for key in COMPATIBILITY_ATTRS if key in payload})
            elif vector["kind"] == "disconnect-kind-compatibility" and payload:
                if before.has_edge(payload["source"], payload["target"]):
                    before.remove_edge(payload["source"], payload["target"])
                # 🏷️A kind label exists in this relation only while some pair still names it, so the
                # vertex set is derived from the edge set — by `networkx.isolates`, not by hand.
                before.remove_nodes_from(list(nx.isolates(before)))
            if not nx.utils.graphs_equal(before, after):
                failures.append("%s: the compatibility relation networkx computed does not equal the committed after-snapshot's relation" % vector["id"])
            elif vector["kind"] not in RELATION_KINDS and vector["kind"] != "replace-kind-catalogs" and not nx.utils.graphs_equal(relation_graph(vector["before"]), after):
                failures.append("%s: kind %s moved the compatibility relation, which only %s may do" % (vector["id"], vector["kind"], " or ".join(RELATION_KINDS)))
        rows.append({"id": vector["id"], "kind": vector["kind"], "checks": checks, "pairs": before.number_of_edges()})
    return report("kind-compatibility", rows, failures)
# endregion 🔖️Graph


# region 🔖️Geometry
def footprint(node):
    """📐 A node's footprint as a real shapely geometry, at the node's own scale.

    A circle is a `Point.buffer`, a rectangle a `box` centred on the node's position. Which corner a
    rectangle's `x`/`y` names is not stated by any committed document, so every assertion below is
    written to be invariant under that choice: they compare a TRANSFORM of the before footprint with
    the after footprint, and both are built by this one function."""
    scale = node.get("scale", 1.0)
    # 🫥️`shape` is omitted whenever it holds its default, so the extent MEMBERS are what say which
    # figure this is: a radius is a circle, a width and a height are a rectangle.
    if "radius" in node:
        base = Point(node["x"], node["y"]).buffer(node["radius"], quad_segs=CIRCLE_SEGMENTS)
    elif "width" in node and "height" in node:
        half_width = node["width"] / 2.0
        half_height = node["height"] / 2.0
        base = box(node["x"] - half_width, node["y"] - half_height, node["x"] + half_width, node["y"] + half_height)
    else:
        return None
    if scale == 1.0:
        return base
    return affinity.scale(base, xfact=scale, yfact=scale, origin=(node["x"], node["y"]))


def node_by_id(document, identity):
    """🔎️ One node record, or `None` when the board does not hold it."""
    for node in document["nodes"]:
        if node["id"] == identity:
            return node
    return None


def edge_by_id(document, identity):
    """🔎️ One edge record, or `None` when the board does not hold it."""
    for edge in document["edges"]:
        if edge["id"] == identity:
            return edge
    return None


def geometry_transforms(ctx):
    """📐 shapely answers position, scale, anchor invariance and the null-dropping extent rebuild."""
    rows = []
    failures = []
    for vector in vectors(ctx):
        if vector["before"] is None or vector["after"] is None:
            continue
        # 🚦️A rejected or no-op vector reaches the footprint-invariance branch below on purpose: what
        # shapely must then confirm is that NOTHING moved, which is the whole content of a refusal.
        kind = vector["kind"] if moves_document(vector) else "declared-still"
        payload = payload_of(vector)
        checks = 0
        if kind in GEOMETRY_KINDS:
            before_node = node_by_id(vector["before"], payload["id"])
            after_node = node_by_id(vector["after"], payload["id"])
            if before_node is None or after_node is None:
                failures.append("%s: kind %s addresses node %r, which one of the two snapshots does not hold" % (vector["id"], kind, payload["id"]))
                continue
            before_shape = footprint(before_node)
            after_shape = footprint(after_node)
            if before_shape is None or after_shape is None:
                failures.append("%s: node %r carries neither a radius nor a width and a height, so it has no footprint" % (vector["id"], payload["id"]))
                continue
            checks += 1
            if kind == "move-node":
                moved = affinity.translate(before_shape, xoff=payload["newX"] - before_node["x"], yoff=payload["newY"] - before_node["y"])
                if not moved.equals_exact(after_shape, TOLERANCE):
                    failures.append("%s: shapely's translate of the before footprint does not equal the after footprint (bounds %r vs %r)" % (vector["id"], moved.bounds, after_shape.bounds))
                if abs(moved.area - before_shape.area) > TOLERANCE:
                    failures.append("%s: a translation changed the footprint's area" % vector["id"])
            elif kind == "scale-node":
                factor = payload["newScale"] / before_node.get("scale", 1.0)
                scaled = affinity.scale(before_shape, xfact=factor, yfact=factor, origin=(before_node["x"], before_node["y"]))
                if not scaled.equals_exact(after_shape, TOLERANCE):
                    failures.append("%s: shapely's scale about the node's own position does not equal the after footprint (bounds %r vs %r)" % (vector["id"], scaled.bounds, after_shape.bounds))
                if abs(scaled.area - before_shape.area * factor * factor) > max(TOLERANCE, abs(before_shape.area) * 1e-9):
                    failures.append("%s: scaling by %r did not multiply the area by its square" % (vector["id"], factor))
            elif kind == "change-node-anchor":
                if not before_shape.equals_exact(after_shape, TOLERANCE):
                    failures.append("%s: changing the anchor moved the footprint, which is a board concept and not a geometric one" % vector["id"])
            elif kind == "replace-node-geometry":
                expected = math.pi * payload["newRadius"] ** 2 if payload.get("newRadius") is not None else payload["newWidth"] * payload["newHeight"]
                if abs(after_shape.area - expected) > abs(expected) * 1e-6:
                    failures.append("%s: the rebuilt %s has area %r where its arguments state %r" % (vector["id"], payload.get("newShape"), after_shape.area, expected))
                if abs(after_shape.centroid.x - before_node["x"]) > TOLERANCE or abs(after_shape.centroid.y - before_node["y"]) > TOLERANCE:
                    failures.append("%s: rebuilding the extent moved the node's centroid" % vector["id"])
                for member, argument in (("radius", "newRadius"), ("width", "newWidth"), ("height", "newHeight")):
                    if payload.get(argument) is None and member in after_node:
                        failures.append("%s: argument %s is null yet the after-snapshot still carries %s" % (vector["id"], argument, member))
                checks += 1
        elif kind in ("replace-edge-geometry", "connect-handles"):
            identity = payload["id"]
            after_edge = edge_by_id(vector["after"], identity)
            if after_edge is None:
                failures.append("%s: kind %s addresses edge %r, which the after-snapshot does not hold" % (vector["id"], kind, identity))
                continue
            target_x = payload["newX"] if kind == "replace-edge-geometry" else payload["x"]
            target_y = payload["newY"] if kind == "replace-edge-geometry" else payload["y"]
            origin = edge_by_id(vector["before"], identity) or {"x": 0.0, "y": 0.0}
            moved = affinity.translate(Point(origin["x"], origin["y"]), xoff=target_x - origin["x"], yoff=target_y - origin["y"])
            checks += 1
            if not moved.equals_exact(Point(after_edge["x"], after_edge["y"]), TOLERANCE):
                failures.append("%s: shapely's translate of the edge's routing origin lands at %r, the after-snapshot carries (%r, %r)" % (vector["id"], (moved.x, moved.y), after_edge["x"], after_edge["y"]))
        else:
            for node in vector["before"]["nodes"]:
                shape = footprint(node)
                if shape is None:
                    failures.append("%s: node %r carries neither a radius nor a width and a height, so it has no footprint" % (vector["id"], node["id"]))
                    continue
                checks += 1
                if shape.is_empty or not shape.is_valid:
                    failures.append("%s: node %r has no valid footprint" % (vector["id"], node["id"]))
                after_node = node_by_id(vector["after"], node["id"])
                if after_node is not None and not shape.equals_exact(footprint(after_node), TOLERANCE):
                    failures.append("%s: kind %s moved node %r's footprint, which is not a geometry verb" % (vector["id"], kind, node["id"]))
        rows.append({"id": vector["id"], "kind": vector["kind"], "checks": checks, "verb": kind})
    return report("geometry-transforms", rows, failures)
# endregion 🔖️Geometry


# region 🔖️Schema
def payload_schemas(ctx):
    """🧬️ jsonschema answers every committed payload against its own leaf draft-07 schema."""
    rows = []
    failures = []
    for vector in vectors(ctx):
        if vector["mutation"] is None:
            continue
        checks = 0
        if vector["mutation"].get("mutation") != tag_of(vector["kind"]):
            failures.append("%s: the committed payload is tagged %r where its leaf declares %s" % (vector["id"], vector["mutation"].get("mutation"), vector["kind"]))
        checks += 1
        schema = vector["schema"]
        if schema is None:
            failures.append("%s: the mutation leaf carries no %s" % (vector["id"], LEAF_SCHEMA))
            rows.append({"id": vector["id"], "kind": vector["kind"], "checks": checks})
            continue
        validator_class = jsonschema.validators.validator_for(schema)
        validator_class.check_schema(schema)
        validator = validator_class(schema)
        checks += 1
        errors = sorted(validator.iter_errors(vector["mutation"]), key=lambda error: list(error.absolute_path))
        for error in errors:
            failures.append("%s: %s rejects the committed payload at /%s — %s" % (vector["id"], validator_class.__name__, "/".join(str(part) for part in error.absolute_path), error.message))
        # 🧪️A validator that accepts everything would accept the payload too. The probe proves the
        # opposite by handing it a member the schema does not declare.
        if schema.get("additionalProperties") is False:
            probe = dict(vector["mutation"])
            probe["semioThirdPartyOracleProbe"] = True
            checks += 1
            if validator.is_valid(probe):
                failures.append("%s: the leaf schema declares additionalProperties false yet %s accepted an undeclared member" % (vector["id"], validator_class.__name__))
        rows.append({"id": vector["id"], "kind": vector["kind"], "checks": checks, "draft": schema.get("$schema", ""), "title": schema.get("title", "")})
    return report("payload-schemas", rows, failures)
# endregion 🔖️Schema


# region 🔖️Diff
def touched_members(patch):
    """🔺 The top-level snapshot members an RFC 6902 patch reaches, read off its op PATHS."""
    return {op["path"].lstrip("/").split("/")[0] for op in patch}


def collection_ids(document, member):
    """🆔 The ids a snapshot collection carries, in board order."""
    return [record["id"] for record in document[member]]


def patched_ids(before, after, member):
    """🔺 The ids jsonpatch says CHANGED while surviving — the typed diff's `patched` set.

    Deliberately computed per RECORD rather than by reading indices off the whole-document patch:
    removing an entry from the middle of a collection shifts every later index, and a whole-document
    patch then expresses a removal as field edits on records that did not change at all. Asking
    `make_patch` for each surviving record's own patch is immune to that and is a stronger statement
    — a record is patched exactly when RFC 6902 needs at least one operation to turn its
    before-shape into its after-shape."""
    survivors = {record["id"]: record for record in after[member]}
    reached = set()
    for record in before[member]:
        twin = survivors.get(record["id"])
        if twin is not None and list(jsonpatch.make_patch(record, twin)):
            reached.add(record["id"])
    return reached


def diff_reproduction(ctx):
    """🔺 jsonpatch reproduces every committed after-snapshot and holds the typed diff to its ops."""
    rows = []
    failures = []
    for vector in vectors(ctx):
        if vector["before"] is None or vector["after"] is None:
            continue
        patch = jsonpatch.make_patch(vector["before"], vector["after"])
        operations = list(patch)
        checks = 1
        if patch.apply(vector["before"]) != vector["after"]:
            failures.append("%s: applying the RFC 6902 patch jsonpatch derived does not reproduce the committed after-snapshot" % vector["id"])
        # 🤝️Two unrelated algorithms must agree on whether the document moved at all.
        deep = DeepDiff(vector["before"], vector["after"], ignore_order=False)
        checks += 1
        if bool(deep) != bool(operations):
            failures.append("%s: deepdiff reports %s while jsonpatch derived %d op(s)" % (vector["id"], "a change" if deep else "no change", len(operations)))
        checks += 1
        if not moves_document(vector) and operations:
            failures.append("%s: the committed outcome is %r yet jsonpatch derived %d op(s): %r" % (vector["id"], vector["outcome"], len(operations), operations))
        if moves_document(vector) and not operations:
            failures.append("%s: the committed vector declares the kind applied yet neither library found a change" % vector["id"])
        if vector["diff"] is None:
            rows.append({"id": vector["id"], "kind": vector["kind"], "checks": checks, "ops": len(operations), "note": "no committed diff"})
            continue
        declared = {member for member in MEMBERS if vector["diff"].get(member) is not None}
        checks += 1
        if declared != touched_members(operations):
            failures.append("%s: the typed diff declares %r, the RFC 6902 patch touches %r" % (vector["id"], sorted(declared), sorted(touched_members(operations))))
        for member in ("nodes", "edges"):
            delta = vector["diff"].get(member)
            if delta is None:
                continue
            before_ids = collection_ids(vector["before"], member)
            after_ids = collection_ids(vector["after"], member)
            checks += 3
            if sorted(delta.get("removed", [])) != sorted(set(before_ids) - set(after_ids)):
                failures.append("%s: the typed diff removes %r from %s, the two snapshots differ by %r" % (vector["id"], sorted(delta.get("removed", [])), member, sorted(set(before_ids) - set(after_ids))))
            if sorted(record["id"] for record in delta.get("added", [])) != sorted(set(after_ids) - set(before_ids)):
                failures.append("%s: the typed diff adds %r to %s, the two snapshots differ by %r" % (vector["id"], sorted(record["id"] for record in delta.get("added", [])), member, sorted(set(after_ids) - set(before_ids))))
            declared_patched = {entry["id"] for entry in delta.get("patched", [])}
            reached = patched_ids(vector["before"], vector["after"], member)
            if declared_patched != reached:
                failures.append("%s: the typed diff patches %r in %s, jsonpatch needs operations for %r" % (vector["id"], sorted(declared_patched), member, sorted(reached)))
            for entry in delta.get("patched", []):
                replacement = entry.get("patch", {}).get("replacement")
                if replacement is None:
                    continue
                checks += 1
                if replacement != next((record for record in vector["after"][member] if record["id"] == entry["id"]), None):
                    failures.append("%s: the typed diff's replacement for %s %r does not equal the committed after-snapshot's record" % (vector["id"], member, entry["id"]))
        rows.append({"id": vector["id"], "kind": vector["kind"], "checks": checks, "ops": len(operations), "members": sorted(declared)})
    return report("diff-reproduction", rows, failures)
# endregion 🔖️Diff


# region 🔖️Registration
def adapter():
    """🧭️ Registration in the ORACLE role only. These libraries are the reference; this repository's
    Rust and its Python second implementation are the subjects, and neither is registered here."""
    return (
        Adapter("python")
        .oracle("graph-cascade", graph_cascade)
        .oracle("kind-compatibility", kind_compatibility)
        .oracle("geometry-transforms", geometry_transforms)
        .oracle("payload-schemas", payload_schemas)
        .oracle("diff-reproduction", diff_reproduction)
    )
# endregion 🔖️Registration

"""🧾️ `stdio.semio.graph` mutation verbs: how each verb is applied to the parsed document and the undo steps it owes, read off the committed specification vectors."""

from __future__ import annotations

# region 🔖️Imports
import json

from mutate_semio_graph_text import word, number, point_word, point_number
# endregion 🔖️Imports


# region 🔖️Mutations
#: 🧱️ The incident edges ONE `delete-node` severs at most. The committed leaf payload schema
#: `…/🧬️mutations/🗑️delete-node/🧬️schema/🔣️.json` declares `x-semio-inverse-rows: {"bounded": 1025}`: the undo is the
#: row that restores the node plus one row per severed edge, so the 1025th incident edge makes the delete a refusal
#: (`mutation.target-referenced`) that changes nothing and has nothing to undo.
DELETE_NODE_CASCADE_EDGES_MAXIMUM = 1024

#: 🏷️ The externally tagged JSON name of each kebab-case kind, as the committed specification
#: vectors under `…/🧬️mutations/<kind>/🧪️tests/<fixture>/🦠️mutation/` spell it.
TAG_OF_KIND = {
    "create-node": "CreateNode",
    "delete-node": "DeleteNode",
    "change-node-kind": "ChangeNodeKind",
    "change-node-label": "ChangeNodeLabel",
    "move-node": "MoveNode",
    "add-node-port": "AddNodePort",
    "remove-node-port": "RemoveNodePort",
    "add-node-property": "AddNodeProperty",
    "remove-node-property": "RemoveNodeProperty",
    "create-edge": "CreateEdge",
    "delete-edge": "DeleteEdge",
    "drag-nodes": "DragNodes",
    "set-node-property": "SetNodeProperty",
    "resize-node": "ResizeNode",
    "rename-node": "RenameNode",
    "set-edge-property": "SetEdgeProperty",
    "add-edge-property": "AddEdgeProperty",
    "remove-edge-property": "RemoveEdgeProperty",
}


def clone(value):
    return json.loads(json.dumps(value))


def tagged(mutation: dict) -> tuple:
    """🔎️ Splits `{"CreateNode": {…}}` into its verb and its arguments."""
    if len(mutation) != 1:
        raise AssertionError("a mutation is exactly one externally tagged verb, got %r" % sorted(mutation))
    tag = next(iter(mutation))
    if tag not in TAG_OF_KIND.values():
        raise AssertionError("unknown verb %r — the vocabulary is %s" % (tag, ", ".join(sorted(TAG_OF_KIND.values()))))
    return tag, mutation[tag]


def node_at(document: dict, node_id: dict, verb: str) -> dict:
    """🔎️ The node one verb addresses. An id no node carries is a refusal, never a no-op."""
    for node in document["nodes"]:
        if node["id"] == node_id:
            return node
    raise AssertionError("%s addresses node %r, which the graph does not carry" % (verb, node_id))


def slot_index(items: list, index, verb: str, what: str, inclusive: bool) -> int:
    """🔎️ A positional index into a node's own ordered `ports`/`properties` list."""
    limit = len(items) if inclusive else len(items) - 1
    if not isinstance(index, int) or index < 0 or index > limit:
        raise AssertionError("%s addresses %s %r of a node carrying %d" % (verb, what, index, len(items)))
    return index


def insert_index(items: list, at) -> int:
    """📍️ Where a created record lands: its `at` clamped to the end, or the end when `at` is absent."""
    return len(items) if at is None else min(at, len(items))


def property_of(owner: dict, key: str, verb: str) -> dict:
    """🔎️ The keyed property one verb addresses on a node or an edge. A key it does not carry is a refusal, never a no-op."""
    for entry in owner.get("properties", []):
        if entry["key"] == key:
            return entry
    raise AssertionError("%s addresses property %r, which %r does not carry" % (verb, key, owner["id"]))


def edge_at(document: dict, edge_id: dict, verb: str) -> dict:
    """🔎️ The edge one verb addresses. An id no edge carries is a refusal, never a no-op."""
    for edge in document["edges"]:
        if edge["id"] == edge_id:
            return edge
    raise AssertionError("%s addresses edge %r, which the graph does not carry" % (verb, edge_id))


def severed_edges(document: dict, node_id: dict, verb: str) -> list:
    """✂️ `(index, edge)` of every edge a `delete-node` of `node_id` severs, in document order. More than
    `DELETE_NODE_CASCADE_EDGES_MAXIMUM` of them is a refusal, never a partial cascade."""
    severed = [(at, edge) for at, edge in enumerate(document["edges"]) if edge["source"] == node_id or edge["target"] == node_id]
    if len(severed) > DELETE_NODE_CASCADE_EDGES_MAXIMUM:
        raise AssertionError("%s addresses node %r, which %d edges still reference — one delete severs at most %d" % (verb, node_id, len(severed), DELETE_NODE_CASCADE_EDGES_MAXIMUM))
    return severed


def apply_mutation(document: dict, mutation: dict) -> dict:
    """🧬️ Applies one verb, returning a NEW document. `delete-node` CASCADES into every edge with
    that node as source or target, which is the behaviour the committed `removes-the-sink-node-and-
    severs-the-edge-into-it` vector pins. An unaddressable id or index is a refusal, never a silent
    no-op — a quietly skipped mutation would report as a pass."""
    result = clone(document)
    tag, args = tagged(mutation)
    if tag == "CreateNode":
        if any(node["id"] == args["id"] for node in result["nodes"]):
            raise AssertionError("CreateNode uses id %r, which the graph already carries" % args["id"])
        created = {"id": clone(args["id"]), "kind": args["kind"], "label": args["label"], "position": point_word(args["position"]), "width": word(args["width"]), "height": word(args["height"]), "ports": clone(args["ports"]), "properties": clone(args["properties"])}
        result["nodes"].insert(insert_index(result["nodes"], args.get("at")), created)
    elif tag == "DeleteNode":
        node_at(result, args["id"], tag)
        severed = {at for at, _ in severed_edges(result, args["id"], tag)}
        result["nodes"] = [node for node in result["nodes"] if node["id"] != args["id"]]
        result["edges"] = [edge for at, edge in enumerate(result["edges"]) if at not in severed]
    elif tag == "ChangeNodeKind":
        node_at(result, args["id"], tag)["kind"] = args["new_kind"]
    elif tag == "ChangeNodeLabel":
        node_at(result, args["id"], tag)["label"] = args["new_label"]
    elif tag == "MoveNode":
        node_at(result, args["id"], tag)["position"] = point_word(args["new_position"])
    elif tag == "AddNodePort":
        ports = node_at(result, args["node_id"], tag)["ports"]
        ports.insert(slot_index(ports, args["index"], tag, "port", True), clone(args["port"]))
    elif tag == "RemoveNodePort":
        ports = node_at(result, args["node_id"], tag)["ports"]
        del ports[slot_index(ports, args["index"], tag, "port", False)]
    elif tag == "AddNodeProperty":
        properties = node_at(result, args["node_id"], tag)["properties"]
        properties.insert(slot_index(properties, args["index"], tag, "property", True), clone(args["property"]))
    elif tag == "RemoveNodeProperty":
        node = node_at(result, args["node_id"], tag)
        node["properties"].remove(property_of(node, args["key"], tag))
    elif tag == "DragNodes":
        present = [node for node in result["nodes"] if node["id"] in args["targets"]]
        if not present:
            raise AssertionError("DragNodes addresses none of the graph's nodes: %r" % args["targets"])
        for node in present:
            node["position"] = point_word({"x": number(node["position"]["x"]) + args["dx"], "y": number(node["position"]["y"]) + args["dy"]})
    elif tag == "SetNodeProperty":
        property_of(node_at(result, args["node_id"], tag), args["key"], tag)["value"] = clone(args["value"])
    elif tag == "ResizeNode":
        if not all(isinstance(args[side], (int, float)) and args[side] >= 0 for side in ("width", "height")):
            raise AssertionError("ResizeNode needs a finite non-negative size, got %r" % args)
        node = node_at(result, args["id"], tag)
        node["width"], node["height"] = word(args["width"]), word(args["height"])
    elif tag == "RenameNode":
        node = node_at(result, args["id"], tag)
        if not args["new_id"]["value"] or any(other["id"] == args["new_id"] for other in result["nodes"]):
            raise AssertionError("RenameNode needs a free non-empty id, got %r" % args["new_id"])
        node["id"] = clone(args["new_id"])
        for edge in result["edges"]:
            for end in ("source", "target"):
                if edge[end] == args["id"]:
                    edge[end] = clone(args["new_id"])
    elif tag == "SetEdgeProperty":
        property_of(edge_at(result, args["edge_id"], tag), args["key"], tag)["value"] = clone(args["value"])
    elif tag == "AddEdgeProperty":
        properties = edge_at(result, args["edge_id"], tag)["properties"]
        if any(entry["key"] == args["property"]["key"] for entry in properties):
            raise AssertionError("AddEdgeProperty attaches key %r, which the edge already carries" % args["property"]["key"])
        properties.insert(min(args["index"], len(properties)), clone(args["property"]))
    elif tag == "RemoveEdgeProperty":
        edge = edge_at(result, args["edge_id"], tag)
        edge["properties"].remove(property_of(edge, args["key"], tag))
    elif tag == "CreateEdge":
        if any(edge["id"] == args["id"] for edge in result["edges"]):
            raise AssertionError("CreateEdge uses id %r, which the graph already carries" % args["id"])
        created = {"id": clone(args["id"]), "source": clone(args["source"]), "target": clone(args["target"]), "kind": args["kind"], "label": args["label"], "properties": clone(args.get("properties", []))}
        for member, key in (("sourcePort", "source_port"), ("targetPort", "target_port")):
            if args.get(key) is not None:
                created[member] = args[key]
        result["edges"].insert(insert_index(result["edges"], args.get("at")), created)
    else:
        if not any(edge["id"] == args["id"] for edge in result["edges"]):
            raise AssertionError("DeleteEdge addresses edge %r, which the graph does not carry" % args["id"])
        result["edges"] = [edge for edge in result["edges"] if edge["id"] != args["id"]]
    return result


def edge_mutation(edge: dict, at: int) -> dict:
    """➡️ The `CreateEdge` that puts one edge back exactly as it was, at the index it held."""
    return {"CreateEdge": {"id": clone(edge["id"]), "source": clone(edge["source"]), "target": clone(edge["target"]), "kind": edge["kind"], "label": edge["label"], "source_port": edge.get("sourcePort"), "target_port": edge.get("targetPort"), "properties": clone(edge["properties"]), "at": at}}


def inverse_mutation(document: dict, mutation: dict) -> list:
    """↩️ The undo of one verb against the state it was applied to. Derived from the verbs' own
    meanings — an append is undone by the matching delete, an overwrite by an overwrite with the
    value it displaced, and a cascading delete by re-creating the node AND every edge it severed."""
    tag, args = tagged(mutation)
    if tag == "CreateNode":
        return [{"DeleteNode": {"id": clone(args["id"])}}]
    if tag == "DeleteNode":
        node = node_at(document, args["id"], tag)
        steps = [{"CreateNode": {"id": clone(node["id"]), "kind": node["kind"], "label": node["label"], "position": point_number(node["position"]), "width": number(node["width"]), "height": number(node["height"]), "ports": clone(node["ports"]), "properties": clone(node["properties"]), "at": document["nodes"].index(node)}}]
        steps.extend(edge_mutation(edge, at) for at, edge in severed_edges(document, args["id"], tag))
        return steps
    if tag == "ChangeNodeKind":
        return [{"ChangeNodeKind": {"id": clone(args["id"]), "new_kind": node_at(document, args["id"], tag)["kind"]}}]
    if tag == "ChangeNodeLabel":
        return [{"ChangeNodeLabel": {"id": clone(args["id"]), "new_label": node_at(document, args["id"], tag)["label"]}}]
    if tag == "MoveNode":
        return [{"MoveNode": {"id": clone(args["id"]), "new_position": point_number(node_at(document, args["id"], tag)["position"])}}]
    if tag == "AddNodePort":
        return [{"RemoveNodePort": {"node_id": clone(args["node_id"]), "index": args["index"]}}]
    if tag == "RemoveNodePort":
        ports = node_at(document, args["node_id"], tag)["ports"]
        index = slot_index(ports, args["index"], tag, "port", False)
        return [{"AddNodePort": {"node_id": clone(args["node_id"]), "index": index, "port": clone(ports[index])}}]
    if tag == "AddNodeProperty":
        return [{"RemoveNodeProperty": {"node_id": clone(args["node_id"]), "key": args["property"]["key"]}}]
    if tag == "RemoveNodeProperty":
        node = node_at(document, args["node_id"], tag)
        entry = property_of(node, args["key"], tag)
        return [{"AddNodeProperty": {"node_id": clone(args["node_id"]), "index": node["properties"].index(entry), "property": clone(entry)}}]
    if tag == "DragNodes":
        seen = []
        for target in args["targets"]:
            if target not in seen:
                seen.append(target)
        return [{"MoveNode": {"id": clone(node["id"]), "new_position": point_number(node["position"])}} for target in seen for node in document["nodes"] if node["id"] == target]
    if tag == "SetNodeProperty":
        return [{"SetNodeProperty": {"node_id": clone(args["node_id"]), "key": args["key"], "value": clone(property_of(node_at(document, args["node_id"], tag), args["key"], tag)["value"])}}]
    if tag == "ResizeNode":
        node = node_at(document, args["id"], tag)
        return [{"ResizeNode": {"id": clone(args["id"]), "width": number(node["width"]), "height": number(node["height"])}}]
    if tag == "RenameNode":
        return [{"RenameNode": {"id": clone(args["new_id"]), "new_id": clone(args["id"])}}]
    if tag == "SetEdgeProperty":
        return [{"SetEdgeProperty": {"edge_id": clone(args["edge_id"]), "key": args["key"], "value": clone(property_of(edge_at(document, args["edge_id"], tag), args["key"], tag)["value"])}}]
    if tag == "AddEdgeProperty":
        return [{"RemoveEdgeProperty": {"edge_id": clone(args["edge_id"]), "key": args["property"]["key"]}}]
    if tag == "RemoveEdgeProperty":
        edge = edge_at(document, args["edge_id"], tag)
        entry = property_of(edge, args["key"], tag)
        return [{"AddEdgeProperty": {"edge_id": clone(args["edge_id"]), "index": edge["properties"].index(entry), "property": clone(entry)}}]
    if tag == "CreateEdge":
        return [{"DeleteEdge": {"id": clone(args["id"])}}]
    for at, edge in enumerate(document["edges"]):
        if edge["id"] == args["id"]:
            return [edge_mutation(edge, at)]
    raise AssertionError("DeleteEdge addresses edge %r, which the graph does not carry" % args["id"])


def apply_all(document: dict, mutations: list) -> dict:
    """🧬️ Folds a list of verbs over a document, left to right."""
    for mutation in mutations:
        document = apply_mutation(document, mutation)
    return document


# endregion 🔖️Mutations

"""🧾️ `stdio.semio.drawing` mutation verbs: how each verb is applied to the parsed document and the undo steps it owes, read off the committed specification vectors."""

from __future__ import annotations

# region 🔖️Imports
import json

from mutate_semio_drawing_text import to_f32
# endregion 🔖️Imports


# region 🔖️Mutations
VERBS = {
    "CreateLayer": "create-layer",
    "DeleteLayer": "delete-layer",
    "CreateNode": "create-node",
    "DeleteNode": "delete-node",
    "MoveNode": "move-node",
    "DragNodes": "drag-nodes",
    "RotateNode": "rotate-node",
    "ScaleNode": "scale-node",
    "ReorderNodes": "reorder-nodes",
    "GroupNodes": "group-nodes",
    "UngroupNode": "ungroup-node",
    "FlattenNode": "flatten-node",
    "UnflattenNode": "unflatten-node",
    "ReplacePath": "replace-path",
    "ReplaceFill": "replace-fill",
    "ChangeStrokeColor": "change-stroke-color",
    "ChangeStrokeWidth": "change-stroke-width",
}


def clone(value):
    """🧬️ A deep copy, so no arm ever aliases the document it was handed."""
    return json.loads(json.dumps(value))


def single(colour):
    """🎨️ A wire colour narrowed to the single precision the model actually holds."""
    return None if colour is None else {key: to_f32(colour[key]) for key in ("r", "g", "b", "a")}


def verb_of(mutation: dict) -> tuple:
    """🏷️ The externally tagged wire form: one key, the variant's own `PascalCase` name, whose value
    carries the verb's `snake_case` arguments."""
    keys = list(mutation.keys())
    if len(keys) != 1 or keys[0] not in VERBS:
        raise AssertionError("%r is not one of this subset's eighteen declared verbs" % (keys,))
    return keys[0], mutation[keys[0]]


def parent_of(document: dict, at: dict, verb: str) -> list:
    """🌳️ The child list a `NodePath` addresses INTO — `layer` selects the layer, `path` is a chain of
    child indices from its root, and the last index is the position inside the list this returns."""
    layer = at["layer"]
    if layer < 0 or layer >= len(document["layers"]):
        raise AssertionError("%s addresses layer %d of a %d-layer drawing" % (verb, layer, len(document["layers"])))
    node = document["layers"][layer]["root"]
    for index in at["path"]:
        if node["kind"] != "group":
            raise AssertionError("%s walks into a %s node, which carries no children" % (verb, node["kind"]))
        children = node.get("children", [])
        if index < 0 or index >= len(children):
            raise AssertionError("%s addresses child %d of a %d-child group" % (verb, index, len(children)))
        node = children[index]
    return node


def node_at(document: dict, at: dict, verb: str) -> dict:
    """🌳️ The node a `NodePath` addresses. An empty `path` is the layer's own root."""
    return parent_of(document, at, verb)


def container_of(document: dict, at: dict, verb: str) -> tuple:
    """🌳️ The child list holding the addressed node, and its index inside it."""
    if not at["path"]:
        raise AssertionError("%s addresses a layer root, which has no containing list" % verb)
    owner = node_at(document, {"layer": at["layer"], "path": at["path"][:-1]}, verb)
    if owner["kind"] != "group":
        raise AssertionError("%s addresses a child of a %s node, which carries no children" % (verb, owner["kind"]))
    return owner.setdefault("children", []), at["path"][-1]


IDENTITY = {"translation": {"x": 0.0, "y": 0.0, "z": 0.0}, "rotation": {"x": 0.0, "y": 0.0, "z": 0.0, "w": 1.0}, "scale": {"x": 1.0, "y": 1.0, "z": 1.0}}


def leaves_of(node: dict) -> list:
    """🫓 Every non-group descendant of a node, in depth-first order — what `flatten` keeps."""
    if node["kind"] != "group":
        return [clone(node)]
    found = []
    for child in node.get("children", []):
        found.extend(leaves_of(child))
    return found


def transformed_descendant_group(node: dict) -> bool:
    """🫓 Whether any DESCENDANT group carries a transform other than the identity. Dissolving such a
    group would silently drop the transform its children are drawn under, so `flatten` refuses the
    whole mutation when one is present — the specification vector's own slug says as much
    (`flattens-an-IDENTITY-nested-group-into-its-leaves`), and the addressed node's own transform is
    not at issue because `flatten` keeps the node itself."""
    for child in node.get("children", []):
        if child["kind"] != "group":
            continue
        if child["transform"] != IDENTITY or transformed_descendant_group(child):
            return True
    return False


def style_of(document: dict, name: str, verb: str) -> dict:
    """🎨️ The named style a style verb addresses."""
    for style in document["styles"]:
        if style["name"] == name:
            return style
    raise AssertionError("%s addresses the style %r, which the drawing does not carry" % (verb, name))


def set_optional(holder: dict, key: str, value) -> None:
    """🎨️ Writes an optional leaf the way the wire form spells it: absent rather than null."""
    if value is None:
        holder.pop(key, None)
        return
    holder[key] = value


def apply_mutation(document: dict, mutation: dict) -> dict:
    """▶️ One verb applied to a drawing, returning the resulting drawing.

    Each arm is the behaviour its committed `(before, mutation, after)` specification vector states:
    layers are an ordered list with an id-keyed removal, nodes are addressed by a structural
    `NodePath`, and the four hierarchy verbs are the ones the vectors pin most sharply — `group`
    replaces the addressed children with one new group AT THE FIRST INDEX, `ungroup` splices a
    group's children back into its parent in place, `flatten` replaces a group's children with all
    its LEAF descendants in depth-first order, and `unflatten` puts a captured node back.
    """
    verb, argument = verb_of(mutation)
    kind = VERBS[verb]
    result = clone(document)
    if kind == "create-layer":
        index = argument["index"]
        if index < 0 or index > len(result["layers"]):
            raise AssertionError("create-layer addresses position %d of a %d-layer drawing" % (index, len(result["layers"])))
        result["layers"].insert(index, clone(argument["layer"]))
        return result
    if kind == "delete-layer":
        if not any(layer["id"] == argument["id"] for layer in result["layers"]):
            raise AssertionError("delete-layer addresses %r, which the drawing does not carry" % argument["id"])
        result["layers"] = [layer for layer in result["layers"] if layer["id"] != argument["id"]]
        return result
    if kind == "create-node":
        parent = node_at(result, argument["parent"], "create-node")
        if parent["kind"] != "group":
            raise AssertionError("create-node addresses a %s node, which carries no children" % parent["kind"])
        children = parent.setdefault("children", [])
        index = argument["index"]
        if index < 0 or index > len(children):
            raise AssertionError("create-node addresses position %d of a %d-child group" % (index, len(children)))
        children.insert(index, clone(argument["node"]))
        return result
    if kind == "delete-node":
        children, index = container_of(result, argument["at"], "delete-node")
        children.pop(index)
        return result
    if kind == "move-node":
        node = node_at(result, argument["at"], "move-node")
        origin = argument["new_origin"]
        if node["kind"] in ("text", "image"):
            node["at"] = clone(origin)
            return result
        if node["kind"] == "group":
            node["transform"]["translation"]["x"] = origin["x"]
            node["transform"]["translation"]["y"] = origin["y"]
            return result
        raise AssertionError("move-node addresses a %s node, which carries no origin" % node["kind"])
    if kind == "drag-nodes":
        offset = argument["offset"]
        for at in argument["ats"]:
            node = node_at(result, at, "drag-nodes")
            if node["kind"] in ("text", "image"):
                node["at"]["x"] += offset["x"]
                node["at"]["y"] += offset["y"]
                continue
            if node["kind"] == "group":
                node["transform"]["translation"]["x"] += offset["x"]
                node["transform"]["translation"]["y"] += offset["y"]
                continue
            raise AssertionError("drag-nodes addresses a %s node, which carries no origin" % node["kind"])
        return result
    if kind == "rotate-node":
        node = node_at(result, argument["at"], "rotate-node")
        if node["kind"] != "group":
            raise AssertionError("rotate addresses a %s node, which carries no transform" % node["kind"])
        node["transform"]["rotation"] = clone(argument["new_rotation"])
        return result
    if kind == "scale-node":
        node = node_at(result, argument["at"], "scale-node")
        if node["kind"] != "group":
            raise AssertionError("scale addresses a %s node, which carries no transform" % node["kind"])
        node["transform"]["scale"] = clone(argument["new_scale"])
        return result
    if kind == "reorder-nodes":
        parent = node_at(result, argument["parent"], "reorder-nodes")
        if parent["kind"] != "group":
            raise AssertionError("reorder-nodes addresses a %s node, which carries no children" % parent["kind"])
        children = parent.setdefault("children", [])
        source = argument["from"]
        target = argument["to"]
        for index in (source, target):
            if index < 0 or index >= len(children):
                raise AssertionError("reorder-nodes addresses child %d of a %d-child group" % (index, len(children)))
        children.insert(target, children.pop(source))
        return result
    if kind == "group-nodes":
        parent = node_at(result, argument["parent"], "group-nodes")
        if parent["kind"] != "group":
            raise AssertionError("group addresses a %s node, which carries no children" % parent["kind"])
        children = parent.setdefault("children", [])
        indices = sorted(argument["indices"])
        for index in indices:
            if index < 0 or index >= len(children):
                raise AssertionError("group addresses child %d of a %d-child group" % (index, len(children)))
        taken = [clone(children[index]) for index in argument["indices"]]
        for index in reversed(indices):
            children.pop(index)
        children.insert(indices[0], {"kind": "group", "transform": clone(argument["transform"]), "children": taken})
        return result
    if kind == "ungroup-node":
        children, index = container_of(result, argument["at"], "ungroup-node")
        node = children[index]
        if node["kind"] != "group":
            raise AssertionError("ungroup addresses a %s node, which is not a group" % node["kind"])
        children[index : index + 1] = clone(node.get("children", []))
        return result
    if kind == "flatten-node":
        node = node_at(result, argument["at"], "flatten-node")
        if node["kind"] != "group":
            raise AssertionError("flatten addresses a %s node, which is not a group" % node["kind"])
        if transformed_descendant_group(node):
            return result
        node["children"] = leaves_of(node)
        return result
    if kind == "unflatten-node":
        at = argument["at"]
        if not at["path"]:
            result["layers"][at["layer"]]["root"] = clone(argument["original"])
            return result
        children, index = container_of(result, at, "unflatten-node")
        children[index] = clone(argument["original"])
        return result
    if kind == "replace-path":
        node = node_at(result, argument["at"], "replace-path")
        if node["kind"] != "path":
            raise AssertionError("replace-path addresses a %s node, which carries no segments" % node["kind"])
        node["segments"] = clone(argument["new_segments"])
        return result
    if kind == "replace-fill":
        set_optional(style_of(result, argument["style_name"], "replace-fill"), "fill", single(argument["new_fill"]))
        return result
    if kind == "change-stroke-color":
        set_optional(style_of(result, argument["style_name"], "change-stroke-color"), "stroke", single(argument["new_color"]))
        return result
    set_optional(style_of(result, argument["style_name"], "change-stroke-width"), "strokeWidth", argument["new_width"])
    return result


def inverse_mutation(document: dict, mutation: dict) -> list:
    """↩️ The verb's own inverse against the drawing it is about to be applied to, as the ordered
    sequence of verbs that restores it. Every hierarchy verb's inverse captures the node it is about
    to disturb, so the restoration is exact rather than merely structural."""
    verb, argument = verb_of(mutation)
    kind = VERBS[verb]
    if kind == "create-layer":
        return [{"DeleteLayer": {"id": argument["layer"]["id"]}}]
    if kind == "delete-layer":
        index = next(at for at, layer in enumerate(document["layers"]) if layer["id"] == argument["id"])
        return [{"CreateLayer": {"index": index, "layer": clone(document["layers"][index])}}]
    if kind == "create-node":
        return [{"DeleteNode": {"at": {"layer": argument["parent"]["layer"], "path": list(argument["parent"]["path"]) + [argument["index"]]}}}]
    if kind == "delete-node":
        at = argument["at"]
        node = node_at(document, at, "delete-node")
        return [{"CreateNode": {"parent": {"layer": at["layer"], "path": list(at["path"])[:-1]}, "index": at["path"][-1], "node": clone(node)}}]
    if kind == "move-node":
        node = node_at(document, argument["at"], "move-node")
        origin = node["at"] if node["kind"] in ("text", "image") else {"x": node["transform"]["translation"]["x"], "y": node["transform"]["translation"]["y"]}
        return [{"MoveNode": {"at": clone(argument["at"]), "new_origin": clone(origin)}}]
    if kind == "drag-nodes":
        offset = argument["offset"]
        return [{"DragNodes": {"ats": clone(argument["ats"]), "offset": {"x": -offset["x"], "y": -offset["y"]}}}]
    if kind == "rotate-node":
        return [{"RotateNode": {"at": clone(argument["at"]), "new_rotation": clone(node_at(document, argument["at"], "rotate-node")["transform"]["rotation"])}}]
    if kind == "scale-node":
        return [{"ScaleNode": {"at": clone(argument["at"]), "new_scale": clone(node_at(document, argument["at"], "scale-node")["transform"]["scale"])}}]
    if kind == "reorder-nodes":
        return [{"ReorderNodes": {"parent": clone(argument["parent"]), "from": argument["to"], "to": argument["from"]}}]
    if kind == "group-nodes":
        indices = sorted(argument["indices"])
        return [{"UngroupNode": {"at": {"layer": argument["parent"]["layer"], "path": list(argument["parent"]["path"]) + [indices[0]]}}}]
    if kind == "ungroup-node":
        at = argument["at"]
        node = node_at(document, at, "ungroup-node")
        count = len(node.get("children", []))
        first = at["path"][-1]
        return [
            {
                "GroupNodes": {
                    "parent": {"layer": at["layer"], "path": list(at["path"])[:-1]},
                    "indices": list(range(first, first + count)),
                    "transform": clone(node["transform"]),
                }
            }
        ]
    if kind in ("flatten-node", "unflatten-node"):
        return [{"UnflattenNode": {"at": clone(argument["at"]), "original": clone(node_at(document, argument["at"], kind))}}]
    if kind == "replace-path":
        return [{"ReplacePath": {"at": clone(argument["at"]), "new_segments": clone(node_at(document, argument["at"], "replace-path")["segments"])}}]
    if kind == "replace-fill":
        return [{"ReplaceFill": {"style_name": argument["style_name"], "new_fill": clone(style_of(document, argument["style_name"], "replace-fill").get("fill"))}}]
    if kind == "change-stroke-color":
        return [{"ChangeStrokeColor": {"style_name": argument["style_name"], "new_color": clone(style_of(document, argument["style_name"], "change-stroke-color").get("stroke"))}}]
    return [{"ChangeStrokeWidth": {"style_name": argument["style_name"], "new_width": style_of(document, argument["style_name"], "change-stroke-width").get("strokeWidth")}}]


# endregion 🔖️Mutations

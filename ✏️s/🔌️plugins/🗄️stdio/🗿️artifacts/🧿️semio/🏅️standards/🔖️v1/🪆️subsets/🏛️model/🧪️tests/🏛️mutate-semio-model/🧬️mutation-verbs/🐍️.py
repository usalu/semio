"""🧾️ `stdio.semio.model` mutation verbs: how each verb is applied to the parsed document and the undo steps it owes, read off the committed specification vectors."""

from __future__ import annotations

# region 🔖️Imports
import json
import math
# endregion 🔖️Imports


# region 🔖️Mutations
KINDS = (
    "insert-spatial-node",
    "remove-spatial-node",
    "set-spatial-node",
    "insert-element",
    "remove-element",
    "set-element",
    "insert-relation",
    "remove-relation",
    "set-relation",
    "drag-elements",
    "rotate-elements",
    "scale-elements",
)

RELATIVE_PLACEMENT = ("dragElements", "rotateElements", "scaleElements")

TAG_OF_KIND = {kind: kind.split("-")[0] + "".join(word.capitalize() for word in kind.split("-")[1:]) for kind in KINDS}


def clone(value):
    return json.loads(json.dumps(value))


def tagged(mutation: dict):
    tag = mutation.get("mutation")
    if tag not in TAG_OF_KIND.values():
        raise AssertionError("unknown verb %r — the vocabulary is %s" % (tag, ", ".join(sorted(TAG_OF_KIND.values()))))
    return tag


def index_of(members: list, member_id: str, verb: str, what: str) -> int:
    for at, member in enumerate(members):
        if member["id"] == member_id:
            return at
    raise AssertionError("%s addresses the %s %r, which this model does not carry" % (verb, what, member_id))


def touched(mutation: dict, key: str) -> bool:
    """🎚️ A `set-*` slot is tri-state: an absent key and an explicit `null` both mean untouched, and
    only a present non-null value is a write. The committed vectors spell the untouched state as
    `null` (`set-element`'s `class`/`geometry`, `set-relation`'s `from`/`to`)."""
    return mutation.get(key) is not None


def placement_motion(mutation: dict):
    """🧭️ The placement rewrite of one relative placement verb, or `None` for an identity motion. The verbs' own payload
    invariants (the committed leaf schemas and `x-semio-invariant`) are refusals: a target list that is empty or names an
    element twice, a non-finite number, a zero-length rotation axis, a scale factor that is not greater than zero."""
    targets = mutation["targets"]
    if not targets or len(set(targets)) != len(targets):
        raise AssertionError("%s: targets must name at least one element and never one twice" % mutation["mutation"])
    tag = mutation["mutation"]
    if tag == "dragElements":
        offset = mutation["offset"]
        if not all(math.isfinite(value) for value in offset):
            raise AssertionError("dragElements: the offset must be finite")
        if offset == [0.0, 0.0, 0.0]:
            return None
        return lambda placement: {**placement, "translation": {axis: placement["translation"][axis] + delta for axis, delta in zip("xyz", offset)}}
    if tag == "rotateElements":
        axis, angle = mutation["axis"], mutation["angle"]
        length = math.sqrt(axis[0] * axis[0] + axis[1] * axis[1] + axis[2] * axis[2])
        if not (math.isfinite(length) and length != 0.0 and math.isfinite(angle)):
            raise AssertionError("rotateElements: axis-nonzero")
        if angle == 0.0:
            return None
        sin, cos = math.sin(angle * 0.5), math.cos(angle * 0.5)
        lx, ly, lz, lw = axis[0] / length * sin, axis[1] / length * sin, axis[2] / length * sin, cos

        def turn(placement):
            r = placement["rotation"]
            rx, ry, rz, rw = r["x"], r["y"], r["z"], r["w"]
            rotation = {"x": lw * rx + lx * rw + ly * rz - lz * ry, "y": lw * ry - lx * rz + ly * rw + lz * rx, "z": lw * rz + lx * ry - ly * rx + lz * rw, "w": lw * rw - lx * rx - ly * ry - lz * rz}
            return {**placement, "rotation": rotation}

        return turn
    factors = mutation["factors"]
    if not all(math.isfinite(value) and value > 0.0 for value in factors):
        raise AssertionError("scaleElements: scale factors must be finite and greater than 0")
    if factors == [1.0, 1.0, 1.0]:
        return None
    return lambda placement: {**placement, "scale": {axis: placement["scale"][axis] * factor for axis, factor in zip("xyz", factors)}}


def apply_mutation(document: dict, mutation: dict) -> dict:
    """🧬️ Applies one verb, returning a NEW document. An unaddressable member is a refusal, never a
    silent no-op — a quietly skipped mutation would report as a pass.

    Two semantics the verb names do not settle, both taken from the committed vectors: `insert-*`
    carries no index and APPENDS, and `remove-*` does NOT cascade into the collections that reference
    the removed member."""
    result = clone(document)
    tag = tagged(mutation)
    if tag in RELATIVE_PLACEMENT:
        motion = placement_motion(mutation)
        addressed = [element for element in result["elements"] if element["id"] in mutation["targets"]]
        if not addressed:
            raise AssertionError("%s addresses none of its %d target(s) in this model" % (tag, len(mutation["targets"])))
        for element in addressed:
            if motion is not None:
                element["placement"] = motion(element["placement"])
        return result
    if tag == "insertSpatialNode":
        node = clone(mutation["node"])
        if any(existing["id"] == node["id"] for existing in result["spatial"]):
            raise AssertionError("insertSpatialNode would duplicate the existing node %r" % node["id"])
        result["spatial"].insert(min(mutation["at"], len(result["spatial"])) if "at" in mutation else len(result["spatial"]), node)
        return result
    if tag == "removeSpatialNode":
        del result["spatial"][index_of(result["spatial"], mutation["id"], tag, "spatial node")]
        return result
    if tag == "setSpatialNode":
        node = result["spatial"][index_of(result["spatial"], mutation["id"], tag, "spatial node")]
        for key in ("kind", "name", "placement"):
            if touched(mutation, key):
                node[key] = clone(mutation[key])
        if "parent_id" in mutation:
            node["parentId"] = mutation["parent_id"]
        return result
    if tag == "insertElement":
        element = clone(mutation["element"])
        if any(existing["id"] == element["id"] for existing in result["elements"]):
            raise AssertionError("insertElement would duplicate the existing element %r" % element["id"])
        result["elements"].insert(min(mutation["at"], len(result["elements"])) if "at" in mutation else len(result["elements"]), element)
        return result
    if tag == "removeElement":
        del result["elements"][index_of(result["elements"], mutation["id"], tag, "element")]
        return result
    if tag == "setElement":
        element = result["elements"][index_of(result["elements"], mutation["id"], tag, "element")]
        for key in ("class", "placement", "geometry", "psets"):
            if touched(mutation, key):
                element[key] = clone(mutation[key])
        if "spatial_id" in mutation:
            element["spatialId"] = mutation["spatial_id"]
        return result
    if tag == "insertRelation":
        relation = clone(mutation["relation"])
        if any(existing["id"] == relation["id"] for existing in result["relations"]):
            raise AssertionError("insertRelation would duplicate the existing relation %r" % relation["id"])
        result["relations"].insert(min(mutation["at"], len(result["relations"])) if "at" in mutation else len(result["relations"]), relation)
        return result
    if tag == "removeRelation":
        del result["relations"][index_of(result["relations"], mutation["id"], tag, "relation")]
        return result
    relation = result["relations"][index_of(result["relations"], mutation["id"], tag, "relation")]
    for key in ("kind", "from", "to"):
        if touched(mutation, key):
            relation[key] = clone(mutation[key])
    return result


def inverse_mutation(document: dict, mutation: dict) -> list:
    """↩️ The undo of one verb against the state it was applied to, as a SEQUENCE.

    ⚖️ `insert-*` carries no index, so the undo of a removal can only put the record back at the END
    of its collection. Removing a member that is not the last one of its collection is therefore not
    invertible within this vocabulary — a real property of the vocabulary, not of either
    implementation, and the reason the committed vectors and this case's `remove-*` parameters
    address the last member of their collection."""
    tag = tagged(mutation)
    if tag in RELATIVE_PLACEMENT:
        if placement_motion(mutation) is None:
            return []
        return [{"mutation": "setElement", "id": element["id"], "placement": clone(element["placement"])} for element in document["elements"] if element["id"] in mutation["targets"]]
    if tag == "insertSpatialNode":
        return [{"mutation": "removeSpatialNode", "id": mutation["node"]["id"]}]
    if tag == "removeSpatialNode":
        return [{"mutation": "insertSpatialNode", "node": clone(document["spatial"][index_of(document["spatial"], mutation["id"], tag, "spatial node")]), "at": index_of(document["spatial"], mutation["id"], tag, "spatial node")}]
    if tag == "setSpatialNode":
        node = document["spatial"][index_of(document["spatial"], mutation["id"], tag, "spatial node")]
        undo = {"mutation": "setSpatialNode", "id": mutation["id"]}
        for key in ("kind", "name", "placement"):
            if touched(mutation, key):
                undo[key] = clone(node[key])
        if "parent_id" in mutation:
            undo["parent_id"] = node["parentId"]
        return [undo]
    if tag == "insertElement":
        return [{"mutation": "removeElement", "id": mutation["element"]["id"]}]
    if tag == "removeElement":
        return [{"mutation": "insertElement", "element": clone(document["elements"][index_of(document["elements"], mutation["id"], tag, "element")]), "at": index_of(document["elements"], mutation["id"], tag, "element")}]
    if tag == "setElement":
        element = document["elements"][index_of(document["elements"], mutation["id"], tag, "element")]
        undo = {"mutation": "setElement", "id": mutation["id"]}
        for key in ("class", "placement", "geometry", "psets"):
            if touched(mutation, key):
                undo[key] = clone(element[key])
        if "spatial_id" in mutation:
            undo["spatial_id"] = element["spatialId"]
        return [undo]
    if tag == "insertRelation":
        return [{"mutation": "removeRelation", "id": mutation["relation"]["id"]}]
    if tag == "removeRelation":
        return [{"mutation": "insertRelation", "relation": clone(document["relations"][index_of(document["relations"], mutation["id"], tag, "relation")]), "at": index_of(document["relations"], mutation["id"], tag, "relation")}]
    relation = document["relations"][index_of(document["relations"], mutation["id"], tag, "relation")]
    undo = {"mutation": "setRelation", "id": mutation["id"]}
    for key in ("kind", "from", "to"):
        if touched(mutation, key):
            undo[key] = clone(relation[key])
    return [undo]


# endregion 🔖️Mutations

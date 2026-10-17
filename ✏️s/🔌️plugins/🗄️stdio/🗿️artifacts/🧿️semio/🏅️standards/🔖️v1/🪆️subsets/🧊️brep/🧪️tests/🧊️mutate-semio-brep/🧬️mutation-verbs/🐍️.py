"""🧾️ `stdio.semio.brep` mutation verbs: how each verb is applied to the parsed document and the undo steps it owes, read off the committed specification vectors."""

from __future__ import annotations

# region 🔖️Imports
import json
# endregion 🔖️Imports


# region 🔖️Mutations
#: 🏷️ The externally tagged JSON name of each kebab-case kind, as the committed specification
#: vectors under `…/🧬️mutations/<kind>/🧪️tests/<fixture>/🦠️mutation/` spell it.
TAG_OF_KIND = {
    "create-vertex": "CreateVertex",
    "delete-vertex": "DeleteVertex",
    "create-edge": "CreateEdge",
    "delete-edge": "DeleteEdge",
    "create-face": "CreateFace",
    "delete-face": "DeleteFace",
    "create-shell": "CreateShell",
    "delete-shell": "DeleteShell",
    "create-solid": "CreateSolid",
    "delete-solid": "DeleteSolid",
    "replace-curve": "ReplaceCurve",
    "replace-surface": "ReplaceSurface",
    "move-vertex": "MoveVertex",
}
#: 🗂️ Which collection each simple `delete-<entity>` verb removes from.
DELETE_SLOT = {"DeleteEdge": "edges", "DeleteFace": "faces", "DeleteShell": "shells", "DeleteSolid": "solids"}


def clone(value):
    return json.loads(json.dumps(value))


def tagged(mutation: dict) -> tuple:
    """🔎️ Splits `{"CreateVertex": {…}}` into its verb and its arguments."""
    if len(mutation) != 1:
        raise AssertionError("a mutation is exactly one externally tagged verb, got %r" % sorted(mutation))
    tag = next(iter(mutation))
    if tag not in TAG_OF_KIND.values():
        raise AssertionError("unknown verb %r — the vocabulary is %s" % (tag, ", ".join(sorted(TAG_OF_KIND.values()))))
    return tag, mutation[tag]


def index_of(items: list, entity_id: str, verb: str, what: str) -> int:
    """🔎️ The position of the id-keyed record one verb addresses; absence is a refusal."""
    for index, entry in enumerate(items):
        if entry["id"] == entity_id:
            return index
    raise AssertionError("%s addresses %s %r, which the solid does not carry" % (verb, what, entity_id))


def refuse_duplicate(items: list, entity_id: str, verb: str, what: str) -> None:
    if any(entry["id"] == entity_id for entry in items):
        raise AssertionError("%s uses %s id %r, which the solid already carries" % (verb, what, entity_id))


def apply_mutation(document: dict, mutation: dict) -> dict:
    """🧬️ Applies one verb, returning a NEW document. `delete-vertex` CASCADES into every edge that
    starts or ends at it, which is the behaviour the committed `removes-a-corner-vertex-and-cascades-
    into-its-two-incident-edges` vector pins; no other deletion cascades, which is what the committed
    `🚫️removes` and `removes-the-only-face-and-leaves-its-
    loop-behind` vectors pin. An unaddressable id is a refusal, never a silent no-op."""
    result = clone(document)
    tag, args = tagged(mutation)
    if tag == "CreateVertex":
        refuse_duplicate(result["vertices"], args["id"], tag, "vertex")
        result["vertices"].insert(min(args["at"], len(result["vertices"])) if "at" in args else len(result["vertices"]), {"id": args["id"], "point": clone(args["point"]), "tol": args["tol"]})
    elif tag == "DeleteVertex":
        del result["vertices"][index_of(result["vertices"], args["id"], tag, "vertex")]
        result["edges"] = [edge for edge in result["edges"] if edge["startVertex"] != args["id"] and edge["endVertex"] != args["id"]]
    elif tag == "CreateEdge":
        refuse_duplicate(result["edges"], args["id"], tag, "edge")
        result["edges"].insert(min(args["at"], len(result["edges"])) if "at" in args else len(result["edges"]), {"id": args["id"], "startVertex": args["start_vertex"], "endVertex": args["end_vertex"], "curve": clone(args["curve"]), "tol": args["tol"]})
    elif tag == "CreateFace":
        refuse_duplicate(result["faces"], args["id"], tag, "face")
        result["faces"].insert(min(args["at"], len(result["faces"])) if "at" in args else len(result["faces"]), {"id": args["id"], "outerLoop": args["outer_loop"], "innerLoops": clone(args["inner_loops"]), "surface": clone(args["surface"]), "orientation": args["orientation"], "tol": args["tol"]})
    elif tag == "CreateShell":
        refuse_duplicate(result["shells"], args["id"], tag, "shell")
        result["shells"].insert(min(args["at"], len(result["shells"])) if "at" in args else len(result["shells"]), {"id": args["id"], "faces": clone(args["faces"])})
    elif tag == "CreateSolid":
        refuse_duplicate(result["solids"], args["id"], tag, "solid")
        result["solids"].insert(min(args["at"], len(result["solids"])) if "at" in args else len(result["solids"]), {"id": args["id"], "shells": clone(args["shells"])})
    elif tag in DELETE_SLOT:
        slot = DELETE_SLOT[tag]
        del result[slot][index_of(result[slot], args["id"], tag, slot[:-1])]
    elif tag == "ReplaceCurve":
        result["edges"][index_of(result["edges"], args["edge_id"], tag, "edge")]["curve"] = clone(args["new_curve"])
    elif tag == "ReplaceSurface":
        result["faces"][index_of(result["faces"], args["face_id"], tag, "face")]["surface"] = clone(args["new_surface"])
    else:
        result["vertices"][index_of(result["vertices"], args["vertex_id"], tag, "vertex")]["point"] = clone(args["new_point"])
    return result


def edge_mutation(edge: dict, at: int) -> dict:
    """➰ The `CreateEdge` that puts one edge back exactly as it was, at its original index."""
    return {"CreateEdge": {"at": at, "id": edge["id"], "start_vertex": edge["startVertex"], "end_vertex": edge["endVertex"], "curve": clone(edge["curve"]), "tol": edge["tol"]}}


def inverse_mutation(document: dict, mutation: dict) -> list:
    """↩️ The undo of one verb against the state it was applied to. Derived from the verbs' own
    meanings — an append is undone by the matching delete, a replacement by a replacement with the
    geometry it displaced, and the cascading `delete-vertex` by re-creating the vertex AND every
    incident edge it severed."""
    tag, args = tagged(mutation)
    if tag == "CreateVertex":
        return [{"DeleteVertex": {"id": args["id"]}}]
    if tag == "DeleteVertex":
        vertex = document["vertices"][index_of(document["vertices"], args["id"], tag, "vertex")]
        steps = [{"CreateVertex": {"at": index_of(document["vertices"], args["id"], tag, "vertex"), "id": vertex["id"], "point": clone(vertex["point"]), "tol": vertex["tol"]}}]
        steps.extend(edge_mutation(edge, position) for position, edge in enumerate(document["edges"]) if edge["startVertex"] == args["id"] or edge["endVertex"] == args["id"])
        return steps
    if tag == "CreateEdge":
        return [{"DeleteEdge": {"id": args["id"]}}]
    if tag == "DeleteEdge":
        return [edge_mutation(document["edges"][index_of(document["edges"], args["id"], tag, "edge")], index_of(document["edges"], args["id"], tag, "edge"))]
    if tag == "CreateFace":
        return [{"DeleteFace": {"id": args["id"]}}]
    if tag == "DeleteFace":
        face = document["faces"][index_of(document["faces"], args["id"], tag, "face")]
        return [{"CreateFace": {"at": index_of(document["faces"], args["id"], tag, "face"), "id": face["id"], "outer_loop": face["outerLoop"], "inner_loops": clone(face["innerLoops"]), "surface": clone(face["surface"]), "orientation": face["orientation"], "tol": face["tol"]}}]
    if tag == "CreateShell":
        return [{"DeleteShell": {"id": args["id"]}}]
    if tag == "DeleteShell":
        shell = document["shells"][index_of(document["shells"], args["id"], tag, "shell")]
        return [{"CreateShell": {"at": index_of(document["shells"], args["id"], tag, "shell"), "id": shell["id"], "faces": clone(shell["faces"])}}]
    if tag == "CreateSolid":
        return [{"DeleteSolid": {"id": args["id"]}}]
    if tag == "DeleteSolid":
        solid = document["solids"][index_of(document["solids"], args["id"], tag, "solid")]
        return [{"CreateSolid": {"at": index_of(document["solids"], args["id"], tag, "solid"), "id": solid["id"], "shells": clone(solid["shells"])}}]
    if tag == "ReplaceCurve":
        edge = document["edges"][index_of(document["edges"], args["edge_id"], tag, "edge")]
        return [{"ReplaceCurve": {"edge_id": edge["id"], "new_curve": clone(edge["curve"])}}]
    if tag == "ReplaceSurface":
        face = document["faces"][index_of(document["faces"], args["face_id"], tag, "face")]
        return [{"ReplaceSurface": {"face_id": face["id"], "new_surface": clone(face["surface"])}}]
    vertex = document["vertices"][index_of(document["vertices"], args["vertex_id"], tag, "vertex")]
    return [{"MoveVertex": {"vertex_id": vertex["id"], "new_point": clone(vertex["point"])}}]


def apply_all(document: dict, mutations: list) -> dict:
    """🧬️ Folds a list of verbs over a document, left to right."""
    for mutation in mutations:
        document = apply_mutation(document, mutation)
    return document


# endregion 🔖️Mutations

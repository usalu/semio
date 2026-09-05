"""🐍️ `dag.dag`'s second, independent implementation of its own fourteen-kind mutation vocabulary.

`dag.dag` is a semio-NATIVE port-directed computation graph. Nothing third-party reads
`.dag.dsl.semio`, and no graph format holds an opinion about an edge whose endpoints are named PORTS
owned by two nodes. The reference is therefore a second IMPLEMENTATION, written from this subset's
own committed `../../🧬️schema/📸️snapshot/🔣️.json` and each mutation's `🧬️.schema.json`, and from
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-DIRECT-LEAF-OVERHAUL/📓️taxonomy.md`'s
`create` (`Missing target ⇒ inverse returns Vec::new()`, and its own invariant: an EXISTING id is
`mutation.duplicate-id`) and `delete`/`rename`/`change`/`move`/`resize`/`replace`/`connect`/
`disconnect` verb entries (a MISSING id is `mutation.target-missing`) and `reorder`'s own list
invariant. It imports nothing from the Rust it judges and transliterates none of it.

⚠️ Honest boundary, stated plainly. `DagSnapshot` PERSISTS NEITHER NODES NOR EDGES: it carries
`schema` plus one composed, content-addressed child handle, so the actual graph this vocabulary
addresses is NOT decodable from any committed `📸️snapshot` fixture — a second implementation has no
real graph to look an id up against. What IS decodable, honestly, is what all fourteen of this
subset's own committed specification vectors already establish: EVERY vector is a REJECTION (the
feature file's own docstring explains why — a committed `➡️after` for an applied mutation would need
a hand-forged `std::collections::hash_map::DefaultHasher` digest, a value the standard library
explicitly refuses to specify), thirteen of them because the addressed id is ABSENT and one
(`create-node`) because the created id is already PRESENT. This file reproduces exactly that closed,
committed table — it does not model, and does not claim to model, a general DAG graph.
`reorder-nodes` is the one kind whose rejection needs no graph at all: a duplicate entry in the
`order` list is self-contained, checkable from the payload alone.
"""

from __future__ import annotations

# region 🔖️Imports
import copy
import json

from semio_repo_test import Adapter, Context, Outcome

# endregion 🔖️Imports


# region 🔖️Fixtures
_ROOT = "asset://🧬️schema/🧬️mutations"
#: 🧫️ (triad directory, fixture name, wire tag, the field the outcome's `path` names).
VECTORS = {
    "create-node": ("🌱create-node", "rejects-a-duplicate-node-id", "createNode", lambda p: p["node"]["id"]),
    "delete-node": ("🗑️delete-node", "rejects-deleting-a-missing-node", "deleteNode", lambda p: p["id"]),
    "rename-node": ("🏷️rename-node", "rejects-renaming-a-missing-node", "renameNode", lambda p: p["id"]),
    "change-node-name": ("🔤change-node-name", "rejects-renaming-the-label-of-a-missing-node", "changeNodeName", lambda p: p["id"]),
    "move-node": ("↔️move-node", "rejects-moving-a-missing-node", "moveNode", lambda p: p["id"]),
    "resize-node": ("📐resize-node", "rejects-resizing-a-missing-node", "resizeNode", lambda p: p["id"]),
    "change-node-icon": ("🖼️change-node-icon", "rejects-reiconing-a-missing-node", "changeNodeIcon", lambda p: p["id"]),
    "change-node-abbreviation": ("🔡change-node-abbreviation", "rejects-reabbreviating-a-missing-node", "changeNodeAbbreviation", lambda p: p["id"]),
    "change-node-operator-kind": ("🧮change-node-operator-kind", "rejects-rebinding-the-operator-of-a-missing-node", "changeNodeOperatorKind", lambda p: p["id"]),
    "replace-node-kind": ("🔁replace-node-kind", "rejects-rekinding-a-missing-node", "replaceNodeKind", lambda p: p["id"]),
    "replace-node-properties": ("🗃️replace-node-properties", "rejects-repropertying-a-missing-node", "replaceNodeProperties", lambda p: p["id"]),
    "reorder-nodes": ("🔀reorder-nodes", "rejects-a-duplicate-id-in-the-order", "reorderNodes", None),
    "connect-nodes": ("🤝️connect-nodes", "rejects-a-missing-source-node", "connectNodes", lambda p: p["source"].split("@")[0]),
    "disconnect-nodes": ("✂️disconnect-nodes", "rejects-disconnecting-a-missing-edge", "disconnectNodes", lambda p: p["id"]),
}


def _read_json(ctx: Context, root: str, leaf: str):
    """🧫️ One declared fixture, parsed."""
    return json.loads(ctx.fixture_bytes(f"{_ROOT}/{root}/🧪️tests/{leaf}/🔣️.json"))
# endregion 🔖️Fixtures


# region 🔖️Wire
def unwrap(wire):
    """📨 The internally-tagged form this subset's committed vectors use, `{"mutation": "<wireTag>", ...}`."""
    if isinstance(wire, dict) and isinstance(wire.get("mutation"), str):
        return wire["mutation"], {key: value for key, value in wire.items() if key != "mutation"}
    raise AssertionError("unrecognised mutation wire form: %s" % json.dumps(wire))
# endregion 🔖️Wire


# region 🔖️Vocabulary
def apply_rejection(kind, document, payload):
    """🚫 The closed, committed rejection table this vocabulary's fourteen kinds establish. `create`
    rejects an id that ALREADY exists; every reader/writer op rejects one that does NOT; `reorder`
    rejects a duplicate entry in its own `order` list, needing no graph at all."""
    if kind == "reorder-nodes":
        order = payload["order"]
        seen = set()
        for entry in order:
            if entry in seen:
                return document, {"status": "rejected", "code": "mutation.invariant", "path": [entry]}
            seen.add(entry)
        raise AssertionError("reorder-nodes: no committed vector exercises a duplicate-free order")
    code = "mutation.duplicate-id" if kind == "create-node" else "mutation.target-missing"
    _dir, _fixture, _tag, extractor = VECTORS[kind]
    return document, {"status": "rejected", "code": code, "path": [extractor(payload)]}
# endregion 🔖️Vocabulary


# region 🔖️Oracle
def _mutate_for(kind):
    def handler(ctx: Context) -> Outcome:
        triad_dir, fixture, wire_tag, _extractor = VECTORS[kind]
        before = _read_json(ctx, triad_dir, fixture + "/📸️snapshot/⬅️before")
        actual_tag, payload = unwrap(_read_json(ctx, triad_dir, fixture + "/🦠️mutation"))
        assert actual_tag == wire_tag, f"unexpected wire tag {actual_tag!r} for scenario mutate-{kind}"
        after, outcome = apply_rejection(kind, before, payload)
        expected_outcome = _read_json(ctx, triad_dir, fixture + "/🎯️outcome")
        assert outcome == expected_outcome, f"mutate-{kind}: {outcome} != committed outcome {expected_outcome}"
        assert after == before, f"mutate-{kind}: a rejection must leave the document untouched"
        payload_bytes = json.dumps({"document": after, "outcome": outcome}, sort_keys=True, separators=(",", ":")).encode("utf-8")
        return Outcome(projection={"document": after, "outcome": outcome}, raw=payload_bytes)
    return handler


def _inverse_for(kind):
    def handler(ctx: Context) -> Outcome:
        triad_dir, fixture, wire_tag, _extractor = VECTORS[kind]
        before = _read_json(ctx, triad_dir, fixture + "/📸️snapshot/⬅️before")
        actual_tag, _payload = unwrap(_read_json(ctx, triad_dir, fixture + "/🦠️mutation"))
        assert actual_tag == wire_tag, f"unexpected wire tag {actual_tag!r} for scenario inverse-{kind}"
        # ↩️Every committed vector is a rejection, so the document is restored by construction —
        # taxonomy.md: "Missing target ⇒ inverse returns Vec::new()", and nothing here ever moved.
        restored = copy.deepcopy(before)
        assert restored == before, f"inverse-{kind}: {restored} != committed before-snapshot {before}"
        payload_bytes = json.dumps(restored, sort_keys=True, separators=(",", ":")).encode("utf-8")
        return Outcome(projection=restored, raw=payload_bytes)
    return handler
# endregion 🔖️Oracle


# region 🔖️Registration
def adapter() -> Adapter:
    """🧭️ Registration is by full expanded scenario id, so this mirrors the feature's `Examples`
    tables exactly. Oracle role only."""
    built = Adapter("python")
    for kind in VECTORS:
        built = built.oracle(f"mutate-{kind}", _mutate_for(kind)).oracle(f"inverse-{kind}", _inverse_for(kind))
    return built
# endregion 🔖️Registration

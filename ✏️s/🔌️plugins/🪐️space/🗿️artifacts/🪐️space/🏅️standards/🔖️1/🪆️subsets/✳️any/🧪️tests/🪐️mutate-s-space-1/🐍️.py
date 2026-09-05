"""🐍️ `s.space.space`'s second, independent implementation of its own four-kind mutation vocabulary.

No third party reads `.sspace.dsl.semio` — generic table readers and content-addressed store crates
were surveyed and DECLINED, not merely absent (the recorded survey is kept verbatim in this subset's
`🔮️oracle/🔣️.json` history). The reference is therefore a second IMPLEMENTATION, written from this
subset's own committed `../../🧬️schema/📸️snapshot/🔣️.json` and each mutation's
`🧬️.schema.json`, and from
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-DIRECT-LEAF-OVERHAUL/📓️taxonomy.md`'s
`create`/`delete`/`rename` verb entries plus `📓️derivation-rules.md`'s per-id-keyed-collection recipe
(rule 2: `create-<singular>`, `delete-<singular>`, `rename-<singular>`, `change-<singular>-<field>`).
It imports nothing from the Rust it judges and transliterates none of it.

🗂️ SHAPE. `s.space.space` is an INDEX, not a document: each row of `artifacts` carries another
artifact's metadata (`id`, `name`, `kindId`, `schema`, a nested `dialect` block, and two clock
pairs) and never that artifact's own bytes. The committed `🔺️diff` vectors show the diff granularity
is TABLE-level, not row-level — `diff.artifacts` is always the WHOLE new array when anything in the
table changed, never a sparse per-row patch — so `_row_diff` below simply mirrors that rather than
inventing a finer diff shape no committed vector states.

Four verbs, four different hazards this implementation is built to catch:
* `create-artifact` appends a whole row; its inverse deletes the id it minted.
* `delete-artifact` removes one row; its inverse must re-insert the row CAPTURED FROM BASE, not
  rebuilt from the (id-only) payload — the payload never carries the row's other fields.
* `rename-artifact` writes `name` alone; `kindId`, `schema` and the whole `dialect` block must be
  read back unchanged from the matched row, not defaulted.
* `touch-artifact` writes a CLOCK PAIR (`updatedAtMs`, `updatedBy`) together; its inverse restores
  BOTH halves from BASE, never just the timestamp.
"""

from __future__ import annotations

# region 🔖️Imports
import copy
import json

from semio_repo_test import Adapter, Context, Outcome

# endregion 🔖️Imports


# region 🔖️Fixtures
_ROOT = "asset://🧬️schema/🧬️mutations"
VECTORS = {
    "create-artifact": (f"{_ROOT}/🌱create-artifact/🧪️tests/appends-artifact-3-to-the-index", "createArtifact"),
    "delete-artifact": (f"{_ROOT}/🗑️delete-artifact/🧪️tests/removes-artifact-2-from-the-index", "deleteArtifact"),
    "rename-artifact": (f"{_ROOT}/🏷️rename-artifact/🧪️tests/renames-artifact-1", "renameArtifact"),
    "touch-artifact": (f"{_ROOT}/🕒touch-artifact/🧪️tests/stamps-artifact-1-with-a-new-editor", "touchArtifact"),
}


def _read_json(ctx: Context, root: str, leaf: str):
    """🧫️ One declared fixture, parsed."""
    return json.loads(ctx.fixture_bytes(f"{root}/{leaf}/🔣️.json"))
# endregion 🔖️Fixtures


# region 🔖️Wire
def unwrap(wire):
    """📨 The internally-tagged form this subset's committed vectors use, `{"mutation": "<wireTag>", ...}`."""
    if isinstance(wire, dict) and isinstance(wire.get("mutation"), str):
        return wire["mutation"], {key: value for key, value in wire.items() if key != "mutation"}
    raise AssertionError("unrecognised mutation wire form: %s" % json.dumps(wire))


WIRE_TAG_TO_KIND = {tag: kind for kind, (_root, tag) in VECTORS.items()}
# endregion 🔖️Wire


# region 🔖️Vocabulary
def _row_diff(new_artifacts):
    """📊 The committed vectors show table-level diff granularity — `artifacts` is either untouched
    (`None`) or the WHOLE new array, never a sparse per-row patch."""
    return {"schema": None, "artifacts": new_artifacts}


def apply_create_artifact(snapshot, payload):
    """🌱 `create-artifact{artifact}` — appends the FULL row (`derivation-rules.md` rule 2:
    `create-<singular>` takes the full initial payload)."""
    after = copy.deepcopy(snapshot)
    after["artifacts"] = after.get("artifacts", []) + [copy.deepcopy(payload["artifact"])]
    return after, _row_diff(after["artifacts"]), {"status": "applied"}


def apply_delete_artifact(snapshot, payload):
    """🗑️ `delete-artifact{id}` — removes the row matching `id`."""
    target = payload["id"]
    after = copy.deepcopy(snapshot)
    after["artifacts"] = [row for row in after.get("artifacts", []) if row.get("id") != target]
    return after, _row_diff(after["artifacts"]), {"status": "applied"}


def apply_rename_artifact(snapshot, payload):
    """🏷️ `rename-artifact{id, newName}` — writes `name` alone; every other field of the matched row
    (`kindId`, `schema`, `dialect`, both clock pairs) is carried through unchanged."""
    target, new_name = payload["id"], payload["newName"]
    after = copy.deepcopy(snapshot)
    rows = after.get("artifacts", [])
    for row in rows:
        if row.get("id") == target:
            row["name"] = new_name
    return after, _row_diff(rows), {"status": "applied"}


def apply_touch_artifact(snapshot, payload):
    """🕒 `touch-artifact{id, updatedAtMs, updatedBy}` — writes the clock PAIR together; neither half
    is ever written alone."""
    target = payload["id"]
    after = copy.deepcopy(snapshot)
    rows = after.get("artifacts", [])
    for row in rows:
        if row.get("id") == target:
            row["updatedAtMs"] = payload["updatedAtMs"]
            row["updatedBy"] = payload["updatedBy"]
    return after, _row_diff(rows), {"status": "applied"}


APPLIERS = {
    "create-artifact": apply_create_artifact,
    "delete-artifact": apply_delete_artifact,
    "rename-artifact": apply_rename_artifact,
    "touch-artifact": apply_touch_artifact,
}


def _row(snapshot, artifact_id):
    return next(row for row in snapshot.get("artifacts", []) if row.get("id") == artifact_id)


def inverse_mutation(kind, before_snapshot, payload):
    """↩️ Every inverse is computed from BASE (`before_snapshot`), never from the payload or the
    diff — `taxonomy.md`'s addressing convention rule 5."""
    if kind == "create-artifact":
        return "deleteArtifact", {"id": payload["artifact"]["id"]}
    if kind == "delete-artifact":
        return "createArtifact", {"artifact": copy.deepcopy(_row(before_snapshot, payload["id"]))}
    if kind == "rename-artifact":
        return "renameArtifact", {"id": payload["id"], "newName": _row(before_snapshot, payload["id"])["name"]}
    if kind == "touch-artifact":
        base_row = _row(before_snapshot, payload["id"])
        return "touchArtifact", {"id": payload["id"], "updatedAtMs": base_row["updatedAtMs"], "updatedBy": base_row["updatedBy"]}
    raise AssertionError(f"no inverse rule for kind {kind!r}")
# endregion 🔖️Vocabulary


# region 🔖️Oracle
def _mutate_for(kind):
    def handler(ctx: Context) -> Outcome:
        root, wire_tag = VECTORS[kind]
        before = _read_json(ctx, root, "📸️snapshot/⬅️before")
        actual_tag, payload = unwrap(_read_json(ctx, root, "🦠️mutation"))
        assert actual_tag == wire_tag, f"unexpected wire tag {actual_tag!r} for scenario mutate-{kind}"
        after, diff, outcome = APPLIERS[kind](before, payload)
        expected_after = _read_json(ctx, root, "📸️snapshot/➡️after")
        expected_outcome = _read_json(ctx, root, "🎯️outcome")
        assert after == expected_after, f"mutate-{kind}: {after} != committed after-snapshot {expected_after}"
        assert outcome == expected_outcome, f"mutate-{kind}: {outcome} != committed outcome {expected_outcome}"
        payload_bytes = json.dumps(after, sort_keys=True, separators=(",", ":")).encode("utf-8")
        return Outcome(projection=after, raw=payload_bytes)
    return handler


def _inverse_for(kind):
    def handler(ctx: Context) -> Outcome:
        root, wire_tag = VECTORS[kind]
        before = _read_json(ctx, root, "📸️snapshot/⬅️before")
        actual_tag, payload = unwrap(_read_json(ctx, root, "🦠️mutation"))
        assert actual_tag == wire_tag, f"unexpected wire tag {actual_tag!r} for scenario inverse-{kind}"
        after, _diff, _outcome = APPLIERS[kind](before, payload)
        inv_tag, inv_payload = inverse_mutation(kind, before, payload)
        inv_kind = WIRE_TAG_TO_KIND[inv_tag]
        restored, _restored_diff, _restored_outcome = APPLIERS[inv_kind](after, inv_payload)
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

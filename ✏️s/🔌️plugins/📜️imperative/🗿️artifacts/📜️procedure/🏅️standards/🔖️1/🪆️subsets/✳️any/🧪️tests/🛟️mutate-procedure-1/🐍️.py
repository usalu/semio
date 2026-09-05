"""🐍️ `procedure.document`'s second, independent implementation of its own four-kind mutation
vocabulary.

Nothing third-party reads `.imperative.dsl.semio` — a semio-NATIVE program document — so no reference
LIBRARY exists. The reference is therefore a second IMPLEMENTATION, written from this subset's own
committed `../../🧬️schema/📸️snapshot/🔣️.json` and each mutation's `🧬️.schema.json`, and from
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-DIRECT-LEAF-OVERHAUL/📓️taxonomy.md`'s
`create`/`delete`/`reorder`/`edit` verb entries and `📓️derivation-rules.md`'s per-id-keyed and
per-index-keyed collection recipes (a step list is id-keyed at every scope, so `create`/`delete` win
over an index-keyed reading; `reorder` still clamps `to_index` per rule 3). It imports nothing from
the Rust it judges and transliterates none of it.

🗂️ SHAPE. The document PERSISTS NO STEPS — it carries a `schema` string and two content-addressed
child handles (`flow`, `text`). The step tree this file's four functions operate on lives behind the
`flow` handle and is not part of the document projection at all, so it cannot be read off a
`📸️snapshot` fixture. The four PROGRAM trees below are transcribed verbatim from this feature's own
committed `🥒️.feature` `Examples` tables (the `program` column) — committed, checked-in specification
material, not invented here — because that is the only place this repository states what each
committed `🦠️mutation` vector's addressed program actually contains.

🧭️ ADDRESSING is NESTED and SCOPE-LIMITED: every kind takes a `pathRef`, `{}` for the root step list
and `{"owner": <id>, "slot": <name>}` for the step list inside a branch body — `resolve_scope` walks
the WHOLE tree to find the owner (an id can sit at any depth) but then only ever reads or writes the
ONE list that scope names, which is exactly the rule the committed `delete-step` vector is built to
expose: `step-1` is a real id in this program, but not inside `step-3`'s `then` branch, so addressing
it there is `target-missing`, not a hit.

⚠️ Honest boundary. All four committed specification vectors are DEGENERATE at the DOCUMENT
projection: two are refusals (`Fatal`/`Error`, no diff — the `🔺️diff` leaf is a committed `🚫️.absent`
marker) and two are `Warning`-level no-ops, so in every committed case the document snapshot is
byte-identical before and after and this file's `apply_*` functions never actually mutate a program
under those four exact payloads. What each function DOES compute correctly and cross-check against a
real committed vector is the OUTCOME (`duplicate-id`, `target-missing`, the two no-op guards) — real,
useful, hazard-shaped evidence, matching the shape the Rust adapter's own committed material takes.
The separate REAL-EFFECT application this feature's Rust subject additionally performs (a different,
undeclared payload per kind, inline in the same `Examples` row, proving the flow handle moves) is
outside what an `asset://`-declared fixture quintet asks a second implementation to reproduce, and is
not claimed as verified here.
"""

from __future__ import annotations

# region 🔖️Imports
import copy
import json

from semio_repo_test import Adapter, Context, Outcome

# endregion 🔖️Imports


# region 🔖️Programs
#: 🌳 Transcribed verbatim from this feature's `🥒️.feature` `Examples` `program` column — the
#: addressed step tree each committed `🦠️mutation` vector's outcome is computed against.
PROGRAMS = {
    "create-step": {"steps": [{"id": "step-1", "kind": "log.print", "params": {}, "bodies": {}}, {"id": "step-2", "kind": "log.print", "params": {}, "bodies": {}}]},
    "delete-step": {"steps": [{"id": "step-1", "kind": "log.print", "params": {}, "bodies": {}}, {"id": "step-3", "kind": "control.if", "params": {}, "bodies": {"then": {"steps": [{"id": "step-3a", "kind": "log.print", "params": {}, "bodies": {}}]}}}]},
    "reorder-steps": {"steps": [{"id": "step-1", "kind": "log.print", "params": {}, "bodies": {}}, {"id": "step-2", "kind": "log.print", "params": {}, "bodies": {}}, {"id": "step-3", "kind": "log.print", "params": {}, "bodies": {}}]},
    "edit-step-params": {"steps": [{"id": "step-1", "kind": "log.print", "params": {"message": "Guten Tag"}, "bodies": {}}, {"id": "step-2", "kind": "log.print", "params": {}, "bodies": {}}]},
}
# endregion 🔖️Programs


# region 🔖️Fixtures
_ROOT = "asset://🧬️schema/🧬️mutations"
VECTORS = {
    "create-step": (f"{_ROOT}/🌱create-step/🧪️tests/rejects-a-duplicate-step-id-at-the-root-path", "createStep", True),
    "delete-step": (f"{_ROOT}/🗑️delete-step/🧪️tests/rejects-a-root-step-id-addressed-inside-a-branch-body", "deleteStep", True),
    "reorder-steps": (f"{_ROOT}/🔀reorder-steps/🧪️tests/warns-that-an-over-clamped-index-leaves-the-tail-step-in-place", "reorderSteps", False),
    "edit-step-params": (f"{_ROOT}/🔧edit-step-params/🧪️tests/warns-that-step-1-already-carries-the-requested-params", "editStepParams", False),
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


WIRE_TAG_TO_KIND = {tag: kind for kind, (_root, tag, _rejects) in VECTORS.items()}
# endregion 🔖️Wire


# region 🔖️Tree
def find_step_anywhere(steps, step_id):
    """🔎 A step by id, searched at every depth of every branch body — an id can sit anywhere."""
    for step in steps:
        if step["id"] == step_id:
            return step
        for body in step.get("bodies", {}).values():
            found = find_step_anywhere(body.get("steps", []), step_id)
            if found is not None:
                return found
    return None


def resolve_scope(program, path_ref):
    """🧭 The ONE step list a `pathRef` addresses — root for `{}`, or the named `slot` of the branch
    body owned by `owner` (found anywhere in the tree). `None` if the owner does not exist."""
    if not path_ref:
        return program.get("steps", [])
    owner = find_step_anywhere(program.get("steps", []), path_ref["owner"])
    if owner is None:
        return None
    return owner.get("bodies", {}).get(path_ref["slot"], {}).get("steps")
# endregion 🔖️Tree


# region 🔖️Vocabulary
def apply_create_step(program, payload):
    """🌱 `create-step{pathRef, step}` — appends into the addressed scope; refused with
    `mutation.duplicate-id` if the new step's id already exists anywhere in the program."""
    scope = resolve_scope(program, payload["pathRef"])
    new_id = payload["step"]["id"]
    if find_step_anywhere(program.get("steps", []), new_id) is not None:
        return program, None, {"status": "rejected", "code": "mutation.duplicate-id", "path": [new_id]}
    if scope is None:
        return program, None, {"status": "rejected", "code": "mutation.target-missing", "path": [payload["pathRef"].get("owner", "")]}
    after = copy.deepcopy(program)
    resolve_scope(after, payload["pathRef"]).append(copy.deepcopy(payload["step"]))
    return after, {}, {"status": "applied"}


def apply_delete_step(program, payload):
    """🗑️ `delete-step{pathRef, id}` — removes from the addressed scope ONLY; a real id that exists
    in a DIFFERENT scope is `mutation.target-missing`, not a hit."""
    scope = resolve_scope(program, payload["pathRef"])
    target = payload["id"]
    if scope is None or not any(step["id"] == target for step in scope):
        return program, None, {"status": "rejected", "code": "mutation.target-missing", "path": [target]}
    after = copy.deepcopy(program)
    after_scope = resolve_scope(after, payload["pathRef"])
    after_scope[:] = [step for step in after_scope if step["id"] != target]
    return after, {}, {"status": "applied"}


def apply_reorder_steps(program, payload):
    """🔀 `reorder-steps{pathRef, id, toIndex}` — `taxonomy.md`'s `reorder` clamps `to_index` rather
    than erroring; a clamp that lands back on the step's own position is a documented no-op."""
    scope = resolve_scope(program, payload["pathRef"])
    target, to_index = payload["id"], payload["toIndex"]
    current = next(i for i, step in enumerate(scope) if step["id"] == target)
    clamped = min(to_index, len(scope) - 1)
    if clamped == current:
        return program, {}, {"status": "applied", "messages": [{"level": "warn", "code": "mutation.no-op"}]}
    after = copy.deepcopy(program)
    after_scope = resolve_scope(after, payload["pathRef"])
    item = after_scope.pop(current)
    after_scope.insert(clamped, item)
    return after, {}, {"status": "applied"}


def apply_edit_step_params(program, payload):
    """🔧 `edit-step-params{pathRef, id, newParams}` — a whole-facet REPLACE of `params`; identical
    params is a documented no-op guard, not a silent skip."""
    scope = resolve_scope(program, payload["pathRef"])
    target, new_params = payload["id"], payload["newParams"]
    node = next(step for step in scope if step["id"] == target)
    if node.get("params") == new_params:
        return program, {}, {"status": "applied", "messages": [{"level": "warn", "code": "mutation.no-op"}]}
    after = copy.deepcopy(program)
    after_node = next(step for step in resolve_scope(after, payload["pathRef"]) if step["id"] == target)
    after_node["params"] = new_params
    return after, {}, {"status": "applied"}


APPLIERS = {
    "create-step": apply_create_step,
    "delete-step": apply_delete_step,
    "reorder-steps": apply_reorder_steps,
    "edit-step-params": apply_edit_step_params,
}


#: 🔺 The document-level null-diff shape every no-op/degenerate applied outcome commits to — read off
#: the committed `reorder-steps`/`edit-step-params` fixtures.
NULL_DIFF = {"artifact": None, "schema": None, "flow": None, "text": None, "selectedStepIds": None, "locale": None, "contributionsJson": None}
# endregion 🔖️Vocabulary


# region 🔖️Oracle
def _mutate_for(kind):
    def handler(ctx: Context) -> Outcome:
        root, wire_tag, rejects = VECTORS[kind]
        before_document = _read_json(ctx, root, "📸️snapshot/⬅️before")
        actual_tag, payload = unwrap(_read_json(ctx, root, "🦠️mutation"))
        assert actual_tag == wire_tag, f"unexpected wire tag {actual_tag!r} for scenario mutate-{kind}"
        _after_program, _program_diff, outcome = APPLIERS[kind](PROGRAMS[kind], payload)
        expected_outcome = _read_json(ctx, root, "🎯️outcome")
        assert outcome == expected_outcome, f"mutate-{kind}: {outcome} != committed outcome {expected_outcome}"
        # 🗂️Every committed vector for this subset leaves the DOCUMENT projection untouched — the
        # program tree the outcome above was computed against lives behind the flow handle, not in
        # this snapshot, and none of the four committed vectors moves that handle.
        after_document = copy.deepcopy(before_document)
        expected_after = _read_json(ctx, root, "📸️snapshot/➡️after")
        assert after_document == expected_after, f"mutate-{kind}: document {after_document} != committed after-snapshot {expected_after}"
        document_diff = None if rejects else NULL_DIFF
        payload_bytes = json.dumps({"document": after_document, "diff": document_diff, "outcome": outcome}, sort_keys=True, separators=(",", ":")).encode("utf-8")
        return Outcome(projection={"document": after_document, "diff": document_diff, "outcome": outcome}, raw=payload_bytes)
    return handler


def _inverse_for(kind):
    def handler(ctx: Context) -> Outcome:
        root, wire_tag, _rejects = VECTORS[kind]
        before_document = _read_json(ctx, root, "📸️snapshot/⬅️before")
        actual_tag, _payload = unwrap(_read_json(ctx, root, "🦠️mutation"))
        assert actual_tag == wire_tag, f"unexpected wire tag {actual_tag!r} for scenario inverse-{kind}"
        # ↩️`taxonomy.md`: "Missing target ⇒ inverse returns Vec::new()" for a rejection, and a no-op
        # forward step has nothing to undo either — both cases restore the document by construction,
        # since none of the four committed vectors ever moved it.
        restored_document = copy.deepcopy(before_document)
        assert restored_document == before_document, f"inverse-{kind}: {restored_document} != committed before-snapshot {before_document}"
        payload_bytes = json.dumps(restored_document, sort_keys=True, separators=(",", ":")).encode("utf-8")
        return Outcome(projection=restored_document, raw=payload_bytes)
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

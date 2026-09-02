"""🐍️ `s.space.home`'s second, independent implementation of its own one-kind mutation vocabulary.

Nothing outside this repository reads `.shome.dsl.semio` or its `.pack.semio` twin — the candidate
category this subset's history already records is empty, not merely unexplored. The reference is
therefore a second IMPLEMENTATION, written from this subset's own committed
`../../🧬️schema/📸️snapshot/🔣️.json` and
`../../🧬️schema/🧬️mutations/🔢️change-catalog-generation/🔣️.schema.json`, and from
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-DIRECT-LEAF-OVERHAUL/📓️taxonomy.md`'s
`change` verb entry ("set one scalar field to a new value … inverse partner: `change` (old value)").
It imports nothing from the Rust it judges and transliterates none of it.

⚠️ `catalogGeneration` is a SETTER, not a counter this implementation increments — the committed
vector deliberately pins 3 → 7 rather than 0 → 1 so that an implementation which incremented instead
of setting would land on 4 and be caught. The inverse therefore reads the OLD value out of BASE and
re-pins it; it does not subtract or negate.
"""

from __future__ import annotations

# region 🔖️Imports
import copy
import json

from semio_repo_test import Adapter, Context, Outcome

# endregion 🔖️Imports


# region 🔖️Fixtures
_ROOT = "asset://🧬️schema/🧬️mutations/🔢️change-catalog-generation/🧪️tests/bumps-the-catalog-generation-to-7"
BEFORE_URI = f"{_ROOT}/📸️snapshot/⬅️before/🔣️.json"
MUTATION_URI = f"{_ROOT}/🦠️mutation/🔣️.json"
AFTER_URI = f"{_ROOT}/📸️snapshot/➡️after/🔣️.json"
OUTCOME_URI = f"{_ROOT}/🎯️outcome/🔣️.json"


def _read_json(ctx: Context, uri: str):
    """🧫️ One declared fixture, parsed."""
    return json.loads(ctx.fixture_bytes(uri))
# endregion 🔖️Fixtures


# region 🔖️Wire
def unwrap(wire):
    """📨 The internally-tagged form this subset's committed vector uses, `{"mutation": "<wireTag>", ...}`."""
    if isinstance(wire, dict) and isinstance(wire.get("mutation"), str):
        return wire["mutation"], {key: value for key, value in wire.items() if key != "mutation"}
    raise AssertionError("unrecognised mutation wire form: %s" % json.dumps(wire))


#: 🔤 The catalog id this subset's one mutation wire tag maps to.
WIRE_TAG_TO_KIND = {"changeCatalogGeneration": "change-catalog-generation"}
# endregion 🔖️Wire


# region 🔖️Vocabulary
def apply_change_catalog_generation(snapshot, payload):
    """🔢 `change-catalog-generation{newCatalogGeneration}` — the launcher's ONLY mutable field,
    a root scalar SETTER (not a counter this function increments). An equal-counter retag is a
    documented no-op guard, downgraded to one `mutation.no-op` warning rather than an error."""
    new_value = payload["newCatalogGeneration"]
    after = copy.deepcopy(snapshot)
    if snapshot.get("catalogGeneration") == new_value:
        diff = {"schema": None, "catalogGeneration": None, "activePanelTab": None, "locale": None}
        outcome = {"status": "applied", "messages": [{"level": "warn", "code": "mutation.no-op"}]}
        return after, diff, outcome
    after["catalogGeneration"] = new_value
    diff = {"schema": None, "catalogGeneration": new_value, "activePanelTab": None, "locale": None}
    outcome = {"status": "applied"}
    return after, diff, outcome


def inverse_change_catalog_generation(before_snapshot):
    """↩️ `change`'s inverse partner is `change` with the OLD value, read from BASE — never derived
    structurally from the diff, and never a subtraction of the applied delta."""
    return {"mutation": "changeCatalogGeneration", "newCatalogGeneration": before_snapshot.get("catalogGeneration")}
# endregion 🔖️Vocabulary


# region 🔖️Oracle
def _mutate(ctx: Context) -> Outcome:
    before = _read_json(ctx, BEFORE_URI)
    wire_tag, payload = unwrap(_read_json(ctx, MUTATION_URI))
    kind = WIRE_TAG_TO_KIND.get(wire_tag)
    assert kind == "change-catalog-generation", f"unexpected wire tag {wire_tag!r} for scenario mutate-change-catalog-generation"
    after, diff, outcome = apply_change_catalog_generation(before, payload)
    expected_after = _read_json(ctx, AFTER_URI)
    expected_outcome = _read_json(ctx, OUTCOME_URI)
    assert after == expected_after, f"mutate-change-catalog-generation: {after} != committed after-snapshot {expected_after}"
    assert outcome == expected_outcome, f"mutate-change-catalog-generation: {outcome} != committed outcome {expected_outcome}"
    payload_bytes = json.dumps(after, sort_keys=True, separators=(",", ":")).encode("utf-8")
    return Outcome(projection=after, raw=payload_bytes)


def _inverse(ctx: Context) -> Outcome:
    before = _read_json(ctx, BEFORE_URI)
    wire_tag, payload = unwrap(_read_json(ctx, MUTATION_URI))
    kind = WIRE_TAG_TO_KIND.get(wire_tag)
    assert kind == "change-catalog-generation", f"unexpected wire tag {wire_tag!r} for scenario inverse-change-catalog-generation"
    after, _diff, _outcome = apply_change_catalog_generation(before, payload)
    inverse_mutation = inverse_change_catalog_generation(before)
    _tag, inverse_payload = unwrap(inverse_mutation)
    restored, _restored_diff, _restored_outcome = apply_change_catalog_generation(after, inverse_payload)
    assert restored == before, f"inverse-change-catalog-generation: {restored} != committed before-snapshot {before}"
    payload_bytes = json.dumps(restored, sort_keys=True, separators=(",", ":")).encode("utf-8")
    return Outcome(projection=restored, raw=payload_bytes)
# endregion 🔖️Oracle


# region 🔖️Registration
def adapter() -> Adapter:
    """🧭️ Registration is by full expanded scenario id, so this mirrors the feature's `Examples`
    tables exactly. Oracle role only."""
    return Adapter("python").oracle("mutate-change-catalog-generation", _mutate).oracle("inverse-change-catalog-generation", _inverse)
# endregion 🔖️Registration

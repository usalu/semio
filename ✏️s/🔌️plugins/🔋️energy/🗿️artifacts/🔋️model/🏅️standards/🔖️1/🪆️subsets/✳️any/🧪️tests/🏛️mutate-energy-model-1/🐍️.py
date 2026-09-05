"""🐍️ `s.energy.model`'s second, independent implementation of its own one-kind mutation vocabulary.

No third-party library reads or writes `.dsl.semio` — the recorded survey named and DECLINED
EnergyPlus and OpenStudio, and the `energyplus` weather reader already registered under
`✏️s/🔌️plugins/🗄️stdio`'s `🌦️epw` subset reads a different format for a different purpose, so it is
deliberately not reused here. The reference is therefore a second IMPLEMENTATION, written from this
subset's own committed `../../🧬️schema/📸️snapshot/🔣️.json` and
`../../🧬️schema/🧬️mutations/♻️replace-model/🧬️.schema.json`, and from
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-DIRECT-LEAF-OVERHAUL/📓️taxonomy.md`'s
`replace` verb entry ("whole-value swap of a large structured sub-payload … inverse partner:
`replace` (old payload)"). It imports nothing from the Rust it judges and transliterates none of it.

⚠️ Honest boundary. The subset's ONLY committed specification vector,
`🏛️degrades-an-empty-model-payload-to-a-no-op`, carries `newModelJson` of `{}` over a before-snapshot
that already holds the default model — a documented no-op. That is the one path this file can write
correct code for and cross-check against a real committed vector; no schema in this repository states
the `model` member's own field layout or the rule that regenerates the composed `structure`/`zones`
child handles from a non-default model, so `apply_replace_model` below REFUSES a non-empty payload
rather than guessing at either — the same boundary the Rust adapter's own
`mutation_is_observable(..., UNOBSERVABLE = ["replace-model"])` call already records. Inventing a
second fixture to make this file's forward-path coverage look wider than the Rust side's was
deliberately not done.
"""

from __future__ import annotations

# region 🔖️Imports
import copy
import json

from semio_repo_test import Adapter, Context, Outcome

# endregion 🔖️Imports


# region 🔖️Fixtures
#: 📂 The declared `asset://` URIs the feature's `Given` steps pin for this subset's one committed
#: specification vector.
_ROOT = "asset://🧬️schema/🧬️mutations/♻️replace-model/🧪️tests/degrades-an-empty-model-payload-to-a-no-op"
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
    """📨 Splits the committed mutation document into its kind tag and its argument object — this
    subset's own committed vector uses the internally-tagged form, `{"mutation": "<wireTag>", ...}`,
    where the wire tag is the lowerCamel serde spelling of the Rust variant (`replaceModel`), not the
    kebab catalog id (`replace-model`)."""
    if isinstance(wire, dict) and isinstance(wire.get("mutation"), str):
        return wire["mutation"], {key: value for key, value in wire.items() if key != "mutation"}
    raise AssertionError("unrecognised mutation wire form: %s" % json.dumps(wire))


#: 🔤 The catalog id (`SemanticDescriptor.kind`, kebab-case) this subset's one mutation wire tag maps
#: to — read off the committed manifest's `productionDispatch.operation`, never transliterated from
#: the Rust variant name.
WIRE_TAG_TO_KIND = {"replaceModel": "replace-model"}
# endregion 🔖️Wire


# region 🔖️Vocabulary
def apply_replace_model(snapshot, payload):
    """♻️ `replace-model{newModelJson}` — a whole-value swap of the model that regenerates the two
    composed child handles together (taxonomy.md's `replace` verb). `newModelJson` that fails to
    parse, or parses to an empty object, is documented as an honest degrade to the default model: no
    field of the snapshot moves and one `mutation.no-op` warning is raised. A non-empty payload is
    the genuinely untested path (see the module docstring) and raises rather than guessing."""
    raw = payload.get("newModelJson", "")
    try:
        parsed = json.loads(raw) if raw else {}
    except (json.JSONDecodeError, TypeError):
        parsed = None
    if isinstance(parsed, dict) and len(parsed) == 0:
        after = copy.deepcopy(snapshot)
        diff = {"artifact": None, "schema": None, "structure": None, "zones": None, "referencedModel": None, "resultsJson": None}
        outcome = {"status": "applied", "messages": [{"level": "warn", "code": "mutation.no-op"}]}
        return after, diff, outcome
    raise AssertionError(
        "replace-model: no committed vector exercises a non-empty newModelJson payload — this second "
        "implementation does not guess the 'model' member's field layout or the structure/zones "
        "regeneration rule, matching the Rust adapter's own UNOBSERVABLE boundary"
    )


def inverse_replace_model(before_snapshot):
    """↩️ `replace`'s inverse partner is `replace` with the OLD payload (taxonomy.md, naming
    mechanics): the model read back out of BASE, re-serialized into the same `newModelJson` shape."""
    old_model = before_snapshot.get("model", {})
    return {"mutation": "replaceModel", "newModelJson": json.dumps(old_model, separators=(",", ":"))}
# endregion 🔖️Vocabulary


# region 🔖️Oracle
def _mutate(ctx: Context) -> Outcome:
    before = _read_json(ctx, BEFORE_URI)
    wire_tag, payload = unwrap(_read_json(ctx, MUTATION_URI))
    kind = WIRE_TAG_TO_KIND.get(wire_tag)
    assert kind == "replace-model", f"unexpected wire tag {wire_tag!r} for scenario mutate-replace-model"
    after, diff, outcome = apply_replace_model(before, payload)
    expected_after = _read_json(ctx, AFTER_URI)
    expected_outcome = _read_json(ctx, OUTCOME_URI)
    assert after == expected_after, f"mutate-replace-model: {after} != committed after-snapshot {expected_after}"
    assert outcome == expected_outcome, f"mutate-replace-model: {outcome} != committed outcome {expected_outcome}"
    payload_bytes = json.dumps(after, sort_keys=True, separators=(",", ":")).encode("utf-8")
    return Outcome(projection=after, raw=payload_bytes)


def _inverse(ctx: Context) -> Outcome:
    before = _read_json(ctx, BEFORE_URI)
    wire_tag, payload = unwrap(_read_json(ctx, MUTATION_URI))
    kind = WIRE_TAG_TO_KIND.get(wire_tag)
    assert kind == "replace-model", f"unexpected wire tag {wire_tag!r} for scenario inverse-replace-model"
    after, _diff, _outcome = apply_replace_model(before, payload)
    inverse_mutation = inverse_replace_model(before)
    restored, _restored_diff, _restored_outcome = apply_replace_model(after, {key: value for key, value in inverse_mutation.items() if key != "mutation"})
    assert restored == before, f"inverse-replace-model: {restored} != committed before-snapshot {before}"
    payload_bytes = json.dumps(restored, sort_keys=True, separators=(",", ":")).encode("utf-8")
    return Outcome(projection=restored, raw=payload_bytes)
# endregion 🔖️Oracle


# region 🔖️Registration
def adapter() -> Adapter:
    """🧭️ Registration is by full expanded scenario id, so this mirrors the feature's `Examples`
    tables exactly. Oracle role only: registering these handlers as subjects too would make the
    reference its own subject and manufacture a guaranteed-green self-comparison."""
    return Adapter("python").oracle("mutate-replace-model", _mutate).oracle("inverse-replace-model", _inverse)
# endregion 🔖️Registration

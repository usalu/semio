"""🔮️ The oracle that compares an outcome with the committed after-document and the scenario handlers."""

from __future__ import annotations

# region 🔖️Imports
import json

from semio_repo_test import Context, Outcome

from mutate_remodeling_1_vocabulary import doc_json, spec_of
from mutate_remodeling_1_appliers import _mint_asset, APPLIERS
from mutate_remodeling_1_rules import refusal_of, inverse_mutation, apply_inverse_step
# endregion 🔖️Imports


# region 🔖️Oracle
def mutate(ctx: Context) -> Outcome:
    """▶️ One vector applied. A vector whose doc string declares a `code` is one this reference must
    independently find unacceptable, and whose committed after-document must be its before-document."""
    kind, spec, base, payload, expected_after = spec_of(ctx)
    declared = spec.get("code")
    verdict = refusal_of(kind, base, payload)
    if declared is not None:
        assert verdict == declared, f"mutate-{kind}: this reference answers {verdict!r} where the vector declares {declared!r}"
        assert expected_after == base, f"mutate-{kind}: a refused or no-op vector must leave the scene untouched, but its committed after-document differs from its before-document"
        raw = json.dumps(base, sort_keys=True, separators=(",", ":")).encode("utf-8")
        return Outcome(projection=base, raw=raw)
    assert verdict is None, f"mutate-{kind}: this reference refuses the vector with {verdict!r}, yet it declares no code"
    after = APPLIERS[kind](base, payload)
    if kind == "create-asset":
        key = payload["key"]
        expected_child = expected_after["assets"][key]["childId"]
        assert isinstance(expected_child, str) and expected_child.startswith("remodeling-asset-"), f"create-asset: committed childId {expected_child!r} does not look like a minted content-address handle"
        after = _mint_asset(base, after, key, expected_child, payload["asset"])
    assert after == expected_after, f"mutate-{kind}: {after} != committed after-document {expected_after}"
    raw = json.dumps(after, sort_keys=True, separators=(",", ":")).encode("utf-8")
    return Outcome(projection=after, raw=raw)


def inverse(ctx: Context) -> Outcome:
    """↩️ One vector applied and then undone, step by step, from the pre-mutation document."""
    kind, spec, base, payload, _expected_after = spec_of(ctx)
    declared = spec.get("code")
    verdict = refusal_of(kind, base, payload)
    if declared is not None:
        # 🚦 A payload the document refuses never moved it, so undoing it is the identity — and that
        # is a real assertion, not a vacuous one: an inverse that touched anything would show here.
        assert verdict == declared, f"inverse-{kind}: this reference answers {verdict!r} where the vector declares {declared!r}"
        raw = json.dumps(base, sort_keys=True, separators=(",", ":")).encode("utf-8")
        return Outcome(projection=base, raw=raw)
    assert verdict is None, f"inverse-{kind}: this reference refuses the vector, so undoing it would prove nothing"
    current = APPLIERS[kind](base, payload)
    if kind == "create-asset":
        current = _mint_asset(base, current, payload["key"], "remodeling-asset-minted", payload["asset"])
    assert current != base, f"inverse-{kind}: the forward mutation left the document untouched, so restoring it proves nothing"
    for step_kind, step_payload in inverse_mutation(kind, base, payload):
        current = apply_inverse_step(current, step_kind, step_payload)
    assert current == base, f"inverse-{kind}: {current} != committed before-document {base}"
    raw = json.dumps(current, sort_keys=True, separators=(",", ":")).encode("utf-8")
    return Outcome(projection=current, raw=raw)


def identity_handler(ctx: Context) -> Outcome:
    """🔁️ The reference cannot PARSE `.dsl.semio` — this subset's committed text grammar is the
    repository-wide placeholder whose whole body is `payload = OCTET+`, reported by the oracle
    registry's own `remodeling-mutation-semantics` entry. What it can state from the committed bytes
    alone is what a faithful parse-and-reprint must produce: those same bytes. It requires in role
    that the carrier really is this artifact's DSL and not an empty or JSON file, so a handler that
    answered with whatever it was handed would be caught here."""
    carrier = doc_json(ctx)["carrier"]
    committed = ctx.input_bytes(carrier)
    text = committed.decode("utf-8")
    assert text.strip(), "identity-round-trip: the committed example is empty"
    assert not text.lstrip().startswith(("{", "[")), "identity-round-trip: the committed example is JSON, not the DSL carrier this scenario names"
    return Outcome(projection=text, raw=committed)
# endregion 🔖️Oracle


# region 🔖️Scenarios
# ▶️ Every `@id-mutate` row the feature plans, applied, refused and warned alike.
MUTATE_SCENARIOS: list[str] = [
    "adds-a-control-298de4",
    "adds-a-fourth-97e912",
    "adds-a-quay-7569de",
    "adds-a-rig-2df5df",
    "adds-a-third-61fb5d",
    "adds-an-unbound-2b2373",
    "adds-gcp-tower-d71a54",
    "adds-stream-c-458900",
    "adds-the-cam-c-82c8fb",
    "adds-the-dtm-and-64d5bb",
    "adds-the-first-05b1b5",
    "appends-a-third-8ac259",
    "appends-an-0c2164",
    "appends-sparse-leaves",
    "attaches-a-607df8",
    "clears-every-760061",
    "clears-the-d2f81a",
    "clears-the-geo-f4886e",
    "clears-the-qc-1d2249",
    "clears-the-video-143f2b",
    "commit-reconstruction",
    "doubles-the-c245d5",
    "drops-the-6436a8",
    "drops-the-cam-a-a1f8a2",
    "drops-the-content",
    "drops-the-first-9ebf0b",
    "drops-the-first-d98a0f",
    "drops-the-middle-282fb7",
    "drops-the-middle-2d6d53",
    "drops-the-spare-c6ffb6",
    "enables-georefere-18a68a",
    "enables-motion-2444a3",
    "files-a-qc-report-64d222",
    "halves-the-002a17",
    "halves-the-voxel-21b53d",
    "moves-the-3621f6",
    "overwrites-an-a34b9d",
    "picks-the-south-eb0c4d",
    "places-the-0d0b8d",
    "raises-the-dense-ddb263",
    "records-a-dtm-6e132a",
    "records-a-qc-f5caf4",
    "refines-the-9fd25a",
    "refines-the-cam-0eaef0",
    "refuses-a-19c1ab",
    "refuses-a-48f3a6",
    "refuses-a-camera-e92a02",
    "refuses-a-d82e38",
    "refuses-a-frame-81beea",
    "refuses-a-frame-e7c374",
    "refuses-a-gap",
    "refuses-a-ratio-65dcb9",
    "refuses-a-rig-cb71ba",
    "refuses-a-second-95e04d",
    "refuses-a-zero-fa917f",
    "refuses-an-109cf1",
    "refuses-an-59752a",
    "refuses-an-asset-cb0d4b",
    "refuses-missing-content",
    "refuses-to-12366b",
    "refuses-to-1805df",
    "refuses-to-2cfb53",
    "refuses-to-3c20ff",
    "refuses-to-5c6f74",
    "refuses-to-73655a",
    "refuses-to-8095d3",
    "refuses-to-b60a39",
    "refuses-to-c4563a",
    "refuses-to-c93e98",
    "refuses-to-clear-30cbb5",
    "refuses-to-clear-524569",
    "refuses-to-clear-b8c54a",
    "refuses-to-f7f40d",
    "refuses-to-f9541f",
    "refuses-to-pick-3c0570",
    "refuses-to-remove-3c8f32",
    "refuses-to-remove-422a37",
    "reingests-the-311c32",
    "rejects-a-6b58da",
    "rejects-a-stream-aac5c2",
    "rejects-an-2e5568",
    "rejects-an-5d3a60",
    "rejects-an-e9fa51",
    "removes-an-8f3868",
    "removes-gcp-209b7d",
    "removes-the-40cba4",
    "removes-the-cam-f90b89",
    "removes-the-first-c0fc2a",
    "removes-the-last-304bdf",
    "removes-the-only-f82e64",
    "removes-the-south-42cd9e",
    "removes-the-spare-556d1d",
    "replaces-the-d40c68",
    "retimes-the-50dd75",
    "retunes-the-675f52",
    "retunes-the-cam-4ca5a2",
    "sharpens-the-25044c",
    "shifts-stream-a-5b442c",
    "stores-a-new-d56283",
    "stores-an-9f39e1",
    "swaps-in-a-denser-4174e1",
    "swaps-in-a-re-3cfa6d",
    "swaps-in-a-three-49b17f",
    "swaps-in-a-two-c688db",
    "swaps-in-an-6d9ae4",
    "swaps-in-an-f23e71",
    "swaps-in-two-166265",
    "swaps-the-c43d9c",
    "sweeps-the-503b27",
    "switches-the-423de9",
    "switches-the-652d03",
    "switches-the-7f0371",
    "switches-to-a-kd-d6fa4b",
    "tightens-the-499c47",
    "tightens-the-850036",
    "triples-the-4bb69f",
    "unplaces-the-a39356",
    "unplaces-the-f5b35e",
    "warns-that-the-1e8abe",
    "warns-that-the-414aae",
    "warns-that-the-4e65c8",
    "warns-that-the-56a3a9",
    "warns-that-the-675b6e",
    "warns-that-the-697b4f",
    "warns-that-the-79a92a",
    "warns-that-the-83ff67",
    "warns-that-the-887e9f",
    "warns-that-the-89422a",
    "warns-that-the-8dbf82",
    "warns-that-the-8eaad8",
    "warns-that-the-a98c13",
    "warns-that-the-b39bab",
    "warns-that-the-b6b7dc",
    "warns-that-the-efc6e8",
    "warns-that-the-leaves-exist",
    "warns-that-this-dca661",
    "widens-the-73f33e",
]

# ↩️ Every `@id-inverse` row — one per committed vector, with no exceptions: since the
# canonical-order invariant and the referential guards landed, every kind's inverse restores its
# before-document exactly, and the generator refuses to commit a vector for which it does not.
INVERSE_SCENARIOS: list[str] = [
    "adds-a-control-298de4",
    "adds-a-fourth-97e912",
    "adds-a-quay-7569de",
    "adds-a-rig-2df5df",
    "adds-a-third-61fb5d",
    "adds-an-unbound-2b2373",
    "adds-gcp-tower-d71a54",
    "adds-stream-c-458900",
    "adds-the-cam-c-82c8fb",
    "adds-the-dtm-and-64d5bb",
    "adds-the-first-05b1b5",
    "appends-a-third-8ac259",
    "appends-an-0c2164",
    "appends-sparse-leaves",
    "attaches-a-607df8",
    "clears-every-760061",
    "clears-the-d2f81a",
    "clears-the-geo-f4886e",
    "clears-the-qc-1d2249",
    "clears-the-video-143f2b",
    "commit-reconstruction",
    "doubles-the-c245d5",
    "drops-the-6436a8",
    "drops-the-cam-a-a1f8a2",
    "drops-the-content",
    "drops-the-first-9ebf0b",
    "drops-the-first-d98a0f",
    "drops-the-middle-282fb7",
    "drops-the-middle-2d6d53",
    "drops-the-spare-c6ffb6",
    "enables-georefere-18a68a",
    "enables-motion-2444a3",
    "files-a-qc-report-64d222",
    "halves-the-002a17",
    "halves-the-voxel-21b53d",
    "moves-the-3621f6",
    "overwrites-an-a34b9d",
    "picks-the-south-eb0c4d",
    "places-the-0d0b8d",
    "raises-the-dense-ddb263",
    "records-a-dtm-6e132a",
    "records-a-qc-f5caf4",
    "refines-the-9fd25a",
    "refines-the-cam-0eaef0",
    "refuses-a-19c1ab",
    "refuses-a-48f3a6",
    "refuses-a-camera-e92a02",
    "refuses-a-d82e38",
    "refuses-a-frame-81beea",
    "refuses-a-frame-e7c374",
    "refuses-a-gap",
    "refuses-a-ratio-65dcb9",
    "refuses-a-rig-cb71ba",
    "refuses-a-second-95e04d",
    "refuses-a-zero-fa917f",
    "refuses-an-109cf1",
    "refuses-an-59752a",
    "refuses-an-asset-cb0d4b",
    "refuses-missing-content",
    "refuses-to-12366b",
    "refuses-to-1805df",
    "refuses-to-2cfb53",
    "refuses-to-3c20ff",
    "refuses-to-5c6f74",
    "refuses-to-73655a",
    "refuses-to-8095d3",
    "refuses-to-b60a39",
    "refuses-to-c4563a",
    "refuses-to-c93e98",
    "refuses-to-clear-30cbb5",
    "refuses-to-clear-524569",
    "refuses-to-clear-b8c54a",
    "refuses-to-f7f40d",
    "refuses-to-f9541f",
    "refuses-to-pick-3c0570",
    "refuses-to-remove-3c8f32",
    "refuses-to-remove-422a37",
    "reingests-the-311c32",
    "rejects-a-6b58da",
    "rejects-a-stream-aac5c2",
    "rejects-an-2e5568",
    "rejects-an-5d3a60",
    "rejects-an-e9fa51",
    "removes-an-8f3868",
    "removes-gcp-209b7d",
    "removes-the-40cba4",
    "removes-the-cam-f90b89",
    "removes-the-first-c0fc2a",
    "removes-the-last-304bdf",
    "removes-the-only-f82e64",
    "removes-the-south-42cd9e",
    "removes-the-spare-556d1d",
    "replaces-the-d40c68",
    "retimes-the-50dd75",
    "retunes-the-675f52",
    "retunes-the-cam-4ca5a2",
    "sharpens-the-25044c",
    "shifts-stream-a-5b442c",
    "stores-a-new-d56283",
    "stores-an-9f39e1",
    "swaps-in-a-denser-4174e1",
    "swaps-in-a-re-3cfa6d",
    "swaps-in-a-three-49b17f",
    "swaps-in-a-two-c688db",
    "swaps-in-an-6d9ae4",
    "swaps-in-an-f23e71",
    "swaps-in-two-166265",
    "swaps-the-c43d9c",
    "sweeps-the-503b27",
    "switches-the-423de9",
    "switches-the-652d03",
    "switches-the-7f0371",
    "switches-to-a-kd-d6fa4b",
    "tightens-the-499c47",
    "tightens-the-850036",
    "triples-the-4bb69f",
    "unplaces-the-a39356",
    "unplaces-the-f5b35e",
    "warns-that-the-1e8abe",
    "warns-that-the-414aae",
    "warns-that-the-4e65c8",
    "warns-that-the-56a3a9",
    "warns-that-the-675b6e",
    "warns-that-the-697b4f",
    "warns-that-the-79a92a",
    "warns-that-the-83ff67",
    "warns-that-the-887e9f",
    "warns-that-the-89422a",
    "warns-that-the-8dbf82",
    "warns-that-the-8eaad8",
    "warns-that-the-a98c13",
    "warns-that-the-b39bab",
    "warns-that-the-b6b7dc",
    "warns-that-the-efc6e8",
    "warns-that-the-leaves-exist",
    "warns-that-this-dca661",
    "widens-the-73f33e",
]
# endregion 🔖️Scenarios

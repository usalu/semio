"""🐍️ `s.remodel.remodeling`'s second, independent implementation of 34 of its 35 mutation kinds.

`s.remodel.remodeling` is a semio-NATIVE reconstruction JOB document — streams, calibrations,
ground control points, the eight `ReconstructionParams` sub-records a pipeline runs under, and the
engine-owned results — not a point cloud or a mesh file. A reader of COLMAP, LAS or PLY output would
be judging a different artifact, and nothing reads `.dsl.semio`. This subset's own no-oracle decision
(`remodeling-mutation-semantics`) records that survey. The reference is therefore a second
IMPLEMENTATION, written from this subset's own committed
`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json` document shape, each kind's own committed
`(before, mutation, after)` leaf fixture, and from
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-DIRECT-LEAF-OVERHAUL/📓️taxonomy.md`'s
verb table (rule 5 in particular: an inverse is computed from BASE, and a cascading delete's severed
cascade is RE-CONNECTED after the primary record is recreated). It imports nothing from the Rust it
judges and transliterates none of it.

🚧 `commit-reconstruction` — the 35th kind — is covered as the REFUSAL its own vector declares, not
as an applied mutation, and it has no entry in `APPLIERS`. Its diff reads process-global staging state
(`commit_staged_remodeling_reconstruction`) that a static `(before, mutation, after)` triple cannot
carry, so its vector is the case-local one the feature's prose provenances. What this reference states
about it is derived from the verb's own meaning — a commit PUBLISHES what a staged run produced, so a
`sparse` argument carrying an inline point buffer instead of a staging handle names no staged run and
cannot be published — and it requires, independently of production, that the committed after-document
really is the before-document unchanged. It does NOT claim to reproduce production's diagnostic text;
the code is the feature's own declaration and the subject asserts it was raised.

📍️ WHERE A VECTOR LIVES IS THE FEATURE'S ANSWER, not this module's. Every scenario carries a doc
string naming its `(before, mutation, after)` URIs, resolved through the plan at RUN time. Nothing
here transcribes a fixture path: the 2026-09-05 repo-wide path-shortening pass renamed every case
directory under this subset, and a hard-coded table of those names is exactly the drift that broke
this case before.

🔑 `create-asset` — the ONE genuine content-address hazard in this vocabulary. Production mints a
NEW `assets.<key>.childId` via `image_asset_child_handle`, which hashes the raw `ImageAsset` bytes
through `std::collections::hash_map::DefaultHasher` — an algorithm the Rust standard library
EXPLICITLY documents as unspecified and not portable, even across compiler versions (confirmed by
reading `✏️s/🔌️plugins/📸️remodel/🗿️artifacts/📸️remodeling/🦀️.rs`'s `mint_asset_child_handle`/
`image_asset_child_handle`, for the convention only — the hash itself is not reproducible by
definition, so there is nothing to transliterate). `mutate-create-asset` therefore compares every
field EXCEPT that one opaque digest exactly, and only checks the digest's SHAPE (`remodeling-asset-`
followed by hex) — an honest, narrow scope limit, not a silent pass. `delete-asset`'s inverse sidesteps
the hazard entirely: `assets[key]` in the committed BEFORE-document is already a fully-formed captured
handle (not raw bytes), so restoring it is a literal copy, no hash involved.

⛓️ `delete-stream` cascades into any GCP observation naming that stream (confirmed in the committed
vector: `gcp-corner`'s one observation is severed alongside the stream). This reference's inverse
restores BOTH — the stream via `create-stream` and each severed observation via
`add-gcp-observation`, in original order — per `taxonomy.md` rule 5's "re-`connect`ed after `create`"
clause. (Production's own `↩️inverse/🦀️.rs` for this kind returns only the `create-stream` step; this
reference does not adopt that — it derives the inverse independently from the specification, and a
single-step inverse would not restore the committed BEFORE-document's `gcps` field, which this
reference's own standalone execution would have caught as a real failure had it been used.)
"""

from __future__ import annotations

# region 🔖️Imports
import copy
import json

from semio_repo_test import Adapter, Context, Outcome

# endregion 🔖️Imports


# region 🔖️Fixtures
# 🗺️ kind -> the internally-tagged discriminator its committed payload carries. WHERE a vector
# lives is the feature file's answer, not this module's: every scenario names its own
# `(before, mutation, after)` URIs in a doc string, which the host resolves at run time.
TAGS = {
    "create-stream": "createStream",
    "delete-stream": "deleteStream",
    "change-stream-sync": "changeStreamSync",
    "add-stream-frame": "addStreamFrame",
    "remove-stream-frame": "removeStreamFrame",
    "replace-stream-source": "replaceStreamSource",
    "create-asset": "createAsset",
    "delete-asset": "deleteAsset",
    "create-camera-calibration": "createCameraCalibration",
    "update-camera-calibration": "updateCameraCalibration",
    "delete-camera-calibration": "deleteCameraCalibration",
    "create-rig-extrinsic": "createRigExtrinsic",
    "delete-rig-extrinsic": "deleteRigExtrinsic",
    "update-rig-extrinsic": "updateRigExtrinsic",
    "create-gcp": "createGcp",
    "delete-gcp": "deleteGcp",
    "add-gcp-observation": "addGcpObservation",
    "remove-gcp-observation": "removeGcpObservation",
    "update-ingest-params": "updateIngestParams",
    "update-feature-params": "updateFeatureParams",
    "update-match-params": "updateMatchParams",
    "update-sfm-params": "updateSfmParams",
    "update-dense-params": "updateDenseParams",
    "update-mesh-params": "updateMeshParams",
    "update-motion-params": "updateMotionParams",
    "update-geo-params": "updateGeoParams",
    "replace-job": "replaceJob",
    "replace-sparse": "replaceSparse",
    "replace-dense": "replaceDense",
    "replace-mesh-result": "replaceMeshResult",
    "replace-trajectory": "replaceTrajectory",
    "replace-tracks": "replaceTracks",
    "replace-geo-products": "replaceGeoProducts",
    "replace-qc": "replaceQc",
    "commit-reconstruction": "commitReconstruction",
}

# 🚧 The one kind whose committed vector is a REFUSAL rather than an applied mutation, so it is
# answered by the refusal handlers below instead of by an entry in `APPLIERS`.
REFUSAL_KIND = "commit-reconstruction"

PARAMS_KEY = {
    "update-ingest-params": "ingest",
    "update-feature-params": "feature",
    "update-match-params": "matching",
    "update-sfm-params": "sfm",
    "update-dense-params": "dense",
    "update-mesh-params": "mesh",
    "update-motion-params": "motion",
    "update-geo-params": "geo",
}


def doc_json(ctx: Context):
    """📜️ The scenario's own doc string — the Python `Context` has no accessor of its own. It is the
    feature file's single statement of where this row's vector lives, so nothing here transcribes a
    fixture path that could drift away from the directory it names."""
    for step in ctx.scenario["steps"]:
        if step.get("docString"):
            return json.loads(step["docString"])
    raise AssertionError("scenario %s carries no doc string" % ctx.scenario["id"])


def _read_json(ctx: Context, uri: str):
    return json.loads(ctx.fixture_bytes(uri))


def spec_of(ctx: Context, kind: str):
    """🧫️ The `(before, mutation, after)` triple the scenario addresses, with its payload already
    unwrapped and checked to carry this kind's own discriminator."""
    spec = doc_json(ctx)
    assert spec["kind"] == kind, f"scenario {ctx.scenario['id']} carries a {spec['kind']!r} doc string"
    base = _read_json(ctx, spec["before"])
    actual_tag, payload = unwrap(_read_json(ctx, spec["mutation"]))
    assert actual_tag == TAGS[kind], f"unexpected wire tag {actual_tag!r} for scenario {ctx.scenario['id']}"
    return spec, base, payload, _read_json(ctx, spec["after"])


def unwrap(wire):
    """📨 The internally-tagged form every committed vector uses: `{"mutation": "<tag>", ...fields}`."""
    if isinstance(wire, dict) and isinstance(wire.get("mutation"), str):
        return wire["mutation"], {key: value for key, value in wire.items() if key != "mutation"}
    raise AssertionError("unrecognised mutation wire form: %s" % json.dumps(wire))
# endregion 🔖️Fixtures


# region 🔖️Addressing
def _find(items, item_id, key="id"):
    for index, item in enumerate(items):
        if item.get(key) == item_id:
            return index, item
    raise AssertionError(f"no member with {key}={item_id!r} among {[i.get(key) for i in items]!r}")
# endregion 🔖️Addressing


# region 🔖️Vocabulary — forward appliers
def apply_create_stream(doc, p):
    after = copy.deepcopy(doc)
    after["streams"].append(copy.deepcopy(p["stream"]))
    return after


def apply_delete_stream(doc, p):
    """🪓 Cascade-aware: also severs any GCP observation naming this stream."""
    after = copy.deepcopy(doc)
    idx, _ = _find(after["streams"], p["id"])
    after["streams"].pop(idx)
    for gcp in after["gcps"]:
        gcp["observations"] = [o for o in gcp["observations"] if o["streamId"] != p["id"]]
    return after


def apply_change_stream_sync(doc, p):
    after = copy.deepcopy(doc)
    _, s = _find(after["streams"], p["id"])
    s["syncOffsetMs"] = p["newSyncOffsetMs"]
    return after


def apply_add_stream_frame(doc, p):
    after = copy.deepcopy(doc)
    _, s = _find(after["streams"], p["id"])
    s["frames"].append(copy.deepcopy(p["frame"]))
    s["kind"] = p["kind"]
    return after


def apply_remove_stream_frame(doc, p):
    after = copy.deepcopy(doc)
    _, s = _find(after["streams"], p["id"])
    s["frames"].pop(p["frameIndex"])
    return after


def apply_replace_stream_source(doc, p):
    after = copy.deepcopy(doc)
    _, s = _find(after["streams"], p["id"])
    s["source"] = copy.deepcopy(p["source"])
    return after


def apply_create_asset(doc, p):
    """🔑 `childId` is an unspecified `DefaultHasher` digest — see module docstring. Set to `None`
    here; `mutate-create-asset`'s comparison adopts the committed value for that one field only,
    after asserting its shape, rather than fabricating an independent match."""
    after = copy.deepcopy(doc)
    after["assets"][p["key"]] = {
        "childId": None,
        "target": {"artifactId": f"{p['key']}-image", "dialect": {"artifactKind": "s.stdio.semio", "standard": "v1", "subset": "image"}},
    }
    return after


def apply_delete_asset(doc, p):
    after = copy.deepcopy(doc)
    del after["assets"][p["key"]]
    return after


def apply_restore_asset_handle(doc, p):
    """🔑 Inverse-only: restores a previously captured, ALREADY-COMPUTED handle verbatim — no hash
    is recomputed, because the captured value already carries a real, committed `childId`."""
    after = copy.deepcopy(doc)
    after["assets"][p["key"]] = copy.deepcopy(p["handle"])
    return after


def apply_create_camera_calibration(doc, p):
    after = copy.deepcopy(doc)
    after["calibration"]["cameras"].append(copy.deepcopy(p["camera"]))
    return after


def apply_update_camera_calibration(doc, p):
    after = copy.deepcopy(doc)
    idx, _ = _find(after["calibration"]["cameras"], p["camera"]["id"])
    after["calibration"]["cameras"][idx] = copy.deepcopy(p["camera"])
    return after


def apply_delete_camera_calibration(doc, p):
    after = copy.deepcopy(doc)
    idx, _ = _find(after["calibration"]["cameras"], p["cameraId"])
    after["calibration"]["cameras"].pop(idx)
    return after


def apply_create_rig_extrinsic(doc, p):
    after = copy.deepcopy(doc)
    after["calibration"]["rig"].append(copy.deepcopy(p["extrinsic"]))
    return after


def apply_delete_rig_extrinsic(doc, p):
    after = copy.deepcopy(doc)
    idx, _ = _find(after["calibration"]["rig"], p["cameraId"], key="cameraId")
    after["calibration"]["rig"].pop(idx)
    return after


def apply_update_rig_extrinsic(doc, p):
    after = copy.deepcopy(doc)
    idx, _ = _find(after["calibration"]["rig"], p["extrinsic"]["cameraId"], key="cameraId")
    after["calibration"]["rig"][idx] = copy.deepcopy(p["extrinsic"])
    return after


def apply_create_gcp(doc, p):
    after = copy.deepcopy(doc)
    after["gcps"].append(copy.deepcopy(p["gcp"]))
    return after


def apply_delete_gcp(doc, p):
    after = copy.deepcopy(doc)
    idx, _ = _find(after["gcps"], p["id"])
    after["gcps"].pop(idx)
    return after


def apply_add_gcp_observation(doc, p):
    after = copy.deepcopy(doc)
    _, gcp = _find(after["gcps"], p["id"])
    gcp["observations"].append(copy.deepcopy(p["observation"]))
    return after


def apply_remove_gcp_observation(doc, p):
    after = copy.deepcopy(doc)
    _, gcp = _find(after["gcps"], p["id"])
    gcp["observations"].pop(p["observationIndex"])
    return after


def _apply_update_params(doc, p, key):
    after = copy.deepcopy(doc)
    after["params"][key] = copy.deepcopy(p["params"])
    return after


def apply_replace_job(doc, p):
    after = copy.deepcopy(doc)
    after["job"] = copy.deepcopy(p["job"])
    return after


def apply_replace_sparse(doc, p):
    after = copy.deepcopy(doc)
    after["results"]["sparse"] = copy.deepcopy(p["sparse"])
    return after


def apply_replace_dense(doc, p):
    after = copy.deepcopy(doc)
    after["results"]["dense"] = copy.deepcopy(p["dense"])
    return after


def apply_replace_mesh_result(doc, p):
    after = copy.deepcopy(doc)
    after["results"]["mesh"] = copy.deepcopy(p["mesh"])
    return after


def apply_replace_trajectory(doc, p):
    after = copy.deepcopy(doc)
    after["results"]["trajectory"] = copy.deepcopy(p["trajectory"])
    return after


def apply_replace_tracks(doc, p):
    after = copy.deepcopy(doc)
    after["results"]["tracks"] = copy.deepcopy(p["tracks"])
    return after


def apply_replace_geo_products(doc, p):
    after = copy.deepcopy(doc)
    after["results"]["geo"] = copy.deepcopy(p["geo"])
    return after


def apply_replace_qc(doc, p):
    after = copy.deepcopy(doc)
    after["results"]["qc"] = copy.deepcopy(p["qc"])
    return after


APPLIERS = {
    "create-stream": apply_create_stream,
    "delete-stream": apply_delete_stream,
    "change-stream-sync": apply_change_stream_sync,
    "add-stream-frame": apply_add_stream_frame,
    "remove-stream-frame": apply_remove_stream_frame,
    "replace-stream-source": apply_replace_stream_source,
    "create-asset": apply_create_asset,
    "delete-asset": apply_delete_asset,
    "__restore-asset-handle": apply_restore_asset_handle,
    "create-camera-calibration": apply_create_camera_calibration,
    "update-camera-calibration": apply_update_camera_calibration,
    "delete-camera-calibration": apply_delete_camera_calibration,
    "create-rig-extrinsic": apply_create_rig_extrinsic,
    "delete-rig-extrinsic": apply_delete_rig_extrinsic,
    "update-rig-extrinsic": apply_update_rig_extrinsic,
    "create-gcp": apply_create_gcp,
    "delete-gcp": apply_delete_gcp,
    "add-gcp-observation": apply_add_gcp_observation,
    "remove-gcp-observation": apply_remove_gcp_observation,
    "replace-job": apply_replace_job,
    "replace-sparse": apply_replace_sparse,
    "replace-dense": apply_replace_dense,
    "replace-mesh-result": apply_replace_mesh_result,
    "replace-trajectory": apply_replace_trajectory,
    "replace-tracks": apply_replace_tracks,
    "replace-geo-products": apply_replace_geo_products,
    "replace-qc": apply_replace_qc,
}
for _kind, _key in PARAMS_KEY.items():
    APPLIERS[_kind] = (lambda key: lambda doc, p: _apply_update_params(doc, p, key))(_key)
# endregion 🔖️Vocabulary — forward appliers


# region 🔖️Vocabulary — inverse rule
def inverse_mutation(kind, base, payload):
    """↩️ Every inverse is computed from BASE, never from the payload. Returns a LIST of
    `(appliers_key, payload)` steps, applied in order — most kinds return exactly one; `delete-stream`
    returns one PER severed cascade member plus the primary restore, per `taxonomy.md` rule 5."""
    if kind == "create-stream":
        return [("delete-stream", {"id": payload["stream"]["id"]})]
    if kind == "delete-stream":
        _, stream = _find(base["streams"], payload["id"])
        steps = [("create-stream", {"stream": stream})]
        for gcp in base["gcps"]:
            for observation in gcp["observations"]:
                if observation["streamId"] == payload["id"]:
                    steps.append(("add-gcp-observation", {"id": gcp["id"], "observation": observation}))
        return steps
    if kind == "change-stream-sync":
        _, s = _find(base["streams"], payload["id"])
        return [("change-stream-sync", {"id": payload["id"], "newSyncOffsetMs": s["syncOffsetMs"]})]
    if kind == "add-stream-frame":
        _, s = _find(base["streams"], payload["id"])
        return [("remove-stream-frame", {"id": payload["id"], "frameIndex": len(s["frames"])})]
    if kind == "remove-stream-frame":
        _, s = _find(base["streams"], payload["id"])
        frame = s["frames"][payload["frameIndex"]]
        return [("add-stream-frame", {"id": payload["id"], "frame": frame, "kind": s["kind"]})]
    if kind == "replace-stream-source":
        _, s = _find(base["streams"], payload["id"])
        return [("replace-stream-source", {"id": payload["id"], "source": s.get("source")})]
    if kind == "create-asset":
        return [("delete-asset", {"key": payload["key"]})]
    if kind == "delete-asset":
        handle = base["assets"][payload["key"]]
        return [("__restore-asset-handle", {"key": payload["key"], "handle": handle})]
    if kind == "create-camera-calibration":
        return [("delete-camera-calibration", {"cameraId": payload["camera"]["id"]})]
    if kind == "update-camera-calibration":
        _, camera = _find(base["calibration"]["cameras"], payload["camera"]["id"])
        return [("update-camera-calibration", {"camera": camera})]
    if kind == "delete-camera-calibration":
        _, camera = _find(base["calibration"]["cameras"], payload["cameraId"])
        return [("create-camera-calibration", {"camera": camera})]
    if kind == "create-rig-extrinsic":
        return [("delete-rig-extrinsic", {"cameraId": payload["extrinsic"]["cameraId"]})]
    if kind == "delete-rig-extrinsic":
        _, extrinsic = _find(base["calibration"]["rig"], payload["cameraId"], key="cameraId")
        return [("create-rig-extrinsic", {"extrinsic": extrinsic})]
    if kind == "update-rig-extrinsic":
        _, extrinsic = _find(base["calibration"]["rig"], payload["extrinsic"]["cameraId"], key="cameraId")
        return [("update-rig-extrinsic", {"extrinsic": extrinsic})]
    if kind == "create-gcp":
        return [("delete-gcp", {"id": payload["gcp"]["id"]})]
    if kind == "delete-gcp":
        _, gcp = _find(base["gcps"], payload["id"])
        return [("create-gcp", {"gcp": gcp})]
    if kind == "add-gcp-observation":
        _, gcp = _find(base["gcps"], payload["id"])
        return [("remove-gcp-observation", {"id": payload["id"], "observationIndex": len(gcp["observations"])})]
    if kind == "remove-gcp-observation":
        _, gcp = _find(base["gcps"], payload["id"])
        observation = gcp["observations"][payload["observationIndex"]]
        return [("add-gcp-observation", {"id": payload["id"], "observation": observation})]
    if kind in PARAMS_KEY:
        key = PARAMS_KEY[kind]
        return [(kind, {"params": base["params"][key]})]
    if kind == "replace-job":
        return [("replace-job", {"job": base["job"]})]
    if kind == "replace-sparse":
        return [("replace-sparse", {"sparse": base["results"]["sparse"]})]
    if kind == "replace-dense":
        return [("replace-dense", {"dense": base["results"]["dense"]})]
    if kind == "replace-mesh-result":
        return [("replace-mesh-result", {"mesh": base["results"]["mesh"]})]
    if kind == "replace-trajectory":
        return [("replace-trajectory", {"trajectory": base["results"].get("trajectory")})]
    if kind == "replace-tracks":
        return [("replace-tracks", {"tracks": base["results"]["tracks"]})]
    if kind == "replace-geo-products":
        return [("replace-geo-products", {"geo": base["results"]["geo"]})]
    if kind == "replace-qc":
        return [("replace-qc", {"qc": base["results"]["qc"]})]
    raise AssertionError(f"no inverse rule for kind {kind!r}")
# endregion 🔖️Vocabulary — inverse rule


# region 🔖️Oracle
def _mutate_for(kind):
    def handler(ctx: Context) -> Outcome:
        _spec, base, payload, expected_after = spec_of(ctx, kind)
        after = APPLIERS[kind](base, payload)
        if kind == "create-asset":
            key = payload["key"]
            expected_child = expected_after["assets"][key]["childId"]
            assert isinstance(expected_child, str) and expected_child.startswith("remodeling-asset-"), f"create-asset: committed childId {expected_child!r} does not look like a minted content-address handle"
            after["assets"][key]["childId"] = expected_child
        assert after == expected_after, f"mutate-{kind}: {after} != committed after-document {expected_after}"
        raw = json.dumps(after, sort_keys=True, separators=(",", ":")).encode("utf-8")
        return Outcome(projection=after, raw=raw)

    return handler


def _inverse_for(kind):
    def handler(ctx: Context) -> Outcome:
        _spec, base, payload, _expected_after = spec_of(ctx, kind)
        current = APPLIERS[kind](base, payload)
        assert current != base, f"inverse-{kind}: the forward mutation left the document untouched, so restoring it proves nothing"
        for step_kind, step_payload in inverse_mutation(kind, base, payload):
            current = APPLIERS[step_kind](current, step_payload)
        assert current == base, f"inverse-{kind}: {current} != committed before-document {base}"
        raw = json.dumps(current, sort_keys=True, separators=(",", ":")).encode("utf-8")
        return Outcome(projection=current, raw=raw)

    return handler


def refuses_commit(payload) -> bool:
    """🚧 `commit-reconstruction` PUBLISHES what a staged reconstruction produced, so every result
    argument it accepts must be a replayable staging handle. This vector's `sparse` argument is a
    plain point buffer — a `points` blob with no handle beside it — which names no staged run and
    therefore cannot be published. Derived from the verb's own meaning, not from production's code."""
    sparse = payload.get("sparse")
    return isinstance(sparse, dict) and "points" in sparse and "handle" not in sparse


def _refusal_mutate(kind):
    def handler(ctx: Context) -> Outcome:
        spec, base, payload, expected_after = spec_of(ctx, kind)
        assert refuses_commit(payload), f"mutate-{kind}: this reference accepts the vector, yet the feature declares it refused as {spec['code']!r}"
        assert expected_after == base, f"mutate-{kind}: a refused commit must leave the scene untouched, but the committed after-document differs from the before-document"
        raw = json.dumps(base, sort_keys=True, separators=(",", ":")).encode("utf-8")
        return Outcome(projection=base, raw=raw)

    return handler


def _refusal_inverse(kind):
    def handler(ctx: Context) -> Outcome:
        _spec, base, payload, _expected_after = spec_of(ctx, kind)
        assert refuses_commit(payload), f"inverse-{kind}: this reference accepts the vector, so undoing it would not be the identity the feature describes"
        raw = json.dumps(base, sort_keys=True, separators=(",", ":")).encode("utf-8")
        return Outcome(projection=base, raw=raw)

    return handler


def identity_handler(ctx: Context) -> Outcome:
    """🔁️ The reference cannot PARSE `.dsl.semio` — this subset's committed text grammar is the
    repository-wide placeholder whose whole body is `payload = OCTET+`, reported by the oracle
    registry's own `remodeling-mutation-semantics` entry. What it can state from the committed bytes
    alone is what a faithful parse-and-reprint must produce: those same bytes. It requires in role
    that the carrier really is this artifact's DSL and not an empty or JSON file, so a handler that
    answered with whatever it was handed would be caught here."""
    carrier = doc_json(ctx)["carrier"]
    committed = ctx.fixture_bytes(carrier)
    text = committed.decode("utf-8")
    assert text.strip(), "identity-round-trip: the committed example is empty"
    assert not text.lstrip().startswith(("{", "[")), "identity-round-trip: the committed example is JSON, not the DSL carrier this scenario names"
    return Outcome(projection=text, raw=committed)
# endregion 🔖️Oracle


# region 🔖️Registration
def adapter() -> Adapter:
    """🧭️ Registration is by full expanded scenario id, mirroring the feature's `Examples` tables, in
    the ORACLE role only — registering these handlers as subjects too would make the reference its own
    subject and manufacture a green self-comparison. Every planned scenario is answered: the runner
    plans a role over ALL of them and errors on a gap, so a kind left out would report as an
    unregistered scenario rather than as the honest scope limit it was meant to be."""
    built = Adapter("python")
    for kind in TAGS:
        if kind == REFUSAL_KIND:
            built = built.oracle(f"mutate-{kind}", _refusal_mutate(kind)).oracle(f"inverse-{kind}", _refusal_inverse(kind))
        else:
            built = built.oracle(f"mutate-{kind}", _mutate_for(kind)).oracle(f"inverse-{kind}", _inverse_for(kind))
    return built.oracle("identity-round-trip", identity_handler)
# endregion 🔖️Registration

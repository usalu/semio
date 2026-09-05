"""🐍️ `s.remodeling.remodeling`'s second, independent implementation of 34 of its 35 mutation kinds.

`s.remodeling.remodeling` is a semio-NATIVE reconstruction JOB document — streams, calibrations,
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

🚧 `commit-reconstruction` — the 35th kind — is DELIBERATELY NOT covered here, for the same reason
this feature's own Rust adapter treats it specially: its diff reads process-global staging state
(`commit_staged_remodeling_reconstruction`) a static `(before, mutation, after)` fixture cannot carry,
and its one committed vector is a REFUSAL, not an applied mutation. It stays exactly where it already
was — asserted by the Rust subject alone, in its own `@mode-error`/unconverted `@mode-property`
scenario outlines — and is not claimed by this reference.

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
_ROOT = "asset://🧬️schema/🧬️mutations"

# 🗺️ kind -> (dir, fixture, wire tag). `commit-reconstruction` is intentionally absent.
VECTORS = {
    "create-stream": ("🌱create-stream", "🎥️adds-stream-c-bound-to-cam-b", "createStream"),
    "delete-stream": ("🪓delete-stream", "🚫️removes-stream-b-and-cascades-its-gcp-observation", "deleteStream"),
    "change-stream-sync": ("⏱️change-stream-sync", "⏱️shifts-stream-a-sync-offset-to-minus-seven-and-a-half", "changeStreamSync"),
    "add-stream-frame": ("➕add-stream-frame", "🎞️appends-a-third-frame-to-stream-a", "addStreamFrame"),
    "remove-stream-frame": ("➖remove-stream-frame", "🚫️removes-the-last-frame-of-stream-a", "removeStreamFrame"),
    "replace-stream-source": ("🔁replace-stream-source", "🧹️clears-the-video-source-of-stream-a", "replaceStreamSource"),
    "create-asset": ("🧷create-asset", "🖼️stores-a-new-jpeg-frame-asset", "createAsset"),
    "delete-asset": ("🗞️delete-asset", "🗑️removes-asset-a-and-reports-its-stale-references", "deleteAsset"),
    "create-camera-calibration": ("🔭create-camera-calibration", "📷️adds-the-cam-c-fisheye-calibration", "createCameraCalibration"),
    "update-camera-calibration": ("🛠️update-camera-calibration", "🔍️refines-the-cam-a-focal-length-and-rms", "updateCameraCalibration"),
    "delete-camera-calibration": ("🚫delete-camera-calibration", "🚫️removes-the-cam-b-calibration", "deleteCameraCalibration"),
    "create-rig-extrinsic": ("⛓️create-rig-extrinsic", "🔗️adds-a-rig-extrinsic-for-cam-b", "createRigExtrinsic"),
    "delete-rig-extrinsic": ("✂️delete-rig-extrinsic", "✂️drops-the-cam-a-rig-extrinsic", "deleteRigExtrinsic"),
    "update-rig-extrinsic": ("🔩update-rig-extrinsic", "📍️retunes-the-cam-a-rig-translation", "updateRigExtrinsic"),
    "create-gcp": ("🧿create-gcp", "📍️adds-gcp-tower-with-one-observation", "createGcp"),
    "delete-gcp": ("🚮delete-gcp", "🚫️removes-gcp-corner-and-cascades-its-observation", "deleteGcp"),
    "add-gcp-observation": ("🔎add-gcp-observation", "🔎️adds-the-first-observation-to-gcp-ridge", "addGcpObservation"),
    "remove-gcp-observation": ("🚷remove-gcp-observation", "🚫️removes-the-only-observation-of-gcp-corner", "removeGcpObservation"),
    "update-ingest-params": ("🥣update-ingest-params", "🔍️tightens-the-ingest-sharpness-gate", "updateIngestParams"),
    "update-feature-params": ("🌠update-feature-params", "🔎️switches-the-detector-to-akaze", "updateFeatureParams"),
    "update-match-params": ("🪢update-match-params", "🌳️switches-the-matcher-to-a-kd-tree", "updateMatchParams"),
    "update-sfm-params": ("🧮update-sfm-params", "🎯️switches-the-robust-loss-to-cauchy", "updateSfmParams"),
    "update-dense-params": ("🌁update-dense-params", "🔬️raises-the-dense-resolution-and-confidence-gate", "updateDenseParams"),
    "update-mesh-params": ("🕸️update-mesh-params", "🔳️doubles-the-texture-size-and-drops-the-watertight-guarantee", "updateMeshParams"),
    "update-motion-params": ("🏎️update-motion-params", "🏃️enables-motion-tracking", "updateMotionParams"),
    "update-geo-params": ("🌐update-geo-params", "🌐️enables-georeferencing-with-an-origin", "updateGeoParams"),
    "replace-job": ("🏗️replace-job", "🎨️advances-the-job-to-texturing", "replaceJob"),
    "replace-sparse": ("⭐replace-sparse", "✨️swaps-in-an-uncolored-four-point-sparse-cloud", "replaceSparse"),
    "replace-dense": ("☁️replace-dense", "☁️swaps-in-a-two-point-classified-dense-cloud", "replaceDense"),
    "replace-mesh-result": ("🧱replace-mesh-result", "🕸️swaps-in-an-imported-untextured-mesh", "replaceMeshResult"),
    "replace-trajectory": ("🛣️replace-trajectory", "🧹️clears-the-camera-trajectory", "replaceTrajectory"),
    "replace-tracks": ("🚂replace-tracks", "⏸️replaces-the-moving-track-with-two-static-tracks", "replaceTracks"),
    "replace-geo-products": ("🗾replace-geo-products", "🗺️adds-the-dtm-and-ortho-rasters", "replaceGeoProducts"),
    "replace-qc": ("🧾replace-qc", "📋️records-a-qc-report-carrying-a-watertight-summary", "replaceQc"),
}

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


def _leaf_root(kind: str) -> str:
    dirname, fixture, _tag = VECTORS[kind]
    return f"{_ROOT}/{dirname}/🧪️tests/{fixture}"


def _read_json(ctx: Context, uri: str):
    return json.loads(ctx.fixture_bytes(uri))


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
        root = _leaf_root(kind)
        _dir, _fixture, wire_tag = VECTORS[kind]
        base = _read_json(ctx, f"{root}/📸️snapshot/⬅️before/🔣️.json")
        actual_tag, payload = unwrap(_read_json(ctx, f"{root}/🦠️mutation/🔣️.json"))
        assert actual_tag == wire_tag, f"unexpected wire tag {actual_tag!r} for scenario mutate-{kind}"
        after = APPLIERS[kind](base, payload)
        expected_after = _read_json(ctx, f"{root}/📸️snapshot/➡️after/🔣️.json")
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
        root = _leaf_root(kind)
        _dir, _fixture, wire_tag = VECTORS[kind]
        base = _read_json(ctx, f"{root}/📸️snapshot/⬅️before/🔣️.json")
        actual_tag, payload = unwrap(_read_json(ctx, f"{root}/🦠️mutation/🔣️.json"))
        assert actual_tag == wire_tag, f"unexpected wire tag {actual_tag!r} for scenario inverse-{kind}"
        current = APPLIERS[kind](base, payload)
        assert current != base, f"inverse-{kind}: the forward mutation left the document untouched, so restoring it proves nothing"
        for step_kind, step_payload in inverse_mutation(kind, base, payload):
            current = APPLIERS[step_kind](current, step_payload)
        assert current == base, f"inverse-{kind}: {current} != committed before-document {base}"
        raw = json.dumps(current, sort_keys=True, separators=(",", ":")).encode("utf-8")
        return Outcome(projection=current, raw=raw)

    return handler
# endregion 🔖️Oracle


# region 🔖️Registration
def adapter() -> Adapter:
    """🧭️ Registration is by full expanded scenario id, mirroring the feature's `Examples` tables.
    Oracle role only, and only for the 34 kinds `VECTORS` declares — `commit-reconstruction` and
    `identity-round-trip` both stay subject-only, for the reasons the module docstring and the
    feature's own prose state."""
    built = Adapter("python")
    for kind in VECTORS:
        built = built.oracle(f"mutate-{kind}", _mutate_for(kind)).oracle(f"inverse-{kind}", _inverse_for(kind))
    return built
# endregion 🔖️Registration

"""🐍️ `s.remodel.remodeling`'s second, independent implementation of all 35 of its mutation kinds.

`s.remodel.remodeling` is a semio-NATIVE reconstruction JOB document — streams, calibrations,
ground control points, the eight `ReconstructionParams` sub-records a pipeline runs under, and the
engine-owned results — not a point cloud or a mesh file. A reader of COLMAP, LAS or PLY output would
be judging a different artifact, and nothing reads `.dsl.semio`. This subset's own no-oracle decision
(`remodeling-mutation-semantics`) records that survey. The reference is therefore a second
IMPLEMENTATION, written from this subset's own committed
`../../🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/🔣️.json` document shape, each kind's own committed
`(before, mutation, after)` leaf fixture, and from
`.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-DIRECT-LEAF-OVERHAUL/📓️taxonomy.md`'s
verb table (rule 5 in particular: an inverse is computed from BASE and must restore a member's
POSITION, not merely its presence). It imports nothing from the Rust it judges and transliterates
none of it.

🚦 Two answers, not one. A vector whose scenario doc string carries a `code` member declares that
this payload is REFUSED (or warned as a no-op) and that the document therefore does not move; the
reference then states, from the verb's own meaning, WHICH diagnostic that payload earns and requires
the committed after-document to be the before-document unchanged. Every other vector is applied and
compared field by field. The refusal statements below are derived from what each verb MEANS — a
stream may not be created twice, a frame index must address a frame that exists, a commit publishes
only what a staged run produced — not from production's control flow, and they carry none of
production's diagnostic prose.

📍️ WHERE A VECTOR LIVES IS THE FEATURE'S ANSWER, not this module's. Every scenario carries a doc
string naming its `(before, mutation, after)` URIs, resolved through the plan at RUN time. Nothing
here transcribes a fixture path: the 2026-09-05 repo-wide path-shortening pass renamed every case
directory under this subset, and a hard-coded table of those names is exactly the drift that broke
this case before.

🔑 `create-asset` — the ONE genuine content-address hazard in this vocabulary. Production mints a
NEW `assets.<key>.childId` via `image_asset_child_handle`, which hashes the raw `ImageAsset` bytes
through `std::collections::hash_map::DefaultHasher` — an algorithm the Rust standard library
EXPLICITLY documents as unspecified and not portable, even across compiler versions. This reference
therefore compares every field EXCEPT that one opaque digest exactly, and only checks the digest's
SHAPE (`remodeling-asset-` followed by hex) — an honest, narrow scope limit, not a silent pass. It
does state the DURABLE leaf that digest keys independently and in full: the bounded 4 KiB chunking of
the payload's own base64 content, its mime and its declared raster size are all recomputed here, and
only the key they are filed under is adopted. `delete-asset`'s inverse sidesteps the hazard entirely:
`assets[key]` in the committed BEFORE-document is already a fully-formed captured handle.

⛓️ OWNERSHIP AND ORDER, the two rules every verb here is read against. A record may be removed with
what it OWNS (a stream carries its frames, a GCP carries its observations) but never with what merely
NAMES it: a stream a GCP observation addresses, a camera a stream or a rig entry binds, an asset a
frame, texture or geo product references, all earn `mutation.referenced` instead. And every keyed
collection is held in ascending key order, so a `create`/`add` puts a member back exactly where a
`delete`/`remove` took it from. Together those make every kind's inverse a single step of this same
vocabulary that restores the committed BEFORE-document exactly — this module needs no synthetic
"move" step to state it, and it asserts that for all 134 committed vectors.
"""

from __future__ import annotations

# region 🔖️Imports
import base64
import copy
import json

from semio_repo_test import Adapter, Context, Outcome

# endregion 🔖️Imports


# region 🔖️Vocabulary
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

RESULT_SLOT = {
    "replace-sparse": "sparse",
    "replace-dense": "dense",
    "replace-mesh-result": "mesh",
    "replace-trajectory": "trajectory",
    "replace-tracks": "tracks",
    "replace-geo-products": "geo",
    "replace-qc": "qc",
}

# 🔑 The argument key each `replace-*` verb carries its replacement under.
RESULT_ARG = dict(RESULT_SLOT, **{"replace-mesh-result": "mesh"})

# 🚧 A private staging handle, which only `commit-reconstruction` may publish.
CONTENT_HANDLE_PREFIX = "remodeling-content:"
MESH_STAGE_HANDLE_PREFIX = "mesh-stage:"
DURABLE_CHUNK_RAW_BYTES = 4096
RASTER_CONTENT_BYTES = 1_114_112
# endregion 🔖️Vocabulary


# region 🔖️Fixtures
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


def spec_of(ctx: Context):
    """🧫️ The `(before, mutation, after)` triple the scenario addresses, with its payload already
    unwrapped and checked to carry the kind the doc string names."""
    spec = doc_json(ctx)
    kind = spec["kind"]
    assert kind in TAGS, f"scenario {ctx.scenario['id']} names {kind!r}, which is not a declared kind"
    base = _read_json(ctx, spec["before"])
    actual_tag, payload = unwrap(_read_json(ctx, spec["mutation"]))
    assert actual_tag == TAGS[kind], f"unexpected wire tag {actual_tag!r} for scenario {ctx.scenario['id']}"
    return kind, spec, base, payload, _read_json(ctx, spec["after"])


def unwrap(wire):
    """📨 The internally-tagged form every committed vector uses: `{"mutation": "<tag>", ...fields}`."""
    if isinstance(wire, dict) and isinstance(wire.get("mutation"), str):
        return wire["mutation"], {key: value for key, value in wire.items() if key != "mutation"}
    raise AssertionError("unrecognised mutation wire form: %s" % json.dumps(wire))
# endregion 🔖️Fixtures


# region 🔖️Addressing
def _index_of(items, item_id, key="id"):
    for index, item in enumerate(items):
        if item.get(key) == item_id:
            return index
    return -1


def _find(items, item_id, key="id"):
    index = _index_of(items, item_id, key)
    assert index >= 0, f"no member with {key}={item_id!r} among {[i.get(key) for i in items]!r}"
    return index, items[index]
def ordered_index(items, key_value, key_of) -> int:
    """🔢️ Where a member with this key belongs in a collection held in ascending key order. Every
    keyed collection in this document is ordered — ids for streams, cameras, rig entries and ground
    control points, `(index, assetId)` for a stream's frames, `(streamId, frameIndex)` for a GCP's
    observations — which is what lets a `create`/`add` put a member back exactly where a `delete`/
    `remove` took it from."""
    position = 0
    while position < len(items) and key_of(items[position]) < key_value:
        position += 1
    return position
# endregion 🔖️Addressing


# region 🔖️Vocabulary — forward appliers
def apply_create_stream(doc, p):
    after = copy.deepcopy(doc)
    after["streams"].insert(ordered_index(after["streams"], p["stream"]["id"], lambda stream: stream["id"]), copy.deepcopy(p["stream"]))
    return after


def apply_delete_stream(doc, p):
    """🪓 Removes the stream and the frames it OWNS. GCP observations belong to their GCPs, so a stream
    any observation still names is refused rather than severed (see `refusal_of`)."""
    after = copy.deepcopy(doc)
    idx, _ = _find(after["streams"], p["id"])
    after["streams"].pop(idx)
    return after


def apply_change_stream_sync(doc, p):
    after = copy.deepcopy(doc)
    _, s = _find(after["streams"], p["id"])
    s["syncOffsetMs"] = p["newSyncOffsetMs"]
    return after


def apply_add_stream_frame(doc, p):
    """➕ `kind` ASSERTS the owner stream's media kind (see `refusal_of`); it is never written."""
    after = copy.deepcopy(doc)
    _, s = _find(after["streams"], p["id"])
    key = (p["frame"]["index"], p["frame"]["assetId"])
    s["frames"].insert(ordered_index(s["frames"], key, lambda frame: (frame["index"], frame["assetId"])), copy.deepcopy(p["frame"]))
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


def durable_leaf(asset):
    """🧩️ The document-owned durable leaf one bounded raster becomes: its own base64 content cut into
    4 KiB raw chunks, beside the mime and raster size the payload declares. Recomputed here in full —
    only the content-address it is FILED under is adopted from the committed vector, because that key
    is an unspecified `DefaultHasher` digest (see the module docstring)."""
    raw = base64.b64decode(asset["data"], validate=True)
    assert len(raw) <= RASTER_CONTENT_BYTES, "a durable raster leaf is bounded"
    chunks = [base64.b64encode(raw[offset : offset + DURABLE_CHUNK_RAW_BYTES]).decode("ascii") for offset in range(0, len(raw), DURABLE_CHUNK_RAW_BYTES)]
    return {"kind": "image", "mime": asset["mime"], "width": asset["width"], "height": asset["height"], "chunks": chunks}


def apply_create_asset(doc, p):
    """🔑 `childId` is an unspecified `DefaultHasher` digest — see the module docstring. It is left
    `None` here and filled in by the comparison from the committed vector, after its shape is checked;
    the durable leaf it keys is computed in full."""
    after = copy.deepcopy(doc)
    after["assets"][p["key"]] = {
        "childId": None,
        "target": {"artifactId": f"{p['key']}-image", "dialect": {"artifactKind": "s.stdio.semio", "standard": "v1", "subset": "image"}},
    }
    return after


def apply_delete_asset(doc, p):
    """🗞️ The durable leaf the handle owned goes with the entry — an asset key is the only address the
    document has for that leaf, so keeping it would leave content nothing can ever reach again."""
    after = copy.deepcopy(doc)
    child_id = after["assets"][p["key"]]["childId"]
    del after["assets"][p["key"]]
    after["durableArtifacts"] = {key: value for key, value in after["durableArtifacts"].items() if key != child_id}
    return after


def _mint_asset(base, after, key, child_id, asset):
    """🔑 What minting one asset does to BOTH lanes: the entry takes the content-address handle, the
    durable store gains that address' leaf, and an overwritten entry's own leaf goes — an asset key is
    the only address the document has for a leaf, so the store never keeps one nothing can reach."""
    after = copy.deepcopy(after)
    after["assets"][key]["childId"] = child_id
    dropped = base["assets"].get(key, {}).get("childId")
    durable = {name: value for name, value in after["durableArtifacts"].items() if name != dropped}
    durable[child_id] = durable_leaf(asset)
    after["durableArtifacts"] = {name: durable[name] for name in sorted(durable)}
    return after


def apply_restore_asset_handle(doc, p):
    """🔑 Inverse-only: restores a previously captured, ALREADY-COMPUTED handle and the durable leaf it
    keys, verbatim — no hash is recomputed, because the captured value already carries a real,
    committed `childId`. This is `create-asset` narrowed to the one thing this reference cannot state
    independently (the `DefaultHasher` digest); every other lane it moves is stated in full."""
    after = copy.deepcopy(doc)
    dropped = after["assets"].get(p["key"], {}).get("childId")
    after["assets"][p["key"]] = copy.deepcopy(p["handle"])
    after["assets"] = {key: after["assets"][key] for key in sorted(after["assets"])}
    durable = {key: value for key, value in after["durableArtifacts"].items() if key != dropped}
    durable[p["handle"]["childId"]] = copy.deepcopy(p["leaf"])
    after["durableArtifacts"] = {key: durable[key] for key in sorted(durable)}
    return after


def apply_create_camera_calibration(doc, p):
    after = copy.deepcopy(doc)
    cameras = after["calibration"]["cameras"]
    cameras.insert(ordered_index(cameras, p["camera"]["id"], lambda camera: camera["id"]), copy.deepcopy(p["camera"]))
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
    rig = after["calibration"]["rig"]
    rig.insert(ordered_index(rig, p["extrinsic"]["cameraId"], lambda entry: entry["cameraId"]), copy.deepcopy(p["extrinsic"]))
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
    after["gcps"].insert(ordered_index(after["gcps"], p["gcp"]["id"], lambda gcp: gcp["id"]), copy.deepcopy(p["gcp"]))
    return after


def apply_delete_gcp(doc, p):
    after = copy.deepcopy(doc)
    idx, _ = _find(after["gcps"], p["id"])
    after["gcps"].pop(idx)
    return after


def apply_add_gcp_observation(doc, p):
    after = copy.deepcopy(doc)
    _, gcp = _find(after["gcps"], p["id"])
    key = (p["observation"]["streamId"], p["observation"]["frameIndex"])
    gcp["observations"].insert(ordered_index(gcp["observations"], key, lambda observation: (observation["streamId"], observation["frameIndex"])), copy.deepcopy(p["observation"]))
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


def _apply_replace_result(doc, p, slot, arg):
    after = copy.deepcopy(doc)
    after["results"][slot] = copy.deepcopy(p[arg])
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
}
for _kind, _key in PARAMS_KEY.items():
    APPLIERS[_kind] = (lambda key: lambda doc, p: _apply_update_params(doc, p, key))(_key)
for _kind, _slot in RESULT_SLOT.items():
    APPLIERS[_kind] = (lambda slot, arg: lambda doc, p: _apply_replace_result(doc, p, slot, arg))(_slot, RESULT_ARG[_kind])
# endregion 🔖️Vocabulary — forward appliers


# region 🔖️Vocabulary — refusal rules
def _finite(value) -> bool:
    return isinstance(value, (int, float)) and not isinstance(value, bool) and value == value and abs(value) != float("inf")


def refusal_of(kind, base, payload):
    """🚦 The diagnostic this payload earns against this document, or `None` when the verb accepts it.

    Every line below is a statement about what the VERB means, not about how production is written:
    a record may not be created under an id the document already holds (`mutation.duplicate-id`); a
    verb may only address a member that exists, index included (`mutation.target-missing`); a value a
    verb constrains must satisfy that constraint (`mutation.invariant`); resubmitting the value a
    document already holds changes nothing (`mutation.no-op`); a private staging handle belongs to the
    commit that produced it and to nothing else (`mutation.incomplete-mesh`,
    `mutation.invalid-asset-payload`, `mutation.invalid-reconstruction-*`)."""
    streams, gcps = base["streams"], base["gcps"]
    cameras, rig = base["calibration"]["cameras"], base["calibration"]["rig"]
    params, results = base["params"], base["results"]

    if kind == "create-stream":
        if _index_of(streams, payload["stream"]["id"]) >= 0:
            return "mutation.duplicate-id"
        camera_id = payload["stream"].get("cameraId")
        if camera_id is not None and _index_of(cameras, camera_id) < 0:
            return "mutation.invariant"
        return None
    if kind == "replace-stream-source":
        return None if _index_of(streams, payload["id"]) >= 0 else "mutation.target-missing"
    if kind == "delete-stream":
        if _index_of(streams, payload["id"]) < 0:
            return "mutation.target-missing"
        # 🔗 A GCP owns its observations; a stream one of them names may not be taken out from under it.
        return "mutation.referenced" if any(o["streamId"] == payload["id"] for gcp in gcps for o in gcp["observations"]) else None
    if kind == "change-stream-sync":
        index = _index_of(streams, payload["id"])
        if index < 0:
            return "mutation.target-missing"
        if not _finite(payload["newSyncOffsetMs"]):
            return "mutation.invariant"
        return "mutation.no-op" if streams[index]["syncOffsetMs"] == payload["newSyncOffsetMs"] else None
    if kind == "add-stream-frame":
        index = _index_of(streams, payload["id"])
        if index < 0:
            return "mutation.target-missing"
        # 🗂️ A stream's media kind is its provenance, fixed when it was created; a frame declaring a
        # different one is describing a different stream.
        if payload["kind"] != streams[index]["kind"]:
            return "mutation.invariant"
        return "mutation.no-op" if payload["frame"] in streams[index]["frames"] else None
    if kind == "remove-stream-frame":
        index = _index_of(streams, payload["id"])
        if index < 0 or payload["frameIndex"] >= len(streams[index]["frames"]):
            return "mutation.target-missing"
        return None
    if kind == "create-asset":
        return "mutation.invalid-asset-payload" if payload["asset"]["data"].startswith(CONTENT_HANDLE_PREFIX) else None
    if kind == "delete-asset":
        if payload["key"] not in base["assets"]:
            return "mutation.target-missing"
        referenced = any(frame["assetId"] == payload["key"] for stream in streams for frame in stream["frames"])
        referenced = referenced or results["mesh"]["textureAssetId"] == payload["key"]
        if results["geo"] is not None:
            referenced = referenced or any(results["geo"][slot] == payload["key"] for slot in ("dsmAssetId", "dtmAssetId", "orthoAssetId"))
        # 🔗 Removing content the document still names would leave a reference pointing at nothing.
        return "mutation.referenced" if referenced else None
    if kind == "create-camera-calibration":
        return "mutation.duplicate-id" if _index_of(cameras, payload["camera"]["id"]) >= 0 else None
    if kind == "update-camera-calibration":
        index = _index_of(cameras, payload["camera"]["id"])
        if index < 0:
            return "mutation.target-missing"
        camera = payload["camera"]
        lanes = [camera["fx"], camera["fy"], camera["cx"], camera["cy"], camera["skew"], *camera["distortion"]]
        if camera["rmsReprojectionPx"] is not None:
            lanes.append(camera["rmsReprojectionPx"])
        if not all(_finite(value) for value in lanes):
            return "mutation.invariant"
        return "mutation.no-op" if cameras[index] == camera else None
    if kind == "delete-camera-calibration":
        if _index_of(cameras, payload["cameraId"]) < 0:
            return "mutation.target-missing"
        referenced = any(stream["cameraId"] == payload["cameraId"] for stream in streams) or _index_of(rig, payload["cameraId"], key="cameraId") >= 0
        # 🔗 A calibration owns nothing; it may not leave while a stream binding or a rig entry needs it.
        return "mutation.referenced" if referenced else None
    if kind == "create-rig-extrinsic":
        camera_id = payload["extrinsic"]["cameraId"]
        if _index_of(rig, camera_id, key="cameraId") >= 0:
            return "mutation.duplicate-id"
        return None if _index_of(cameras, camera_id) >= 0 else "mutation.invariant"
    if kind == "delete-rig-extrinsic":
        return None if _index_of(rig, payload["cameraId"], key="cameraId") >= 0 else "mutation.target-missing"
    if kind == "update-rig-extrinsic":
        extrinsic = payload["extrinsic"]
        index = _index_of(rig, extrinsic["cameraId"], key="cameraId")
        if index < 0:
            return "mutation.target-missing"
        if not all(_finite(value) for value in extrinsic["rotationWxyz"] + extrinsic["translationM"]):
            return "mutation.invariant"
        return "mutation.no-op" if rig[index] == extrinsic else None
    if kind == "create-gcp":
        return "mutation.duplicate-id" if _index_of(gcps, payload["gcp"]["id"]) >= 0 else None
    if kind == "delete-gcp":
        return None if _index_of(gcps, payload["id"]) >= 0 else "mutation.target-missing"
    if kind == "add-gcp-observation":
        index = _index_of(gcps, payload["id"])
        if index < 0:
            return "mutation.target-missing"
        if _index_of(streams, payload["observation"]["streamId"]) < 0:
            return "mutation.invariant"
        return "mutation.no-op" if payload["observation"] in gcps[index]["observations"] else None
    if kind == "remove-gcp-observation":
        index = _index_of(gcps, payload["id"])
        if index < 0 or payload["observationIndex"] >= len(gcps[index]["observations"]):
            return "mutation.target-missing"
        return None
    if kind in PARAMS_KEY:
        slot, incoming = PARAMS_KEY[kind], payload["params"]
        invalid = {
            "ingest": lambda p: not _finite(p["minSharpness"]) or p["minSharpness"] < 0.0 or p["maxFrames"] == 0 or p["frameSampleStride"] == 0,
            "feature": lambda p: p["targetCount"] == 0 or not _finite(p["edgeThreshold"]) or p["edgeThreshold"] < 0.0,
            "matching": lambda p: not _finite(p["ratioTest"]) or not 0.0 < p["ratioTest"] <= 1.0,
            "geo": lambda p: p["orthoMaxPx"] == 0
            or not all(_finite(v) and v > 0.0 for v in (p["gsdM"], p["dsmCellM"], p["dtmFilterRadiusM"]))
            or (p["originLat"] is not None and not (_finite(p["originLat"]) and -90.0 <= p["originLat"] <= 90.0))
            or (p["originLon"] is not None and not (_finite(p["originLon"]) and -180.0 <= p["originLon"] <= 180.0)),
            "sfm": lambda p: not _finite(p["ransacThresholdPx"]) or not _finite(p["huberDeltaPx"]),
            "dense": lambda p: not _finite(p["confidenceThreshold"]),
            "mesh": lambda p: not _finite(p["tsdfVoxelSizeMm"]) or not _finite(p["tsdfTruncationMm"]),
            "motion": lambda p: not _finite(p["minTrackQuality"]),
        }[slot]
        # 🚦 One guard order for all eight blocks, and for the whole vocabulary: an argument that
        # breaks the constraint the verb exists to uphold is a fault whether or not it happens to equal
        # what the document already holds, so the invariant always answers before "nothing changed".
        if invalid(incoming):
            return "mutation.invariant"
        return "mutation.no-op" if incoming == params[slot] else None
    if kind == "replace-job":
        return "mutation.no-op" if payload["job"] == base["job"] else None
    if kind == "replace-mesh-result":
        if payload["mesh"]["mesh"]["target"]["artifactId"].startswith(MESH_STAGE_HANDLE_PREFIX):
            return "mutation.incomplete-mesh"
        return "mutation.no-op" if payload["mesh"] == results["mesh"] else None
    if kind in RESULT_SLOT:
        slot, arg = RESULT_SLOT[kind], RESULT_ARG[kind]
        clearable = slot in ("trajectory", "geo", "qc")
        if clearable and payload[arg] is None and results[slot] is None:
            return "mutation.target-missing"
        return "mutation.no-op" if payload[arg] == results[slot] else None
    if kind == "commit-reconstruction":
        return refuses_commit(payload)
    raise AssertionError(f"no refusal rule for kind {kind!r}")


def refuses_commit(payload):
    """🚧 `commit-reconstruction` PUBLISHES what a staged reconstruction produced, so every result
    argument it accepts must be a replayable staging handle — a `remodeling-content:` address for a
    point buffer or a raster, a `mesh-stage:` artifact id for a mesh. An inline buffer names no staged
    run and cannot be published. Derived from the verb's own meaning; the order below is the order the
    three arguments are published in, sparse cloud first and mesh last."""
    sparse = payload.get("sparse")
    if isinstance(sparse, dict) and sparse.get("points") and not sparse["points"].startswith(CONTENT_HANDLE_PREFIX):
        return "mutation.invalid-reconstruction-sparse"
    for committed in payload.get("assets", []):
        if not committed["asset"]["data"].startswith(CONTENT_HANDLE_PREFIX):
            return "mutation.invalid-reconstruction-asset"
    mesh = payload.get("mesh")
    if isinstance(mesh, dict) and not mesh["mesh"]["target"]["artifactId"].startswith(MESH_STAGE_HANDLE_PREFIX):
        return "mutation.invalid-reconstruction-mesh"
    return None
# endregion 🔖️Vocabulary — refusal rules


# region 🔖️Vocabulary — inverse rule
def inverse_mutation(kind, base, payload):
    """↩️ Every inverse is computed from BASE, never from the payload. Returns a LIST of
    `(appliers_key, payload)` steps, applied in order. Every kind returns exactly one, and it restores
    the member's POSITION as well as its presence (`taxonomy.md` rule 5) because the forward verbs
    insert at a canonical key position rather than appending."""
    if kind == "create-stream":
        return [("delete-stream", {"id": payload["stream"]["id"]})]
    if kind == "delete-stream":
        _, stream = _find(base["streams"], payload["id"])
        return [("create-stream", {"stream": stream})]
    if kind == "change-stream-sync":
        _, s = _find(base["streams"], payload["id"])
        return [("change-stream-sync", {"id": payload["id"], "newSyncOffsetMs": s["syncOffsetMs"]})]
    if kind == "add-stream-frame":
        _, s = _find(base["streams"], payload["id"])
        key = (payload["frame"]["index"], payload["frame"]["assetId"])
        return [("remove-stream-frame", {"id": payload["id"], "frameIndex": ordered_index(s["frames"], key, lambda frame: (frame["index"], frame["assetId"]))})]
    if kind == "remove-stream-frame":
        _, s = _find(base["streams"], payload["id"])
        frame = s["frames"][payload["frameIndex"]]
        return [("add-stream-frame", {"id": payload["id"], "frame": frame, "kind": s["kind"]})]
    if kind == "replace-stream-source":
        _, s = _find(base["streams"], payload["id"])
        return [("replace-stream-source", {"id": payload["id"], "source": s.get("source")})]
    if kind == "create-asset":
        if payload["key"] not in base["assets"]:
            return [("delete-asset", {"key": payload["key"]})]
        return [("__restore-asset-handle", _captured_asset(base, payload["key"]))]
    if kind == "delete-asset":
        return [("__restore-asset-handle", _captured_asset(base, payload["key"]))]
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
        key = (payload["observation"]["streamId"], payload["observation"]["frameIndex"])
        return [("remove-gcp-observation", {"id": payload["id"], "observationIndex": ordered_index(gcp["observations"], key, lambda observation: (observation["streamId"], observation["frameIndex"]))})]
    if kind == "remove-gcp-observation":
        _, gcp = _find(base["gcps"], payload["id"])
        observation = gcp["observations"][payload["observationIndex"]]
        return [("add-gcp-observation", {"id": payload["id"], "observation": observation})]
    if kind in PARAMS_KEY:
        return [(kind, {"params": base["params"][PARAMS_KEY[kind]]})]
    if kind == "replace-job":
        return [("replace-job", {"job": base["job"]})]
    if kind in RESULT_SLOT:
        return [(kind, {RESULT_ARG[kind]: base["results"][RESULT_SLOT[kind]]})]
    raise AssertionError(f"no inverse rule for kind {kind!r}")


def _captured_asset(base, key):
    """🔑 The handle AND the durable leaf the document carries for one asset key — the pair
    `create-asset` writes and `delete-asset` drops together."""
    handle = base["assets"][key]
    return {"key": key, "handle": handle, "leaf": base["durableArtifacts"][handle["childId"]]}


def apply_inverse_step(doc, step_kind, step_payload):
    """↩️ Applies one inverse step. Every step is a real verb of this vocabulary: since a `create`/`add`
    inserts at its member's canonical key position rather than appending, restoring a member removed
    from the middle takes no synthetic move step, and `taxonomy.md` rule 5's POSITION requirement is
    met by the forward verbs themselves. The one narrowing left is `__restore-asset-handle`, which
    adopts a committed `DefaultHasher` content-address this reference cannot specify."""
    return APPLIERS[step_kind](doc, step_payload)
# endregion 🔖️Vocabulary — inverse rule


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
    committed = ctx.fixture_bytes(carrier)
    text = committed.decode("utf-8")
    assert text.strip(), "identity-round-trip: the committed example is empty"
    assert not text.lstrip().startswith(("{", "[")), "identity-round-trip: the committed example is JSON, not the DSL carrier this scenario names"
    return Outcome(projection=text, raw=committed)
# endregion 🔖️Oracle


# region 🔖️Scenarios
# ▶️ Every `@id-mutate` row the feature plans, applied, refused and warned alike.
MUTATE_SCENARIOS: list[str] = [
    "add-gcp-observation",
    "add-gcp-observation-missing",
    "add-gcp-observation-noop",
    "add-gcp-observation-realworld",
    "add-stream-frame",
    "add-stream-frame-kind",
    "add-stream-frame-missing",
    "add-stream-frame-noop",
    "add-stream-frame-realworld",
    "change-stream-sync",
    "change-stream-sync-missing",
    "change-stream-sync-noop",
    "change-stream-sync-realworld",
    "commit-reconstruction",
    "commit-reconstruction-asset",
    "commit-reconstruction-mesh",
    "commit-reconstruction-sparse",
    "create-asset",
    "create-asset-realworld",
    "create-asset-staging-handle",
    "create-asset-upsert",
    "create-camera-calibration",
    "create-camera-calibration-duplicate",
    "create-camera-calibration-realworld",
    "create-gcp",
    "create-gcp-duplicate",
    "create-gcp-realworld",
    "create-gcp-unobserved",
    "create-rig-extrinsic",
    "create-rig-extrinsic-duplicate",
    "create-rig-extrinsic-realworld",
    "create-rig-extrinsic-unknown-camera",
    "create-stream",
    "create-stream-duplicate-id",
    "create-stream-realworld",
    "create-stream-unbound",
    "create-stream-unknown-camera",
    "delete-asset",
    "delete-asset-geo-product",
    "delete-asset-missing",
    "delete-asset-realworld",
    "delete-asset-referenced-frames",
    "delete-camera-calibration",
    "delete-camera-calibration-missing",
    "delete-camera-calibration-realworld",
    "delete-camera-calibration-referenced",
    "delete-gcp",
    "delete-gcp-missing",
    "delete-gcp-realworld",
    "delete-gcp-unobserved",
    "delete-rig-extrinsic",
    "delete-rig-extrinsic-first",
    "delete-rig-extrinsic-missing",
    "delete-rig-extrinsic-realworld",
    "delete-stream",
    "delete-stream-missing",
    "delete-stream-realworld",
    "delete-stream-referenced",
    "remove-gcp-observation",
    "remove-gcp-observation-first",
    "remove-gcp-observation-out-of-range",
    "remove-gcp-observation-realworld",
    "remove-stream-frame",
    "remove-stream-frame-first",
    "remove-stream-frame-out-of-range",
    "remove-stream-frame-realworld",
    "replace-dense",
    "replace-dense-noop",
    "replace-dense-realworld",
    "replace-geo-products",
    "replace-geo-products-absent",
    "replace-geo-products-clears",
    "replace-geo-products-realworld",
    "replace-job",
    "replace-job-noop",
    "replace-job-realworld",
    "replace-mesh-result",
    "replace-mesh-result-noop",
    "replace-mesh-result-realworld",
    "replace-mesh-result-staged",
    "replace-qc",
    "replace-qc-absent",
    "replace-qc-clears",
    "replace-qc-realworld",
    "replace-sparse",
    "replace-sparse-noop",
    "replace-sparse-realworld",
    "replace-stream-source",
    "replace-stream-source-attaches",
    "replace-stream-source-missing",
    "replace-stream-source-realworld",
    "replace-tracks",
    "replace-tracks-empty",
    "replace-tracks-noop",
    "replace-tracks-realworld",
    "replace-trajectory",
    "replace-trajectory-absent",
    "replace-trajectory-clears",
    "replace-trajectory-realworld",
    "update-camera-calibration",
    "update-camera-calibration-missing",
    "update-camera-calibration-noop",
    "update-camera-calibration-realworld",
    "update-dense-params",
    "update-dense-params-noop",
    "update-dense-params-realworld",
    "update-feature-params",
    "update-feature-params-invariant",
    "update-feature-params-noop",
    "update-feature-params-realworld",
    "update-geo-params",
    "update-geo-params-invariant",
    "update-geo-params-noop",
    "update-geo-params-realworld",
    "update-ingest-params",
    "update-ingest-params-invariant",
    "update-ingest-params-noop",
    "update-ingest-params-realworld",
    "update-match-params",
    "update-match-params-invariant",
    "update-match-params-noop",
    "update-match-params-realworld",
    "update-mesh-params",
    "update-mesh-params-noop",
    "update-mesh-params-realworld",
    "update-motion-params",
    "update-motion-params-noop",
    "update-motion-params-realworld",
    "update-rig-extrinsic",
    "update-rig-extrinsic-missing",
    "update-rig-extrinsic-noop",
    "update-rig-extrinsic-realworld",
    "update-sfm-params",
    "update-sfm-params-noop",
    "update-sfm-params-realworld",
]

# ↩️ Every `@id-inverse` row — one per committed vector, with no exceptions: since the
# canonical-order invariant and the referential guards landed, every kind's inverse restores its
# before-document exactly, and the generator refuses to commit a vector for which it does not.
INVERSE_SCENARIOS: list[str] = [
    "add-gcp-observation",
    "add-gcp-observation-missing",
    "add-gcp-observation-noop",
    "add-gcp-observation-realworld",
    "add-stream-frame",
    "add-stream-frame-kind",
    "add-stream-frame-missing",
    "add-stream-frame-noop",
    "add-stream-frame-realworld",
    "change-stream-sync",
    "change-stream-sync-missing",
    "change-stream-sync-noop",
    "change-stream-sync-realworld",
    "commit-reconstruction",
    "commit-reconstruction-asset",
    "commit-reconstruction-mesh",
    "commit-reconstruction-sparse",
    "create-asset",
    "create-asset-realworld",
    "create-asset-staging-handle",
    "create-asset-upsert",
    "create-camera-calibration",
    "create-camera-calibration-duplicate",
    "create-camera-calibration-realworld",
    "create-gcp",
    "create-gcp-duplicate",
    "create-gcp-realworld",
    "create-gcp-unobserved",
    "create-rig-extrinsic",
    "create-rig-extrinsic-duplicate",
    "create-rig-extrinsic-realworld",
    "create-rig-extrinsic-unknown-camera",
    "create-stream",
    "create-stream-duplicate-id",
    "create-stream-realworld",
    "create-stream-unbound",
    "create-stream-unknown-camera",
    "delete-asset",
    "delete-asset-geo-product",
    "delete-asset-missing",
    "delete-asset-realworld",
    "delete-asset-referenced-frames",
    "delete-camera-calibration",
    "delete-camera-calibration-missing",
    "delete-camera-calibration-realworld",
    "delete-camera-calibration-referenced",
    "delete-gcp",
    "delete-gcp-missing",
    "delete-gcp-realworld",
    "delete-gcp-unobserved",
    "delete-rig-extrinsic",
    "delete-rig-extrinsic-first",
    "delete-rig-extrinsic-missing",
    "delete-rig-extrinsic-realworld",
    "delete-stream",
    "delete-stream-missing",
    "delete-stream-realworld",
    "delete-stream-referenced",
    "remove-gcp-observation",
    "remove-gcp-observation-first",
    "remove-gcp-observation-out-of-range",
    "remove-gcp-observation-realworld",
    "remove-stream-frame",
    "remove-stream-frame-first",
    "remove-stream-frame-out-of-range",
    "remove-stream-frame-realworld",
    "replace-dense",
    "replace-dense-noop",
    "replace-dense-realworld",
    "replace-geo-products",
    "replace-geo-products-absent",
    "replace-geo-products-clears",
    "replace-geo-products-realworld",
    "replace-job",
    "replace-job-noop",
    "replace-job-realworld",
    "replace-mesh-result",
    "replace-mesh-result-noop",
    "replace-mesh-result-realworld",
    "replace-mesh-result-staged",
    "replace-qc",
    "replace-qc-absent",
    "replace-qc-clears",
    "replace-qc-realworld",
    "replace-sparse",
    "replace-sparse-noop",
    "replace-sparse-realworld",
    "replace-stream-source",
    "replace-stream-source-attaches",
    "replace-stream-source-missing",
    "replace-stream-source-realworld",
    "replace-tracks",
    "replace-tracks-empty",
    "replace-tracks-noop",
    "replace-tracks-realworld",
    "replace-trajectory",
    "replace-trajectory-absent",
    "replace-trajectory-clears",
    "replace-trajectory-realworld",
    "update-camera-calibration",
    "update-camera-calibration-missing",
    "update-camera-calibration-noop",
    "update-camera-calibration-realworld",
    "update-dense-params",
    "update-dense-params-noop",
    "update-dense-params-realworld",
    "update-feature-params",
    "update-feature-params-invariant",
    "update-feature-params-noop",
    "update-feature-params-realworld",
    "update-geo-params",
    "update-geo-params-invariant",
    "update-geo-params-noop",
    "update-geo-params-realworld",
    "update-ingest-params",
    "update-ingest-params-invariant",
    "update-ingest-params-noop",
    "update-ingest-params-realworld",
    "update-match-params",
    "update-match-params-invariant",
    "update-match-params-noop",
    "update-match-params-realworld",
    "update-mesh-params",
    "update-mesh-params-noop",
    "update-mesh-params-realworld",
    "update-motion-params",
    "update-motion-params-noop",
    "update-motion-params-realworld",
    "update-rig-extrinsic",
    "update-rig-extrinsic-missing",
    "update-rig-extrinsic-noop",
    "update-rig-extrinsic-realworld",
    "update-sfm-params",
    "update-sfm-params-noop",
    "update-sfm-params-realworld",
]
# endregion 🔖️Scenarios


# region 🔖️Registration
def adapter() -> Adapter:
    """🧭️ Registration is by full expanded scenario id, mirroring the feature's `Examples` tables, in
    the ORACLE role only — registering these handlers as subjects too would make the reference its own
    subject and manufacture a green self-comparison. Every planned scenario is answered: the runner
    plans a role over ALL of them and errors on a gap, so a kind left out would report as an
    unregistered scenario rather than as the honest scope limit it was meant to be."""
    built = Adapter("python")
    for scenario in MUTATE_SCENARIOS:
        built = built.oracle(f"mutate-{scenario}", mutate)
    for scenario in INVERSE_SCENARIOS:
        built = built.oracle(f"inverse-{scenario}", inverse)
    return built.oracle("identity-round-trip", identity_handler)
# endregion 🔖️Registration

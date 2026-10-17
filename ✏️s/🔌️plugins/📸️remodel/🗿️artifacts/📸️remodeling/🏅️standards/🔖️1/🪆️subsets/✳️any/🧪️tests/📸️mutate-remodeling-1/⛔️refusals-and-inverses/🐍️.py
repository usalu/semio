"""⛔️ The refusal rules each verb means and the single-step inverse of every kind."""

from __future__ import annotations

# region 🔖️Imports
import base64

from mutate_remodeling_1_vocabulary import (
    PARAMS_KEY,
    RESULT_SLOT,
    RESULT_ARG,
    RASTER_CONTENT_BYTES,
    CONTENT_ENVELOPE,
    _index_of,
    _find,
    ordered_index,
)
from mutate_remodeling_1_appliers import (
    content_handle,
    mesh_content_handle,
    leaf,
    content_complete,
    apply_commit_reconstruction,
    APPLIERS,
)
# endregion 🔖️Imports


# region 🔖️Vocabulary — refusal rules
def _finite(value) -> bool:
    return isinstance(value, (int, float)) and not isinstance(value, bool) and value == value and abs(value) != float("inf")


def refusal_of(kind, base, payload):
    """🚦 The diagnostic this payload earns against this document, or `None` when the verb accepts it.

    Every line below is a statement about what the VERB means, not about how production is written:
    a record may not be created under an id the document already holds (`mutation.duplicate-id`); a
    verb may only address or reference a member that exists, index and media kind included
    (`mutation.target-missing`); a value a
    verb constrains must satisfy that constraint (`mutation.invariant`); resubmitting the value a
    document already holds changes nothing (`mutation.no-op`); a content handle names only content the
    document stores complete, and only a reconstruction commit binds one (`mutation.target-mismatch`,
    `mutation.invariant`, `mutation.target-mismatch`); a content leaf is bounded,
    contiguous and never rewritten (`mutation.invariant`, `mutation.target-mismatch`)."""
    streams, gcps = base["streams"], base["gcps"]
    cameras, rig = base["calibration"]["cameras"], base["calibration"]["rig"]
    params, results = base["params"], base["results"]

    if kind == "create-stream":
        if _index_of(streams, payload["stream"]["id"]) >= 0:
            return "mutation.duplicate-id"
        camera_id = payload["stream"].get("cameraId")
        if camera_id is not None and _index_of(cameras, camera_id) < 0:
            return "mutation.target-missing"
        return None
    if kind == "replace-stream-source":
        return None if _index_of(streams, payload["id"]) >= 0 else "mutation.target-missing"
    if kind == "delete-stream":
        if _index_of(streams, payload["id"]) < 0:
            return "mutation.target-missing"
        # 🔗 A GCP owns its observations; a stream one of them names may not be taken out from under it.
        return "mutation.target-referenced" if any(o["streamId"] == payload["id"] for gcp in gcps for o in gcp["observations"]) else None
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
        # different one addresses a stream of that kind this document does not hold.
        if payload["kind"] != streams[index]["kind"]:
            return "mutation.target-missing"
        return "mutation.no-op" if payload["frame"] in streams[index]["frames"] else None
    if kind == "remove-stream-frame":
        index = _index_of(streams, payload["id"])
        if index < 0 or payload["frameIndex"] >= len(streams[index]["frames"]):
            return "mutation.target-missing"
        return None
    if kind == "create-asset":
        # 🔗 Durable content is bound by a reconstruction commit; an imported raster carries its own bytes.
        if content_handle(payload["asset"]["data"]) is not None:
            return "mutation.invariant"
        try:
            raw = base64.b64decode(payload["asset"]["data"], validate=True)
        except ValueError:
            return "mutation.invariant"
        return "mutation.invariant" if len(raw) > RASTER_CONTENT_BYTES else None
    if kind == "delete-asset":
        if payload["key"] not in base["assets"]:
            return "mutation.target-missing"
        referenced = any(frame["assetId"] == payload["key"] for stream in streams for frame in stream["frames"])
        referenced = referenced or results["mesh"]["textureAssetId"] == payload["key"]
        if results["geo"] is not None:
            referenced = referenced or any(results["geo"][slot] == payload["key"] for slot in ("dsmAssetId", "dtmAssetId", "orthoAssetId"))
        # 🔗 Removing content the document still names would leave a reference pointing at nothing.
        return "mutation.target-referenced" if referenced else None
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
        return "mutation.target-referenced" if referenced else None
    if kind == "create-rig-extrinsic":
        camera_id = payload["extrinsic"]["cameraId"]
        if _index_of(rig, camera_id, key="cameraId") >= 0:
            return "mutation.duplicate-id"
        return None if _index_of(cameras, camera_id) >= 0 else "mutation.target-missing"
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
            return "mutation.target-missing"
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
    if kind == "replace-mesh-result":
        named = mesh_content_handle(payload["mesh"]["mesh"])
        if named is not None and not content_complete(base["durableArtifacts"], named[0], "mesh", named[1]):
            return "mutation.target-mismatch"
        return "mutation.no-op" if payload["mesh"] == results["mesh"] else None
    if kind in RESULT_SLOT:
        slot, arg = RESULT_SLOT[kind], RESULT_ARG[kind]
        clearable = slot in ("trajectory", "geo", "qc")
        if clearable and payload[arg] is None and results[slot] is None:
            return "mutation.target-missing"
        return "mutation.no-op" if payload[arg] == results[slot] else None
    if kind == "append-content":
        return refuses_append(base["durableArtifacts"], payload)
    if kind == "remove-content":
        entry = base["durableArtifacts"].get(payload["contentId"])
        if entry is None:
            return "mutation.target-missing"
        if payload["from"] > len(entry["chunks"]):
            return "mutation.target-mismatch"
        return "mutation.no-op" if payload["from"] == len(entry["chunks"]) else None
    if kind == "commit-reconstruction":
        return refuses_commit(base, payload)
    raise AssertionError(f"no refusal rule for kind {kind!r}")


def refuses_append(store, payload):
    """📦 A leaf is a base64 string of at most 4 KiB and an append carries at least one; leaves are
    contiguous, so the first may not start past the stored count; one entry keeps one kind and
    presentation; a stored leaf is immutable, so an overlapping leaf must equal it; an append that
    only repeats stored leaves changes nothing; and the entry stays inside its kind's envelope."""
    raws = [leaf(encoded) for encoded in payload["chunks"]]
    if not raws or any(raw is None for raw in raws):
        return "mutation.invariant"
    entry = store.get(payload["contentId"])
    stored = [] if entry is None else entry["chunks"]
    first = payload["first"]
    if first > len(stored):
        return "mutation.target-mismatch"
    if entry is not None and (entry["kind"], entry["mime"], entry["width"], entry["height"]) != (payload["kind"], payload["mime"], payload["width"], payload["height"]):
        return "mutation.target-mismatch"
    repeated = stored[first : first + len(payload["chunks"])]
    if repeated != payload["chunks"][: len(repeated)]:
        return "mutation.target-mismatch"
    if len(repeated) == len(payload["chunks"]):
        return "mutation.no-op"
    max_bytes, max_leaves = CONTENT_ENVELOPE[payload["kind"]]
    kept = sum(len(leaf(encoded) or b"") for encoded in stored[: first + len(repeated)])
    if first + len(payload["chunks"]) > max_leaves or kept + sum(len(raw) for raw in raws) > max_bytes:
        return "mutation.target-mismatch"
    return None


def refuses_commit(base, payload):
    """🏁 `commit-reconstruction` binds content by id, so every handle it names must address content
    the BASE document already stores complete: the sparse point buffer, then the mesh child, then each
    asset binding in payload order. Inline buffers and constant meshes name no content. A commit that
    changes neither a result nor a binding is a no-op."""
    store = base["durableArtifacts"]
    sparse = payload["sparse"]
    named = None if sparse is None else content_handle(sparse["points"])
    if named is not None and not content_complete(store, named[0], "sparse", named[1]):
        return "mutation.target-mismatch"
    named = None if payload["mesh"] is None else mesh_content_handle(payload["mesh"]["mesh"])
    if named is not None and not content_complete(store, named[0], "mesh", named[1]):
        return "mutation.target-mismatch"
    for binding in payload["assets"]:
        content_id = binding.get("contentId")
        if content_id is not None and not (content_id in store and content_complete(store, content_id, "image", len(store[content_id]["chunks"]))):
            return "mutation.target-mismatch"
    return "mutation.no-op" if apply_commit_reconstruction(base, payload) == base else None
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
    if kind in RESULT_SLOT:
        return [(kind, {RESULT_ARG[kind]: base["results"][RESULT_SLOT[kind]]})]
    if kind == "append-content":
        entry = base["durableArtifacts"].get(payload["contentId"])
        stored = 0 if entry is None else len(entry["chunks"])
        return [] if payload["first"] + len(payload["chunks"]) <= stored else [("remove-content", {"contentId": payload["contentId"], "from": stored})]
    if kind == "remove-content":
        entry = base["durableArtifacts"][payload["contentId"]]
        if entry["kind"] not in CONTENT_ENVELOPE or payload["from"] >= len(entry["chunks"]):
            return []
        restored = {"contentId": payload["contentId"], "kind": entry["kind"], "mime": entry["mime"], "width": entry["width"], "height": entry["height"], "first": payload["from"], "chunks": entry["chunks"][payload["from"] :]}
        return [("append-content", restored)]
    if kind == "commit-reconstruction":
        results = base["results"]
        bindings = [{"id": binding["id"], "contentId": base["assets"][binding["id"]]["childId"] if binding["id"] in base["assets"] else None} for binding in payload["assets"]]
        return [("commit-reconstruction", {"sparse": results["sparse"], "trajectory": results["trajectory"], "mesh": results["mesh"], "geo": results["geo"], "qc": results["qc"], "assets": bindings})]
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

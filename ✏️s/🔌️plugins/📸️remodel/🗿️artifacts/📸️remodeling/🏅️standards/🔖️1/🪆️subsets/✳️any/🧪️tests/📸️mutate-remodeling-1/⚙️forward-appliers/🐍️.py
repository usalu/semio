"""⚙️ The forward appliers: one function per kind that moves the document."""

from __future__ import annotations

# region 🔖️Imports
import base64
import copy
import re

from mutate_remodeling_1_vocabulary import (
    PARAMS_KEY,
    RESULT_SLOT,
    RESULT_ARG,
    DURABLE_CHUNK_RAW_BYTES,
    RASTER_CONTENT_BYTES,
    U64_MAX,
    CONTENT_ENVELOPE,
    MESH_FIELDS,
    _find,
    ordered_index,
)
# endregion 🔖️Imports


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


def _count(text):
    """🔢 A decimal leaf count as an unsigned 64-bit integer, or `None`."""
    return int(text) if re.fullmatch(r"\+?[0-9]+", text) and int(text) <= U64_MAX else None


def content_handle(points):
    """🔗 `(contentId, leafCount)` a `remodeling-content:<id>|<leaves>` point buffer names, else `None`."""
    if not isinstance(points, str) or not points.startswith("remodeling-content:"):
        return None
    content_id, bar, leaves = points[len("remodeling-content:") :].rpartition("|")
    count = _count(leaves)
    return (content_id, count) if bar and count is not None else None


def mesh_content_handle(child):
    """🔗 `(contentId, leafCount)` a `remodeling-mesh-content:<leaves>` mesh child names, else `None`."""
    artifact_id = child["target"]["artifactId"]
    if not artifact_id.startswith("remodeling-mesh-content:"):
        return None
    count = _count(artifact_id[len("remodeling-mesh-content:") :])
    return None if count is None else (child["childId"], count)


def leaf(encoded):
    """🧱 One durable leaf's raw bytes, or `None` when it is not base64 or exceeds 4 KiB."""
    if not isinstance(encoded, str) or len(encoded) > (DURABLE_CHUNK_RAW_BYTES + 2) // 3 * 4:
        return None
    try:
        raw = base64.b64decode(encoded, validate=True)
    except ValueError:
        return None
    return raw if len(raw) <= DURABLE_CHUNK_RAW_BYTES else None


def mesh_resolves(leaves):
    """🧊 Whether field-tagged mesh leaves decode, in ascending field order, into a mesh inside the
    512-vertex/512-triangle envelope with consistent per-vertex and per-triangle lanes."""
    counts = dict.fromkeys(MESH_FIELDS, 0)
    indices = []
    previous = -1
    total = 0
    for encoded in leaves:
        raw = leaf(encoded)
        if raw is None or not raw:
            return False
        total += len(raw)
        field, values = raw[0], raw[1:]
        if total > CONTENT_ENVELOPE["mesh"][0] or field not in MESH_FIELDS or field < previous:
            return False
        width, cap = MESH_FIELDS[field]
        if len(values) % width:
            return False
        if field == 11:
            try:
                values.decode("utf-8")
            except UnicodeDecodeError:
                return False
        counts[field] += len(values) // width
        if counts[field] > cap:
            return False
        if field == 3:
            indices.extend(int.from_bytes(values[offset : offset + 4], "little") for offset in range(0, len(values), 4))
        previous = field
    if counts[0] % 3 or len(indices) % 3:
        return False
    vertices, triangles = counts[0] // 3, len(indices) // 3
    return (
        vertices <= 512
        and triangles <= 512
        and all(index < vertices for index in indices)
        and counts[1] in (0, vertices * 3)
        and counts[2] in (0, vertices * 3, vertices * 4)
        and counts[4] in (0, vertices * 2)
        and counts[5] in (0, triangles)
        and counts[6] in (0, vertices)
    )


def content_complete(store, content_id, kind, leaf_count):
    """✅ Whether the document stores `content_id` as complete content of `kind`: exactly `leaf_count`
    (at least one) leaves inside the kind's byte envelope, a mesh additionally resolving."""
    entry = store.get(content_id)
    if entry is None or entry["kind"] != kind or leaf_count == 0 or len(entry["chunks"]) != leaf_count:
        return False
    if kind == "mesh":
        return mesh_resolves(entry["chunks"])
    raws = [leaf(encoded) for encoded in entry["chunks"]]
    return all(raw is not None for raw in raws) and sum(len(raw) for raw in raws) <= CONTENT_ENVELOPE[kind][0]


def bound_asset_handle(asset_id, content_id):
    """🖼️ The `assets` handle a reconstruction binding writes: the content id is the child id."""
    return {"childId": content_id, "target": {"artifactId": f"{asset_id}-image", "dialect": {"artifactKind": "s.stdio.semio", "standard": "v1", "subset": "image"}}}


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


def apply_append_content(doc, p):
    """📦 Places the payload's leaves at leaf index `first`, creating the entry with the payload's kind
    and presentation when the document holds none; leaves already stored are not written twice."""
    after = copy.deepcopy(doc)
    store = after["durableArtifacts"]
    entry = store.setdefault(p["contentId"], {"kind": p["kind"], "mime": p["mime"], "width": p["width"], "height": p["height"], "chunks": []})
    if p["first"] + len(p["chunks"]) > len(entry["chunks"]):
        entry["chunks"] = entry["chunks"][: p["first"]] + list(p["chunks"])
    after["durableArtifacts"] = {key: store[key] for key in sorted(store)}
    return after


def apply_remove_content(doc, p):
    """🔪 Keeps the leaves before `from`; keeping none removes the entry itself."""
    after = copy.deepcopy(doc)
    store = after["durableArtifacts"]
    if p["from"] == 0:
        del store[p["contentId"]]
    else:
        store[p["contentId"]]["chunks"] = store[p["contentId"]]["chunks"][: p["from"]]
    return after


def apply_commit_reconstruction(doc, p):
    """🏁 Replaces the sparse, trajectory, geo and QC results outright, the mesh only when one is given,
    and binds or unbinds every named asset — all in one step."""
    after = copy.deepcopy(doc)
    for slot in ("sparse", "trajectory", "geo", "qc"):
        after["results"][slot] = copy.deepcopy(p[slot])
    if p["mesh"] is not None:
        after["results"]["mesh"] = copy.deepcopy(p["mesh"])
    assets = after["assets"]
    for binding in p["assets"]:
        if binding.get("contentId") is None:
            assets.pop(binding["id"], None)
        else:
            assets[binding["id"]] = bound_asset_handle(binding["id"], binding["contentId"])
    after["assets"] = {key: assets[key] for key in sorted(assets)}
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
    "append-content": apply_append_content,
    "remove-content": apply_remove_content,
    "commit-reconstruction": apply_commit_reconstruction,
}
for _kind, _key in PARAMS_KEY.items():
    APPLIERS[_kind] = (lambda key: lambda doc, p: _apply_update_params(doc, p, key))(_key)
for _kind, _slot in RESULT_SLOT.items():
    APPLIERS[_kind] = (lambda slot, arg: lambda doc, p: _apply_replace_result(doc, p, slot, arg))(_slot, RESULT_ARG[_kind])
# endregion 🔖️Vocabulary — forward appliers

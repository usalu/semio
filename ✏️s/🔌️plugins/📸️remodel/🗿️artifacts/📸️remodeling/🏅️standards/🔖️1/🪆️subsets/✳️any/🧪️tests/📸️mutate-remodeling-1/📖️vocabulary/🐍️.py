"""📖️ The kind tables, parameter and result slots, durable-content envelopes, fixture readers and the addressing helpers."""

from __future__ import annotations

# region 🔖️Imports
import json

from semio_repo_test import Context
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
    "replace-sparse": "replaceSparse",
    "replace-dense": "replaceDense",
    "replace-mesh-result": "replaceMeshResult",
    "replace-trajectory": "replaceTrajectory",
    "replace-tracks": "replaceTracks",
    "replace-geo-products": "replaceGeoProducts",
    "replace-qc": "replaceQc",
    "append-content": "appendContent",
    "remove-content": "removeContent",
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

DURABLE_CHUNK_RAW_BYTES = 4096
RASTER_CONTENT_BYTES = 1_114_112
U64_MAX = (1 << 64) - 1

# 📏 Each durable content kind's envelope: the most raw bytes and leaves one entry may hold.
CONTENT_ENVELOPE = {"sparse": (512 * 12, 2), "mesh": (87_552 + 30, 30), "image": (RASTER_CONTENT_BYTES, 272)}

# 🧊 Mesh leaves are `[field, values...]`; each field's element width (4 for f32/u32 lanes, 1 for raw
# bytes and UTF-8 text) and the most elements the 512-vertex/512-triangle envelope admits.
MESH_FIELDS = {
    0: (4, 512 * 3),
    1: (4, 512 * 3),
    2: (4, 512 * 4),
    3: (4, 512 * 3),
    4: (4, 512 * 2),
    5: (4, 512),
    6: (4, 512),
    7: (4, 512 * 6),
    8: (4, 512 * 3),
    9: (4, 512 * 4),
    10: (1, 512 * 3),
    11: (1, 24_576),
}
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
    return json.loads(ctx.input_bytes(uri))


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

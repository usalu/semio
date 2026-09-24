"""🐍️ `s.remodel.remodeling`'s second, independent implementation of all 36 of its mutation kinds.

`s.remodel.remodeling` is a semio-NATIVE reconstruction project document — streams, calibrations,
ground control points, the eight `ReconstructionParams` sub-records a pipeline runs under, the
durable content leaves a run publishes, and the engine-owned results — not a point cloud or a mesh
file. A reader of COLMAP, LAS or PLY output would
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
stream may not be created twice, a frame index must address a frame that exists, a commit binds only
content the document already stores complete — not from production's control flow, and they carry none of
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
"move" step to state it, and it asserts that for all 136 committed vectors.

📦️ DURABLE CONTENT is ordinary document state. A reconstruction run publishes every sparse cloud, mesh
and raster as bounded 4 KiB leaves through `append-content` (removed again by `remove-content`), and
`commit-reconstruction` then names that content by id: a `remodeling-content:<id>|<leaves>` point
buffer, a `remodeling-mesh-content:<leaves>` mesh child, or an asset binding. Completeness of named
content is therefore decidable from the BASE document alone, so the commit is modelled in full here —
its refusals, its no-op, its forward application and its inverse.
"""

from __future__ import annotations

# region 🔖️Imports
import base64
import copy
import json
import re

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


# region 🔖️Vocabulary — refusal rules
def _finite(value) -> bool:
    return isinstance(value, (int, float)) and not isinstance(value, bool) and value == value and abs(value) != float("inf")


def refusal_of(kind, base, payload):
    """🚦 The diagnostic this payload earns against this document, or `None` when the verb accepts it.

    Every line below is a statement about what the VERB means, not about how production is written:
    a record may not be created under an id the document already holds (`mutation.duplicate-id`); a
    verb may only address a member that exists, index included (`mutation.target-missing`); a value a
    verb constrains must satisfy that constraint (`mutation.invariant`); resubmitting the value a
    document already holds changes nothing (`mutation.no-op`); a content handle names only content the
    document stores complete, and only a reconstruction commit binds one (`mutation.incomplete-mesh`,
    `mutation.invalid-asset-payload`, `mutation.invalid-reconstruction-*`); a content leaf is bounded,
    contiguous and never rewritten (`mutation.invalid-content-chunk`, `mutation.content-*`)."""
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
        # 🔗 Durable content is bound by a reconstruction commit; an imported raster carries its own bytes.
        if content_handle(payload["asset"]["data"]) is not None:
            return "mutation.invalid-asset-payload"
        try:
            raw = base64.b64decode(payload["asset"]["data"], validate=True)
        except ValueError:
            return "mutation.invalid-asset-payload"
        return "mutation.invalid-asset-payload" if len(raw) > RASTER_CONTENT_BYTES else None
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
    if kind == "replace-mesh-result":
        named = mesh_content_handle(payload["mesh"]["mesh"])
        if named is not None and not content_complete(base["durableArtifacts"], named[0], "mesh", named[1]):
            return "mutation.incomplete-mesh"
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
            return "mutation.content-gap"
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
        return "mutation.invalid-content-chunk"
    entry = store.get(payload["contentId"])
    stored = [] if entry is None else entry["chunks"]
    first = payload["first"]
    if first > len(stored):
        return "mutation.content-gap"
    if entry is not None and (entry["kind"], entry["mime"], entry["width"], entry["height"]) != (payload["kind"], payload["mime"], payload["width"], payload["height"]):
        return "mutation.content-kind-mismatch"
    repeated = stored[first : first + len(payload["chunks"])]
    if repeated != payload["chunks"][: len(repeated)]:
        return "mutation.content-conflict"
    if len(repeated) == len(payload["chunks"]):
        return "mutation.no-op"
    max_bytes, max_leaves = CONTENT_ENVELOPE[payload["kind"]]
    kept = sum(len(leaf(encoded) or b"") for encoded in stored[: first + len(repeated)])
    if first + len(payload["chunks"]) > max_leaves or kept + sum(len(raw) for raw in raws) > max_bytes:
        return "mutation.content-capacity"
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
        return "mutation.invalid-reconstruction-sparse"
    named = None if payload["mesh"] is None else mesh_content_handle(payload["mesh"]["mesh"])
    if named is not None and not content_complete(store, named[0], "mesh", named[1]):
        return "mutation.invalid-reconstruction-mesh"
    for binding in payload["assets"]:
        content_id = binding.get("contentId")
        if content_id is not None and not (content_id in store and content_complete(store, content_id, "image", len(store[content_id]["chunks"]))):
            return "mutation.invalid-reconstruction-asset"
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

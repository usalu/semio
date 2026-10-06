#!/usr/bin/env python3
"""🧩️ An INDEPENDENT second implementation of the `s.puzzle.3d` scene document and its thirty-eight
typed mutations, in Python, serving as this case's differential oracle.

**Why a second implementation and not a third-party library.** A `puzzle3d` document is a SCENE whose
connectivity is not spatial: objects carry their own VORTICES, and an attraction joins two of them by
a two-part address `"<objectId>:<vortexId>"` rather than by a transform. Beside the objects sit target
volumes and image references, each with its own placement, and a metadata block holding a
kind-compatibility relation and an optional kind catalogue. No scene-graph interchange format —
glTF, USD, IFC — models a joint whose endpoints are named ports owned by two nodes, and none of them
reads `.dsl.semio`. What a reference genuinely can adjudicate is this document's own algebra, and that
it IS adjudicable was settled in this same wave by `mutate-fem3d-1` and `🗺️mutate-gisterrain-1`, which
took Python second implementations over this same carrier.

**What it was written from.**

* ``🏅️standards/🔖️1/🪆️subsets/✳️any/🧬️schema/📸️snapshot/🔣️.json`` — the seven members of
  `Puzzle3dSnapshot`.
* rules 2, 4 and 7 of
  `.🧬semio/🦑️repo/🎫️tickets/🎆️26/🌙️08/☀️12/SEMANTIC-MUTATIONS-OVERHAUL/📓️derivation-rules.md`.
* the thirty-five committed `(before, mutation, diff, outcome, after)` quintets, for the verbs and
  their argument lists and for the three things only they state: that a vortex list is INSIDE the
  object, so removing a vortex is an object mutation that CASCADES into the attractions addressed to
  it; that deleting an object severs every attraction naming any of its vortices; and that `scale` is
  a union — `scale-object` writes a per-axis triple over a uniform scalar and `scale-target-volume`
  writes a uniform scalar over a per-axis triple.

The verbs were NOT read from ``…/🧬️schema/🧬️mutations/🔣️.json``: that file is titled
`Puzzle3dMutation` but declares the SNAPSHOT's members, the pre-migration whole-snapshot-shaped
generic schema `s.architect.program`'s own mutation schema records itself as superseding. It was never
replaced here.

**No Rust was read to write this.** `🦀️.rs` beside this file registers the SUBJECT half
only. The one exception is NUMERIC, not semantic: re-deriving an attraction's six connection parameters
(`gap`, `shift`, `rise`, `rotation`, `turn`, `tilt`) after a selection move is the compose kernel's pose
inversion, ported operation for operation so both sides round alike. WHICH objects follow a move and WHICH
attractions re-derive is this file's own reading of the selection leaves' schema descriptions.

**One kind this implementation REFUSES, by clause rather than by absence.** See `UNDERDETERMINED`.
"""

# region 🔖️Imports
import copy
import json
import math

from semio_repo_test import Adapter, Outcome

# endregion 🔖️Imports


# region 🔖️Vocabulary
MEMBERS = ("schema", "domain", "meta", "objects", "attractions", "targetVolumes", "references")
"""🗂️ The seven members `Puzzle3dSnapshot` declares — and the cross-language projection."""

OBJECT_FIELDS = {"move-object": ("origin", "newOrigin"), "rotate-object": ("orientation", "newOrientation"), "scale-object": ("scale", "newScale"), "change-object-mesh": ("meshUrl", "newMeshUrl"), "edit-object-label": ("label", "newLabel"), "change-object-kind": ("objectKind", "newObjectKind"), "change-object-anchor": ("anchor", "newAnchor"), "change-object-hidden": ("hidden", "newHidden"), "change-object-locked": ("locked", "newLocked")}
"""✏️ The nine single-field object setters."""

VOLUME_FIELDS = {"move-target-volume": ("origin", "newOrigin"), "rotate-target-volume": ("orientation", "newOrientation"), "scale-target-volume": ("scale", "newScale"), "change-target-volume-hidden": ("hidden", "newHidden"), "change-target-volume-locked": ("locked", "newLocked")}
"""📦 The five single-field target-volume setters."""

REFERENCE_FIELDS = {"move-reference": ("origin", "newOrigin"), "resize-reference": ("widthWorld", "newWidthWorld"), "replace-reference-source": ("source", "newSource"), "change-reference-hidden": ("hidden", "newHidden"), "change-reference-locked": ("locked", "newLocked")}
"""🖼️ The five single-field reference setters."""

COLLECTIONS = {"create-object": ("objects", "object"), "create-target-volume": ("targetVolumes", "targetVolume"), "create-reference": ("references", "reference")}
"""🌱 The three indexed append verbs: which member each writes and what its whole-record payload is
called."""

REMOVALS = {"delete-object": "objects", "delete-target-volume": "targetVolumes", "delete-reference": "references"}
"""🗑️ The three id-addressed removals. `delete-object` additionally severs attractions."""

ATTRACTION_GEOMETRY = ("gap", "shift", "rise", "rotation", "turn", "tilt", "x", "y")
"""🧮 The eight geometry members of an attraction. `replace-attraction-geometry` names them with a
`new` prefix; `connect-vortices` names them bare, the one place in this vocabulary where the same
eight values are addressed under two spellings."""

COMPATIBILITY_FIELDS = ("source", "target", "bidirectional", "important", "specificity")
"""🤝 The members of one kind-compatibility record, in the order `connect-kind-compatibility` names
them."""

UNDERDETERMINED = {"replace-object-vortex"}
"""🚧️ The one kind this implementation refuses to state — see `UNDERDETERMINED_REASON`."""

UNDERDETERMINED_REASON = (
    "this implementation refuses this kind rather than guessing it. Its single committed vector supplies a genuinely different vortex — `vortex-1` "
    "moves from `vortex-kind-a` to `vortex-kind-c` — and yet the committed outcome declares `mutation.no-op` and the after-snapshot is identical to "
    "the before-snapshot. At least three rules produce exactly that and no committed document distinguishes them: the verb is unimplemented; it "
    "refuses a vortex an attraction is addressed to, which `vortex-1` is; or it refuses a vortex kind the `kindCompatibility` relation does not admit, "
    "which `vortex-kind-c` is. `📓️derivation-rules.md` rule 2 says `replace-<singular>-<member>` replaces the addressed record, so a second "
    "implementation written from the specification would move the document. ONE more committed vector, on an unattracted vortex, decides it. Its "
    "sibling `mutate-puzzle-2d-1` reports the identical gap over `replace-node-handle`."
)

KINDS = (
    "create-object",
    "delete-object",
    "move-object",
    "rotate-object",
    "scale-object",
    "change-object-mesh",
    "edit-object-label",
    "change-object-kind",
    "change-object-anchor",
    "change-object-hidden",
    "change-object-locked",
    "add-object-vortex",
    "remove-object-vortex",
    "replace-object-vortex",
    "connect-vortices",
    "disconnect-vortices",
    "replace-attraction-geometry",
    "create-target-volume",
    "delete-target-volume",
    "move-target-volume",
    "rotate-target-volume",
    "scale-target-volume",
    "change-target-volume-hidden",
    "change-target-volume-locked",
    "create-reference",
    "delete-reference",
    "move-reference",
    "resize-reference",
    "replace-reference-source",
    "change-reference-hidden",
    "change-reference-locked",
    "change-domain",
    "connect-kind-compatibility",
    "disconnect-kind-compatibility",
    "replace-kind-catalogs",
    "drag-selection",
    "rotate-selection",
    "scale-selection",
)
"""🏷️ Every kind the catalog declares, in its declared order."""

SELECTION = {"drag-selection": "origin", "rotate-selection": "orientation", "scale-selection": "scale"}
"""🧭️ The three parametric selection kinds and the one pose member each rewrites on its targets. They state
INTENT rather than a final value: every addressed id that names an unlocked object or target volume is
transformed IN PLACE from whatever pose the scene holds; absent and locked ids are skipped. A drag and a turn
also re-solve the attraction graph — attracted objects follow, attractions between ends that stop moving
together re-derive — so resolving the scene afterwards never snaps a moved object back."""

SETTERS = {("objects", "origin"): ("move-object", "newOrigin"), ("objects", "orientation"): ("rotate-object", "newOrientation"), ("objects", "scale"): ("scale-object", "newScale"), ("targetVolumes", "origin"): ("move-target-volume", "newOrigin"), ("targetVolumes", "orientation"): ("rotate-target-volume", "newOrientation"), ("targetVolumes", "scale"): ("scale-target-volume", "newScale")}
"""↩️ The absolute setter that restores one pose member of one collection — how a selection kind is undone."""

IDENTITY = [0.0, 0.0, 0.0, 1.0]
"""🧭️ The quaternion of no rotation, `[x, y, z, w]` — what an object without an orientation stands at."""


def tag_of(kind):
    """🔤️ The internally tagged `mutation` discriminator of a kind — lowerCamelCase of its words."""
    head, *rest = kind.split("-")
    return head + "".join(word[:1].upper() + word[1:] for word in rest)


TAGS = {kind: tag_of(kind) for kind in KINDS}
# endregion 🔖️Vocabulary


# region 🔖️Document
def validate(document, where):
    """✅️ Holds the document to the shape the committed vectors agree on: seven members, unique
    object, vortex, attraction, volume and reference ids, and every attraction addressed to a
    `"<objectId>:<vortexId>"` pair the scene really holds."""
    if set(document) != set(MEMBERS):
        raise AssertionError("%s: a puzzle3d document must carry exactly %r, found %r" % (where, sorted(MEMBERS), sorted(document)))
    if "kindCompatibility" not in document["meta"]:
        raise AssertionError("%s: meta must carry a kindCompatibility relation, found %r" % (where, sorted(document["meta"])))
    ports = set()
    for member in ("objects", "targetVolumes", "references"):
        identifiers = [record["id"] for record in document[member]]
        if len(set(identifiers)) != len(identifiers):
            raise AssertionError("%s: %s carries a duplicate id in %r" % (where, member, identifiers))
    for record in document["objects"]:
        for vortex in record["vortices"]:
            port = "%s:%s" % (record["id"], vortex["id"])
            if port in ports:
                raise AssertionError("%s: the port %r is declared twice" % (where, port))
            ports.add(port)
    for attraction in document["attractions"]:
        for end in ("attracting", "attracted"):
            if attraction[end] not in ports:
                raise AssertionError("%s: attraction %r names %s port %r, which this scene does not hold" % (where, attraction["id"], end, attraction[end]))


def record_at(document, member, identity, kind, where):
    """🔎️ The index of the record this kind addresses; an absent id is an error, never a no-op."""
    for at, record in enumerate(document[member]):
        if record["id"] == identity:
            return at
    raise AssertionError("%s-%s: the committed vector addresses %s %r, which the before-snapshot does not hold" % (where, kind, member, identity))


def ports_of(record):
    """🔌 The two-part addresses one object's vortices answer to."""
    return {"%s:%s" % (record["id"], vortex["id"]) for vortex in record["vortices"]}


def attached_to(document, ports):
    """✂️ The attractions addressed to any of these ports, in scene order — what a removal severs."""
    return [held for held in document["attractions"] if held["attracting"] in ports or held["attracted"] in ports]
# endregion 🔖️Document


# region 🔖️Selection
def hamilton(a, b):
    """✖️ The Hamilton product `a·b` of two `[x, y, z, w]` quaternions: turning by `b` and then by `a`."""
    ax, ay, az, aw = a
    bx, by, bz, bw = b
    return [aw * bx + ax * bw + ay * bz - az * by, aw * by - ax * bz + ay * bw + az * bx, aw * bz + ax * by - ay * bx + az * bw, aw * bw - ax * bx - ay * by - az * bz]


def axis_angle(axis, angle):
    """🧭️ The unit quaternion turning `angle` radians about `axis` (right-handed); no turn for a degenerate axis."""
    length = math.sqrt(axis[0] * axis[0] + axis[1] * axis[1] + axis[2] * axis[2])
    if length < 1e-8:
        return list(IDENTITY)
    sine = math.sin(angle * 0.5)
    return [axis[0] / length * sine, axis[1] / length * sine, axis[2] / length * sine, math.cos(angle * 0.5)]


def moves(kind, payload):
    """🚦️ Whether the parameters move anything at all: a zero offset, a turn that is no turn and unit factors
    leave every pose exactly as it stands."""
    if kind == "drag-selection":
        return payload["offset"] != [0.0, 0.0, 0.0]
    if kind == "rotate-selection":
        return payload["angle"] != 0.0 and axis_angle(payload["axis"], payload["angle"]) != IDENTITY
    return payload["factors"] != [1.0, 1.0, 1.0]


def addressed(document, kind, payload):
    """🎯️ `(member, index)` of every addressed unlocked object or target volume a moving selection kind
    rewrites, in scene order."""
    if not moves(kind, payload):
        return []
    return [(member, at) for member in ("objects", "targetVolumes") for at, record in enumerate(document[member]) if record["id"] in payload["targets"] and not record["locked"]]


def transformed(kind, payload, record):
    """🧮️ The value the selection kind writes into one record's pose member."""
    if kind == "drag-selection":
        return [record["origin"][axis] + payload["offset"][axis] for axis in range(3)]
    if kind == "rotate-selection":
        return hamilton(axis_angle(payload["axis"], payload["angle"]), record.get("orientation") or IDENTITY)
    held = record.get("scale")
    triple = [1.0, 1.0, 1.0] if held is None else ([held] * 3 if isinstance(held, (int, float)) else held)
    return [triple[axis] * payload["factors"][axis] for axis in range(3)]


def selection_effect(document, kind, payload):
    """🌲️ What one selection kind changes: `{index: record}` for objects (the targets AND every unlocked object an
    attraction hangs off a moved object, re-placed breadth first from its moved parent with the attraction's own
    parameters), `{index: record}` for target volumes, and every OTHER attraction touching a moved object,
    re-derived from the moved poses. A locked object never follows, and neither does anything hanging off it. A
    scaling moves no pose: nothing follows, nothing re-derives."""
    targets = [entry for entry in addressed(document, kind, payload)]
    volumes = {at: dict(document["targetVolumes"][at], **{SELECTION[kind]: transformed(kind, payload, document["targetVolumes"][at])}) for member, at in targets if member == "targetVolumes"}
    objects, queue = {}, []
    for identifier in payload["targets"]:
        for member, at in targets:
            if member == "objects" and document["objects"][at]["id"] == identifier and at not in objects:
                objects[at] = dict(document["objects"][at], **{SELECTION[kind]: transformed(kind, payload, document["objects"][at])})
                queue.append(at)
    if kind == "scale-selection" or not objects:
        return objects, volumes, []
    port = {"%s:%s" % (record["id"], vortex["id"]): (at, index) for at, record in enumerate(document["objects"]) for index, vortex in enumerate(record["vortices"])}
    ends = [(port.get(held["attracting"]), port.get(held["attracted"])) for held in document["attractions"]]
    ends = [(a, b) if a is not None and b is not None and a[0] != b[0] else None for a, b in ends]
    placing = set()
    while queue:
        parent = queue.pop(0)
        for index, end in enumerate(ends):
            if end is None or end[0][0] != parent:
                continue
            (_, port_a), (child, port_b) = end
            if child in objects or document["objects"][child]["locked"]:
                continue
            source, held, target = objects[parent], document["attractions"][index], document["objects"][child]
            vortex_a, vortex_b = source["vortices"][port_a], target["vortices"][port_b]
            origin, orientation = placement_of(source["origin"], source.get("orientation") or IDENTITY, vortex_a["position"], vortex_a.get("direction") or [0.0, 0.0, -1.0], vortex_b["position"], vortex_b.get("direction") or [0.0, 0.0, -1.0], [held[member] for member in ("gap", "shift", "rise", "rotation", "turn", "tilt")])
            objects[child] = dict(target, origin=origin, orientation=orientation)
            placing.add(index)
            queue.append(child)
    pose = lambda at: objects.get(at, document["objects"][at])
    rederived = []
    for index, (held, end) in enumerate(zip(document["attractions"], ends)):
        if end is None or index in placing:
            continue
        (a, port_a), (b, port_b) = end
        if a not in objects and b not in objects:
            continue
        source, target = pose(a), pose(b)
        vortex_a, vortex_b = source["vortices"][port_a], target["vortices"][port_b]
        params = connection_of(source["origin"], source.get("orientation") or IDENTITY, vortex_a["position"], vortex_a.get("direction") or [0.0, 0.0, -1.0], vortex_b["position"], vortex_b.get("direction") or [0.0, 0.0, -1.0], target["origin"], target.get("orientation") or IDENTITY)
        updated = dict(held, **dict(zip(("gap", "shift", "rise", "rotation", "turn", "tilt"), params)))
        if updated != held:
            rederived.append(updated)
    return objects, volumes, rederived
# endregion 🔖️Selection


# region 🔖️ConnectionPose
def rotated(quat, vector):
    """🌐️ `vector` turned by the unit quaternion `quat`."""
    x, y, z, w = quat
    vx, vy, vz = vector
    ix = w * vx + y * vz - z * vy
    iy = w * vy + z * vx - x * vz
    iz = w * vz + x * vy - y * vx
    iw = -x * vx - y * vy - z * vz
    return [ix * w + iw * -x + iy * -z - iz * -y, iy * w + iw * -y + iz * -x - ix * -z, iz * w + iw * -z + ix * -y - iy * -x]


def unit(vector):
    """🎯️ `vector` scaled to unit length (a near-zero vector stays)."""
    length = math.sqrt(vector[0] * vector[0] + vector[1] * vector[1] + vector[2] * vector[2])
    return vector if length < 1e-12 else [vector[0] * (1.0 / length), vector[1] * (1.0 / length), vector[2] * (1.0 / length)]


def unit_quaternion(q):
    """🧼️ `q` scaled to unit length; a degenerate quaternion is no turn."""
    length = math.sqrt(q[0] * q[0] + q[1] * q[1] + q[2] * q[2] + q[3] * q[3])
    return list(IDENTITY) if length < 1e-12 else [q[0] / length, q[1] / length, q[2] / length, q[3] / length]


def cross(a, b):
    return [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]]


def dot(a, b):
    return a[0] * b[0] + a[1] * b[1] + a[2] * b[2]


def between(source, target):
    """🧭️ The quaternion turning unit `source` onto unit `target`."""
    r = dot(source, target) + 1.0
    if r < 0.000001:
        quat = [-source[1], source[0], 0.0, 0.0] if abs(source[0]) > abs(source[2]) else [0.0, -source[2], source[1], 0.0]
    else:
        c = cross(source, target)
        quat = [c[0], c[1], c[2], r]
    return unit_quaternion(quat)


def aligned(parent, child):
    """🧲️ The turn that sets the attracted port against the attracting one (with the compose kernel's
    (anti)parallel special cases)."""
    reverse = [child[0] * -1.0, child[1] * -1.0, child[2] * -1.0]
    if math.sqrt(dot(cross(parent, reverse), cross(parent, reverse))) < 0.01:
        if abs(parent[2]) < 0.01:
            return between([0.0, 1.0, 0.0], [0.0, 0.0, -1.0])
        axis = cross([0.0, 0.0, 1.0], parent)
        if math.sqrt(dot(axis, axis)) < 1e-9:
            axis = cross([1.0, 0.0, 0.0], parent)
        axis = unit(axis)
        half = math.pi / 2
        return unit_quaternion([axis[0] * math.sin(half), axis[1] * math.sin(half), axis[2] * math.sin(half), math.cos(half)])
    return between(reverse, parent)


def placement_of(t_a, q_a, p_a, d_a, p_b, d_b, params):
    """📐️ The attracted pose `(origin, orientation)` the six connection parameters place against the attracting pose:
    the port alignment, then rotation about the attracting port and turn and tilt about the turned rise and shift
    axes, applied to the attracting port offset by gap, shift and rise."""
    gap, shift, rise, rotation, turn, tilt = params
    conjugate = lambda q: [-q[0], -q[1], -q[2], q[3]]
    radians = lambda deg: deg * math.pi / 180.0
    parent, child = unit(d_a), unit(d_b)
    align = aligned(parent, child)
    frame = between([0.0, 1.0, 0.0], parent)
    gap_dir, shift_dir, raise_dir = rotated(frame, [0.0, 1.0, 0.0]), rotated(frame, [1.0, 0.0, 0.0]), rotated(frame, [0.0, 0.0, 1.0])
    spin = axis_angle(parent, -radians(rotation))
    turn_axis, tilt_axis = rotated(spin, raise_dir), rotated(spin, shift_dir)
    local = conjugate(align)
    for step in (spin, axis_angle(turn_axis, radians(turn)), axis_angle(tilt_axis, radians(tilt))):
        local = hamilton(local, conjugate(step))
    local = unit_quaternion(local)
    reach = [gap_dir[axis] * gap for axis in range(3)], [shift_dir[axis] * shift for axis in range(3)], [raise_dir[axis] * rise for axis in range(3)]
    offset = [(t_a[axis] + p_a[axis]) + ((reach[0][axis] + reach[1][axis]) + reach[2][axis]) for axis in range(3)]
    landed = rotated(local, offset)
    return [landed[axis] - p_b[axis] for axis in range(3)], unit_quaternion(hamilton(local, q_a))


def connection_of(t_a, q_a, p_a, d_a, p_b, d_b, t_b, q_b):
    """🧮️ The six connection parameters that place the attracted pose `(t_b, q_b)` against the attracting one."""
    conjugate = lambda q: [-q[0], -q[1], -q[2], q[3]]
    parent, child = unit(d_a), unit(d_b)
    align = aligned(parent, child)
    frame = between([0.0, 1.0, 0.0], parent)
    gap_dir, shift_dir, raise_dir = rotated(frame, [0.0, 1.0, 0.0]), rotated(frame, [1.0, 0.0, 0.0]), rotated(frame, [0.0, 0.0, 1.0])
    local = unit_quaternion(hamilton(q_b, conjugate(q_a)))
    offset = rotated(conjugate(local), [t_b[0] + p_b[0], t_b[1] + p_b[1], t_b[2] + p_b[2]])
    diff = [(offset[axis] - t_a[axis]) - p_a[axis] for axis in range(3)]
    m = hamilton(hamilton(conjugate(frame), hamilton(align, local)), frame)
    col_x, col_y = rotated(m, [1.0, 0.0, 0.0]), rotated(m, [0.0, 1.0, 0.0])
    tilt = -math.asin(max(-1.0, min(1.0, col_y[2])))
    if abs(abs(col_y[2]) - 1.0) < 1e-6:
        rotation, turn = math.atan2(col_x[1], col_x[0]), 0.0
    else:
        col_z = rotated(m, [0.0, 0.0, 1.0])
        rotation, turn = math.atan2(-col_x[2], col_z[2]), math.atan2(col_y[0], col_y[1])
    degrees = lambda rad: rad * 180.0 / math.pi
    return dot(diff, gap_dir), dot(diff, shift_dir), dot(diff, raise_dir), degrees(rotation), degrees(turn), degrees(tilt)
# endregion 🔖️ConnectionPose


# region 🔖️Verbs
def put(record, member, value):
    """✏️ Writes one member; an absent optional member is removed, never stored as `null`."""
    if value is None:
        record.pop(member, None)
    else:
        record[member] = value


def apply_mutation(document, kind, payload):
    """🦠️ Applies one kind. Every committed vector of this subset is accepted (`applied` or `no-op`), so an
    address the scene does not hold is an error rather than a rejection outcome."""
    if kind in UNDERDETERMINED:
        raise AssertionError("mutate-%s: %s" % (kind, UNDERDETERMINED_REASON))
    document = copy.deepcopy(document)
    if kind in COLLECTIONS:
        member, argument = COLLECTIONS[kind]
        index = payload.get("index")
        document[member].insert(len(document[member]) if index is None else index, copy.deepcopy(payload[argument]))
    elif kind in REMOVALS:
        member = REMOVALS[kind]
        at = record_at(document, member, payload["id"], kind, "mutate")
        if member == "objects":
            severed = ports_of(document["objects"][at])
            document["attractions"] = [held for held in document["attractions"] if held["attracting"] not in severed and held["attracted"] not in severed]
        document[member].pop(at)
    elif kind in OBJECT_FIELDS:
        member, argument = OBJECT_FIELDS[kind]
        put(document["objects"][record_at(document, "objects", payload["id"], kind, "mutate")], member, copy.deepcopy(payload[argument]))
    elif kind in VOLUME_FIELDS:
        member, argument = VOLUME_FIELDS[kind]
        put(document["targetVolumes"][record_at(document, "targetVolumes", payload["id"], kind, "mutate")], member, copy.deepcopy(payload[argument]))
    elif kind in SELECTION:
        objects, volumes, rederived = selection_effect(document, kind, payload)
        for at, record in objects.items():
            document["objects"][at] = record
        for at, record in volumes.items():
            document["targetVolumes"][at] = record
        for updated in rederived:
            document["attractions"][record_at(document, "attractions", updated["id"], kind, "mutate")] = updated
    elif kind in REFERENCE_FIELDS:
        member, argument = REFERENCE_FIELDS[kind]
        document["references"][record_at(document, "references", payload["id"], kind, "mutate")][member] = copy.deepcopy(payload[argument])
    elif kind == "add-object-vortex":
        record = document["objects"][record_at(document, "objects", payload["objectId"], kind, "mutate")]
        index = payload.get("index")
        record["vortices"].insert(len(record["vortices"]) if index is None else index, copy.deepcopy(payload["vortex"]))
    elif kind == "remove-object-vortex":
        record = document["objects"][record_at(document, "objects", payload["objectId"], kind, "mutate")]
        if not any(vortex["id"] == payload["vortexId"] for vortex in record["vortices"]):
            raise AssertionError("mutate-%s: object %r declares no vortex %r" % (kind, payload["objectId"], payload["vortexId"]))
        port = "%s:%s" % (payload["objectId"], payload["vortexId"])
        record["vortices"] = [vortex for vortex in record["vortices"] if vortex["id"] != payload["vortexId"]]
        document["attractions"] = [held for held in document["attractions"] if held["attracting"] != port and held["attracted"] != port]
    elif kind == "connect-vortices":
        attraction = {"id": payload["id"], "attracting": payload["attracting"], "attracted": payload["attracted"]}
        for member in ATTRACTION_GEOMETRY:
            attraction[member] = payload[member]
        document["attractions"].append(attraction)
    elif kind == "disconnect-vortices":
        document["attractions"].pop(record_at(document, "attractions", payload["id"], kind, "mutate"))
    elif kind == "replace-attraction-geometry":
        attraction = document["attractions"][record_at(document, "attractions", payload["id"], kind, "mutate")]
        for member in ATTRACTION_GEOMETRY:
            attraction[member] = payload["new" + member[:1].upper() + member[1:]]
    elif kind == "change-domain":
        document["domain"] = payload["newDomain"]
    elif kind == "connect-kind-compatibility":
        document["meta"]["kindCompatibility"].append({member: payload[member] for member in COMPATIBILITY_FIELDS})
    elif kind == "disconnect-kind-compatibility":
        held = [rule for rule in document["meta"]["kindCompatibility"] if rule["source"] == payload["source"] and rule["target"] == payload["target"]]
        if not held:
            raise AssertionError("mutate-%s: the relation declares no %r to %r rule" % (kind, payload["source"], payload["target"]))
        document["meta"]["kindCompatibility"] = [rule for rule in document["meta"]["kindCompatibility"] if rule not in held]
    elif kind == "replace-kind-catalogs":
        document["meta"]["kindCatalogs"] = copy.deepcopy(payload["newCatalogs"])
    else:
        raise AssertionError("mutate-%s: this implementation declares no verb for that kind" % kind)
    return document


def reconnect(attraction):
    """🔗 The `connect-vortices` arguments that rebuild one attraction exactly as it stands."""
    payload = {"id": attraction["id"], "attracting": attraction["attracting"], "attracted": attraction["attracted"]}
    for member in ATTRACTION_GEOMETRY:
        payload[member] = attraction[member]
    return payload


def inverse_mutation(document, kind, payload):
    """↩️ The kind's OWN inverse, expressed in this same closed vocabulary, computed against the
    pre-mutation scene. `delete-object` and `remove-object-vortex` invert to SEVERAL steps, because
    they sever attractions: the object or vortex is put back at its own index first and every severed
    attraction is reconnected after it, in scene order."""
    if kind in UNDERDETERMINED:
        raise AssertionError("inverse-%s: %s" % (kind, UNDERDETERMINED_REASON))
    if kind in COLLECTIONS:
        member, argument = COLLECTIONS[kind]
        undo = {"objects": "delete-object", "targetVolumes": "delete-target-volume", "references": "delete-reference"}[member]
        return [(undo, {"id": payload[argument]["id"]})]
    if kind in REMOVALS:
        member = REMOVALS[kind]
        at = record_at(document, member, payload["id"], kind, "inverse")
        record = document[member][at]
        redo = {"objects": ("create-object", "object"), "targetVolumes": ("create-target-volume", "targetVolume"), "references": ("create-reference", "reference")}[member]
        steps = [(redo[0], {redo[1]: copy.deepcopy(record), "index": at})]
        if member == "objects":
            steps += [("connect-vortices", reconnect(held)) for held in attached_to(document, ports_of(record))]
        return steps
    if kind in OBJECT_FIELDS:
        member, argument = OBJECT_FIELDS[kind]
        return [(kind, {"id": payload["id"], argument: copy.deepcopy(document["objects"][record_at(document, "objects", payload["id"], kind, "inverse")][member])})]
    if kind in VOLUME_FIELDS:
        member, argument = VOLUME_FIELDS[kind]
        return [(kind, {"id": payload["id"], argument: copy.deepcopy(document["targetVolumes"][record_at(document, "targetVolumes", payload["id"], kind, "inverse")][member])})]
    if kind in REFERENCE_FIELDS:
        member, argument = REFERENCE_FIELDS[kind]
        return [(kind, {"id": payload["id"], argument: copy.deepcopy(document["references"][record_at(document, "references", payload["id"], kind, "inverse")][member])})]
    if kind in SELECTION:
        objects, volumes, rederived = selection_effect(document, kind, payload)
        steps = []
        for member, changed in (("objects", objects), ("targetVolumes", volumes)):
            for at in sorted(changed):
                for pose in ("origin", "orientation", "scale"):
                    if changed[at].get(pose) != document[member][at].get(pose):
                        setter, argument = SETTERS[(member, pose)]
                        steps.append((setter, {"id": document[member][at]["id"], argument: copy.deepcopy(document[member][at].get(pose))}))
        for updated in rederived:
            held = document["attractions"][record_at(document, "attractions", updated["id"], kind, "inverse")]
            steps.append(("replace-attraction-geometry", dict({"id": held["id"]}, **{"new" + member[:1].upper() + member[1:]: held[member] for member in ATTRACTION_GEOMETRY})))
        return steps
    if kind == "add-object-vortex":
        return [("remove-object-vortex", {"objectId": payload["objectId"], "vortexId": payload["vortex"]["id"]})]
    if kind == "remove-object-vortex":
        record = document["objects"][record_at(document, "objects", payload["objectId"], kind, "inverse")]
        at = next(index for index, vortex in enumerate(record["vortices"]) if vortex["id"] == payload["vortexId"])
        port = "%s:%s" % (payload["objectId"], payload["vortexId"])
        steps = [("add-object-vortex", {"objectId": payload["objectId"], "vortex": copy.deepcopy(record["vortices"][at]), "index": at})]
        return steps + [("connect-vortices", reconnect(held)) for held in attached_to(document, {port})]
    if kind == "connect-vortices":
        return [("disconnect-vortices", {"id": payload["id"]})]
    if kind == "disconnect-vortices":
        return [("connect-vortices", reconnect(document["attractions"][record_at(document, "attractions", payload["id"], kind, "inverse")]))]
    if kind == "replace-attraction-geometry":
        attraction = document["attractions"][record_at(document, "attractions", payload["id"], kind, "inverse")]
        return [(kind, dict({"id": payload["id"]}, **{"new" + member[:1].upper() + member[1:]: attraction[member] for member in ATTRACTION_GEOMETRY}))]
    if kind == "change-domain":
        return [(kind, {"newDomain": document["domain"]})]
    if kind == "connect-kind-compatibility":
        return [("disconnect-kind-compatibility", {"source": payload["source"], "target": payload["target"]})]
    if kind == "disconnect-kind-compatibility":
        held = next(rule for rule in document["meta"]["kindCompatibility"] if rule["source"] == payload["source"] and rule["target"] == payload["target"])
        return [("connect-kind-compatibility", copy.deepcopy(held))]
    if kind == "replace-kind-catalogs":
        held = document["meta"].get("kindCatalogs")
        if held is None:
            raise AssertionError(
                "inverse-%s: this implementation refuses to guess this inverse. The committed vector INSTALLS a catalogue where the before-snapshot "
                "carried none, so undoing it requires REMOVING the member — and no verb in this closed vocabulary can express that. The sibling "
                "`mutate-puzzle-5d-1` commits the deciding evidence: its `null-catalogs-is-noop` vector shows that `replace-kind-catalogs` with a "
                "NULL argument is accepted and is a NO-OP, not a removal. So the gap is in the vocabulary, not in this implementation, and it is "
                "invisible to the subject half of this case, which asserts only that the committed diff and the committed snapshots agree on a "
                "footprint and never applies an inverse at all." % kind
            )
        return [(kind, {"newCatalogs": copy.deepcopy(held)})]
    raise AssertionError("inverse-%s: this implementation declares no inverse for that kind" % kind)
# endregion 🔖️Verbs


# region 🔖️Laws
def equals_committed(kind, produced, committed):
    """🎯️ The committed after-snapshot claim, member by member, with no tolerance and no ignored key."""
    for member in MEMBERS:
        if produced[member] != committed[member]:
            raise AssertionError("mutate-%s: %s is %s, the committed after-snapshot says %s" % (kind, member, json.dumps(produced[member], sort_keys=True)[:400], json.dumps(committed[member], sort_keys=True)[:400]))


def observable(kind, before, after, no_op):
    """👁️ A vector whose committed outcome does NOT declare `mutation.no-op` must move the compared
    projection; one that does must move nothing. The exemption is read off the committed outcome."""
    if no_op and before != after:
        raise AssertionError("mutate-%s: the committed outcome declares mutation.no-op, yet the scene moved" % kind)
    if not no_op and before == after:
        raise AssertionError("mutate-%s: the committed vector declares this kind applied, yet the scene did not move" % kind)


def restores(kind, restored, original):
    """↩️ The full inverse law: applying the kind and then its OWN computed inverse must land back on
    the committed before-snapshot, member for member and index for index."""
    for member in MEMBERS:
        if restored[member] != original[member]:
            raise AssertionError("inverse-%s: %s came back as %s, not %s" % (kind, member, json.dumps(restored[member], sort_keys=True)[:400], json.dumps(original[member], sort_keys=True)[:400]))
# endregion 🔖️Laws


# region 🔖️Plan
def doc_json(ctx):
    """📜️ The scenario's doc string — the Python `Context` has no accessor of its own."""
    for step in ctx.scenario["steps"]:
        if step.get("docString"):
            return json.loads(step["docString"])
    raise AssertionError("scenario %s carries no doc string" % ctx.scenario["id"])


def leaf(ctx, spec, name):
    """🧫️ One committed leaf of the vector the doc string addresses."""
    return json.loads(ctx.input_bytes(spec[name]).decode("utf-8"))


def uri_in(ctx, needle):
    """🧫️ The one declared fixture URI of this scenario's steps containing `needle`."""
    for step in ctx.scenario["steps"]:
        for token in step["text"].split():
            if token.startswith(("asset://", "shared://")) and needle in token:
                return token
    raise AssertionError("scenario %s declares no fixture URI containing %r" % (ctx.scenario["id"], needle))


def payload_of(spec_mutation, kind):
    """🦠️ The committed payload, checked to carry this kind's own internally tagged discriminator."""
    if spec_mutation.get("mutation") != TAGS[kind]:
        raise AssertionError("mutate-%s: the committed vector carries a %r payload" % (kind, spec_mutation.get("mutation")))
    return {key: value for key, value in spec_mutation.items() if key != "mutation"}


def declares_no_op(outcome):
    """🚦️ Whether the committed outcome itself records that the mutation had nothing to do."""
    return any(message.get("code") == "mutation.no-op" for message in outcome.get("messages", []))


def outcome_of(payload):
    """📤️ Wraps a projection with its own compact serialization as the raw artifact."""
    return Outcome(payload, raw=json.dumps(payload, separators=(",", ":"), ensure_ascii=False).encode("utf-8"))
# endregion 🔖️Plan


# region 🔖️Handlers
def mutate_handler(kind):
    """🎯️ Applies one kind to its committed before-snapshot and asserts, in role, the committed
    after-snapshot and the observability the committed outcome implies."""

    def handler(ctx):
        spec = doc_json(ctx)
        if spec["kind"] != kind:
            raise AssertionError("mutate-%s: the feature's doc string states %r" % (kind, spec["kind"]))
        before = leaf(ctx, spec, "before")
        after = leaf(ctx, spec, "after")
        outcome = leaf(ctx, spec, "outcome")
        validate(before, "mutate-%s" % kind)
        applied = apply_mutation(before, kind, payload_of(leaf(ctx, spec, "mutation"), kind))
        validate(applied, "mutate-%s" % kind)
        equals_committed(kind, applied, after)
        observable(kind, before, applied, declares_no_op(outcome))
        return outcome_of(applied)

    return handler


def inverse_handler(kind):
    """↩️ Applies one kind and then its OWN computed inverse and requires the committed before-snapshot
    back — the full inverse law, which the subject half of this case cannot assert because it never
    applies anything."""

    def handler(ctx):
        spec = doc_json(ctx)
        if spec["kind"] != kind:
            raise AssertionError("inverse-%s: the feature's doc string states %r" % (kind, spec["kind"]))
        before = leaf(ctx, spec, "before")
        payload = payload_of(leaf(ctx, spec, "mutation"), kind)
        validate(before, "inverse-%s" % kind)
        current = apply_mutation(before, kind, payload)
        for step_kind, step_payload in inverse_mutation(before, kind, payload):
            current = apply_mutation(current, step_kind, step_payload)
        restores(kind, current, before)
        return outcome_of(current)

    return handler


def identity_handler(ctx):
    """🔁️ Reads the committed scene and answers with the whole document. This implementation
    additionally requires, in role, that it really is this scene and not a graph: two objects owning
    vortices, an attraction whose two endpoints are `"<objectId>:<vortexId>"` ports rather than object
    ids, a target volume and an image reference."""
    uri = uri_in(ctx, "⬅️before")
    committed = ctx.input_bytes(uri)
    document = json.loads(committed.decode("utf-8"))
    validate(document, "identity-round-trip")
    object_ids = {record["id"] for record in document["objects"]}
    if len(document["objects"]) < 2 or not document["attractions"] or not document["targetVolumes"] or not document["references"]:
        raise AssertionError("identity-round-trip: the committed scene must carry two objects, an attraction, a target volume and a reference")
    for attraction in document["attractions"]:
        for end in ("attracting", "attracted"):
            if ":" not in attraction[end] or attraction[end] in object_ids:
                raise AssertionError("identity-round-trip: attraction %r names %r as its %s; in this scene an attraction joins two \"<objectId>:<vortexId>\" ports" % (attraction["id"], attraction[end], end))
    reserialized = json.dumps(document, separators=(",", ":"), ensure_ascii=False).encode("utf-8")
    if reserialized == committed:
        raise AssertionError("identity-round-trip: the committed file is pretty-printed and this writer is compact, so reproducing its bytes exactly would mean the handler returned the input unread")
    reparsed = json.loads(reserialized.decode("utf-8"))
    if reparsed != document:
        raise AssertionError("identity-round-trip: serializing and re-reading the document moved it")
    return Outcome(reparsed, raw=reserialized)
# endregion 🔖️Handlers


# region 🔖️Registration
def adapter():
    """🧭️ Registration by FULL expanded scenario id, in the ORACLE role only — registering these
    handlers as subjects too would make the reference its own subject and manufacture a green
    self-comparison."""
    built = Adapter("python")
    for kind in KINDS:
        built = built.oracle("mutate-%s" % kind, mutate_handler(kind))
        built = built.oracle("inverse-%s" % kind, inverse_handler(kind))
    return built.oracle("identity-round-trip", identity_handler)
# endregion 🔖️Registration

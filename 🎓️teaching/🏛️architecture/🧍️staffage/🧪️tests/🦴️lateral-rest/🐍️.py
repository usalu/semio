"""🦴️ Oracle of the lateral rest placement: scipy `Rotation` solves the bone chain in three dimensions from the rest placement (`angleDeg`, `abductionDeg`, `twistDeg`) and the offsets of every bone, numpy carries points, shapely measures the projected solids.

A bone frame has the columns (u, v, w) in the world axes (ahead, up, left). `angleDeg` turns about w (u towards v), `abductionDeg` about v (u towards w), `twistDeg` about u (v towards -w): in the right-handed local axes (x, y, z) = (u, v, w) these are the intrinsic rotations
Z(angle) · Y(-abduction) · X(-twist). The local rotation of a bone is rest · offset, its origin `parent.o + parent.R · (x + dx, y + dy, z)`.

Input: the path of a JSON document `{rig: [bones], root, cases: {name: offsets}, solids: {view: [shapes]}, anchors: [anchors]}`.
Output: JSON `{case: {"frames": {bone: {origin, tip, u, v, w}}, "placements": {bone: {translation, rotation}}, "extents": {"view/shape": {minX, maxX, minY, maxY}}, "anchors": {id: [x, y, z]}, "joints": {bone: {moment, torsionNm, momentNm, momentLateralNm, axialN, shearN, shearZN}}}}` (`joints`: for every bone named in the optional input list `joints`, numpy sums the weights `mass.kg · gravityMS2` of the bones of its subtree at their centres `(mass.comX, mass.comY)` and takes the moment `r × F` about the joint and its parts about u, w and v; `axialN` is the part of the summed weight along u (tension +), `shearN` and `shearZN` minus its parts along v and w); a placement is the local transform in the neutral model frame (a point (x, y, z) is (z, y, x) there and the node basis is (u, v, -w)), its rotation the quaternion [x, y, z, w] with w >= 0.
"""
import json
import sys

import numpy as np
from scipy.spatial.transform import Rotation
from shapely.geometry import MultiPoint


def turn(angle, abduction, twist):
    return Rotation.from_euler("ZYX", [angle, -abduction, -twist], degrees=True).as_matrix()


def solve(bones, offsets, root):
    frames = {}
    for bone in bones:
        offset = offsets.get(bone["id"], {})
        if bone["parent"] is None:
            origin, basis = np.array([root["x"], root["y"], 0.0]), turn(root["angleDeg"], 0.0, 0.0)
        else:
            origin, basis = frames[bone["parent"]]
        here = origin + basis @ np.array([bone["x"] + offset.get("dx", 0.0), bone["y"] + offset.get("dy", 0.0), bone.get("z", 0.0)])
        rest = turn(bone["angleDeg"], bone.get("abductionDeg", 0.0), bone.get("twistDeg", 0.0))
        moved = turn(offset.get("angleDeg", 0.0), offset.get("abductionDeg", 0.0), offset.get("twistDeg", 0.0))
        frames[bone["id"]] = (here, basis @ rest @ moved)
    return frames


def neutral(basis):
    swap = np.array([[0.0, 0.0, 1.0], [0.0, 1.0, 0.0], [1.0, 0.0, 0.0]])
    return swap @ basis @ np.diag([1.0, 1.0, -1.0])


def placements(bones, frames):
    out = {}
    swap = np.array([[0.0, 0.0, 1.0], [0.0, 1.0, 0.0], [1.0, 0.0, 0.0]])
    for bone in bones:
        origin, basis = frames[bone["id"]]
        here = neutral(basis)
        if bone["parent"] is None:
            translation, local = swap @ origin, here
        else:
            parent_origin, parent_basis = frames[bone["parent"]]
            parent = neutral(parent_basis)
            translation, local = parent.T @ (swap @ (origin - parent_origin)), parent.T @ here
        quaternion = Rotation.from_matrix(local).as_quat()
        out[bone["id"]] = {"translation": translation.tolist(), "rotation": (quaternion if quaternion[3] >= 0 else -quaternion).tolist()}
    return out


def project(view, q):
    return (q[0], q[2]) if view == "plan" else (q[0], q[1]) if view == "elevation" else (q[2], q[1])


def extent(view, shape, frames):
    solid = shape["solid"]
    origin, basis = frames[shape["bone"]]
    corners = [(shape["x"], shape["y"]), (shape["x"] + shape["w"], shape["y"]), (shape["x"] + shape["w"], shape["y"] + shape["h"]), (shape["x"], shape["y"] + shape["h"])]
    points = []
    for c in (solid["z0"], solid["z1"]):
        for a, b in corners:
            local = {"u": [c, a, b], "v": [a, c, b]}[solid["sweep"]]
            points.append(project(view, origin + basis @ np.array(local)))
    min_x, min_y, max_x, max_y = MultiPoint(points).convex_hull.bounds
    return {"minX": min_x, "maxX": max_x, "minY": min_y, "maxY": max_y}


def joint_loads(bones, frames, joints, gravity):
    out = {}
    parents = {bone["id"]: bone["parent"] for bone in bones}
    for joint in joints:
        origin, basis = frames[joint]
        force, moment = np.zeros(3), np.zeros(3)
        for bone in bones:
            chain, at = [], bone["id"]
            while at is not None:
                chain.append(at)
                at = parents[at]
            if joint not in chain or "mass" not in bone:
                continue
            here, axes = frames[bone["id"]]
            weight = np.array([0.0, -bone["mass"]["kg"] * gravity, 0.0])
            force += weight
            moment += np.cross(here + axes @ np.array([bone["mass"]["comX"], bone["mass"]["comY"], 0.0]) - origin, weight)
        out[joint] = {"moment": moment.tolist(), "torsionNm": float(moment @ basis[:, 0]), "momentNm": float(moment @ basis[:, 2]), "momentLateralNm": float(moment @ basis[:, 1]), "axialN": float(force @ basis[:, 0]), "shearN": float(-force @ basis[:, 1]), "shearZN": float(-force @ basis[:, 2])}
    return out


def main():
    document = json.load(open(sys.argv[1], encoding="utf-8"))
    bones = document["rig"]
    result = {}
    for name, offsets in document["cases"].items():
        frames = solve(bones, offsets, document["root"])
        lengths = {bone["id"]: bone["lengthM"] for bone in bones}
        result[name] = {
            "frames": {id_: {"origin": o.tolist(), "tip": (o + b[:, 0] * lengths[id_]).tolist(), "u": b[:, 0].tolist(), "v": b[:, 1].tolist(), "w": b[:, 2].tolist()} for id_, (o, b) in frames.items()},
            "placements": placements(bones, frames),
            "extents": {f"{view}/{shape['id']}": extent(view, shape, frames) for view, shapes in document["solids"].items() for shape in shapes},
            "anchors": {a["id"]: (frames[a["bone"]][0] + frames[a["bone"]][1] @ np.array([a["x"], a["y"], a.get("z", 0.0)])).tolist() for a in document["anchors"]},
            "joints": joint_loads(bones, frames, document.get("joints", []), document.get("gravityMS2", 9.80665)),
        }
    json.dump(result, sys.stdout)


main()

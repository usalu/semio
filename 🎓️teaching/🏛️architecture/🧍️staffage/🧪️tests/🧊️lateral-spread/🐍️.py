"""🧊️ Oracle of the lateral spread and the axial twist (shared by 🧪️tests/🧊️axial-twist): numpy rotation matrices solve the bone chain in three dimensions (a bone turns out of the sagittal plane about its normal; the side view is the projection of the chain), shapely measures the stretched front and plan rectangles.

Input: the path of a JSON document written by the test: `rig` (bones with parent, x, y, angleDeg, lengthM), `cases` (name, offsets per bone, root, shapes per view, standing travel `lift` of the sagittal pose, rest travel).
Output: JSON `{case: {"bones": {id: {"origin": [x, y, z], "tip": [x, y, z]}} (absolute positions of the spread chain), "extents": {"view/shape": {minX, maxX, minY, maxY}}}}`.
"""
import json
import math
import sys

import numpy as np
from shapely.geometry import MultiPoint, Polygon
from shapely.ops import unary_union


def rz(deg):
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return np.array([[c, -s, 0.0], [s, c, 0.0], [0.0, 0.0, 1.0]])


def ry(deg):
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return np.array([[c, 0.0, -s], [0.0, 1.0, 0.0], [s, 0.0, c]])


def rx(deg):
    c, s = math.cos(math.radians(deg)), math.sin(math.radians(deg))
    return np.array([[1.0, 0.0, 0.0], [0.0, c, s], [0.0, -s, c]])


def solve(bones, offsets, root, spread):
    frames = {}
    for bone in bones:
        offset = offsets.get(bone["id"], {})
        parent = frames.get(bone["parent"]) if bone["parent"] else (np.array([root["x"], root["y"], 0.0]), rz(root["angleDeg"]))
        origin = parent[0] + parent[1] @ np.array([bone["x"] + offset.get("dx", 0.0), bone["y"] + offset.get("dy", 0.0), bone.get("z", 0.0)])
        rotation = parent[1] @ rz(bone["angleDeg"] + offset.get("angleDeg", 0.0))
        if spread:
            rotation = rotation @ ry(offset.get("abductionDeg", 0.0)) @ rx(offset.get("twistDeg", 0.0))
        frames[bone["id"]] = (origin, rotation, origin + rotation @ np.array([bone["lengthM"], 0.0, 0.0]))
    return frames


def positions(case, rig):
    spread = solve(rig, case["offsets"], case["root"], True)
    return {id_: {"origin": spread[id_][0].tolist(), "tip": spread[id_][2].tolist(), "rotation": spread[id_][1].tolist()} for id_ in spread}


def affected(rig, offsets):
    result = {}
    for bone in rig:
        offset = offsets.get(bone["id"], {})
        own = offset.get("abductionDeg", 0.0) != 0.0 or offset.get("twistDeg", 0.0) != 0.0
        result[bone["id"]] = own or (bool(bone["parent"]) and result[bone["parent"]])
    return result


def carry(rest, now, point):
    local = np.array(rest["rotation"]).T @ (np.array(point) - np.array(rest["origin"]))
    return (np.array(now["origin"]) + np.array(now["rotation"]) @ local).tolist()


def project(view, q):
    return (q[0], q[2]) if view == "plan" else (q[0], q[1]) if view == "elevation" else (q[2], q[1])


def solid_geometry(view, shape, rig, rest, now):
    solid = shape["solid"]
    bone = shape["bone"]
    corners = [tuple(point) for point in shape["points"]] if shape["kind"] == "poly" else [(shape["x"], shape["y"]), (shape["x"] + shape["w"], shape["y"]), (shape["x"] + shape["w"], shape["y"] + shape["h"]), (shape["x"], shape["y"] + shape["h"])]
    origin = np.array(now[bone]["origin"])
    rotation = np.array(now[bone]["rotation"])

    def place(a, b, c):
        return origin + rotation @ np.array([a, b, c])

    if solid["kind"] == "extruded":
        rest_z = rest[bone]["origin"][2]
        low = [project(view, place(a, b, solid["z0"] - rest_z)) for a, b in corners]
        high = [project(view, place(a, b, solid["z1"] - rest_z)) for a, b in corners]
        parts = [Polygon(low), Polygon(high)] + [Polygon([low[i], low[(i + 1) % len(low)], high[(i + 1) % len(high)], high[i]]) for i in range(len(low))]
        return unary_union([part.buffer(0) for part in parts if part.area > 1e-12])
    axis_bone = solid["axisBone"]
    axis_rest = rest[axis_bone]
    axis_now = now[axis_bone]
    rest_rotation = np.array(axis_rest["rotation"])
    now_rotation = np.array(axis_now["rotation"])
    base = np.array(axis_now["origin"]) + now_rotation @ rest_rotation.T @ np.array([solid["axisX"], 0.0, solid.get("axisZ", 0.0)])
    axis = now_rotation[:, 0]
    reference = np.array([0.0, 0.0, 1.0]) if abs(axis[2]) < 0.9 else np.array([1.0, 0.0, 0.0])
    e1 = np.cross(axis, reference)
    e1 /= np.linalg.norm(e1)
    e2 = np.cross(axis, e1)
    circles = []
    for a, b in corners:
        q = place(a, b, axis_rest["origin"][2] + solid.get("axisZ", 0.0) - rest[bone]["origin"][2]) - base
        h = float(q @ axis)
        r = float(np.linalg.norm(q - h * axis))
        circles.append([project(view, base + h * axis + r * (math.cos(t) * e1 + math.sin(t) * e2)) for t in np.linspace(0.0, 2.0 * math.pi, 361)])
    return unary_union([MultiPoint(circles[i] + circles[(i + 1) % len(circles)]).convex_hull for i in range(len(circles))])


def lateral_fraction(points, outward):
    low, high = min(points), max(points)
    return lambda value: 0.0 if outward == 0 else 1.0 if high == low else (value - low) / (high - low) if outward > 0 else (high - value) / (high - low)


def slice_depth(view, shapes, rest_bone):
    rings = []
    for t in shapes:
        corners = [tuple(point) for point in t["points"]] if t["kind"] == "poly" else [(t["x"], t["y"]), (t["x"] + t["w"], t["y"]), (t["x"] + t["w"], t["y"] + t["h"]), (t["x"], t["y"] + t["h"])]
        rings.append([np.array(rest_bone["origin"]) + np.array(rest_bone["rotation"]) @ np.array([a, b, 0.0]) for a, b in corners])
    plan = view == "plan"
    keys = [w[0] if plan else w[1] for ring in rings for w in ring]
    low_key, high_key = min(keys), max(keys)
    across = [w[1] if plan else w[0] for ring in rings for w in ring]
    fallback = (min(across) + max(across)) / 2

    def at(key):
        key = min(max(key, low_key), high_key)
        values = []
        for ring in rings:
            for i, a in enumerate(ring):
                b = ring[(i + 1) % len(ring)]
                ka, kb, da, db = (a[0], b[0], a[1], b[1]) if plan else (a[1], b[1], a[0], b[0])
                if min(ka, kb) - 1e-12 > key or max(ka, kb) + 1e-12 < key:
                    continue
                values.extend([da, db] if kb == ka else [da + (db - da) * (key - ka) / (kb - ka)])
        return (min(values) + max(values)) / 2 if values else fallback

    return at


def moved(view, shape, bone, rest, now, is_affected, depth=None):
    corners = [(shape["x"], shape["y"]), (shape["x"] + shape["w"], shape["y"]), (shape["x"] + shape["w"], shape["y"] + shape["h"]), (shape["x"], shape["y"] + shape["h"])]
    front = view == "front"
    stretches = shape.get("followTip") is True
    span = (rest["tip"][1] - rest["origin"][1]) if front else (rest["tip"][0] - rest["origin"][0])
    in_view = abs(span) >= 0.5 * bone["lengthM"]
    if view == "elevation":
        origin = np.array(now["origin"])
        rotation = np.array(now["rotation"])
        return MultiPoint([tuple((origin + rotation @ np.array([a, b, 0.0]))[:2]) for a, b in corners]).convex_hull
    result = []
    grows = stretches and not in_view
    for x, y in corners:
        d = None if depth is None or grows else depth(y if front else x) if callable(depth) else depth
        q = carry(rest, now, [rest["origin"][0] if d is None else d, y, x]) if front else carry(rest, now, [x, rest["origin"][1] if d is None else d, y])
        result.append([q[2], q[1]] if front else [q[0], q[2]])
    if grows:
        origin = [now["origin"][k] - rest["origin"][k] for k in range(3)]
        tip = [now["tip"][k] - rest["tip"][k] for k in range(3)]
        outward = np.sign(tip[2] - origin[2])
        if outward != 0:
            across = [c[0] if front else c[1] for c in corners]
            low, high = min(across), max(across)
            for index, c in enumerate(across):
                u = 1.0 if high == low else (c - low) / (high - low) if outward > 0 else (high - c) / (high - low)
                if front:
                    result[index][0] += (tip[2] - origin[2]) * u
                    result[index][1] += (tip[1] - origin[1]) * u
                else:
                    result[index][0] += (tip[0] - origin[0]) * u
                    result[index][1] += (tip[2] - origin[2]) * u
    return MultiPoint([tuple(point) for point in result]).convex_hull


def main():
    with open(sys.argv[1], encoding="utf-8") as handle:
        document = json.load(handle)
    rig = document["rig"]
    rest_frames = solve(rig, {}, {"x": 0.0, "y": 0.0, "angleDeg": 0.0}, False)
    rest = {id_: {"origin": f[0].tolist(), "tip": f[2].tolist(), "rotation": f[1].tolist()} for id_, f in rest_frames.items()}
    output = {}
    for name, case in document["cases"].items():
        now = positions(case, rig)
        flags = affected(rig, case["offsets"])
        extents = {}
        for view, shapes in case["shapes"].items():
            for shape in shapes:
                bone = next(b for b in rig if b["id"] == shape["bone"])
                if shape.get("solid"):
                    geometry = solid_geometry(view, shape, rig, rest, now)
                else:
                    twin = next((t for t in case["shapes"].get("elevation", []) if t["id"] == shape["id"] and t["bone"] == shape["bone"] and not t.get("solid")), None)
                    depth = None
                    if twin is not None and view != "elevation":
                        rest_bone = rest[bone["id"]]
                        twin_corners = [tuple(point) for point in twin["points"]] if twin["kind"] == "poly" else [(twin["x"], twin["y"]), (twin["x"] + twin["w"], twin["y"]), (twin["x"] + twin["w"], twin["y"] + twin["h"]), (twin["x"], twin["y"] + twin["h"])]
                        world = [np.array(rest_bone["origin"]) + np.array(rest_bone["rotation"]) @ np.array([a, b, 0.0]) for a, b in twin_corners]
                        values = [w[0] if view == "front" else w[1] for w in world]
                        depth = (min(values) + max(values)) / 2
                    elif view != "elevation":
                        others = [t for t in case["shapes"].get("elevation", []) if t["bone"] == shape["bone"] and not t.get("solid") and t.get("role") != "ground"]
                        if others:
                            depth = slice_depth(view, others, rest[bone["id"]])
                    geometry = moved(view, shape, bone, rest[bone["id"]], now[bone["id"]], flags[bone["id"]], depth)
                minx, miny, maxx, maxy = geometry.bounds
                extents[f"{view}/{shape['id']}"] = {"minX": minx, "maxX": maxx, "minY": miny, "maxY": maxy, **({"area": geometry.area} if shape.get("solid") else {})}
        output[name] = {"bones": {k: {"origin": v["origin"], "tip": v["tip"]} for k, v in now.items()}, "extents": extents}
    json.dump(output, sys.stdout, allow_nan=False)


if __name__ == "__main__":
    main()

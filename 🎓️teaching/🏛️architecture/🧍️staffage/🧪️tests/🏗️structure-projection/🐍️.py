"""🏗️ Oracle of the three-dimensional structural cases and of the projection of forces and moments (numpy; run with `--write` or `--check`).

The solver works in the physical frame x ahead, y up, z the asset's left, a LEFT-handed triple. numpy statics are only valid in a right-handed frame, so every oracle converts explicitly:
points and forces `(x, y, z)` become `(x, y, -z)`, and a moment `(mx, my, mz)` (positive about axis k turns i towards j) becomes the right-hand-rule pseudovector `(-mx, -my, mz)`; results are converted back with the same involutions.
The three cameras are derived from the physical position of the viewer and the paper-up direction with `right = up x toward_viewer`, not copied from the code.

Cases written to `🧫️fixtures/<case>/🔣️.json`: `🏗️balcony-lateral`, `🏗️crane-slewed`, `🏗️lean-lateral`, `🏗️projection-vectors`.
"""
import importlib.util
import math
import sys
from pathlib import Path

import numpy as np

_spec = importlib.util.spec_from_file_location("structure_oracle", Path(__file__).resolve().parents[1] / "🏗️structure-oracle" / "🐍️.py")
oracle = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(oracle)

G = 9.81
E_PA, A, I, C = 2.1e11, 2.85e-3, 1.943e-5, 0.1


def rh(v):
    return np.array([v[0], v[1], -v[2]], float)


def rh_moment(m):
    return np.array([-m[0], -m[1], m[2]], float)


def physical(v):
    return [float(v[0]), float(v[1]), float(-v[2])]


def physical_moment(m):
    return [float(-m[0]), float(-m[1]), float(m[2])]


def wrench(loads):
    """⚖️ The total force and the moment about the origin of loads `(point, force, couple)`, all physical, evaluated in the right-handed frame."""
    f = sum((rh(force) for _, force, _ in loads), np.zeros(3))
    m = sum((np.cross(rh(p), rh(force)) + rh_moment(couple) for p, force, couple in loads), np.zeros(3))
    return f, m


def reactions_of(loads, supports):
    """🧷️ Reactions that balance the loads: `supports` is a list of `(point, kind)` with kind `fixed` (3 forces, 3 couples) or `pin` (3 forces), `guide` (one force along the lateral axis z) or `normal` (one force along +y); least squares in the right-handed frame."""
    columns, owners = [], []
    for i, (point, kind) in enumerate(supports):
        p = rh(point)
        if kind == "fixed":
            for axis in np.eye(3):
                columns.append(np.concatenate([axis, np.cross(p, axis)]))
                owners.append((i, "force", axis))
            for axis in np.eye(3):
                columns.append(np.concatenate([np.zeros(3), axis]))
                owners.append((i, "couple", axis))
        elif kind == "pin":
            for axis in np.eye(3):
                columns.append(np.concatenate([axis, np.cross(p, axis)]))
                owners.append((i, "force", axis))
        else:
            axis = np.array([0.0, 0.0, -1.0]) if kind == "guide" else np.array([0.0, 1.0, 0.0])
            columns.append(np.concatenate([axis, np.cross(p, axis)]))
            owners.append((i, "force", axis))
    f, m = wrench(loads)
    q, *_ = np.linalg.lstsq(np.array(columns).T, -np.concatenate([f, m]), rcond=None)
    out = [{"force": np.zeros(3), "couple": np.zeros(3)} for _ in supports]
    for (i, what, axis), value in zip(owners, q):
        out[i][what] = out[i][what] + value * axis
    return [{"fxN": physical(r["force"])[0], "fyN": physical(r["force"])[1], "fzN": physical(r["force"])[2],
             "mxNm": physical_moment(r["couple"])[0], "myNm": physical_moment(r["couple"])[1], "mzNm": physical_moment(r["couple"])[2]} for r in out]


def cut(loads, q):
    """✂️ Force and moment (physical components) of the distal loads about the cut point `q`."""
    f = sum((rh(force) for _, force, _ in loads), np.zeros(3))
    m = sum((np.cross(rh(p) - rh(q), rh(force)) + rh_moment(couple) for p, force, couple in loads), np.zeros(3))
    return physical(f), physical_moment(m)


def dot(a, b):
    return float(sum(x * y for x, y in zip(a, b)))


SECTION = {"areaM2": A, "secondMomentM4": I, "extremeFibreM": C, "material": "steel"}
STEEL = [{"id": "steel", "elasticModulusPa": E_PA}]


def balcony_build():
    tip = {"id": "tip", "bone": "beam", "at": {"x": 2.0, "y": 0, "z": 0}, "fxN": 0, "fyN": -1000.0, "fzN": 300.0}
    return {
        "case": "🏗️balcony-lateral", "gravityMS2": G, "dynamics": "quasi-static", "kind": "structural", "materials": STEEL,
        "root": {"x": 0, "y": 0, "angleDeg": 0},
        "bones": [{"id": "beam", "parent": None, "x": 0, "y": 0, "angleDeg": 0, "lengthM": 2.0, "section": SECTION}],
        "anchors": [{"id": "wall", "bone": "beam", "x": 0, "y": 0, "kind": "support", "support": "fixed"}],
        "contacts": [], "supports": ["wall"], "loads": [tip],
        "scenarios": [{"id": "gravity-only", "loads": [{**tip, "fzN": 0}]}, {"id": "lateral-only", "loads": [{**tip, "fyN": 0}]}],
    }


def balcony_compute(doc):
    out = {"_tool": "numpy", "_version": oracle.numpy_version()}
    for name, loads in [("both", doc["loads"])] + [(s["id"], s["loads"]) for s in doc["scenarios"]]:
        tuples = [([l["at"]["x"], l["at"]["y"], l["at"]["z"]], [l["fxN"], l["fyN"], l.get("fzN", 0)], [0, 0, 0]) for l in loads]
        out[name] = {"reaction": reactions_of(tuples, [([0, 0, 0], "fixed")])[0], "stations": []}
        for s in (0.0, 0.5, 1.0, 1.5):
            f, m = cut(tuples, [s, 0, 0])
            out[name]["stations"].append({"s": s, "axialN": dot(f, [1, 0, 0]), "shearN": -dot(f, [0, 1, 0]), "shearZN": -dot(f, [0, 0, 1]),
                                          "torsionNm": dot(m, [1, 0, 0]), "momentNm": dot(m, [0, 0, 1]), "momentLateralNm": dot(m, [0, 1, 0])})
        out[name]["tipLateralDeflectionM"] = loads[0].get("fzN", 0) * 2.0**3 / (3 * E_PA * I)
    return out


def crane_build():
    hook = {"id": "hook", "bone": "jib", "at": {"x": 0, "y": 20.0, "z": 15.0}, "fxN": 0, "fyN": -20000.0}
    return {
        "case": "🏗️crane-slewed", "gravityMS2": G, "dynamics": "quasi-static", "kind": "structural", "materials": STEEL,
        "root": {"x": 0, "y": 0, "angleDeg": 90},
        "bones": [
            {"id": "mast", "parent": None, "x": 0, "y": 0, "angleDeg": 0, "lengthM": 20.0, "twistDeg": 90, "mass": {"kg": 4000.0, "comX": 10.0, "comY": 0.0, "distribution": "uniform"}, "section": SECTION},
            {"id": "jib", "parent": "mast", "x": 20.0, "y": 0, "angleDeg": -90, "lengthM": 15.0, "mass": {"kg": 1500.0, "comX": 7.5, "comY": 0.0, "distribution": "uniform"}, "section": SECTION},
        ],
        "anchors": [{"id": "base", "bone": "mast", "x": 0, "y": 0, "kind": "support", "support": "fixed"}],
        "contacts": [], "supports": ["base"], "loads": [hook],
        "scenarios": [{"id": "unslewed", "bones": {"mast": {"twistDeg": 0}}, "loads": [{**hook, "at": {"x": 15.0, "y": 20.0, "z": 0}}]}],
    }


def crane_compute(doc):
    mast = ([0, 10.0, 0], [0, -4000 * G, 0], [0, 0, 0])
    hook = ([0, 20.0, 15.0], [0, -20000.0, 0], [0, 0, 0])
    jib = ([0, 20.0, 7.5], [0, -1500 * G, 0], [0, 0, 0])
    loads = [mast, jib, hook]
    reaction = reactions_of(loads, [([0, 0, 0], "fixed")])[0]
    f, m = cut([jib, hook], [0, 20.0, 0])
    unslewed = [([0, 10.0, 0], mast[1], [0, 0, 0]), ([7.5, 20.0, 0], jib[1], [0, 0, 0]), ([15.0, 20.0, 0], hook[1], [0, 0, 0])]
    return {
        "_tool": "numpy", "_version": oracle.numpy_version(),
        "slewed": {"reaction": reaction, "jibRootBendingNm": math.hypot(m[0], m[1]), "jibRootMomentVector": m, "jibRootForce": f, "jibDirection": [0, 0, 1]},
        "unslewed": {"reaction": reactions_of(unslewed, [([0, 0, 0], "fixed")])[0]},
    }


FEET = 0.1
LEAN_DEG = 20.0


def lean_build():
    return {
        "case": "🏗️lean-lateral", "gravityMS2": G, "dynamics": "quasi-static", "kind": "anatomical", "materials": STEEL,
        "root": {"x": 0, "y": 0, "angleDeg": 90},
        "bones": [
            {"id": "legs", "parent": None, "x": 0, "y": 0, "angleDeg": 0, "lengthM": 0.9, "mass": {"kg": 25.0, "comX": 0.45, "comY": 0.0, "distribution": "uniform"}},
            {"id": "torso", "parent": "legs", "x": 0.9, "y": 0, "angleDeg": 0, "abductionDeg": LEAN_DEG, "lengthM": 0.6, "mass": {"kg": 30.0, "comX": 0.3, "comY": 0.0, "distribution": "uniform"}},
        ],
        "anchors": [
            {"id": "foot-left", "bone": "legs", "x": 0, "y": 0, "z": FEET, "kind": "contact", "support": "contact", "normalDeg": 90, "footprintLengthM": 0.26, "footprintWidthM": 0.1},
            {"id": "foot-right", "bone": "legs", "x": 0, "y": 0, "z": -FEET, "kind": "contact", "support": "contact", "normalDeg": 90, "footprintLengthM": 0.26, "footprintWidthM": 0.1},
        ],
        "contacts": ["foot-left", "foot-right"], "supports": [], "loads": [],
        "scenarios": [{"id": "upright", "bones": {"torso": {"abductionDeg": 0}}}],
    }


def lean_compute(doc):
    phi = math.radians(LEAN_DEG)

    def loads_for(lean):
        torso_com = [0.0, 0.9 + 0.3 * math.cos(lean), 0.3 * math.sin(lean)]
        return [([0, 0.45, 0], [0, -25 * G, 0], [0, 0, 0]), (torso_com, [0, -30 * G, 0], [0, 0, 0])], torso_com

    out = {"_tool": "numpy", "_version": oracle.numpy_version()}
    for name, lean in (("lean", phi), ("upright", 0.0)):
        loads, com = loads_for(lean)
        left, right = reactions_of(loads, [([0, 0, FEET], "normal"), ([0, 0, -FEET], "normal")])
        f, m = cut([loads[1]], [0, 0.9, 0])
        out[name] = {"leftNormalN": left["fyN"], "rightNormalN": right["fyN"], "torsoJointMxNm": m[0], "torsoJointMyNm": m[1], "torsoJointMzNm": m[2], "torsoComZ": com[2]}
    return out


HEAD, IGU, SPANDREL = 3.3, 2.075, 0.45
W_IGU, W_SPANDREL = -1080.0 * 3.36, -1080.0 * 1.26


def curtain_build():
    loads = [{"id": "igu-centre", "bone": "mullion", "at": {"x": 0, "y": IGU, "z": 0}, "fxN": 0, "fyN": 0, "fzN": W_IGU}, {"id": "spandrel-centre", "bone": "mullion", "at": {"x": 0, "y": SPANDREL, "z": 0}, "fxN": 0, "fyN": 0, "fzN": W_SPANDREL}]
    return {
        "case": "🏗️curtain-wall-bay", "gravityMS2": G, "dynamics": "quasi-static", "kind": "structural", "materials": STEEL,
        "root": {"x": 0, "y": 0, "angleDeg": 90},
        "bones": [{"id": "mullion", "parent": None, "x": 0, "y": 0, "angleDeg": 0, "lengthM": HEAD, "section": SECTION}],
        "anchors": [
            {"id": "head", "bone": "mullion", "x": HEAD, "y": 0, "kind": "support", "support": "pin"},
            {"id": "base", "bone": "mullion", "x": 0, "y": 0, "kind": "support", "support": "roller", "normalDeg": 0, "tiltDeg": 90},
        ],
        "contacts": [], "supports": ["head", "base"], "loads": loads, "scenarios": [],
    }


def curtain_compute(doc):
    loads = [([0, l["at"]["y"], 0], [0, 0, l["fzN"]], [0, 0, 0]) for l in doc["loads"]]
    head, base = reactions_of(loads, [([0, HEAD, 0], "pin"), ([0, 0, 0], "guide")])
    stations = []
    for y in (0.275, SPANDREL, 0.9625, IGU, 2.8875):
        distal = [l for l in loads if l[0][1] > y + 1e-9] + [([0, HEAD, 0], [head["fxN"], head["fyN"], head["fzN"]], [0, 0, 0])]
        f, m = cut(distal, [0, y, 0])
        stations.append({"s": y, "mxNm": m[0], "myNm": m[1], "mzNm": m[2], "shearZN": -f[2], "shearN": dot(f, [1, 0, 0]), "momentLateralNm": dot(m, [-1, 0, 0]), "momentNm": m[2]})
    return {"_tool": "numpy", "_version": oracle.numpy_version(), "head": head, "base": base, "stations": stations,
            "hand": {"headFzN": 2467.3, "baseFzN": 2522.3, "momentAtSpandrelNm": 1135.0, "peakMomentNm": 3022.5, "shears": [2522.3, 1161.5, -2467.3]}}


CAMERAS = {"elevation": ([0, 0, -1], [0, 1, 0]), "front": ([1, 0, 0], [0, 1, 0]), "plan": ([0, 1, 0], [0, 0, 1])}
VECTORS = [[0, -1, 0], [300, 0, 0], [0, 0, 300], [120, -450, 75], [-80, 60, -220], [0, 0, -1000], [5, 5, 5], [-1, 2, -3]]


def projection_build():
    return {"case": "🏗️projection-vectors", "vectors": VECTORS}


def projection_compute(doc):
    out = {"_tool": "numpy", "_version": oracle.numpy_version()}
    for name, (viewer, up) in CAMERAS.items():
        d, u = rh(viewer), rh(up)
        right = np.cross(u, d)
        forces = [{"x": float(rh(v) @ right), "y": float(rh(v) @ u), "towardViewer": float(rh(v) @ d)} for v in doc["vectors"]]
        moments = [{"x": -float(rh_moment(v) @ right), "y": -float(rh_moment(v) @ u), "curl": float(rh_moment(v) @ d)} for v in doc["vectors"]]
        out[name] = {"right": [float(x) for x in right], "forces": forces, "moments": moments}
    return out


if __name__ == "__main__":
    for case, build, compute in (("🏗️balcony-lateral", balcony_build, balcony_compute), ("🏗️crane-slewed", crane_build, crane_compute), ("🏗️lean-lateral", lean_build, lean_compute), ("🏗️curtain-wall-bay", curtain_build, curtain_compute), ("🏗️projection-vectors", projection_build, projection_compute)):
        oracle.run(case, build, compute)

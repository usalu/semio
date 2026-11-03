"""🏗️ Oracle of seeded three-dimensional rigid trees: bones with lateral offsets (`z`), sagittal angles, abduction and twist, uniform masses, and loads with all three force components and all three couples (numpy; run with `--write` or `--check`).

The fixed support at the root makes every tree statically determinate. The oracle composes the bone frames as matrix products (`F_child = F_parent B1(angle) B2(abduction) B3(twist)`, columns `u, v, w`), solves the root reaction and every joint load by the wrench of the subtree in a right-handed frame, and converts explicitly:
physical `(x, y, z)` with z the asset's left is left-handed, so points and forces map to `(x, y, -z)` and a moment `(mx, my, mz)` (positive about axis k turns i towards j) to the pseudovector `(-mx, -my, mz)`, and back.
Member cuts are the moment of the distal loads about the cut point (uniform weight integrated over the distal part); `momentNm` is `M . w`, `momentLateralNm` `M . v`, `torsionNm` `M . u`, shear `-F . v` and `-F . w`.
"""
import importlib.util
import math
from pathlib import Path

import numpy as np

_spec = importlib.util.spec_from_file_location("structure_oracle", Path(__file__).resolve().parents[1] / "🏗️structure-oracle" / "🐍️.py")
oracle = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(oracle)

G = 9.81
TREES = 8


def rh(v):
    return np.array([v[0], v[1], -v[2]], float)


def rh_moment(m):
    return np.array([-m[0], -m[1], m[2]], float)


def physical(v):
    return [float(v[0]), float(v[1]), float(-v[2])]


def physical_moment(m):
    return [float(-m[0]), float(-m[1]), float(m[2])]


def basis(angle, abduction, twist):
    a, p, t = (math.radians(x) for x in (angle, abduction, twist))
    b1 = np.array([[math.cos(a), -math.sin(a), 0], [math.sin(a), math.cos(a), 0], [0, 0, 1]])
    b2 = np.array([[math.cos(p), 0, -math.sin(p)], [0, 1, 0], [math.sin(p), 0, math.cos(p)]])
    b3 = np.array([[1, 0, 0], [0, math.cos(t), math.sin(t)], [0, -math.sin(t), math.cos(t)]])
    return b1 @ b2 @ b3


def frames_of(doc):
    """🧊️ World origin and basis (columns u, v, w, physical components) of every bone."""
    out = {}
    root = math.radians(doc["root"]["angleDeg"])
    base = np.array([[math.cos(root), -math.sin(root), 0], [math.sin(root), math.cos(root), 0], [0, 0, 1]])
    for b in doc["bones"]:
        if b["parent"] is None:
            parent_o, parent_f = np.array([doc["root"]["x"], doc["root"]["y"], 0.0]), base
            angle = 0.0
        else:
            parent_o, parent_f = out[b["parent"]]
            angle = b["angleDeg"]
        origin = parent_o + parent_f @ np.array([b["x"], b["y"], b.get("z", 0.0)])
        out[b["id"]] = (origin, parent_f @ basis(angle, b.get("abductionDeg", 0.0), b.get("twistDeg", 0.0)))
    return out


def point_of(frames, bone, s, y=0.0, z=0.0):
    o, f = frames[bone]
    return o + f @ np.array([s, y, z])


def build(seed):
    rng = np.random.default_rng(seed)
    bones = [{"id": "b0", "parent": None, "x": 0, "y": 0, "angleDeg": 0, "lengthM": 2.0, "twistDeg": float(rng.uniform(-90, 90)), "mass": {"kg": float(rng.uniform(5, 50)), "comX": 1.0, "comY": 0.0, "distribution": "uniform"}}]
    for i, parent in ((1, "b0"), (2, "b1"), (3, "b0")):
        length = float(rng.uniform(0.8, 1.6))
        parent_length = next(b["lengthM"] for b in bones if b["id"] == parent)
        bones.append({"id": f"b{i}", "parent": parent, "x": float(rng.uniform(0.6, 0.95) * parent_length), "y": float(rng.uniform(-0.1, 0.1)), "z": float(rng.uniform(-0.3, 0.3)), "angleDeg": float(rng.uniform(-40, 40)),
                      "abductionDeg": float(rng.uniform(-60, 60)), "twistDeg": float(rng.uniform(-90, 90)), "lengthM": length, "mass": {"kg": float(rng.uniform(5, 50)), "comX": length / 2, "comY": 0.0, "distribution": "uniform"}})
    doc = {"case": f"🏗️rigid-tree-3d-{seed}", "gravityMS2": G, "dynamics": "quasi-static", "kind": "structural", "materials": [], "root": {"x": 0, "y": 0, "angleDeg": 90}, "bones": bones,
           "anchors": [{"id": "base", "bone": "b0", "x": 0, "y": 0, "kind": "support", "support": "fixed"}], "contacts": [], "supports": ["base"], "loads": [], "scenarios": []}
    frames = frames_of(doc)
    for k, b in enumerate(bones):
        s = float(rng.uniform(0.55, 0.95) * b["lengthM"])
        at = point_of(frames, b["id"], s, float(rng.uniform(-0.05, 0.05)), float(rng.uniform(-0.1, 0.1)))
        f = rng.uniform(-800, 800, 3)
        m = rng.uniform(-300, 300, 3)
        doc["loads"].append({"id": f"load-{k}", "bone": b["id"], "at": {"x": float(at[0]), "y": float(at[1]), "z": float(at[2])}, "fxN": float(f[0]), "fyN": float(f[1]), "fzN": float(f[2]), "momentXNm": float(m[0]), "momentYNm": float(m[1]), "momentNm": float(m[2])})
    return doc


def pieces(doc, frames):
    """⚖️ The weights (at the centre of each segment) and the authored loads of the tree as `(bone, point, force, couple)` in physical components."""
    weights = [(b["id"], point_of(frames, b["id"], b["mass"]["comX"]), np.array([0, -b["mass"]["kg"] * G, 0.0]), np.zeros(3)) for b in doc["bones"]]
    authored = [(l["bone"], np.array([l["at"]["x"], l["at"]["y"], l["at"]["z"]]), np.array([l["fxN"], l["fyN"], l["fzN"]]), np.array([l["momentXNm"], l["momentYNm"], l["momentNm"]])) for l in doc["loads"]]
    return weights, authored


def wrench(loads, about):
    """⚖️ Force and moment about `about` of loads `(bone, point, force, couple)`, evaluated in the right-handed frame and returned in physical components."""
    f = sum((rh(force) for _, _, force, _ in loads), np.zeros(3))
    m = sum((np.cross(rh(p) - rh(about), rh(force)) + rh_moment(c) for _, p, force, c in loads), np.zeros(3))
    return np.array(physical(f)), np.array(physical_moment(m))


def subtree(doc, bone):
    ids, todo = {bone}, [bone]
    while todo:
        cur = todo.pop()
        for b in doc["bones"]:
            if b["parent"] == cur and b["id"] not in ids:
                ids.add(b["id"])
                todo.append(b["id"])
    return ids


def compute_tree(doc):
    frames = frames_of(doc)
    weights, authored = pieces(doc, frames)
    loads = weights + authored
    f, m = wrench(loads, np.zeros(3))
    values = {"reaction": {"fxN": float(-f[0]), "fyN": float(-f[1]), "fzN": float(-f[2]), "mxNm": float(-m[0]), "myNm": float(-m[1]), "mzNm": float(-m[2])}, "joints": {}, "stations": {}, "frames": {}}
    for b in doc["bones"]:
        o, fr = frames[b["id"]]
        values["frames"][b["id"]] = {"origin": [float(x) for x in o], "u": [float(x) for x in fr[:, 0]], "v": [float(x) for x in fr[:, 1]], "w": [float(x) for x in fr[:, 2]]}
        u, v, w = fr[:, 0], fr[:, 1], fr[:, 2]
        if b["parent"] is not None:
            sub_f, sub_m = wrench([l for l in loads if l[0] in subtree(doc, b["id"])], o)
            values["joints"][b["id"]] = {"fxN": float(-sub_f[0]), "fyN": float(-sub_f[1]), "fzN": float(-sub_f[2]), "axialN": float(sub_f @ u), "shearN": float(-(sub_f @ v)), "shearZN": float(-(sub_f @ w)),
                                         "mxNm": float(sub_m[0]), "myNm": float(sub_m[1]), "mzNm": float(sub_m[2]), "torsionNm": float(sub_m @ u), "momentNm": float(sub_m @ w), "momentLateralNm": float(sub_m @ v)}
        for s in (0.0, b["lengthM"] / 2):
            q = o + u * s
            own = [l for l in authored if l[0] == b["id"]]
            distal = [l for l in own if float((l[1] - o) @ u) > s + 1e-9]
            share = (b["lengthM"] - s) / b["lengthM"]
            centre = point_of(frames, b["id"], (s + b["lengthM"]) / 2)
            distal.append((b["id"], centre, np.array([0, -b["mass"]["kg"] * G * share, 0.0]), np.zeros(3)))
            for c in doc["bones"]:
                if c["parent"] == b["id"] and float((frames[c["id"]][0] - o) @ u) > s + 1e-9:
                    distal += [l for l in loads if l[0] in subtree(doc, c["id"])]
            cf, cm = wrench(distal, q)
            values["stations"].setdefault(b["id"], []).append({"s": float(s), "axialN": float(cf @ u), "shearN": float(-(cf @ v)), "shearZN": float(-(cf @ w)), "torsionNm": float(cm @ u), "momentNm": float(cm @ w), "momentLateralNm": float(cm @ v)})
    return values


def build_all():
    return {"case": "🏗️rigid-trees-3d", "trees": [build(seed) for seed in range(TREES)]}


def compute_all(doc):
    return {"_tool": "numpy", "_version": oracle.numpy_version(), **{tree["case"]: compute_tree(tree) for tree in doc["trees"]}}


if __name__ == "__main__":
    oracle.run("🏗️rigid-trees-3d", build_all, compute_all)

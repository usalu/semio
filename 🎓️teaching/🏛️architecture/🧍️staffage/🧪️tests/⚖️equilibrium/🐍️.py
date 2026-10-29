"""⚖️ Oracle of the seeded random rigid trees: determinate supports by numpy equilibrium, indeterminate pins by an independent numpy assembly of the equal-stiffness spring model."""
import importlib.util
import math
from pathlib import Path

import numpy as np

_spec = importlib.util.spec_from_file_location("structure_oracle", Path(__file__).resolve().parents[1] / "🏗️structure-oracle" / "🐍️.py")
oracle = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(oracle)

G, TREES = 9.81, 18


def tree(seed):
    rng = np.random.default_rng(seed)
    count = int(rng.integers(4, 10))
    bones = []
    for i in range(count):
        bones.append({
            "id": f"b{i}", "parent": None if i == 0 else f"b{int(rng.integers(0, i))}",
            "x": 0 if i == 0 else float(rng.uniform(0, 0.8)), "y": 0 if i == 0 else float(rng.uniform(-0.1, 0.1)), "angleDeg": float(rng.uniform(-60, 60)) if i else 0.0,
            "lengthM": float(rng.uniform(0.3, 1.2)),
            "mass": {"kg": float(rng.uniform(1, 20)), "comX": 0.0, "comY": 0.0, "distribution": "uniform"} if rng.random() < 0.6 else {"kg": float(rng.uniform(1, 20)), "comX": float(rng.uniform(0.05, 0.25)), "comY": float(rng.uniform(-0.05, 0.05)), "distribution": "point"},
        })
    doc = {"case": f"tree-{seed}", "gravityMS2": G, "dynamics": "quasi-static", "kind": "structural", "materials": [], "root": {"x": float(rng.uniform(-1, 1)), "y": float(rng.uniform(0, 2)), "angleDeg": float(rng.uniform(-30, 30))}, "bones": bones, "anchors": [], "contacts": [], "supports": [], "loads": [], "scenarios": []}
    frame = oracle.forward(doc)
    for k in range(int(rng.integers(1, 4))):
        b = bones[int(rng.integers(0, count))]
        x, y = oracle.point(doc, frame, b["id"], float(rng.uniform(0, b["lengthM"])), float(rng.uniform(-0.05, 0.05)))
        doc["loads"].append({"id": f"load-{k}", "bone": b["id"], "at": {"x": x, "y": y}, "fxN": float(rng.uniform(-200, 200)), "fyN": float(rng.uniform(-400, 100)), "momentNm": float(rng.uniform(-30, 30))})
    kind = ("fixed", "pin-roller", "three-pins")[seed % 3]
    for k, (support, normal) in enumerate({"fixed": [("fixed", None)], "pin-roller": [("pin", None), ("roller", 90)], "three-pins": [("pin", None)] * 3}[kind]):
        b = bones[int(rng.integers(0, count))]
        a = {"id": f"s{k}", "bone": b["id"], "x": float(rng.uniform(0, b["lengthM"])), "y": float(rng.uniform(-0.05, 0.05)), "kind": "support", "support": support}
        if normal:
            a["normalDeg"] = normal
        doc["anchors"].append(a)
        doc["supports"].append(a["id"])
    doc["kindOfSupports"] = kind
    return doc


def external(doc):
    frame = oracle.forward(doc)
    forces = [(x, y, 0.0, -kg * G, 0.0) for _, x, y, kg in oracle.masses(doc, frame)]
    return forces + [(l["at"]["x"], l["at"]["y"], l["fxN"], l["fyN"], l["momentNm"]) for l in doc["loads"]]


def anchor_points(doc):
    frame = oracle.forward(doc)
    return [oracle.point(doc, frame, a["bone"], a["x"], a["y"]) for a in doc["anchors"]]


def reactions(doc):
    forces, pts = external(doc), anchor_points(doc)
    fx, fy = sum(f[2] for f in forces), sum(f[3] for f in forces)
    kind = doc["kindOfSupports"]
    if kind == "fixed":
        px, py = pts[0]
        m = sum((f[0] - px) * f[3] - (f[1] - py) * f[2] + f[4] for f in forces)
        return [{"fxN": -fx, "fyN": -fy, "momentNm": -m}]
    if kind == "pin-roller":
        (ax, ay), (bx, _) = pts
        m = sum((f[0] - ax) * f[3] - (f[1] - ay) * f[2] + f[4] for f in forces)
        roller = -m / (bx - ax)
        return [{"fxN": -fx, "fyN": -fy - roller, "momentNm": 0.0}, {"fxN": 0.0, "fyN": roller, "momentNm": 0.0}]
    cx, cy = np.mean([p[0] for p in pts]), np.mean([p[1] for p in pts])
    rows = [(np.array([1.0, 0.0, -(py - cy)])) for _, py in pts] + [(np.array([0.0, 1.0, px - cx])) for px, _ in pts]
    stiffness = sum(np.outer(r, r) for r in rows)
    load = np.array([fx, fy, sum((f[0] - cx) * f[3] - (f[1] - cy) * f[2] + f[4] for f in forces)])
    u = np.linalg.solve(stiffness, load)
    return [{"fxN": float(-(np.array([1.0, 0.0, -(py - cy)]) @ u)), "fyN": float(-(np.array([0.0, 1.0, px - cx]) @ u)), "momentNm": 0.0} for px, py in pts]


def build():
    return {"case": "⚖️equilibrium", "trees": [tree(seed) for seed in range(1, TREES + 1)]}


def compute(doc):
    return {"_tool": "numpy", "_version": oracle.numpy_version(), **{t["case"]: reactions(t) for t in doc["trees"]}}


oracle.run("⚖️equilibrium", build, compute)

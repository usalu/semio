"""🏃️ Oracle of the inertial consistency: reactions of a moving two-foot body from numpy linear algebra, sum of reactions = Σ m (g + a), friction capped at μ·N."""
import importlib.util
import math
from pathlib import Path

import numpy as np

_spec = importlib.util.spec_from_file_location("structure_oracle", Path(__file__).resolve().parents[1] / "🏗️structure-oracle" / "🐍️.py")
oracle = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(oracle)

G, MU, FOOT_X, HIP = 9.81, 0.2, 0.3, 0.95
PARTS = ("pelvis", "leg-left", "leg-right", "torso")


def build():
    rng = np.random.default_rng(7)
    def body(id, parent, x, y, angle, length, kg, distribution, com_x=0.0):
        return {"id": id, "parent": parent, "x": x, "y": y, "angleDeg": angle, "lengthM": length, "mass": {"kg": kg, "comX": com_x, "comY": 0, "distribution": distribution}}
    moving = {b: {"x": float(rng.uniform(-1.2, 1.2)), "y": float(rng.uniform(-2, 2))} for b in PARTS}
    return {
        "case": "🏃️inertial-consistency", "gravityMS2": G, "dynamics": "inertial", "kind": "anatomical", "materials": [],
        "root": {"x": 0, "y": HIP, "angleDeg": 0},
        "bones": [body("pelvis", None, 0, 0, 0, 0.0, 14, "point"), body("leg-left", "pelvis", -FOOT_X, 0, -90, HIP, 10, "uniform"), body("leg-right", "pelvis", FOOT_X, 0, -90, HIP, 10, "uniform"), body("torso", "pelvis", 0, 0, 75, 0.7, 35, "uniform")],
        "anchors": [{"id": f"foot-{side}", "bone": f"leg-{side}", "x": HIP, "y": 0, "kind": "contact", "support": "contact", "normalDeg": 90, "frictionCoefficient": MU} for side in ("left", "right")],
        "contacts": ["foot-left", "foot-right"], "supports": [], "loads": [],
        "scenarios": [
            {"id": "moving", "accelerations": moving},
            {"id": "friction-saturated", "accelerations": {b: {"x": 2.6, "y": 0.0} for b in PARTS}},
            {"id": "flight", "contacts": [], "accelerations": {b: {"x": 0.0, "y": -G} for b in PARTS}},
            {"id": "pulled-down", "accelerations": {b: {"x": 0.0, "y": -2 * G} for b in PARTS}},
            {"id": "quasi-static", "dynamics": "quasi-static", "accelerations": moving},
        ],
    }


def scenario_values(doc, sc):
    frame = oracle.forward(doc)
    dynamic = sc.get("dynamics", doc["dynamics"]) == "inertial"
    parts = oracle.masses(doc, frame)
    acc = lambda b: sc["accelerations"][b] if dynamic else {"x": 0.0, "y": 0.0}
    need_x = sum(kg * acc(b)["x"] for b, _, _, kg in parts)
    need_y = sum(kg * (G + acc(b)["y"]) for b, _, _, kg in parts)
    if sc["id"] == "pulled-down":
        com_x = sum(kg * x for _, x, _, kg in parts) / sum(kg for _, _, _, kg in parts)
        return {"keptFoot": "foot-left" if abs(com_x + FOOT_X) < abs(com_x - FOOT_X) else "foot-right", "comX": com_x}
    if not sc.get("contacts", doc["contacts"]):
        return {"sumReactionFxN": 0.0, "sumReactionFyN": 0.0, "residualFxN": -need_x, "residualFyN": -need_y, "expectedSumFyN": need_y, "expectedSumFxN": need_x}
    moment = sum(kg * (G + acc(b)["y"]) * x - kg * acc(b)["x"] * y for b, x, y, kg in parts)
    xs = np.array([-FOOT_X, FOOT_X])
    normal = np.linalg.solve(np.array([[1.0, 1.0], xs]).astype(float), np.array([need_y, moment]))
    tangent = np.array([need_x / 2, need_x / 2])
    free = [True, True]
    for _ in range(3):
        cap = MU * normal
        for i in range(2):
            if free[i] and abs(tangent[i]) > cap[i] + 1e-12:
                free[i] = False
                tangent[i] = np.sign(need_x) * cap[i]
        rest = [i for i in range(2) if free[i]]
        if rest:
            share = (need_x - sum(tangent[i] for i in range(2) if not free[i])) / len(rest)
            for i in rest:
                tangent[i] = share
    return {
        "normalLeftN": float(normal[0]), "normalRightN": float(normal[1]), "tangentLeftN": float(tangent[0]), "tangentRightN": float(tangent[1]),
        "sumReactionFxN": float(tangent.sum()), "sumReactionFyN": float(normal.sum()), "expectedSumFxN": need_x, "expectedSumFyN": need_y,
        "residualFxN": float(tangent.sum() - need_x), "residualFyN": float(normal.sum() - need_y),
    }


def compute(doc):
    return {"_tool": "numpy", "_version": oracle.numpy_version(), **{sc["id"]: scenario_values(doc, sc) for sc in doc["scenarios"]}}


oracle.run("🏃️inertial-consistency", build, compute)

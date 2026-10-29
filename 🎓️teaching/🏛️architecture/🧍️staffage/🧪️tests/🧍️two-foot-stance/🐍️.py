"""🧍️ Oracle of the two-foot stance: lever rule between the feet by an anastruct beam on hinge and roller, joint moment at the shoulder by direct lever arms."""
import importlib.util
import math
from pathlib import Path

_spec = importlib.util.spec_from_file_location("structure_oracle", Path(__file__).resolve().parents[1] / "🏗️structure-oracle" / "🐍️.py")
oracle = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(oracle)

from anastruct import SystemElements

G, HIP, FOOT_X = 9.81, 0.95, 0.15
LOAD_N = 150.0
FOOT_AREA = 0.25 * 0.10


def bone(id, parent, x, y, angle, length, kg, distribution, com_x=0.0):
    return {"id": id, "parent": parent, "x": x, "y": y, "angleDeg": angle, "lengthM": length, "mass": {"kg": kg, "comX": com_x, "comY": 0, "distribution": distribution}}


def build():
    doc = {
        "case": "🧍️two-foot-stance", "gravityMS2": G, "dynamics": "quasi-static", "kind": "anatomical", "materials": [],
        "root": {"x": 0, "y": HIP, "angleDeg": 0},
        "bones": [
            bone("pelvis", None, 0, 0, 0, 0.0, 12, "point"),
            bone("leg-left", "pelvis", -FOOT_X, 0, -90, HIP, 10, "uniform"),
            bone("leg-right", "pelvis", FOOT_X, 0, -90, HIP, 10, "uniform"),
            bone("trunk", "pelvis", 0, 0, 80, 0.6, 32, "uniform"),
            bone("head", "trunk", 0.6, 0, 0, 0.25, 5, "point", 0.12),
            bone("arm", "trunk", 0.55, 0, -170, 0.6, 4, "uniform"),
        ],
        "anchors": [
            {"id": "foot-left", "bone": "leg-left", "x": HIP, "y": 0, "kind": "contact", "support": "contact", "normalDeg": 90, "footprintLengthM": 0.25, "footprintWidthM": 0.10, "frictionCoefficient": 0.6},
            {"id": "foot-right", "bone": "leg-right", "x": HIP, "y": 0, "kind": "contact", "support": "contact", "normalDeg": 90, "footprintLengthM": 0.25, "footprintWidthM": 0.10, "frictionCoefficient": 0.6},
            {"id": "hand", "bone": "arm", "x": 0.6, "y": 0, "kind": "load"},
        ],
        "contacts": ["foot-left", "foot-right"], "supports": [], "loads": [], "scenarios": [],
    }
    for sid, world_deg, loaded in (("unloaded", -90, False), ("reach-right", 70, True), ("reach-left", 160, True), ("reach-far-right", 20, True)):
        sc = {"id": sid, "bones": {"arm": {"angleDeg": world_deg - 80}}, "loads": []}
        if loaded:
            hand = oracle.point(doc, oracle.forward(doc, sc), "arm", 0.6)
            sc["loads"] = [{"id": "carried", "bone": "arm", "at": {"x": hand[0], "y": hand[1]}, "fxN": 0, "fyN": -LOAD_N}]
        doc["scenarios"].append(sc)
    return doc


def scenario_values(doc, sc):
    frame = oracle.forward(doc, sc)
    forces = [(x, -kg * G) for _, x, _, kg in oracle.masses(doc, frame)] + [(l["at"]["x"], l["fyN"]) for l in sc["loads"]]
    by_x = {}
    for x, f in forces:
        by_x[round(x, 9)] = by_x.get(round(x, 9), 0.0) + f
    for foot in (-FOOT_X, FOOT_X):
        by_x.setdefault(round(foot, 9), 0.0)
    xs = sorted(by_x)
    ss = SystemElements()
    for a, b in zip(xs, xs[1:]):
        ss.add_element([[a, 0], [b, 0]], EA=1e9, EI=1e7)
    ss.add_support_hinged(xs.index(round(-FOOT_X, 9)) + 1)
    ss.add_support_roll(xs.index(round(FOOT_X, 9)) + 1)
    for i, x in enumerate(xs):
        if by_x[x]:
            ss.point_load(i + 1, Fy=by_x[x])
    ss.solve()
    left = float(-ss.get_node_results_system(xs.index(round(-FOOT_X, 9)) + 1)["Fy"])
    right = float(-ss.get_node_results_system(xs.index(round(FOOT_X, 9)) + 1)["Fy"])
    ox, oy, a = frame["arm"]
    ex, ey = (math.cos(a), math.sin(a)), (-math.sin(a), math.cos(a))
    arm = [(x, y, 0.0, -kg * G) for b, x, y, kg in oracle.masses(doc, frame) if b == "arm"] + [(l["at"]["x"], l["at"]["y"], l["fxN"], l["fyN"]) for l in sc["loads"]]
    return {
        "reactionLeftN": left, "reactionRightN": right, "pressureLeftPa": left / FOOT_AREA, "pressureRightPa": right / FOOT_AREA,
        "heavierFoot": "left" if left > right else "right", "liftOff": bool(min(left, right) < 0),
        "shoulderMomentNm": sum((x - ox) * fy - (y - oy) * fx for x, y, fx, fy in arm),
        "shoulderAxialN": sum(fx * ex[0] + fy * ex[1] for _, _, fx, fy in arm),
        "shoulderShearN": -sum(fx * ey[0] + fy * ey[1] for _, _, fx, fy in arm),
        "shoulderFxN": -sum(fx for _, _, fx, _ in arm), "shoulderFyN": -sum(fy for _, _, _, fy in arm),
    }


def compute(doc):
    return {"_tool": "anastruct", "_version": oracle.anastruct_version(), **{sc["id"]: scenario_values(doc, sc) for sc in doc["scenarios"]}}


oracle.run("🧍️two-foot-stance", build, compute)

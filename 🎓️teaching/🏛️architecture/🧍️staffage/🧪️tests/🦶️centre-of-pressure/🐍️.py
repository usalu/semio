"""🦶️ Oracle of the centre of pressure of a single foot: base moment of the loaded stance by an anastruct fixed-support beam, COP = M / N clamped at half the footprint."""
import importlib.util
from pathlib import Path

_spec = importlib.util.spec_from_file_location("structure_oracle", Path(__file__).resolve().parents[1] / "🏗️structure-oracle" / "🐍️.py")
oracle = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(oracle)
from anastruct import SystemElements

G, LEG_KG, TORSO_KG, LENGTH, HIP = 9.81, 8.0, 62.0, 0.25, 0.95


def build():
    return {
        "case": "🦶️centre-of-pressure", "gravityMS2": G, "dynamics": "quasi-static", "kind": "anatomical", "materials": [],
        "root": {"x": 0, "y": HIP, "angleDeg": -90},
        "bones": [
            {"id": "leg", "parent": None, "x": 0, "y": 0, "angleDeg": 0, "lengthM": HIP, "mass": {"kg": LEG_KG, "comX": 0, "comY": 0, "distribution": "uniform"}},
            {"id": "torso", "parent": "leg", "x": 0, "y": 0.0, "angleDeg": 0, "lengthM": 0.0, "mass": {"kg": TORSO_KG, "comX": 0, "comY": 0, "distribution": "point"}},
        ],
        "anchors": [{"id": "foot", "bone": "leg", "x": HIP, "y": 0, "kind": "contact", "support": "contact", "normalDeg": 90, "footprintLengthM": LENGTH, "footprintWidthM": 0.1, "frictionCoefficient": 0.6}],
        "contacts": ["foot"], "supports": [], "loads": [],
        "scenarios": [{"id": "within", "bones": {"torso": {"y": 0.04}}}, {"id": "beyond", "bones": {"torso": {"y": 0.2}}}],
    }


def scenario_values(doc, sc):
    offset = sc["bones"]["torso"]["y"]
    ss = SystemElements()
    ss.add_element([[0, 0], [offset, 0]], EA=1e9, EI=1e7)
    ss.add_support_fixed(1)
    ss.point_load(1, Fy=-LEG_KG * G)
    ss.point_load(2, Fy=-TORSO_KG * G)
    ss.solve()
    base = ss.get_node_results_system(1)
    normal, moment = float(-base["Fy"]), float(-base["Tz"])
    limit = LENGTH / 2
    return {"normalN": normal, "baseMomentNm": moment, "copX": max(-limit, min(limit, moment / normal)), "unclampedCopX": moment / normal, "clamped": abs(moment / normal) > limit}


def compute(doc):
    return {"_tool": "anastruct", "_version": oracle.anastruct_version(), **{sc["id"]: scenario_values(doc, sc) for sc in doc["scenarios"]}}


oracle.run("🦶️centre-of-pressure", build, compute)

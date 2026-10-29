"""🌉️ Oracle of the simply supported case: L = 4 m, self-weight UDL plus a midspan point load, anastruct reference."""
import importlib.util
import math
from pathlib import Path

_spec = importlib.util.spec_from_file_location("structure_oracle", Path(__file__).resolve().parents[1] / "🏗️structure-oracle" / "🐍️.py")
oracle = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(oracle)

from anastruct import SystemElements

E_PA, A, I, C, L, P, G = 2.1e11, 2.85e-3, 1.943e-5, 0.1, 4.0, 5000.0, 9.81
Q = 1000.0


def build():
    return {
        "case": "🌉️simply-supported", "gravityMS2": G, "dynamics": "quasi-static", "kind": "structural",
        "materials": [{"id": "steel", "elasticModulusPa": E_PA}],
        "root": {"x": 0, "y": 0, "angleDeg": 0},
        "bones": [{"id": "beam", "parent": None, "x": 0, "y": 0, "angleDeg": 0, "lengthM": L, "mass": {"kg": Q * L / G, "comX": L / 2, "comY": 0, "distribution": "uniform"}, "section": {"areaM2": A, "secondMomentM4": I, "extremeFibreM": C, "material": "steel"}}],
        "anchors": [
            {"id": "pin", "bone": "beam", "x": 0, "y": 0, "kind": "support", "support": "pin"},
            {"id": "roller", "bone": "beam", "x": L, "y": 0, "kind": "support", "support": "roller", "normalDeg": 90},
        ],
        "contacts": [], "supports": ["pin", "roller"],
        "loads": [{"id": "mid", "bone": "beam", "at": {"x": L / 2, "y": 0}, "fxN": 0, "fyN": -P}],
        "scenarios": [],
    }


def compute(doc):
    ss = SystemElements()
    ss.add_element([[0, 0], [L / 2, 0]], EA=E_PA * A, EI=E_PA * I)
    ss.add_element([[L / 2, 0], [L, 0]], EA=E_PA * A, EI=E_PA * I)
    ss.add_support_hinged(1)
    ss.add_support_roll(3)
    ss.q_load(q=-Q, element_id=[1, 2])
    ss.point_load(2, Fy=-P)
    ss.solve()
    left, mid, right = (ss.get_node_results_system(i) for i in (1, 2, 3))
    moments = ss.get_element_result_range("moment")
    return {
        "_tool": "anastruct", "_version": oracle.anastruct_version(),
        "reactionLeftFyN": float(-left["Fy"]), "reactionRightFyN": float(-right["Fy"]),
        "maxMomentNm": float(-max(moments)), "midDeflectionM": float(-mid["uy"]),
        "analyticMaxMomentNm": -(Q * L**2 / 8 + P * L / 4),
        "analyticMidDeflectionM": -(5 * Q * L**4 / (384 * E_PA * I) + P * L**3 / (48 * E_PA * I)),
        "stationMomentNm": {str(s): float(-(Q * s * (L - s) / 2 + (P * s / 2 if s <= L / 2 else P * (L - s) / 2))) for s in (0.5, 1.0, 2.0, 3.0)},
    }


oracle.run("🌉️simply-supported", build, compute)

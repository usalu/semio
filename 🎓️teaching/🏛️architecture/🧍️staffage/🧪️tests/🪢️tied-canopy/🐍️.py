"""🪢️ Oracle of the tied canopy: pin at the wall plus a tie rod to a wall point 1 m above, UDL self-weight and a point load, anastruct truss reference."""
import importlib.util
import math
from pathlib import Path

_spec = importlib.util.spec_from_file_location("structure_oracle", Path(__file__).resolve().parents[1] / "🏗️structure-oracle" / "🐍️.py")
oracle = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(oracle)

from anastruct import SystemElements

E_PA, A, I, C, L, H, P, G = 2.1e11, 2.85e-3, 1.943e-5, 0.1, 2.0, 1.0, 500.0, 9.81
Q = 800.0
ROD_DEG = math.degrees(math.atan2(-H, L))


def build():
    return {
        "case": "🪢️tied-canopy", "gravityMS2": G, "dynamics": "quasi-static", "kind": "structural",
        "materials": [{"id": "steel", "elasticModulusPa": E_PA}],
        "root": {"x": 0, "y": 0, "angleDeg": 0},
        "bones": [{"id": "canopy", "parent": None, "x": 0, "y": 0, "angleDeg": 0, "lengthM": L, "mass": {"kg": Q * L / G, "comX": L / 2, "comY": 0, "distribution": "uniform"}, "section": {"areaM2": A, "secondMomentM4": I, "extremeFibreM": C, "material": "steel"}}],
        "anchors": [
            {"id": "wall-pin", "bone": "canopy", "x": 0, "y": 0, "kind": "support", "support": "pin"},
            {"id": "tie-rod", "bone": "canopy", "x": L, "y": 0, "kind": "support", "support": "roller", "normalDeg": ROD_DEG},
        ],
        "contacts": [], "supports": ["wall-pin", "tie-rod"],
        "loads": [{"id": "snow", "bone": "canopy", "at": {"x": L / 2, "y": 0}, "fxN": 0, "fyN": -P}],
        "scenarios": [],
    }


def compute(doc):
    ss = SystemElements()
    ss.add_element([[0, 0], [L / 2, 0]], EA=E_PA * A, EI=E_PA * I)
    ss.add_element([[L / 2, 0], [L, 0]], EA=E_PA * A, EI=E_PA * I)
    ss.add_truss_element([[L, 0], [0, H]], EA=E_PA * 5e-4)
    ss.add_support_hinged([1, 4])
    ss.q_load(q=-Q, element_id=[1, 2])
    ss.point_load(2, Fy=-P)
    ss.solve()
    rod = ss.get_element_results(3)
    pin = ss.get_node_results_system(1)
    beam = ss.get_element_results(1)
    wall_pull = ss.get_node_results_system(4)
    tension = (Q * L**2 / 2 + P * L / 2) * math.hypot(L, H) / L
    return {
        "_tool": "anastruct", "_version": oracle.anastruct_version(),
        "rodAxialAnastructN": float(rod["Nmax"]), "rodTensionN": float(abs(rod["Nmax"])), "analyticRodTensionN": tension,
        "pinFxN": float(-pin["Fx"]), "pinFyN": float(-pin["Fy"]), "wallPullFxN": float(-wall_pull["Fx"]), "wallPullFyN": float(-wall_pull["Fy"]),
        "beamAxialN": float(-abs(beam["Nmax"])), "rodDeg": ROD_DEG,
    }


oracle.run("🪢️tied-canopy", build, compute)

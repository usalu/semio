"""🏗️ Oracle of the cantilever case: L = 2 m, tip load P = 1 kN, self-weight off, anastruct reference (run with `--write` or `--check`)."""
import importlib.util
import math
from pathlib import Path

_spec = importlib.util.spec_from_file_location("structure_oracle", Path(__file__).resolve().parents[1] / "🏗️structure-oracle" / "🐍️.py")
oracle = importlib.util.module_from_spec(_spec)
_spec.loader.exec_module(oracle)

from anastruct import SystemElements

E_PA, A, I, C, L, P = 2.1e11, 2.85e-3, 1.943e-5, 0.1, 2.0, 1000.0


def build():
    return {
        "case": "🏗️cantilever", "gravityMS2": 9.81, "dynamics": "quasi-static", "kind": "structural",
        "materials": [{"id": "steel", "elasticModulusPa": E_PA}],
        "root": {"x": 0, "y": 0, "angleDeg": 0},
        "bones": [{"id": "beam", "parent": None, "x": 0, "y": 0, "angleDeg": 0, "lengthM": L, "section": {"areaM2": A, "secondMomentM4": I, "extremeFibreM": C, "material": "steel"}}],
        "anchors": [{"id": "wall", "bone": "beam", "x": 0, "y": 0, "kind": "support", "support": "fixed"}],
        "contacts": [], "supports": ["wall"],
        "loads": [{"id": "tip", "bone": "beam", "at": {"x": L, "y": 0}, "fxN": 0, "fyN": -P}],
        "scenarios": [],
    }


def compute(doc):
    ss = SystemElements()
    ss.add_element([[0, 0], [L, 0]], EA=E_PA * A, EI=E_PA * I)
    ss.add_support_fixed(1)
    ss.point_load(2, Fy=-P)
    ss.solve()
    root, tip = ss.get_node_results_system(1), ss.get_node_results_system(2)
    el = ss.get_element_results(1)
    return {
        "_tool": "anastruct", "_version": oracle.anastruct_version(),
        "reactionFxN": float(-root["Fx"]), "reactionFyN": float(-root["Fy"]), "reactionMomentNm": float(-root["Tz"]),
        "tipDeflectionM": float(-tip["uy"]), "rootMomentNm": float(-el["Mmax"]), "shearN": float(-el["Qmax"]),
        "rootStressPa": float(el["Mmax"] * C / I), "analyticTipDeflectionM": -P * L**3 / (3 * E_PA * I),
    }


oracle.run("🏗️cantilever", build, compute)

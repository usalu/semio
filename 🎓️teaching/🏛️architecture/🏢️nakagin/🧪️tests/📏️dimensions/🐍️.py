"""📏 Primary dimensional chain checked on raw data and actual placed geometry with NumPy."""
import importlib
import json
from pathlib import Path
import sys
import unittest
from unittest.mock import patch
from copy import deepcopy
from dataclasses import replace
import numpy as np
ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
MODEL = ROOT / "🎓️teaching/🏛️architecture/🏢️nakagin"
M = importlib.import_module("🎓️teaching.🏛️architecture.🏢️nakagin.🐍️")
C = json.loads((MODEL / "🧪️cases/📏️dimensions/🔣️.json").read_text(encoding="utf-8"))

class PrimaryDimensions(unittest.TestCase):
    def test_primary_scalar_chain(self):
        data = M.load()
        for actual, expected in ((data["core"]["storey_height"], C["pitch"]), (data["foundation"]["basement_depth"], C["basement_depth"]), (data["towers"][0]["roof_elevation"], C["a_core_top"])):
            self.assertAlmostEqual(actual["value"], expected, delta=C["tolerance"])
            self.assertEqual(actual["source"], C["source"])
            self.assertIn(actual["basis"], ("sourced", "derived"))

    def test_placed_capsule_phase_grid(self):
        for tower in M.towers():
            for slot in M.slots():
                if slot.tower != tower.index:
                    continue
                expected = M.podium_height() + (slot.floor - 3 + slot.sublevel / C["phase_count"]) * C["pitch"]
                self.assertAlmostEqual(slot.plane.origin[2], expected, delta=C["tolerance"])
                self.assertAlmostEqual(slot.facade_plane.origin[2], expected + M.value("detail.floor_finish_height").value, delta=C["tolerance"])

    def test_core_and_head_datums(self):
        tower = M.towers()[0]
        walls = [p for p in M.core_assembly(tower).parts if p.role == "core_wall"]
        core_top = max(float(np.max(np.asarray(p.solid.vertices)[:, 2])) for p in walls)
        head_top = float(np.max(np.asarray(M.capital_solid(tower).vertices)[:, 2]))
        self.assertAlmostEqual(core_top, C["a_core_top"], delta=C["tolerance"])
        self.assertAlmostEqual(tower.capital_height, C["a_head"], delta=C["tolerance"])
        self.assertAlmostEqual(head_top, C["a_top"], delta=C["tolerance"])

    def test_podium_office_band(self):
        levels = [q["value"] for q in M.load()["podium"]["levels"]]
        self.assertAlmostEqual(levels[2] - levels[1], C["office_band"], delta=C["tolerance"])
        roofs = [p for p in M.podium_assembly().parts if p.key.startswith("podium-floor-2-")]
        self.assertTrue(roofs)
        np.testing.assert_allclose([np.asarray(p.solid.vertices)[:, 2].max() for p in roofs], levels[2], atol=C["tolerance"], rtol=0)

    def test_basement_actual_floor(self):
        floors = [p for p in M.foundation_assembly().parts if p.role == "basement_floor"]
        self.assertTrue(floors)
        np.testing.assert_allclose([np.asarray(p.solid.vertices)[:, 2].max() for p in floors], -C["basement_depth"], atol=C["tolerance"], rtol=0)

    def test_derived_service_clearance(self):
        data, a = M.load(), M.towers()[0]
        crown = max(s.plane.origin[2] + M.value("capsule.height").value for s in M.slots() if s.tower == a.index)
        value = data["towers"][0]["roof_clearance"]
        self.assertEqual(value["basis"], C["clearance_basis"])
        self.assertAlmostEqual(value["value"], C["a_core_top"] - crown, delta=C["tolerance"])

    def test_roof_profile_keeps_native_corner_proportions(self):
        for tower in M.towers():
            vertices = np.asarray(M.capital_solid(tower).vertices)
            actual = (vertices[4:, 2] - M.capital_base(tower)) / tower.capital_height
            np.testing.assert_allclose(actual, C["native_roof_ratios"], atol=1e-7, rtol=0)

    def test_tower_head_parameters_are_independent(self):
        data = deepcopy(M.load())
        data["towers"][0]["capital_height"]["value"] *= .8
        M.towers.cache_clear()
        try:
            with patch.object(M, "_data", return_value=data):
                actual = {t.key: t.capital_height for t in M.towers()}
        finally:
            M.towers.cache_clear()
        for tower in data["towers"]:
            self.assertAlmostEqual(actual[tower["key"]], tower["capital_height"]["value"])

    def test_registered_podium_is_flat_roof_not_parapet(self):
        self.assertAlmostEqual(M.podium_height(), C["podium_roof"], delta=C["tolerance"])
        scan = C["scan"]
        scale = scan["A_height"] / (scan["A_apex_raw"] - scan["A_ground_raw"])
        roof = (scan["podium_roof_raw"] - scan["A_ground_raw"]) * scale
        self.assertAlmostEqual(M.podium_height(), roof, delta=scan["metric_tolerance"])

    def test_first_capsule_roof_agrees_with_registered_surface(self):
        scan = C["scan"]
        scale = scan["A_height"] / (scan["A_apex_raw"] - scan["A_ground_raw"])
        observed = (scan["A_phase1_roof_raw"] - scan["A_ground_raw"]) * scale
        roofs = [s.plane.origin[2] + M.value("capsule.height").value for s in M.slots() if s.tower == 0 and s.floor == 3]
        np.testing.assert_allclose(roofs, observed, atol=scan["metric_tolerance"], rtol=0)

    def test_b_independent_head_and_total_agree_with_scan(self):
        b = M.towers()[1]
        self.assertAlmostEqual(b.height, C["b_core_top"], delta=C["tolerance"])
        self.assertAlmostEqual(b.capital_height, C["b_head"], delta=C["tolerance"])
        top = np.asarray(M.capital_solid(b).vertices)[:, 2].max()
        self.assertAlmostEqual(top, C["scan"]["B_height"], delta=C["scan"]["metric_tolerance"])

    def test_live_partial_head_uses_explicit_selected_core_top(self):
        for original in M.towers():
            partial = replace(original, height=original.height - C["pitch"])
            vertices = np.asarray(M.capital_solid(partial).vertices)
            self.assertAlmostEqual(M.capital_base(partial), partial.height, delta=C["tolerance"])
            self.assertAlmostEqual(vertices[:, 2].min(), partial.height, delta=C["tolerance"])
            self.assertAlmostEqual(vertices[:, 2].max(), partial.height + partial.capital_height, delta=C["tolerance"])

    def test_core_metadata_uses_actual_structural_floor_datums(self):
        data = M.load()
        for tower in M.towers():
            expected = [q["value"] for q in data["podium"]["levels"][:2]]
            for floor in data["towers"][tower.index]["floors"]:
                expected.append(min(i["center"][2]["value"] - i["sublevel"] * C["pitch"] / C["phase_count"] for g in floor["groups"] for i in g["items"]))
            expected.append(tower.height)
            np.testing.assert_allclose(M.core_solid(tower).meta["levels"], expected, atol=C["tolerance"], rtol=0)

if __name__ == "__main__":
    unittest.main()

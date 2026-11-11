"""🧊 Verify native mesh finishes work without redundant prism fields."""
import importlib
import json
import sys
import unittest
from pathlib import Path

from shapely.geometry import Polygon, box
from shapely.ops import unary_union

ROOT = next(p for p in Path(__file__).resolve().parents if (p / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
CASES = json.loads(Path(__file__).with_name("🔣️.json").read_text(encoding="utf-8"))
MODEL = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
AXON = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.🧊️axon.🧩️primitives")


class NativeFinish(unittest.TestCase):
    def test_mesh_footprint_in_vehicle_band(self):
        """🚘 Compare the native solid's band projection with an independent polygon."""
        rings = MODEL.part_band_footprints(CASES["part"], *CASES["band"])
        result = unary_union([Polygon(ring) for ring in rings])
        self.assertLess(result.symmetric_difference(box(0, 0, 2, 1)).area, CASES["tolerance"])

    def test_screen_tangent_and_positive_section(self):
        """📐 A tangent contributes no poché; an occupied cut retains its physical area."""
        from types import SimpleNamespace
        import numpy as np
        plan = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.📋️plan.🐍️")
        case = CASES["tangent_screen"]
        part = case["part"]
        self.assertEqual(MODEL.part_outline_at(part, case["empty_z"]), [])
        ring = MODEL.part_outline_at(part, case["area_z"])
        self.assertAlmostEqual(Polygon(ring).area, case["area"])
        for z, expected in ((case["empty_z"], 0), (case["area_z"], 1)):
            result = []
            ctx = {"view": lambda x, y, _: np.array([x, y, 0.0]), "look": None, "w": 1,
                   "catalogue": [{"kind": "screens", "tag": "screen", "data": {"base": 0, "top": 1, "parts": [part]}}],
                   "cut_z": z, "bands": {"ground": (0, 1)}, "level": "ground",
                   "put": lambda rank, tag, group: result.append(group)}
            owner = SimpleNamespace(_pair=lambda *args: ("#ff0000", "#ffffff", 1, 0))
            plan.Plan._plan_screens(owner, ctx)
            self.assertEqual(len(result), 1)
            self.assertEqual(len(result[0].submobjects), expected)

    def test_mesh_finish_has_axon_faces_and_placement(self):
        """🔻 A mesh supplies drawable faces, position and elevation without prism fields."""
        centre, z, faces = AXON._finish_geometry(CASES["part"])
        self.assertEqual(len(faces), 6)
        self.assertEqual(tuple(centre), (1, .5, .5))
        self.assertEqual(z, 0)


    def test_plan_and_exploded_axon_accept_mesh_finish(self):
        """🏗️ Both visual paths accept the same native mesh representation."""
        plan = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.📋️plan.🐍️")
        axon = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.🧊️axon.🐍️")
        slab = {"tag": "S", "guid": "S", "parts": [{"outline": [[0, 0], [2, 0], [2, 1], [0, 1]], "z0": -.2, "z1": 0}], "finish_parts": [CASES["part"]]}
        geometry = {"storeys": {"ground": 0}, "box": {"height": 1}, "slabs": [slab]}
        self.assertEqual(plan.model_vertical_bounds(geometry), (-.2, 1))
        from unittest.mock import patch
        with patch.object(axon, "_item", return_value={}):
            pieces = axon._slab_pieces(geometry, {}, slab, 1, 0)
        self.assertEqual(len(pieces), 2)
        self.assertEqual(len(pieces[1].faces), 6)

if __name__ == "__main__":
    unittest.main()

"""⚡ Slab caching preserves mutations and does not repeat physical partition work."""

import importlib
import json
from pathlib import Path
import unittest

from shapely.geometry import Polygon


class SlabPartitionCacheContract(unittest.TestCase):
    def test_same_geometry_reuses_partition_without_stale_mutations(self):
        api = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
        api = importlib.reload(api)
        api._source.cache_clear()
        case = json.loads(Path(__file__).with_suffix(".json").read_text(encoding="utf-8"))
        model = api.load_model()
        slab = api._find(model, case["slab"])
        api._SLAB_GEOMETRIES.clear()
        original = api._slab_geometry_uncached
        builds = []
        def observed(model, slab):
            builds.append(slab["tag"])
            return original(model, slab)
        api._slab_geometry_uncached = observed
        try:
            first = api._slab_geometry(model, slab)
            self.assertEqual(len(builds), case["expected_builds"][0])
            volume = sum(Polygon(p["outline"]).area * (p["z1"] - p["z0"]) for p in first["parts"])
            first["parts"][0]["z0"] = case["mutated_output_z"]
            again = api._slab_geometry(model, slab)
            self.assertNotEqual(again["parts"][0]["z0"], case["mutated_output_z"])
            self.assertEqual(len(builds), case["expected_builds"][1])
            for region in slab["geometry"]["regions"]:
                region["top_offset"]["value"] += case["offset_change"]
            slab["geometry"]["top_offset"]["value"] += case["offset_change"]
            changed = api._slab_geometry(model, slab)
            self.assertEqual(len(builds), case["expected_builds"][2])
            self.assertAlmostEqual(changed["top"] - again["top"], case["offset_change"])
            self.assertAlmostEqual(sum(Polygon(p["outline"]).area*(p["z1"]-p["z0"]) for p in changed["parts"]), volume)
        finally:
            api._slab_geometry_uncached = original


if __name__ == "__main__":
    unittest.main()

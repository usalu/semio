"""🧪 Portable solid-cut and reconstruction contracts for the axonometric drawing."""

import importlib
import json
from pathlib import Path
import unittest

import numpy as np

_axon = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.🧊️axon.🧩️primitives")


class AxonReconstructionContract(unittest.TestCase):
    def test_wall_cells_remove_every_opening(self):
        cases = json.loads(Path(__file__).with_name("🧪️cases.json").read_text(encoding="utf-8"))
        for case in cases["walls"]:
            with self.subTest(case=case["name"]):
                cells = _axon._wall_cells(case["length"], case["base"], case["top"], case["openings"])
                rings = np.array([[[a, c], [b, c], [b, d], [a, d]] for a, b, c, d in cells])
                oracle = abs(np.sum(rings[:, :, 0] * np.roll(rings[:, :, 1], -1, axis=1) - rings[:, :, 1] * np.roll(rings[:, :, 0], -1, axis=1))) / 2
                self.assertAlmostEqual(float(oracle), case["solid_area"])
                for a, b, c, d in cells:
                    for opening in case["openings"]:
                        self.assertFalse(opening["start"] < (a + b) / 2 < opening["end"] and opening["sill_z"] < (c + d) / 2 < opening["head_z"])

    def test_complete_stair_parts_and_arrivals_are_rendered(self):
        geometry = _axon._c.geometry()
        pieces = _axon.prims()
        for stair in geometry["stairs"]:
            matching = [p for p in pieces if p.tag == stair["tag"]]
            self.assertEqual(len(matching), len(stair["parts"]))
            for part,piece in zip(stair["parts"],matching):
                self.assertAlmostEqual(piece.z,part["z0"])
            for landing in stair["landings"]:
                self.assertTrue(any(abs(p.z - landing["z0"]) < 1e-8 for p in matching))

    def test_all_storeys_and_authored_parts_are_drawable(self):
        geometry = _axon._c.geometry()
        pieces = _axon.prims()
        tags = {piece.tag for piece in pieces}
        for kind in ("pilotis", "slabs", "walls", "stairs", "railings", "details", "furnishings"):
            self.assertTrue({item["tag"] for item in geometry.get(kind, ())} <= tags, kind)
        for wall in geometry["walls"]:
            for opening in wall.get("openings", ()):
                tag = opening.get("fill_tag") or opening["tag"]
                self.assertIn(tag, tags)
                if not opening.get("fill_tag"):
                    self.assertFalse(any(face[-1] == "glass" for piece in pieces if piece.tag == tag for face in piece.faces))
        for piece in pieces:
            self.assertIsNotNone(piece.tag)
            self.assertIsNotNone(piece.item["guid"])


if __name__ == "__main__":
    unittest.main()

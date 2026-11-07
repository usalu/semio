"""📏 Physical dimensions of the primary-energy scene against Manim geometry."""
import importlib.util
import json
import unittest
from pathlib import Path
from unittest.mock import patch

import numpy as np
from manim import Rectangle

HERE = Path(__file__).resolve().parent
VECTORS = json.loads((HERE / "🔣️scale.json").read_text(encoding="utf-8"))
SPEC = importlib.util.spec_from_file_location("primary_energy_scale", HERE.parent / "🐍️.py")
SCENE = importlib.util.module_from_spec(SPEC)
SPEC.loader.exec_module(SCENE)


class Scale(unittest.TestCase):
    def test_each_cast_uses_the_same_metre(self):
        unit = SCENE.room_metre(VECTORS["roomHeight"])
        self.assertAlmostEqual(unit, VECTORS["unit"])
        catalog = json.loads((SCENE._WORKSPACE_ROOT / "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔣️.json").read_text(encoding="utf-8"))
        rows = {row["id"]: row for row in catalog["entries"]}
        for vector in VECTORS["assets"]:
            with self.subTest(vector=vector):
                self.assertEqual(rows[vector["asset"]]["heightM"], vector["height"])
                oracle = Rectangle(width=unit, height=vector["height"] * unit)
                calls = []
                def draw(cast, **kwargs):
                    calls.append((cast, kwargs))
                    return Rectangle(width=unit, height=vector["height"] * kwargs["unit"])
                with patch.object(SCENE, "staffage", side_effect=draw), patch.object(SCENE, "staffage_motion", side_effect=draw):
                    if vector["helper"] == "_stand":
                        result = SCENE._stand(0, unit, 0)
                    else:
                        result = getattr(SCENE, vector["helper"])(np.zeros(3), unit)
                self.assertEqual(calls[0][0], vector["cast"])
                self.assertAlmostEqual(calls[0][1]["unit"], unit)
                self.assertAlmostEqual(result.height, oracle.height)

    def test_furnish_shares_the_room_metre(self):
        calls = []
        def draw(cast, **kwargs):
            calls.append((cast, kwargs["unit"]))
            return Rectangle(width=0.2, height=0.2)
        house = {"bottom_left": np.array([0.0, 0.0, 0.0]), "center": np.array([1.0, 0.0, 0.0]), "w_height": 1.8}
        with patch.object(SCENE, "staffage", side_effect=draw), patch.object(SCENE, "staffage_motion", side_effect=draw):
            SCENE._furnish(house, 0.75, -0.3, 0.3)
        self.assertEqual([cast for cast, _ in calls], ["shared-radiator-front-winter", "shared-person-winter"])
        for _, unit in calls:
            self.assertAlmostEqual(unit, VECTORS["unit"])

    def test_scene_separates_the_upstream_scale(self):
        source = (HERE.parent / "🐍️.py").read_text(encoding="utf-8")
        self.assertIn("Vorkette: eigener Maßstab", source)
        self.assertIn("_derrick(np.array([-6.1, G]), upstream_metre)", source)
        self.assertIn("_power_plant(np.array([-4.45, G]), upstream_metre)", source)
        self.assertIn("_boiler(np.array([annex_x, G, 0.0]), metre)", source)
        self.assertIn("room_metre(hs[\"w_height\"] / 2)", source)

    def test_boiler_mounting_uses_floor_datum_and_catalog_bracket(self):
        mount = VECTORS["mounting"]
        asset_path = SCENE._WORKSPACE_ROOT / "🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🔌️appliances/gas-boiler/🔣️.json"
        asset = json.loads(asset_path.read_text(encoding="utf-8"))["asset"]
        bracket = next(anchor for anchor in asset["anchors"] if anchor["id"] == "wall-bracket")
        casing = next(bone for bone in asset["rig"]["bones"] if bone["id"] == bracket["bone"])
        self.assertAlmostEqual(bracket["y"] + casing["y"], mount["bracketLocalM"])
        for vector in mount["cases"]:
            with self.subTest(vector=vector):
                floor, unit = vector["floorY"], vector["unit"]
                recorded = []
                def draw(cast, **kwargs):
                    recorded.append(kwargs["anchor"])
                    return Rectangle(width=unit * 0.44, height=unit * 0.72).move_to(
                        kwargs["anchor"] + np.array([0.0, unit * 0.36, 0.0]))
                with patch.object(SCENE, "staffage_motion", side_effect=draw):
                    boiler = SCENE._boiler(np.array([1.0, floor, 0.0]), unit)
                oracle = Rectangle(width=unit, height=unit * mount["boilerOriginM"]).move_to(
                    np.array([1.0, floor + unit * mount["boilerOriginM"] / 2, 0.0]))
                self.assertAlmostEqual(recorded[0][1], oracle.get_top()[1])
                self.assertAlmostEqual(boiler.get_bottom()[1], oracle.get_top()[1])
                self.assertAlmostEqual(recorded[0][1] + unit * mount["bracketLocalM"],
                                       floor + unit * mount["bracketHeightM"])

    def test_front_radiator_uses_the_floor_origin(self):
        mount = VECTORS["mounting"]
        floor, unit = mount["cases"][0]["floorY"], mount["cases"][0]["unit"]
        calls = []
        def draw(cast, **kwargs):
            calls.append((cast, kwargs["anchor"]))
            return Rectangle(width=0.2, height=0.2)
        house = {"bottom_left": np.array([0.0, floor, 0.0]), "center": np.array([1.0, 0.0, 0.0]), "w_height": 2 * VECTORS["roomHeight"]}
        with patch.object(SCENE, "staffage", side_effect=draw), patch.object(SCENE, "staffage_motion", side_effect=draw):
            SCENE._furnish(house, 0.75, -0.3, 0.3)
        oracle = Rectangle(width=unit, height=unit).move_to(np.array([0.0, floor + unit / 2, 0.0]))
        self.assertEqual(calls[0][0], "shared-radiator-front-winter")
        self.assertAlmostEqual(calls[0][1][1], oracle.get_bottom()[1] + unit * mount["radiatorOriginM"])


if __name__ == "__main__":
    unittest.main()

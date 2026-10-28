"""🧪 Compare authored architectural dimensions with Manim geometry."""

import ast
import importlib
import json
import unittest
from pathlib import Path

import numpy as np

BASE = "🎓️teaching.🏛️architecture.⚡️energy."
CASES = json.loads(Path(__file__).with_name("🔣️scenes.json").read_text(encoding="utf-8"))


class SceneScaleTest(unittest.TestCase):
    def test_physics_uses_canonical_architecture(self):
        shared = importlib.import_module(BASE + "🏗️authoring.🎨️visuals.🐍️")
        physics = importlib.import_module(BASE + "🧲️physics.🎬️clip.🧲️fundamentals.🐍️")
        for local, canonical in CASES["physics_shared"].items():
            with self.subTest(helper=local):
                self.assertIs(getattr(physics, local), getattr(shared, canonical))

    def test_heating_plant_uses_one_physical_scale(self):
        module = importlib.import_module(BASE + "🔥️heating.🎬️clip.🧮️calculation.🐍️")
        plant = module._plant_house()
        scale = plant["room_height"] / CASES["house"]["storey_m"]
        self.assertAlmostEqual(plant["person"].height, scale * CASES["house"]["person_m"], places=6)
        self.assertAlmostEqual(plant["person"].get_bottom()[1], plant["floor_y"], places=6)
        for radiator in plant["radiators"]:
            self.assertAlmostEqual(radiator.height, scale * CASES["house"]["radiator_height_m"], places=6)
            self.assertAlmostEqual(radiator.width, scale * CASES["house"]["radiator_width_m"], places=6)
            self.assertAlmostEqual(radiator.get_bottom()[1] - plant["floor_y"], scale * 0.12, places=6)

    def test_overview_emitters_scale_with_house(self):
        module = importlib.import_module(BASE + "📊️demand.🎬️clip.⚡️demand-power.🐍️")
        for scale in CASES["overview_scales"]:
            house = module._heated_house(np.zeros(3), scale)
            metre = house["storey"] / CASES["house"]["storey_m"]
            for radiator, floor in zip(house["rads"], house["floors"]):
                with self.subTest(scale=scale):
                    self.assertAlmostEqual(radiator.height, metre * CASES["house"]["radiator_height_m"], places=6)
                    self.assertAlmostEqual(radiator.get_bottom()[1] - floor, metre * 0.12, places=6)


    def test_openings_cut_through_section_material(self):
        shared = importlib.import_module(BASE + "🏗️authoring.🎨️visuals.🐍️")
        conduction = importlib.import_module(BASE + "🔥️heating.🎬️clip.🧱️conduction.🐍️")
        colors = {name: shared.P_WHITE for name in ("Dach", "Außenwand", "Fenster", "Haustür", "Boden")}
        house = conduction._envelope_house(np.zeros(3), 0.75, colors)
        self.assertAlmostEqual(house["wall_right"].get_bottom()[1], house["door_top"], places=6)
        regulations = importlib.import_module(BASE + "📊️demand.🎬️clip.⚖️regulations.🐍️")
        house = shared.house_section(np.zeros(3), 0.75)
        glass = regulations._north_window(house)
        parts = house["wall_right"].submobjects
        self.assertEqual(len(parts), 2)
        self.assertAlmostEqual(parts[0].get_top()[1], glass["sill"], places=6)
        self.assertAlmostEqual(parts[1].get_bottom()[1], glass["lintel"], places=6)
        self.assertEqual(glass["inward"], -1)

    def test_heating_closeup_retains_real_dimensions(self):
        module = importlib.import_module(BASE + "🔥️heating.🎬️clip.🧲️fundamentals.🐍️")
        metre = (module.CEIL_Y - module.FLOOR_Y) / CASES["house"]["storey_m"]
        self.assertAlmostEqual(module.WALL_X1 - module.WALL_X0, 0.35 * metre, places=6)
        self.assertAlmostEqual(module._occupant().height, CASES["house"]["person_m"] * metre, places=6)
        self.assertAlmostEqual(module._radiator().height, CASES["house"]["radiator_height_m"] * metre, places=6)


    def test_windows_are_uniformly_scaled_in_overviews(self):
        shared = importlib.import_module(BASE + "🏗️authoring.🎨️visuals.🐍️")
        canonical = shared.window_glyph(0, 0.3, 0.85, depth=0.42, wall=0.14)
        for scale in CASES["overview_scales"]:
            window = shared.window_glyph(0, 0.3 * scale, 0.85 * scale, depth=0.42 * scale, wall=0.14 * scale)
            self.assertAlmostEqual(window["group"].height, canonical["group"].height * scale, places=6)
            self.assertAlmostEqual(window["group"].width, canonical["group"].width * scale, places=6)


    def test_plant_equipment_and_material_use_room_scale(self):
        module = importlib.import_module(BASE + "🔥️heating.🎬️clip.🧮️calculation.🐍️")
        plant = module._plant_house()
        metre = plant["room_height"] / CASES["house"]["storey_m"]
        for dimension in ("width", "height"):
            self.assertAlmostEqual(getattr(plant["boiler"], dimension), metre * CASES["plant"]["boiler_" + dimension + "_m"], places=6)
            self.assertAlmostEqual(getattr(plant["tank"]["frame"], dimension), metre * CASES["plant"]["tank_" + dimension + "_m"], places=6)
        self.assertAlmostEqual(plant["boiler"].get_bottom()[1], plant["cellar"].get_bottom()[1], places=6)
        self.assertAlmostEqual(plant["tank"]["frame"].get_bottom()[1], plant["cellar"].get_bottom()[1], places=6)
        for material in plant["walls"]:
            self.assertGreaterEqual(material.get_fill_opacity(), 0.7)
        np.testing.assert_allclose(plant["flow_paths"][0].get_start(), plant["boiler"].get_left(), atol=1e-6)
        np.testing.assert_allclose(plant["flow_paths"][0].get_end(), plant["tank"]["frame"].get_right(), atol=1e-6)


    def test_occupants_clear_open_window_swing(self):
        module = importlib.import_module(BASE + "🌬️ventilation.🎬️clip.❓️need.🐍️")
        shared = importlib.import_module(BASE + "🏗️authoring.🎨️visuals.🐍️")
        case = CASES["occupancy"]
        center = np.array(case["center"], dtype=float)
        y = center[1] + 0.3
        left = module._window([center[0] - case["width"] / 2, y, 0])
        right = module._window([center[0] + case["width"] / 2, y, 0], inward=-1)
        people = module._room_occupants(center, case["height"], left, right, count=case["count"])
        self.assertEqual(len(people), case["count"])
        left_edge = shared.window_opening(left.window).get_right()[0]
        right_edge = shared.window_opening(right.window).get_left()[0]
        self.assertGreaterEqual(people.get_left()[0] - left_edge, case["minimum_drawing_clearance"])
        self.assertGreaterEqual(right_edge - people.get_right()[0], case["minimum_drawing_clearance"])
        for first, second in zip(people, list(people)[1:]):
            self.assertGreater(second.get_left()[0], first.get_right()[0])


    def test_ventilation_moisture_tracks_placed_occupants(self):
        module = importlib.import_module(BASE + "🌬️ventilation.🎬️clip.❓️need.🐍️")
        tree = ast.parse(Path(module.__file__).read_text(encoding="utf-8"))
        scene = next(node for node in tree.body if isinstance(node, ast.ClassDef) and node.name == "Beat1_Lueftungsbedarf")
        construct = next(node for node in scene.body if isinstance(node, ast.FunctionDef) and node.name == "construct")
        wanted = {"room_c", "room_w", "room_h", "win_y", "left_win", "right_win", "people", "xs", "floor_top", "drops"}
        setup = [node for node in construct.body if isinstance(node, ast.Assign)
                 and any(isinstance(target, ast.Name) and target.id in wanted
                         for root in node.targets for target in ast.walk(root))]
        scope = vars(module).copy()
        exec(compile(ast.Module(body=setup, type_ignores=[]), module.__file__, "exec"), scope)
        people, drops = scope["people"], scope["drops"]
        self.assertEqual(len(drops), CASES["occupancy"]["count"])
        for person, cluster in zip(people, drops):
            self.assertLessEqual(abs(cluster.get_x() - person.get_x() - CASES["occupancy"]["moisture_offset_x"]),
                                 CASES["occupancy"]["moisture_spread_x"])


    def test_solar_readouts_clear_room_material(self):
        module = importlib.import_module(BASE + "❄️cooling.🎬️clip.☀️solar-radiation.🐍️")
        tree = ast.parse(Path(module.__file__).read_text(encoding="utf-8"))
        scene = next(node for node in tree.body if isinstance(node, ast.ClassDef) and node.name == "Beat5_SolarCoolingLoad")
        construct = next(node for node in scene.body if isinstance(node, ast.FunctionDef) and node.name == "construct")
        wanted = {"example", "room", "read_at", "area", "result"}
        setup = [node for node in construct.body if
                 (isinstance(node, ast.Assign) and any(isinstance(target, ast.Name) and target.id in wanted
                  for root in node.targets for target in ast.walk(root)))
                 or (isinstance(node, ast.FunctionDef) and node.name == "_factor_read")]
        scope = vars(module).copy()
        exec(compile(ast.Module(body=setup, type_ignores=[]), module.__file__, "exec"), scope)
        factor = scope["_factor_read"](lambda: CASES["solar_layout"]["factor"], module.P_YELLOW)
        room, gap = scope["room"], CASES["solar_layout"]["clearance"]
        for label in (scope["area"], scope["result"], factor):
            with self.subTest(label=label):
                self.assertLessEqual(label.get_right()[0], room["x_r"] - gap)
                self.assertLessEqual(label.get_top()[1], room["y_c"] - gap)
                self.assertGreaterEqual(label.get_bottom()[1], room["y_f"] + gap)


if __name__ == "__main__":
    unittest.main()

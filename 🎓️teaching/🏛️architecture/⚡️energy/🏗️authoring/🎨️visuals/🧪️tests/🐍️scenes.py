"""🧪 Compare authored architectural dimensions with Manim geometry."""

import ast
import importlib
import json
import os
import unittest
from pathlib import Path
from unittest import mock

import numpy as np
from manim import DOWN, Rectangle, UP

BASE = "🎓️teaching.🏛️architecture.⚡️energy."
CASES = json.loads(Path(__file__).with_name("🔣️scenes.json").read_text(encoding="utf-8"))


class SceneScaleTest(unittest.TestCase):
    def setUp(self):
        language = mock.patch.dict(os.environ, {"SEMIO_TEACHING_LANGUAGE": CASES["language"]})
        language.start()
        self.addCleanup(language.stop)

    def test_physics_uses_canonical_architecture(self):
        shared = importlib.import_module(BASE + "🏗️authoring.🎨️visuals.🐍️")
        physics = importlib.import_module(BASE + "🧲️physics.🎬️clip.🧲️fundamentals.🐍️")
        for local, canonical in CASES["physics_shared"].items():
            with self.subTest(helper=local):
                self.assertIs(getattr(physics, local), getattr(shared, canonical))

    def test_physics_figures_are_sized_staffage(self):
        shared = importlib.import_module(BASE + "🏗️authoring.🎨️visuals.🐍️")
        physics = importlib.import_module(BASE + "🧲️physics.🎬️clip.🧲️fundamentals.🐍️")
        spec = CASES["physics_staffage"]
        foot = np.array([1.0, -2.0, 0.0])
        radiator = physics._staffage(spec["radiator"]["cast"], unit=spec["radiator"]["height"] / physics._staffage_height(spec["radiator"]["cast"]), anchor=foot)
        person = physics._person(spec["person"]["cast"], foot, spec["person"]["height"], motion=False)
        lamp = physics._lamp(np.array([0.0, 1.0, 0.0]), spec["lamp"]["height"])
        for name, figure, height in (("radiator", radiator, spec["radiator"]["height"]),
                                     ("person", person, spec["person"]["height"]),
                                     ("lamp", lamp["group"], spec["lamp"]["height"])):
            with self.subTest(figure=name):
                self.assertGreater(len(figure.submobjects), 0)
                self.assertAlmostEqual(figure.height, height, delta=0.15 * height)
        for name, figure in (("radiator", radiator), ("person", person)):
            with self.subTest(foot=name):
                self.assertAlmostEqual(figure.get_bottom()[1], foot[1], delta=0.06 * figure.height)
                self.assertAlmostEqual(figure.get_center()[0], foot[0], delta=0.5 * figure.width)
        self.assertAlmostEqual(lamp["group"].get_top()[1], 1.0, places=6)
        bulb = lamp["bulb"].get_center()
        self.assertGreater(bulb[1], lamp["group"].get_bottom()[1])
        self.assertLess(bulb[1], lamp["group"].get_center()[1])
        self.assertAlmostEqual(lamp["cable"].get_start()[1], 1.0, places=6)
        self.assertTrue(callable(shared.staffage_height))

    def test_heating_plant_uses_one_physical_scale(self):
        module = importlib.import_module(BASE + "🔥️heating.🎬️clip.🧮️calculation.🐍️")
        with mock.patch.dict(os.environ, {"SEMIO_TEACHING_LANGUAGE": CASES["language"]}):
            plant = module._plant_house()
        scale = plant["room_height"] / CASES["house"]["storey_m"]
        person_m = scale * CASES["house"]["person_m"]
        shared = importlib.import_module(BASE + "🏗️authoring.🎨️visuals.🐍️")
        radiator_m = scale * shared.staffage_height("shared-radiator-thermostat")
        self.assertAlmostEqual(plant["person"].height, person_m, delta=0.1 * person_m)
        self.assertAlmostEqual(plant["person"].get_bottom()[1], plant["floor_y"], delta=0.05 * person_m)
        for radiator in plant["radiators"]:
            self.assertAlmostEqual(radiator.height, radiator_m, delta=0.15 * radiator_m)
            self.assertLessEqual(radiator.width, scale * CASES["house"]["radiator_width_m"] + 1e-6)
            self.assertAlmostEqual(radiator.get_bottom()[1] - plant["floor_y"], 0.0, delta=0.08 * radiator_m)

    def test_heating_plant_equipment_has_one_metre_and_connected_flue(self):
        module = importlib.import_module(BASE + "🔥️heating.🎬️clip.🧮️calculation.🐍️")
        case = CASES["plant"]
        metre = 1.6 / CASES["house"]["storey_m"]
        calls = {}
        radiator_anchors = []
        casts = {
            "shared-boiler-heating-cycle": "boiler",
            "shared-tank-charge-up": "tank",
            "heating-calculation-thermostat": "thermostat",
        }

        def oracle(cast, **kwargs):
            if cast not in casts:
                if cast == "shared-radiator-thermostat":
                    radiator_anchors.append(np.array(kwargs["anchor"]))
                return Rectangle(width=0.3 * metre, height=metre).move_to(np.array(kwargs["anchor"]) + UP * metre / 2)
            name = casts[cast]
            calls[name] = kwargs["unit"]
            width, height = (case[name + "_" + dimension + "_m"] * kwargs["unit"] for dimension in ("width", "height"))
            shape = Rectangle(width=width, height=height).move_to(np.array(kwargs["anchor"]) + UP * (height / 2 if name != "thermostat" else case["thermostat_center_height_m"] * kwargs["unit"]))
            shape.seek = lambda _: None
            return shape

        def height(cast):
            return case[casts[cast] + "_height_m"] if cast in casts else 1.0

        with mock.patch.object(module, "staffage_motion", side_effect=oracle), mock.patch.object(module, "staffage_height", side_effect=height), mock.patch.dict(os.environ, {"SEMIO_TEACHING_LANGUAGE": CASES["language"]}):
            plant = module._plant_house()
        for name, figure in (("boiler", plant["boiler"]), ("tank", plant["tank"]["frame"]), ("thermostat", plant["thermostat"])):
            with self.subTest(equipment=name):
                self.assertAlmostEqual(calls[name], metre, places=9)
                expected = np.array(case[name + "_anchor"]) + (UP * case["thermostat_center_height_m"] * metre if name == "thermostat" else 0)
                np.testing.assert_allclose(figure.get_center() if name == "thermostat" else figure.get_bottom(), expected, atol=1e-9)
                self.assertAlmostEqual(figure.width, case[name + "_width_m"] * metre, places=9)
                self.assertAlmostEqual(figure.height, case[name + "_height_m"] * metre, places=9)
        for i, anchor in enumerate(radiator_anchors):
            support = Rectangle(width=0.01, height=0.01).move_to(anchor + UP * case["radiator_support_height_m"] * metre)
            np.testing.assert_allclose(support.get_center(), case["radiator_contacts"][i], atol=1e-9)
            wall = plant["walls"][i]
            self.assertAlmostEqual(support.get_center()[0], (wall.get_right() if i == 0 else wall.get_left())[0], places=9)
        self.assertEqual(len(radiator_anchors), 2)
        for path, expected in zip(plant["flow_paths"][1:], case["radiator_pipe_ends"]):
            np.testing.assert_allclose(path.get_end(), expected, atol=1e-9)
        np.testing.assert_allclose(plant["chimney"][0].get_bottom(), plant["boiler"].get_top(), atol=1e-9)
        self.assertAlmostEqual(plant["chimney"][0].get_top()[1], case["chimney_top"], places=9)
        np.testing.assert_allclose(plant["flow_paths"][0].get_start(), plant["boiler"].get_left(), atol=1e-9)
        np.testing.assert_allclose(plant["flow_paths"][0].get_end(), plant["tank"]["frame"].get_right(), atol=1e-9)

    def test_heating_pendant_uses_scene_metre(self):
        module = importlib.import_module(BASE + "🔥️heating.🎬️clip.🧮️calculation.🐍️")
        case = CASES["house"]
        metre, top = 0.64, np.array([1.0, 2.0, 0.0])

        def oracle(cast, *, unit):
            self.assertEqual(cast, "shared-lamp-winter")
            self.assertAlmostEqual(unit, metre, places=9)
            return Rectangle(width=case["lamp_width_m"] * unit, height=case["lamp_height_m"] * unit)

        with mock.patch.object(module, "staffage", side_effect=oracle), mock.patch.object(module, "staffage_height", return_value=case["lamp_height_m"]):
            lamp = module._pendant(top, metre)["group"]
        self.assertAlmostEqual(lamp.height, case["lamp_height_m"] * metre, places=9)
        np.testing.assert_allclose(lamp.get_top(), top, atol=1e-9)

    def test_overview_emitters_scale_with_house(self):
        module = importlib.import_module(BASE + "📊️demand.🎬️clip.⚡️demand-power.🐍️")
        shared = importlib.import_module(BASE + "🏗️authoring.🎨️visuals.🐍️")
        for scale in CASES["overview_scales"]:
            house = module._heated_house(np.zeros(3), scale)
            metre = house["storey"] / CASES["house"]["storey_m"]
            nominal = metre * shared.staffage_height("shared-radiator-winter")
            for radiator, floor in zip(house["rads"], house["floors"]):
                with self.subTest(scale=scale):
                    self.assertAlmostEqual(radiator.height, nominal, delta=0.08 * nominal)
                    self.assertAlmostEqual(radiator.get_bottom()[1], floor, delta=0.08 * nominal)


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
        shared = importlib.import_module(BASE + "🏗️authoring.🎨️visuals.🐍️")
        metre =(module.CEIL_Y - module.FLOOR_Y) / CASES["house"]["storey_m"]
        self.assertAlmostEqual(module.WALL_X1 - module.WALL_X0, 0.35 * metre, places=6)
        stage = module._stage()
        self.assertAlmostEqual(stage["occupant"].height, CASES["house"]["person_m"] * metre, delta=0.06 * metre)
        self.assertAlmostEqual(stage["radiator"].height, shared.staffage_height("shared-radiator-front-winter") * metre, delta=0.06 * metre)


    def test_windows_are_uniformly_scaled_in_overviews(self):
        shared = importlib.import_module(BASE + "🏗️authoring.🎨️visuals.🐍️")
        canonical = shared.window_glyph(0, 0.3, 0.85, depth=0.42, wall=0.14)
        for scale in CASES["overview_scales"]:
            window = shared.window_glyph(0, 0.3 * scale, 0.85 * scale, depth=0.42 * scale, wall=0.14 * scale)
            self.assertAlmostEqual(window["group"].height, canonical["group"].height * scale, places=6)
            self.assertAlmostEqual(window["group"].width, canonical["group"].width * scale, places=6)


    def test_plant_equipment_and_material_use_room_scale(self):
        module = importlib.import_module(BASE + "🔥️heating.🎬️clip.🧮️calculation.🐍️")
        with mock.patch.dict(os.environ, {"SEMIO_TEACHING_LANGUAGE": CASES["language"]}):
            plant = module._plant_house()
        metre = plant["room_height"] / CASES["house"]["storey_m"]
        for dimension in ("width", "height"):
            self.assertAlmostEqual(getattr(plant["boiler"], dimension), metre * CASES["plant"]["boiler_" + dimension + "_m"], delta=0.25 * metre * CASES["plant"]["boiler_" + dimension + "_m"])
            self.assertAlmostEqual(getattr(plant["tank"]["frame"], dimension), metre * CASES["plant"]["tank_" + dimension + "_m"], delta=0.25 * metre * CASES["plant"]["tank_" + dimension + "_m"])
        self.assertAlmostEqual(plant["boiler"].get_bottom()[1] - plant["cellar"].get_bottom()[1], CASES["plant"]["boiler_clearance_m"] * metre, delta=0.06)
        self.assertAlmostEqual(plant["tank"]["frame"].get_bottom()[1], plant["cellar"].get_bottom()[1], delta=0.06)
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
        shift = [module._stand(center, case["height"], 0.0, mirror=mirror).get_x() for mirror in (False, True)]
        feet = [person.get_x() - shift[index % 2] for index, person in enumerate(people)]
        body = 0.2 * 0.7 * (case["height"] - 0.16)
        for first, second in zip(feet, feet[1:]):
            self.assertGreater(second - first, body)


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


    def test_transport_casts_are_sized_and_floored_by_their_figure(self):
        shared = importlib.import_module(BASE + "🏗️authoring.🎨️visuals.🐍️")
        case = CASES["transport_figure"]
        for cast in case["casts"]:
            with self.subTest(cast=cast):
                index = shared._staffage._index(shared._staffage.ensure_casts([cast])[cast])
                figure = index["logical"]["figureBoundsM"]
                width, height = figure["maxX"] - figure["minX"], figure["maxY"] - figure["minY"]
                unit = case["height"] / height
                self.assertAlmostEqual(shared.staffage_height(cast), height, delta=case["tolerance"] * height)
                for group in (shared.fitted_staffage(cast, height=case["height"], rate=0.0), shared.fitted_staffage(cast, height=case["height"])):
                    group.align_to(np.array([0.0, case["floor_y"], 0.0]), DOWN)
                    self.assertAlmostEqual(group.height, case["height"], delta=case["tolerance"])
                    self.assertAlmostEqual(group.width, width * unit, delta=case["tolerance"])
                    self.assertAlmostEqual(group.get_bottom()[1], case["floor_y"], delta=case["tolerance"])
                    self.assertGreater(max(member.get_top()[1] for member in group.family_members_with_points()), group.get_top()[1] + case["tolerance"])

    def test_mirror_flips_every_path_about_the_origin_and_swaps_the_transport_bounds(self):
        shared = importlib.import_module(BASE + "🏗️authoring.🎨️visuals.🐍️")
        case = CASES["transport_figure"]
        cast = case["casts"][0]
        figure = shared._staffage._index(shared._staffage.ensure_casts([cast])[cast])["logical"]["figureBoundsM"]
        for mirror in (False, True):
            with self.subTest(mirror=mirror):
                plain, flipped = (shared.staffage(cast, unit=1.0, mirror=flag) for flag in (False, mirror))
                drawn = [[member for member in group.family_members_with_points() if member.get_num_points() > 1] for group in (plain, flipped)]
                self.assertEqual(len(drawn[0]), len(drawn[1]))
                for first, second in zip(*drawn):
                    expected = first.get_all_points() * np.array([-1.0 if mirror else 1.0, 1.0, 1.0])
                    self.assertTrue(np.allclose(second.get_all_points(), expected, atol=1e-9))
                self.assertAlmostEqual(flipped.get_left()[0], -figure["maxX"] if mirror else figure["minX"], delta=case["tolerance"])
                moving = shared.staffage_motion(cast, unit=1.0, mirror=mirror)
                self.assertAlmostEqual(moving.get_right()[0], -figure["minX"] if mirror else figure["maxX"], delta=case["tolerance"])


if __name__ == "__main__":
    unittest.main()

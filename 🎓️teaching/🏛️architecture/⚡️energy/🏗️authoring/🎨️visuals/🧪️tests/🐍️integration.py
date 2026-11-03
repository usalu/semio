"""🧪 Validate authored occupants and emitters with third-party Manim bounds."""

import ast
import importlib
import json
import unittest
from pathlib import Path

import numpy as np

BASE = "🎓️teaching.🏛️architecture.⚡️energy."
CASES = json.loads(Path(__file__).with_name("🔣️integration.json").read_text(encoding="utf-8"))


class SceneIntegrationTest(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        import os
        import tempfile
        from manim import tempconfig
        workspace = next(parent for parent in Path(__file__).parents if (parent / "🧰️framework").is_dir())
        output = Path(os.environ.get("SEMIO_TEACHING_OUTPUT_ROOT", workspace / ".🧬semio/🎓️teaching/⚡️cache/tests"))
        output.mkdir(parents=True, exist_ok=True)
        cls.directory = tempfile.TemporaryDirectory(dir=output)
        cls.configuration = tempconfig({"text_dir": str(Path(cls.directory.name) / "text")})
        cls.configuration.__enter__()
        cls.addClassCleanup(cls.directory.cleanup)
        cls.addClassCleanup(cls.configuration.__exit__, None, None, None)
        cls.shared = importlib.import_module(BASE + "🏗️authoring.🎨️visuals.🐍️")

    def assert_radiator(self, radiator, floor, room_height, cast="shared-radiator-winter"):
        metre = room_height / CASES["physical"]["storey_m"]
        expected = self.shared.staffage_height(cast) * metre
        self.assertAlmostEqual(radiator.height, expected, delta=0.02 * expected)
        self.assertAlmostEqual(radiator.get_bottom()[1], floor, delta=0.02 * expected)
        self.assertLessEqual(radiator.width, CASES["physical"]["radiator_width_m"] * metre + 1e-6)

    def assert_person(self, person, floor, room_height, expected=None):
        expected = CASES["physical"]["person_m"] / CASES["physical"]["storey_m"] * room_height if expected is None else expected
        self.assertGreaterEqual(person.height, 0.95 * expected)
        self.assertLessEqual(person.height, 1.1 * expected)
        self.assertAlmostEqual(person.get_bottom()[1], floor, delta=0.04 * expected)

    def test_demand_emitters_and_occupants(self):
        for chapter, kind in CASES["demand_heaters"].items():
            module = importlib.import_module(BASE + "📊️demand.🎬️clip." + chapter + ".🐍️")
            for scale in CASES["scales"]:
                with self.subTest(chapter=chapter, scale=scale):
                    house = self.shared.house_section(np.array([1.0, -0.4, 0.0]), scale)
                    floor, room_height = house["bottom_left"][1], house["w_height"] / 2
                    home = module._home(np.array([1.0, -0.4, 0.0]), scale) if kind == "home" else None
                    heater_args = (house, 1.0) if kind == "house" else (home,) if kind == "home" else (floor, room_height, 1.0)
                    person_args = (house, 1.0) if kind == "house" else (home, 1.0) if kind == "home" else (floor, room_height, 1.0)
                    cast = CASES["demand_radiator_casts"][chapter]
                    self.assert_radiator(module._heater(*heater_args), floor, room_height, cast)
                    person = module._stand(*person_args)
                    self.assert_person(person, floor, room_height)

    def test_system_losses_and_regulation_stages(self):
        module = importlib.import_module(BASE + "📊️demand.🎬️clip.♨️system-losses.🐍️")
        plant = module._plant()
        self.assert_radiator(plant["radiator"], plant["room"]["y_f"], plant["room"]["h"])
        self.assert_person(plant["person"], plant["room"]["y_f"], plant["room"]["h"])
        module = importlib.import_module(BASE + "📊️demand.🎬️clip.⚖️regulations.🐍️")
        stage = module._stage(with_radiator=True)
        room_height = stage["hs"]["w_height"] / 2
        self.assert_radiator(stage["radiator"], stage["floor_y"], room_height)
        self.assert_person(stage["person"], stage["floor_y"], room_height)

    def test_summer_occupant_follows_house_scale(self):
        module = importlib.import_module(BASE + "❄️cooling.🎬️clip.🌤️summer-strategy.🐍️")
        for scale in CASES["scales"]:
            house = self.shared.house_section(np.zeros(3), scale)
            figure = module._person(house, 0.0)
            self.assert_person(figure, house["bottom_left"][1], house["w_height"] / 2)

    def test_seated_room_occupant_has_human_proportions(self):
        module = importlib.import_module(BASE + "❄️cooling.🎬️clip.👥️internal-gains.🐍️")
        for height in (2.25, 3.4):
            room = self.shared.room_section(np.zeros(3), h=height)
            seated = module._seated(room, -0.5)
            expected = self.shared.staffage_height("cooling-internal-gains-seated-worker") * height / 2.5
            self.assertAlmostEqual(seated["group"].height, expected, delta=0.1 * expected)
            self.assertAlmostEqual(seated["group"].get_bottom()[1], room["y_f"], delta=0.05 * expected)
            self.assertGreater(seated["chest"][1], seated["laptop"][1])
            self.assertGreater(seated["head"][1], seated["chest"][1])
            self.assertLess(seated["head_top"][1], seated["group"].get_top()[1] + 0.05 * expected)

    def test_physics_scene_constructor_bounds(self):
        module = importlib.import_module(BASE + "🧲️physics.🎬️clip.🧲️fundamentals.🐍️")
        tree = ast.parse(Path(module.__file__).read_text(encoding="utf-8"))
        assignments = [node for node in ast.walk(tree) if isinstance(node, ast.Assign) and len(node.targets) == 1
                       and isinstance(node.targets[0], ast.Name)]
        room = self.shared.room_section(np.zeros(3), h=CASES["physics"]["room_height"])
        house = self.shared.house_section(np.array([1.8, -.3, 0.0]))
        scope = dict(vars(module), room=room, fe_house=house)
        for name in ("person_h", "person", "fe_metre", "fe_floor", "fe_heater"):
            nodes = [node for node in assignments if node.targets[0].id == name
                     and (name != "person" or "person_h" in ast.unparse(node))]
            self.assertEqual(len(nodes), 1)
            exec(compile(ast.Module(body=nodes, type_ignores=[]), module.__file__, "exec"), scope)
        self.assert_person(scope["person"], room["y_f"], room["h"])
        self.assert_radiator(scope["fe_heater"], house["bottom_left"][1], house["w_height"] / 2,
                             "shared-radiator-front-winter")
        winter = next(node for node in assignments if node.targets[0].id == "radiator"
                      and "'shared-radiator-winter'" in ast.unparse(node))
        winter_room = next(node for node in assignments if node.targets[0].id == "room"
                           and "window=(0.34, 0.92)" in ast.unparse(node))
        exec(compile(ast.Module(body=[winter_room, winter], type_ignores=[]), module.__file__, "exec"), scope)
        radiator, winter_room = scope["radiator"], scope["room"]
        metre = winter_room["h"] / CASES["physical"]["storey_m"]
        expected = self.shared.staffage_height("shared-radiator-front-winter") * metre
        self.assertAlmostEqual(radiator.height, expected, delta=CASES["physics"]["radiator_tolerance"] * expected)
        self.assertAlmostEqual(radiator.get_bottom()[1], winter_room["y_f"], delta=0.02 * expected)
        self.assertLessEqual(radiator.width, CASES["physical"]["radiator_width_m"] * metre + 1e-6)
        self.assertLess(radiator.get_top()[1], winter_room["win_lo"])
        self.assertAlmostEqual(radiator.get_left()[0], winter_room["x_l"], delta=0.05 * expected)

    def test_windows_text_native_allocations_are_bounded(self):
        import ctypes
        import ctypes.wintypes as w
        import sys
        import tempfile
        from manim import WHITE, tempconfig
        import manimpango
        if sys.platform != "win32":
            self.skipTest("Native Windows font-map regression")
        fonts = importlib.import_module(BASE + "🏗️authoring.🔤️fonts.🐍️")
        class Memory(ctypes.Structure):
            _fields_ = [("cb", w.DWORD), ("PageFaultCount", w.DWORD), *[(name, ctypes.c_size_t) for name in
                         ("PeakWorkingSetSize", "WorkingSetSize", "QuotaPeakPagedPoolUsage", "QuotaPagedPoolUsage",
                          "QuotaPeakNonPagedPoolUsage", "QuotaNonPagedPoolUsage", "PagefileUsage", "PeakPagefileUsage")]]
        kernel = ctypes.WinDLL("kernel32")
        kernel.GetCurrentProcess.restype = w.HANDLE
        psapi = ctypes.WinDLL("psapi")
        psapi.GetProcessMemoryInfo.argtypes = [w.HANDLE, ctypes.c_void_p, w.DWORD]
        def private_bytes():
            result = Memory()
            result.cb = ctypes.sizeof(result)
            self.assertTrue(psapi.GetProcessMemoryInfo(kernel.GetCurrentProcess(), ctypes.byref(result), result.cb))
            return result.PagefileUsage
        settings = CASES["native_font_map"]
        output = next(parent for parent in Path(__file__).parents if (parent / "🧰️framework").is_dir()) / ".🧬semio/🎓️teaching/⚡️cache/tests"
        import os
        output = Path(os.environ.get("SEMIO_TEACHING_OUTPUT_ROOT", output))
        output.mkdir(parents=True, exist_ok=True)
        with tempfile.TemporaryDirectory(dir=output) as directory, tempconfig({"text_dir": directory}):
            fonts.body_text("Native font-map warmup", font_size=22, color=WHITE)
            before = private_bytes()
            registered = set(manimpango.registered_fonts)
            for number in range(settings["unique_labels"]):
                label = fonts.body_text(f"Native font-map sample {number}", font_size=22, color=WHITE)
                self.assertGreater(label.width, 0)
            self.assertEqual(set(manimpango.registered_fonts), registered)
            self.assertLess(private_bytes() - before, settings["max_growth_mb"] * 1048576)

    def test_native_font_registry_is_restored_after_failure(self):
        import sys
        from unittest.mock import patch
        import manimpango
        if sys.platform != "win32":
            self.skipTest("Native Windows font-map regression")
        fonts = importlib.import_module(BASE + "🏗️authoring.🔤️fonts.🐍️")
        fonts.register_fonts()
        registered = set(manimpango.registered_fonts)
        attached = set(fonts._PANGO_ATTACHED_FONTS)
        fonts._PANGO_ATTACHED_FONTS.update(font.path for font in registered if font.type == "win32")
        def fail(*args, **kwargs):
            self.assertFalse(any(font.type == "win32" for font in manimpango.registered_fonts))
            raise ValueError("Native renderer failure")
        try:
            with patch.object(fonts, "_PANGO_TEXT_RENDERER", side_effect=fail), self.assertRaisesRegex(ValueError, "Native renderer failure"):
                fonts._render_pango_text()
            self.assertEqual(set(manimpango.registered_fonts), registered)
        finally:
            fonts._PANGO_ATTACHED_FONTS.clear()
            fonts._PANGO_ATTACHED_FONTS.update(attached)

    def test_authored_captions_fit_in_both_languages(self):
        from manim import VGroup
        for case in CASES["captions"]:
            module = importlib.import_module("🎓️teaching." + case["script"].removesuffix(".py").replace("/", "."))
            narration = next(row for row in getattr(module, case["scene"]).NARRATION if row[0] == case["key"])
            for index, language in ((1, "en"), (2, "de")):
                with self.subTest(scene=case["scene"], language=language):
                    self.assertEqual(narration[index], case[language])
                    caption = self.shared.caption_bar(narration[index])
                    label = caption[1]
                    lines = len(label) if isinstance(label, VGroup) else 1
                    self.assertLessEqual(lines, case["maximum_lines"])
            for previous in case["previous"].values():
                with self.assertRaisesRegex(ValueError, "wrapped to [3-9] lines"):
                    self.shared.caption_bar(previous)

    def test_authored_radiators_have_explicit_dimensions(self):
        energy = Path(__file__).parents[3]
        paths = [energy / "📊️demand" / "🎬️clip" / chapter / "🐍️.py" for chapter in
                 ("♨️system-losses", "⚖️regulations", "🏷️certificate", "🔁️recap", "🧪️practice")]
        paths.append(energy / "🧲️physics" / "🎬️clip" / "🧲️fundamentals" / "🐍️.py")
        for path in paths:
            tree = ast.parse(path.read_text(encoding="utf-8"))
            for node in ast.walk(tree):
                if isinstance(node, ast.Call) and isinstance(node.func, ast.Name) and node.func.id in {"radiator", "_radiator"}:
                    with self.subTest(path=path, line=node.lineno):
                        if node.func.id == "_radiator":
                            self.assertTrue(len(node.args) >= 3 or "height" in {keyword.arg for keyword in node.keywords})
                        else:
                            self.assertTrue({"height", "width"}.issubset({keyword.arg for keyword in node.keywords}))


if __name__ == "__main__":
    unittest.main()

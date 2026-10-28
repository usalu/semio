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

    def assert_radiator(self, radiator, floor, room_height):
        metre = room_height / CASES["physical"]["storey_m"]
        for name, actual in (("radiator_height_m", radiator.height), ("radiator_width_m", radiator.width),
                             ("clearance_m", radiator.get_bottom()[1] - floor)):
            self.assertAlmostEqual(actual, CASES["physical"][name] * metre, places=6)

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
                    self.assert_radiator(module._heater(*heater_args), floor, room_height)
                    person = module._stand(*person_args)
                    self.assertAlmostEqual(person.height, .7 * room_height, places=6)
                    self.assertAlmostEqual(person.get_bottom()[1], floor, places=6)

    def test_system_losses_and_regulation_stages(self):
        module = importlib.import_module(BASE + "📊️demand.🎬️clip.♨️system-losses.🐍️")
        plant = module._plant()
        self.assert_radiator(plant["radiator"], plant["room"]["y_f"], plant["room"]["h"])
        self.assertAlmostEqual(plant["person"].height, .7 * plant["room"]["h"], places=6)
        module = importlib.import_module(BASE + "📊️demand.🎬️clip.⚖️regulations.🐍️")
        stage = module._stage(with_radiator=True)
        room_height = stage["hs"]["w_height"] / 2
        self.assert_radiator(stage["radiator"], stage["floor_y"], room_height)
        self.assertAlmostEqual(stage["person"].get_bottom()[1], stage["floor_y"], places=6)
        self.assertAlmostEqual(stage["person"].height, .7 * room_height, places=6)

    def test_summer_occupant_follows_house_scale(self):
        module = importlib.import_module(BASE + "❄️cooling.🎬️clip.🌤️summer-strategy.🐍️")
        for scale in CASES["scales"]:
            house = self.shared.house_section(np.zeros(3), scale)
            figure = module._person(house, 0.0)
            self.assertAlmostEqual(figure.height, .7 * house["w_height"] / 2, places=6)
            self.assertAlmostEqual(figure.get_bottom()[1], house["bottom_left"][1], places=6)

    def test_seated_room_occupant_has_human_proportions(self):
        module = importlib.import_module(BASE + "❄️cooling.🎬️clip.👥️internal-gains.🐍️")
        for height in (2.25, 3.4):
            room = self.shared.room_section(np.zeros(3), h=height)
            seated = module._seated(room, -0.5)
            self.assertAlmostEqual(seated["figure"].height, height * 1.3 / 2.5, places=6)
            self.assertAlmostEqual(seated["figure"].get_bottom()[1], room["y_f"], places=6)
            self.assertGreater(seated["chest"][1], seated["lap"][1])

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
        self.assertAlmostEqual(scope["person"].height, room["h"] * 1.75 / 2.5, places=6)
        self.assertAlmostEqual(scope["person"].get_bottom()[1], room["y_f"], places=6)
        self.assert_radiator(scope["fe_heater"], house["bottom_left"][1], house["w_height"] / 2)
        node = next(node for node in assignments if node.targets[0].id == "radiator"
                    and "5.45" in ast.unparse(node))
        exec(compile(ast.Module(body=[node], type_ignores=[]), module.__file__, "exec"), scope)
        self.assertAlmostEqual(scope["radiator"].height, CASES["physics"]["conceptual_heater_height"], places=6)
        self.assertAlmostEqual(scope["radiator"].width, CASES["physics"]["conceptual_heater_width"], places=6)

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
                        self.assertTrue({"height", "width"}.issubset({keyword.arg for keyword in node.keywords}))


if __name__ == "__main__":
    unittest.main()

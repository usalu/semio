"""🧪 Live tank fill attachment after container transforms."""

from __future__ import annotations

import importlib
import json
import unittest
from pathlib import Path

import numpy as np

VISUALS = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")
CASES = json.loads(Path(__file__).with_name("🔣️transforms.json").read_text(encoding="utf-8"))


class TransformTest(unittest.TestCase):
    def test_tank_fill_follows_scaled_moved_and_stretched_frame(self):
        from manim import Rectangle, UP

        case = CASES["tank"]
        for transform in case["transforms"]:
            tank = VISUALS.energy_tank(case["center"], width=case["width"], height=case["height"])
            tank["group"].scale(case["scale"]).move_to(case["target"])
            for dimension, factor in enumerate(transform["stretch"]):
                tank["group"].stretch(factor, dimension)
            for level, height in zip(case["levels"], transform["fill_heights"], strict=True):
                with self.subTest(stretch=transform["stretch"], level=level):
                    tank["level"].set_value(level)
                    tank["fill"].update(0)
                    oracle = Rectangle(width=transform["fill_width"], height=height)
                    oracle.move_to(tank["frame"].get_bottom() + UP * (transform["base_inset"] + height / 2))
                    self.assertAlmostEqual(tank["fill"].width, oracle.width)
                    self.assertAlmostEqual(tank["fill"].height, oracle.height)
                    np.testing.assert_allclose(tank["fill"].get_center(), oracle.get_center())
                    self.assertGreater(tank["fill"].get_left()[0], tank["frame"].get_left()[0])
                    self.assertLess(tank["fill"].get_right()[0], tank["frame"].get_right()[0])
                    self.assertGreater(tank["fill"].get_bottom()[1], tank["frame"].get_bottom()[1])
                    self.assertLess(tank["fill"].get_top()[1], tank["frame"].get_top()[1])

    def test_house_material_stays_visible_after_windows_open_with_live_tank(self):
        import os
        import tempfile
        from manim import Scene, VGroup, LEFT, UP, tempconfig

        case = CASES["overview_visibility"]
        heating = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🔥️heating.🎬️clip.🧮️calculation.🐍️")

        class VisibilityScene(Scene):
            def construct(self):
                plant = heating._plant_house()
                keys = ("room", "cellar", "walls", "roof", "windows", "radiators", "thermostat", "store", "boiler", "chimney", "pipes", "person")
                self.add(VGroup(*(plant[key] for key in keys)).scale(0.85).move_to(LEFT * 3.1 + UP * 0.1))
                house = VISUALS.house_section(case["house_center"], case["house_scale"])
                self.add(house["group"])
                initial = set(house["group"].get_family())
                self.wait(case["wait_before"])
                self.play(*(VISUALS.open_window(window) for window in house["windows"]), run_time=case["open_seconds"])
                self.wait(case["wait_after"])
                self.house_retained = initial <= set(self.get_mobject_family_members())

        with tempfile.TemporaryDirectory(dir=os.environ.get("SEMIO_TEACHING_OUTPUT_ROOT")) as directory:
            folder = Path(directory)
            with tempconfig({"pixel_width": case["pixels"][0], "pixel_height": case["pixels"][1], "frame_rate": case["fps"],
                             "write_to_movie": True, "save_last_frame": False, "disable_caching": False,
                             "media_dir": str(folder), "video_dir": str(folder / "v"), "partial_movie_dir": str(folder / "p"),
                             "text_dir": str(folder / "t"), "tex_dir": str(folder / "x"),
                             "progress_bar": "none", "verbosity": "ERROR"}):
                scene = VisibilityScene()
                scene.render()
                image = scene.renderer.get_frame()[:, :, :3]
                self.assertTrue(scene.house_retained)
                camera = scene.renderer.camera
                for point in case["material_probes"]:
                    column = int(round((point[0] / camera.frame_width + 0.5) * camera.pixel_width))
                    row = int(round((0.5 - point[1] / camera.frame_height) * camera.pixel_height))
                    self.assertGreater(int(image[row, column].min()), case["minimum_material_channel"], point)
                left, right, bottom, top = case["window_region"]
                x0 = int((left / camera.frame_width + 0.5) * camera.pixel_width)
                x1 = int((right / camera.frame_width + 0.5) * camera.pixel_width)
                y0 = int((0.5 - top / camera.frame_height) * camera.pixel_height)
                y1 = int((0.5 - bottom / camera.frame_height) * camera.pixel_height)
                pane = image[y0:y1, x0:x1].astype(int)
                cyan = (pane[:, :, 1] > pane[:, :, 0] + 25) & (pane[:, :, 2] > pane[:, :, 0] + 25)
                self.assertGreater(int(cyan.sum()), case["minimum_window_pixels"])



if __name__ == "__main__":
    unittest.main()

"""🧪 The drive ruler and the swept-path car are built once and updated per frame: every sampled frame equals the per-frame rebuild they replaced, point for point and style for style."""

import importlib
import math
import os
import unittest
from pathlib import Path
from unittest.mock import patch

import numpy as np
from manim import tempconfig

module = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.📊️information.🐍️")
model = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
_draft = module._draft
_engine = module._engine

SAMPLES = (module.DRIVE_FROM, 0.2, 0.37, 0.5, 0.71, 0.93, 1.0)


def reference_ruler(scene, t):
    """📏 The per-frame ruler of the previous implementation, rebuilt from scratch at fraction ``t``."""
    length = scene._path["length"]
    x0, y, k = 0.95, -1.35, 5.4 / length
    ruler = _engine.VGroup(_draft.polygon([(x0, y - 0.07), (x0 + length * k, y - 0.07), (x0 + length * k, y + 0.07), (x0, y + 0.07)], None, 1.0, module.MUTED, _draft.W_THIN),
                           _draft.polygon([(x0, y - 0.07), (x0 + t * length * k, y - 0.07), (x0 + t * length * k, y + 0.07), (x0, y + 0.07)], module.INK, 0.7))
    for metres in range(0, int(length) + 1, 5):
        ruler.add(_draft.segments([((x0 + metres * k, y - 0.07, 0), (x0 + metres * k, y - 0.17, 0))], module.MUTED, _draft.W_EDGE),
                  scene._say(scene.number(metres, 0) + (_draft.NARROW + "m" if metres + 5 > length else ""), (x0 + metres * k, y - 0.38), size=_draft.S_SMALL, color=module.MUTED))
    return ruler


def reference_driver(scene, t):
    """🛣️ The per-frame driver of the previous implementation, rebuilt from scratch at fraction ``t``."""
    group = _engine.VGroup()
    half = module.swept_half_metres()
    steps = max(int((t - module.DRIVE_FROM) * 160), 2)
    left, right = [], []
    for k in range(steps + 1):
        x, y, h = scene._pose(module.DRIVE_FROM + (t - module.DRIVE_FROM) * k / steps)
        n = np.array([-math.sin(math.radians(h)), math.cos(math.radians(h))]) * half
        left.append(scene._at(x + n[0], y + n[1]))
        right.append(scene._at(x - n[0], y - n[1]))
    group.add(_draft.polygon(left + right[::-1], module.DATA, 0.14, module.DATA, _draft.W_HAIR, 0.55))
    centre_points = [scene._at(*scene._pose(module.DRIVE_FROM + (t - module.DRIVE_FROM) * k / steps)[:2]) for k in range(steps + 1)]
    group.add(_draft.polyline(centre_points, module.INK, _draft.W_THIN, 0.9, dash=0.06))
    for metres in np.arange(5.0, scene._path["length"], 5.0):
        fraction = metres / scene._path["length"]
        if module.DRIVE_FROM < fraction <= t:
            x, y, h = scene._pose(fraction)
            n = np.array([-math.sin(math.radians(h)), math.cos(math.radians(h))]) * 0.09
            centre = scene._at(x, y)
            group.add(_draft.segments([(centre + np.array([n[0], n[1], 0.0]), centre - np.array([n[0], n[1], 0.0]))], module.INK, _draft.W_EDGE))
    x, y, h = scene._pose(t)
    group.add(scene._car(module.SCALE, h).move_to(scene._at(x, y)))
    return group


def drawn(group):
    """🖍️ Every drawn member of a group as ``(points, stroke, fill, widths)`` in draw order, members without points left out."""
    return [(member.points.copy(), np.asarray(member.get_stroke_opacity(background=False)).tolist(), np.asarray(member.get_fill_opacity()).tolist(),
             float(member.get_stroke_width()), member.get_stroke_color().to_hex(), member.get_fill_color().to_hex())
            for member in group.family_members_with_points() if len(member.points)]


class Rig(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.source = model.load_model()
        geometry = model.building_geometry(cls.source)
        cls.path = model.car_path(cls.source)
        cls.centre = np.array([geometry["box"]["length"] / 2, geometry["box"]["width"] / 2])
        cls.output = Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"])

    def scene(self):
        scene = module.Beat1_DataToInformation.__new__(module.Beat1_DataToInformation)
        scene._m, scene._mid, scene._path, scene._entry = self.source, self.centre, self.path, self.path["entry_pair"]
        return scene

    def assertSame(self, actual, expected):
        first, second = drawn(actual), drawn(expected)
        self.assertEqual(len(first), len(second))
        for index, (a, b) in enumerate(zip(first, second)):
            with self.subTest(member=index):
                np.testing.assert_allclose(a[0], b[0], atol=1e-9, rtol=0)
                self.assertEqual(a[1:], b[1:])

    def run_rig(self, language, build, reference, order):
        with patch.dict(os.environ, {"SEMIO_TEACHING_LANGUAGE": language}), tempconfig({
                "media_dir": str(self.output / "media"), "text_dir": str(self.output / "text"), "pixel_width": 480, "pixel_height": 270}):
            scene = self.scene()
            tracker = _engine.ValueTracker(module.DRIVE_FROM)
            rig = build(scene, tracker)
            for t in order:
                with self.subTest(t=t):
                    tracker.set_value(t)
                    rig.update(0)
                    self.assertSame(rig, reference(scene, t))

    def test_ruler_equals_the_per_frame_rebuild_forward_and_backward(self):
        for language in ("en", "de"):
            with self.subTest(language=language):
                self.run_rig(language, lambda scene, tracker: scene._ruler(tracker), reference_ruler, SAMPLES + SAMPLES[::-1])

    def test_driver_equals_the_per_frame_rebuild_forward_and_backward(self):
        for language in ("en",):
            with self.subTest(language=language):
                self.run_rig(language, lambda scene, tracker: scene._driver(tracker), reference_driver, SAMPLES + SAMPLES[::-1])

    def test_one_frame_builds_no_typeset_text_and_no_staffage(self):
        with patch.dict(os.environ, {"SEMIO_TEACHING_LANGUAGE": "en"}), tempconfig({
                "media_dir": str(self.output / "media"), "text_dir": str(self.output / "text"), "pixel_width": 480, "pixel_height": 270}):
            scene = self.scene()
            tracker = _engine.ValueTracker(module.DRIVE_FROM)
            ruler, mover = scene._ruler(tracker), scene._driver(tracker)
            with patch.object(type(scene), "_say", side_effect=AssertionError("text rebuilt")), patch.object(type(scene), "_car", side_effect=AssertionError("car rebuilt")):
                for t in SAMPLES:
                    tracker.set_value(t)
                    ruler.update(0)
                    mover.update(0)


if __name__ == "__main__":
    unittest.main()

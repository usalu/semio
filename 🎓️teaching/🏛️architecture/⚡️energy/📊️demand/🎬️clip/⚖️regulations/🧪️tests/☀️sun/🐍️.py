"""☀️ Validate the canonical live solar silhouette against native Manim output."""
from __future__ import annotations
import importlib
import json
import os
from pathlib import Path
import sys
import unittest

sys.dont_write_bytecode = True
ROOT = next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())
if str(ROOT) not in sys.path:
    sys.path.insert(0, str(ROOT))
FIXTURE = json.loads(Path(__file__).with_name("🔣️.json").read_text(encoding="utf-8"))
os.environ["SEMIO_TEACHING_LANGUAGE"] = FIXTURE["language"]
from manim import Camera, FadeIn
import numpy as np
_MODULE = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.📊️demand.🎬️clip.⚖️regulations.🐍️")

def pixels(glyph):
    """🖼️ Capture native Cairo pixels without writing temporary media."""
    camera = Camera(pixel_width=480, pixel_height=270)
    camera.capture_mobjects([glyph])
    return camera.pixel_array.copy()

class SunContractTests(unittest.TestCase):
    """🔆 Exercise the real authored stage with its canonical single-contour sun."""

    def test_live_fill_matches_fixture_and_native_raster(self):
        """🎚️ Compare declared solar levels with an independent native glyph reference."""
        stage = _MODULE._stage()
        sun = stage["sun"]
        initial = sun.get_all_points().copy()
        self.assertEqual(len(sun.submobjects), FIXTURE["drawables"])
        for case in FIXTURE["cases"]:
            with self.subTest(level=case["level"]):
                stage["lvl"]["sun"].set_value(case["level"])
                sun.update(0)
                reference = _MODULE.sun_glyph(_MODULE.SUN_C).scale(0.42)
                reference[0].set_fill(opacity=case["fill_opacity"])
                self.assertAlmostEqual(sun[0].get_fill_opacity(), case["fill_opacity"])
                np.testing.assert_array_equal(sun.get_all_points(), initial)
                np.testing.assert_array_equal(sun.get_all_points(), reference.get_all_points())
                np.testing.assert_array_equal(pixels(sun), pixels(reference))

    def test_fade_in_preserves_live_solar_level_updates(self):
        """🌅 Keep the authored updater active through the real FadeIn lifecycle."""
        stage = _MODULE._stage()
        sun = stage["sun"]
        for case in FIXTURE["cases"]:
            with self.subTest(level=case["level"]):
                stage["lvl"]["sun"].set_value(case["level"])
                animation = FadeIn(sun, run_time=0.25)
                animation.begin()
                animation.interpolate(0.5)
                animation.finish()
                self.assertTrue(sun.updaters)
                self.assertAlmostEqual(sun[0].get_fill_opacity(), case["fill_opacity"])
                np.testing.assert_array_equal(pixels(sun), pixels(
                    _MODULE.sun_glyph(_MODULE.SUN_C).scale(0.42).set_fill(opacity=case["fill_opacity"])))

if __name__ == "__main__":
    unittest.main()

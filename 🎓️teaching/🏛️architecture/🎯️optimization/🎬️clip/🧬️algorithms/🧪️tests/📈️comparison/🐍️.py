"""📈 Exercise the actual solver-comparison entrance with native Manim and Cairo."""
import hashlib
import importlib
import json
import os
from pathlib import Path
import sys
import unittest
from unittest.mock import patch

import numpy as np
from PIL import Image

ROOT = next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
MODULE = importlib.import_module("🎓️teaching.🏛️architecture.🎯️optimization.🎬️clip.🧬️algorithms.🐍️")
FIXTURE = json.loads(Path(__file__).with_name("🔣️.json").read_text(encoding="utf-8"))
mn = MODULE.mn


def descendants(animation):
    """🌳 Inspect actual native compositions without replacing their orchestration."""
    yield animation
    for child in getattr(animation, "animations", []):
        yield from descendants(child)


class ComparisonLifecycle(unittest.TestCase):
    """🧪 Keep live solver widgets intact across their authored entrance."""

    def test_actual_native_comparison(self):
        """🎞️ Compare native lifecycle pixels with an independent direct endpoint and affine NumPy points."""
        source = Path(MODULE.__file__)
        initial = hashlib.sha256(source.read_bytes()).hexdigest()
        prepare = importlib.import_module("manim.animation.animation").prepare_animation
        output = Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"])
        output.mkdir(parents=True, exist_ok=True)
        for language in FIXTURE["languages"]:
            with self.subTest(language=language), patch.dict(os.environ, {"SEMIO_TEACHING_LANGUAGE": language}), mn.tempconfig({"pixel_width": 480, "pixel_height": 270, "frame_rate": FIXTURE["fps"], "disable_caching": True}):
                owner = self
                class Probe(MODULE.Beat6_EvolutionarySearch):
                    """🔬 Retain authored geometry while exercising only the comparison stage."""
                    def _clear(self, **kwargs):
                        """🪟 Start this focused stage on an otherwise empty native scene."""
                        return None

                    def _stopping(self, *args):
                        """🛑 End at the comparison checkpoint rather than replaying the later story."""
                        return None

                    def step(self, key, *animations, seconds=1.2):
                        """⏱️ Run native setup, frame updates, interpolation, finish and cleanup."""
                        self.keys.append(key)
                        values = [prepare(value) for value in animations]
                        if self.direct:
                            for value in values:
                                for item in descendants(value):
                                    if isinstance(item, mn.FadeIn):
                                        self.add(item.mobject)
                            if key == "compare":
                                self.runs.set_value(FIXTURE["budget"])
                            self.update_mobjects(0)
                            return
                        group = mn.AnimationGroup(*values, run_time=seconds)
                        group._setup_scene(self)
                        group.begin()
                        previous = 0
                        for alpha in np.linspace(0, 1, max(2, round(seconds * FIXTURE["fps"])) + 1):
                            dt = (float(alpha) - previous) * seconds
                            group.update_mobjects(dt)
                            group.interpolate(float(alpha))
                            self.update_mobjects(dt)
                            if key == "compare":
                                for item in descendants(group):
                                    if isinstance(item, mn.FadeIn) and getattr(item, "last_alpha", 1) < 1:
                                        owner.assertEqual(self.runs.get_value(), FIXTURE["entry_run"])
                            previous = float(alpha)
                        group.finish()
                        group.clean_up_from_scene(self)
                        self.update_mobjects(0)

                native_fade = mn.FadeIn
                class ObservedFade(native_fade):
                    """👁️ Measure entrance progress without changing native interpolation."""
                    def interpolate_mobject(self, alpha):
                        """📏 Retain the real entrance fraction for the ordering assertion."""
                        self.last_alpha = alpha
                        return super().interpolate_mobject(alpha)

                images = []
                with patch.object(mn, "FadeIn", ObservedFade):
                    for direct in (False, True):
                        scene = Probe()
                        scene.direct, scene.keys = direct, []
                        scene.d = MODULE._data()
                        scene.budget = scene.d["budget"]
                        scene.optimum = scene.d["surface"]["minimum"]
                        scene.runs, scene.total = mn.ValueTracker(FIXTURE["entry_run"]), mn.ValueTracker(scene.budget)
                        scene.ramp_fn = scene.ramp()
                        scene.counter = scene._counter()
                        scene.add(scene.runs, scene.total)
                        scene._compare()
                        self.assertEqual(scene.keys, FIXTURE["keys"])
                        self.assertEqual(scene.runs.get_value(), FIXTURE["budget"])
                        ax, curves = scene.compare_parts[0], scene.compare_parts[8]
                        self.assertEqual(len(curves), len(FIXTURE["curves"]))
                        origin = ax.c2p(0, ax.y_range[0])
                        basis_x = ax.c2p(scene.budget, ax.y_range[0]) - origin
                        basis_y = ax.c2p(0, ax.y_range[1]) - origin
                        for curve, key in zip(curves, FIXTURE["curves"]):
                            self.assertTrue(curve.updaters and curve._builder is not None)
                            self.assertEqual(len(scene.d["series"][key]), FIXTURE["used_runs"][key])
                            final_value = scene.d["series"][key][-1]
                            expected = origin + basis_x * FIXTURE["used_runs"][key] / scene.budget + basis_y * (final_value - ax.y_range[0]) / (ax.y_range[1] - ax.y_range[0])
                            np.testing.assert_allclose(curve[1].get_center(), expected, atol=1e-9)
                        scene.camera.reset()
                        scene.camera.capture_mobjects(scene.mobjects)
                        images.append(scene.camera.pixel_array.copy())
                np.testing.assert_array_equal(images[0], images[1])
                Image.fromarray(images[0]).save(output / ("comparison-" + language + ".png"))
        self.assertEqual(hashlib.sha256(source.read_bytes()).hexdigest(), initial, "Authored source changed during the comparison oracle")


if __name__ == "__main__":
    unittest.main()

"""🧩️ Check current algorithm widget ownership with actual native Manim geometry."""
from pathlib import Path
import ast
import hashlib
import importlib
import json
import os
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


def flatten(values):
    """🌳 Inspect native composition children without replacing their execution."""
    for value in values:
        yield value
        yield from flatten(getattr(value, "animations", []))


def widgets(scene, keys):
    """📦 Resolve the semantic current widget groups and their intentionally separate rows."""
    result = []
    for key in keys:
        value = getattr(scene, key)
        result.extend(value if isinstance(value, (list, tuple)) or key == "ledger" else [value])
    return result


class Probe(MODULE.Beat6_EvolutionarySearch):
    """🎬 Exercise actual geometry at the two widget boundaries without replaying later solvers."""

    def _clear(self, **kwargs):
        """🪟 Keep the focused closing test independent of the preceding story."""
        return None

    def _selection(self, generations):
        """🛑 End the focused population prefix before the later tournament."""
        return None

    def _breeding(self, generations):
        """🧬 Leave later generations to the canonical production scene."""
        return None

    def step(self, key, *animations, seconds=1.2):
        """⏱️ Execute real setup, per-frame interpolation, updating and native cleanup."""
        prepare = importlib.import_module("manim.animation.animation").prepare_animation
        values = [prepare(value) for value in animations]
        self.calls[key] = values
        group = mn.AnimationGroup(*values, run_time=seconds)
        group._setup_scene(self)
        group.begin()
        previous = 0
        for alpha in np.linspace(0, 1, max(2, round(seconds * 4)) + 1):
            dt = (float(alpha) - previous) * seconds
            group.update_mobjects(dt)
            group.interpolate(float(alpha))
            self.update_mobjects(dt)
            previous = float(alpha)
        group.finish()
        group.clean_up_from_scene(self)
        self.update_mobjects(0)


class WidgetOwnership(unittest.TestCase):
    """🧪 Verify both-room and energy-widget ownership against the current narrated layout."""

    def scene(self):
        """🏠 Build the actual model, room sections, energy bar and three-row derivation."""
        scene = Probe()
        scene.calls = {}
        scene._prepare()
        scene.solver = scene._method("Pattern search", "Mustersuche", MODULE.PATTERN)
        scene.add(scene.solver)
        return scene

    def evidence(self, scene, name):
        """🖼️ Capture the actual native Cairo endpoint and reject empty/nonfinite geometry."""
        for top in scene.mobjects:
            for member in top.get_family():
                self.assertTrue(np.isfinite(member.points).all())
        scene.camera.reset()
        scene.camera.capture_mobjects(scene.mobjects)
        image = scene.camera.pixel_array
        self.assertGreater(int(np.ptp(image[:, :, :3])), 0)
        output = Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"])
        output.mkdir(parents=True, exist_ok=True)
        Image.fromarray(image).save(output / name)

    def test_genetic_native_boundary(self):
        """🧬 Consume both current rooms and derivation before revealing the population explanation."""
        initial = hashlib.sha256(Path(MODULE.__file__).read_bytes()).hexdigest()
        for language in FIXTURE["languages"]:
            with self.subTest(language=language), patch.dict(os.environ, {"SEMIO_TEACHING_LANGUAGE": language}), mn.tempconfig({"pixel_width": 480, "pixel_height": 270, "frame_rate": 4, "disable_caching": True}):
                scene = self.scene()
                hidden = widgets(scene, FIXTURE["genetic_hidden"])
                scene.add(*hidden, scene.pointer, scene.seasons)
                scene._genetic()
                outgoing = {id(item.mobject) for item in flatten(scene.calls["population"]) if isinstance(item, mn.FadeOut)}
                incoming = {id(item.mobject) for item in flatten(scene.calls["population_2"]) if isinstance(item, mn.FadeIn)}
                self.assertTrue(all(id(item) in outgoing for item in hidden))
                self.assertTrue(all(id(item) in incoming for item in widgets(scene, FIXTURE["genetic_visible"])))
                members = {id(member) for top in scene.mobjects for member in top.get_family()}
                self.assertTrue(all(id(item) not in members for item in [*scene.rooms, *scene.ledger, scene.note_chip]))
                self.evidence(scene, "population-" + language + ".png")
        self.assertEqual(hashlib.sha256(Path(MODULE.__file__).read_bytes()).hexdigest(), initial)

    def test_close_uses_current_native_widgets(self):
        """🏁 Render the actual closing method with both rooms, the bar, note and answer summary."""
        initial = hashlib.sha256(Path(MODULE.__file__).read_bytes()).hexdigest()
        for language in FIXTURE["languages"]:
            with self.subTest(language=language), patch.dict(os.environ, {"SEMIO_TEACHING_LANGUAGE": language}), mn.tempconfig({"pixel_width": 480, "pixel_height": 270, "frame_rate": 4, "disable_caching": True}):
                scene = self.scene()
                scene._close(mn.Mobject(), mn.Mobject())
                introduced = {id(item.mobject) for item in flatten(scene.calls["close"]) if isinstance(item, mn.FadeIn)}
                self.assertTrue(all(id(item) in introduced for item in widgets(scene, FIXTURE["close_visible"])))
                members = {id(member) for top in scene.mobjects for member in top.get_family()}
                self.assertTrue(all(id(item) not in members for item in widgets(scene, FIXTURE["close_hidden"])))
                self.evidence(scene, "close-" + language + ".png")
        self.assertEqual(hashlib.sha256(Path(MODULE.__file__).read_bytes()).hexdigest(), initial)

    def test_retired_consumers_are_absent(self):
        """🔍 Reject obsolete singular-room/table consumers in the current authored source."""
        tree = ast.parse(Path(MODULE.__file__).read_text(encoding="utf-8"))
        attributes = {node.attr for node in ast.walk(tree) if isinstance(node, ast.Attribute) and isinstance(node.value, ast.Name) and node.value.id == "self"}
        names = {node.id for node in ast.walk(tree) if isinstance(node, ast.Name)}
        self.assertFalse(set(FIXTURE["retired_consumers"]) & attributes)
        self.assertFalse({name for name in FIXTURE["retired_consumers"] if name.isupper()} & names)


if __name__ == "__main__":
    unittest.main()

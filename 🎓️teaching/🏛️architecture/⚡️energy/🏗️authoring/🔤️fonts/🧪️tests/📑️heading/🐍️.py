"""📑️ Validate standalone instructional headings and title-free authored headers."""
from pathlib import Path
import ast
import importlib
import json
import os
import sys
import tempfile
import unittest

ROOT = next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
PACKAGE = "🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring"
FIXTURE = json.loads(Path(__file__).with_name("🔣️.json").read_text(encoding="utf-8"))

class StandaloneHeading(unittest.TestCase):
    """🏷️ Keep instructional content while removing duplicated video-name chrome."""
    def test_canonical_sources_use_only_current_heading_api(self):
        """🧬 Reject removed title APIs and obsolete persistent scene-title fields."""
        runtime = importlib.import_module(PACKAGE + ".🎬️clip.⚙️runtime.🐍️")
        scripts = {ROOT / "🎓️teaching" / job["segment"]["script"] for job in runtime.source_plan(runtime.segment_plan(runtime.load_manifest(source_ids=[])))}
        scripts.update(ROOT / f"🎓️teaching/🏛️architecture/{subject}/🎬️clip/🎨️visuals/🐍️.py" for subject in ("🏗️bim", "🎯️optimization"))
        for script in sorted(scripts):
            with self.subTest(script=script.relative_to(ROOT).as_posix()):
                tree = ast.parse(script.read_text(encoding="utf-8"))
                names = [(node.lineno, node.attr) for node in ast.walk(tree) if isinstance(node, ast.Attribute)
                         and node.attr in {*FIXTURE["removed_apis"], "_title"}]
                names.extend((node.lineno, node.id) for node in ast.walk(tree) if isinstance(node, ast.Name)
                             and node.id in FIXTURE["removed_apis"])
                self.assertEqual(names, [])
                importlib.import_module(".".join(script.relative_to(ROOT).with_suffix("").parts))
        fonts = importlib.import_module(PACKAGE + ".🔤️fonts.🐍️")
        for name in FIXTURE["removed_apis"]:
            self.assertFalse(hasattr(fonts, name), name)

    def test_standalone_geometry_matches_independent_native_position(self):
        """📐 Compare the actual chip geometry to an independently positioned native reference."""
        import numpy as np
        from manim import Scene, config, tempconfig
        from PIL import Image
        fonts = importlib.import_module(PACKAGE + ".🔤️fonts.🐍️")
        visuals = importlib.import_module(PACKAGE + ".🎨️visuals.🐍️")
        output = Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"])
        output.mkdir(parents=True, exist_ok=True)
        with tempfile.TemporaryDirectory(dir=output) as directory, tempconfig({"media_dir": directory, "text_dir": str(Path(directory) / "t")}):
            fonts.apply_body_font()
            for sample in FIXTURE["heading"]["samples"]:
                with self.subTest(language=sample["language"]):
                    actual = fonts.section_heading(sample["text"])
                    label = fonts.centered_body_text(sample["text"], font_size=fonts.LABEL_FONT_SIZE,
                        color=visuals.P_PRIMARY, max_width=FIXTURE["heading"]["max_text_width"])
                    expected = visuals.window_chip(label, color=visuals.P_PRIMARY)
                    left = -config.frame_width / 2 + fonts._theme.UI_SPACING * FIXTURE["heading"]["left_spacing"]
                    top = config.frame_height / 2 - fonts._theme.UI_SPACING * FIXTURE["heading"]["top_spacing"]
                    expected.move_to(np.array([left + expected.width / 2, top - expected.height / 2, 0]))
                    np.testing.assert_allclose(actual.get_all_points(), expected.get_all_points(), atol=1e-8)
                    self.assertAlmostEqual(actual.get_left()[0], left)
                    self.assertAlmostEqual(actual.get_top()[1], top)
                    self.assertTrue(np.isfinite(actual.get_all_points()).all())
                    scene = Scene()
                    fonts.apply_scene_style(scene)
                    scene.camera.capture_mobjects([actual])
                    Image.fromarray(scene.camera.pixel_array).save(output / (sample["language"] + "-heading.png"))

    def test_actual_source_header_has_instruction_but_no_video_title(self):
        """🎬 Execute the current authored header and inspect real visible native text and pixels."""
        from manim import Scene, Text, tempconfig
        import numpy as np
        case = FIXTURE["actual_header"]
        module = importlib.import_module(case["module"])
        source = Path(module.__file__).read_text(encoding="utf-8")
        cls = next(node for node in ast.parse(source).body if isinstance(node, ast.ClassDef) and node.name == case["scene"])
        construct = next(node for node in cls.body if isinstance(node, ast.FunctionDef) and node.name == "construct")
        end = next(index + 1 for index, node in enumerate(construct.body)
                   if isinstance(node, ast.Expr) and isinstance(node.value, ast.Call)
                   and isinstance(node.value.func, ast.Attribute) and node.value.func.attr == "play"
                   and any(isinstance(child, ast.Name) and child.id == "subtitle" for child in ast.walk(node.value)))
        code = compile(ast.fix_missing_locations(ast.Module(body=construct.body[:end], type_ignores=[])), str(module.__file__), "exec")
        output = Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"])
        output.mkdir(parents=True, exist_ok=True)
        with tempfile.TemporaryDirectory(dir=output) as directory, tempconfig({"media_dir": directory, "pixel_width": 480, "pixel_height": 270}):
            scene = getattr(module, case["scene"])()
            scene.play = lambda *animations, **kwargs: scene.add(*(animation.mobject for animation in animations))
            exec(code, vars(module), {"self": scene})
            texts = [part.original_text for item in scene.mobjects for part in item.get_family() if isinstance(part, Text)]
            self.assertNotIn(getattr(module, case["forbidden"]), texts)
            self.assertIn(case["instruction"], texts)
            scene.camera.capture_mobjects(scene.mobjects)
            self.assertGreater(np.ptp(scene.camera.pixel_array[..., :3]), 0)
            from PIL import Image
            Image.fromarray(scene.camera.pixel_array).save(output / "actual-source-header.png")

if __name__ == "__main__":
    unittest.main()

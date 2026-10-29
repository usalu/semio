"""🎭 Validate actual BIM scene-owned presentation and narration contracts."""
import importlib
import json
from pathlib import Path
import sys
import unittest

ROOT = next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
CASES = json.loads(Path(__file__).with_name("🔣️.cases.json").read_text(encoding="utf-8"))


class BimPresentationContract(unittest.TestCase):
    """🏛️ Require every localized BIM leaf to implement its declared presentation."""


    @classmethod
    def setUpClass(cls):
        """🔐 Capture current authored BIM files before importing native scene implementations."""
        cls.inputs = cls.source_hashes()

    @classmethod
    def source_hashes(cls):
        """📦 Bind each focused run to finite current Python and JSON inputs."""
        import hashlib
        import os

        clip = ROOT / "🎓️teaching/🏛️architecture/🏗️bim/🎬️clip"
        selected = set(filter(None, os.environ.get("SEMIO_TEST_BIM_TOPICS", "").split(",")))
        omitted = {Path(case["script"]).parts[-2] for case in CASES["chapters"]
                   if selected and case["topic"] not in selected}
        return {str(path): hashlib.sha256(path.read_bytes()).hexdigest()
                for path in clip.rglob("*") if path.suffix in {".py", ".json"}
                and path.relative_to(clip).parts[0] not in omitted
                and "🧪️tests" not in path.parts and "🧪️cases" not in path.parts}

    @classmethod
    def tearDownClass(cls):
        """🔒 Reject a stale GREEN if a concurrent author changed the tested inputs."""
        current = cls.source_hashes()
        if cls.inputs != current:
            changed = sorted(path for path in cls.inputs.keys() | current.keys() if cls.inputs.get(path) != current.get(path))
            print("[DEBUG] Changed BIM inputs", changed, flush=True)
            raise AssertionError("BIM authored inputs changed during this focused run; rerun against current files")

    def test_dispatch_contract(self):
        """📜 Match real scene handlers and the complete authored narration sequence."""
        for chapter in CASES["chapters"]:
            with self.subTest(topic=chapter["topic"]):
                module = "🎓️teaching." + chapter["script"].removesuffix(".py").replace("/", ".")
                scene = getattr(importlib.import_module(module), chapter["scene"])
                handler = "present_" + chapter["topic"]
                self.assertTrue(handler in scene.__dict__, handler)
                self.assertTrue(callable(getattr(scene, handler)))
                self.assertEqual([row[0] for row in scene.NARRATION], chapter["keys"])
                self.assertTrue(all(len(row) == 3 and row[1].strip() and row[2].strip() for row in scene.NARRATION))

    def test_native_lifecycle(self):
        """🎞️ Exercise real animation setup, interpolation, cleanup and Cairo in both languages."""
        from unittest.mock import patch
        import os

        import numpy as np
        from manim import tempconfig
        from PIL import Image

        output = Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"])
        output.mkdir(parents=True, exist_ok=True)
        selected = set(filter(None, os.environ.get("SEMIO_TEST_BIM_TOPICS", "").split(",")))
        self.assertTrue(selected <= {item["topic"] for item in CASES["chapters"]})
        for chapter in CASES["chapters"]:
            if selected and chapter["topic"] not in selected:
                continue
            for language in CASES["languages"]:
                with self.subTest(topic=chapter["topic"], language=language), patch.dict(os.environ, {
                    "SEMIO_TEACHING_LANGUAGE": language, "SEMIO_TEACHING_AUDIENCE": "spectator"
                }), tempconfig({"media_dir": str(output / "media"), "text_dir": str(output / "text"),
                                "pixel_width": 480, "pixel_height": 270, "frame_rate": 4,
                                "renderer": "cairo", "dry_run": True, "disable_caching": True}):
                    module = "🎓️teaching." + chapter["script"].removesuffix(".py").replace("/", ".")
                    print(f"[DEBUG] BIM lifecycle {chapter['topic']} {language}", flush=True)
                    base = getattr(importlib.import_module(module), chapter["scene"])
                    seen, played = [], set()

                    def play(scene, *animations, **kwargs):
                        """🎬 Run native lifecycle at three interpolation points without video output."""
                        compiled = scene.compile_animations(*animations, **kwargs)
                        scene.add_mobjects_from_animations(compiled)
                        scene.animations = compiled
                        scene.begin_animations()
                        for animation in compiled:
                            pending = [animation]
                            while pending:
                                item = pending.pop()
                                played.add(type(item).__name__)
                                pending.extend(getattr(item, "animations", ()))
                        for alpha in (0.0, 0.5, 1.0):
                            for animation in compiled:
                                animation.update_mobjects(0.25)
                                animation.interpolate(alpha)
                            scene.update_mobjects(0.25)
                        for animation in compiled:
                            animation.finish()
                            animation.clean_up_from_scene(scene)
                        scene.update_mobjects(0)
                        scene.renderer.time += max((animation.run_time for animation in compiled), default=0)

                    def wait(scene, duration=1, **kwargs):
                        """⏱️ Advance the real scene clock without encoding narration holds."""
                        scene.update_mobjects(0.25)
                        scene.renderer.time += duration

                    def step(scene, key, *animations, **kwargs):
                        """🔎 Observe narration while retaining its real caption and animation path."""
                        seen.append(key)
                        result = super(probe, scene).step(key, *animations, **kwargs)
                        caption = set(scene._caption.get_family())
                        body = [member for member in scene.mobjects if member not in caption]
                        self.assertTrue(any(len(member.points) for item in body for member in item.get_family()), key)
                        scene.camera.reset()
                        scene.camera.capture_mobjects(body)
                        body_pixels = scene.camera.pixel_array
                        self.assertTrue(np.any(body_pixels != body_pixels[0, 0]), key)
                        for member in scene.get_mobject_family_members():
                            self.assertTrue(np.isfinite(member.points).all(), key)
                        scene.camera.reset()
                        scene.camera.capture_mobjects(scene.mobjects)
                        Image.fromarray(scene.camera.pixel_array).save(
                            output / (chapter["topic"] + "-" + language + "-" + key + ".png"))
                        return result

                    probe = type("NativeBimProbe", (base,), {"play": play, "wait": wait, "step": step})
                    scene = probe()
                    scene.construct()
                    self.assertEqual(seen, chapter["keys"])
                    self.assertTrue(set(CASES["animations"]) <= played, played)
                    family = scene.get_mobject_family_members()
                    self.assertTrue(family)
                    self.assertNotIn(scene._caption, family)
                    for member in family:
                        self.assertTrue(np.isfinite(member.points).all())
                    before = [member.points.copy() for member in family if len(member.points)]
                    scene.update_mobjects(0.25)
                    after = [member.points for member in scene.get_mobject_family_members() if len(member.points)]
                    self.assertEqual(len(before), len(after))
                    for left, right in zip(before, after, strict=True):
                        np.testing.assert_array_equal(left, right)
                    scene.camera.reset()
                    scene.camera.capture_mobjects(scene.mobjects)
                    pixels = scene.camera.pixel_array
                    self.assertEqual(pixels.shape[:2], (270, 480))
                    self.assertTrue(np.any(pixels != pixels[0, 0]))
                    Image.fromarray(pixels).save(output / (chapter["topic"] + "-" + language + ".png"))


    def test_literal_dependency_closure(self):
        """🔐 Include every actual imported BIM widget and model provider in render provenance."""
        runtime = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎬️clip.⚙️runtime.🐍️")
        teaching = ROOT / "🎓️teaching"
        required = {teaching / path for path in CASES["dependencies"]}
        for chapter in CASES["chapters"]:
            with self.subTest(topic=chapter["topic"]):
                actual = set(runtime.source_dependencies(teaching / chapter["script"]))
                missing = sorted(str(path.relative_to(teaching)) for path in required - actual)
                self.assertEqual(missing, [])


    def test_window_identity_matches_canonical_model(self):
        """🪟 Keep the drawable window identity distinct from its geometric opening."""
        model = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
        catalogue = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.🧱️core.🐍️")
        expected = CASES["window"]
        element = model.element(expected["tag"])
        self.assertEqual(element["class"], expected["class"])
        self.assertEqual(element["guid"], expected["guid"])
        entries = catalogue.catalogue()
        windows = [item for item in entries if item["kind"] == "windows"]
        matches = [item for item in windows if item["tag"] == expected["tag"]]
        self.assertEqual(len(matches), 1)
        self.assertEqual(matches[0]["guid"], element["guid"])
        self.assertEqual(matches[0]["cls"], element["class"])
        self.assertFalse(any(item["tag"] == expected["opening"] for item in windows))


    def test_authored_dock_points_are_canonical_3d(self):
        """📍 Keep authored thumbnail destinations compatible with native 3D centers."""
        import numpy as np

        module = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.👁️representations.🐍️")
        self.assertEqual(set(module.SLOTS), set(CASES["slots"]))
        for name, expected in CASES["slots"].items():
            np.testing.assert_array_equal(np.asarray(module.SLOTS[name]), expected)

    def test_plan_geometry_and_identity(self):
        """📋 Compare native plan projection with independent NumPy coordinates and Cairo pixels."""
        import os
        from unittest.mock import patch

        import numpy as np
        from manim import tempconfig
        from PIL import Image

        visuals = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🐍️")
        model = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
        case = CASES["plan"]
        output = Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"])
        output.mkdir(parents=True, exist_ok=True)
        with patch.dict(os.environ, {"SEMIO_TEACHING_LANGUAGE": "en"}), tempconfig({
                "media_dir": str(output / "media"), "text_dir": str(output / "text"),
                "pixel_width": 480, "pixel_height": 270, "renderer": "cairo", "dry_run": True}):
            scene = visuals.ClipScene()
            expected = np.array(case["point"], dtype=float)
            expected[:2] += case["scale"] * (np.array(case["known_point"][:2]) - np.array(case["centre"][:2]))
            for level in case["levels"]:
                with self.subTest(level=level["id"]):
                    drawing = scene.villa_plan(level["id"], case["point"], case["scale"], model=model.load_model(), **level.get("kwargs", {}))
                    self.native_fade_probe(scene, drawing)
                    self.assertGreater(drawing.width, 0)
                    self.assertGreater(drawing.height, 0)
                    self.assertTrue(set(level["parts"]) <= set(drawing.parts))
                    np.testing.assert_allclose(drawing.at(*case["known_point"]), expected, atol=1e-8)
                    if "C2" in level["parts"]:
                        np.testing.assert_allclose(drawing.at("C2"), expected, atol=1e-8)
                    for member in drawing.get_family():
                        self.assertTrue(np.isfinite(member.points).all())
                    scene.camera.reset()
                    scene.camera.capture_mobjects([drawing])
                    pixels = scene.camera.pixel_array
                    self.assertTrue(np.any(pixels != pixels[0, 0]))
                    Image.fromarray(pixels).save(output / ("plan-" + level["id"] + ".png"))


    def test_native_drawing_provider_contracts(self):
        """🏛️ Require current section, detail and analysis interfaces to draw finite semantic geometry."""
        import os
        from unittest.mock import patch

        import numpy as np
        from manim import tempconfig
        from PIL import Image

        visuals = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🐍️")
        model = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
        source = model.load_model()
        output = Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"])
        output.mkdir(parents=True, exist_ok=True)
        with patch.dict(os.environ, {"SEMIO_TEACHING_LANGUAGE": "en"}), tempconfig({
                "media_dir": str(output / "media"), "text_dir": str(output / "text"),
                "pixel_width": 480, "pixel_height": 270, "renderer": "cairo", "dry_run": True}):
            scene = visuals.ClipScene()
            for case in CASES["providers"]:
                with self.subTest(provider=case["id"]):
                    args = [getattr(model, value["model_call"])(source) if isinstance(value, dict) and "model_call" in value else value
                            for value in case["args"]]
                    kwargs = dict(case.get("kwargs", {}))
                    if case.get("model", True):
                        kwargs["model"] = source
                    drawing = getattr(scene, case["method"])(*args, **kwargs)
                    self.native_fade_probe(scene, drawing)
                    self.assertTrue(drawing.family_members_with_points())
                    if case.get("parts"):
                        self.assertTrue(set(case["parts"]) <= set(drawing.parts))
                    for tag, dimensions in case.get("spans", {}).items():
                        for attribute, expected in dimensions.items():
                            self.assertAlmostEqual(float(getattr(drawing.parts[tag], attribute)), expected, places=7)
                    if "projection" in case:
                        projection = case["projection"]
                        expected = np.asarray(args[0], dtype=float) + float(args[1]) * (
                            np.asarray(projection["basis"]) @ (np.asarray(projection["source"]) - np.asarray(projection["centre"])))
                        np.testing.assert_allclose(drawing.at(*projection["source"]), expected, atol=1e-8)
                    if case["method"] == "analysis_view":
                        analysis = args[0]
                        anchors = set(drawing.parts) | set(drawing.anchors)
                        self.assertTrue({"C2", analysis["shells"][0]["id"], analysis["supports"][0]["node"]} <= anchors)
                    for member in drawing.get_family():
                        self.assertTrue(np.isfinite(member.points).all())
                    scene.camera.reset()
                    scene.camera.capture_mobjects([drawing])
                    pixels = scene.camera.pixel_array
                    self.assertTrue(np.any(pixels != pixels[0, 0]))
                    Image.fromarray(pixels).save(output / ("provider-" + case["id"] + ".png"))


    def test_information_entry_dimension_matches_model_grid(self):
        """📏 Compare the current entry dimension with portable model coordinates and NumPy projection."""
        import os
        from unittest.mock import patch

        import numpy as np
        from manim import tempconfig

        module = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.📊️information.🐍️")
        model = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
        source = model.load_model()
        geometry = model.building_geometry(source)
        expected = CASES["information_dimension"]
        self.assertEqual(model.car_path(source)["entry_pair"], expected["pair"])
        output = Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"])
        centre = np.array([geometry["box"]["length"] / 2, geometry["box"]["width"] / 2])

        for language in CASES["languages"]:
            with self.subTest(language=language), patch.dict(os.environ, {"SEMIO_TEACHING_LANGUAGE": language}), tempconfig({
                    "media_dir": str(output / "media"), "text_dir": str(output / "text"),
                    "pixel_width": 480, "pixel_height": 270}):
                scene = module.Beat1_DataToInformation.__new__(module.Beat1_DataToInformation)
                scene._m, scene._g, scene._mid, scene._entry = source, geometry, centre, expected["pair"]
                native, calls = scene.dimension, []

                def dimension(*args, **kwargs):
                    """🔎 Capture real authored arguments while retaining native dimension construction."""
                    calls.append(args)
                    return native(*args, **kwargs)

                scene.dimension = dimension
                for override in expected["x_overrides"]:
                    points = np.array(expected["points"], dtype=float)
                    if override is not None:
                        points[1, 0] = override
                    label = scene.quantity(points[1, 0] - points[0, 0], "m")
                    drawing = scene._plan_dim(label, x_b=override)
                    first, last, label = calls[-1]
                    projected = module.ORIGIN + module.SCALE * np.column_stack((points - centre, np.zeros(2)))
                    np.testing.assert_allclose(first, projected[0], atol=1e-8)
                    np.testing.assert_allclose(last, projected[1], atol=1e-8)
                    self.assertEqual(label, scene.quantity(points[1, 0] - points[0, 0], "m"))
                    self.assertTrue(all(np.isfinite(member.points).all() for member in drawing.get_family()))


    def test_current_authored_analysis_beats_are_implemented(self):
        """🧩 Reject the actual missing authored model-story beats before a full render."""
        import ast

        chapter = next(case for case in CASES["chapters"] if case["topic"] == "models")
        tree = ast.parse((ROOT / "🎓️teaching" / chapter["script"]).read_text(encoding="utf-8"))
        scene = next(node for node in tree.body if isinstance(node, ast.ClassDef) and node.name == chapter["scene"])
        methods = {node.name: node for node in scene.body if isinstance(node, ast.FunctionDef)}
        for name in CASES["analysis_beats"]:
            with self.subTest(beat=name):
                self.assertIn(name, methods)
                self.assertFalse(any(isinstance(node, ast.Raise) and (
                    isinstance(node.exc, ast.Name) and node.exc.id == "NotImplementedError"
                    or isinstance(node.exc, ast.Call) and isinstance(node.exc.func, ast.Name) and node.exc.func.id == "NotImplementedError")
                    for node in ast.walk(methods[name])), name)


    def test_authored_renderer_calls_match_current_interface(self):
        """🔌 Reject invalid current renderer keywords before a long authored scene reaches them."""
        import ast
        import inspect

        visuals = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🐍️")
        for case in CASES["invalid_renderer_calls"]:
            signature = inspect.signature(getattr(visuals.ClipScene, case["renderer"]))
            with self.assertRaises(TypeError):
                signature.bind_partial(None, **case["keywords"])
        for chapter in CASES["chapters"]:
            tree = ast.parse((ROOT / "🎓️teaching" / chapter["script"]).read_text(encoding="utf-8"))
            for call in ast.walk(tree):
                if not isinstance(call, ast.Call) or not isinstance(call.func, ast.Attribute):
                    continue
                if not isinstance(call.func.value, ast.Name) or call.func.value.id != "self" or call.func.attr not in CASES["renderer_calls"]:
                    continue
                with self.subTest(topic=chapter["topic"], renderer=call.func.attr, line=call.lineno):
                    signature = inspect.signature(getattr(visuals.ClipScene, call.func.attr))
                    signature.bind_partial(None, *[None for arg in call.args if not isinstance(arg, ast.Starred)],
                                           **{keyword.arg: None for keyword in call.keywords if keyword.arg is not None})


    def native_fade_probe(self, scene, drawing):
        """🫥 Exercise real native copy/fade lifecycle without changing the original drawing."""
        from manim import FadeIn, FadeOut

        for name in CASES["native_methods"]:
            self.assertTrue(callable(getattr(drawing, name)), name)
        candidate = drawing.copy()
        for animation in (FadeIn(candidate), FadeOut(candidate)):
            animation._setup_scene(scene)
            animation.begin()
            animation.interpolate(0.5)
            animation.finish()
            animation.clean_up_from_scene(scene)


    def test_consuming_sequence_starts_morph_at_stage_boundary(self):
        """🎬 Compare a current fade/morph sequence with its independent native Cairo endpoint."""
        import os

        import numpy as np
        from manim import Circle, FadeIn, ReplacementTransform, Square, VGroup, tempconfig

        module = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🔌️exchange.🐍️")
        visuals = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🐍️")
        output = Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"])
        with tempconfig({"media_dir": str(output / "media"), "pixel_width": 480, "pixel_height": 270,
                         "renderer": "cairo", "dry_run": True}):
            scene = visuals.ClipScene()
            source = Square()
            target = VGroup(*(Circle(radius=0.6).shift([position, 0, 0]) for position in CASES["consuming_sequence"]["centres"]))
            animation = module._seq((FadeIn(source),), (ReplacementTransform(source, target),))
            animation._setup_scene(scene)
            animation.begin()
            for alpha in CASES["consuming_sequence"]["samples"]:
                animation.update_mobjects(0)
                animation.interpolate(alpha)
                scene.update_mobjects(0)
            animation.finish()
            animation.clean_up_from_scene(scene)
            self.assertNotIn(source, scene.get_mobject_family_members())
            self.assertIn(target, scene.get_mobject_family_members())
            scene.camera.reset()
            scene.camera.capture_mobjects(scene.mobjects)
            actual = scene.camera.pixel_array.copy()
            scene.camera.reset()
            scene.camera.capture_mobjects([target])
            np.testing.assert_array_equal(actual, scene.camera.pixel_array)


    def test_segment_quad_matches_independent_projection(self):
        """📐 Compare segment thickness geometry with portable coordinates and NumPy projections."""
        import numpy as np

        contract = CASES["segment_quads"]
        function = getattr(importlib.import_module(contract["module"]), contract["function"])
        for case in contract["cases"]:
            with self.subTest(case=case["id"]):
                a, b = np.asarray(case["a"], dtype=float), np.asarray(case["b"], dtype=float)
                delta = b - a
                normal = np.array([-delta[1], delta[0]]) / np.linalg.norm(delta) * case["half_width"]
                expected = np.array([a + normal, b + normal, b - normal, a - normal])
                np.testing.assert_allclose(expected, case["corners"], atol=1e-12)
                actual = function(case["a"], case["b"], case["half_width"])
                self.assertIsInstance(actual, list)
                self.assertTrue(all(isinstance(point, list) and len(point) == 2 for point in actual))
                actual = np.asarray(actual, dtype=float)
                self.assertEqual(actual.shape, (4, 2))
                np.testing.assert_allclose(np.array(sorted(actual.tolist())), np.array(sorted(expected.tolist())), atol=1e-12)
                area = abs(np.dot(actual[:, 0], np.roll(actual[:, 1], -1)) - np.dot(actual[:, 1], np.roll(actual[:, 0], -1))) / 2
                self.assertAlmostEqual(area, case["area"])
                np.testing.assert_allclose(actual.mean(axis=0), (a + b) / 2, atol=1e-12)
        for case in contract["invalid"]:
            with self.subTest(invalid=case), self.assertRaises(ValueError):
                function(case["a"], case["b"], case["half_width"])

    def test_objects_displayed_arithmetic_holds_and_matches_the_model(self):
        """🧮 Compare every product shown by the objects clip with independent math and the model's own volumes."""
        import math
        from types import SimpleNamespace

        module = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧬️objects.🐍️")
        model = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
        source = model.load_model()
        kind = next(t for t in source["types"] if t["class"] == "IfcColumnType")
        count = len(model.type_occurrences(kind["tag"], source))
        scene = module.Beat2_ObjectOrientation.__new__(module.Beat2_ObjectOrientation)
        facts = SimpleNamespace(model=source, chains={}, count=count)
        for diameter in (0.30, 0.32, 0.34, 0.36, 0.38, 0.40):
            with self.subTest(diameter=diameter):
                chain = scene._chain(facts, diameter)
                self.assertAlmostEqual(chain.area, round(math.pi * (diameter / 2) ** 2, 5), places=9)
                self.assertAlmostEqual(chain.volume, round(chain.area * chain.height, 4), places=9)
                self.assertAlmostEqual(chain.total, count * chain.volume, places=9)
                exact = count * math.pi * (diameter / 2) ** 2 * chain.height
                self.assertLess(abs(chain.total - exact) / exact, 1e-3)
        self.assertAlmostEqual(scene._chain(facts, 0.30).total, model.quantities(kind["tag"], source)["volume"], delta=0.002)


def load_tests(loader, tests, pattern):
    """🚪 Include the domain-local consumed-visual lifecycle feature in the existing native BIM suite."""
    tests.addTests(loader.loadTestsFromName("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧬️objects.🧪️tests.🚪️retirement.🐍️"))
    return tests


if __name__ == "__main__":
    unittest.main()

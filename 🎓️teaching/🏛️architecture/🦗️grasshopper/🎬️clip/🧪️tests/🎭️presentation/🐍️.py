"""🎭 Validate Grasshopper scene-owned presentation and narration contracts for every scene found under the series folder, plus the canvas toolkit."""
import ast
import importlib
import inspect
import json
import os
import sys
import tempfile
import unittest
from pathlib import Path
from unittest.mock import patch

ROOT = next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
CLIP = Path(__file__).resolve().parents[2]
CASES = json.loads(Path(__file__).with_name("🔣️.cases.json").read_text(encoding="utf-8"))
PACKAGE = "🎓️teaching.🏛️architecture.🦗️grasshopper.🎬️clip"
VISUALS = importlib.import_module(PACKAGE + ".🎨️visuals.🐍️")


def discover() -> list:
    """🔭 Find every ClipScene subclass in the topic folders of the series: ``(folder, class, file)``."""
    found = []
    for folder in sorted(path for path in CLIP.iterdir() if path.is_dir() and path.name not in CASES["excluded"] and (path / "🐍️.py").exists()):
        module = importlib.import_module(f"{PACKAGE}.{folder.name}.🐍️")
        for _, scene in inspect.getmembers(module, inspect.isclass):
            if scene.__module__ == module.__name__ and issubclass(scene, VISUALS.ClipScene) and "NARRATION" in vars(scene):
                found.append((folder.name, scene, folder / "🐍️.py"))
    return found


def calls(path: Path):
    """🧮 Parse a scene file: literal step keys, literal present topics, literal imports and the count of non-literal step calls."""
    tree = ast.parse(path.read_text(encoding="utf-8"))
    steps, topics, imports, dynamic = [], [], [], 0
    for node in ast.walk(tree):
        if not isinstance(node, ast.Call):
            continue
        func, first = node.func, node.args[0] if node.args else None
        literal = isinstance(first, ast.Constant) and isinstance(first.value, str)
        if isinstance(func, ast.Attribute) and isinstance(func.value, ast.Name) and func.value.id == "self":
            if func.attr == "step":
                steps.append(first.value) if literal else None
                dynamic += 0 if literal else 1
            if func.attr == "present" and literal:
                topics.append(first.value)
        if isinstance(func, ast.Attribute) and func.attr == "import_module" and literal:
            imports.append(first.value)
    return steps, topics, imports, dynamic


class GrasshopperPresentationContract(unittest.TestCase):
    """🦗 Require every localized Grasshopper scene to implement its declared presentation."""

    @classmethod
    def setUpClass(cls):
        """🔐 Discover the scenes once for all checks."""
        cls.scenes = discover()

    def test_narration_is_bilingual_and_unique(self):
        """🗣️ Every clause has a unique key and non-empty English and German text; the title exists in both languages; the clause count fits the video length."""
        for folder, scene, _ in self.scenes:
            with self.subTest(scene=scene.__name__):
                keys = [row[0] for row in scene.NARRATION]
                self.assertEqual(len(keys), len(set(keys)), "duplicate clause keys")
                self.assertTrue(all(isinstance(row, (list, tuple)) and len(row) == 3 and all(isinstance(part, str) and part.strip() for part in row) for row in scene.NARRATION))
                self.assertTrue(CASES["clauses"]["min"] <= len(keys) <= CASES["clauses"]["max"], len(keys))
                self.assertEqual(set(scene.TITLE), {"en", "de"})
                self.assertTrue(all(text.strip() for text in scene.TITLE.values()))

    def test_dispatch_and_step_keys(self):
        """📜 ``construct`` presents a topic with a real ``present_<topic>`` handler and every ``step`` key exists in NARRATION (and none stays unused when all keys are literal)."""
        for folder, scene, path in self.scenes:
            with self.subTest(scene=scene.__name__):
                steps, topics, _, dynamic = calls(path)
                keys = {row[0] for row in scene.NARRATION}
                self.assertEqual(sorted(set(steps) - keys), [], "step uses unknown keys")
                if not dynamic:
                    self.assertEqual(sorted(keys - set(steps)), [], "narration clauses never stepped")
                self.assertTrue(topics, "construct never presents a topic")
                for topic in topics:
                    self.assertTrue(callable(vars(scene).get("present_" + topic)), "present_" + topic)

    def test_declared_chapters_match(self):
        """📋 Scenes pinned in the cases file keep their exact clause sequence."""
        for chapter in CASES["chapters"]:
            with self.subTest(topic=chapter["topic"]):
                module = importlib.import_module(f"{PACKAGE}.{chapter['folder']}.🐍️")
                scene = getattr(module, chapter["scene"])
                self.assertEqual([row[0] for row in scene.NARRATION], chapter["keys"])
                self.assertTrue(callable(vars(scene).get("present_" + chapter["topic"])))

    def test_imports_resolve(self):
        """📦 Every literal ``import_module`` of every scene, the visuals and the canvas resolves."""
        files = [path for _, _, path in self.scenes] + sorted((CLIP / "🎨️visuals").rglob("🐍️.py"))
        for path in files:
            with self.subTest(file=str(path.relative_to(CLIP))):
                for name in calls(path)[2]:
                    importlib.import_module(name)

    def test_provenance_files_exist(self):
        """🧾 ``source_files`` lists real files for render provenance and includes the canvas and the engine inputs."""
        listed = VISUALS.source_files()
        self.assertTrue(all(path.exists() for path in listed), [str(path) for path in listed if not path.exists()])
        names = {path.parent.name for path in listed}
        self.assertTrue({"🦗️canvas", "🎨️visuals"} <= names)

    def test_language_is_explicit(self):
        """🌐 No default language: the scene base raises without the environment variable and formats numbers per language."""
        scene = VISUALS.ClipScene.__new__(VISUALS.ClipScene)
        with patch.dict(os.environ, {}, clear=False):
            os.environ.pop("SEMIO_TEACHING_LANGUAGE", None)
            with self.assertRaises(ValueError):
                scene.language()
        for language, expected in (("en", "1,250.50"), ("de", "1.250,50")):
            with patch.dict(os.environ, {"SEMIO_TEACHING_LANGUAGE": language}):
                self.assertEqual(scene.number(1250.5), expected)
                self.assertEqual(scene.tr("Slider", "Schieberegler"), "Slider" if language == "en" else "Schieberegler")
                self.assertEqual(scene.number(-2.5, 1), ("−2.5" if language == "en" else "−2,5"))

    def test_native_lifecycle(self):
        """🎞️ Run each discovered scene and the toolkit probe through real animation setup, interpolation, cleanup and Cairo in both languages."""
        output = Path(os.environ.get("SEMIO_TEACHING_OUTPUT_ROOT") or tempfile.mkdtemp(prefix="gh-presentation-"))
        output.mkdir(parents=True, exist_ok=True)
        targets = [(Probe.__name__, Probe)] + [(scene.__name__, scene) for _, scene, _ in self.scenes]
        for name, base in targets:
            for language in CASES["languages"]:
                with self.subTest(scene=name, language=language):
                    self.lifecycle(name, base, language, output)

    def lifecycle(self, name, base, language, output):
        """🔬 One scene in one language with patched timing; asserts finite geometry, visible pixels and the exact stepped keys."""
        import numpy as np
        from manim import tempconfig

        seen, played = [], set()

        def play(scene, *animations, **kwargs):
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
            scene.update_mobjects(0.25)
            scene.renderer.time += duration

        def step(scene, key, *animations, **kwargs):
            seen.append(key)
            result = super(probe, scene).step(key, *animations, **kwargs)
            for member in scene.get_mobject_family_members():
                self.assertTrue(np.isfinite(member.points).all(), key)
            return result

        env = {"SEMIO_TEACHING_LANGUAGE": language, "SEMIO_TEACHING_AUDIENCE": "spectator"}
        settings = {"media_dir": str(output / "media"), "text_dir": str(output / "text"), "pixel_width": 480, "pixel_height": 270, "frame_rate": 4,
                    "renderer": "cairo", "dry_run": True, "disable_caching": True}
        with patch.dict(os.environ, env), tempconfig(settings):
            print(f"[DEBUG] GH lifecycle {name} {language}", flush=True)
            probe = type("NativeProbe", (base,), {"play": play, "wait": wait, "step": step})
            scene = probe()
            scene.construct()
            self.assertEqual(seen, [row[0] for row in base.NARRATION])
            self.assertTrue(set(CASES["animations"]) <= played, played)
            family = scene.get_mobject_family_members()
            self.assertTrue(family)
            scene.camera.reset()
            scene.camera.capture_mobjects(scene.mobjects)
            pixels = scene.camera.pixel_array
            self.assertEqual(pixels.shape[:2], (270, 480))
            self.assertTrue(np.any(pixels != pixels[0, 0]))


class Probe(VISUALS.ClipScene):
    """🧪 Minimal series scene exercising the base class and the canvas widgets together."""

    TITLE = {"en": "Toolkit Probe", "de": "Werkzeug-Probe"}
    NARRATION = [["slider", "A slider feeds the component.", "Ein Schieberegler speist die Komponente."],
                 ["wire", "A wire carries the value.", "Eine Verbindung trägt den Wert."]]

    def construct(self):
        """▶️ Present the probe chapter."""
        self.present("probe")

    def present_probe(self):
        """🎞️ Slider with tracker, component and live wire."""
        from manim import FadeIn, ValueTracker
        tracker = ValueTracker(2.0)
        slider = self.gh_slider("Width", 2.0, 1.0, 4.0, at=(-3.5, 0, 0), tracker=tracker, width=3.2)
        box = self.gh_component({"name": "Center Box", "nickname": "CBox", "inputs": [{"nickname": "X"}], "outputs": [{"nickname": "B"}]}, at=(1, 0, 0))
        wire = self.gh_wire(slider.grip_out(), box.grip_in("X"))
        self.step("slider", FadeIn(slider), FadeIn(box))
        self.step("wire", FadeIn(wire), tracker.animate.set_value(3.5))


class CanvasWidgets(unittest.TestCase):
    """🧰 Every canvas widget builds in both languages with working grips, parts and wire kinds."""

    def test_widget_zoo(self):
        """🦗 Build one of everything and check anchors, parts, states and wire attachment."""
        import numpy as np

        spec = {"name": "Center Box", "nickname": "CBox", "inputs": [{"name": "Base", "nickname": "B"}, {"name": "X Size", "nickname": "X"}],
                "outputs": [{"name": "Box", "nickname": "B"}]}
        for language in CASES["languages"]:
            with self.subTest(language=language), patch.dict(os.environ, {"SEMIO_TEACHING_LANGUAGE": language}):
                scene = VISUALS.ClipScene.__new__(VISUALS.ClipScene)
                VISUALS._fonts.register_fonts()
                for state in CASES["widgets"]["states"]:
                    component = scene.gh_component(spec, at=(1, 2, 0), state=state, selected=state == "ok")
                    self.assertIsNotNone(component.body)
                    self.assertLess(component.grip_in("X").get_center()[0], component.body.get_left()[0])
                    self.assertGreater(component.grip_out("B").get_center()[0], component.body.get_right()[0])
                    np.testing.assert_allclose(component.body.get_center(), (1, 2, 0), atol=1e-6)
                slider = scene.gh_slider("Width", 2.0, 1.0, 4.0, at=(0, 0, 0))
                self.assertGreater(slider.grip_out().get_center()[0], slider.body.get_right()[0])
                for kind in CASES["widgets"]["wire_kinds"]:
                    wire = scene.gh_wire(slider.grip_out(), component.grip_in("B"), kind=kind)
                    self.assertGreater(len(wire.submobjects), 0)
                    np.testing.assert_allclose(wire.curve[0], slider.grip_out().get_center(), atol=1e-6)
                    np.testing.assert_allclose(wire.curve[-1], component.grip_in("B").get_center(), atol=1e-6)
                with self.assertRaises(ValueError):
                    scene.gh_wire((0, 0), (1, 1), kind="bundle")
                for level in CASES["widgets"]["levels"]:
                    self.assertIsNotNone(scene.gh_bubble("Message", level=level, target=component).body)
                self.assertIn("Params", scene.gh_ribbon(active="Params").parts["tabs"])
                tree = {(0, 0): [1, 2], (0, 1): [3]}
                panel = scene.gh_panel(tree, at=(0, 0, 0))
                self.assertEqual(len(panel.parts["rows"]), 5)
                self.assertEqual(len(scene.gh_param_viewer(tree).parts["rows"]), 2)
                self.assertIsNotNone(scene.gh_toggle(True).body)
                self.assertIsNotNone(scene.gh_value_list(["A", "B"], 1, is_open=True).body)
                self.assertIsNotNone(scene.gh_param("Number", label="Num").body)
                self.assertIsNotNone(scene.gh_viewport().body)
                self.assertIsNotNone(scene.gh_group([component, slider], "Group").body)

    def test_every_engine_view_builds(self):
        """🕸️ Every named view of the five engine canvases builds in English (and in German where a message bubble is shown): all objects and wires present, widgets inside the frame, readable scale, wire kinds from the data, bubbles for warnings and errors."""
        engine = importlib.import_module(PACKAGE + ".🧱️model.🐍️")
        frame = (13.4, 5.1)
        total = 0
        for language in CASES["languages"]:
            with patch.dict(os.environ, {"SEMIO_TEACHING_LANGUAGE": language, "SEMIO_TEACHING_CACHE_DIR": tempfile.mkdtemp(prefix="gh-fonts-")}):
                scene = VISUALS.ClipScene.__new__(VISUALS.ClipScene)
                VISUALS._fonts.register_fonts()
                for key in engine.canvas_keys():
                    definition = engine.canvas(key)
                    solution = engine.evaluate(definition)
                    for name in definition.views:
                        if language == "de" and not any(solution.msgs(oid) for oid in definition.views[name]["objects"]):
                            continue
                        with self.subTest(language=language, canvas=key, view=name):
                            total += 1
                            canvas = scene.gh_canvas(definition, solution, view=name, frame=frame)
                            self.assertEqual(set(canvas.objects), set(definition.views[name]["objects"]))
                            sheet = canvas.sheet
                            for oid, widget in canvas.objects.items():
                                self.assertGreaterEqual(widget.get_left()[0], sheet.get_left()[0] - 1e-6, oid)
                                self.assertLessEqual(widget.get_right()[0], sheet.get_right()[0] + 1e-6, oid)
                                self.assertGreaterEqual(widget.get_bottom()[1], sheet.get_bottom()[1] - 1e-6, oid)
                                self.assertLessEqual(widget.get_top()[1], sheet.get_top()[1] + 1e-6, oid)
                                self.assertEqual(canvas.bubbles[oid] is not None, bool(solution.msgs(oid)), oid)
                            self.assertGreaterEqual(canvas.factor, 0.75)
                            self.assertEqual(len(canvas.wires), len(definition.view_wires(name)))
                            for wire, drawn in zip(definition.view_wires(name), canvas.wires):
                                tree = solution.tree(wire.src, wire.out)
                                self.assertEqual(drawn.kind, "tree" if tree.path_count > 1 else "list" if tree.count > 1 else "item")
        self.assertGreaterEqual(total, 51)

    def test_states_and_panels_come_from_the_solution(self):
        """🚦 The failed Center Box is red with an error bubble, and a tree panel shows the engine's real path headers and item counts."""
        engine = importlib.import_module(PACKAGE + ".🧱️model.🐍️")
        with patch.dict(os.environ, {"SEMIO_TEACHING_LANGUAGE": "en"}):
            scene = VISUALS.ClipScene.__new__(VISUALS.ClipScene)
            VISUALS._fonts.register_fonts()
            definition = engine.canvas("casting")
            solution = engine.evaluate(definition)
            canvas = scene.gh_canvas(definition, solution, view="fail")
            self.assertEqual(solution.state("cbx"), "error")
            self.assertEqual(canvas.obj("cbx").state, "error")
            self.assertEqual(canvas.bubbles["cbx"].level, "error")
            definition = engine.canvas("trees")
            solution = engine.evaluate(definition)
            canvas = scene.gh_canvas(definition, solution, view="source")
            panels = [o for o in definition.view("source") if o.component == "Panel"]
            self.assertTrue(any(len(canvas.obj(o.id).parts["rows"]) > 1 for o in panels))

    def test_tree_notation(self):
        """🌳 Path and tree helpers produce Grasshopper notation."""
        canvas = importlib.import_module(PACKAGE + ".🎨️visuals.🦗️canvas.🐍️")
        self.assertEqual(canvas.path_text((0, 1, 2)), "{0;1;2}")
        self.assertEqual(canvas.branches({(0,): [1, 2], (1,): [3]}), [((0,), [1, 2]), ((1,), [3])])
        self.assertEqual(canvas.branches([[1, 2], [3]]), [((0,), [1, 2]), ((1,), [3])])

    def test_bezier_is_an_s_curve(self):
        """〰️ The wire starts and ends horizontally at the grips and is monotone between them."""
        import numpy as np

        canvas = importlib.import_module(PACKAGE + ".🎨️visuals.🦗️canvas.🐍️")
        curve = canvas.bezier((0, 0), (3, -1))
        np.testing.assert_allclose(curve[0], (0, 0, 0))
        np.testing.assert_allclose(curve[-1], (3, -1, 0))
        self.assertTrue(np.all(np.diff(curve[:, 0]) > 0))
        self.assertAlmostEqual(curve[1, 1] / curve[1, 0] if curve[1, 0] else 0.0, 0.0, delta=0.2)


if __name__ == "__main__":
    unittest.main()

"""🧪 Verify actual cooling morphs and canonical microbe points with independent Manim/Cairo oracles."""
from __future__ import annotations

import ast
import copy
import importlib
import json
import os
import sys
import unittest
from pathlib import Path

import numpy as np
from manim import Camera, Dot, ReplacementTransform, Square, SurroundingRectangle, VGroup, tempconfig
from manim.animation.animation import prepare_animation

ROOT = next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
CASES = json.loads(Path(__file__).with_name("🔣️.cases.json").read_text(encoding="utf-8"))
PACKAGE = "🎓️teaching.🏛️architecture.⚡️energy"


class CoolingAnimationTest(unittest.TestCase):
    """🧬 Freeze aliased section geometry without duplicating alpha or disabling live cube faces."""

    def setUp(self):
        """📦 Scope native rendering output to the declared test root."""
        output = Path(os.environ["SEMIO_TEACHING_OUTPUT_ROOT"])
        self.config = tempconfig({"media_dir": str(output / "media"), "text_dir": str(output / "text")})
        self.config.__enter__()
        self.addCleanup(self.config.__exit__, None, None, None)
        self.freeze = importlib.import_module(PACKAGE + ".🏗️authoring.🎬️clip.🎞️animation.🐍️").freeze_drawables

    def pixels(self, mobject):
        """🖼️ Ask native Cairo to paint the unique drawable family in its original order."""
        raster = CASES["raster"]
        camera = Camera(pixel_width=raster["width"], pixel_height=raster["height"], background_color=raster["background"])
        camera.capture_mobjects([mobject])
        return np.asarray(camera.get_image()).copy()

    def drive(self, animation):
        """🎬 Run the real begin/copy/update/interpolate/finish lifecycle at portable sample times."""
        animation = prepare_animation(animation)
        animation.begin()
        for alpha in CASES["interpolation"]:
            animation.update_mobjects(0.25)
            animation.interpolate(alpha)
        animation.finish()

    def construct(self, module, name):
        """🔬 Read one actual authored construct body without rendering unrelated narration."""
        tree = ast.parse(Path(module.__file__).read_text(encoding="utf-8"))
        scene = next(node for node in tree.body if isinstance(node, ast.ClassDef) and node.name == name)
        return next(node for node in scene.body if isinstance(node, ast.FunctionDef) and node.name == "construct")

    def office(self):
        """🏢 Rebuild the exact facade and room geometry at the failed office boundary."""
        module = importlib.import_module(PACKAGE + ".❄️cooling.🎬️clip.👥️internal-gains.🐍️")
        case = CASES["office"]
        facade = module.house_section(np.array(case["origin"]))["group"]
        facade.scale(case["scaleAfter"]).move_to(case["centerAfter"])
        room = module.room_section(np.array(case["roomCenter"]), w=case["roomWidth"], h=case["roomHeight"])
        return module, facade, room

    def solar(self):
        """☀️ Execute the actual authored cube definitions and face updater registrations."""
        module = importlib.import_module(PACKAGE + ".❄️cooling.🎬️clip.☀️solar-radiation.🐍️")
        body = self.construct(module, "Beat1_SolarIrradiance")
        wanted = {"cube_c", "cs", "ch", "a0", "b0", "c0", "d0", "a1", "b1", "c1", "d1", "faces", "solid_edges", "hidden_edges", "face_tags", "cube", "h_tr"}
        nodes = []
        for node in body.body:
            if isinstance(node, ast.FunctionDef) and node.name == "CV":
                nodes.append(copy.deepcopy(node))
            elif isinstance(node, ast.Assign) and any(isinstance(name, ast.Name) and name.id in wanted for target in node.targets for name in ast.walk(target)):
                nodes.append(copy.deepcopy(node))
            elif isinstance(node, ast.For) and isinstance(node.iter, ast.Call) and isinstance(node.iter.func, ast.Attribute) and node.iter.func.attr == "items" and isinstance(node.iter.func.value, ast.Name) and node.iter.func.value.id == "faces":
                nodes.append(copy.deepcopy(node))
        scope = dict(vars(module))
        exec(compile(ast.fix_missing_locations(ast.Module(body=nodes, type_ignores=[])), str(module.__file__), "exec"), scope)
        case = CASES["solar"]
        building = module.house_section(np.array(case["origin"]), scale=case["scale"])["group"]
        return module, building, scope

    def boundary(self, module, name, source, scope):
        """▶️ Execute the actual authored freeze statements and original concurrent morph play."""
        body = self.construct(module, name)
        index = next(index for index, node in enumerate(body.body) if isinstance(node, ast.Expr) and isinstance(node.value, ast.Call) and any(isinstance(call, ast.Call) and isinstance(call.func, ast.Name) and call.func.id == "ReplacementTransform" and call.args and isinstance(call.args[0], ast.Name) and call.args[0].id == source for call in ast.walk(node)))
        nodes = [copy.deepcopy(node) for node in body.body[:index] if isinstance(node, ast.Expr) and any(isinstance(call, ast.Call) and isinstance(call.func, ast.Name) and call.func.id == "freeze_drawables" for call in ast.walk(node))]
        nodes.append(copy.deepcopy(body.body[index]))
        drive = self.drive

        class Driver:
            """🎭 Interpolate the actual submitted native animations without a full lesson render."""

            def play(self, *animations, **kwargs):
                """⏯️ Drive each concurrent component with the same native animation lifecycle."""
                for animation in animations:
                    drive(animation)

        namespace = {**vars(module), **scope, "self": Driver()}
        exec(compile(ast.fix_missing_locations(ast.Module(body=nodes, type_ignores=[])), str(module.__file__), "exec"), namespace)

    def test_translucent_aliases_paint_once_and_root_points_keep_exact_pixels(self):
        """🔒 Preserve native alpha, layer order, root identity and independent point ownership."""
        pane = Square(fill_color="#FF344F", fill_opacity=0.35, stroke_width=0)
        layer = Square(fill_color="#58BFFF", fill_opacity=0.4, stroke_width=0).shift([0.5, 0, 0])
        for root in (VGroup(VGroup(pane), layer, VGroup(pane)), Square(fill_opacity=0.2).add(VGroup(pane), layer, VGroup(pane))):
            before, members, identity = self.pixels(root), list(root.family_members_with_points()), id(root)
            self.assertIs(self.freeze(root), root)
            self.assertEqual(id(root), identity)
            np.testing.assert_array_equal(self.pixels(root), before)
            self.assertEqual(len(root.get_family()), len(members) + 1)
            self.assertTrue(all(not part.submobjects and not part.updaters for part in root.submobjects))
            for original, frozen in zip(members, root.submobjects, strict=True):
                self.assertFalse(np.shares_memory(original.points, frozen.points))

    def test_office_original_morph_red_and_snapshot_oracle_and_authored_boundary(self):
        """📐 Reproduce source69 and prove both exact static endpoints and the authored repair."""
        _, source, room = self.office()
        with self.assertRaisesRegex(ValueError, CASES["topologyError"]):
            self.drive(ReplacementTransform(source, room["shell"]))
        module, source, room = self.office()
        for mobject in (source, room["shell"]):
            before = self.pixels(mobject)
            self.freeze(mobject)
            np.testing.assert_array_equal(self.pixels(mobject), before)
        self.drive(ReplacementTransform(source, room["shell"]))
        module, source, room = self.office()
        self.boundary(module, "Beat1_OfficeRoom", "facade", {"facade": source, "room": room,
                      "insulation_box": SurroundingRectangle(room["shell"]), "caption": None})

    def test_solar_original_morph_red_and_live_target_survives_authored_repair(self):
        """🌞 Reproduce source78 while retaining the cube's live irradiance face updaters."""
        _, source, scope = self.solar()
        with self.assertRaisesRegex(ValueError, CASES["topologyError"]):
            self.drive(ReplacementTransform(source, scope["cube"]))
        module, source, scope = self.solar()
        before = self.pixels(source)
        self.freeze(source)
        np.testing.assert_array_equal(self.pixels(source), before)
        self.drive(ReplacementTransform(source, scope["cube"]))
        self.assertTrue(all(face.updaters for face in scope["faces"].values()))
        scope["h_tr"].set_value(12)
        scope["cube"].update(0)
        self.assertGreater(scope["faces"]["S"].get_fill_opacity(), 0)
        module, source, scope = self.solar()
        self.boundary(module, "Beat1_SolarIrradiance", "building", {**scope, "building": source,
                      "sun_path": Dot(), "sun": Dot(), "sun_screen": lambda hour: np.zeros(3)})

    def test_microbe_helper_and_every_authored_point_are_canonical_three_dimensions(self):
        """🦠 Compare real silhouette placement with native NumPy three-component positions."""
        module = importlib.import_module(PACKAGE + ".❄️cooling.🎬️clip.🌤️summer-strategy.🐍️")
        body = self.construct(module, CASES["microbes"]["scene"])
        helper = next(node for node in body.body if isinstance(node, ast.FunctionDef) and node.name == "_microbe")
        scope = dict(vars(module))
        exec(compile(ast.fix_missing_locations(ast.Module(body=[copy.deepcopy(helper)], type_ignores=[])), str(module.__file__), "exec"), scope)
        with self.assertRaisesRegex(ValueError, "broadcast"):
            scope["_microbe"](CASES["microbes"]["points"][0][:2], module.P_TEAL)
        for point in CASES["microbes"]["points"]:
            np.testing.assert_allclose(scope["_microbe"](point, module.P_TEAL).get_center(), point, atol=1e-8)
        calls = sorted((node for node in ast.walk(body) if isinstance(node, ast.Call) and isinstance(node.func, ast.Name) and node.func.id == "_microbe"), key=lambda node: (node.lineno, node.col_offset))
        self.assertEqual(len(calls), 3)
        observed = []
        for call in calls:
            self.assertIsInstance(call.args[0], ast.List)
            self.assertEqual(len(call.args[0].elts), CASES["microbes"]["dimensions"])
            repeated = any(isinstance(node, ast.Name) and node.id == "i" for node in ast.walk(call.args[0]))
            for i in range(4) if repeated else (0,):
                point = eval(compile(ast.fix_missing_locations(ast.Expression(copy.deepcopy(call.args[0]))), str(module.__file__), "eval"), {**scope, "i": i, "strip_y": CASES["microbes"]["stripY"]})
                self.assertEqual(point[2], CASES["microbes"]["z"])
                observed.append(point)
        np.testing.assert_allclose(observed, CASES["microbes"]["points"], atol=1e-8)


if __name__ == "__main__":
    unittest.main()

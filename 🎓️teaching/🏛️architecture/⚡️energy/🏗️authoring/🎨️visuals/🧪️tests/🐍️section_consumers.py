"""🧪 Exercise local construction against portable opaque material cases and Manim."""

import ast
import importlib
import json
import os
import tempfile
import unittest
from pathlib import Path

import numpy as np
from manim import Circle, Line, Rectangle, tempconfig

BASE = "🎓️teaching.🏛️architecture.⚡️energy."
CASES = json.loads(Path(__file__).with_name("🔣️section_consumers.json").read_text(encoding="utf-8"))


class SectionConsumersTest(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        workspace = next(parent for parent in Path(__file__).parents if (parent / "🧰️framework").is_dir())
        output = Path(os.environ.get("SEMIO_TEACHING_OUTPUT_ROOT", workspace / ".🧬semio/🎓️teaching/⚡️cache/tests"))
        output.mkdir(parents=True, exist_ok=True)
        cls.directory = tempfile.TemporaryDirectory(dir=output)
        cls.configuration = tempconfig({"text_dir": str(Path(cls.directory.name) / "text")})
        cls.configuration.__enter__()
        cls.addClassCleanup(cls.directory.cleanup)
        cls.addClassCleanup(cls.configuration.__exit__, None, None, None)
        cls.shared = importlib.import_module(BASE + "🏗️authoring.🎨️visuals.🐍️")

    def assert_material(self, material):
        oracle = Rectangle(fill_color=self.shared.P_WHITE, **CASES["material"])
        leaves = [part for part in material.family_members_with_points() if not part.submobjects]
        self.assertTrue(leaves)
        for part in leaves:
            self.assertEqual(part.get_fill_color(), oracle.get_fill_color())
            self.assertEqual(part.get_fill_opacity(), oracle.get_fill_opacity())
            self.assertEqual(part.get_stroke_width(), oracle.get_stroke_width())
            self.assertEqual(part.get_stroke_opacity(), oracle.get_stroke_opacity())

    def test_heating_plant_construction(self):
        module = importlib.import_module(BASE + "🔥️heating.🎬️clip.🧮️calculation.🐍️")
        house = module._plant_house()
        for key in ("walls", "roof", "chimney", "material"):
            with self.subTest(key=key):
                self.assert_material(house[key])
        wall = module._window_wall(-1, 0)
        self.assert_material(wall["group"][0])
        self.assert_material(wall["group"][1])
        self.assert_material(module._house(np.zeros(3))["floor"])

    def test_heating_stage_material_has_no_masonry_seams(self):
        module = importlib.import_module(BASE + "🔥️heating.🎬️clip.🧲️fundamentals.🐍️")
        wall = module._brick_wall()
        self.assert_material(wall["core"])
        self.assertEqual(set(wall), {"core", "tint", "group"})
        stage = module._stage()
        for part in list(stage["shell"])[2:5]:
            self.assert_material(part)

    def test_conduction_magnifier_and_lattice_setup(self):
        module = importlib.import_module(BASE + "🔥️heating.🎬️clip.🧲️fundamentals.🐍️")
        case = CASES["magnifier"]
        stage = module._stage(scale=case["scale"])
        lens = module._lens(stage)
        lattice = module._lattice(lens)
        disc = Circle(radius=case["radius"] * case["scale"]).move_to(lens["c"])
        bond = Line(color=module.semio_color(case["bond_color"]), stroke_width=case["bond_width"],
                    stroke_opacity=case["bond_opacity"])
        self.assertAlmostEqual(lens["disc"].width, disc.width)
        self.assertAlmostEqual(lens["disc"].height, disc.height)
        np.testing.assert_allclose(lens["disc"].get_center(), disc.get_center())
        self.assertEqual(lens["bands"][1].get_fill_color().to_hex(), module.semio_color(case["solid_color"]).upper())
        self.assertEqual(len(lattice["air"]), 2 * case["air_particles_per_side"])
        self.assertEqual(len(lattice["dots"]), len(lattice["homes"]))
        self.assertEqual(len(lattice["bonds"]), len(lattice["pairs"]))
        self.assertGreater(len(lattice["bonds"]), 0)
        for actual in lattice["bonds"]:
            self.assertEqual(actual.get_stroke_color(), bond.get_stroke_color())
            self.assertEqual(actual.get_stroke_width(), bond.get_stroke_width())
            self.assertEqual(actual.get_stroke_opacity(), bond.get_stroke_opacity())
        self.assert_material(stage["material"])

    def test_ventilation_rooms_and_house_material(self):
        for name in CASES["ventilation"]:
            module = importlib.import_module(BASE + "🌬️ventilation.🎬️clip." + name + ".🐍️")
            with self.subTest(module=name):
                self.assert_material(module._room(np.zeros(3), 4, 3, color=self.shared.P_ORANGE))
        module = importlib.import_module(BASE + "🌬️ventilation.🎬️clip.📦️volume.🐍️")
        for scale in CASES["scales"]:
            with self.subTest(scale=scale):
                self.assert_material(module._pf_house(module._house_section(), np.zeros(3), scale)["material"])
        module = importlib.import_module(BASE + "🌬️ventilation.🎬️clip.🪟️natural.🐍️")
        room = module._vent_room(-4, -1, 0.8, "Section", self.shared.P_ORANGE)
        self.assert_material(room["material"])

    def test_practice_room_row(self):
        module = importlib.import_module(BASE + "📊️demand.🎬️clip.🧪️practice.🐍️")
        self.assert_material(module._room_row(-4, 2, 4, 0, 3)["shell"])

    def test_opening_replacements_remain_opaque(self):
        module = importlib.import_module(BASE + "📊️demand.🎬️clip.⚖️regulations.🐍️")
        house = self.shared.house_section(np.zeros(3))
        module._north_window(house)
        self.assert_material(house["wall_right"])
        module = importlib.import_module(BASE + "🔥️heating.🎬️clip.🧱️conduction.🐍️")
        colors = {key: self.shared.P_ORANGE for key in ("Außenwand", "Dach", "Fenster", "Haustür", "Boden")}
        house = module._envelope_house(np.zeros(3), 1, colors)
        for key in ("walls", "roof", "floor", "wall_right"):
            self.assert_material(house[key])

    def test_cooling_envelope_heat_is_separate(self):
        module = importlib.import_module(BASE + "❄️cooling.🎬️clip.💧️transmission-humidity.🐍️")
        house = module._build_house()
        house["envelope"].set_fill(module.P_RED, opacity=0.35)
        self.assert_material(house["material"])

    def test_authored_construction_setup(self):
        for case in CASES["ast_cases"]:
            module = importlib.import_module(BASE + case["module"] + ".🐍️")
            tree = ast.parse(Path(module.__file__).read_text(encoding="utf-8"))
            scope_node = tree
            for name in filter(None, case["function"].split(".")):
                scope_node = next(node for node in scope_node.body if isinstance(node, (ast.FunctionDef, ast.ClassDef)) and node.name == name)
            assignments = [node for node in ast.walk(scope_node) if isinstance(node, ast.Assign)]
            scope = dict(vars(module))
            for name in case["assignments"]:
                selected = [node for node in assignments if name in {item.id for target in node.targets for item in ast.walk(target) if isinstance(item, ast.Name)}]
                self.assertEqual(len(selected), 1, (case["module"], name))
                node = selected[0]
                referenced = {item.id for item in ast.walk(node.value) if isinstance(item, ast.Name)}
                dependencies = [candidate for candidate in assignments if candidate.lineno < node.lineno and any(isinstance(target, ast.Name) and target.id in referenced and target.id not in scope for target in candidate.targets)]
                exec(compile(ast.Module(body=dependencies + [node], type_ignores=[]), module.__file__, "exec"), scope)
            with self.subTest(module=case["module"]):
                self.assert_material(scope[case["result"]])

    def test_animation_styles_target_analysis_overlays(self):
        energy = Path(__file__).parents[3]
        violations = []
        for path in energy.rglob("*.py"):
            if "🏗️authoring" in path.parts:
                continue
            tree = ast.parse(path.read_text(encoding="utf-8"))
            for node in ast.walk(tree):
                if not isinstance(node, ast.Call) or not isinstance(node.func, ast.Attribute) or node.func.attr not in {"set_fill", "set_color", "set_opacity"}:
                    continue
                receiver = node.func.value
                if any(isinstance(item, ast.Call) and isinstance(item.func, ast.Attribute) and item.func.attr == "copy" for item in ast.walk(receiver)):
                    continue
                keys = [item.slice.value for item in ast.walk(receiver) if isinstance(item, ast.Subscript)
                        and isinstance(item.slice, ast.Constant) and isinstance(item.value, ast.Name)
                        and item.value.id in CASES["construction_variables"]]
                if any(key in CASES["construction_keys"] for key in keys):
                    violations.append(f"{path}:{node.lineno}")
        self.assertEqual(violations, [])


if __name__ == "__main__":
    unittest.main()

"""🧪 Heating conduction comparison caption fits the display."""
import ast
import importlib
import json
import unittest
from pathlib import Path

CASES = json.loads(Path(__file__).with_name("🔣️.cases.json").read_text(encoding="utf-8"))
FONTS = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🔤️fonts.🐍️")
VISUALS = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")


class ConductionCaption(unittest.TestCase):
    """📏 The English comparison fits the course caption box."""

    def test_comparison(self):
        tree = ast.parse((Path(__file__).parents[1] / "🐍️.py").read_text(encoding="utf-8"))
        scene = next(node for node in tree.body if isinstance(node, ast.ClassDef) and node.name == "Beat4_WattVergleich")
        assignment = next(node for node in scene.body if isinstance(node, ast.Assign) and any(isinstance(target, ast.Name) and target.id == "NARRATION" for target in node.targets))
        caption = next(row[1] for row in ast.literal_eval(assignment.value) if row[0] == "compare")
        self.assertEqual(caption, CASES["caption"])
        self.assertGreater(VISUALS.caption_bar(caption).width, 0)

    def test_bridge_names_fit_before_dimension_values(self):
        tree = ast.parse((Path(__file__).parents[1] / "🐍️.py").read_text(encoding="utf-8"))
        scene = next(node for node in tree.body if isinstance(node, ast.ClassDef) and node.name == "Beat6_Waermebruecken")
        assignment = next(node for node in ast.walk(scene) if isinstance(node, ast.Assign) and any(isinstance(target, ast.Name) and target.id == "names" for target in node.targets))
        for language, labels in CASES["bridgeNames"].items():
            selected = eval(compile(ast.Expression(assignment.value), "<bridge names>", "eval"), {"tr": lambda en, de: en if language == "en" else de})
            self.assertEqual(list(selected.values()), labels)
            for label in labels:
                with self.subTest(language=language, label=label):
                    text = FONTS.body_text(label, font_size=FONTS.LABEL_FONT_SIZE, color="#ffffff")
                    self.assertLess(text.width, CASES["maxNameWidth"])


    def test_flat_formula_handoff_drops_before_appear(self):
        tree = ast.parse((Path(__file__).parents[1] / "🐍️.py").read_text(encoding="utf-8"))
        scene = next(node for node in tree.body if isinstance(node, ast.ClassDef) and node.name == "Beat6_Waermebruecken")
        source = ast.dump(scene)
        case = CASES["flatFormulaHandoff"]
        for forbidden in case["forbidden"]:
            self.assertNotIn(forbidden, source)
        for required in case["required"]:
            self.assertIn(required, source)


    def test_bridge_method_notes_clear_formula_band(self):
        import os
        os.environ.setdefault("SEMIO_TEACHING_LANGUAGE", "de")
        case = CASES["bridgeMethodLayout"]
        tree = ast.parse((Path(__file__).parents[1] / "🐍️.py").read_text(encoding="utf-8"))
        scene = next(node for node in tree.body if isinstance(node, ast.ClassDef) and node.name == "Beat6_Waermebruecken")
        source = ast.get_source_segment((Path(__file__).parents[1] / "🐍️.py").read_text(encoding="utf-8"), scene) or ""
        self.assertIn(f"base_y, k_h = {case['baseY']}, {case['kH']}", source)
        self.assertIn(f"base_y - {case['labelOffset']}", source)
        self.assertIn(f"base_y - {case['noteOffset']}", source)
        note_bottom = case["baseY"] - case["noteOffset"] - 0.12
        formula_top = VISUALS.math_panel([
            ("h", r"H", "#ffffff"), (None, "=", "#ffffff"), ("ua", r"U", "#ffffff"),
            (None, "+", "#ffffff"), ("du", r"D", "#ffffff"),
        ])[1].get_top()[1]
        self.assertGreaterEqual(note_bottom - formula_top, case["minNoteToFormulaGap"])



if __name__ == "__main__":
    unittest.main()

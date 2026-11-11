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



if __name__ == "__main__":
    unittest.main()

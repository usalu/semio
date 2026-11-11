"""🧪 Cooling system sizing captions render with the course font."""
import ast
import importlib
import json
import os
import unittest
from unittest import mock
from pathlib import Path

CASES = json.loads(Path(__file__).with_name("🔣️.cases.json").read_text(encoding="utf-8"))
VISUALS = importlib.import_module("🎓️teaching.🏛️architecture.⚡️energy.🏗️authoring.🎨️visuals.🐍️")


class SizingCaptions(unittest.TestCase):
    """🔤 The authored English clause fits and renders through Manim and Pango."""

    def test_substitution_caption(self):
        tree = ast.parse((Path(__file__).parents[1] / "🐍️.py").read_text(encoding="utf-8"))
        scene = next(node for node in tree.body if isinstance(node, ast.ClassDef) and node.name == "Beat3_IsolateAirflow")
        assignment = next(node for node in scene.body if isinstance(node, ast.Assign) and any(isinstance(target, ast.Name) and target.id == "NARRATION" for target in node.targets))
        clauses = ast.literal_eval(assignment.value)
        caption = next(row[1] for row in clauses if row[0] == "substitute")
        self.assertEqual(caption, CASES["caption"])
        with unittest.mock.patch.dict(os.environ, {"SEMIO_TEACHING_LANGUAGE": "en"}):
            self.assertGreater(VISUALS.caption_bar(caption).width, 0)

    def test_english_clauses_render(self):
        tree = ast.parse((Path(__file__).parents[1] / "🐍️.py").read_text(encoding="utf-8"))
        for scene in (node for node in tree.body if isinstance(node, ast.ClassDef)):
            for assignment in (node for node in scene.body if isinstance(node, ast.Assign) and any(isinstance(target, ast.Name) and target.id == "NARRATION" for target in node.targets)):
                for key, english, _ in ast.literal_eval(assignment.value):
                    with self.subTest(scene=scene.name, clause=key):
                        self.assertGreater(VISUALS.caption_bar(english).width, 0)


if __name__ == "__main__":
    unittest.main()

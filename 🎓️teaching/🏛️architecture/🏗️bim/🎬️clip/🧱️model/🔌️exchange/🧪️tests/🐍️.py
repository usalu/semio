"""🔌 Validate the current B3 semantic exchange and mesh through independent NumPy geometry."""
import importlib
import json
from pathlib import Path
import re
import sys
import unittest

import numpy as np

ROOT = next(path for path in Path(__file__).resolve().parents if (path / "🧰️framework").is_dir())
sys.path.insert(0, str(ROOT))
CASES = json.loads(Path(__file__).with_name("🔣️.cases.json").read_text(encoding="utf-8"))


class B3ExchangeContract(unittest.TestCase):
    """🏛️ Preserve effective properties, stable identity and finite closed geometry."""

    @classmethod
    def setUpClass(cls):
        """📦 Load the actual current canonical core and its declared domain exports."""
        cls.api = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
        cls.model = cls.api.load_model()

    def test_step_identity_spans_and_effective_properties(self):
        """📄 Check every local reference and typed span against the current B3 contract."""
        element = self.api.element(CASES["tag"], self.model)
        self.assertEqual(element["guid"], CASES["guid"])
        self.assertEqual(element["material"]["tag"], CASES["material"])
        self.assertIs(element["psets"]["Pset_ColumnCommon"]["LoadBearing"]["value"], True)
        for language in CASES["languages"]:
            first = self.api.step_excerpt(CASES["tag"], self.model, first_id=CASES["first_id"], language=language)
            second = self.api.step_excerpt(CASES["tag"], self.model, first_id=first["first_id"] + CASES["shift"], language=language)
            for document in (first, second):
                self.assertEqual(document["first_id"], min(int(value) for value in re.findall(r"^#(\d+)\s*=", "\n".join(document["lines"]), re.M)))
                definitions = re.findall(r"^#(\d+)\s*=", "\n".join(document["lines"]), re.M)
                self.assertEqual(len(definitions), len(set(definitions)))
                references = re.findall(r"#(\d+)", "\n".join(document["lines"]))
                self.assertTrue(set(references) <= set(definitions))
                roles = {span["role"] for span in document["spans"]}
                self.assertTrue(set(CASES["roles"]) <= roles, roles)
                for span in document["spans"]:
                    text = document["lines"][span["line"]][span["start"]:span["end"]]
                    self.assertTrue(text)
                self.assertIn(CASES["guid"], "\n".join(document["lines"]))
                self.assertIn("IFC4X3_ADD2", "\n".join(document["lines"]))
            self.assertNotEqual(first["lines"], second["lines"])
            self.assertEqual(re.sub(r"#\d+", "#", "\n".join(first["lines"])),
                             re.sub(r"#\d+", "#", "\n".join(second["lines"])))

    def test_mesh_matches_independent_prism_and_ids_source(self):
        """🔺 Integrate triangle geometry independently and keep semantic applicability explicit."""
        for count in CASES["segments"]:
            mesh = self.api.mesh(CASES["tag"], count, self.model)
            vertices, faces = np.asarray(mesh["vertices"], dtype=float), np.asarray(mesh["faces"], dtype=int)
            self.assertEqual(mesh["vertex_count"], 2 * count + 2)
            self.assertEqual(mesh["triangle_count"], 4 * count)
            self.assertEqual(vertices.shape, (2 * count + 2, 3))
            self.assertTrue(np.isfinite(vertices).all())
            angles = 2 * np.pi * np.arange(count) / count
            expected = np.array(CASES["axis"]) + CASES["radius"] * np.column_stack((np.cos(angles), np.sin(angles)))
            np.testing.assert_allclose(vertices[:count, :2], expected, atol=1e-8, rtol=0)
            np.testing.assert_allclose(vertices[:count, 2], CASES["base"], atol=1e-9)
            np.testing.assert_allclose(vertices[count:2 * count, 2], CASES["top"], atol=1e-9)
            triangles = vertices[faces]
            signed = float(np.einsum("ij,ij->i", triangles[:, 0], np.cross(triangles[:, 1], triangles[:, 2])).sum() / 6)
            expected_volume = count / 2 * CASES["radius"] ** 2 * np.sin(2 * np.pi / count) * (CASES["top"] - CASES["base"])
            self.assertAlmostEqual(signed, expected_volume, places=7)
            edges = np.sort(np.concatenate((faces[:, [0, 1]], faces[:, [1, 2]], faces[:, [2, 0]])), axis=1)
            self.assertTrue(np.all(np.unique(edges, axis=0, return_counts=True)[1] == 2))
        semantic = self.api.ids_check(model=self.model, source="ifc")
        mesh = self.api.ids_check(model=self.model, source="mesh")
        self.assertIn(CASES["tag"], semantic["applicable"])
        self.assertIn(CASES["tag"], semantic["passed_tags"])
        self.assertFalse(semantic["failed"])
        self.assertTrue(semantic["requirement"])
        self.assertEqual(mesh["applicable"], [])


    def test_material_label_uses_effective_localized_name(self):
        """🏷️ Evaluate the actual authored label argument against the canonical material-name field."""
        import ast
        import os
        from types import SimpleNamespace
        from unittest.mock import patch

        module = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🔌️exchange.🐍️")
        scene = module.Beat5_Interfaces
        tree = ast.parse(Path(module.__file__).read_text(encoding="utf-8"))
        beat = next(node for node in ast.walk(tree) if isinstance(node, ast.FunctionDef) and node.name == "_beat_properties")
        call = next(node for node in ast.walk(beat) if isinstance(node, ast.Call) and isinstance(node.func, ast.Attribute)
                    and node.func.attr == "_name" and "material" in ast.dump(node.args[0]))
        element = self.api.element(CASES["tag"], self.model)
        expected = element
        for field in CASES["material_label_path"]:
            expected = expected[field]
        argument = eval(compile(ast.Expression(call.args[0]), str(module.__file__), "eval"),
                        {"f": SimpleNamespace(element=element)})
        for language in CASES["languages"]:
            with self.subTest(language=language), patch.dict(os.environ, {"SEMIO_TEACHING_LANGUAGE": language}):
                actual = scene._name(scene.__new__(scene), argument)
                self.assertEqual(actual, expected[language])


if __name__ == "__main__":
    unittest.main()

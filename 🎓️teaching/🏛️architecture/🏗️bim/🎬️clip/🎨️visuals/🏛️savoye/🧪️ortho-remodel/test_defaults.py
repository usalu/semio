"""🧪 Archival role defaults use actual product IDs rather than reference-grid labels."""
import ast
import json
from pathlib import Path
import unittest

import numpy as np


class DetailDefaultContract(unittest.TestCase):
    """🏛️ Omitted selectors resolve source-authored teaching roles."""

    def test_default_supports_exist(self):
        """📍 Verify actual column role and floor band without forcing posts onto a grid."""
        root = Path(__file__).resolve().parent
        defaults = json.loads((root / "cases.json").read_text(encoding="utf-8"))["defaults"]
        source = ast.parse((root.parent / "🔩️detail" / "🐍️.py").read_text(encoding="utf-8"))
        detail = next(node for node in source.body if isinstance(node,ast.ClassDef) and node.name == "Detail")
        methods = {node.name:node for node in detail.body if isinstance(node,ast.FunctionDef)}
        for name,key in (("piloti_detail","tag"),("slab_strip","row"),("slab_strip","bays"),("slab_strip","tag")):
            args = methods[name].args
            self.assertIsNone(ast.literal_eval(args.kw_defaults[[arg.arg for arg in args.kwonlyargs].index(key)]))
        model = json.loads((root.parents[2] / "🧱️model" / "🔣️.json").read_text(encoding="utf-8"))
        self.assertEqual(model["roles"][defaults["column_role"]],defaults["column_tag"])
        self.assertEqual(model["roles"][defaults["floor_role"]],defaults["floor_tag"])
        column = next(item for item in model["elements"] if item["tag"] == defaults["column_tag"])
        floor = next(item for item in model["elements"] if item["tag"] == defaults["floor_tag"])
        self.assertEqual(column["class"],"IfcColumn")
        self.assertEqual(floor["class"],"IfcSlab")
        axis = np.asarray(column["geometry"]["axis"],float)
        self.assertEqual(axis.shape,(2,))
        self.assertTrue(np.all(np.isfinite(axis)))


class LevelLabelContract(unittest.TestCase):
    """🔢 Authored millimetre datums survive the shared bilingual sheet labels."""

    def test_source_precision_and_locale(self):
        """🔬 Compare production formatting with NumPy's independent decimal conversion."""
        from types import SimpleNamespace
        root=Path(__file__).resolve().parent
        cases=json.loads((root/"cases.json").read_text(encoding="utf-8"))["level_labels"]
        tree=ast.parse((root.parent/"📏️sheet/🐍️.py").read_text(encoding="utf-8"))
        node=next(n for n in tree.body if isinstance(n,ast.FunctionDef) and n.name=="level_text")
        scope={}
        exec(compile(ast.Module(body=[node],type_ignores=[]),str(root),"exec"),scope)
        for case in cases:
            for language in ("en","de"):
                scene=SimpleNamespace(number=lambda value,precision:format(value,f".{precision}f").replace(".","," if language=="de" else "."))
                actual=scope["level_text"](scene,case["value"])
                self.assertEqual(actual,case[language])
                precision=3 if abs(case["value"]-round(case["value"],2))>1e-9 else 2
                sign="±" if abs(case["value"])<1e-9 else "+" if case["value"]>0 else ""
                expected=sign+np.format_float_positional(case["value"],precision=precision,unique=False,trim="k").replace(".","," if language=="de" else ".")
                self.assertEqual(actual,expected)


if __name__ == "__main__":
    unittest.main()

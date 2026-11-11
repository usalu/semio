"""📐 Verify real lengths are mapped, not equated, to scaled paper lengths."""
import ast,json,unittest
from pathlib import Path
from jsonschema import Draft202012Validator
from shapely.geometry import LineString
HERE=Path(__file__).parent
ROOT=next(p for p in HERE.resolve().parents if (p/"🧰️framework").is_dir())
class PaperScale(unittest.TestCase):
    def test_source_conversion_matches_paper_geometry(self):
        """🧮 The authored label distinguishes physical and paper lengths and preserves tenths."""
        data=json.loads((HERE/"🔣️.json").read_text(encoding="utf8"))
        Draft202012Validator(json.loads((HERE/"🔣️.schema.json").read_text(encoding="utf8"))).validate(data)
        source=ROOT/"🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/👁️representations/🐍️.py"
        tree=ast.parse(source.read_text(encoding="utf8"))
        owner=next(n for n in tree.body if isinstance(n,ast.ClassDef) and n.name=="Beat3_SameObject")
        method=next(n for n in owner.body if isinstance(n,ast.FunctionDef) and n.name=="_beat_scale20")
        label=next(n for n in ast.walk(method) if isinstance(n,ast.JoinedStr) and any(isinstance(v,ast.Constant) and " mm" in str(v.value) for v in n.values))
        fragments=[v.value for v in label.values if isinstance(v,ast.Constant)]
        self.assertTrue(any("→" in x for x in fragments))
        self.assertFalse(any("=" in x for x in fragments))
        number=next(n for n in ast.walk(label) if isinstance(n,ast.Call) and isinstance(n.func,ast.Attribute) and n.func.attr=="number")
        self.assertEqual(ast.literal_eval(number.args[1]),1)
        for case in data["cases"]:
            expected=LineString([(0,0),(case["metres"]*1000/case["scale"],0)]).length
            self.assertAlmostEqual(expected,case["paper_mm"])
            actual=eval(compile(ast.Expression(number.args[0]),str(source),"eval"),{"__builtins__":{}},{"t":case["metres"]})
            self.assertAlmostEqual(actual,expected)
if __name__=="__main__":unittest.main()

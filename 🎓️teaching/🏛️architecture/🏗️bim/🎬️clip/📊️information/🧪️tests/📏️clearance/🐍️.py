"""🧪 Compare the actual BIM clearance formula with geometric support intervals."""
import ast,json,unittest
from pathlib import Path
from jsonschema import Draft202012Validator
from shapely.geometry import LineString
ROOT=next(p for p in Path(__file__).resolve().parents if (p/"🧰️framework").is_dir())
HERE=Path(__file__).parent
CASES=json.loads((HERE/"🔣️.json").read_text(encoding="utf8"))
class Formula(unittest.TestCase):
    def test_actual_symbolic_formula_matches_clear_interval(self):
        """📏 Both support half-widths are subtracted from the axis spacing."""
        Draft202012Validator(json.loads((HERE/"🔣️.schema.json").read_text(encoding="utf8"))).validate(CASES)
        source=ROOT/"🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/📊️information/🐍️.py"
        tree=ast.parse(source.read_text(encoding="utf8"))
        owner=next(n for n in tree.body if isinstance(n,ast.ClassDef) and n.name=="Beat1_DataToInformation")
        method=next(n for n in owner.body if isinstance(n,ast.FunctionDef) and n.name=="_bay")
        assignment=next(n for n in ast.walk(method) if isinstance(n,ast.Assign) and any(isinstance(t,ast.Name) and t.id=="clear_row" for t in n.targets))
        mathcall=next(n for n in ast.walk(assignment.value) if isinstance(n,ast.Call) and isinstance(n.func,ast.Attribute) and n.func.attr=="_math")
        terms=[ast.literal_eval(item.elts[1]) for item in mathcall.args[0].elts]
        expression="".join(value.replace("−","-") for value in terms).split("=")[0]
        parsed=ast.parse(expression,mode="eval")
        self.assertTrue(all(isinstance(n,(ast.Expression,ast.BinOp,ast.Add,ast.Sub,ast.Name,ast.Load)) for n in ast.walk(parsed)))
        for case in CASES["cases"]:
            with self.subTest(case=case):
                actual=eval(compile(parsed,"<clear-width>","eval"),{"__builtins__":{}},{"s":case["spacing"],"r_1":case["left"],"r_2":case["right"]})
                expected=LineString([(case["left"],0),(case["spacing"]-case["right"],0)]).length
                self.assertAlmostEqual(expected,case["clear"])
                self.assertAlmostEqual(actual,expected)
    def test_native_clearance_and_grid_units_against_geometry(self):
        """🧮 The displayed span, margins and grid chain agree with independent lengths."""
        model_path=ROOT/"🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model/🐍️.py"
        source=ast.parse(model_path.read_text(encoding="utf8"))
        function=next(n for n in source.body if isinstance(n,ast.FunctionDef) and n.name=="clearance_limits")
        data=json.loads(model_path.with_name("🔣️.json").read_text(encoding="utf8"))
        self.assertEqual(data["grid"]["spacing"]["unit"],"m")
        for key in ("length","width","height","turning_radius","wall_clearance","min_side_margin"):
            self.assertEqual(data["car"][key]["unit"],"m")
        grid=data["grid"]["x"];length=data["building"]["box"]["length"]["value"]
        chain=sum(LineString([(a,0),(b,0)]).length for a,b in zip(grid,grid[1:]))
        self.assertAlmostEqual(chain,(len(grid)-1)*data["grid"]["spacing"]["value"])
        self.assertAlmostEqual(length-chain,2*grid[0])
        for case in CASES["cases"]:
            with self.subTest(case=case):
                env={"_m":lambda value:{}, "_car":lambda model:{"width":case["width"],"min_side_margin":case["margin"]},
                     "car_path":lambda model:{}, "_entry_dimensions":lambda model,path:{"obstruction_width":case["left"]+case["right"],"spacing":case["spacing"]}}
                exec(compile(ast.Module(body=[function],type_ignores=[]),str(model_path),"exec"),env)
                actual=env["clearance_limits"]()
                clear=LineString([(case["left"],0),(case["spacing"]-case["right"],0)]).length
                required=LineString([(0,0),(case["width"]+2*case["margin"],0)]).length
                self.assertAlmostEqual(actual["clear_width"],clear)
                self.assertAlmostEqual(actual["required_clear_width"],required)
                self.assertAlmostEqual(actual["min_spacing"],required+case["left"]+case["right"])
                self.assertEqual(actual["fits"],clear>=required)

if __name__=="__main__":unittest.main()

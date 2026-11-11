"""🚘 Verify the actual assumed collision band encloses the authored car."""
import ast,json,re,unittest
from pathlib import Path
from jsonschema import Draft202012Validator
from shapely.geometry import LineString
ROOT=next(p for p in Path(__file__).resolve().parents if (p/"🧰️framework").is_dir())
HERE=Path(__file__).parent
class CollisionHeight(unittest.TestCase):
    def test_vehicle_containment_and_native_height_filter(self):
        """📏 The actual band contains the asset and the upper obstacle slice."""
        case=json.loads((HERE/"🔣️.json").read_text(encoding="utf8"))
        Draft202012Validator(json.loads((HERE/"🔣️.schema.json").read_text(encoding="utf8"))).validate(case)
        model=ROOT/"🎓️teaching/🏛️architecture/🏗️bim/🎬️clip/🧱️model"
        data=json.loads((model/"🔣️.json").read_text(encoding="utf8"))
        asset=json.loads((ROOT/"🎓️teaching/🏛️architecture/🧍️staffage/🗂️catalog/🚜️vehicles/vintage-car-1929/🔣️.json").read_text(encoding="utf8"))["asset"]
        height=data["car"]["height"]["value"]
        note=next(item for item in data["notes"] if item["id"]=="physical-datums-r12")["text"]
        def agrees(text):
            claims=re.findall(r"(?:height|höhe)\s*([0-9]+[.,][0-9]+)",text,re.IGNORECASE)
            return all(abs(float(value.replace(",","."))-height)<1e-9 for value in claims)
        for example in case["note_cases"]:
            with self.subTest(note=example):
                self.assertEqual(agrees(example["text"]),example["agrees"])
        for text in note.values():
            self.assertTrue(agrees(text))
        asset_height=asset["dimensions"]["heightM"]
        self.assertEqual(asset_height,case["expected"]["vehicle_height"])
        obstacle=case["obstacle"]
        oracle=LineString([(0,0),(0,asset_height)]).intersection(LineString([(0,obstacle["z0"]),(0,obstacle["z1"])]))
        self.assertFalse(oracle.is_empty)
        source=ast.parse((model/"🐍️.py").read_text(encoding="utf8"))
        fn=next(n for n in source.body if isinstance(n,ast.FunctionDef) and n.name=="_vehicle_part")
        namespace={}
        exec(compile(ast.Module(body=[fn],type_ignores=[]),str(model/"🐍️.py"),"exec"),namespace)
        with self.subTest(check="containment"):
            self.assertGreaterEqual(height,asset_height)
        with self.subTest(check="obstacle"):
            self.assertEqual(namespace["_vehicle_part"](obstacle,{"ground":0.,"height":height}),case["expected"]["detected"])
if __name__=="__main__":unittest.main()

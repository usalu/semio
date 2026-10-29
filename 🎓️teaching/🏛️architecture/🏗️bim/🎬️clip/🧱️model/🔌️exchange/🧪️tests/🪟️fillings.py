"""🪟 Native multipart filling STEP geometry and material aspect contracts."""
import importlib
import json
from pathlib import Path
import unittest

core=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
parser=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧪️tests.🐍️")
CASES=json.loads(Path(__file__).with_suffix(".json").read_text())


class NativeFillingStepContract(unittest.TestCase):
    """📦 Preserve native product identity, physical parts and per-component materials."""

    @classmethod
    def setUpClass(cls):
        cls.model=core.load_model()

    def _parse(self,tag,language="en",spatial=True):
        result=core.step_excerpt(tag,self.model,language=language,spatial=spatial)
        return result,parser.step_parse(result["lines"])["entities"]

    def test_full_source_schema_accepts_normalized_panel_stations(self):
        """🔢 A full Draft202012Validator check accepts the source's dimensionless panel stations."""
        import jsonschema
        schema=json.loads((Path(core.__file__).parent/"🧬️schema/🔣️.schema.json").read_text(encoding="utf-8"))
        jsonschema.Draft202012Validator.check_schema(schema)
        jsonschema.Draft202012Validator(schema).validate(self.model)

    def test_public_filling_interface_matches_native_body_source(self):
        """🔗 Native STEP uses the public shared filling interface without private geometry access."""
        for case in CASES["cases"]:
            physical=core.filling_geometry(case["tag"],self.model)
            self.assertGreaterEqual(len(physical["parts"]),case["minimum_parts"])
            self.assertEqual(physical["tag"],case["tag"])
        with self.assertRaises(ValueError):
            core.filling_geometry("W-S",self.model)

    def test_native_entities_types_and_attribute_positions(self):
        """🧾 Native dimensions precede predefined and operation attributes in the IFC schema."""
        for case in CASES["cases"]:
            for language in CASES["languages"]:
                with self.subTest(tag=case["tag"],language=language):
                    result,entities=self._parse(case["tag"],language)
                    element=entities[int(result["ids"]["element"][1:])]
                    self.assertEqual(element[0],case["entity"])
                    self.assertEqual(len(element[1]),CASES["attribute_counts"][case["entity"]])
                    for name,args in entities.values():
                        if name in CASES["attribute_counts"]:
                            self.assertEqual(len(args),CASES["attribute_counts"][name])
                    self.assertEqual(entities[int(result["ids"]["type"][1:])][0],case["type"])
                    if case["entity"]!="IFCCURTAINWALL":
                        q=core.quantities(case["tag"],self.model)
                        self.assertAlmostEqual(element[1][8],q["rough_height"])
                        self.assertAlmostEqual(element[1][9],q["rough_width"])
                        self.assertIsNone(element[1][10])

    def test_all_exported_solid_parts_match_independent_polygon_volume(self):
        """🧮 GEOS reconstructs native part volumes from independently parsed STEP coordinates."""
        from shapely.geometry import Polygon
        for case in CASES["cases"]:
            with self.subTest(tag=case["tag"]):
                _,entities=self._parse(case["tag"])
                solids=[args for name,args in entities.values() if name=="IFCEXTRUDEDAREASOLID"]
                self.assertEqual(len(solids),len(core.filling_geometry(case["tag"],self.model)["parts"]))
                volume=0.0
                for solid in solids:
                    profile=entities[solid[0][1]][1]
                    curve=entities[profile[2][1]][1]
                    points=[entities[ref[1]][1][0] for ref in curve[0]]
                    volume+=Polygon(points).area*solid[3]
                self.assertAlmostEqual(volume,core.quantities(case["tag"],self.model)["volume"],places=7)

    def test_material_constituents_address_every_solid_exactly_once(self):
        """🪵 Every physical solid has one matching named material constituent and shape aspect."""
        for case in CASES["cases"]:
            with self.subTest(tag=case["tag"]):
                result,entities=self._parse(case["tag"])
                constituents={args[0]:args for name,args in entities.values() if name=="IFCMATERIALCONSTITUENT"}
                aspects={args[1]:args for name,args in entities.values() if name=="IFCSHAPEASPECT"}
                self.assertEqual(set(constituents),set(aspects))
                self.assertEqual({c[4] for c in constituents.values()},set(case["categories"]))
                covered=[]
                for name,aspect in aspects.items():
                    self.assertEqual(aspect[4],("ref",int(result["ids"]["shape"][1:])))
                    for representation in aspect[0]:
                        covered+=parser.step_refs(entities[representation[1]][1][3])
                solids=[number for number,(name,_) in entities.items() if name=="IFCEXTRUDEDAREASOLID"]
                self.assertCountEqual(covered,solids)
                self.assertEqual(len(covered),len(set(covered)))
                self.assertTrue(all(entities[c[2][1]][0]=="IFCMATERIAL" for c in constituents.values()))
                materials=[parser.step_decode(a[0]) for n,a in entities.values() if n=="IFCMATERIAL"]
                self.assertNotIn("Air cavity (excluded from solid body)",materials)

    def test_spatial_free_excerpt_has_native_occurrence_attributes_and_constituents(self):
        """🧭 Suppressing spatial context keeps a complete native selected-product fragment."""
        for case in CASES["cases"]:
            result,entities=self._parse(case["tag"],spatial=False)
            self.assertNotIn("type",result["ids"])
            self.assertEqual(sum(name=="IFCMATERIALCONSTITUENTSET" for name,_ in entities.values()),1)
            element=entities[int(result["ids"]["element"][1:])]
            self.assertEqual(len(element[1]),CASES["attribute_counts"][case["entity"]])


if __name__=="__main__":
    unittest.main()

"""🏗️ Registered hollow cellar projections and independent solid oracles."""
from copy import deepcopy
import importlib
import json
from pathlib import Path
import unittest

from scipy.integrate import quad
from scipy.spatial import ConvexHull
from shapely.geometry import Polygon
from shapely.ops import unary_union

api=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")

class CellarServicesContract(unittest.TestCase):
    def setUp(self):
        self.case=json.loads(Path(__file__).with_suffix(".json").read_text(encoding="utf-8"))
        self.model=api.load_model()
        self.elements={e["tag"]:e for e in self.model["elements"]}

    def test_registered_projections_have_native_connected_apertures(self):
        self.assertTrue(set(self.case["new_tags"])<=set(self.elements))
        for service,wall in (("hoist","W-CELLAR-E"),("coal","W-CELLAR-W")):
            case=self.case[service]
            hatch=self.elements[case["opening"]]
            self.assertEqual(hatch["class"],"IfcOpeningElement")
            self.assertEqual(hatch["geometry"]["slab"],case["host"])
            expected=Polygon(case["clear"] if service=="hoist" else case["terminal"])
            self.assertLess(Polygon(hatch["geometry"]["outline"]).symmetric_difference(expected).area,1e-10)
            self.assertEqual(self.elements[case["passage"]]["geometry"]["wall"],wall)
            self.assertEqual(self.elements[case["cover"]]["geometry"]["opening"],case["opening"])
            self.assertTrue(any(r["class"]=="IfcRelVoidsElement" and r["relating"]==case["host"] and case["opening"] in r["related"] for r in self.model["relations"]))
        self.assertLess(Polygon(self.case["coal"]["terminal"]).area,Polygon(self.case["coal"]["clear"]).area/3)

    def test_native_hatches_remove_host_even_without_covers(self):
        model=deepcopy(self.model)
        model["elements"]=[e for e in model["elements"] if e["tag"] not in (self.case["hoist"]["cover"],self.case["coal"]["cover"])]
        host=api.element("SL-GROUND-CELLAR",model)
        physical=api.represent(host["tag"],"3d",model)["primitives"]
        footprints=unary_union([Polygon(p["polygon"]) for p in physical])
        for service in ("hoist","coal"):
            aperture=Polygon(api.element(self.case[service]["opening"],model)["geometry"]["outline"])
            self.assertLess(footprints.intersection(aperture).area,1e-10)
            self.assertGreater(api.quantities(self.case[service]["opening"],model)["volume"],0)

    def test_hollow_service_solids_and_loft_volumes_match_independent_libraries(self):
        for service in ("hoist","coal"):
            obj=self.elements[self.case[service]["assembly"]]
            parts=api.solid_geometry(obj["tag"],self.model)["parts"]
            expected=0.
            for part in parts:
                vertices=api.part_vertices(part)
                expected+=ConvexHull(vertices).volume
            self.assertAlmostEqual(api.quantities(obj["tag"],self.model)["volume"],expected,places=7)
            clear=Polygon(self.case[service]["clear"])
            at=-1.2 if service=="hoist" else -.5
            slices=[Polygon(api.part_outline_at(p,at)) for p in parts if p["z0"]<at<p["z1"]]
            self.assertLess(unary_union(slices).intersection(clear.buffer(-.02)).area,1e-10)
        coal=self.elements[self.case["coal"]["assembly"]]["geometry"]["parts"]
        self.assertTrue(any("section" in p for p in coal))

    def test_spatial_type_material_and_partial_model_ownership(self):
        types={t["tag"] for t in self.model["types"]}
        for tag in self.case["new_tags"]:
            obj=self.elements[tag]
            self.assertTrue(any(r["class"]=="IfcRelContainedInSpatialStructure" and r["relating"]==obj["storey"] and tag in r["related"] for r in self.model["relations"]))
            if obj["class"]!="IfcOpeningElement":
                self.assertIn(obj["type"],types)
                self.assertTrue(any(r["class"]=="IfcRelDefinesByType" and r["relating"]==obj["type"] and tag in r["related"] for r in self.model["relations"]))
                self.assertTrue(any(r["class"]=="IfcRelAssociatesMaterial" and r["relating"]==obj["material"] and tag in r["related"] for r in self.model["relations"]))
            for domain in obj["metadata"]["domains"]:
                self.assertIn(tag,next(m for m in self.model["models"] if m["id"]==domain)["element_tags"])

    def test_registered_projections_clear_actual_numbered_structure(self):
        footprints=[Polygon(self.case[s]["clear"]) for s in ("hoist","coal")]
        for obj in self.model["elements"]:
            if obj["class"]=="IfcColumn":
                for part in api.column_geometry(self.model,obj)["parts"]:
                    if part["z0"]>=0 or part["z1"]<=-2.50:
                        continue
                    self.assertTrue(all(p.intersection(Polygon(part["outline"])).area<1e-9 for p in footprints),obj["tag"])
            if obj["class"]=="IfcFooting" and obj["tag"].startswith("FT-PAD-"):
                for part in obj["geometry"]["parts"]:
                    self.assertTrue(all(p.intersection(Polygon(api.part_footprint(part))).area<1e-9 for p in footprints),obj["tag"])
                                                                                                             
    def test_actual_cellar_wall_mouths_are_open_and_closed_covers_are_separate(self):
        for service,wall in (("hoist","W-CELLAR-E"),("coal","W-CELLAR-W")):
            case=self.case[service]
            mouth=api.represent(case["passage"],"3d",self.model)["primitives"]
            wall_solids=api.represent(wall,"3d",self.model)["primitives"]
            opening=unary_union([Polygon(p["polygon"]) for p in mouth if p["z0"]< -1.2 <p["z1"]])
            material=unary_union([Polygon(p["polygon"]) for p in wall_solids if p["z0"]< -1.2 <p["z1"]])
            self.assertGreater(opening.area,0)
            self.assertLess(opening.intersection(material).area,1e-10)
            hatch=Polygon(self.elements[case["opening"]]["geometry"]["outline"])
            cover=api.represent(case["cover"],"3d",self.model)["primitives"]
            self.assertLess(unary_union([Polygon(p["polygon"]) for p in cover]).symmetric_difference(hatch).area,1e-10)
            self.assertTrue(all(p["z1"]<=0 for p in cover))

    def test_service_types_classify_and_aggregate_native_occurrences(self):
        for tag in ("TY-CELLAR-SERVICE-CONCRETE","TY-CELLAR-SERVICE-METAL","TY-CELLAR-HATCH"):
            obj=api.element(tag,self.model)
            chain=[item["class"] for item in api.class_chain(obj["class"],self.model)]
            self.assertIn("IfcTypeProduct",chain)
            self.assertEqual(chain[-1],obj["class"])
            occurrences=api.type_occurrences(tag,self.model)
            quantities=api.quantities(tag,self.model)
            self.assertEqual(quantities["count"],len(occurrences))
            self.assertAlmostEqual(quantities["volume"],sum(api.quantities(t,self.model)["volume"] for t in occurrences))

    def test_physical_service_solids_do_not_overlap_columns_or_door_fillings(self):
        services=[p for tag in ("CELLAR-HOIST-PIT","CELLAR-COAL-CHUTE","HOIST-JIB") for p in api.solid_geometry(tag,self.model)["parts"]]
        foreign=[]
        for obj in self.model["elements"]:
            if obj["class"]=="IfcColumn":
                foreign.extend((obj["tag"],p) for p in api.column_geometry(self.model,obj)["parts"])
            elif obj["class"]=="IfcDoor":
                foreign.extend((obj["tag"],p) for p in api.filling_geometry(obj["tag"],self.model)["parts"])
        def slice_polygon(part,z):
            if "section" in part:
                from shapely.geometry import LineString,box
                section=part["section"]
                hit=Polygon(section["profile"]).intersection(LineString([[-100,z],[100,z]]))
                if hit.is_empty or hit.geom_type!="LineString":
                    return Polygon()
                a,b=hit.bounds[0],hit.bounds[2]
                lo,hi=section["sweep"]
                return box(a,lo,b,hi) if section["plane"]=="XZ" else box(lo,a,hi,b)
            if "top_outline" in part:
                t=(z-part["z0"])/(part["z1"]-part["z0"])
                return Polygon([[a[0]+t*(b[0]-a[0]),a[1]+t*(b[1]-a[1])] for a,b in zip(part["outline"],part["top_outline"])])
            return Polygon(part["outline"])
        for p in services:
            for tag,q in foreign:
                lo,hi=max(p["z0"],q["z0"]),min(p["z1"],q["z1"])
                if hi<=lo or not Polygon(api.part_footprint(p)).intersects(Polygon(api.part_footprint(q))):
                    continue
                volume=quad(lambda z:slice_polygon(p,z).intersection(slice_polygon(q,z)).area,lo,hi,epsabs=1e-10)[0]
                self.assertLess(volume,1e-9,tag)

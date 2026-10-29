"""🧮 Slab regions partition reference surfaces before any editable holes are removed."""

from copy import deepcopy
import importlib
import json
from pathlib import Path
import unittest

from shapely.geometry import Polygon
from shapely.ops import unary_union
from shapely.strtree import STRtree

api=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
CASE=json.loads(Path(__file__).with_suffix(".json").read_text(encoding="utf-8"))


def specimen(case):
    m=api.load_model()
    slab=deepcopy(api.element("SL-1",m))
    slab.update(tag="SL-PARTITION-CHECK",guid=api.global_id("SL-PARTITION-CHECK"),storey="ground")
    slab.pop("material",None)
    q=lambda value:{"value":value,"unit":"m","basis":"inferred"}
    slab["geometry"]={"level":"ground","outline":case["outline"],"voids":case["voids"],"top_offset":q(0),"regions":[{"tag":r["tag"],"outline":r["outline"],"thickness":q(r["thickness"]),"top_offset":q(0)} for r in case["regions"]]}
    m["elements"].append(slab)
    return m,slab


class SlabPartitionContract(unittest.TestCase):
    def test_clipping_retains_authored_reference_precision(self):
        """🔬 Match GEOS area without rounding distinct source coordinates to nanometres."""
        case=CASE["clipping_precision"]
        parts=api._slab_surface(case["outline"],case["outline"],[])
        expected=Polygon(case["outline"])
        self.assertLess(abs(sum(Polygon(p).area for p in parts)-expected.area),case["area_tolerance"])
        self.assertLess(unary_union([Polygon(p) for p in parts]).symmetric_difference(expected).area,case["area_tolerance"])

        case=CASE["shared_edge_clipping"]
        self.assertEqual(Polygon(case["subject"]).intersection(Polygon(case["clip"])).area,case["expected_area"])
        for subject,clip in ((case["subject"],case["clip"]),(case["clip"],case["subject"])):
            for reverse in (False,True):
                for offset in range(len(clip)):
                    boundary=clip[offset:]+clip[:offset]
                    boundary=list(reversed(boundary)) if reverse else boundary
                    for digits in case["digits"]:
                        with self.subTest(reverse=reverse,offset=offset,digits=digits):
                            ring=api._analysis._clip(subject,boundary,digits=digits)
                            self.assertLess(len(ring),3,"A shared edge must not become occupied material")
        positive=case["positive_facet"]
        expected=Polygon(positive["subject"]).intersection(Polygon(positive["clip"]))
        for digits in case["digits"]:
            actual=Polygon(api._analysis._clip(positive["subject"],positive["clip"],digits=digits))
            self.assertTrue(actual.is_valid)
            self.assertGreater(actual.area,0)
            self.assertAlmostEqual(expected.area,positive["expected_area"],delta=positive["area_tolerance"])
            self.assertAlmostEqual(actual.area,expected.area,delta=positive["area_tolerance"])

    def test_near_boundary_and_disjoint_holes_match_independent_union(self):
        m,slab=specimen(CASE)
        before=deepcopy(m)
        g=api._slab_geometry(m,slab)
        expected=Polygon(CASE["outline"]).difference(unary_union([Polygon(v["outline"]) for v in CASE["voids"]]))
        actual=unary_union([Polygon(p["outline"]) for p in g["parts"]])
        self.assertLess(actual.symmetric_difference(expected).area,1e-8)
        self.assertAlmostEqual(expected.area,CASE["expected"]["area"],places=9)
        expected_volume=sum(Polygon(r["outline"]).intersection(expected).area*r["thickness"] for r in CASE["regions"])
        actual_volume=sum(api._area(p["outline"])*(p["z1"]-p["z0"]) for p in g["parts"])
        self.assertAlmostEqual(expected_volume,CASE["expected"]["volume"],places=9)
        self.assertAlmostEqual(actual_volume,expected_volume,places=8)
        self.assertEqual(m,before)

    def test_editable_holes_cannot_hide_ambiguous_region_ownership(self):
        case=CASE["hidden_overlap"]
        left,right=[Polygon(r["outline"]) for r in case["regions"]]
        self.assertAlmostEqual(left.intersection(right).area,.5)
        hole=Polygon(case["voids"][0]["outline"])
        self.assertAlmostEqual(left.difference(hole).intersection(right.difference(hole)).area,0)
        m,slab=specimen(case)
        with self.assertRaisesRegex(ValueError,"overlap"):
            api._slab_geometry(m,slab)

    def test_source_region_surfaces_and_sleeves_match_independent_geos(self):
        m=api.load_model()
        fixture=json.loads(Path(__file__).with_name("🏗️bearing.json").read_text(encoding="utf-8"))
        source=next(item for item in m["elements"] if item["tag"]==fixture["slab"]);slab=api._slab_geometry(m,source)
        self.assertEqual(source["geometry"]["column_reserves"],fixture["columns"])
        self.assertEqual({part["owner"] for part in slab["shared_parts"]},set(fixture["columns"]))
        full=[(part,Polygon(part["outline"])) for part in slab["full_parts"]]
        reference=0.0
        for tag in fixture["columns"]:
            for column in api.column_geometry(m,next(item for item in m["elements"] if item["tag"]==tag))["parts"]:
                self.assertFalse(column.get("holes"),tag)
                profile=Polygon(column["outline"])
                for part,shape in full:
                    low,high=max(part["z0"],column["z0"]),min(part["z1"],column["z1"])
                    if high>low:
                        reference+=shape.intersection(profile).area*(high-low)
        volume=lambda parts:sum(Polygon(part["outline"]).area*(part["z1"]-part["z0"]) for part in parts)
        area=lambda parts:sum(Polygon(part["outline"]).area for part in parts)
        self.assertGreater(reference,0)
        self.assertAlmostEqual(volume(slab["shared_parts"]),reference,places=8)
        self.assertAlmostEqual(reference,fixture["shared_structural_volume"],places=8)
        self.assertAlmostEqual(volume(slab["parts"])+volume(slab["shared_parts"]),volume(slab["full_parts"]),places=8)
        self.assertAlmostEqual(area(slab["parts"])+area(slab["shared_parts"]),area(slab["full_parts"]),places=8)

        for slab in [e for e in m["elements"] if e["class"]=="IfcSlab"]:
            with self.subTest(slab=slab["tag"]):
                g=api._slab_geometry(m,slab)
                expected=Polygon(g["outline"]).difference(unary_union([Polygon(v) for v in g["voids"]]))
                for tag in slab["geometry"].get("column_reserves", []):
                    self.assertFalse(any(p.get("holes") for p in api.column_geometry(m,api.element(tag,m))["parts"]), "Source reserve fixture requires solid columns")
                polygons=[Polygon(p["outline"]) for p in g["full_parts"]]
                tree=STRtree(polygons)
                self.assertAlmostEqual(sum(p.area for p in polygons),expected.area,places=6)
                for i,polygon in enumerate(polygons):
                    self.assertTrue(polygon.is_valid)
                    if not expected.covers(polygon):
                        self.assertLess(polygon.difference(expected).area,1e-8)
                    for j in tree.query(polygon):
                        if j>i:
                            self.assertLess(polygon.intersection(polygons[j]).area,1e-8)

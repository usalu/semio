"""🪚 Material leaves exclude concrete members and owned filling assemblies."""

from copy import deepcopy
import importlib
import json
import math
from pathlib import Path
import unittest

from scipy.integrate import quad
from shapely.geometry import Polygon
from shapely.ops import unary_union

api=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")


class MaterialOwnershipContract(unittest.TestCase):
    def test_source_wall_ownership_matches_independent_material_difference(self):
        """🔍 Check each changed wall against independent raw-minus-owned GEOS solids."""
        case=json.loads(Path(__file__).with_suffix(".json").read_text(encoding="utf-8"))["source_partition"]
        for tag in [case["wall"],*case["additional_walls"]]:
            with self.subTest(wall=tag):
                self._assert_source_wall_partition(tag,case["empty_material_band"] if tag==case["wall"] else None)

    def _assert_source_wall_partition(self,tag,empty_band=None):
        """🏡 Prove source wall bands exclude actual native cutters without phantom leaves."""
        from unittest.mock import patch
        from shapely import set_precision
        source=api.load_model();wall=api._find(source,tag)
        def raw(model,parts):
            """📋 Preserve source-authored leaves before material ownership subtraction."""
            return deepcopy(parts),0.,sum(Polygon(p["outline"]).area*(p["z1"]-p["z0"]) for p in parts)
        with patch.object(api,"_wall_owned_parts",side_effect=raw):unowned=api._wall_geometry(source,wall)["parts"]
        owned=api._wall_geometry(source,wall)["parts"]
        bounds=unary_union([Polygon(p["outline"]) for p in unowned]).bounds
        zmin,zmax=min(p["z0"] for p in unowned),max(p["z1"] for p in unowned)
        cutters=[]
        for p in api._wall_cutters(source):
            shape=Polygon(p["outline"],p.get("holes",[]));x0,y0,x1,y1=shape.bounds
            if x0<bounds[2] and x1>bounds[0] and y0<bounds[3] and y1>bounds[1] and p["z0"]<zmax and p["z1"]>zmin:cutters.append((p,shape))
        levels=sorted({p[k] for p in unowned for k in ("z0","z1")}|{max(zmin,min(zmax,p[k])) for p,_ in cutters for k in ("z0","z1")})
        materials={p["material"] for p in unowned}
        for lo,hi in zip(levels,levels[1:]):
            if hi-lo<=1e-10:continue
            mid=(lo+hi)/2;active=[shape for p,shape in cutters if p["z0"]<mid<p["z1"]]
            for material in materials:
                raw_shapes=[Polygon(p["outline"]) for p in unowned if p["material"]==material and p["z0"]<mid<p["z1"]]
                shapes=[Polygon(p["outline"]) for p in owned if p["material"]==material and p["z0"]<mid<p["z1"]]
                self.assertTrue(all(p.is_valid for p in shapes))
                perimeter=sum(p.length for p in raw_shapes+active+shapes)
                grid=min(1e-10,1e-9/(perimeter+1))
                bound=perimeter*grid+len(raw_shapes+active+shapes)*math.pi*grid*grid
                base=unary_union([set_precision(p,grid) for p in raw_shapes])
                cut=unary_union([set_precision(p,grid) for p in active])
                expected=base.difference(cut)
                actual=unary_union([set_precision(p,grid) for p in shapes])
                self.assertLess(actual.symmetric_difference(expected).area+bound,1e-8,(tag,material,lo,hi))
                self.assertLess(abs(sum(p.area for p in shapes)-expected.area)+bound,1e-8)
        if empty_band is not None:
            mid=(empty_band["z0"]+empty_band["z1"])/2
            self.assertAlmostEqual(sum(Polygon(p["outline"]).area for p in owned if p["material"]==empty_band["material"] and p["z0"]<mid<p["z1"]),empty_band["expected_area"],delta=1e-8)

    def test_disjoint_cutters_do_not_fragment_occupied_material(self):
        """⚡ Preserve disjoint partitions and compare every cut with independent GEOS."""
        case=json.loads(Path(__file__).with_suffix(".json").read_text(encoding="utf-8"))["disjoint_difference"]
        for item in case["cases"]:
            with self.subTest(case=item["id"]):
                parts=api._analysis._subtract(case["part"],item["cutter"],digits=12,tolerance=1e-12)
                expected=Polygon(case["part"]).difference(Polygon(item["cutter"]))
                self.assertLess(unary_union([Polygon(p) for p in parts]).symmetric_difference(expected).area,1e-10)
                if "maximum_parts" in item:self.assertLessEqual(len(parts),item["maximum_parts"])

    def test_prism_subtraction_preserves_metadata_and_disjoint_material_volume(self):
        case=json.loads(Path(__file__).with_suffix(".json").read_text(encoding="utf-8"))
        original=deepcopy(case)
        parts=api._subtract_prisms([case["part"]],case["cutters"])
        self.assertEqual(case,original)
        for part in parts:
            self.assertEqual(part["material"],case["part"]["material"])
            self.assertEqual(part["layer"],case["part"]["layer"])
            for cutter in case["cutters"]:
                overlap=max(0,min(part["z1"],cutter["z1"])-max(part["z0"],cutter["z0"]))
                self.assertLess(Polygon(part["outline"]).intersection(Polygon(cutter["outline"])).area*overlap,1e-10)
        def area(z):
            active=[Polygon(c["outline"]) for c in case["cutters"] if c["z0"] < z < c["z1"]]
            return Polygon(case["part"]["outline"]).difference(unary_union(active)).area
        expected=quad(area,0,3,points=[1,2,2.7],epsabs=1e-12)[0]
        self.assertAlmostEqual(expected,case["volume"])
        self.assertAlmostEqual(api._occupied_volume(parts),expected,places=9)
        self.assertEqual(api._subtract_prisms([case["part"]],[{**case["part"],"tag":"whole"}]),[])

    def test_source_masonry_excludes_registered_concrete_columns(self):
        case=json.loads(Path(__file__).with_suffix(".json").read_text(encoding="utf-8"))
        source=api.load_model()
        wall=api._wall_geometry(source,api._find(source,case["source_wall"]))
        cutters=[p for obj in api._by_class(source,"IfcColumn") for p in api.column_geometry(source,obj)["parts"]]
        overlap=0.
        for part in wall["parts"]:
            polygon=Polygon(part["outline"])
            for cutter in cutters:
                height=max(0,min(part["z1"],cutter["z1"])-max(part["z0"],cutter["z0"]))
                if height:
                    overlap += polygon.intersection(Polygon(cutter["outline"])).area*height
        self.assertLess(overlap,1e-9)

    def test_coplanar_cuts_keep_connected_boundaries_instead_of_render_fragments(self):
        case=json.loads(Path(__file__).with_suffix(".json").read_text(encoding="utf-8"))["circular"]
        result=api._subtract_prisms([case["wall"]],[case["cutter"]])
        self.assertLessEqual(len(result),case["max_parts"])
        expected=Polygon(case["wall"]["outline"]).difference(Polygon(case["cutter"]["outline"]))
        actual=unary_union([Polygon(p["outline"]) for p in result])
        self.assertTrue(actual.is_valid)
        self.assertLess(actual.symmetric_difference(expected).area,1e-10)


    def test_exact_collinear_spikes_preserve_thin_material(self):
        """🪡 Remove only zero-area boundary excursions while preserving a real nanometre-wide triangle."""
        case=json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf8'))['exact_spike']
        expected=Polygon(case['expected_polygon'])
        self.assertAlmostEqual(expected.area,case['expected_area'],delta=case['area_tolerance'])
        for direction in (case['ring'],list(reversed(case['ring']))):
            for i in range(len(direction)):
                actual=api._analysis._ccw(direction[i:]+direction[:i]);shape=Polygon(actual)
                self.assertTrue(shape.is_valid)
                self.assertEqual({tuple(p) for p in actual},{tuple(p) for p in case['expected_polygon']})
                self.assertLessEqual(shape.symmetric_difference(expected).area,case['area_tolerance'])

    def test_source_curtain_material_rings_are_valid_without_material_loss(self):
        """🏷️ Preserve true thin material while assigning hosted door material exactly once."""
        case=json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf8'))['exact_spike'];source=api.load_model()
        parts=api.filling_geometry(case['source']['tag'],source)['parts']
        filling=api.filling_geometry(case['source']['hosted_filling'],source)['parts']
        self.assertTrue(all(Polygon(p['outline']).is_valid for p in parts))
        original=Polygon(case['expected_polygon']);lo=case['source']['z0'];hi=case['source']['z1']
        levels=sorted({lo,hi}|{max(lo,min(hi,p[k])) for p in parts+filling for k in ('z0','z1')})
        volume=0.
        for low,high in zip(levels,levels[1:]):
            mid=(low+high)/2
            hosted=unary_union([Polygon(p['outline'],p.get('holes',[])) for p in filling if p['z0']<mid<p['z1']])
            expected=original.difference(hosted)
            retained=unary_union([Polygon(p['outline'],p.get('holes',[])).intersection(original) for p in parts if p['z0']<mid<p['z1']])
            self.assertLessEqual(expected.area,case['area_tolerance'])
            self.assertLessEqual(retained.symmetric_difference(expected).area,case['area_tolerance'])
            self.assertLessEqual(retained.intersection(hosted).area,case['area_tolerance'])
            self.assertAlmostEqual(original.intersection(hosted).area,case['expected_area'],delta=case['area_tolerance'])
            volume+=retained.area*(high-low)
        self.assertAlmostEqual(volume,case['source']['expected_retained_area']*(hi-lo),delta=case['area_tolerance'])

if __name__ == "__main__":
    unittest.main()
 
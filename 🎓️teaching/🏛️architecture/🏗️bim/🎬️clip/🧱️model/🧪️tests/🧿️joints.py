"""🧿️ Source-neutral conservative circular interface envelopes without changing the circle."""
import importlib,json,math
from pathlib import Path
import unittest
import numpy as np
from scipy.spatial import ConvexHull
from shapely.geometry import Polygon
profiles=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧊️profiles.🐍️")
class CircularInterfaceContract(unittest.TestCase):
    def test_every_side_contains_the_true_circle_and_outward_error_is_bounded(self):
        """📏 Compare exact tangent bounds with independent NumPy, SciPy and GEOS geometry."""
        from jsonschema import Draft202012Validator
        c=json.loads(Path(__file__).with_suffix(".json").read_text());Draft202012Validator(json.loads(Path(__file__).with_suffix(".schema.json").read_text())).validate(c)
        for row in c["cases"]:
            g=profiles.circle_envelope(**row);points=np.asarray(g["outline"]);lengths=np.linalg.norm(points,axis=1);edges=np.roll(points,-1,axis=0)-points;distance=np.abs(edges[:,0]*points[:,1]-edges[:,1]*points[:,0])/np.linalg.norm(edges,axis=1)
            self.assertGreaterEqual(g["sides"],row["minimum_sides"])
            if g["sides"]%4==0:
                self.assertEqual(g["outline"][g["sides"]//4],[0.,g["outer_radius"]])
                self.assertEqual(g["outline"][g["sides"]//2],[-g["outer_radius"],0.])
            self.assertGreaterEqual(float(distance.min()),row["radius"])
            self.assertLessEqual(float(lengths.max())-row["radius"],row["maximum_outward"])
            self.assertAlmostEqual(ConvexHull(points).volume,Polygon(points).area,delta=max(1e-40,Polygon(points).area*1e-12))
            expected=g["sides"]*g["outer_radius"]**2*math.sin(2*math.pi/g["sides"])/2
            self.assertAlmostEqual(Polygon(points).area,expected,delta=max(1e-40,expected*1e-12))
            self.assertGreater(Polygon(points).area,math.pi*row["radius"]**2)
    def test_nonpositive_precision_and_unbounded_safety_fail(self):
        """🚫 Reject envelopes that cannot preserve their declared finite geometric bound."""
        for row in json.loads(Path(__file__).with_suffix(".json").read_text())["invalid"]:
            with self.assertRaises(ValueError):profiles.circle_envelope(**row)


class SourceAffineStockContract(unittest.TestCase):
    def test_registered_ramp_and_post_share_exact_source_plane(self):
        """🪜 Preserve original decimal affine stations before the first public preview."""
        from fractions import Fraction
        from decimal import Decimal
        a=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️");m=a.load_model();c=json.loads(Path(__file__).with_suffix(".json").read_text())["affine"];stock=a._ramp_reference_stock(m,a._find(m,c["flight"]));column=a._interface_reference_parts(m,c["column"])
        self.assertEqual(Decimal(c["origin"])+Decimal(c["offset"]),Decimal(c["shared_x"]))
        self.assertTrue(any(v[0]==Fraction(c["shared_x"]) for v in stock["mesh"]["vertices"]))
        self.assertTrue(any(p==[Fraction(c["shared_x"]),Fraction(c["shared_y"])] for part in column for p in part["outline"]))
        public=np.asarray(stock["mesh"]["vertices"],dtype=float);self.assertAlmostEqual(ConvexHull(public).volume,8.659*1.458*.06,places=10)
    def test_private_affine_stock_retains_positive_one_e_minus30_section(self):
        """🪡 Keep a positive source section without rounding, snapping or facet cleanup."""
        from copy import deepcopy
        from fractions import Fraction
        a=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️");m=a.load_model();obj=deepcopy(a._find(m,"RF-2A"));q=lambda v:{"value":v,"unit":"m","basis":"inferred"};obj["geometry"].update(path=[[0.,0.],[1e-15,0.]],from_level=0.,to_level=1e-15,portion=[0,1],structural_offsets=[q(-1e-30),q(1e-30)],thickness=q(1e-30),finish_thickness=q(1e-30),finish_direction="vertical");stock=a._ramp_reference_stock(m,obj);vertices=stock["mesh"]["vertices"]
        self.assertTrue(all(isinstance(x,Fraction) for point in vertices for x in point))
        for lower,upper in zip(vertices[:4],vertices[4:]):self.assertEqual(upper[2]-lower[2],Fraction("1e-30"))
        self.assertGreater(max(p[1] for p in vertices)-min(p[1] for p in vertices),0)

    def test_native_prism_endpoints_keep_original_fraction_coordinates(self):
        """🧊 Preserve registered source planes through native prism endpoint resolution."""
        from fractions import Fraction as Q
        a=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️");p={"outline":[[Q("9.47"),Q("16.814")],[Q("9.56"),Q("16.814")],[Q("9.56"),Q("16.889")],[Q("9.47"),Q("16.889")]],"z0":Q("3.17"),"z1":Q("6.6"),"material":"NEUTRAL","role":"stock"}
        vertices=a.part_vertices(p)
        self.assertTrue(all(isinstance(v,Q) for point in vertices for v in point))
        self.assertEqual(vertices[0],[Q("9.47"),Q("16.814"),Q("3.17")])
        cells=a._native_material_cells(p)
        self.assertTrue(all(isinstance(v,Q) for cell in cells for point in cell["mesh"]["vertices"] for v in point))

    def test_embedded_member_keeps_original_region_and_real_aperture(self):
        """🧱 Reconcile exposed and slab-owned stock against independent perforated solid volume."""
        from copy import deepcopy
        from fractions import Fraction as Q
        from shapely.geometry import Point,MultiPoint
        a=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️");m=a.load_model();c=json.loads(Path(__file__).with_suffix(".json").read_text())["embedded"];beam=deepcopy(a._find(m,"BEAM-PRIMARY-F-14-19"));beam["geometry"].update(parts=[],embedded_slabs=["NEUTRAL-MEMBER-SLAB"]);slab={"tag":"NEUTRAL-MEMBER-SLAB","guid":a.global_id("NEUTRAL-MEMBER-SLAB"),"class":"IfcSlab","geometry":{"level":0,"outline":c["outline"],"voids":[{"tag":"PHYSICAL-HOLE","outline":c["hole"]}],"regions":[{"member":beam["tag"],"outline":c["outline"],"top_offset":{"value":c["top"]},"thickness":{"value":c["depth"]},"material":"MAT-RC"}]}};m["elements"]=[e for e in m["elements"] if e["tag"]!=beam["tag"]]+[beam,slab];parts=a._interface_reference_parts(m,beam["tag"]);expected=Polygon(c["outline"],[c["hole"]]);self.assertAlmostEqual(expected.area*c["depth"],c["volume"]);self.assertAlmostEqual(sum(ConvexHull(np.asarray(a.part_vertices(p),dtype=float)).volume for p in parts),c["volume"])
        self.assertTrue(all(isinstance(v,Q) for p in parts for vertex in a.part_vertices(p) for v in vertex))
        for p in parts:
            projection=MultiPoint(np.asarray(a.part_vertices(p),dtype=float)[:,:2]).convex_hull
            self.assertFalse(projection.contains(Point(1,1)))

    def test_single_flight_query_does_not_partition_unrelated_structural_interfaces(self):
        """🎯 Resolve only requested material while preserving independent original primary stocks."""
        from unittest.mock import patch
        a=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️");m=a.load_model();c=json.loads(Path(__file__).with_suffix(".json").read_text())["selective"];resolve=a._interface_reference_parts;seen=[]
        def selected(model,key):
            self.assertIn(key,c["allowed_owners"]);seen.append(key);return resolve(model,key)
        with patch.object(a,"_interface_reference_parts",selected):g=a.ramp_geometry(c["flight"],m)
        self.assertEqual(set(seen),set(c["allowed_owners"]))
        self.assertAlmostEqual(sum(ConvexHull(np.asarray(a.part_vertices(p),dtype=float)).volume for p in g["parts"]),c["volume"],places=9)

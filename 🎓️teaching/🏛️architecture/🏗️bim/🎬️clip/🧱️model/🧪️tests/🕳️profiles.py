"""🕳️ Actual column bores preserve analytic material and closed physical geometry."""
from copy import deepcopy
import importlib
import json
import math
from pathlib import Path
import unittest
import numpy as np
from shapely.geometry import Polygon,Point
from shapely.ops import unary_union

api=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")

class HollowProfileContract(unittest.TestCase):
    def setUp(self):
        self.fixture=json.loads(Path(__file__).with_suffix(".json").read_text(encoding="utf-8"))

    def test_net_profile_material_and_second_moments(self):
        for case in self.fixture["cases"]:
            g=api.profile_geometry(case["profile"],[0,0],1024)
            expected=Polygon(g["outline"],[h for h in g["holes"]])
            self.assertAlmostEqual(g["area"],case["area"],places=11)
            self.assertLess(abs(expected.area-g["area"]),1e-6)
            self.assertEqual(len(g["holes"]),1)
            if "ix" in case:
                self.assertAlmostEqual(g["second_moments"]["x"],case["ix"],places=12)
                self.assertAlmostEqual(g["second_moments"]["y"],case["iy"],places=12)

    def test_hollow_mesh_is_closed_and_its_cap_matches_geos_difference(self):
        for case in self.fixture["cases"]:
            m=api.load_model();obj=next(e for e in m["elements"] if e["tag"]=="19")
            kind=next(t for t in m["types"] if t["tag"]==obj["type"])
            kind["profile"]=case["profile"]
            obj["geometry"]={"axis":[0,0],"base_level":0,"top_level":2}
            mesh=api.mesh(obj["tag"],48,m)
            vertices=np.asarray(mesh["vertices"])
            counts={}
            for face in mesh["faces"]:
                for a,b in zip(face,face[1:]+face[:1]):
                    edge=tuple(sorted((a,b)));counts[edge]=counts.get(edge,0)+1
            self.assertTrue(all(c==2 for c in counts.values()))
            signed=sum(np.dot(vertices[a],np.cross(vertices[b],vertices[c]))/6 for a,b,c in mesh["faces"])
            p=api.column_geometry(m,obj,48)["parts"][0]
            expected=Polygon(p["outline"],p["holes"])
            self.assertAlmostEqual(signed,expected.area*2,places=8)
            top=[Polygon(vertices[face,:2]) for face in mesh["faces"] if all(abs(vertices[i,2]-2)<1e-10 for i in face)]
            self.assertLess(unary_union(top).symmetric_difference(expected).area,1e-9)

    def test_native_column_representations_retain_voids(self):
        m=api.load_model();obj=next(e for e in m["elements"] if e["tag"]=="19")
        kind=next(t for t in m["types"] if t["tag"]==obj["type"]);kind["profile"]=self.fixture["cases"][0]["profile"]
        obj["geometry"]={"axis":[0,0],"base_level":0,"top_level":2}
        for kind in ("plan","footprint","3d","body"):
            self.assertTrue(api.represent(obj["tag"],kind,m)["primitives"][0]["holes"])
        for kind in ("section","elevation","axis"):
            self.assertTrue(api.represent(obj["tag"],kind,m)["primitives"])
        cut=api.represent(obj["tag"],"section",m)["primitives"]
        self.assertEqual(len(cut),2)

    def test_only_attested_pipe_route_sections_are_bored(self):
        m=api.load_model()
        for tag in self.fixture["source"]["tags"]:
            obj=next(e for e in m["elements"] if e["tag"]==tag)
            for p in api.column_geometry(m,obj)["parts"]:
                material=Polygon(p["outline"],p["holes"])
                centre=Point(obj["geometry"]["axis"])
                if p["z1"]<=self.fixture["source"]["solid_buried"][1]:
                    self.assertTrue(material.contains(centre))
                    self.assertFalse(p["profile"].get("voids"))
                else:
                    self.assertFalse(material.contains(centre))
                    self.assertGreaterEqual(p["z0"],self.fixture["source"]["ground_bore"][0])
                    if p["profile"].get("voids"):
                        self.assertEqual(p["profile"]["voids"][0]["diameter"]["value"],.1)
        lesson=api.column_geometry(m,next(e for e in m["elements"] if e["tag"]=="19"))
        self.assertFalse(lesson["profile"].get("voids"))

    def test_offset_bore_centroid_moments_match_geos_scipy_integration(self):
        from scipy.integrate import quad
        from shapely.geometry import LineString
        case=self.fixture["cases"][1]
        profile=case["profile"]
        g=api.profile_geometry(profile,[0,0],2048)
        physical=Polygon(g["outline"],g["holes"])
        cx,cy=physical.centroid.x,physical.centroid.y
        self.assertLess(math.dist(g["centroid"],[cx,cy]),1e-8)
        def width(y):
            return physical.intersection(LineString([[-1,y],[1,y]])).length
        ix=quad(lambda y:width(y)*(y-cy)**2,-.15,.15,epsabs=1e-11,points=[-.06,.02],limit=100)[0]
        def vertical(x):
            return physical.intersection(LineString([[x,-1],[x,1]])).length
        iy=quad(lambda x:vertical(x)*(x-cx)**2,-.2,.2,epsabs=1e-11,points=[0,.08],limit=100)[0]
        self.assertAlmostEqual(g["second_moments"]["x"],ix,places=9)
        self.assertAlmostEqual(g["second_moments"]["y"],iy,places=9)

    def test_native_step_uses_hollow_and_arbitrary_void_profiles(self):
        for i,case in enumerate(self.fixture["cases"]):
            m=api.load_model();obj=next(e for e in m["elements"] if e["tag"]=="19")
            next(t for t in m["types"] if t["tag"]==obj["type"])["profile"]=case["profile"]
            obj["geometry"]={"axis":[0,0],"base_level":0,"top_level":2}
            step=api.step_excerpt(obj["tag"],m)
            self.assertIn("IFCCIRCLEHOLLOWPROFILEDEF" if i==0 else "IFCARBITRARYPROFILEDEFWITHVOIDS","\n".join(step["lines"]))
            if i:
                self.assertIn("IFCCIRCLE(","\n".join(step["lines"]))

    def test_material_cuts_and_collision_slices_preserve_bore_air(self):
        case=self.fixture["cases"][0]
        part={**api.profile_geometry(case["profile"],[0,0],48),"z0":0,"z1":1}
        expected=Polygon(part["outline"],part["holes"])
        regions=api.part_regions(part,.5)
        actual=unary_union([Polygon(r) for r in regions])
        self.assertLess(actual.symmetric_difference(expected).area,1e-9)
        self.assertLess(unary_union([Polygon(r) for r in api.part_band_footprints(part,.2,.8)]).symmetric_difference(expected).area,1e-9)
        solid={"outline":[[-.2,-.2],[.2,-.2],[.2,.2],[-.2,.2]],"z0":0,"z1":1}
        result=api._subtract_prisms([solid],[part])
        self.assertLess(unary_union([Polygon(p["outline"]) for p in result]).symmetric_difference(Polygon(solid["outline"]).difference(expected)).area,1e-9)

    def test_production_plan_section_and_axon_show_a_real_bore(self):
        from types import SimpleNamespace
        drawing=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.🧪️ortho-remodel.test_frustums")
        scope=drawing._scope()
        m=api.load_model();obj=next(e for e in m["elements"] if e["tag"]=="19")
        next(t for t in m["types"] if t["tag"]==obj["type"])["profile"]=self.fixture["cases"][0]["profile"]
        obj["geometry"]={"axis":[0,0],"base_level":0,"top_level":2}
        data=api.column_geometry(m,obj,48);part=data["parts"][0]
        expected=Polygon(part["outline"],part["holes"])
        recorded=[]
        def region(rings,*args,**kwargs):
            recorded.append(Polygon(np.asarray(rings[0])[:,:2],[np.asarray(h)[:,:2] for h in rings[1:]]))
            return drawing.Shape()
        scope["_d"].region=region
        plan=scope["Plan"]();plan._pair=lambda *a:(0,0,1,0)
        ctx={"view":lambda x,y,z:np.array([x,y,z]),"look":SimpleNamespace(state=lambda item:"normal"),"w":1,"cut_z":1,"catalogue":[{"kind":"pilotis","tag":"19","data":data}],"bands":{"ground":(0,3)},"level":"ground","window":None,"screen":lambda ring:np.c_[ring,np.zeros(len(ring))],"scale":1,"put":lambda *a:None}
        plan._plan_pilotis(ctx,0,False,False)
        self.assertLess(recorded[0].symmetric_difference(expected).area,1e-10)
        section=scope["part_projection"](part,SimpleNamespace(value=0,index=1),"cut")
        actual=unary_union([Polygon(r) for r in section])
        self.assertLess(actual.intersection(Polygon([[-.049,0],[.049,0],[.049,2],[-.049,2]])).area,1e-10)
        faces=scope["_prism"]([part["outline"],*part["holes"]],0,2,"post")
        cap=next(face for face in faces if np.allclose(face[2],[0,0,1]))
        self.assertEqual(len(cap[1]),2)
        self.assertLess(Polygon([point[:2] for point in cap[1][0]],[[point[:2] for point in ring] for ring in cap[1][1:]]).symmetric_difference(expected).area,1e-10)
        self.assertEqual(len(faces),len(part["outline"])+len(part["holes"][0])+1)

    def test_belowgrade_exit_is_open_and_does_not_bore_the_footing(self):
        from shapely.geometry import LineString
        m=api.load_model()
        for tag in ("2","4"):
            obj=next(e for e in m["elements"] if e["tag"]==tag)
            parts=api.column_geometry(m,obj)["parts"]
            at=next(p for p in parts if p["z0"]<-.6<p["z1"])
            x,y=obj["geometry"]["axis"]
            ray=LineString([[x,y],[x+.20,y]])
            self.assertLess(Polygon(at["outline"],at["holes"]).intersection(ray).length,1e-10)
            self.assertEqual([round(v,6) for v in Polygon(at["outline"]).bounds],[round(x-.125,6),round(y-.125,6),round(x+.125,6),round(y+.125,6)])
            stem=next(p for p in parts if p["z0"]<-.9<p["z1"])
            self.assertTrue(Polygon(stem["outline"]).contains(Point(x,y)))
            footing=next(e for e in m["elements"] if e["tag"]=="FT-PAD-"+tag)
            self.assertTrue(all(not p.get("holes") and not p.get("profile",{}).get("voids") for p in footing["geometry"]["parts"]))

    def test_source_pipe_and_elbow_material_do_not_overlap_authored_column_material(self):
        m=api.load_model()
        for tag,pipe in (("2","DP-1"),("4","DP-2")):
            columns=[*api.column_geometry(m,next(e for e in m["elements"] if e["tag"]==tag))["parts"],*api.column_geometry(m,next(e for e in m["elements"] if e["tag"]==tag+"-FIRST"))["parts"]]
            pipes=[*api.service_geometry(pipe,m)["parts"],*api.service_geometry(pipe+"-OUTLET",m)["parts"]]
            for z in (-.64,-.63,-.60,-.56,-.54,-.4,-.2,0,1,2.8,3.2,6.3):
                concrete=unary_union([Polygon(p["outline"],p["holes"]) for p in columns if p["z0"]<=z<p["z1"]])
                metal=unary_union([Polygon(r) for p in pipes if p["z0"]<=z<p["z1"] for r in api.part_regions(p,z)])
                self.assertLess(concrete.intersection(metal).area,1e-10,(tag,z))

    def test_invalid_bores_are_rejected_and_native_evidence_is_explicit(self):
        for diameter,offset in ((.33,[0,0]),(.1,[.2,0])):
            profile=deepcopy(self.fixture["cases"][0]["profile"])
            profile["voids"][0]["diameter"]["value"]=diameter
            profile["voids"][0]["offset"]=offset
            with self.assertRaises(ValueError):
                api.profile_geometry(profile,[0,0])
        evidence="\n".join(api.step_excerpt("2")["lines"])
        self.assertIn("BoreConvention",evidence)
        self.assertIn("IFCTEXT(",evidence)
        self.assertIn("teaching inferences",evidence)

    def test_bore_quantities_are_localized_and_surface_ownership_is_explicit(self):
        m=api.load_model()
        values=api.quantities("2-FIRST",m)
        self.assertAlmostEqual(values["outer_surface_area"]+values["bore_surface_area"],values["lateral_surface_area"])
        for language in ("en","de"):
            for key in ("outer_surface_area","bore_surface_area","lateral_surface_area"):
                info=api.information("2-FIRST",key,language,m)
                self.assertEqual(info["unit"],"m²")
                self.assertEqual(info["basis"],"derived")
                self.assertNotEqual(info["label"],key)

    def test_structural_bearing_requires_actual_material_contact_outside_the_bore(self):
        part={**api.profile_geometry(self.fixture["cases"][0]["profile"],[0,0]),"z0":0,"z1":2}
        material=Polygon(part["outline"],part["holes"])
        for case in self.fixture["bearing_cases"]:
            slab={"parts":[{"outline":case["outline"],"z0":2,"z1":2.2}]}
            expected=material.intersection(Polygon(case["outline"])).area>1e-9
            self.assertEqual(expected,case["expected"])
            self.assertEqual(api._analysis._bearing({"parts":[part]},slab),expected)

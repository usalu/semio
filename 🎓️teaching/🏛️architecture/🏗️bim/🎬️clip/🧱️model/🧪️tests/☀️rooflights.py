"""☀️ Registered rooflight physical construction and independent material ownership."""
import importlib,json,unittest
from pathlib import Path
from shapely.geometry import Polygon,box,LineString
from scipy.spatial import ConvexHull
api=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
class RooflightConstruction(unittest.TestCase):
    def test_registered_aperture_and_material_assembly(self):
        model=api.load_model()
        for case in json.loads(Path(__file__).with_suffix(".json").read_text(encoding="utf-8"))["cases"]:
            obj=next(e for e in model["elements"] if e["tag"]==case["tag"])
            parts=api.solid_geometry(case["tag"],model)["parts"]
            self.assertEqual(set(case["roles"]),{p.get("role") for p in parts})
            host=next(e for e in model["elements"] if e["tag"]==case["host"])
            aperture=next(v for v in host["geometry"]["voids"] if v["tag"]==case["void"])["outline"]
            self.assertLess(Polygon(aperture).symmetric_difference(box(*case["clear_bounds"])).area,1e-10)
            space=next(s for s in model["spaces"] if s["tag"]==case["space"])
            self.assertTrue(Polygon(space["outline"]).contains(Polygon(aperture)))
            materials={m["tag"] for m in model["materials"]}
            for part in parts:
                self.assertIn(part["material"],materials)
                self.assertAlmostEqual(api._occupied_volume([part]),ConvexHull(api.part_vertices(part)).volume,places=8)
            concrete=[p for p in parts if p["role"]=="outer_concrete"]
            self.assertAlmostEqual(max(p["z1"] for p in concrete),case["outer_wall_top"])
            self.assertAlmostEqual(min(p["z0"] for p in concrete),case["structural_top"])
            upstands=[p for p in parts if p["role"]=="inner_upstand"]
            self.assertTrue(all(abs(p["z1"]-p["z0"]-case["inner_upstand"])<1e-10 for p in upstands))
            glazing=[p for p in parts if p["role"]=="glazing"]
            self.assertEqual(len(glazing),1)
            self.assertIn("section",glazing[0])
            section=glazing[0]["section"]["profile"]
            self.assertNotAlmostEqual(section[0][1],section[1][1])
            self.assertAlmostEqual(section[3][1]-section[0][1],case["glass_thickness"])
            for i,left in enumerate(parts):
                for right in parts[i+1:]:
                    levels=sorted({left["z0"],left["z1"],right["z0"],right["z1"]})
                    for lo,hi in zip(levels,levels[1:]):
                        z=(lo+hi)/2
                        if not(max(left["z0"],right["z0"])<z<min(left["z1"],right["z1"])):continue
                        a,b=api.part_outline_at(left,z),api.part_outline_at(right,z)
                        if len(a)>2 and len(b)>2:
                            self.assertLessEqual(Polygon(a).intersection(Polygon(b)).area,case["max_pair_overlap"])
            ventilation=LineString([(3.225,10.78),(3.225,10.95)])
            for part in parts:
                if part["z0"]<6.86<part["z1"]:
                    ring=api.part_outline_at(part,6.86)
                    if len(ring)>2:self.assertLess(Polygon(ring).intersection(ventilation).length,1e-10)
            slab_parts=api._slab_geometry(model,host)["parts"]
            for part in parts:
                for slab in slab_parts:
                    lo,hi=max(part["z0"],slab["z0"]),min(part["z1"],slab["z1"])
                    if lo>=hi:continue
                    ring=api.part_outline_at(part,(lo+hi)/2)
                    if len(ring)>2:self.assertLess(Polygon(ring).intersection(Polygon(slab["outline"],slab.get("holes",[]))).area,1e-10)
            self.assertEqual(obj["historical_evidence"]["status"],"inferred_reconstruction")
            for kind in ("plan","section","elevation","3d","body"):
                self.assertTrue(api.represent(case["tag"],kind,model)["primitives"])

    def test_actual_production_material_and_plan_callbacks(self):
        import ast
        import numpy as np
        from types import SimpleNamespace
        root=Path(__file__).parents[2]/"🎨️visuals/🏛️savoye"
        def definitions(file,names,scope,methods=False):
            tree=ast.parse(file.read_text(encoding="utf-8"))
            nodes=[n for cls in tree.body if isinstance(cls,ast.ClassDef) for n in cls.body] if methods else tree.body
            nodes=[n for n in nodes if isinstance(n,ast.FunctionDef) and n.name in names]
            exec(compile(ast.Module(body=nodes,type_ignores=[]),str(file),"exec"),scope)
        draft=SimpleNamespace(GLASS="glass",CONCRETE="concrete",INK="ink",GOOD="garden",MUTED="metal",mix=lambda *a:"paving")
        core={"_d":draft,"_model":api,"np":np}
        definitions(root/"🧱️core/📐️geometry.py",{"surface_role","part_faces"},core)
        definitions(root/"🧱️core/🐍️.py",{"surface_color"},core)
        shared=SimpleNamespace(**core)
        physical=api.solid_geometry("ROOFLIGHT-1",api.load_model())
        for part in physical["parts"]:
            expected="glass" if part["material"]=="MAT-GLASS" else "metal" if part["material"]=="MAT-METAL" else "concrete"
            self.assertEqual(shared.surface_role(physical,part),expected)
            faces=shared.part_faces(part)
            self.assertTrue(faces)
            area=sum(np.linalg.norm(np.cross(np.asarray(vertices)[i]-vertices[0],np.asarray(vertices)[i+1]-vertices[0]))/2 for vertices,normal in faces for i in range(1,len(vertices)-1))
            expected_area=ConvexHull(api.part_vertices(part)).area-(0 if "section" in part else Polygon(part["outline"]).area)
            self.assertAlmostEqual(area,expected_area,places=8)
        marks=[]
        class Group:
            def __init__(self,*args):self.submobjects=list(args)
            def add(self,*args):self.submobjects.extend(args);return self
        def polygon(ring,fill,*args,**kwargs):
            marks.append((fill,Polygon(ring)))
            return Group()
        scope={"_c":shared,"_model":api,"np":np,"_engine":SimpleNamespace(VGroup=Group),"_d":SimpleNamespace(polygon=polygon,polyline=lambda *a,**k:Group()),"INK":"ink","W_HAIR":1,"W_THIN":1,"EPS":1e-9}
        definitions(root/"📋️plan/🐍️.py",{"part_plan_outline","plan_part_visible","plan_part_rank"},scope)
        definitions(root/"📋️plan/🐍️.py",{"_plan_solids"},scope,True)
        item={"tag":"ROOFLIGHT-1"}
        view=SimpleNamespace(_pair=lambda look,item,fill,stroke:(fill,stroke,1,0))
        ctx={"bands":{"roof":(6.6,9.4)},"level":"roof","g":{"details":[physical]},"use":{"architecture"},"catalogue":[item],"look":SimpleNamespace(state=lambda item:"normal"),"cut_z":6.87,"below":False,"screen":lambda p:p,"put":lambda *a:None}
        scope["_plan_solids"](view,ctx)
        self.assertTrue(marks)
        self.assertIn("glass",{fill for fill,shape in marks})
        self.assertIn("concrete",{fill for fill,shape in marks})
        self.assertTrue(all(shape.is_valid and shape.area>0 for fill,shape in marks))
        sec={"_c":shared,"EPS":1e-9}
        definitions(root/"✂️section/🐍️.py",{"part_projection"},sec)
        glass=next(p for p in physical["parts"] if p["role"]=="glazing")
        cut=sec["part_projection"](glass,SimpleNamespace(value=3.225,index=0),"cut")
        self.assertLess(Polygon(cut[0]).symmetric_difference(Polygon(glass["section"]["profile"])).area,1e-10)

    def test_mixed_material_inventory_and_rooflight_mass(self):
        from copy import deepcopy
        model=api.load_model()
        for case in json.loads(Path(__file__).with_suffix(".json").read_text(encoding="utf-8"))["material_cases"]:
            sample=deepcopy(model)
            obj=next(e for e in sample["elements"] if e["tag"]=="ROOFLIGHT-1")
            obj["material"]=case["fallback_material"]
            obj["geometry"]={"kind":"mixed-material-test","base_level":"ground","parts":case["parts"]}
            quantity=api.quantities(obj["tag"],sample)
            self.assertEqual({m["material"]:m["volume"] for m in quantity["material_quantities"]},case["material_volumes"])
            self.assertAlmostEqual(quantity["weight"],case["weight_kn"])
            self.assertAlmostEqual(quantity["mass"],case["mass_kg"],places=6)
            for m in quantity["material_quantities"]:
                density=next(mat["density"] for mat in sample["materials"] if mat["tag"]==m["material"])
                self.assertEqual(m["unit_weight"],density)
        parts=api.solid_geometry("ROOFLIGHT-1",model)["parts"]
        expected={}
        for part in parts:
            volume=ConvexHull(api.part_vertices(part)).volume
            expected[part["material"]]=expected.get(part["material"],0)+volume
        quantity=api.quantities("ROOFLIGHT-1",model)
        self.assertEqual(set(expected),{m["material"] for m in quantity["material_quantities"]})
        weight=0
        for entry in quantity["material_quantities"]:
            self.assertAlmostEqual(entry["volume"],expected[entry["material"]],places=8)
            weight+=expected[entry["material"]]*entry["unit_weight"]["value"]
        self.assertAlmostEqual(quantity["weight"],weight,places=8)
        self.assertAlmostEqual(quantity["mass"],weight*1000/9.80665,places=6)

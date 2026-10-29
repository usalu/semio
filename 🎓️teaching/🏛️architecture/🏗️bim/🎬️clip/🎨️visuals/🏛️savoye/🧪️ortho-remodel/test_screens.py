"""🌀 Production solarium projection recorders preserve physical apertures and sloping caps."""
import ast
import importlib
import json
from pathlib import Path
from types import SimpleNamespace
import unittest
import numpy as np
from shapely.geometry import Polygon
from shapely.ops import unary_union

h=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.🧪️ortho-remodel.test_frustums")
CASE=json.loads(Path(__file__).with_name("screens.json").read_text(encoding="utf-8"))

class ScreenDrawingContract(unittest.TestCase):
    @classmethod
    def setUpClass(cls):
        cls.model=h.MODEL.load_model()
        cls.screen=h.MODEL.path_wall_geometry(cls.model,h.MODEL.element(CASE["source"],cls.model))
        cls.item={"tag":cls.screen["tag"],"kind":"screens","data":cls.screen}

    def test_plan_provider_records_native_aperture_slices(self):
        scope=h._scope()
        scope["_s"].poche=lambda rings,*a,**k:h._polygon(rings[0])
        scope["WALL_FILL"]=0
        scene=scope["Plan"]()
        scene._pair=lambda *a:(0,0,1,0)
        for cut in CASE["plan_cuts"]:
            recorded=[]
            ctx={"view":lambda x,y,z:np.array([x,y,z]),"look":None,"w":1,"catalogue":[self.item],"cut_z":cut,"bands":{"roof":(6.6,9.325)},"level":"roof","put":lambda *a:recorded.append(a[-1])}
            scene._plan_screens(ctx)
            actual=unary_union([p for group in recorded for p in h._polygons(group)])
            expected=unary_union([Polygon(h.MODEL.part_outline_at(p,cut)) for p in self.screen["parts"] if p["z0"]<=cut<p["z1"]])
            self.assertLess(actual.symmetric_difference(expected).area,1e-9)
            if 7.25<cut<8.475:
                aperture=unary_union([Polygon(p["outline"]) for p in self.screen["openings"][0]["parts"]])
                self.assertLess(actual.intersection(aperture).area,1e-9)

    def test_section_provider_records_material_below_and_above_window(self):
        scope=h._scope()
        scope["WALL_FILL"]=0
        scene=scope["Section"]()
        scene._pair=lambda *a:(0,0,1,0)
        scene._cut_poche=lambda points,*a,**k:h._polygon(points)
        opening=self.screen["openings"][0]
        value=(opening["p0"][0]+opening["p1"][0])/2
        plane=SimpleNamespace(index=0,value=value,extent=lambda ring:(min(p[0] for p in ring),max(p[0] for p in ring)))
        plane.classify=lambda lo,hi:"cut" if lo<=value<=hi else "removed"
        plane.intervals=lambda rings:scope["profile_cut"](rings[0],0,value)
        plane.hrange=lambda ring:(min(p[1] for p in ring),max(p[1] for p in ring))
        recorded=[]
        ctx={"g":{},"catalogue":[self.item],"plane":plane,"look":None,"to":lambda h,z:np.array([h,z,0]),"put":lambda *a:recorded.append(a[-1])}
        scene._sec_screens(ctx)
        actual=unary_union([p for group in recorded for p in h._polygons(group)])
        expected=unary_union([Polygon(h.MODEL.part_section(p,value,0)) for p in self.screen["parts"] if h.MODEL.part_section(p,value,0)])
        self.assertLess(actual.symmetric_difference(expected).area,1e-9)
        aperture=Polygon([[2,7.25],[2.5,7.25],[2.5,8.475],[2,8.475]])
        self.assertLess(actual.intersection(aperture).area,1e-9)
        self.assertGreater(actual.intersection(Polygon([[2,6.6],[2.5,6.6],[2.5,7.19],[2,7.19]])).area,.05)

    def test_axon_provider_uses_same_native_parts(self):
        scope=h._scope()
        tree=ast.parse((h.ROOT/"🧊️axon/🧩️primitives.py").read_text(encoding="utf-8"))
        build=next(n for n in tree.body if isinstance(n,ast.FunctionDef) and n.name=="_build")
        loop=next(n for n in build.body if isinstance(n,ast.For) and isinstance(n.target,ast.Name) and n.target.id=="screen")
        fn=ast.FunctionDef(name="screen_prims",args=ast.arguments(posonlyargs=[],args=[ast.arg(arg="g"),ast.arg(arg="model")],kwonlyargs=[],kw_defaults=[],defaults=[]),body=[ast.Assign(targets=[ast.Name(id="out",ctx=ast.Store())],value=ast.List(elts=[],ctx=ast.Load())),loop,ast.Return(value=ast.Name(id="out",ctx=ast.Load()))],decorator_list=[])
        scope["_item"]=lambda *a:{}
        module=ast.fix_missing_locations(ast.Module(body=[fn],type_ignores=[]))
        exec(compile(module,"screen-provider","exec"),scope)
        prims=scope["screen_prims"]({"screens":[self.screen]},self.model)
        self.assertEqual(len(prims),len(self.screen["parts"]))
        actual=[]
        for prim in prims:
            for kind,rings,normal,role in prim.faces:
                actual.extend(tuple(p) for p in rings[0])
        expected=[tuple([*p,z]) for part in self.screen["parts"] for z in (part["z0"],part["z1"]) for p in h.MODEL.part_outline_at(part,z)]
        self.assertEqual(set(actual),set(expected))

    def test_solarium_detail_records_shelf_head_and_sloping_cap(self):
        scope=h._scope()
        beams=[h.MODEL._solid_geometry(self.model,h.MODEL._find(self.model,tag)) for tag in ("SC-SILL","SC-HEAD")]
        scope["_c"].geometry=lambda ignored:{"screens":[self.screen],"beams":beams}
        scope["_d"].View=lambda *a,**k:lambda x,y,z:np.array([y,z,0])
        scene=scope["Detail"]()
        drawing=scene.solarium_detail(model=self.model,dims=False)
        opening=self.screen["openings"][0]
        cut=(opening["p0"][0]+opening["p1"][0])/2
        for tag,parts in [(self.screen["tag"],self.screen["parts"])]+[(b["tag"],b["parts"]) for b in beams]:
            actual=unary_union(list(h._polygons(drawing.parts[tag])))
            rings=[]
            for p in parts:
                if "top_outline" in p:
                    ring=h.MODEL.part_section(p,cut,0)
                    if ring:rings.append(Polygon(ring))
                else:
                    rings.extend(Polygon([[lo,p["z0"]],[hi,p["z0"]],[hi,p["z1"]],[lo,p["z1"]]]) for lo,hi in scope["profile_cut"](p["outline"],0,cut))
            expected=unary_union(rings)
            self.assertLess(actual.symmetric_difference(expected).area,1e-9)

    def test_complete_elevation_provider_projects_aperture_wall_and_cap(self):
        from shapely.geometry import MultiPoint
        aperture=self.screen["openings"][0]
        selected=[p for p in self.screen["parts"] if aperture["start"]-1e-9<=p["start"] and p["end"]<=aperture["end"]+1e-9]
        item={**self.item,"data":{**self.screen,"parts":selected}}
        scope=h._scope()
        scene=scope["Section"]()
        scene._pair=lambda *a:(0,0,1,0)
        scope["WALL_FILL"]=0
        class View:
            def __init__(self,*a,face="south",**k):self.face=face
            def __call__(self,x,y,z):return np.array([x if self.face in ("south","north") else y,z,0])
            def facing(self,normal):return np.dot(normal,{"south":[0,-1,0],"north":[0,1,0],"east":[1,0,0],"west":[-1,0,0]}[self.face])
            def depth(self,x,y,z):return -y if self.face=="south" else y if self.face=="north" else x if self.face=="east" else -x
        scope["_d"].View=View
        scope["P"]=lambda p:np.asarray(p,float)
        scope["_s"].Look=lambda *a:SimpleNamespace(state=lambda item:"normal")
        scope["_s"].Sheet=lambda *a:h.Shape()
        scope["_s"].expand_names=lambda names,g:names
        g={"box":{"length":19,"width":21.5,"height":9.325},"storeys":{"roof":6.6},"slabs":[]}
        scope["_c"].geometry=lambda ignored:g
        scope["_s"].items=lambda ignored:[item]
        for face in ("south","north","east","west"):
            drawing=scene.villa_elevation(face,scale=1,centre=(0,0),model=g,ground=False,levels=False,layers=("architecture",))
            actual=unary_union([p for p in h._polygons(drawing.parts[self.screen["tag"]]) if p.area>1e-12])
            axis=0 if face in ("south","north") else 1
            expected=unary_union([MultiPoint([[v[axis],p["z0"]] for v in p["outline"]]+[[v[axis],p["z1"]] for v in p.get("top_outline",p["outline"])]).convex_hull for p in selected])
            self.assertLess(actual.symmetric_difference(expected).area,1e-8)

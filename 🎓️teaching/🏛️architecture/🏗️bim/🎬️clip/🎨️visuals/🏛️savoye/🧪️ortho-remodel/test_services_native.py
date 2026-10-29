"""🚰 Production service drawing recorders preserve native hollow and mitred material."""
from copy import deepcopy
import importlib
import json
from pathlib import Path
from types import SimpleNamespace
import unittest
import numpy as np
from scipy.spatial import ConvexHull
from shapely.geometry import Polygon,MultiPoint
from shapely.ops import unary_union

h=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.🧪️ortho-remodel.test_frustums")
CASE=json.loads(Path(__file__).with_name("services.json").read_text(encoding="utf-8"))

class ServiceDrawingContract(unittest.TestCase):
    def cases(self):
        model=h.MODEL.load_model()
        prototype=h.MODEL.element("DP-1",model)
        for case in CASE["cases"]:
            source=deepcopy(prototype)
            source.update(tag="TEST-SERVICE-DRAWING",guid=h.MODEL.global_id("TEST-SERVICE-DRAWING"),material="MAT-METAL")
            source["geometry"]={"points":case["points"],"tube":{"wall_thickness":{"value":.02,"unit":"m","basis":"inferred"},"segments":CASE["segments"]}}
            if "clip_start" in case:source["geometry"]["tube"]["clip_start"]=case["clip_start"]
            source["attributes"]={"diameter":{"value":.2,"unit":"m","basis":"inferred"}}
            model["elements"]=[source]
            yield case,h.MODEL.service_geometry(source["tag"],model)

    def test_plan_provider_preserves_actual_hollow_horizontal_slices(self):
        scope=h._scope()
        scene=scope["Plan"]()
        scene._pair=lambda *a:(0,0,1,0)
        for case,data in self.cases():
            recorded=[]
            ctx={"look":None,"bands":{"ground":(-1,5)},"level":"ground","window":None,"catalogue":[{"kind":"services","data":data,"tag":data["tag"]}],"cut_z":case["cut_z"],"screen":lambda ring:np.c_[ring,np.zeros(len(ring))],"put":lambda *a:recorded.append(a[-1])}
            scene._plan_services(ctx)
            actual=unary_union([p for group in recorded for p in h._polygons(group) if p.area>1e-12])
            expected=unary_union([Polygon(ring) for p in data["parts"] if (ring:=h.MODEL.part_outline_at(p,case["cut_z"]))])
            self.assertLess(actual.symmetric_difference(expected).area,1e-9)

    def test_section_provider_keeps_annular_air_and_inclined_end_planes(self):
        scope=h._scope()
        scene=scope["Section"]()
        scene._pair=lambda *a:(0,0,1,0)
        for case,data in self.cases():
            axis,value=case["axis"],case["value"]
            plane=SimpleNamespace(index=axis,value=value,extent=lambda ring:(min(p[axis] for p in ring),max(p[axis] for p in ring)))
            plane.classify=lambda lo,hi:"cut" if lo<=value<=hi else "removed"
            plane.intervals=lambda rings:scope["profile_cut"](rings[0],axis,value)
            plane.hrange=lambda ring:(min(p[1-axis] for p in ring),max(p[1-axis] for p in ring))
            recorded=[]
            ctx={"plane":plane,"to":lambda h,z:np.array([h,z,0]),"look":None,"catalogue":[{"kind":"services","data":data,"tag":data["tag"]}],"put":lambda *a:recorded.append(a[-1])}
            scene._sec_services(ctx)
            actual=unary_union([p for group in recorded for p in h._polygons(group) if p.area>1e-12])
            expected=unary_union([Polygon(ring) for p in data["parts"] for ring in h.MODEL.part_sections(p,value,axis)])
            self.assertLess(actual.symmetric_difference(expected).area,1e-9)

    def test_axon_provider_emits_exact_material_without_axis_padding(self):
        scope=h._scope()
        for case,data in self.cases():
            prims=scope["_service_prims"](data,{},0,3.14)
            volume=0
            for prim in prims:
                vertices=np.array([v for kind,rings,normal,role in prim.faces for ring in rings for v in ring])
                volume+=ConvexHull(vertices).volume
            expected=sum(ConvexHull(np.asarray(h.MODEL.part_vertices(p))).volume for p in data["parts"])
            self.assertAlmostEqual(volume,expected,places=9)

    def test_complete_service_elevation_uses_physical_faces(self):
        scope=h._scope()
        scene=scope["Section"]()
        scene._pair=lambda *a:(0,0,1,0)
        class View:
            def __init__(self,*a,face="south",**k):self.face=face
            def __call__(self,x,y,z):return np.array([x if self.face in ("south","north") else y,z,0])
            def facing(self,n):return np.dot(n,{"south":[0,-1,0],"north":[0,1,0],"east":[1,0,0],"west":[-1,0,0]}[self.face])
            def depth(self,x,y,z):return -y if self.face=="south" else y if self.face=="north" else x if self.face=="east" else -x
        scope["_d"].View=View
        scope["P"]=lambda p:np.asarray(p,float)
        scope["_s"].Look=lambda *a:SimpleNamespace(state=lambda item:"normal")
        scope["_s"].Sheet=lambda *a:h.Shape()
        scope["_s"].expand_names=lambda names,g:names
        for case,data in self.cases():
            g={"box":{"length":5,"width":5,"height":5},"storeys":{"ground":0},"slabs":[]}
            scope["_c"].geometry=lambda ignored:g
            scope["_s"].items=lambda ignored:[{"kind":"services","data":data,"tag":data["tag"]}]
            for face in ("south","north","east","west"):
                drawing=scene.villa_elevation(face,model=g,layers=("services",),ground=False,levels=False)
                actual=unary_union([p for p in h._polygons(drawing.parts[data["tag"]]) if p.area>1e-12])
                axis=0 if face in ("south","north") else 1
                expected=unary_union([MultiPoint(np.array(h.MODEL.part_vertices(p))[:,[axis,2]]).convex_hull for p in data["parts"]])
                self.assertLess(actual.symmetric_difference(expected).area,1e-9)

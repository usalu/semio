"""🔺 Production tapered-solid drawing consumers checked by NumPy, GEOS and SciPy hulls."""
import ast
import importlib
import json
from pathlib import Path
from types import SimpleNamespace
import unittest

import numpy as np
from scipy.spatial import ConvexHull
from shapely.geometry import MultiPoint,Polygon
from shapely.ops import unary_union

ROOT = Path(__file__).resolve().parent.parent
FIXTURE = json.loads((ROOT/"🧪️ortho-remodel/frustums.json").read_text(encoding="utf-8"))
MODEL = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")


class Shape:
    """📝 Record actual production-emitted vertices through the drawing interface."""
    def __init__(self,*members):
        self.submobjects=list(members)
        self.parts={}
        self.anchors={}
    def add(self,*members):
        self.submobjects.extend(members)
        return self
    def part(self,tag,shape,anchor=None):
        self.parts[tag]=shape
    def register(self,tag,shape):
        self.parts[tag]=shape
        self.add(shape)
    def alias(self,*args):
        return self


def _polygon(points,*args,**kwargs):
    shape=Shape()
    shape.vertices=np.asarray(points,float)
    shape.polygon=True
    return shape


def _line(points,*args,**kwargs):
    shape=_polygon(points)
    shape.polygon=False
    return shape


def _polygons(shape):
    if getattr(shape,"polygon",False):
        yield Polygon(shape.vertices[:,:2])
    for child in shape.submobjects:
        yield from _polygons(child)


def _definitions(path,names=None):
    tree=ast.parse(path.read_text(encoding="utf-8"))
    return [node for node in tree.body if isinstance(node,(ast.FunctionDef,ast.ClassDef)) and (names is None or node.name in names)]


def _scope():
    engine=SimpleNamespace(VGroup=Shape,RIGHT=np.array([1,0,0]),UP=np.array([0,1,0]))
    drafting=SimpleNamespace(polygon=_polygon,polyline=_line,segments=lambda *a,**k:Shape(),hatch=lambda *a,**k:Shape(),mix=lambda *a:0,P=lambda p:np.asarray(p,float),
                              CONCRETE=0,STRUCT=0,DATA=0,INK=0,GLASS=0,S_LABEL=1,W_EDGE=1,W_THIN=1,Drawing=lambda view:Shape())
    shared=SimpleNamespace(_model=MODEL,inside=lambda *a:True,surface_color=lambda *a:0,surface_role=lambda *a:"concrete")
    scope={"np":np,"EPS":1e-9,"_engine":engine,"_d":drafting,"_c":shared,"_model":MODEL,"_s":SimpleNamespace(SheetMixin=object,pseudo=lambda tag,*a:{"tag":tag}),
           "INK":0,"MUTED":0,"GOOD":0,"GLASS":0,"BACKGROUND":0,"CONCRETE":0,"SERV":0,"BEYOND":0,"W_CUT":1,"W_EDGE":1,"W_THIN":1,"W_HAIR":1,"UP":np.array([0,0,1]),"STAGE_TOP":6,"STAGE_UNDER":1,"STAGE_FIRST":3}
    core={"np":np,"_model":MODEL}
    nodes=_definitions(ROOT/"🧱️core/📐️geometry.py",{"part_faces"})
    exec(compile(ast.Module(body=nodes,type_ignores=[]),str(ROOT),"exec"),core)
    if "part_faces" in core:
        shared.part_faces=core["part_faces"]
    nodes=_definitions(ROOT/"📋️plan/🐍️.py")
    exec(compile(ast.Module(body=nodes,type_ignores=[]),str(ROOT),"exec"),scope)
    scope["_plan"]=SimpleNamespace(**{key:value for key,value in scope.items() if callable(value)})
    nodes=_definitions(ROOT/"✂️section/🐍️.py")
    exec(compile(ast.Module(body=nodes,type_ignores=[]),str(ROOT),"exec"),scope)
    nodes=_definitions(ROOT/"🔩️detail/🐍️.py")
    exec(compile(ast.Module(body=nodes,type_ignores=[]),str(ROOT),"exec"),scope)
    nodes=_definitions(ROOT/"🧊️axon/🧩️primitives.py",{"Prim","_face","_poly","_ring3","_prism","ring_ccw","_part_faces","_service_prims"})
    nodes+=_definitions(ROOT/"🧊️axon/🐍️.py",{"_lift"})
    scope["_measure"]=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.📐️drafting.📏️geometry")
    exec(compile(ast.Module(body=nodes,type_ignores=[]),str(ROOT),"exec"),scope)
    return scope


def _vertices(part):
    return np.array([[*p,part["z0"]] for p in part["outline"]]+[[*p,part["z1"]] for p in part["top_outline"]],float)


def _section(part,axis,value):
    points=_vertices(part)
    hull=ConvexHull(points)
    hits=[]
    for triangle in hull.simplices:
        for i,j in zip(triangle,np.roll(triangle,-1)):
            a,b=points[i],points[j]
            if abs(a[axis]-value)<1e-10:
                hits.append([a[1-axis],a[2]])
            if (a[axis]-value)*(b[axis]-value)<0:
                p=a+(b-a)*(value-a[axis])/(b[axis]-a[axis])
                hits.append([p[1-axis],p[2]])
    return MultiPoint(hits).convex_hull if hits else Polygon()


class FrustumDrawingContract(unittest.TestCase):
    """🏗️ Every projection preserves tapered geometry rather than extruding the lower outline."""
    def test_plan_records_actual_cut_and_visible_caps(self):
        scope=_scope()
        scene=scope["Plan"]()
        scene._pair=lambda *a:(0,0,1,0)
        part=FIXTURE["parts"][0]
        data={"tag":"PAD","guid":"pad","class":"IfcFooting","domains":["structure"],"kind":"pad","storey":"basement","parts":[part]}
        for cut in (-3,-1.5,0):
            recorded=[]
            ctx={"bands":{"basement":(-4,2)},"level":"basement","g":{"foundations":[data]},"catalogue":[],"look":None,"use":{"structure"},"below":True,"cut_z":cut,
                 "screen":lambda ring:np.c_[ring,np.zeros(len(ring))],"put":lambda rank,tag,group:recorded.append(group)}
            scene._plan_solids(ctx)
            actual=unary_union([polygon for group in recorded for polygon in _polygons(group)])
            t=np.clip((cut-part["z0"])/(part["z1"]-part["z0"]),0,1)
            expected=Polygon((1-t)*np.array(part["outline"])+t*np.array(part["top_outline"]))
            self.assertLess(actual.symmetric_difference(expected).area,1e-10)

    def test_section_records_real_inclined_cut_and_partial_height(self):
        scope=_scope()
        scene=scope["Section"]()
        scene._pair=lambda *a:(0,0,1,0)
        scene._cut_poche=lambda points,*a,**k:_polygon(points)
        for part in FIXTURE["parts"]:
            for case in FIXTURE["cuts"]:
                axis,value=case["axis"],case["value"]
                recorded=[]
                plane=SimpleNamespace(index=axis,value=value,extent=lambda ring:(min(p[axis] for p in ring),max(p[axis] for p in ring)),classify=lambda *a:"cut")
                plane.intervals=lambda rings:scope["profile_cut"](rings[0],axis,value)
                data={"tag":"PAD","guid":"pad","class":"IfcFooting","domains":["structure"],"kind":"pad","parts":[part]}
                ctx={"g":{"foundations":[data]},"catalogue":[],"plane":plane,"look":None,"use":{"structure"},"to":lambda h,z:np.array([h,z,0]),"put":lambda *a:recorded.append(a[-1])}
                scene._sec_solids(ctx)
                actual=unary_union([polygon for group in recorded for polygon in _polygons(group)])
                self.assertLess(actual.symmetric_difference(_section(part,axis,value)).area,1e-9)

    def test_axon_side_faces_match_scipy_hull_planes_and_actual_top(self):
        scope=_scope()
        self.assertTrue(callable(scope.get("_part_faces")))
        for part in FIXTURE["parts"]:
            faces=scope["_part_faces"](part,"foot")
            hull=ConvexHull(_vertices(part))
            top=[]
            area=0.
            for kind,rings,normal,role in faces:
                self.assertEqual((kind,role),("poly","foot"))
                ring=np.asarray(rings[0])
                self.assertAlmostEqual(float(np.linalg.norm(normal)),1.)
                self.assertTrue(any(np.max(np.abs(ring@eq[:3]+eq[3]))<1e-9 for eq in hull.equations))
                self.assertTrue(np.any(np.linalg.norm(hull.equations[:,:3]-normal,axis=1)<1e-9))
                area+=sum(np.linalg.norm(np.cross(ring[i]-ring[0],ring[i+1]-ring[0]))/2 for i in range(1,len(ring)-1))
                if normal[2]>.999:
                    top.append(Polygon(ring[:,:2]))
            self.assertLess(unary_union(top).symmetric_difference(Polygon(part["top_outline"])).area,1e-10)
            self.assertAlmostEqual(area,hull.area-Polygon(part["outline"]).area,places=8)

    def test_elevation_painter_records_inclined_world_vertices(self):
        scope=_scope()
        scene=scope["Section"]()
        scene._pair=lambda *a:(0,0,1,0)
        part=FIXTURE["parts"][0]
        vertices=np.array([[*part["outline"][0],part["z0"]],[*part["outline"][1],part["z0"]],[*part["top_outline"][1],part["z1"]],[*part["top_outline"][0],part["z1"]]])
        face={"vertices":vertices,"facing":1.,"item":{},"kind":"solid","fill":0,"stroke":0}
        view=lambda x,y,z:np.array([x,z,0])
        group=scene._elev_face(face,None,view,SimpleNamespace(state=lambda item:"normal"),1)
        actual=unary_union(list(_polygons(group)))
        self.assertLess(actual.symmetric_difference(Polygon(vertices[:,[0,2]])).area,1e-10)

    def test_complete_elevation_provider_uses_actual_tapered_faces(self):
        scope=_scope()
        scene=scope["Section"]()
        scene._pair=lambda *a:(0,0,1,0)
        class View:
            def __init__(self,*a,face="south",**k):
                self.face=face
            def __call__(self,x,y,z):
                return np.array([x if self.face in ("south","north") else y,z,0])
            def facing(self,normal):
                return np.dot(normal,{"south":[0,-1,0],"north":[0,1,0],"east":[1,0,0],"west":[-1,0,0]}[self.face])
            def depth(self,x,y,z):
                return -y if self.face=="south" else y if self.face=="north" else x if self.face=="east" else -x
        scope["_d"].View=View
        scope["P"]=lambda p:np.asarray(p,float)
        look=SimpleNamespace(state=lambda item:"normal")
        scope["_s"].Look=lambda *a:look
        scope["_s"].Sheet=lambda *a:Shape()
        scope["_s"].expand_names=lambda names,g:names
        for part in FIXTURE["parts"]:
            data={"tag":"PAD","guid":"pad","class":"IfcFooting","domains":["structure"],"kind":"pad","parts":[part]}
            g={"box":{"length":4,"width":3,"height":1},"storeys":{"basement":-3},"slabs":[],"foundations":[data]}
            scope["_c"].geometry=lambda ignored:g
            scope["_s"].items=lambda ignored:[{"tag":"PAD","kind":"foundations","data":data}]
            for face in ("south","north","east","west"):
                drawing=scene.villa_elevation(face,scale=1,centre=(0,0),model=g,ground=False,levels=False,layers=("structure",))
                actual=unary_union(list(_polygons(drawing.parts["PAD"])))
                axis=0 if face in ("south","north") else 1
                expected=MultiPoint(_vertices(part)[:,[axis,2]]).convex_hull
                self.assertLess(actual.symmetric_difference(expected).area,1e-9)

    def test_detail_reveal_preserves_actual_slope_and_section_station(self):
        scope=_scope()
        part=FIXTURE["parts"][0]
        column={"tag":"PAD-PROFILE","x":0,"y":0,"base":part["z0"],"top":part["z1"],"outline":part["outline"],"parts":[part]}
        g={"pilotis":[column]}
        scope["_c"].geometry=lambda ignored:g
        scope["_d"].View=lambda *a,**k:lambda x,y,z:np.array([x,z,0])
        scene=scope["Detail"]()
        scene.text=lambda *a,**k:Shape()
        drawing=scene.piloti_detail(model=g,tag=column["tag"],dims=False,grow=.5)
        actual=unary_union(list(_polygons(drawing.parts["elevation"])))
        top=(np.array(part["outline"])+np.array(part["top_outline"]))/2
        vertices=np.array([[*p,part["z0"]] for p in part["outline"]]+[[*p,-1.5] for p in top])
        self.assertLess(actual.symmetric_difference(MultiPoint(vertices[:,[0,2]]).convex_hull).area,1e-9)
        expected=np.ptp(.75*np.array(part["outline"])+.25*np.array(part["top_outline"]),axis=0)*.62
        np.testing.assert_allclose(np.ptp(drawing.parts["section"].vertices,axis=0)[:2],expected,rtol=0,atol=1e-12)

    def test_live_extrusion_recomputes_inclined_normals(self):
        scope=_scope()
        part=FIXTURE["parts"][0]
        prim=scope["Prim"]("PAD",{},0,(0,0),scope["_part_faces"](part,"foot"))
        moved=scope["_lift"](prim,part["z0"],.5)
        vertices=_vertices(part)
        vertices[:,2]=part["z0"]+(vertices[:,2]-part["z0"])*.5
        hull=ConvexHull(vertices)
        for face in moved.faces:
            self.assertTrue(np.any(np.linalg.norm(hull.equations[:,:3]-face[2],axis=1)<1e-9))

    def test_architectural_ground_plan_hides_buried_foundations(self):
        scope=_scope()
        scene=scope["Plan"]()
        scene._pair=lambda *a:(0,0,1,0)
        part=FIXTURE["parts"][0]
        data={"tag":"PAD","guid":"pad","class":"IfcFooting","domains":["structure"],"kind":"pad","storey":"ground","parts":[part]}
        for use,lit,visible in (({"architecture","structure"},False,False),({"structure"},False,True),({"architecture","structure"},True,True)):
            recorded=[]
            ctx={"bands":{"ground":(0,4)},"level":"ground","g":{"storeys":{"ground":0},"foundations":[data]},"catalogue":[],"look":SimpleNamespace(state=lambda item:"highlight" if lit else "normal"),
                 "use":use,"below":True,"cut_z":1.2,"screen":lambda ring:np.c_[ring,np.zeros(len(ring))],"put":lambda *a:recorded.append(a[-1])}
            scene._plan_solids(ctx)
            self.assertEqual(bool(recorded),visible)

    def test_staged_services_keep_the_true_band_cap_rings(self):
        scope=_scope()
        part=FIXTURE["parts"][0]
        service={"tag":"TAPERED-EQUIPMENT","kind":"pipe","parts":[part]}
        pieces=scope["_service_prims"](service,{},-2.,-1.5)
        self.assertEqual(len(pieces),2)
        for piece in pieces:
            top=[ring for face in piece.faces if face[0]=="poly" and face[2][2]>.999 for ring in face[1]]
            ring=np.asarray(top[0], float)
            t=(ring[0,2]-part["z0"])/(part["z1"]-part["z0"])
            expected=Polygon((1-t)*np.array(part["outline"])+t*np.array(part["top_outline"]))
            self.assertLess(Polygon(ring[:,:2]).symmetric_difference(expected).area,1e-10)


if __name__=="__main__":
    unittest.main()

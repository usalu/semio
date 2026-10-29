"""🪜 Native stair waists survive every physical drawing consumer."""
import ast
from copy import deepcopy
import importlib
import json
from types import SimpleNamespace
import unittest

import numpy as np
from scipy.spatial import ConvexHull
from shapely.geometry import MultiPoint, Polygon
from shapely.ops import unary_union

REC = importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.🧪️ortho-remodel.test_frustums")
FIXTURE = json.loads((REC.ROOT/"🧪️ortho-remodel/stairs.json").read_text(encoding="utf-8"))


def stair():
    """📦 Assemble the portable occupied-part contract without invoking its producer."""
    data = deepcopy(FIXTURE["stair"])
    data["parts"] = data["steps"]+data["landings"]+data["structural_parts"]
    return data


def section(part, axis, value):
    """📏 Independently intersect SciPy hull edges with the requested station."""
    vertices = np.asarray(part["mesh"]["vertices"],float)
    points = []
    projection = [i for i in range(3) if i != axis]
    for triangle in ConvexHull(vertices).simplices:
        for i,j in zip(triangle,np.roll(triangle,-1)):
            a,b = vertices[i],vertices[j]
            if abs(a[axis]-value)<1e-10:
                points.append(a[projection])
            if (a[axis]-value)*(b[axis]-value)<0:
                points.append((a+(b-a)*(value-a[axis])/(b[axis]-a[axis]))[projection])
    return MultiPoint(points).convex_hull


class StairDrawingContract(unittest.TestCase):
    """🏗️ Record emitted geometry and compare occupied material with independent engines."""
    def test_material_roles_preserve_source_tile_and_concrete(self):
        scope = {"_d":SimpleNamespace()}
        nodes = REC._definitions(REC.ROOT/"🧱️core/📐️geometry.py",{"surface_role"})
        exec(compile(ast.Module(body=nodes,type_ignores=[]),str(REC.ROOT),"exec"),scope)
        self.assertEqual(scope["surface_role"]({"kind":"stair"},{"material":"MAT-TILE"}),"tile")
        self.assertEqual(scope["surface_role"]({"kind":"stair"},{"material":"MAT-RC"}),"concrete")

    def test_path_wall_door_catalogue_and_native_plan_section(self):
        door = deepcopy(FIXTURE["path_door"])
        g = {"walls":[],"fillings":[door],"slabs":[],"pilotis":[]}
        core = {"geometry":lambda model:g,"disciplines_of":lambda *a:("architecture",),"KIND_CLASSES":{}}
        nodes = REC._definitions(REC.ROOT/"🧱️core/🐍️.py",{"catalogue"})
        exec(compile(ast.Module(body=nodes,type_ignores=[]),str(REC.ROOT),"exec"),core)
        catalogue = core["catalogue"](None)
        self.assertEqual([item["tag"] for item in catalogue],[door["tag"]])
        linear = {**door,"tag":"D-LINE","guid":"door-line","host":"W-LINE"}
        g["walls"] = [{"tag":"W-LINE","guid":"wall-line","openings":[{"tag":"OP-LINE","guid":"opening-line","fill_tag":linear["tag"],"fill_guid":linear["guid"],"fill_class":"IfcDoor"}]}]
        g["fillings"].append(linear)
        core["KIND_CLASSES"]["walls"] = "IfcWall"
        tags = [item["tag"] for item in core["catalogue"](None)]
        self.assertEqual(tags.count(door["tag"]),1)
        self.assertEqual(tags.count(linear["tag"]),1)
        scope = REC._scope()
        plan = scope["Plan"]()
        plan._pair = lambda *a:(0,0,1,0)
        groups = []
        plan._plan_curtain({"g":g,"view":lambda x,y:np.array([x,y,0]),"look":None,"w":1,"cut_z":1.2,"catalogue":catalogue,"put":lambda *a,**k:groups.append(a[-1])},False)
        actual = unary_union([p for group in groups for p in REC._polygons(group)])
        self.assertLess(actual.symmetric_difference(Polygon(door["parts"][0]["outline"])).area,1e-10)
        section_scene = scope["Section"]()
        section_scene._pair = lambda *a:(0,0,1,0)
        groups = []
        plane = SimpleNamespace(index=1,value=.06,extent=lambda ring:(0,.12),classify=lambda *a:"cut",intervals=lambda rings:[(0,1.2)])
        section_scene._sec_enclosure({"g":g,"catalogue":catalogue,"plane":plane,"to":lambda h,z:np.array([h,z,0]),"look":None,"put":lambda *a,**k:groups.append(a[-1])})
        actual = unary_union([p for group in groups for p in REC._polygons(group)])
        self.assertLess(actual.symmetric_difference(Polygon([[0,0],[1.2,0],[1.2,2.1],[0,2.1]])).area,1e-10)

    def test_plan_includes_waist_cut_between_treads(self):
        scope = REC._scope()
        scene = scope["Plan"]()
        scene._pair = lambda *a:(0,0,1,0)
        data = stair()
        groups = []
        ctx = {"bands":{"ground":(0,3)},"level":"ground","g":{"stairs":[data]},"catalogue":[],"look":None,"use":{"architecture"},"below":False,"cut_z":FIXTURE["plan_cut"],
               "screen":lambda ring:np.c_[ring,np.zeros(len(ring))],"put":lambda *a:groups.append(a[-1])}
        scene._plan_solids(ctx)
        actual = unary_union([p for group in groups for p in REC._polygons(group)])
        expected = unary_union([Polygon(p["outline"]) for p in data["steps"]+data["landings"]]+[section(p,2,FIXTURE["plan_cut"]) for p in data["structural_parts"]])
        self.assertLess(actual.symmetric_difference(expected).area,1e-10)

    def test_section_includes_inclined_waist_material(self):
        scope = REC._scope()
        scene = scope["Section"]()
        scene._pair = lambda *a:(0,0,1,0)
        scene._cut_poche = lambda points,*a,**k:REC._polygon(points)
        data = stair()
        axis,value = FIXTURE["section"]["axis"],FIXTURE["section"]["value"]
        groups = []
        plane = SimpleNamespace(index=axis,value=value,extent=lambda ring:(min(p[axis] for p in ring),max(p[axis] for p in ring)),classify=lambda *a:"cut")
        scene._sec_solids({"g":{"stairs":[data]},"catalogue":[],"plane":plane,"look":None,"use":{"structure"},"to":lambda h,z:np.array([h,z,0]),"put":lambda *a:groups.append(a[-1])})
        actual = unary_union([p for group in groups for p in REC._polygons(group)])
        expected = unary_union([section(p,axis,value) for p in data["structural_parts"]]+[Polygon([[p["outline"][0][0],p["z0"]],[p["outline"][1][0],p["z0"]],[p["outline"][1][0],p["z1"]],[p["outline"][0][0],p["z1"]]]) for p in data["steps"]+data["landings"]])
        self.assertLess(actual.symmetric_difference(expected).area,1e-10)

    def test_axon_collection_preserves_waist_faces_and_finish_parts_once(self):
        scope = REC._scope()
        nodes = REC._definitions(REC.ROOT/"🧊️axon/🧩️primitives.py",{"_build"})
        scope.update({"_item":lambda *a:{},"_slab_prims":lambda *a:[],"_stage":lambda *a:0,"STAGE_CELLAR_SLAB":0,"STAGE_PLANE":1,"STAGE_SLAB":2,"STAGE_ROOF":3})
        scope["_c"].role = lambda name,model:name
        exec(compile(ast.Module(body=nodes,type_ignores=[]),str(REC.ROOT),"exec"),scope)
        data = stair()
        slabs = [{"tag":name,"guid":name,"level":level,"storey":level,"parts":[{"outline":[[0,0],[1,0],[1,1]],"z0":z,"z1":z+.1}]} for name,level,z in (("floor","ground",0),("roof","roof",3))]
        pieces = scope["_build"]({"slabs":slabs,"storeys":{"ground":0,"roof":3},"stairs":[data],"pilotis":[],"walls":[],"fillings":[FIXTURE["path_door"]]},None)
        self.assertEqual(len(pieces),len(data["parts"])+len(FIXTURE["path_door"]["parts"]))
        self.assertEqual(sum(p.tag==FIXTURE["path_door"]["tag"] for p in pieces),len(FIXTURE["path_door"]["parts"]))
        waist = pieces[-1]
        hull = ConvexHull(np.asarray(data["structural_parts"][0]["mesh"]["vertices"]))
        area = 0.
        for kind,rings,normal,role in waist.faces:
            ring = np.asarray(rings[0])
            self.assertTrue(any(np.max(np.abs(ring@eq[:3]+eq[3]))<1e-9 for eq in hull.equations))
            area += sum(np.linalg.norm(np.cross(ring[i]-ring[0],ring[i+1]-ring[0]))/2 for i in range(1,len(ring)-1))
        self.assertAlmostEqual(area,hull.area,places=9)

    def test_elevation_includes_native_waist_silhouette(self):
        scope = REC._scope()
        class View:
            def __init__(self,*a,**k): pass
            def __call__(self,x,y,z): return np.array([x,z,0])
            def facing(self,n): return -n[1]
            def depth(self,x,y,z): return -y
        scope["_d"].View = View
        scope["P"] = lambda p:np.asarray(p,float)
        scope["_s"].Look = lambda *a:SimpleNamespace(state=lambda item:"normal")
        scope["_s"].Sheet = lambda *a:REC.Shape()
        scope["_s"].expand_names = lambda names,g:names
        door = FIXTURE["path_door"]
        scope["_s"].items = lambda model:[{"tag":door["tag"],"kind":"doors","data":door}]
        data = stair()
        g = {"box":{"length":3,"width":1,"height":3},"storeys":{"ground":0},"slabs":[],"stairs":[data],"spaces":[]}
        scope["_c"].geometry = lambda model:g
        scene = scope["Section"]()
        scene._pair = lambda *a:(0,0,1,0)
        for layer in ("architecture","structure"):
            drawing = scene.villa_elevation("south",scale=1,centre=(0,0),model=g,ground=False,levels=False,layers=(layer,))
            actual = unary_union(list(REC._polygons(drawing.parts[data["tag"]])))
            expected = unary_union([section(p,1,.5) for p in data["structural_parts"]]+[Polygon([[p["outline"][0][0],p["z0"]],[p["outline"][1][0],p["z0"]],[p["outline"][1][0],p["z1"]],[p["outline"][0][0],p["z1"]]]) for p in data["steps"]+data["landings"]])
            self.assertLess(actual.symmetric_difference(expected).area,1e-10)
            if layer == "architecture":
                door_area = unary_union(list(REC._polygons(drawing.parts[door["tag"]])))
                self.assertAlmostEqual(door_area.area,1.2*2.1)


if __name__ == "__main__":
    unittest.main()

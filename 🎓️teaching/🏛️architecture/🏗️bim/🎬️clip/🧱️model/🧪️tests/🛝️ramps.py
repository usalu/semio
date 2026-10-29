"""🛝️ Real inclined slab products, assembly ownership and shared projection geometry."""
import ast,importlib,json,unittest
from types import SimpleNamespace
from copy import deepcopy
from pathlib import Path
import numpy as np
from scipy.spatial import ConvexHull,HalfspaceIntersection
from scipy.optimize import linprog
from shapely.geometry import Polygon
api=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
class RampGeometryContract(unittest.TestCase):
    def test_source_products_have_real_shared_geometry_and_all_representations(self):
        fixture=json.loads(Path(__file__).with_suffix(".json").read_text(encoding="utf-8"))["source"]
        model=api.load_model()
        for tag in fixture["parents"]+fixture["flights"]:
            geometry=api.ramp_geometry(tag,model)
            self.assertEqual(geometry["tag"],tag)
            for kind in ("plan","section","elevation","3d","axis","footprint","body"):
                representation=api.represent(tag,kind,model)
                self.assertEqual(representation["guid"],geometry["guid"])
                self.assertIn(representation["derived"]["scope"],("assembly","product"))
                self.assertTrue(representation["primitives"],(tag,kind))
                self.assertTrue(all("owner" in p for p in representation["primitives"]),(tag,kind))
            q=api.quantities(tag,model)
            volume=sum(ConvexHull(api.part_vertices(p)).volume for p in geometry["full_parts"])
            self.assertAlmostEqual(q["volume"],volume,places=8)
            self.assertAlmostEqual(q["volume"],fixture["full_volume"] if tag in fixture["parents"] else fixture["flight_volume"],places=8)
        lower,upper=[api.ramp_geometry(tag,model) for tag in fixture["parents"]]
        self.assertEqual(len(lower["parts"]),3)
        self.assertEqual(len(upper["parts"]),2)
        self.assertEqual(len(upper["shared_parts"]),1)
        self.assertEqual(upper["shared_parts"][0]["owner"],lower["tag"])
        self.assertEqual({p["owner"] for p in lower["child_parts"]},set(fixture["flights"][:2]))
        for tag in fixture["parents"]:
            q=api.quantities(tag,model)
            self.assertAlmostEqual(q["volume"],q["exclusive_volume"]+q["shared_volume"]+q["child_volume"],places=8)
    def test_diagonal_flight_has_true_sloping_bottom_and_independent_slices(self):
        f=json.loads(Path(__file__).with_suffix(".json").read_text(encoding="utf-8"))["diagonal"]
        model=api.load_model()
        obj=api._find(model,"RF-1A")
        obj["geometry"].update({k:f[k] for k in ("path","from_level","to_level","portion")})
        obj["geometry"]["thickness"]["value"]=f["thickness"]
        obj["attributes"]["width"]["value"]=f["width"]
        g=api.ramp_geometry(obj["tag"],model)
        part=g["parts"][0]
        vertices=np.asarray([[x,y,z] for heights in (f["bottom_z"],f["top_z"]) for (x,y),z in zip(f["footprint"],heights)])
        self.assertAlmostEqual(ConvexHull(vertices).volume,f["volume"])
        self.assertAlmostEqual(api._polyhedron_volume(part),f["volume"])
        self.assertLess(Polygon(api.part_footprint(part)).symmetric_difference(Polygon(f["footprint"])).area,1e-10)
        self.assertEqual({round(v[2],10) for v in api.part_vertices(part)},set(f["bottom_z"]+f["top_z"]))
        hull=ConvexHull(vertices)
        for axis,value in ((0,1.5),(1,2.),(2,.5)):
            points=[]
            for face in hull.simplices:
                for a,b in zip(face,np.roll(face,-1)):
                    u,v=vertices[a],vertices[b]
                    if (u[axis]-value)*(v[axis]-value)<=0 and abs(v[axis]-u[axis])>1e-12:
                        p=u+(v-u)*(value-u[axis])/(v[axis]-u[axis])
                        points.append(np.delete(p,axis))
            points=np.unique(np.round(points,12),axis=0)
            expected=Polygon(points[ConvexHull(points).vertices])
            actual=Polygon(api.part_outline_at(part,value) if axis==2 else api.part_section(part,value,axis))
            self.assertLess(actual.symmetric_difference(expected).area,1e-9)
        self.assertEqual(g["thickness_direction"],"vertical")
    def test_owned_source_inventory_has_no_landing_or_flight_double_material(self):
        model=api.load_model()
        parts=[p for tag in ("RM-1","RM-2") for field in ("parts","child_parts") for p in api.ramp_geometry(tag,model)[field]]
        self.assertEqual(len(parts),9)
        hulls=[ConvexHull(api.part_vertices(p)) for p in parts]
        for i,a in enumerate(hulls):
            for b in hulls[i+1:]:
                planes=np.concatenate([a.equations,b.equations])
                fit=linprog([0,0,0,-1],A_ub=np.column_stack([planes[:,:3],np.ones(len(planes))]),b_ub=-planes[:,3],bounds=[(None,None)]*3+[(0,None)],method="highs")
                if fit.success and fit.x[3]>1e-10:
                    common=HalfspaceIntersection(planes,fit.x[:3]).intersections
                    self.assertLess(ConvexHull(common).volume,1e-10)
        self.assertAlmostEqual(sum(h.volume for h in hulls),3.238368,places=8)
    def test_invalid_convention_and_borrowed_landing_drift_fail(self):
        model=api.load_model()
        api._find(model,"RF-1A")["geometry"]["thickness_direction"]="normal"
        with self.assertRaises(ValueError):
            api.ramp_geometry("RF-1A",model)
        model=api.load_model()
        api._find(model,"RM-2")["geometry"]["landings"][0]["thickness"]["value"]+=.01
        with self.assertRaises(ValueError):
            api.ramp_geometry("RM-2",model)
    def test_native_ramps_keep_source_products_and_aggregation_ownership(self):
        exchange=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🔌️exchange.🐍️")
        model=api.load_model()
        for tag in ("RM-1","RM-2","RF-1A"):
            native=exchange.step_excerpt(tag,model)
            self.assertEqual(native["class"],api._find(model,tag)["class"])
            self.assertEqual(native["guid"],api._find(model,tag)["guid"])
            self.assertTrue(any("IFCFACETEDBREP" in line for line in native["lines"]))
            if tag.startswith("RM-"):
                self.assertTrue(any("IFCRELAGGREGATES" in line for line in native["lines"]))

    def test_production_callbacks_and_vehicle_obstacles_use_the_same_parts_once(self):
        root=Path(__file__).parents[2]/"🎨️visuals/🏛️savoye"
        def definitions(file,names,scope,methods=False):
            tree=ast.parse(file.read_text(encoding="utf-8"))
            nodes=[n for cls in tree.body if isinstance(cls,ast.ClassDef) for n in cls.body] if methods else tree.body
            exec(compile(ast.Module(body=[n for n in nodes if isinstance(n,(ast.FunctionDef,ast.ClassDef)) and n.name in names],type_ignores=[]),str(file),"exec"),scope)
        model=api.load_model()
        ramps=[api.ramp_geometry(tag,model) for tag in ("RM-1","RM-2")]
        shared={"np":np,"_model":api}
        definitions(root/"🧱️core/📐️geometry.py",{"part_faces"},shared)
        core=SimpleNamespace(**shared,role=lambda key,model:key)
        scope={"np":np,"_c":core,"_item":lambda tag,*a:{"tag":tag},"_slab_prims":lambda *a:[],**{key:i for i,key in enumerate(("STAGE_CELLAR_SLAB","STAGE_PLANE","STAGE_SLAB","STAGE_ROOF","STAGE_UNDER","STAGE_WALLS","STAGE_FIRST"))}}
        definitions(root/"🧊️axon/🧩️primitives.py",{"Prim","_face","_poly","_part_faces","_build"},scope)
        context={"slabs":[{"tag":"floor","guid":"f","level":"first","storey":"first","parts":[{"z0":2.91}]},{"tag":"roof","guid":"r","level":"roof","storey":"roof","parts":[]}],"storeys":{"first":3.17,"roof":6.6},"pilotis":[],"walls":[],"fillings":[],"ramp":{"ramps":ramps}}
        primitives=scope["_build"](context,model)
        self.assertEqual(len(primitives),9)
        for primitive in primitives:
            vertices=np.unique(np.concatenate([ring for face in primitive.faces for ring in face[1]]),axis=0)
            actual=ConvexHull(vertices).volume
            expected=sum(ConvexHull(api.part_vertices(p)).volume for ramp in ramps for p in ramp["parts"]+ramp["child_parts"] if p["owner"]==primitive.tag and abs((p["z1"] if p["role"]=="landing" else p["z0"])-primitive.z)<1e-9)
            self.assertAlmostEqual(actual,expected,places=8)
        class Group:
            def __init__(self,*args):self.submobjects=list(args)
            def add(self,*args):self.submobjects.extend(args);return self
        polygons,lines=[],[]
        def polygon(ring,*a,**k):polygons.append(Polygon(ring));return Group()
        def polyline(ring,*a,**k):lines.append(ring);return Group()
        scope={"np":np,"_c":core,"_d":SimpleNamespace(polygon=polygon,polyline=polyline),"_engine":SimpleNamespace(VGroup=Group),"CONCRETE":"c","INK":"i","W_THIN":1,"_s":SimpleNamespace(pseudo=lambda *a:{})}
        definitions(root/"📋️plan/🐍️.py",{"_plan_ramps"},scope,True)
        ctx={"g":{"ramp":{"ramps":ramps}},"view":lambda x,y,z:np.array([x,y]),"look":None,"level":"first","w":1,"bands":{"first":(3.17,6.6)},"catalogue":[{"tag":r["tag"]} for r in ramps],"put":lambda *a:None}
        owner=SimpleNamespace(_pair=lambda *a:("c","i",1,0))
        scope["_plan_ramps"](owner,ctx,False)
        self.assertEqual(len(polygons),2)
        self.assertEqual(len(lines),2)
        expected=[Polygon(api.part_footprint(p)) for p in ramps[1]["child_parts"]]
        for actual in polygons:
            self.assertLess(min(actual.symmetric_difference(p).area for p in expected),1e-9)
        sections=[]
        scope={"_c":core,"_s":SimpleNamespace(pseudo=lambda *a:{}),"_engine":SimpleNamespace(VGroup=Group),"CONCRETE":"c","INK":"i","BEYOND":"b","W_THIN":1,"EPS":1e-9}
        definitions(root/"✂️section/🐍️.py",{"part_projection","rect"},scope)
        definitions(root/"✂️section/🐍️.py",{"_sec_ramps"},scope,True)
        plane=SimpleNamespace(value=10.2,index=0,extent=lambda ring:(min(p[0] for p in ring)-10.2,max(p[0] for p in ring)-10.2),classify=lambda lo,hi:"cut" if lo<=0<=hi else "removed")
        owner._cut_poche=lambda ring,*a:sections.append(Polygon(ring)) or Group()
        ctx={"g":{"ramp":{"ramps":ramps}},"plane":plane,"to":lambda h,z:(h,z),"look":None,"put":lambda *a:None,"catalogue":[{"tag":e["tag"]} for e in model["elements"]]}
        scope["_sec_ramps"](owner,ctx)
        expected=[Polygon(api.part_section(p,10.2,0)) for r in ramps for p in r["parts"]+r["child_parts"] if min(v[0] for v in p["outline"])<=10.2<=max(v[0] for v in p["outline"])]
        self.assertEqual(len(sections),7)
        for actual in sections:
            self.assertLess(min(actual.symmetric_difference(p).area for p in expected),1e-9)
        vehicle={"building_geometry":lambda m:{"ramp":{"ramps":ramps}},"deepcopy":deepcopy}
        definitions(Path(__file__).parents[1]/"🐍️.py",{"_vehicle_parts"},vehicle)
        records=vehicle["_vehicle_parts"](model)
        self.assertEqual(len(records),9)
        self.assertEqual(sum(r["class"]=="IfcRampFlight" for r in records),4)

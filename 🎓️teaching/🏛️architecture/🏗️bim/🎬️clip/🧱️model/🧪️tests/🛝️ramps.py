"""🛝️ Real inclined slab products, assembly ownership and shared projection geometry."""
import ast,importlib,json,unittest
from types import SimpleNamespace
from copy import deepcopy
from pathlib import Path
import numpy as np
from scipy.spatial import ConvexHull
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
            if tag in fixture["flights"]:
                self.assertAlmostEqual(q["volume"],fixture["flight_volumes"][tag],places=8)
        lower,upper=[api.ramp_geometry(tag,model) for tag in fixture["parents"]]
        for parent in (lower,upper):
            self.assertAlmostEqual(sum(ConvexHull(api.part_vertices(part)).volume for part in parent["parts"]),fixture["landing_volumes"][parent["tag"]],places=8)
        self.assertEqual({p["owner"] for p in lower["shared_parts"]},{"SL-GROUND-CELLAR","SL-1"})
        self.assertEqual({p["owner"] for p in upper["shared_parts"]},{"SL-1","SL-R"})
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
        q=lambda value:{"value":value,"unit":"m","basis":"inferred","source":"Independent diagonal fixture"}
        obj["geometry"].update(structural_offsets=[q(-f["width"]/2),q(f["width"]/2)],walking_offsets=[q(-f["width"]/2),q(f["width"]/2)],finish_thickness=q(.03),finish_direction="vertical")
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
        self.assertEqual(len({p["owner"] for p in parts}),6)
        hulls=[ConvexHull(api.part_vertices(p)) for p in parts]
        oracle=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🛝️interfaces")
        lower=np.asarray([h.points.min(0) for h in hulls]);upper=np.asarray([h.points.max(0) for h in hulls])
        for i,p in enumerate(parts):
            candidates=np.flatnonzero(np.all(np.minimum(upper[:i],upper[i])>np.maximum(lower[:i],lower[i]),axis=1))
            for j in candidates:self.assertLess(oracle.common_volume(p,parts[int(j)]),1e-10)
        self.assertAlmostEqual(sum(h.volume for h in hulls),json.loads(Path(__file__).with_suffix(".json").read_text())["source"]["exclusive_volume"],places=8)
    def test_invalid_convention_and_borrowed_landing_drift_fail(self):
        model=api.load_model()
        api._find(model,"RF-1A")["geometry"]["thickness_direction"]="normal"
        with self.assertRaises(ValueError):
            api.ramp_geometry("RF-1A",model)
        model=api.load_model()
        api._find(model,"RM-2")["geometry"]["landings"][0]["owner"]="RM-1"
        with self.assertRaises(ValueError):
            api.ramp_geometry("RM-2",model)
        for row in json.loads(Path(__file__).with_suffix(".json").read_text())["invalid_joint_owners"]:
            model=api.load_model();api._find(model,row["flight"])["geometry"]["flight_joint_owners"]=row["owners"]
            with self.assertRaises(ValueError):api.ramp_geometry(row["flight"],model)
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
        self.assertEqual(len(primitives),sum(len(r["parts"])+len(r["child_parts"]) for r in ramps))
        remaining=[(p["owner"],np.unique(np.asarray(api.part_vertices(p)),axis=0)) for ramp in ramps for p in ramp["parts"]+ramp["child_parts"]]
        for primitive in primitives:
            vertices=np.unique(np.concatenate([ring for face in primitive.faces for ring in face[1]]),axis=0)
            match=next((i for i,(owner,points) in enumerate(remaining) if owner==primitive.tag and np.array_equal(points,vertices)),None)
            self.assertIsNotNone(match)
            _,expected=remaining.pop(match)
            self.assertAlmostEqual(ConvexHull(vertices).volume,ConvexHull(expected).volume,places=8)
        self.assertFalse(remaining)
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
        self.assertEqual(len(polygons),sum(len(f["parts"]) for f in ramps[1]["flights"]))
        self.assertEqual(len(lines),sum(landing["owner"]==ramp["tag"] and 3.17-1e-9<=landing["z"]<6.6-1e-9 for ramp in ramps for landing in ramp["landings"]))
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
        self.assertEqual(len(sections),len(expected))
        for actual in sections:
            index=min(range(len(expected)),key=lambda i:actual.symmetric_difference(expected[i]).area)
            self.assertLess(actual.symmetric_difference(expected.pop(index)).area,1e-9)
        self.assertFalse(expected)
        vehicle={"building_geometry":lambda m:{"ramp":{"ramps":ramps}},"deepcopy":deepcopy}
        definitions(Path(__file__).parents[1]/"🐍️.py",{"_vehicle_parts"},vehicle)
        records=vehicle["_vehicle_parts"](model)
        self.assertEqual(len(records),sum(len(r["parts"])+len(r["child_parts"]) for r in ramps))
        self.assertEqual(sum(r["class"]=="IfcRampFlight" for r in records),sum(len(r["child_parts"]) for r in ramps))
                                                                                                                                                                                                                                                                                                                    
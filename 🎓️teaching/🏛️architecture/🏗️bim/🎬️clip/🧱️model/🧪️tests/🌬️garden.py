"""🌬️ Registered functional garden post zones and distinct concave weather cap."""
import importlib,json,unittest
from pathlib import Path
import numpy as np
from shapely.geometry import Polygon,box
from shapely.ops import unary_union
from scipy.spatial import ConvexHull
api=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
class GardenConstruction(unittest.TestCase):
    def test_three_functional_post_sections(self):
        fixture=json.loads(Path(__file__).with_suffix(".json").read_text(encoding="utf-8"))
        model=api.load_model()
        for tag in fixture["posts"]:
            obj=api.element(tag,model)
            source=next(e for e in model["elements"] if e["tag"]==tag)
            physical=api.column_geometry(model,source)
            self.assertEqual(len(physical["parts"]),3)
            for actual,expected in zip(physical["parts"],fixture["zones"]):
                self.assertEqual(actual["role"],expected["role"])
                self.assertAlmostEqual(actual["z0"],expected["base"])
                self.assertAlmostEqual(actual["z1"],expected["top"])
                translated=[(x-physical["x"],y-physical["y"]) for x,y in actual["outline"]]
                self.assertLess(Polygon(translated).symmetric_difference(Polygon(expected["outline"])).area,1e-10)
                xy=np.asarray(expected["outline"])
                area=abs(np.dot(xy[:,0],np.roll(xy[:,1],-1))-np.dot(xy[:,1],np.roll(xy[:,0],-1)))/2
                self.assertAlmostEqual(actual["area"],area)
            self.assertEqual(len({round(p["area"],8) for p in physical["parts"]}),3)
            exposed=physical["parts"][1]["outline"]
            self.assertAlmostEqual(max(y for x,y in exposed)-min(y for x,y in exposed),.15)
            self.assertEqual(source["historical_evidence"]["status"],"inferred_reconstruction")
            self.assertEqual(obj["material"]["tag"],"MAT-RENDERED")
    def test_concave_cap_is_separate_material_owned_geometry(self):
        fixture=json.loads(Path(__file__).with_suffix(".json").read_text(encoding="utf-8"))["cap"]
        model=api.load_model()
        cap=api.solid_geometry(fixture["tag"],model)
        parts=cap["parts"]
        rings=[api.part_section(p,sum(fixture["span"])/2,1) for p in parts]
        actual=unary_union([Polygon(r) for r in rings if len(r)>2])
        expected=Polygon(fixture["profile"])
        self.assertLess(actual.symmetric_difference(expected).area,1e-9)
        self.assertGreater(expected.convex_hull.area-expected.area,.01)
        self.assertEqual({p["material"] for p in parts},set(fixture["materials"]))
        for i,part in enumerate(parts):
            self.assertIn("section",part)
            self.assertAlmostEqual(api._occupied_volume([part]),ConvexHull(api.part_vertices(part)).volume,places=8)
            for other in parts[i+1:]:
                self.assertLess(Polygon(api.part_section(part,10,1)).intersection(Polygon(api.part_section(other,10,1))).area,1e-10)
        self.assertAlmostEqual(max(p["z1"] for p in parts),fixture["top"])
        self.assertAlmostEqual(min(p["z0"] for p in parts),fixture["base"])
        self.assertAlmostEqual(api.quantities(fixture["tag"],model)["volume"],expected.area*(fixture["span"][1]-fixture["span"][0]),places=8)
        beam=api.solid_geometry("BEAM-PRIMARY-R-6-22",model)
        self.assertAlmostEqual(min(p["z0"] for p in beam["parts"]),6.19)
        self.assertAlmostEqual(max(p["z1"] for p in beam["parts"]+beam["shared_parts"]),6.51)
        for part in parts:
            self.assertTrue(all(part["z0"]>=b["z1"]-1e-9 for b in beam["parts"]+beam["shared_parts"]))
        self.assertTrue(all(abs(p["section"]["sweep"][0]-fixture["span"][0])<1e-9 and abs(p["section"]["sweep"][1]-fixture["span"][1])<1e-9 for p in parts))

    def test_actual_cap_roof_plan_and_exact_surface_projections(self):
        import ast
        from types import SimpleNamespace
        root=Path(__file__).parents[2]/"🎨️visuals/🏛️savoye"
        def definitions(file,names,scope,methods=False):
            tree=ast.parse(file.read_text(encoding="utf-8"))
            nodes=[n for cls in tree.body if isinstance(cls,ast.ClassDef) for n in cls.body] if methods else tree.body
            exec(compile(ast.Module(body=[n for n in nodes if isinstance(n,ast.FunctionDef) and n.name in names],type_ignores=[]),str(file),"exec"),scope)
        draft=SimpleNamespace(GLASS="glass",CONCRETE="concrete",INK="ink",GOOD="garden",MUTED="metal",mix=lambda *a:"paving")
        core={"_d":draft,"_model":api,"np":np}
        definitions(root/"🧱️core/📐️geometry.py",{"surface_role","part_faces"},core)
        definitions(root/"🧱️core/🐍️.py",{"surface_color"},core)
        shared=SimpleNamespace(**core)
        cap=api.solid_geometry("GARDEN-WEATHER-CAP",api.load_model())
        records=[]
        class Group:
            def __init__(self,*args):self.submobjects=list(args)
            def add(self,*args):self.submobjects.extend(args);return self
        def polygon(ring,*args,**kwargs):records.append(Polygon(ring));return Group()
        scope={"_c":shared,"_model":api,"np":np,"_engine":SimpleNamespace(VGroup=Group),"_d":SimpleNamespace(polygon=polygon,polyline=lambda *a,**k:Group()),"INK":"ink","W_HAIR":1,"W_THIN":1,"EPS":1e-9}
        definitions(root/"📋️plan/🐍️.py",{"part_plan_outline","plan_part_visible","plan_part_rank"},scope)
        definitions(root/"📋️plan/🐍️.py",{"_plan_solids"},scope,True)
        ctx={"bands":{"roof":(6.6,9.4)},"level":"roof","g":{"details":[cap]},"use":{"architecture"},"catalogue":[{"tag":cap["tag"]}],"look":SimpleNamespace(state=lambda item:"normal"),"cut_z":7.6,"below":True,"screen":lambda p:p,"put":lambda *a:None}
        scope["_plan_solids"](SimpleNamespace(_pair=lambda look,item,fill,stroke:(fill,stroke,1,0)),ctx)
        self.assertTrue(records)
        self.assertLess(unary_union(records).symmetric_difference(box(18.74,6.081,19.10,15.5)).area,1e-9)
        sec={"_c":shared,"EPS":1e-9}
        definitions(root/"✂️section/🐍️.py",{"part_projection"},sec)
        sections=[]
        for part in cap["parts"]:
            faces=shared.part_faces(part)
            area=sum(np.linalg.norm(np.cross(np.asarray(v)[i]-v[0],np.asarray(v)[i+1]-v[0]))/2 for v,n in faces for i in range(1,len(v)-1))
            self.assertAlmostEqual(area,ConvexHull(api.part_vertices(part)).area,places=8)
            sections.extend(Polygon(r) for r in sec["part_projection"](part,SimpleNamespace(value=10,index=1),"cut"))
        expected=json.loads(Path(__file__).with_suffix(".json").read_text(encoding="utf-8"))["cap"]["profile"]
        self.assertLess(unary_union(sections).symmetric_difference(Polygon(expected)).area,1e-9)

    def test_garden_auxiliaries_and_wall_leaves_respect_post_ownership(self):
        model=api.load_model()
        posts=[p for tag in ("16-FIRST","22-FIRST") for p in api.column_geometry(model,api._find(model,tag))["parts"]]
        auxiliaries=[p for e in model["elements"] if e["geometry"].get("section_sweep",{}).get("wall")=="W-GARDEN-E" for p in api.solid_geometry(e["tag"],model)["parts"]]
        leaves=api._wall_geometry(model,api._find(model,"W-GARDEN-E"))["parts"]
        for part in auxiliaries+leaves:
            for post in posts:
                if min(part["z1"],post["z1"])-max(part["z0"],post["z0"])>1e-9:
                    self.assertLess(Polygon(part["outline"]).intersection(Polygon(post["outline"])).area,1e-9)

    def test_support_graph_tracks_actual_garden_and_kiosk_hosts(self):
        fixture=json.loads(Path(__file__).with_suffix(".json").read_text(encoding="utf-8"))
        model=api.load_model()
        for tag,targets in fixture["support_targets"].items():
            relation=next(r for r in model["relations"] if r["tag"]=="REL-SUPPORT-"+tag)
            self.assertEqual(relation["relating"],tag)
            self.assertEqual(relation["related"],targets)
            self.assertNotIn("W-E",relation["related"])
            post=api._find(model,tag)
            for target in targets:
                obj=api._find(model,target)
                if obj["class"]=="IfcWall":
                    ys=[p[1] for p in obj["geometry"]["path"]]
                    self.assertTrue(min(ys)<=post["geometry"]["axis"][1]<=max(ys))

"""🤝 Validate explicit source-owned structural interfaces without alias aggregation or global clipping."""
import copy,hashlib,importlib,json,sys,unittest
from pathlib import Path
from shapely.geometry import Polygon
from jsonschema import Draft202012Validator
sys.dont_write_bytecode=True
api=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
CASE_PATH=Path(__file__).with_suffix(".json")
CASES=json.loads(CASE_PATH.read_text(encoding="utf8"))
SCHEMA=json.loads(CASE_PATH.with_suffix(".schema.json").read_text(encoding="utf8"))

def neutral_owner(row):
    """🧱 Instantiate independently declared fixture stock far from unrelated building material."""
    model=api.load_model();owner=copy.deepcopy(api._find(model,"GUARD-RM-1"));owner.update(tag="NEUTRAL-AUTHORITY-OWNER",guid=api.global_id("NEUTRAL-AUTHORITY-OWNER"),material="MAT-RC",psets={});owner["class"]=row["owner_class"]
    lo,hi=row["owner_x"];ring=[[lo,100],[hi,100],[hi,101],[lo,101]];vertices=[[*p,z]for z in (0,1)for p in ring];faces=[[3,2,1,0],[4,5,6,7]]+[[i,(i+1)%4,(i+1)%4+4,i+4]for i in range(4)]
    owner["geometry"]={"kind":"guard","base_level":0,"parts":[{"mesh":{"vertices":vertices,"faces":faces},"material":"MAT-RC","role":"structural_guard"}]}
    model["elements"]=[obj for obj in model["elements"]if obj["tag"]in ("GL-RAMP-GARDEN","FIN-HALL","FIN-RAMP-EXTERIOR")]+[owner]
    return model,owner,ring


class StructuralAuthority(unittest.TestCase):
    def test_reserve_queries_union_actual_material_and_reject_two_nm_conflicts(self):
        from shapely.ops import unary_union
        Draft202012Validator(SCHEMA).validate(CASES)
        def ring(interval):
            lo,hi=interval
            return [[100+lo,100],[100+hi,100],[100+hi,101],[100+lo,101]]
        for row in CASES["window_cases"]:
            with self.subTest(case=row["id"]):
                model,_,_=neutral_owner({"owner_class":"IfcRailing","owner_x":[102,103]});prototype=api._find(model,"FIN-HALL");objects={}
                for key,interval in row["bodies"].items():
                    obj=copy.deepcopy(prototype);tag="NEUTRAL-WINDOW-"+key.upper();kind=row.get("classes",{}).get(key,"IfcCovering")
                    obj.update(tag=tag,guid=api.global_id(tag),material="MAT-RC"if kind=="IfcChimney"else "MAT-GLASS");obj["class"]=kind
                    obj["geometry"]={"kind":"prismatic_chimney"if kind=="IfcChimney"else "flooring","base_level":0,"parts":[{"outline":ring(interval),"z0":0,"z1":1,"material":obj["material"],"role":"core"if kind=="IfcChimney"else "wearing"}]};objects[key]=obj
                for key,refs in row.get("declarations",{}).items():objects[key]["geometry"]["interface_owners"]=[objects[ref]["tag"]for ref in refs]
                model["elements"]=[*model["elements"],*objects.values()]
                windows=[{"tag":"NEUTRAL-WINDOW-QUERY-"+str(index),"owner":objects[key]["tag"],"outline":ring(interval)}for index,(key,interval)in enumerate(zip(row["primary"],row["windows"]))]
                target=[{"outline":ring(interval),"z0":0,"z1":1,"material":"MAT-GLASS","role":"finish"}for interval in row.get("host_intervals",[[0,1]])]
                slab={"tag":"NEUTRAL-WINDOW-HOST","geometry":{"finish_voids":windows}};before=copy.deepcopy(model)
                if row.get("error"):
                    pattern="share occupied|overlap"if row["error"]=="overlap"else "not fully occupied|unoccupied|incomplete"
                    with self.assertRaisesRegex(ValueError,pattern):api._finish_reservations(model,slab,target,exact=True)
                else:
                    owned,shared=api._finish_reservations(model,slab,target,exact=True)
                    self.assertFalse(owned);self.assertEqual(api._native_parts_exact_volume(shared),api._native_parts_exact_volume(target))
                    independent=unary_union([Polygon(part["outline"])for part in target]).area
                    self.assertAlmostEqual(float(api._native_parts_exact_volume(shared)),independent,delta=1e-13)
                    self.assertAlmostEqual(independent,row["expected_volume"],delta=1e-13)
                self.assertEqual(model,before)

    def test_ramp_finish_uses_declared_native_guard_interfaces(self):
        from unittest.mock import patch
        backend=api._ramp_finish_backend()
        for row in (case for case in CASES["cases"]if not case.get("inherit")and not case.get("error")):
            with self.subTest(case=row["id"]):
                model,owner,ring=neutral_owner(row)
                finish=copy.deepcopy(api._find(model,"FIN-RAMP-EXTERIOR"))
                finish["geometry"]["ramp_finish"]["interface_owners"]=[owner["tag"]]if row["declared"]else []
                finish["geometry"]["ramp_finish"]["wall_owners"]=[]
                stock={"outline":[[100,100],[101,100],[101,101],[100,101]],"z0":0,"z1":1,"material":"MAT-GLASS","role":"wearing"}
                with patch.object(backend,"reference_stocks",return_value=[stock]):
                    result=backend.reference_parts(api,model,finish)
                volume=sum(api._polyhedron_volume(part)for part in result)
                expected=Polygon(stock["outline"]).intersection(Polygon(ring)).area if row["declared"]else 0
                self.assertAlmostEqual(1-volume,expected,delta=1e-13)
                self.assertEqual({part["material"]for part in result},{"MAT-GLASS"})


    def test_exact_slab_boundaries_preserve_native_holes_and_two_nm_material(self):
        from shapely.ops import unary_union
        Draft202012Validator(SCHEMA).validate(CASES)
        for row in CASES["slab_boundary_cases"]:
            with self.subTest(case=row["id"]):
                result=api._slab_surface(row["outline"],row["outline"],row["holes"])
                oracle=Polygon(row["outline"]).difference(unary_union([Polygon(ring)for ring in row["holes"]]))
                actual=unary_union([Polygon(p)for p in result])
                self.assertEqual(actual.symmetric_difference(oracle).area,0)
                self.assertEqual(actual.boundary.symmetric_difference(oracle.boundary).length,0)
                if row.get("retained_width"):
                    self.assertGreater(actual.area,0)
                    self.assertAlmostEqual(actual.area,row["retained_width"],delta=1e-15)


    def test_neutral_declared_guard_filling_ownership_and_two_nm_positive(self):
        Draft202012Validator(SCHEMA).validate(CASES)
        for row in CASES["cases"]:
            with self.subTest(case=row["id"]):
                model,owner,ring=neutral_owner(row)
                filling=copy.deepcopy(api._find(model,"GL-RAMP-GARDEN"))
                filling["geometry"]={"interface_owners":[owner["tag"]]}if row["declared"]else {}
                if row.get("inherit"):
                    filling["geometry"]={"opening":"NEUTRAL-OPENING"}
                    model["elements"]=[*model["elements"],{"tag":"NEUTRAL-OPENING","guid":api.global_id("NEUTRAL-OPENING"),"class":"IfcOpeningElement","geometry":{"wall":"NEUTRAL-HOST"}},{"tag":"NEUTRAL-HOST","guid":api.global_id("NEUTRAL-HOST"),"class":"IfcWall","geometry":{"interface_owners":[owner["tag"]]}}]
                stock={"outline":[[100,100],[101,100],[101,101],[100,101]],"z0":0,"z1":1,"material":"MAT-GLASS","role":"glass"}
                before=json.dumps(owner,sort_keys=True);source_stock=copy.deepcopy(stock)
                if row.get("error"):
                    with self.assertRaises(ValueError):api._filling_structures(model,filling)
                    continue
                structures=api._filling_structures(model,filling);result=api._filling_owned_parts(model,[stock],structures);volume=sum(api._polyhedron_volume(p)for p in result)
                expected=Polygon(stock["outline"]).intersection(Polygon(ring)).area if row["declared"]else 0
                self.assertAlmostEqual(1-volume,expected,delta=1e-13)
                if row["expected_removed"]>0:self.assertGreater(1-volume,row["expected_removed"]*.99)
                else:self.assertEqual(volume,1.)
                self.assertTrue(all(p["material"]=="MAT-GLASS"and p["role"]=="glass"for p in result));self.assertEqual(owner,json.loads(before));self.assertEqual(stock,source_stock)
    def test_actual_source_declares_every_verified_structural_authority(self):
        model=api.load_model();schema=json.loads(Path(api.__file__).with_name("🧬️schema").joinpath("🔣️.schema.json").read_text(encoding="utf8"));Draft202012Validator(schema).validate(model)
        for group,products in CASES["source"].items():
            for tag,owners in products.items():
                obj=api._find(model,tag);declared=set(obj["geometry"].get("interface_owners",()))
                if group=="fillings":
                    opening=api._find(model,obj["geometry"]["opening"])
                    for field in ("wall","curtain_wall","slab"):
                        if field in opening["geometry"]:declared.update(api._find(model,opening["geometry"][field])["geometry"].get("interface_owners",()))
                with self.subTest(group=group,tag=tag):self.assertTrue(set(owners)<=declared)
    def test_actual_owned_ramp_landing_interface_excludes_borrowed_and_child_aliases(self):
        model=api.load_model();parts=api._interface_owner_parts(model,"RM-1")
        self.assertTrue(parts);self.assertTrue(all(p.get("owner")=="RM-1"and p.get("role")=="landing"for p in parts))
        self.assertTrue(all(p.get("tag")=="LANDING-FIRST-MID"for p in parts))
    def test_stringer_reference_receives_the_actual_structural_upper_datum(self):
        model=api.load_model();obj=api._find(model,"BEAM-RAMP-FIRST-STRINGER");owned=api._interface_owner_parts(model,obj["tag"]);reference=api._ramp_stringer_reference_stock(model,obj)
        vertices=[p for part in owned for p in api.part_vertices(part)];station=min(p[1]for p in vertices)
        native_top=max(p[2]for p in vertices if float(p[1])==station);ref_top=max(p[2]for p in api.part_vertices(reference)if float(p[1])==station)
        self.assertEqual(float(ref_top),native_top)

    def test_covering_interface_preserves_full_reference_and_exclusive_material_mass(self):
        row=CASES["cases"][0];model,owner,ring=neutral_owner(row);covering=copy.deepcopy(api._find(model,"FIN-HALL"));covering.update(tag="NEUTRAL-COVERING",guid=api.global_id("NEUTRAL-COVERING"),material="MAT-GLASS")
        stock={"outline":[[100,100],[101,100],[101,101],[100,101]],"z0":0,"z1":1,"material":"MAT-GLASS","role":"wearing"}
        covering["geometry"]={"kind":"flooring","base_level":0,"parts":[stock],"interface_owners":[owner["tag"]]};model["elements"].append(covering)
        source=copy.deepcopy(covering);solid=api._solid_geometry(model,covering,resolve_embedded=False)
        self.assertAlmostEqual(sum(api._polyhedron_volume(p)for p in solid["parts"]),.5,places=12)
        self.assertEqual(sum(api._polyhedron_volume(p)for p in solid["full_parts"]),1.)
        quantity=api.quantities(covering["tag"],model);self.assertEqual(quantity["volume"],.5);self.assertEqual(quantity["reference_volume"],1.)
        expected_mass=.5*api._find(model,"MAT-GLASS")["density"]["value"]*1000/9.80665
        self.assertAlmostEqual(quantity["mass"],expected_mass,places=4);self.assertEqual(covering,source)

    def test_native_finish_reserve_partition_rejects_two_nm_gaps_and_cross_owner_overlap(self):
        for row in CASES["reserve_cases"]:
            with self.subTest(case=row["id"]):
                model,guard,ring=neutral_owner({"owner_class":"IfcRailing","owner_x":[100.5,101]})
                depth=row["depth"];guard["geometry"]["parts"][0]["mesh"]["vertices"]=[[p[0],p[1],depth*p[2]]for p in guard["geometry"]["parts"][0]["mesh"]["vertices"]]
                if row["mode"]=="inclined":
                    guard["geometry"]["parts"][0]["mesh"]["vertices"]=[[p[0],p[1],p[2]+depth*(p[0]-100.5)]for p in guard["geometry"]["parts"][0]["mesh"]["vertices"]]
                covering=copy.deepcopy(api._find(model,"FIN-HALL"));covering.update(tag="NEUTRAL-FINISH-RESERVE",guid=api.global_id("NEUTRAL-FINISH-RESERVE"),material="MAT-GLASS")
                end=101 if row["mode"]in ("partition","inclined")else 100.5+(-1 if row["mode"]=="gap"else 1)*row["offset"]
                stock={"outline":[[100,100],[end,100],[end,101],[100,101]],"z0":0,"z1":depth,"material":"MAT-GLASS","role":"wearing"}
                covering["geometry"]={"kind":"flooring","base_level":0,"parts":[stock]}
                if row["mode"]!="overlap":covering["geometry"]["interface_owners"]=[guard["tag"]]
                model["elements"].append(covering)
                target={"outline":[[100,100],[101,100],[101,101],[100,101]],"z0":0,"z1":depth,"layer":"finish","material":"MAT-GLASS"}
                reserve={"tag":"NEUTRAL-RESERVE","owner":covering["tag"],"outline":target["outline"]}
                if row["mode"]=="overlap":reserve["joint_owners"]=[guard["tag"]]
                slab={"tag":"NEUTRAL-SLAB","geometry":{"finish_voids":[reserve]}}
                if row["mode"]=="gap":
                    oracle_gap=Polygon(target["outline"]).difference(Polygon(stock["outline"]).union(Polygon(ring))).area*depth
                    self.assertGreater(oracle_gap,0);self.assertLess(oracle_gap,1e-10)
                    with self.assertRaisesRegex(ValueError,"not fully occupied|unoccupied|incomplete"):api._finish_reservations(model,slab,[target])
                elif row["mode"]=="overlap":
                    self.assertGreater(Polygon(stock["outline"]).intersection(Polygon(ring)).area*depth,0)
                    with self.assertRaisesRegex(ValueError,"share occupied|overlap"):api._finish_reservations(model,slab,[target])
                else:
                    import numpy as np
                    from scipy.spatial import ConvexHull
                    for exact in (False,True):
                        events=[]
                        owned,shared=api._finish_reservations(model,slab,[target],exact=exact,progress=events.append);self.assertEqual(owned,[]);self.assertAlmostEqual(sum(api._polyhedron_volume(p)for p in shared),depth,places=12)
                        self.assertEqual({p["owner"]for p in shared},{covering["tag"],guard["tag"]})
                        self.assertTrue(set(row.get("progress_phases",()))<={event["phase"]for event in events})
                        oracle=sum(ConvexHull(np.asarray(api.part_vertices(p),dtype=float)).volume for p in shared)
                        self.assertAlmostEqual(oracle,depth,delta=1e-13)
                    if row.get("cancelled_before_work"):
                        with self.assertRaises(InterruptedError):api._slab_geometry_entry({},slab,cancel=lambda:True)

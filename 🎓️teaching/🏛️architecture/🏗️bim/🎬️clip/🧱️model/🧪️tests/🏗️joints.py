"""🏗️ Complete member semantics and disjoint material ownership at column joints."""
import importlib,json,unittest
from copy import deepcopy
from pathlib import Path
from shapely.geometry import Polygon
from shapely.ops import unary_union
from shapely.strtree import STRtree
from shapely import set_precision
from scipy.spatial import ConvexHull
api=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
def shape(parts,z):
    return unary_union([Polygon(p["outline"],p.get("holes",[])) for p in parts if p["z0"]<z<p["z1"]])
class BeamJointOwnershipContract(unittest.TestCase):
    def assert_member(self,tag,owners):
        model=api.load_model()
        obj=api._find(model,tag)
        self.assertEqual(set(obj["geometry"].get("joint_owners",[])),owners)
        before=deepcopy(obj)
        solid=api.solid_geometry(tag,model)
        self.assertEqual(obj,before)
        raw=[{**deepcopy(p),"z0":api._z(model,obj["geometry"]["base_level"])+p["z0"],"z1":api._z(model,obj["geometry"]["base_level"])+p["z1"]} for p in obj["geometry"]["parts"]]
        columns=[p for owner in owners for p in api.column_geometry(model,api._find(model,owner))["parts"]]
        holes=[{"outline":h,"z0":p["z0"],"z1":p["z1"]} for p in columns for h in p.get("holes",[])]
        holes += [{"outline":v["outline"],"z0":min(p["z0"] for p in raw),"z1":max(p["z1"] for p in raw)} for v in obj["geometry"].get("voids",[])]
        envelopes=[{**p,"holes":[]} for p in columns]
        joint=[p for p in solid["shared_parts"] if p.get("owner") in owners and p.get("joint_host")==tag]
        slab_regions=[]
        for slabtag in obj["geometry"].get("embedded_slabs",[]):
            sg=api._find(model,slabtag)["geometry"]
            voids=[api._expand(model,v["outline"]) for v in sg["voids"]]+[e["geometry"]["outline"] for e in model["elements"] if e["class"]=="IfcOpeningElement" and e["geometry"].get("slab")==slabtag]
            for r in sg["regions"]:
                if r.get("member")==tag:
                    top=api._z(model,sg["level"])+r["top_offset"]["value"]
                    polygon=Polygon(api._expand(model,r["outline"])).intersection(Polygon(api._expand(model,sg["outline"])))
                    for v in voids:
                        polygon=polygon.difference(Polygon(v))
                    slab_regions.append((polygon,top-r["thickness"]["value"],top))
        levels=sorted({round(p[k],12) for p in raw+columns+solid["parts"]+joint+solid["full_parts"] for k in ("z0","z1")})
        volume=0
        for lo,hi in zip(levels,levels[1:]):
            if hi-lo<1e-10:
                continue
            z=(lo+hi)/2
            full=shape(raw,z).difference(shape(holes,z))
            cuts=shape(envelopes,z)
            slab=unary_union([p for p,lo,hi in slab_regions if lo<z<hi])
            self.assert_partition(solid["parts"],z,full.difference(cuts),tag)
            self.assert_partition(solid["parts"]+joint,z,full,tag)
            self.assert_partition(solid["full_parts"],z,full.union(slab),tag)
            volume+=full.union(slab).area*(hi-lo)
        q=api.quantities(tag,model)
        self.assertAlmostEqual(q["volume"],q["exclusive_volume"]+q["shared_volume"],places=7)
        self.assertAlmostEqual(q["volume"],volume,places=7)
        self.assertAlmostEqual(sum(x["volume"] for x in q["material_quantities"]),q["exclusive_volume"],places=7)
    def assert_partition(self,parts,z,expected,tag):
        cells=[set_precision(Polygon(p["outline"],p.get("holes",[])),1e-10) for p in parts if p["z0"]<z<p["z1"]]
        expected=set_precision(expected,1e-10)
        for part,cell in zip((p for p in parts if p["z0"]<z<p["z1"]),cells):
            original=Polygon(part["outline"],part.get("holes",[]))
            self.assertLessEqual(abs(original.area-cell.area),original.length*1e-10+1e-16,tag)
        self.assertAlmostEqual(sum(p.area for p in cells),expected.area,places=8,msg=tag)
        for p in cells:
            self.assertLess(p.difference(expected).area,1e-8,tag)
        tree=STRtree(cells)
        for i,p in enumerate(cells):
            for k in tree.query(p):
                if k>i:
                    self.assertLess(p.intersection(cells[k]).area,1e-8,tag)
    def test_complete_nonauxiliary_beam_column_register(self):
        model=api.load_model()
        fixture=json.loads(Path(__file__).with_suffix(".json").read_text(encoding="utf-8"))
        expected={(p["beam"],p["column"],p["part"]) for p in fixture["pairs"]}
        actual=set()
        columns=[(e["tag"],api.column_geometry(model,e)["parts"]) for e in model["elements"] if e["class"]=="IfcColumn"]
        for beam in (e for e in model["elements"] if e["class"]=="IfcBeam" and "section_sweep" not in e["geometry"]):
            base=api._z(model,beam["geometry"]["base_level"])
            owners=set()
            for index,p in enumerate(beam["geometry"]["parts"]):
                self.assertNotIn("section",p)
                self.assertNotIn("mesh",p)
                polygon=Polygon(p["outline"],p.get("holes",[]))
                for tag,sections in columns:
                    volume=sum(polygon.intersection(Polygon(c["outline"],c.get("holes",[]))).area*max(0,min(base+p["z1"],c["z1"])-max(base+p["z0"],c["z0"])) for c in sections)
                    if volume>1e-10:
                        actual.add((beam["tag"],tag,index))
                        owners.add(tag)
            self.assertEqual(set(beam["geometry"].get("joint_owners",[])),owners,beam["tag"])
        self.assertEqual(actual,expected)
    def test_invalid_and_stale_joint_owners_fail(self):
        model=api.load_model()
        for owner in ("SL-1","1"):
            altered=deepcopy(model)
            obj=api._find(altered,"BEAM-PRIMARY-R-6-22")
            obj["geometry"]["joint_owners"]=[owner]
            with self.assertRaises(ValueError):
                api.solid_geometry(obj["tag"],altered)
    def test_native_body_declares_member_and_material_volume_scopes(self):
        exchange=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🔌️exchange.🐍️")
        native=exchange.step_excerpt("BEAM-PRIMARY-R-6-22",api.load_model())
        self.assertIn("ownership_FullMemberVolumeM3",native["ids"])
        self.assertIn("ownership_OwnedVolumeM3",native["ids"])
        self.assertIn("ownership_SharedVolumeM3",native["ids"])
        self.assertTrue(any("exclusive_owned_material" in line for line in native["lines"]))
    def test_exact_pair_register_matches_independent_geos_and_scipy(self):
        fixture=json.loads(Path(__file__).with_suffix(".json").read_text(encoding="utf-8"))
        model=api.load_model()
        total=0
        for pair in fixture["pairs"]:
            obj=api._find(model,pair["beam"])
            p=deepcopy(obj["geometry"]["parts"][pair["part"]])
            base=api._z(model,obj["geometry"]["base_level"])
            p["z0"]+=base
            p["z1"]+=base
            volume=0
            for c in api.column_geometry(model,api._find(model,pair["column"]))["parts"]:
                h=max(0,min(p["z1"],c["z1"])-max(p["z0"],c["z0"]))
                ring=Polygon(p["outline"]).intersection(Polygon(c["outline"],c.get("holes",[])))
                volume+=ring.area*h
                if h and ring.geom_type=="Polygon" and not ring.interiors and ring.area>1e-12:
                    vertices=[[x,y,z] for x,y in list(ring.exterior.coords)[:-1] for z in (0,h)]
                    self.assertAlmostEqual(ConvexHull(vertices).volume,ring.convex_hull.area*h,places=9)
            self.assertAlmostEqual(volume,pair["volume"],places=9,msg=pair)
            total+=volume
        self.assertAlmostEqual(total,fixture["pairwise_volume"],places=9)

def _member_case(tag,owners):
    def check(self):
        self.assert_member(tag,owners)
    return check
_fixture=json.loads(Path(__file__).with_suffix(".json").read_text(encoding="utf-8"))
for _tag in sorted({p["beam"] for p in _fixture["pairs"]}):
    setattr(BeamJointOwnershipContract,"test_"+_tag.replace("-","_"),_member_case(_tag,{p["column"] for p in _fixture["pairs"] if p["beam"]==_tag}))

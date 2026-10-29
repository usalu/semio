"""🏗️ Continuous inferred roof post material bears on RC without occupying weather finish twice."""
import importlib,json,unittest
from copy import deepcopy
from shapely import set_precision
from shapely.ops import unary_union
from shapely.geometry import box
from scipy.spatial import ConvexHull
from pathlib import Path
from shapely.geometry import Polygon
api=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
class RoofPostBearingContract(unittest.TestCase):
    def test_actual_native_posts_have_continuous_rc_material_through_roof_deck(self):
        fixture=json.loads(Path(__file__).with_suffix(".json").read_text(encoding="utf-8"))
        model=api.load_model()
        for tag in fixture["columns"]:
            geometry=api.column_geometry(model,api._find(model,tag))
            sections=sorted(geometry["parts"],key=lambda p:p["z0"])
            for a,b in zip(sections,sections[1:]):
                self.assertAlmostEqual(a["z1"],b["z0"],places=10,msg=tag)
                self.assertGreater(Polygon(a["outline"]).intersection(Polygon(b["outline"])).area,.03)
            for z in (6.341,6.5,6.58,6.599):
                active=[p for p in sections if p["z0"]<z<p["z1"]]
                self.assertEqual(len(active),1,tag)
                self.assertAlmostEqual(Polygon(active[0]["outline"]).area,fixture["section_area"],places=10)
            self.assertAlmostEqual(sections[-1]["z1"],fixture["exposed"][1])
            canopy=api._slab_geometry(model,api._find(model,fixture["canopy"]))
            contact=sum(Polygon(p["outline"]).intersection(Polygon(sections[-1]["outline"])).area for p in canopy["full_parts"] if abs(p["z0"]-sections[-1]["z1"])<1e-10)
            self.assertAlmostEqual(contact,fixture["section_area"],places=10,msg=tag)

    def test_structural_and_weathering_material_are_owned_once_and_conserve_union(self):
        fixture=json.loads(Path(__file__).with_suffix(".json").read_text(encoding="utf-8"))
        model=api.load_model()
        slab=api._slab_geometry(model,api._find(model,fixture["slab"]))
        self.assertEqual(api._find(model,"SL-R")["geometry"]["column_reserves"],fixture["columns"])
        for key,expected in (("shared_parts",fixture["shared_structural_volume"]),("shared_finish_parts",fixture["shared_finish_volume"])):
            selected=[p for p in slab[key] if p["owner"] in fixture["columns"]]
            actual=sum(ConvexHull(api.part_vertices(p)).volume for p in selected)
            self.assertAlmostEqual(actual,expected,places=10)
            self.assertTrue(all(p["owner"] in fixture["columns"] and p["material"]=="MAT-RC" for p in selected))
        self.assertAlmostEqual(fixture["shared_structural_volume"]+fixture["shared_finish_volume"],2*fixture["volume_increase_per_column"],places=10)
        for tag,axis in fixture["axes"].items():
            self.assertEqual(api._find(model,tag)["geometry"]["axis"],axis)
            domain=box(axis[0]-.11,axis[1]-.11,axis[0]+.11,axis[1]+.11)
            post=api.column_geometry(model,api._find(model,tag))["parts"]
            relevant=[e for e in model["elements"] if e["class"]=="IfcBeam" and "section_sweep" not in e["geometry"] and any(Polygon(p["outline"]).intersection(domain).area>1e-12 for p in e["geometry"]["parts"])]
            beams=[p for e in relevant for p in api.solid_geometry(e["tag"],model)["parts"]]
            for z in (6.341,6.5,6.54,6.58,6.599):
                def section(parts):
                    return unary_union([set_precision(Polygon(p["outline"],p.get("holes",[])),1e-10).intersection(domain) for p in parts if p["z0"]<z<p["z1"]])
                self.assertLess(section(slab["parts"]+slab["finish_parts"]+beams).area,1e-9,tag)
                self.assertLess(section(post).symmetric_difference(domain).area,1e-9,tag)
                before=section(slab["full_parts"]+slab["full_finish_parts"])
                self.assertLess(before.symmetric_difference(domain).area,1e-9,tag)
                self.assertLess(before.symmetric_difference(section(post)).area,1e-9,tag)
        q=api.quantities("SL-R",model)
        self.assertAlmostEqual(q["volume"],q["exclusive_volume"]+q["shared_volume"],places=8)
        self.assertAlmostEqual(q["finish_volume"],q["exclusive_finish_volume"]+q["shared_finish_volume"],places=8)
        self.assertAlmostEqual(sum(p["volume"] for p in q["material_quantities"]),q["exclusive_volume"],places=8)
    def test_analytical_slab_domain_keeps_native_column_supports_and_full_load_surface(self):
        analysis=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧮️analysis.🐍️")
        model=api.load_model()
        slab=api._slab_geometry(model,api._find(model,"SL-R"))
        columns=[api.column_geometry(model,api._find(model,t)) for t in ("10-FIRST","15-FIRST")]
        context={"roles":{"roof":"SL-R"},"slabs":[slab],"pilotis":columns}
        domain=analysis._slab(context,"roof")
        self.assertEqual(domain["parts"],slab["full_parts"])
        self.assertEqual(domain["finish_parts"],slab["full_finish_parts"])
        self.assertEqual({c["tag"] for c in analysis._supports(context,domain)},{"10-FIRST","15-FIRST"})
        self.assertAlmostEqual(sum(Polygon(p["outline"]).area for p in domain["parts"]),api.quantities("SL-R",model)["area"],places=8)
        self.assertEqual(api.represent("SL-R","body",model)["derived"]["scope"],"full_member")
    def test_cache_tracks_reserved_columns_and_returns_isolated_parts(self):
        model=api.load_model()
        source=api._find(model,"SL-R")
        first=api._slab_geometry(model,source)
        first["parts"][0]["outline"][0][0]+=100
        self.assertNotEqual(first,api._slab_geometry(model,source))
        changed=deepcopy(model)
        api._find(changed,"10-FIRST")["geometry"]["axis"][0]+=.02
        updated=api._slab_geometry(changed,api._find(changed,"SL-R"))
        self.assertNotEqual(updated["shared_parts"],api._slab_geometry(model,source)["shared_parts"])
    def test_source_embedded_junction_is_explicitly_inferred_and_reinforcement_unassessed(self):
        model=api.load_model()
        for tag in ("10-FIRST","15-FIRST"):
            obj=api._find(model,tag)
            metadata=obj["psets"]["Pset_EmbeddedJunction"]
            self.assertEqual(metadata["Classification"]["value"],"inferred_continuous_rc_embedded_junction")
            self.assertEqual(metadata["Classification"]["basis"],"inferred")
            self.assertEqual(metadata["Reinforcement"]["value"],"not_assessed")
            self.assertEqual(metadata["LowerDatum"]["value"],6.34)
            self.assertEqual(metadata["ExposedBase"]["value"],6.6)
        self.assertEqual(len(api.column_geometry(model,api._find(model,"10-FIRST"))["parts"]),1)
        self.assertEqual(len(api.column_geometry(model,api._find(model,"15-FIRST"))["parts"]),2)

    def test_a_structural_reserve_need_not_reach_the_separate_finish(self):
        fixture=json.loads(Path(__file__).with_suffix(".json").read_text(encoding="utf-8"))
        model=api.load_model()
        column=api._find(model,"10-FIRST")
        column["geometry"]["sections"][0]["top_level"]=fixture["partial_column_top"]
        slab=api._slab_geometry(model,api._find(model,"SL-R"))
        self.assertTrue(any(p["owner"]=="10-FIRST" for p in slab["shared_parts"]))
        self.assertFalse(any(p["owner"]=="10-FIRST" for p in slab["shared_finish_parts"]))

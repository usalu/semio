"""🚰 Native hollow service profiles and one explicitly evidenced historical state."""
from copy import deepcopy
import importlib
import json
from pathlib import Path
import unittest
import numpy as np
from scipy.spatial import ConvexHull
from shapely.geometry import Point,Polygon
from shapely.ops import unary_union

api=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
CASE=json.loads(Path(__file__).with_suffix(".json").read_text(encoding="utf-8"))

class ServiceProfileContract(unittest.TestCase):
    def setUp(self):
        self.model=api.load_model()
        self.pipe=deepcopy(next(e for e in self.model["elements"] if e["class"]=="IfcPipeSegment"))
        self.pipe.update(tag="TEST-TUBE",guid=api.global_id("TEST-TUBE"))
        self.pipe["attributes"]={"diameter":{"value":.2,"unit":"m","basis":"inferred"}}
        self.pipe["geometry"]={"points":CASE["axes"][0]["points"],"tube":{"wall_thickness":{"value":.02,"unit":"m","basis":"inferred"},"segments":96}}
        self.model["elements"].append(self.pipe)

    def test_three_native_axes_preserve_hollow_material_and_exact_endpoints(self):
        for case in CASE["axes"]:
            self.pipe["geometry"]["points"]=case["points"]
            g=api.service_geometry(self.pipe["tag"],self.model)
            points=np.array([v for p in g["parts"] for v in api.part_vertices(p)])
            np.testing.assert_allclose([points.min(axis=0),points.max(axis=0)],case["bounds"],atol=1e-10)
            axis=next(i for i in range(3) if case["points"][0][i]!=case["points"][1][i])
            transverse=[i for i in range(3) if i!=axis]
            projected=unary_union([Polygon(np.array(api.part_vertices(p))[:len(api.part_vertices(p))//2,transverse]) for p in g["parts"]])
            centre=np.array(case["points"][0])[transverse]
            expected=Point(centre).buffer(.1,quad_segs=24).difference(Point(centre).buffer(.08,quad_segs=24))
            self.assertLess(projected.symmetric_difference(expected).area,1e-10)
            self.assertFalse(projected.contains(Point(centre)))
            volume=sum(ConvexHull(np.array(api.part_vertices(p))).volume for p in g["parts"])
            self.assertAlmostEqual(api.quantities(self.pipe["tag"],self.model)["volume"],volume,places=9)
            self.assertAlmostEqual(volume,expected.area*3,places=9)

    def test_every_representation_keeps_native_material_and_hollow_cross_sections(self):
        g=api.service_geometry(self.pipe["tag"],self.model)
        expected=unary_union([Polygon(p["outline"]) for p in g["parts"]])
        for kind in ("axis","footprint","plan","section","elevation","3d","body"):
            self.assertTrue(api.represent(self.pipe["tag"],kind,self.model)["primitives"])
        actual=unary_union([Polygon(p["points"]) for p in api.represent(self.pipe["tag"],"plan",self.model)["primitives"]])
        self.assertLess(actual.symmetric_difference(expected).area,1e-10)
        self.assertFalse(actual.contains(Point(2,3)))

    def test_invalid_wall_and_nonorthogonal_axis_are_rejected(self):
        self.pipe["geometry"]["tube"]["wall_thickness"]["value"]=.11
        with self.assertRaises(ValueError):api.service_geometry(self.pipe["tag"],self.model)
        self.pipe["geometry"]["tube"]["wall_thickness"]["value"]=.02
        self.pipe["geometry"]["points"]=[[0,0,0],[1,1,1]]
        with self.assertRaises(ValueError):api.service_geometry(self.pipe["tag"],self.model)

    def test_original_identity_and_registered_visible_service_families(self):
        m=api.load_model()
        self.assertEqual(m["historical_state"]["id"],CASE["historical_target"])
        for host in CASE["source_cases"]["heating_riser_hosts"]:
            covering=api.element("HEAT-RISER-"+host+"-CASING",m)
            self.assertEqual(covering["class"],"IfcCovering")
            self.assertEqual(covering["historical_evidence"]["state"],CASE["historical_target"])
            self.assertIn("Pset_HistoricalEvidence",covering["psets"])
        for tag,host in (("DP-1","2"),("DP-2","4")):
            pipe=api.element(tag,m)
            column=api.element(host,m)
            self.assertEqual(pipe["geometry"]["points"][0][:2],column["geometry"]["axis"])
            self.assertEqual(pipe["attributes"]["diameter"]["value"],.08)
        self.assertFalse(any(e["tag"] in ("DP-3","DP-4","PS-1","PS-2","PS-3") for e in m["elements"]))

    def test_mitred_exit_has_connected_empty_bore_and_disjoint_physical_material(self):
        self.pipe["attributes"]["diameter"]["value"]=.08
        self.pipe["geometry"]={"points":[[0,0,1],[0,0,-.6]],"tube":{"wall_thickness":{"value":.003,"unit":"m","basis":"inferred"},"segments":96,"clip_end":{"normal":[-1,0,1],"point":[0,0,-.6]}}}
        outgoing=deepcopy(self.pipe)
        outgoing.update(tag="TEST-OUTLET",guid=api.global_id("TEST-OUTLET"))
        outgoing["geometry"]={"points":[[0,0,-.6],[.3,0,-.6]],"tube":{"wall_thickness":{"value":.003,"unit":"m","basis":"inferred"},"segments":96,"clip_start":{"normal":[-1,0,1],"point":[0,0,-.6]}}}
        self.model["elements"].append(outgoing)
        incoming=api.service_geometry(self.pipe["tag"],self.model)
        outgoing=api.service_geometry(outgoing["tag"],self.model)
        joint=[]
        for index,g in enumerate((incoming,outgoing)):
            meshes=[np.asarray(api.part_vertices(p)) for p in g["parts"]]
            self.assertTrue(all("mesh" in p for p in g["parts"]))
            self.assertAlmostEqual(sum(ConvexHull(v).volume for v in meshes),api.quantities(g["tag"],self.model)["volume"],places=9)
            value=np.concatenate(meshes)@np.array([-1,0,1])+.6
            self.assertTrue((value>=-1e-10).all() if index==0 else (value<=1e-10).all())
            ring=[]
            for mesh in meshes:
                vertices=mesh[np.abs(mesh@np.array([-1,0,1])+.6)<1e-10]
                ring.append(Polygon(vertices[:,:2]))
            joint.append(unary_union(ring))
            for sample in ([0,0,-.55],[0,0,-.6],[.05,0,-.6]):
                self.assertTrue(all(np.max(ConvexHull(v).equations[:,:3]@np.asarray(sample)+ConvexHull(v).equations[:,3])>=-1e-10 for v in meshes))
        self.assertLess(joint[0].symmetric_difference(joint[1]).area,1e-10)
        self.assertFalse(joint[0].contains(Point(0,0)))

    def test_polyhedral_faces_reject_open_or_nonplanar_shells(self):
        vertices=[[0,0,0],[1,0,0],[1,1,0],[0,1,0],[0,0,1],[1,0,1],[1,1,1],[0,1,.9]]
        with self.assertRaises(ValueError):api._polyhedron_part(vertices)
        with self.assertRaises(ValueError):api._polyhedron_part(vertices,[[0,1,2,3]])

    def test_historical_schema_rejects_conflicting_state_and_proposal_as_presence(self):
        import jsonschema
        schema=json.loads((Path(__file__).parents[1]/"🧬️schema/🔣️.schema.json").read_text(encoding="utf-8"))
        state={**schema["properties"]["historical_state"],"$defs":schema["$defs"]}
        self.assertTrue(jsonschema.Draft202012Validator(state).is_valid(self.model["historical_state"]))
        restored=deepcopy(self.model["historical_state"])
        restored["id"]="restored-1997"
        self.assertFalse(jsonschema.Draft202012Validator(state).is_valid(restored))
        evidence={"$ref":"#/$defs/historical_evidence","$defs":schema["$defs"]}
        candidate=deepcopy(self.pipe["historical_evidence"])
        candidate.update(status="original_attested",source_kind="proposal")
        self.assertFalse(jsonschema.Draft202012Validator(evidence).is_valid(candidate))
        for language in ("en","de"):
            item=api.information("DP-1","Pset_HistoricalEvidence.TargetState",language,self.model)
            self.assertEqual(item["value"],CASE["historical_target"])
            self.assertNotEqual(item["label"],"TargetState")

    def test_inferred_cellar_plant_is_supported_and_clear_of_stair(self):
        model=api.load_model()
        boiler=api.service_geometry("BL-1",model)
        self.assertEqual(boiler["storey"],"cellar")
        floor=api._slab_geometry(model,api._find(model,"SL-CELLAR"))
        footprint=unary_union([Polygon(p["outline"]) for p in floor["parts"]])
        self.assertTrue(footprint.covers(Polygon(boiler["parts"][0]["outline"])))
        self.assertAlmostEqual(boiler["parts"][0]["z0"],max(p["z1"] for p in floor["parts"]))
        for stair in [e for e in model["elements"] if e["class"]=="IfcStair"]:
            g=api._stair_geometry(model,stair)
            for part in g["steps"]+g["landings"]:
                height=min(part["z1"],boiler["parts"][0]["z1"])-max(part["z0"],boiler["parts"][0]["z0"])
                if height>0:self.assertLess(Polygon(part["outline"]).intersection(Polygon(boiler["parts"][0]["outline"])).area,1e-10)

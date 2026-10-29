"""🌀 Piecewise wall bands and native station apertures share one physical contract."""

from copy import deepcopy
import importlib
import json
from pathlib import Path
import unittest
from shapely.geometry import LineString, Polygon
from shapely.ops import unary_union

api=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")

class PathWallContract(unittest.TestCase):
    def setUp(self):
        self.case=json.loads(Path(__file__).with_suffix(".json").read_text(encoding="utf-8"))
        self.model=api.load_model()
        wall=deepcopy(api.element("SC-1",self.model))
        wall.update(tag=self.case["host"],guid=api.global_id(self.case["host"]),material="MAT-RENDERED")
        wall["geometry"]={"path":self.case["path"],"base_level":0,"top_level":3}
        wall["attributes"]={"thickness":{"value":.2,"unit":"m","basis":"inferred"}}
        opening=deepcopy(next(e for e in self.model["elements"] if e["class"]=="IfcOpeningElement" and "wall" in e["geometry"]))
        opening.update(tag=self.case["opening"],guid=api.global_id(self.case["opening"]),geometry={"wall":wall["tag"]},attributes={k:{"value":v,"unit":"m","basis":"inferred"} for k,v in self.case["aperture"].items()})
        self.model["elements"] = [wall,opening]
        self.wall,self.opening=wall,opening

    def test_miter_band_matches_independent_geos_buffer(self):
        g=api.path_wall_geometry(self.model,self.wall)
        reference=LineString(self.case["path"]).buffer(.1,cap_style=2,join_style=2)
        for z in (.3,1.,2.7):
            actual=unary_union([Polygon(p["outline"]) for p in g["parts"] if p["z0"]<z<p["z1"]])
            expected=reference if z<.7 or z>2 else reference.difference(LineString([[3.5,0],[4,0],[4,.5]]).buffer(.1,cap_style=2,join_style=2))
            self.assertLess(actual.symmetric_difference(expected).area,1e-10)
            self.assertAlmostEqual(sum(Polygon(p["outline"]).area for p in g["parts"] if p["z0"]<z<p["z1"]),actual.area)
        self.assertAlmostEqual(api.quantities(self.wall["tag"],self.model)["volume"],self.case["expected"]["volume"])

    def test_native_opening_has_thickness_in_every_solid_representation(self):
        for kind in ("axis","footprint","plan","section","elevation","3d","body"):
            self.assertTrue(api.represent(self.opening["tag"],kind,self.model)["primitives"])
        solid=api.represent(self.opening["tag"],"3d",self.model)
        self.assertTrue(all(len(p["polygon"])>=4 and Polygon(p["polygon"]).area>0 for p in solid["primitives"]))
        self.assertAlmostEqual(sum(Polygon(p["polygon"]).area*(p["z1"]-p["z0"]) for p in solid["primitives"]),.26)
        for kind in ("axis","footprint","plan","section","elevation","3d","body"):
            self.assertTrue(api.represent(self.wall["tag"],kind,self.model)["primitives"])

    def test_invalid_station_intervals_are_rejected(self):
        self.opening["attributes"]["end"]["value"]=8
        with self.assertRaisesRegex(ValueError,"station"):
            api.path_wall_geometry(self.model,self.wall)

    def test_released_roof_keeps_source_bounded_native_aperture(self):
        model=api.load_model()
        screen=api.path_wall_geometry(model,api.element("SC-1",model))
        self.assertEqual(len(screen["openings"]),1)
        aperture=screen["openings"][0]
        self.assertAlmostEqual(aperture["sill_z"],7.25)
        self.assertAlmostEqual(aperture["head_z"],8.475)
        self.assertAlmostEqual(screen["top"],9.325)
        self.assertAlmostEqual(screen["body_top"],9.105)
        self.assertGreater(len(screen["path"]),50)
        self.assertTrue(any("top_outline" in p for p in screen["parts"]))

    def test_roof_reinforcements_have_exclusive_material_and_exact_volume(self):
        import numpy as np
        from scipy.spatial import ConvexHull
        model=api.load_model()
        g=api.path_wall_geometry(model,api.element("SC-1",model))
        members=[api._solid_geometry(model,api._find(model,tag)) for tag in ("SC-SILL","SC-HEAD")]
        for part in g["parts"]:
            for member in members:
                for concrete in member["parts"]:
                    height=min(part["z1"],concrete["z1"])-max(part["z0"],concrete["z0"])
                    if height>1e-10:
                        self.assertLess(Polygon(part["outline"]).intersection(Polygon(concrete["outline"])).area*height,1e-10)
        expected=sum(ConvexHull(np.array([[*v,p["z0"]] for v in p["outline"]]+[[*v,p["z1"]] for v in p["top_outline"]])).volume if "top_outline" in p else Polygon(p["outline"]).area*(p["z1"]-p["z0"]) for p in g["parts"])
        self.assertAlmostEqual(api.quantities("SC-1",model)["volume"],expected,places=8)
        self.assertGreater(g["joint_displacement_volume"],0)

"""🧊 Native planar section sweeps preserve nonhorizontal solid geometry."""
from copy import deepcopy
import importlib
import json
from pathlib import Path
import unittest
from scipy.spatial import ConvexHull
from shapely.geometry import Polygon,box

api=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")

class SectionExtrusionContract(unittest.TestCase):
    def test_sections_volumes_vertices_and_every_projection(self):
        cases=json.loads(Path(__file__).with_suffix(".json").read_text(encoding="utf-8"))["cases"]
        for case in cases:
            model=api.load_model()
            obj=next(e for e in model["elements"] if e["tag"]=="FIX-HALL-BASIN")
            obj["geometry"]={"kind":"section-extrusion","base_level":"ground","parts":[case["part"]]}
            part=api.solid_geometry(obj["tag"],model)["parts"][0]
            expected=Polygon(case["section"]).area*(case["part"]["section"]["sweep"][1]-case["part"]["section"]["sweep"][0])
            self.assertAlmostEqual(expected,case["volume"])
            self.assertAlmostEqual(ConvexHull(api.part_vertices(part)).volume,expected)
            self.assertAlmostEqual(api.quantities(obj["tag"],model)["volume"],expected)
            self.assertLess(Polygon(api.part_outline_at(part,case["cut_z"])).symmetric_difference(box(*case["cut_bounds"])).area,1e-10)
            self.assertLess(Polygon(api.part_section(part,sum(case["part"]["section"]["sweep"])/2,1 if case["part"]["section"]["plane"]=="XZ" else 0)).symmetric_difference(Polygon(case["section"])).area,1e-10)
            for kind in ("axis","footprint","plan","section","elevation","3d","body"):
                self.assertTrue(api.represent(obj["tag"],kind,model)["primitives"])
            body=api.represent(obj["tag"],"body",model)["primitives"][0]
            self.assertEqual(body["kind"],"extrusion")
            self.assertEqual(body["profile_plane"],case["part"]["section"]["plane"])
            self.assertEqual(body["direction"],[0,1,0] if case["part"]["section"]["plane"]=="XZ" else [1,0,0])
            solid=api.represent(obj["tag"],"3d",model)["primitives"][0]
            self.assertEqual(solid["kind"],"polyhedron")
            edges={}
            for face in solid["faces"]:
                for a,b in zip(face,face[1:]+face[:1]):
                    edge=tuple(sorted((a,b)))
                    edges[edge]=edges.get(edge,0)+1
            self.assertTrue(all(count==2 for count in edges.values()))
            import numpy as np
            vertices=np.asarray(solid["vertices"])
            signed=sum(np.dot(vertices[face[0]],np.cross(vertices[face[i]],vertices[face[i+1]]))/6 for face in solid["faces"] for i in range(1,len(face)-1))
            self.assertAlmostEqual(signed,expected,places=9)
            u,normal=np.asarray(body["ref_direction"]),np.asarray(body["axis"])
            v=np.cross(normal,u)
            bottom=[np.asarray(body["position"])+h*u+z*v for h,z in body["profile"]["points"]]
            placed=bottom+[p+np.asarray(body["direction"])*body["depth"] for p in bottom]
            np.testing.assert_allclose(sorted(map(tuple,placed)),sorted(map(tuple,vertices)),atol=1e-10)
            import jsonschema
            schema=json.loads((Path(__file__).parents[1]/"🧬️schema/🔣️.schema.json").read_text(encoding="utf-8"))
            jsonschema.Draft202012Validator(schema).validate(model)

    def test_production_plan_section_and_axon_share_exact_geometry(self):
        import ast
        from types import SimpleNamespace
        import numpy as np
        root=Path(__file__).parents[2]/"🎨️visuals/🏛️savoye"
        def definitions(path,names,scope):
            tree=ast.parse(path.read_text(encoding="utf-8"))
            nodes=[n for n in tree.body if isinstance(n,ast.FunctionDef) and n.name in names]
            exec(compile(ast.Module(body=nodes,type_ignores=[]),str(path),"exec"),scope)
        shared=SimpleNamespace(_model=api)
        scope={"np":np,"_model":api,"_c":shared,"EPS":1e-9}
        definitions(root/"🧱️core/📐️geometry.py",{"part_faces"},scope)
        definitions(root/"📋️plan/🐍️.py",{"part_plan_outline"},scope)
        definitions(root/"✂️section/🐍️.py",{"part_projection"},scope)
        model=api.load_model()
        obj=next(e for e in model["elements"] if e["tag"]=="FIX-HALL-BASIN")
        case=json.loads(Path(__file__).with_suffix(".json").read_text(encoding="utf-8"))["cases"][0]
        obj["geometry"]={"kind":"section-extrusion","base_level":"ground","parts":[case["part"]]}
        part=api.solid_geometry(obj["tag"],model)["parts"][0]
        faces=scope["part_faces"](part)
        area=sum(np.linalg.norm(np.cross(np.asarray(face)[i]-face[0],np.asarray(face)[i+1]-face[0]))/2 for face,normal in faces for i in range(1,len(face)-1))
        self.assertAlmostEqual(area,ConvexHull(api.part_vertices(part)).area,places=8)
        plan=scope["part_plan_outline"](part,case["cut_z"])
        self.assertLess(Polygon(plan).symmetric_difference(box(*case["cut_bounds"])).area,1e-10)
        plane=SimpleNamespace(value=.3,index=1)
        cut=scope["part_projection"](part,plane,"cut")
        self.assertLess(Polygon(cut[0]).symmetric_difference(Polygon(case["section"])).area,1e-10)

    def test_invalid_and_ambiguous_source_sections_are_rejected(self):
        import jsonschema
        schema=json.loads((Path(__file__).parents[1]/"🧬️schema/🔣️.schema.json").read_text(encoding="utf-8"))
        model=api.load_model()
        obj=next(e for e in model["elements"] if e["tag"]=="FIX-HALL-BASIN")
        for section in ({"plane":"XZ","profile":[[0,0],[2,0],[1,.5],[2,1],[0,1]],"sweep":[0,1]}, {"plane":"XZ","profile":[[0,0],[1,0],[1,1],[0,1]],"sweep":[1,0]}):
            obj["geometry"]={"kind":"section-extrusion","base_level":"ground","parts":[{"section":section}]}
            with self.assertRaises(ValueError):
                api.solid_geometry(obj["tag"],model)
        obj["geometry"]["parts"][0].update(outline=[[0,0],[1,0],[1,1],[0,1]],z0=0,z1=1)
        self.assertFalse(jsonschema.Draft202012Validator(schema).is_valid(model))

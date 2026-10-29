"""🪟 Native curtain apertures and nonuniform planar pane construction."""
from copy import deepcopy
import importlib,json,math
from pathlib import Path
import unittest
import numpy as np
from shapely.geometry import Polygon
from shapely.ops import unary_union

api=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")

def fixture():
    case=json.loads(Path(__file__).with_suffix(".json").read_text())
    model=api.load_model()
    cw=next(e for e in model["elements"] if e["tag"]=="CW-1")
    cw["geometry"]={"base_level":case["base"],"top_level":case["top"],"glazing_paths":[deepcopy(case["path"])]}
    kind=next(t for t in model["types"] if t["tag"]==cw["type"])
    kind["filling"].pop("panel_stations",None)
    kind["filling"]["transoms"]=deepcopy(case["transoms"])
    opening=next(e for e in model["elements"] if e["tag"]=="OP-D-ENTRY")
    opening["geometry"]={"curtain_wall":"CW-1","curtain_path":"PATH-TEST"}
    opening["attributes"]={k:{"value":v,"unit":"m","basis":"assumed"} for k,v in case["opening"].items()}
    door=next(e for e in model["elements"] if e["tag"]=="D-ENTRY")
    door["type"]="TY-SALON-GLAZED"
    model["elements"]=[e for e in model["elements"] if e["class"]!="IfcOpeningElement" or e["tag"]==opening["tag"]]
    return case,model,cw,opening


def independent_volume(parts):
    levels=sorted({p[k] for p in parts for k in ("z0","z1")})
    return sum(unary_union([Polygon(p["outline"],p.get("holes",[])) for p in parts if p["z0"]<(a+b)/2<p["z1"]]).area*(b-a) for a,b in zip(levels,levels[1:]))


class NativeCurtainContract(unittest.TestCase):
    def test_native_host_domain_and_aperture_share_exact_world_geometry(self):
        case,model,cw,opening=fixture()
        geometry=api.opening_geometry(opening["tag"],model)
        self.assertEqual(geometry["host"],case["expected"]["host"])
        self.assertEqual(geometry["host_class"],case["expected"]["host_class"])
        self.assertAlmostEqual(independent_volume(geometry["parts"]),case["expected"]["aperture_volume"],places=10)
        self.assertAlmostEqual(api.quantities(opening["tag"],model)["volume"],case["expected"]["aperture_volume"],places=10)
        filling=api.filling_geometry("D-ENTRY",model)
        self.assertEqual(filling["host"],"CW-1")
        self.assertEqual(filling["axis"],[[.8,0.],[1.2,0.]])
        for name in ("axis","footprint","plan","section","elevation","3d","body"):
            self.assertTrue(api.represent(opening["tag"],name,model)["primitives"])
            self.assertTrue(api.represent("D-ENTRY",name,model)["primitives"])

    def test_physical_panes_are_planar_and_material_never_fills_the_aperture(self):
        case,model,cw,opening=fixture()
        physical=api.filling_geometry("CW-1",model)
        glass=[p for p in physical["parts"] if p["role"]=="glass"]
        self.assertGreaterEqual(len(glass),case["expected"]["minimum_glass_panels"])
        for part in glass:
            polygon=Polygon(part["outline"])
            self.assertEqual(len(part["outline"]),4)
            self.assertLess(polygon.symmetric_difference(polygon.minimum_rotated_rectangle).area,1e-10)
        aperture=api.opening_geometry(opening["tag"],model)["parts"]
        for a in aperture:
            for part in physical["parts"]:
                height=min(a["z1"],part["z1"])-max(a["z0"],part["z0"])
                if height>1e-12:
                    self.assertLess(Polygon(a["outline"]).intersection(Polygon(part["outline"])).area*height,1e-10)
        self.assertTrue(any(p.get("member_kind")=="transom" for p in physical["parts"]))
        self.assertAlmostEqual(api.quantities("CW-1",model)["volume"],independent_volume(physical["parts"]),places=8)

    def test_source_host_schema_is_exactly_one_native_domain(self):
        import jsonschema
        case,model,cw,opening=fixture()
        schema=json.loads((Path(api.__file__).parent/"🧬️schema/🔣️.schema.json").read_text())
        document={"$schema":schema["$schema"],"$defs":schema["$defs"],"$ref":"#/$defs/element"}
        validator=jsonschema.Draft202012Validator(document)
        validator.validate(opening)
        invalid=deepcopy(opening)
        invalid["geometry"]["wall"]="W-S"
        self.assertTrue(list(validator.iter_errors(invalid)))
        invalid=deepcopy(opening)
        invalid["geometry"].pop("curtain_path")
        self.assertTrue(list(validator.iter_errors(invalid)))

    def test_insulated_metal_wings_do_not_turn_the_timber_leaf_core_into_metal(self):
        case,model,cw,opening=fixture()
        door=next(e for e in model["elements"] if e["tag"]=="D-ENTRY")
        spec=next(t for t in model["types"] if t["tag"]==door["type"])["filling"]
        spec.pop("sash_width",None);spec.pop("sash_depth",None)
        spec["panel_material"]="LS-DOOR-LEAF"
        spec["panel_thickness"]={"value":.035,"unit":"m","basis":"sourced"}
        spec["core_edge_width"]={"value":.04,"unit":"m","basis":"assumed"}
        spec["core_edge_material"]="MAT-WOOD"
        spec["panel_stations"]=[{"value":v,"unit":"1","basis":"assumed"} for v in (0,1)]
        wing=case["side_wings"]
        spec["side_wings"]={**{k:{"value":wing[k],"unit":"m","basis":"assumed"} for k in ("width","depth","skin","recess")},"skin_material":"MAT-METAL","core_material":"MAT-SERVICE-INSULATION"}
        physical=api.filling_geometry("D-ENTRY",model)
        self.assertAlmostEqual(physical["clear_width"],wing["clear_width"],places=10)
        for part in physical["parts"]:
            if part["role"]=="batten":
                self.assertEqual(part["material"],"MAT-WOOD")
        insulation=[p for p in physical["parts"] if p["role"]=="wing_insulation"]
        self.assertEqual(len(insulation),2)
        expected=2*(wing["width"]-2*wing["skin"])*(wing["depth"]-2*wing["skin"])*(case["opening"]["head"]-2*wing["skin"])
        self.assertAlmostEqual(independent_volume(insulation),expected,places=10)
        summed=sum(Polygon(p["outline"]).area*(p["z1"]-p["z0"]) for p in physical["parts"])
        self.assertAlmostEqual(independent_volume(physical["parts"]),summed,places=10)

    def test_building_fillings_are_real_apertures_and_vehicle_curtain_exists_once(self):
        import ast
        case,model,cw,opening=fixture()
        model["elements"]=[e for e in model["elements"] if e["tag"] in ("CW-1","OP-D-ENTRY","D-ENTRY")]
        tree=ast.parse(Path(api.__file__).read_text())
        node=next(n for n in tree.body if isinstance(n,ast.FunctionDef) and n.name=="building_geometry")
        returned=next(n.value for n in node.body if isinstance(n,ast.Return))
        expression=next(value for key,value in zip(returned.keys,returned.values) if isinstance(key,ast.Constant) and key.value=="fillings")
        curtain=api.filling_geometry("CW-1",model)
        environment={"m":model,"_filling_geometry":api._filling_geometry,"curtain_filling":curtain}
        fillings=eval(compile(ast.Expression(expression),str(api.__file__),"eval"),environment)
        self.assertEqual([f["tag"] for f in fillings],case["expected"]["aperture_fillings"])
        self.assertTrue(all(f["host"]=="CW-1" for f in fillings))
        geometry={"fillings":fillings,"curtain_wall":curtain,"ramp":{"ramps":[]}}
        node=next(n for n in tree.body if isinstance(n,ast.FunctionDef) and n.name=="_vehicle_parts")
        node.decorator_list=[]
        environment={"building_geometry":lambda m:geometry,"_registry":api._registry,"deepcopy":deepcopy}
        exec(compile(ast.fix_missing_locations(ast.Module(body=[node],type_ignores=[])),str(api.__file__),"exec"),environment)
        records=environment["_vehicle_parts"](model)
        selected=[r for r in records if r["tag"]=="CW-1"]
        self.assertEqual(len(selected),len(curtain["parts"]))
        self.assertEqual({r["kind"] for r in selected},{case["expected"]["curtain_obstacle_kind"]})

    def test_native_curtain_does_not_duplicate_occupied_hosted_filling_material(self):
        """🚪 GEOS occupied-solid contact includes casing wider and deeper than its rough portal."""
        case=json.loads(Path(__file__).with_suffix('.json').read_text())['hosted_fillings']
        model=api.load_model()
        curtain=api.filling_geometry(case['host'],model)
        def overlap(a,b):
            """📏 Integrate a prism intersection without inferring distance from bounding radii."""
            height=min(a['z1'],b['z1'])-max(a['z0'],b['z0'])
            if height<=0: return 0.
            p,q=Polygon(a['outline'],a.get('holes',[])),Polygon(b['outline'],b.get('holes',[]))
            lo,hi=p.bounds,q.bounds
            if lo[2]<=hi[0] or hi[2]<=lo[0] or lo[3]<=hi[1] or hi[3]<=lo[1]: return 0.
            return p.intersection(q).area*height
        for tag in case['fillings']:
            filling=api.filling_geometry(tag,model)
            maximum=max(overlap(a,b) for a in curtain['parts'] for b in filling['parts'])
            self.assertLess(maximum,1e-12,tag)
        self.assertGreater(overlap(curtain['parts'][0],curtain['parts'][0]),0.)

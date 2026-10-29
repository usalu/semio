"""🏗️ Independent geometric contracts for the source-attested auxiliary facade families."""
import importlib
import json
from pathlib import Path

model=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")
facade=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🏗️facade.🐍️")


def test_profile_sections_match_independent_geos_area_and_sweep():
    """📐 Compare disjoint constructive profiles with independently cut GEOS envelopes."""
    from shapely.geometry import box,Polygon
    from shapely.ops import unary_union
    case=json.loads(Path(__file__).with_suffix(".json").read_text())
    for row in case["profiles"]:
        spec={k:({"value":v} if isinstance(v,(int,float)) else v) for k,v in row.items() if k!="area"}
        pieces=facade.section_rectangles(spec)
        actual=unary_union([box(*r) for r in pieces])
        outer=box(0,0,row["width"],row["height"])
        if row["shape"]=="L":
            expected=outer.difference(box(row["web"],row["toe"],row["width"],row["height"]))
        elif row["shape"]=="U":
            expected=outer.difference(box(row["web"],row["toe"],row["width"]-row["web"],row["height"]))
        else:
            expected=outer
        assert actual.symmetric_difference(expected).area<case["tolerance"]
        assert abs(actual.area-row["area"])<case["tolerance"]
        assert abs(sum(box(*r).area for r in pieces)-actual.area)<case["tolerance"]
        assert abs(actual.area*case["sweep"]-row["area"]*case["sweep"])<case["tolerance"]


def test_source_auxiliary_parts_are_disjoint_and_order_independent():
    """🧩 Compare every auxiliary ownership result with GEOS and a reversed source order."""
    from shapely.geometry import Polygon
    source=model.load_model()
    items=[e for e in source["elements"] if "section_sweep" in e["geometry"]]
    assert items
    reversed_source={**source,"elements":list(reversed(source["elements"]))}
    all_parts=[]
    for obj in items:
        parts=model._solid_geometry(source,obj)["parts"]
        other=model._solid_geometry(reversed_source,obj)["parts"]
        assert parts==other
        assert parts
        expected=sum(Polygon(p["outline"]).area*(p["z1"]-p["z0"]) for p in parts)
        assert abs(model.quantities(obj["tag"],source)["volume"]-expected)<1e-8
        for p in parts:
            for q in all_parts:
                overlap=min(p["z1"],q["z1"])-max(p["z0"],q["z0"])
                assert overlap<=0 or Polygon(p["outline"]).intersection(Polygon(q["outline"])).area*overlap<1e-9
        all_parts+=parts


def test_source_clear_apertures_and_all_representations_survive():
    """🪟 Shared physical members preserve rough apertures, fillings and all seven consumers."""
    from shapely.geometry import Polygon
    source=model.load_model()
    items=[e for e in source["elements"] if "section_sweep" in e["geometry"]]
    for obj in items:
        sweep=obj["geometry"]["section_sweep"]
        opening=model._find(source,sweep["opening"])
        wall=model._find(source,opening["geometry"]["wall"])
        values=model._opening_values(source,opening)
        base=model._z(source,wall["geometry"]["base_level"])
        for p in model._solid_geometry(source,obj)["parts"]:
            if p["z1"]>base+values["sill"]+1e-9 and p["z0"]<base+values["head"]-1e-9:
                assert sweep["role"]=="corner-support"
        for mode in ("axis","footprint","plan","section","elevation","3d","body"):
            assert model.represent(obj["tag"],mode,source)["kind"]==mode


def test_shelf_and_upstand_have_independent_source_thicknesses():
    """🧱 Register the six-centimetre shelf below the fourteen-centimetre clay upstand."""
    source=model.load_model()
    for tag in ("W-S","W-N","W-W","W-E"):
        wall=model._find(source,tag)
        parapet=wall["geometry"]["layer_regions"][0]
        assert abs(parapet["to"]["offset"]["value"]+.20)<1e-9
    shelf=model._find(source,"TY-FACADE-SHELF")["sweep_profile"]
    assert shelf["height"]["value"]==.06
    assert shelf["height"]["basis"]=="sourced"
    assert shelf["width"]["basis"]=="inferred"


def test_garden_topology_keeps_open_frame_and_covered_kiosk_distinct():
    """🌿 Keep the open hanging garden unfilled while retaining enclosed salon and kiosk glazing."""
    source=model.load_model()
    fill={e["geometry"]["opening"] for e in source["elements"] if e["class"] in ("IfcWindow","IfcDoor")}
    assert "OP-GARDEN-E" not in fill
    assert {"OP-E","OP-KIOSK-E"}<=fill
    assert model._find(source,"W-E")["geometry"]["path"][1][1]==6.081
    assert model._find(source,"W-GARDEN-E")["attributes"]["thickness"]["value"]==.16
    assert model._find(source,"W-KIOSK-E")["geometry"]["top_level"]=="soffit:SL-KIOSK"
    assert model._find(source,"FACADE-L-KITCHEN-COURT")["type"]=="TY-FACADE-PLAIN16"
    for tag in ("OP-S","OP-N","OP-E","OP-GARDEN-E","OP-KIOSK-E"):
        opening=model._find(source,tag)
        assert opening["attributes"]["sill"]["value"]==.89
        assert opening["attributes"]["head"]["value"]==2.04


def test_auxiliary_concrete_metal_and_material_leaves_never_share_volume():
    """🧱 GEOS intersections independently audit occupied physical materials at facade joints."""
    from shapely.geometry import Polygon
    source=model.load_model()
    walls=[model._find(source,tag) for tag in ("W-S","W-E","W-N","W-W","W-GARDEN-E","W-KIOSK-E")]
    auxiliary=[e for e in source["elements"] if "section_sweep" in e["geometry"]]
    def overlap(p,q):
        height=min(p["z1"],q["z1"])-max(p["z0"],q["z0"])
        return 0 if height<=1e-10 else Polygon(p["outline"]).intersection(Polygon(q["outline"])).area*height
    for wall in walls:
        owned=model._wall_geometry(source,wall)["parts"]
        members=[p for e in auxiliary if e["geometry"]["section_sweep"]["wall"]==wall["tag"] for p in model._solid_geometry(source,e)["parts"]]
        assert all(overlap(p,q)<1e-9 for p in owned for q in members)
    for tag in ("FACADE-U-S","FACADE-U-N"):
        obj=model._find(source,tag)
        raw=facade.raw_parts(model,source,obj)
        actual=model._solid_geometry(source,obj)["parts"]
        assert len(raw)==len(actual)==3
        assert abs(sum(Polygon(p["outline"]).area*(p["z1"]-p["z0"]) for p in raw)-sum(Polygon(p["outline"]).area*(p["z1"]-p["z0"]) for p in actual))<1e-9


def test_support_graph_distinguishes_corners_lower_stiffeners_and_cavity_ties():
    """🕸️ Keep corner bearings, shelf posts and upper hanging ties semantically distinct."""
    source=model.load_model()
    for obj in source["elements"]:
        sweep=obj["geometry"].get("section_sweep")
        if not sweep:
            continue
        for tag in sweep["supports"]+sweep.get("supported_by",[]):
            model._find(source,tag)
        if sweep["role"]=="parapet-stiffener":
            assert sweep["to"]["datum"]=="sill"
            assert sweep["to"]["offset"]["value"]==-.20
            assert sweep["supported_by"]==["SL-1"]
        if sweep["role"]=="corner-support":
            assert sweep["to"]["datum"]=="head"
        if sweep["role"]=="hanging-tie":
            assert sweep["from"]["datum"]=="head"
            assert sweep["to"]["datum"]=="top"
            assert sweep["supports"]
        if sweep["role"]=="inner-leaf-support":
            assert "Suspension" in obj["psets"]["Pset_ArchiveSource"]


def _local_primary_reference(raw,owned,primary):
    """🧮 Apply exact set identities to bbox-local primary candidates without a global primary union."""
    from shapely.ops import unary_union
    from shapely.strtree import STRtree
    assert all(p.is_valid and p.area>0 for p in raw+owned+primary)
    tree=STRtree(primary);remainders=[];common=[]
    for polygon in raw:
        remainder=polygon
        for index in sorted(int(i) for i in tree.query(polygon)):
            remainder=remainder.difference(primary[index])
        remainders.append(remainder)
    for polygon in owned:
        for index in sorted(int(i) for i in tree.query(polygon)):
            intersection=polygon.intersection(primary[index])
            if intersection.area>0:common.append(intersection)
    return unary_union(remainders),unary_union(common).area


def test_auxiliary_union_matches_geos_difference_of_primary_structure():
    """🏛️ GEOS slices certify no primary duplication or lost auxiliary junction volume."""
    from shapely.geometry import Polygon
    from shapely.ops import unary_union
    import jsonschema
    contract=json.loads(Path(__file__).with_suffix('.json').read_text())
    jsonschema.validate(contract,json.loads(Path(__file__).with_name('🏗️facade.schema.json').read_text()))
    for case in contract['local_ownership_cases']:
        raw_case,owned_case,primary_case=([Polygon(r[0],r[1:]) for r in case[key]] for key in ('raw','owned','primary'))
        reference,overlap=_local_primary_reference(raw_case,owned_case,primary_case)
        independent=unary_union(raw_case).difference(unary_union(primary_case))
        assert reference.symmetric_difference(independent).area<1e-8
        assert abs(reference.area-case['difference_area'])<1e-8
        assert abs(overlap-case['overlap_area'])<1e-8
        assert abs(overlap-unary_union(owned_case).intersection(unary_union(primary_case)).area)<1e-8
    source=model.load_model()
    raw,owned,primary=[],[],[]
    for obj in source["elements"]:
        if "section_sweep" in obj["geometry"]:
            raw+=facade.raw_parts(model,source,obj)
            owned+=model._solid_geometry(source,obj)["parts"]
        elif obj["class"]=="IfcColumn":
            primary+=model.column_geometry(source,obj)["parts"]
        elif obj["class"]=="IfcSlab":
            primary+=model._slab_geometry(source,obj)["parts"]
        elif obj["class"]=="IfcBeam":
            g=model._solid_geometry(source,obj)
            primary+=g["parts"]+g["shared_parts"]
    lo,hi=min(p["z0"] for p in raw),max(p["z1"] for p in raw)
    levels=sorted({lo,hi,*[p[k] for p in raw+owned+primary for k in ("z0","z1") if lo<p[k]<hi],*[v[1] for p in primary if "section" in p for v in p["section"]["profile"] if lo<v[1]<hi]})
    def polygons_at(parts,z):
        polygons=[]
        for p in parts:
            if p["z0"]<z<p["z1"]:
                ring=model.part_outline_at(p,z)
                if ring:
                    polygons.append(Polygon(ring,p.get("holes",())))
        return polygons
    for low,high in zip(levels,levels[1:]):
        z=(low+high)/2
        raw_polygons,owned_polygons,primary_polygons=(polygons_at(parts,z) for parts in (raw,owned,primary))
        reference,overlap=_local_primary_reference(raw_polygons,owned_polygons,primary_polygons)
        actual=unary_union(owned_polygons)
        assert reference.symmetric_difference(actual).area<1e-8
        assert overlap<1e-8

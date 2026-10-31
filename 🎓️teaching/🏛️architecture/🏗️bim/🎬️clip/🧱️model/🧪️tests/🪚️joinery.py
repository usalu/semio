"""🪚 Independent contracts for physical frame, sash and sliding-track joinery."""
import importlib
import json
from pathlib import Path

model=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")


def test_registered_original_carpentry_dimensions_and_physical_materials():
    """📜 Verify registered original carpentry sections against independent occupied GEOS solids."""
    from jsonschema import Draft202012Validator
    from shapely.geometry import Polygon
    source=model.load_model()
    case=json.loads(Path(__file__).with_name("🪚️source.json").read_text())
    Draft202012Validator(json.loads(Path(__file__).with_name("🪚️source.schema.json").read_text())).validate(case)
    for witness in case['types']:
        spec=next(t['filling'] for t in source['types'] if t['tag']==witness['tag'])
        for key,value in witness['dimensions'].items():
            assert spec[key]['value']==value,(witness['tag'],key,spec[key]['value'],value)
        filling=next(e for e in source['elements'] if e.get('type')==witness['tag'])
        parts=model.filling_geometry(filling['tag'],source)['parts']
        actual=sum(Polygon(p['outline']).area*(p['z1']-p['z0']) for p in parts)
        assert abs(actual-model.quantities(filling['tag'],source)['volume'])<case['tolerance']
    spec=next(t['filling'] for t in source['types'] if t['tag']==case['curtain']['type'])
    rail=next(t for t in spec['transoms'] if t['tag']==case['curtain']['transom'])
    assert rail['height']['value']==case['curtain']['height']
    geometry=model.filling_geometry(case['curtain']['tag'],source)
    rails=[p for p in geometry['parts'] if p.get('member_kind')=='transom']
    assert rails
    width=spec['frame_width']['value']
    assert all(abs((p['z0']+p['z1'])/2-case['curtain']['height'])<case['tolerance'] for p in rails)
    thickness=spec['member_sections']['interior']['thickness']['value']
    assert all(any(abs(p['z1']-p['z0']-dimension)<case['tolerance'] for dimension in (width,thickness)) for p in rails)


def test_sashes_and_mullions_occupy_glass_divisions_without_intersection():
    """🧩 Compare source-neutral nonuniform panel boundaries and materials using GEOS."""
    from shapely.geometry import Polygon
    source=model.load_model()
    case=json.loads(Path(__file__).with_suffix(".json").read_text())
    spec=next(t["filling"] for t in source["types"] if t["tag"]=="TY-RIBBON")
    spec["panel_stations"]=[{"value":v,"unit":"1","basis":"assumed","source":"Joinery-contract"} for v in case["stations"]]
    for key in ("sash_width","sash_depth","mullion_width"):
        spec[key]={"value":case[key],"unit":"m","basis":"assumed","source":"Joinery-contract"}
    filling=next(e for e in source["elements"] if e.get("type")=="TY-RIBBON")
    for key in ("panel_stations","panel_operations","panel_glazing_stations"):
        filling["geometry"].pop(key,None)
    geometry=model._filling_geometry(source,filling)
    parts=geometry["parts"]
    assert len([p for p in parts if p["role"]=="mullion"])==len(case["stations"])-2
    assert len([p for p in parts if p["role"]=="sash"])==4*(len(case["stations"])-1)
    glass=[p for p in parts if p["role"]=="glass"]
    assert len(glass)==len(case["stations"])-1
    for p in glass:
        assert len(p["outline"])==4
        for q in parts:
            if q is p:
                continue
            height=min(p["z1"],q["z1"])-max(p["z0"],q["z0"])
            assert height<=0 or Polygon(p["outline"]).intersection(Polygon(q["outline"])).area*height<case["tolerance"]
    assert all(p["material"]=="MAT-WOOD" for p in parts if p["role"] in ("sash","mullion"))


def test_salon_has_hollow_two_lane_tracks_and_real_sashes():
    """🛤️ Confirm physical track webs and lane guides rather than an operation-only label."""
    source=model.load_model()
    filling=next(e for e in source["elements"] if e.get("type")=="TY-TERRACE-GLAZING")
    geometry=model._filling_geometry(source,filling)
    parts=geometry["parts"]
    assert any(p["role"]=="sash" for p in parts)
    assert len([p for p in parts if p["role"]=="track_web"])==2
    assert len([p for p in parts if p["role"]=="track_guide"])==6
    assert all(p["material"]=="MAT-METAL" for p in parts if p["role"].startswith("track_"))


def test_source_glazed_joinery_parts_and_quantities_agree_with_geos():
    """📦 Independently integrate all source planar glazing solids and material intersections."""
    from shapely.geometry import Polygon
    source=model.load_model();case=json.loads(Path(__file__).with_suffix(".json").read_text());seen=set()
    for filling in source["elements"]:
        if filling["class"] not in ("IfcDoor","IfcWindow"):
            continue
        geometry=model._filling_geometry(source,filling)
        parts=geometry["parts"]
        if not any(p["role"]=="glass" for p in parts):continue
        seen.add(filling["type"])
        volume=sum(Polygon(p["outline"]).area*(p["z1"]-p["z0"]) for p in parts)
        assert abs(model.quantities(filling["tag"],source)["volume"]-volume)<1e-8
        for i,p in enumerate(parts):
            for q in parts[i+1:]:
                height=min(p["z1"],q["z1"])-max(p["z0"],q["z0"])
                assert height<=0 or Polygon(p["outline"]).intersection(Polygon(q["outline"])).area*height<1e-9
    assert set(case['glazed_types'])<=seen
    witness=case['glazed_leaf'];door=next(e for e in source['elements'] if e['tag']==witness['tag'])
    assert door['type']==witness['type']
    spec=next(t['filling'] for t in source['types'] if t['tag']==door['type'])
    assert [p['value'] for p in spec['panel_stations']]==witness['panel_stations']
    parts=model._filling_geometry(source,door)['parts']
    sashes=[p for p in parts if p['role']=='sash']
    assert len(sashes)==witness['sash_parts']
    assert len([p for p in parts if p['role']=='mullion'])==witness['mullion_parts']
    assert all(p['material']==witness['sash_material'] for p in sashes)


def test_occurrence_primary_assemblies_and_secondary_glazing_are_independent():
    """🪟 Verify occurrence hierarchy and independent GEOS volume without a fictitious central mullion."""
    from jsonschema import Draft202012Validator
    from shapely.geometry import Polygon
    source=model.load_model();case=json.loads(Path(__file__).with_name("🪚️panels.json").read_text())
    filling=model._find(source,case["tag"]);spec=model._find(source,filling["type"])["filling"]
    spec.pop("mullion_width",None);spec["glazing_bar_width"]={"value":case["glazing_bar_width"],"unit":"m","basis":"inferred","source":case["source"]}
    quantity=lambda value:{"value":value,"unit":"1","basis":"inferred","source":case["source"]}
    filling["geometry"].update(panel_stations=[quantity(v) for v in case["panel_stations"]],panel_operations=case["panel_operations"],panel_glazing_stations=[[quantity(v) for v in row] for row in case["panel_glazing_stations"]])
    schema=json.loads(Path(model.__file__).with_name("🧬️schema").joinpath("🔣️.schema.json").read_text())
    Draft202012Validator({"$defs":schema["$defs"],"$ref":"#/$defs/geometry"}).validate(filling["geometry"])
    geometry=model.filling_geometry(case["tag"],source);parts=geometry["parts"]
    for role,key in [("glass","glass_parts"),("sash","sash_parts"),("glazing_bar","glazing_bar_parts"),("mullion","mullion_parts")]:
        assert len([p for p in parts if p["role"]==role])==case["expected"][key],role
    for index,operation in enumerate(case["panel_operations"]):
        members=[p for p in parts if p.get("panel_index")==index]
        assert members and all(p["panel_operation"]==operation for p in members)
        assert len([p for p in members if p["role"]=="glass"])==2
    for index,p in enumerate(parts):
        for q in parts[index+1:]:
            height=min(p["z1"],q["z1"])-max(p["z0"],q["z0"])
            assert height<=0 or Polygon(p["outline"]).intersection(Polygon(q["outline"])).area*height<1e-9
    volume=sum(Polygon(p["outline"]).area*(p["z1"]-p["z0"]) for p in parts)
    assert abs(model.quantities(case["tag"],source)["volume"]-volume)<1e-8


def test_registered_ribbon_occurrences_preserve_piers_and_original_open_kiosk():
    """📐 Check registered nonuniform stations, true wall piers and transparent material volumes with GEOS."""
    from shapely.geometry import Polygon
    source=model.load_model();case=json.loads(Path(__file__).with_name("🪚️panels.json").read_text())
    tags={e["tag"] for e in source["elements"]}
    assert case["open_kiosk"]["removed_filling"] not in tags
    assert case["open_kiosk"]["retained_opening"] in tags
    for row in case["ribbon_occurrences"]:
        filling=model._find(source,row["tag"]);opening=model._find(source,row["opening"])
        assert [v["value"] for v in filling["geometry"]["panel_stations"]]==row["stations"]
        assert opening["geometry"]["wall"]==row["host"]
        for key in ("start","end"):
            assert abs(opening["attributes"][key]["value"]-row[key])<1e-9
        geometry=model.filling_geometry(row["tag"],source);parts=geometry["parts"]
        assert len([p for p in parts if p["role"]=="glass"])==len(row["stations"])-1
        assert abs(model.quantities(row["tag"],source)["volume"]-sum(Polygon(p["outline"]).area*(p["z1"]-p["z0"]) for p in parts))<1e-8
        for index,p in enumerate(parts):
            for q in parts[index+1:]:
                height=min(p["z1"],q["z1"])-max(p["z0"],q["z0"])
                assert height<=0 or Polygon(p["outline"]).intersection(Polygon(q["outline"])).area*height<1e-9
    for pier in case["opaque_piers"]:
        axis=0 if pier["world_axis"]=="x" else 1
        wall=model._wall_geometry(source,model._find(source,pier["host"]))
        low,high=pier["interval"]
        owned=[p for p in wall["parts"] if p["z0"]<5.0<p["z1"] and max(v[axis] for v in p["outline"])>low and min(v[axis] for v in p["outline"])<high]
        assert owned,pier


def test_ground_steel_l_and_t_members_have_true_open_sections():
    """🔩 Independently union physical L/T cells and compare true section area with GEOS."""
    from shapely.geometry import box
    from shapely.ops import unary_union
    provider=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🪟️curtain.🐍️")
    case=json.loads(Path(__file__).with_name("🪚️steel.json").read_text())
    for row in case["shapes"]:
        spec={"shape":row["shape"],"thickness":{"value":case["thickness"],"unit":"m","basis":"sourced","source":case["source"]}}
        cells=provider.member_section(case["width"],case["depth"],spec)
        solids=[box(a,c,b,d) for a,b,c,d in cells]
        assert len(cells)==2
        assert abs(unary_union(solids).area-case["area"])<1e-12
        assert solids[0].intersection(solids[1]).area<1e-12
        assert abs((cells[1][0]+cells[1][1])/2-row["web_center"])<1e-12
    source=model.load_model();curtain=model._find(source,"CW-1");spec=model._find(source,curtain["type"])["filling"]
    spec["frame_width"]["value"]=case["width"];spec["frame_depth"]["value"]=case["depth"]
    spec["member_sections"]={kind:{"shape":shape,"thickness":{"value":case["thickness"],"unit":"m","basis":"sourced","source":case["source"]}} for kind,shape in [("perimeter","angle"),("interior","tee")]}
    geometry=model.filling_geometry("CW-1",source)
    assert {p["section_shape"] for p in geometry["parts"] if p["role"]=="frame"}=={"angle","tee"}


def test_registered_steel_profiles_are_limited_to_their_documented_families():
    """📜 Assert real source assignments without turning oak/salon or small service windows into T bars."""
    source=model.load_model();case=json.loads(Path(__file__).with_name("🪚️steel.json").read_text())
    for tag in case["source_types"]:
        spec=model._find(source,tag)["filling"]
        assert spec["frame_width"]["value"]==case["width"]
        assert spec["frame_depth"]["value"]==case["depth"]
        for role,shape in [("perimeter","angle"),("interior","tee")]:
            assert spec["member_sections"][role]["shape"]==shape
            assert spec["member_sections"][role]["thickness"]["value"]==case["thickness"]
    assert model._find(source,"G-N-WIN")["type"]=="TY-GROUND-STEEL"
    assert model._find(source,"G-WIN-1")["type"]=="TY-METAL-WINDOW"
    assert "member_sections" not in model._find(source,"TY-TERRACE-GLAZING")["filling"]
    assert "member_sections" not in model._find(source,"TY-RIBBON")["filling"]


def test_salon_drive_has_a_hollow_rail_chain_crank_and_executed_canopy():
    """⚙️ Verify source-qualified mechanical solids and canopy dimensions with independent GEOS volumes."""
    from shapely.geometry import Polygon
    source=model.load_model();case=json.loads(Path(__file__).with_name("🪚️mechanism.json").read_text())
    for name in ("handrail","chain","crank","canopy"):
        witness=case[name];obj=model._find(source,witness["tag"]);geometry=model._solid_geometry(source,obj)
        assert obj["psets"]["Pset_HistoricalEvidence"]["GeometryScope"]["basis"]=="inferred"
        volume=sum(Polygon(p["section"]["profile"]).area*(p["section"]["sweep"][1]-p["section"]["sweep"][0]) if "section" in p else Polygon(p["outline"]).area*(p["z1"]-p["z0"]) for p in geometry["parts"])
        assert abs(model.quantities(witness["tag"],source)["volume"]-volume)<1e-6
    canopy=model._find(source,case["canopy"]["tag"])
    for key in ("thickness","projection"):
        assert canopy["attributes"][key]["value"]==case["canopy"][key]
        assert canopy["attributes"][key]["basis"]=="sourced"
    rail=model._find(source,case["handrail"]["tag"])
    assert len([p for p in rail["geometry"]["parts"] if p["role"]=="hollow_handrail_shell"])==4
    assert {p["role"] for p in rail["geometry"]["parts"]}=={"hollow_handrail_shell","inferred_fixed_frame_attachment","inferred_fixed_sash_attachment"}
    chain=model._find(source,case["chain"]["tag"])
    assert len(chain["geometry"]["parts"])==4
    assert all(p["role"]=="chain_strand_envelope" for p in chain["geometry"]["parts"])
    crank=model._find(source,case["crank"]["tag"])
    assert {p["role"] for p in crank["geometry"]["parts"]}=={"crank_shaft","crank_arm","crank_handle"}


def test_occurrence_assembly_invalid_schedules_are_refused():
    """🚧 Reject mismatched operation counts and unsorted primary or secondary stations."""
    from copy import deepcopy
    source=model.load_model();case=json.loads(Path(__file__).with_name("🪚️panels.json").read_text())
    quantity=lambda value:{"value":value,"unit":"1","basis":"inferred","source":case["source"]}
    for invalid in case["invalid"]:
        current=deepcopy(source);filling=model._find(current,case["tag"])
        for key,value in invalid.items():
            filling["geometry"][key]=[quantity(v) for v in value] if key=="panel_stations" else [[quantity(v) for v in row] for row in value] if key=="panel_glazing_stations" else value
        try:
            model.filling_geometry(case["tag"],current)
        except ValueError:
            pass
        else:
            raise AssertionError(invalid)


def test_salon_hollow_drive_and_glazing_have_no_shared_occupied_material():
    """🧩 Slice independent GEOS YZ profiles and prove material separation across every height stratum."""
    from shapely.geometry import Polygon,LineString,box
    from shapely.ops import unary_union
    source=model.load_model();case=json.loads(Path(__file__).with_name("🪚️mechanism.json").read_text())
    parts=[p for name in ("handrail","chain","crank") for p in model._solid_geometry(source,model._find(source,case[name]["tag"]))["parts"]]
    parts+=model.filling_geometry("GL-TERRACE",source)["parts"]
    heights=sorted({z for p in parts for z in ([v[1] for v in p["section"]["profile"]] if "section" in p else [p["z0"],p["z1"]])})
    for low,high in zip(heights,heights[1:]):
        z=(low+high)/2;cross=[]
        for p in parts:
            if not p["z0"]<z<p["z1"]:
                continue
            if "section" not in p:
                cross.append(Polygon(p["outline"]))
                continue
            section=p["section"];profile=Polygon(section["profile"]);lo,_,hi,_=profile.bounds
            cut=profile.intersection(LineString([(lo-1,z),(hi+1,z)]))
            for line in ([cut] if cut.geom_type=="LineString" else list(cut.geoms)):
                if line.is_empty:
                    continue
                y0,y1=min(v[0] for v in line.coords),max(v[0] for v in line.coords)
                cross.append(box(section["sweep"][0],y0,section["sweep"][1],y1))
        if cross:
            assert sum(p.area for p in cross)-unary_union(cross).area<1e-10,(low,high)

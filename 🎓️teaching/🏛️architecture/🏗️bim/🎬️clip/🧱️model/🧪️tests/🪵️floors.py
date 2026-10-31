"""🪵 Native architectural floor layer contracts shared by every implementation."""
import importlib,json
from pathlib import Path

a=importlib.import_module("🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️")

def test_native_flooring_is_architecture_and_preserves_material_layers():
    """🧱 Compare independent GEOS layer area and native IFC floor ownership."""
    from copy import deepcopy
    from shapely.geometry import Polygon
    case=json.loads(Path(__file__).with_suffix('.json').read_text());
    from jsonschema import Draft202012Validator
    Draft202012Validator(json.loads(Path(__file__).with_suffix('.schema.json').read_text())).validate(case)
    m=a.load_model();e=deepcopy(case['fixture']);template=deepcopy(a._find(m,'RAIN-KITCHEN-PATIO-9-COVER'))
    e['metadata'].update(author='Atelier (architecture)',revision=1);template.update(e);template['guid']=a.global_id(e['tag']);template['name']={'en':'Neutral floor','de':'Neutraler Boden'};template.pop('system',None);m['elements'].append(template)
    m['models'][0]['element_tags'].append(e['tag'])
    g=a.building_geometry(m);assert any(x['tag']==e['tag'] for x in g['details']);assert not any(x['tag']==e['tag'] for x in g['services'])
    body=a.solid_geometry(e['tag'],m);assert body['predefined_type']=='FLOORING' and body['domains']==['architecture']
    expected=sum(Polygon(p['outline']).area*(p['z1']-p['z0']) for p in e['geometry']['parts']);assert abs(a.quantities(e['tag'],m)['volume']-expected)<1e-9


def test_first_floor_coverings_partition_actual_support_and_contact_walls():
    """📏 Compare actual floor material unions and support contact with independent GEOS."""
    from shapely.geometry import Polygon
    from shapely.ops import unary_union
    from jsonschema import Draft202012Validator
    m=a.load_model();Draft202012Validator(json.loads((Path(a.__file__).parent/'🧬️schema/🔣️.schema.json').read_text())).validate(m)
    roster=set(json.loads(Path(__file__).with_suffix('.json').read_text())['first_materials'])|{'FIN-TERRACE-PAVING','FIN-TERRACE-GRASS','FIN-KITCHEN-PATIO','FIN-FIRST-THRESHOLDS'};floors=[a._find(m,tag) for tag in sorted(roster)];assert len(floors)==17
    slab=a._slab_geometry(m,a._find(m,'SL-1'));bands={}
    for p in slab['full_parts']:
        bands.setdefault(p['z1'],[]).extend(Polygon(r) for r in a.part_regions(p,p['z1']))
    caps={z:unary_union(rings) for z,rings in bands.items()};support=unary_union(list(caps.values()));cuts=[]
    for e in m['elements']:
        if e['class']=='IfcColumn':
            cuts.extend(Polygon(a.part_footprint(p)) for p in a.column_geometry(m,e)['parts'] if p['z0']<3.17-1e-8 and p['z1']>3.14+1e-8)
        if e['class']=='IfcOpeningElement' and e['geometry'].get('slab')=='SL-1':cuts.append(Polygon(a._expand(m,e['geometry']['outline'])))
    beam_tags=json.loads(Path(__file__).with_name("🪵️raised-beams.json").read_text())["beam_tags"]
    for tag in beam_tags:
        for p in a._solid_geometry(m,a._find(m,tag))["parts"]:
            if p["z0"]<3.17-1e-8<p["z1"]:cuts.extend(Polygon(r) for r in a.part_regions(p,3.17-1e-8))
            if 3.08-1e-8<=p["z1"]<=3.17+1e-8:
                cap=unary_union([Polygon(r) for r in a.part_regions(p,p["z1"])]).intersection(support);caps[p["z1"]]=caps.get(p["z1"],Polygon()).union(cap)
    net=support.difference(unary_union(cuts));wearing=[];bedding=[];bottom_contact=[]
    floor_parts=[p for e in floors for p in a.solid_geometry(e["tag"],m)["parts"]];layer_supports={z:unary_union([Polygon(p["outline"]) for p in floor_parts if abs(p["z1"]-z)<1e-9]) for z in {p["z0"] for p in floor_parts}}
    for e in floors:
        assert e['metadata']['domains']==['architecture'] and 'system' not in e
        assert e['guid']==a.global_id(e['tag']) and all(e['tag'] not in mod['element_tags'] for mod in m['models'] if mod['id']=='services')
        g=a.solid_geometry(e['tag'],m);local=[]
        for p in g['parts']:
            poly=Polygon(p['outline']);assert poly.is_valid
            assert abs(p['z1']-3.17)<1e-8 if p['role']=='wearing' else p['z1']<=3.17+1e-8
            (wearing if p['role']=='wearing' else bedding).append(poly)
            bottom_contact.extend(poly.intersection(cap) for z,cap in caps.items() if abs(z-p['z0'])<1e-8)
            if p['role'] in ('bedding','wearing','bonding','planting_substrate'):
                contact=unary_union([cap for z,cap in caps.items() if abs(z-p['z0'])<1e-8]+[layer_supports[p['z0']]]);assert poly.difference(contact).area<1e-7,(e['tag'],p['z0'],poly.difference(contact).area)
            local.append(poly.area*(p['z1']-p['z0']))
        assert abs(sum(local)-a.quantities(e['tag'],m)['volume'])<1e-7,e['tag']
    union=unary_union(wearing);assert abs(sum(p.area for p in wearing)-union.area)<1e-7
    ramp_parts=[p for tag in ('RF-1A','RF-1B') for p in a.ramp_geometry(tag,m)['parts']]+a.solid_geometry('FIN-RAMP-INTERIOR',m)['parts'];occupied=unary_union([Polygon(r) for p in ramp_parts for r in a.part_regions(p,0.)]);walking=net.difference(occupied);assert union.symmetric_difference(walking).area<1e-6,(union.area,walking.area)
    missing_support=net.difference(unary_union(bottom_contact));assert missing_support.area<1e-6,(missing_support.area,missing_support.bounds,missing_support.wkt[:3000])
    walls=[a._wall_geometry(m,e) for e in m['elements'] if e['class']=='IfcWall' and e['storey']=='first']
    for wall in walls:
        for p in wall['parts']:
            if abs(p['z0']-3.17)>1e-7:continue
            supported=Polygon(p['outline']).intersection(net)
            assert supported.difference(union).area<1e-7,wall['tag']


def test_original_ground_wearing_materials_support_layers_and_exposed_cement():
    """🪵 Verify original ground oak/gres grammar and independent occupied layer volumes."""
    from scipy.spatial import ConvexHull
    from shapely.geometry import Polygon
    from shapely.ops import unary_union
    m=a.load_model();case=json.loads(Path(__file__).with_suffix('.json').read_text())['ground'];floors=[a._find(m,tag) for tag in case['required']];assert set(case['required'])<={e['tag'] for e in floors}
    slabs=[a._slab_geometry(m,a._find(m,t)) for t in case['slab_thickness']];caps=[];exposed=[]
    for slab in slabs:
        for p in slab['full_parts']:
            target=caps if abs(p['z1']-case['ground_cap'])<1e-8 else exposed
            target.extend(Polygon(r) for r in a.part_regions(p,p['z1']))
    levelling=[Polygon(p["outline"]) for e in floors for p in e["geometry"]["parts"] if p["role"]=="inferred_indoor_coal_bay_levelling"]
    underlay=[Polygon(p["outline"]) for e in floors for p in e["geometry"]["parts"] if p["role"]=="inferred_continuous_wall_underlay"]
    net=unary_union(caps+levelling);wearing=[]
    for e in floors:
        g=a.solid_geometry(e['tag'],m);assert e['metadata']['domains']==['architecture'];parts=g['parts'];expected=sum(ConvexHull(a.part_vertices(p)).volume if 'mesh' in p else Polygon(p['outline']).area*(p['z1']-p['z0']) for p in parts)
        assert abs(expected-a.quantities(e['tag'],m)['volume'])<1e-7,e['tag']
        for p in parts:
            poly=Polygon(p['outline']);assert poly.is_valid
            if p['role']=='wearing':assert p['z1']<=1e-8 and ('mesh' in p or abs(p['z1'])<1e-8)
            assert poly.difference(net).area<1e-7,(e['tag'],p['role'])
            if p['role']=='wearing':wearing.extend(Polygon(r) for r in a.part_regions(p,0.))
        if e['material']=='MAT-OAK':
            assert e['attributes']['finish_thickness']['value']==case['oak_thickness'] and e['attributes']['strip_width']['value']==case['oak_strip_width']
            expected_roles={'wearing','inferred_batten','inferred_bitumen_batten_bed'}
            if e['tag']=='FIN-SERVANTS-1':expected_roles.update(case['coal_bay_roles'])
            assert {p['role'] for p in parts}==expected_roles
            for p in parts:
                if p['role']=='wearing':assert abs(p['z0']+.025)<1e-8
                if p['role']=='inferred_batten':assert abs(p['z0']+.039)<1e-8 and abs(p['z1']+.025)<1e-8
    union=unary_union(wearing);assert abs(sum(p.area for p in wearing)-union.area)<1e-7
    ramp_parts=[p for tag in ('RF-1A','RF-1B') for p in a.ramp_geometry(tag,m)['parts']]+a.solid_geometry('FIN-RAMP-INTERIOR',m)['parts'];occupied=unary_union([Polygon(r) for p in ramp_parts for r in a.part_regions(p,0.)]);walking=net.difference(occupied).difference(unary_union(underlay))
    assert union.symmetric_difference(walking).area<1e-6,(union.area,walking.area)
    for tag,thickness in case['slab_thickness'].items():assert all(abs(r['thickness']['value']-thickness)<1e-8 for r in a._find(m,tag)['geometry']['regions'])
    assert a._find(m,'FIN-HALL')['psets']['Pset_FloorConstruction']['LayingAngleDegrees']['value']==45
    assert a._find(m,'FIN-LAUNDRY')['material']=='MAT-GRES-OCHRE'
    assert unary_union(exposed).area>20,'Original exposed cement retained beside covered rooms'
    assert a._find(m,'COVER-HOIST')['geometry']['top_offset']['value']==0 and a._find(m,'COVER-COAL')['geometry']['top_offset']['value']==-.13


def test_completed_first_material_schedule_and_versaille_panel_depth():
    """🪵 Preserve source-secured gres families and28mm upper parquet assembly without invented timber split."""
    m=a.load_model();case=json.loads(Path(__file__).with_suffix('.json').read_text())
    for tag,material in case['first_materials'].items():
        e=a._find(m,tag);assert e['material']==material,(tag,e['material'],material)
        if material=='MAT-PARQUET':
            assert e['attributes']['finish_thickness']['value']==.028
            assert {p['role'] for p in e['geometry']['parts']}<={'wearing','bonding','bedding'}
            assert all(abs(p['z0']+.028)<1e-9 for p in e['geometry']['parts'] if p['role']=='wearing')
            assert all(p['material']=='MAT-BITUMEN' for p in e['geometry']['parts'] if p['role']=='bonding')


def test_ground_cellar_roof_existing_finished_material_has_no_duplicate_layer():
    """🏠 Prove cellar cement and sole roof finish owners serve their actual occupied storeys."""
    from shapely.geometry import Polygon
    from shapely.ops import unary_union
    m=a.load_model();case=json.loads(Path(__file__).with_suffix('.json').read_text())['remaining_storeys'];cellar=a._slab_geometry(m,a._find(m,case['cellar_slab']))
    cap=unary_union([Polygon(r) for p in cellar['full_parts'] if abs(p['z1']-case['cellar_top'])<1e-8 for r in a.part_regions(p,p['z1'])]);rooms=unary_union([Polygon(a._expand(m,a._find(m,t)['outline'])) for t in case['cellar_rooms']])
    floor=a.solid_geometry(case['cellar_floor'],m);wear=unary_union([Polygon(p['outline']) for p in floor['parts']]);assert all(abs(p['z0']-case['cellar_top'])<1e-8 and abs(p['z1']-case['cellar_finished_top'])<1e-8 for p in floor['parts'])
    assert next(s for s in m['storeys'] if s['tag']=='cellar')['elevation']['value']==case['cellar_finished_top']
    assert a._find(m,case['cellar_slab'])['material']==case['cellar_structural_material']
    assert wear.difference(cap).area<1e-7 and wear.area>55
    assert min(p[2] for p in a._find(m,'BL-1')['geometry']['points'])==case['cellar_finished_top']
    assert max(p['z1'] for p in a.stair_geometry('ST-CG',m)['steps'])==0.
    slabs=[a._slab_geometry(m,a._find(m,t)) for t in case["roof_slabs"]]
    weather=unary_union([Polygon(r) for slab in slabs for p in slab["finish_parts"] if p["z0"]<case["roof_mid"]<p["z1"] for r in a.part_regions(p,case["roof_mid"])]);assert weather.area>0
    for slab_tag,slab in zip(case["roof_slabs"],slabs):
        materials={r["id"]:r["material"] for r in a._material(m,a._find(m,slab_tag))["layers"]};assert all(p.get("material",materials.get(p["layer"]))==case["roof_material"] for p in slab["finish_parts"]);assert max(p["z1"] for p in slab["finish_parts"])==case["roof_finished_top"]
    assert set(case["roof_coverings"])<={e["tag"] for e in m["elements"] if e["class"]=="IfcCovering" and e.get("predefined_type")=="FLOORING" and e["storey"]=="roof"}
    space=Polygon(a._expand(m,a._find(m,case["roof_room"])["outline"]));support=unary_union([Polygon(r) for slab in slabs for p in slab["full_parts"] for r in a.part_regions(p,p["z1"])]);reserved=[]
    owners={p["owner"] for slab in slabs for p in slab["shared_finish_parts"]}
    for owner in sorted(owners):
        obj=a._find(m,owner);g=a.column_geometry(m,obj) if obj["class"]=="IfcColumn" else a.solid_geometry(owner,m);reserved.extend(Polygon(r) for p in g["parts"] for r in a.part_regions(p,case["roof_mid"]))
    missing=space.intersection(support).difference(weather.union(unary_union(reserved)));assert missing.area<1e-6,("Actual roof material covers supported walking plane",missing.area,missing.bounds)


def test_actual_terrace_paver_depth_and_explicit_small_patio_datum_limit():
    """🧱 Validate source-secured4cm pavers without thinning the source23cm patio structure."""
    from shapely.geometry import Polygon
    m=a.load_model();case=json.loads(Path(__file__).with_suffix('.json').read_text())['pavers'];pavers=a._find(m,case['main']);known=[p for p in pavers['geometry']['parts'] if p['role']=='wearing' and p['material']=='MAT-PAVING']
    assert known and all(abs(p['z1']-p['z0']-case['family_thickness'])<1e-9 for p in known)
    assert sum(Polygon(p['outline']).area for p in known)>80
    small=a._find(m,case['unresolved_small_patio']);assert small['material']=='MAT-PAVING-UNRESOLVED' and 'DatumDiscrepancy' in small['psets']['Pset_FloorConstruction']
    for e in m['elements']:
        if e['class']=='IfcStair':assert all(p['material']=='MAT-GRES-BLACK' for p in e['geometry']['step_parts'])


def test_cellar_wall_slab_interfaces_are_exclusive_and_have_positive_bearing():
    """🏗️ Independently intersect real cellar masonry with the actual stepped overhead structural slabs."""
    from shapely.geometry import Polygon
    m=a.load_model();slabs=[a._slab_geometry(m,a._find(m,t)) for t in ('SL-GROUND','SL-GROUND-CELLAR')];overhead=[p for s in slabs for p in s['full_parts']];contact=0.
    for e in m['elements']:
        if e['class']!='IfcWall' or e['storey']!='cellar':continue
        g=a._wall_geometry(m,e);overlap=0.
        for p in g['parts']:
            shape=Polygon(p['outline'])
            for q in overhead:
                area=shape.intersection(Polygon(q['outline'])).area
                if area<1e-12:continue
                overlap+=area*max(0,min(p['z1'],q['z1'])-max(p['z0'],q['z0']))
                if abs(p['z1']-q['z0'])<1e-8:contact+=area
        assert overlap<1e-7,(e['tag'],'masonry/actual slab material overlap',overlap)
    assert contact>.05,('actual positive overhead bearing area',contact)
            
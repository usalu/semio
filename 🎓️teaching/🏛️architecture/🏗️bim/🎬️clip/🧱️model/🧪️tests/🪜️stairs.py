"""🪜 Verify source-registered compensated stairs and roof access with independent geometry oracles."""
import importlib,json,math
from pathlib import Path
model=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
def contract():
    """📋 Read the portable architectural contract."""
    return json.loads(Path(__file__).with_name('🪜️stairs.json').read_text(encoding='utf-8'))
def test_complete_u_trains_and_registered_curve_centre():
    """📐 Compare complete walking footprints with an independent GEOS U-region."""
    from shapely.geometry import Polygon,Point,box
    from shapely.ops import unary_union
    source=model.load_model();fixture=contract();cx,cy=fixture['registration']['curve_centre'];r=fixture['registration']['outer_tread_radius'];inner=fixture['registration']['inner_radius']
    for case in fixture['cases']:
        obj=next(e for e in source['elements'] if e['tag']==case['tag']);g=model._stair_geometry(source,obj)
        assert len(g['steps'])==2*case['straight']+case['curved']
        assert math.dist(g['centre'],[cx,cy])<.002
        actual=unary_union([Polygon(p['outline']) for p in g['steps']]);circle=Point(cx,cy).buffer(r,resolution=512).difference(Point(cx,cy).buffer(inner,resolution=512)).intersection(box(cx,cy-r,cx+r,cy+r))
        expected=unary_union([circle,box(cx-case['straight']*case['going'],cy-r,cx,cy-inner),box(cx-case['straight']*case['going'],cy+inner,cx,cy+r)])
        assert actual.symmetric_difference(expected).area<.004
        assert g['base']==case['base'] and g['top']==case['top']
        assert g['structural_parts'] and len(g['parts'])==len(g['steps'])+len(g['landings'])+len(g['structural_parts'])
        assert all(p.get('material')=='MAT-TILE' for p in g['steps'])
def test_native_waist_volume_and_continuous_connections():
    """🔷 Check native RC convexity, face contacts and continuous inclined soffits independently."""
    import numpy as np
    from scipy.spatial import ConvexHull
    from shapely.geometry import Polygon
    source=model.load_model()
    for case in contract()['cases']:
        obj=next(e for e in source['elements'] if e['tag']==case['tag']);g=model._stair_geometry(source,obj)
        assert g['structural_parts']
        volume=0.
        for part in g['structural_parts']:
            assert part['material']=='MAT-RC' and 'mesh' in part
            vertices=np.asarray(part['mesh']['vertices']);oracle=ConvexHull(vertices).volume
            assert math.isclose(model._polyhedron_volume(part),oracle,rel_tol=1e-8,abs_tol=1e-10)
            volume+=oracle
        assert volume>0
        rise=(case['top']-case['base'])/len(g['steps'])
        for index,step in enumerate(g['steps']):
            assert math.isclose(step['z1'],case['base']+(index+1)*rise,abs_tol=1e-8)
            point=Polygon(step['outline']).representative_point()
            support=[p for p in g['structural_parts'] if Polygon(p['outline']).covers(point)]
            assert support and any(abs(p['z1']-step['z0'])<1e-8 for p in support)
        assert any(len(set(round(v[2],8) for v in p['mesh']['vertices']))>2 for p in g['structural_parts'])
def test_roof_stair_cubicle_is_materially_hollow_and_has_real_access():
    """🚪 Verify a layered roof enclosure, native hosted doorway and disjoint roof buildup."""
    from shapely.geometry import Polygon
    from shapely.ops import unary_union
    source=model.load_model();fixture=contract()['roof'];registry={e['tag']:e for e in source['elements']}
    assert all(fixture[k] in registry for k in ['wall','opening','door','canopy','weather'])
    wall=registry[fixture['wall']];g=model.path_wall_geometry(source,wall)
    assert math.isclose(g['body_top']-g['base'],fixture['clear_height'],abs_tol=1e-8)
    assert {p['material'] for p in g['parts']}=={'MAT-RENDER','MAT-HOLLOW-CLAY','MAT-PLASTER'}
    occupied=unary_union([Polygon(p['outline']) for p in g['parts'] if p['z0']<7.5<p['z1']]);gross=unary_union([Polygon(p['outline']) for p in g['gross_parts']])
    assert gross.area-occupied.area>.1
    assert math.isclose(g['gross_material_volume']+g['gross_cavity_volume'],g['gross_envelope_volume'],abs_tol=1e-8)
    assert math.isclose(g['gross_material_volume']-g['opening_material_volume']-g['joint_displacement_volume'],g['solid_volume'],abs_tol=1e-8)
    assert g['gross_cavity_volume']>g['cavity_volume']>0
    assert g['openings'][0]['material_volume']<sum(Polygon(p['outline']).area*(p['z1']-p['z0']) for p in g['openings'][0]['parts'])
    assert registry[fixture['opening']]['geometry']['wall']==fixture['wall']
    assert registry[fixture['door']]['geometry']['opening']==fixture['opening']
    assert registry[fixture['canopy']]['material']=='MAT-RC' and registry[fixture['weather']]['material']=='MAT-ROOF-FINISH'

def test_shared_slab_void_is_explicit_and_union_owned():
    """🕳️ Compare isolated native shared holes with GEOS and reject stale or cross-host claims."""
    import copy
    from shapely.geometry import Polygon
    from shapely.ops import unary_union
    source=copy.deepcopy(model.load_model());case=contract()['shared_void']
    measure=lambda value:{'value':value,'unit':'m','basis':'assumed'}
    metadata={'author':'Fixture','revision':1,'domains':['architecture']}
    slab={'guid':model.global_id(case['slab']),'tag':case['slab'],'class':'IfcSlab','predefined_type':'FLOOR','name':{'en':'Shared-hole fixture','de':'Gemeinsame-Öffnung-Testfall'},'storey':'ground','material':'MAT-RC','geometry':{'outline':case['outline'],'voids':case['voids'],'level':'ground','top_offset':measure(0),'regions':[{'tag':'FIXTURE-SHARED-REGION','outline':case['outline'],'thickness':measure(case['thickness']),'top_offset':measure(0),'material':'MAT-RC'}]},'attributes':{},'psets':{},'metadata':metadata}
    opening={'guid':model.global_id(case['opening']),'tag':case['opening'],'class':'IfcOpeningElement','predefined_type':'OPENING','name':{'en':'Shared-opening fixture','de':'Gemeinsame-Aussparung-Testfall'},'storey':'ground','geometry':{'slab':slab['tag'],'outline':case['opening_outline'],'shared_voids':[case['geometric_void']]},'attributes':{},'psets':{},'metadata':metadata}
    source['elements'] += [slab,opening]
    expected=Polygon(case['outline']).difference(unary_union([Polygon(v['outline']) for v in case['voids']]+[Polygon(case['opening_outline'])]))
    parts=model._slab_geometry(source,slab)['parts'];actual=unary_union([Polygon(p['outline']) for p in parts])
    assert all(Polygon(p['outline']).is_valid for p in parts)
    assert actual.symmetric_difference(expected).area<1e-10
    assert math.isclose(actual.area,case['expected_net_area'],abs_tol=1e-10)
    assert math.isclose(model.quantities(slab['tag'],source)['volume'],case['expected_net_volume'],abs_tol=1e-9)
    for declaration in case['invalid_declarations']:
        trial=copy.deepcopy(source);o=next(e for e in trial['elements'] if e['tag']==case['opening']);o['geometry']['shared_voids']=declaration
        try:model._slab_voids(trial,next(e for e in trial['elements'] if e['tag']==case['slab']))
        except ValueError:pass
        else:raise AssertionError('An undeclared, cross-host or disjoint shared void was accepted')

def test_cellar_first_tread_has_real_door_and_standing_approach():
    """🚶 Test a continuous0.8m body route through the inferred doorway to the registered first tread."""
    from shapely.geometry import LineString,Polygon,Point
    source=model.load_model();g=model.building_geometry(source);registry={e['tag']:e for e in source['elements']}
    assert 'W-CELLAR-STAIR-W' not in registry and 'OP-CELLAR-STAIR-LANDING' not in registry
    route=LineString([(5.38,7.6),(5.38,8.8),(5.8,8.8)]);body=route.buffer(.4,cap_style=2)
    for category in ['walls','screens','pilotis','railings','fillings']:
        for item in g[category]:
            for p in item.get('parts',[]):
                if p['z0']<-2.42+1.8 and p['z1']>-2.42+.7:
                    assert body.intersection(Polygon(model.part_footprint(p))).area<1e-8,item['tag']
    stair=next(e for e in g['stairs'] if e['tag']=='ST-CG');first=stair['steps'][0]
    assert Polygon(first['outline']).covers(Point(5.8,8.8).buffer(.04))
    floor=next(e for e in g['slabs'] if e['tag']=='SL-CELLAR')
    assert any(Polygon(p['outline']).covers(Point(5.38,8.8).buffer(.04)) for p in floor['parts'])

def test_original_roof_ramp_guard_is_open_tubular_construction():
    """🛝 Compare native open metal railing and sourced upstand sections with the portable contract."""
    from shapely.geometry import Polygon
    source=model.load_model();case=contract()['ramp_guard'];registry={e['tag']:e for e in source['elements']}
    tube=registry[case['tag']];parts=model.solid_geometry(case['tag'],source)['parts']
    assert tube['material']=='MAT-METAL'
    assert sum(p.get('role')=='inclined_horizontal_rail_envelope' for p in parts)==case['rail_envelopes']
    assert sum(p.get('role')=='inferred_rail_post_envelope' for p in parts)==case['inferred_posts']
    assert all(p.get('material')=='MAT-METAL' for p in parts)
    upstand=next(p for p in registry[case['host']]['geometry']['parts'] if p.get('role')=='upper_arrival_12x28_upstand')
    assert upstand['material']=='MAT-RC'
    section=upstand['section'];profile=Polygon(section['profile'])
    y0,y1=case['y_interval']
    assert math.isclose(section['sweep'][1]-section['sweep'][0],case['upstand_width'],abs_tol=1e-10)
    assert math.isclose(profile.area,(y1-y0)*case['upstand_height'],abs_tol=1e-9)
    assert tube['historical_evidence']['status']=='inferred_reconstruction'

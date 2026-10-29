"""🚬 Source-gated continuous chimney and real roof/floor sleeve contracts."""
import ast,importlib,json,math
from pathlib import Path
from types import SimpleNamespace
import numpy as np
from scipy.spatial import ConvexHull
from shapely.geometry import Point,Polygon,LineString
from shapely.ops import unary_union
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
CASE=json.loads(Path(__file__).with_suffix('.json').read_text())

def test_original_chimney_registration_and_native_sleeves():
    """📍 Preserve the explicitly inferred source-selected axis with independent native sleeves."""
    m=api.load_model();e=api.element(CASE['tag'],m)
    assert e['geometry']['points']==[[*CASE['axis'],CASE['bottom']],[*CASE['axis'],CASE['top']]]
    assert e['class']=='IfcChimney' and e['storey']=='ground'
    assert e['attributes']['nominal_diameter']['basis']=='sourced'
    assert e['attributes']['diameter']['basis']=='inferred'
    assert e['geometry']['tube']['wall_thickness']['basis']=='inferred'
    assert e['geometry']['penetrations']==list(CASE['openings'])
    assert e['historical_evidence']['status']=='inferred_reconstruction'
    assert all(e['historical_evidence']['uncertainty'].get(k) for k in ['en','de'])
    expected=Point(*CASE['axis']).buffer(CASE['sleeve_radius'],quad_segs=24)
    for tag,host in CASE['openings'].items():
        op=api.element(tag,m);assert op['geometry']['slab']==host
        assert Polygon(op['geometry']['outline']).symmetric_difference(expected).area<1e-10
        assert not op['geometry'].get('shared_voids')
        assert any(r['class']=='IfcRelVoidsElement' and r['relating']==host and tag in r['related'] for r in m['relations'])
        assert not any(r['class']=='IfcRelFillsElement' and r['relating']==tag for r in m['relations'])
        assert all(tag in pm['element_tags'] for pm in m['models'] if pm['id'] in ['architecture','services'])
    assert all('OP-FLUE-ROOF' not in str(api.element(t,m)['geometry']) for t in ['SL-STAIR-CANOPY','SL-STAIR-WEATHER'])

def test_chimney_material_matches_independent_annulus_and_hulls():
    """🧪 NumPy, GEOS and SciPy independently preserve the full hollow visible run."""
    m=api.load_model();g=api.service_geometry(CASE['tag'],m)
    expected=Point(*CASE['axis']).buffer(.2,quad_segs=24).difference(Point(*CASE['axis']).buffer(.14,quad_segs=24))
    actual=unary_union([Polygon(p['outline']) for p in g['parts']])
    assert actual.symmetric_difference(expected).area<1e-10
    assert not actual.contains(Point(*CASE['axis']))
    vertices=np.array([v for p in g['parts'] for v in api.part_vertices(p)])
    np.testing.assert_allclose([vertices.min(axis=0),vertices.max(axis=0)],[[6.44,11.3,0],[6.84,11.7,9.49]],atol=1e-10)
    volume=sum(ConvexHull(np.array(api.part_vertices(p))).volume for p in g['parts'])
    assert math.isclose(volume,expected.area*CASE['top'],abs_tol=1e-10)
    assert math.isclose(volume,api.quantities(CASE['tag'],m)['volume'],abs_tol=1e-10)
    assert len(g['parts'])==CASE['segments']

def test_chimney_and_real_hall_route_have_no_occupied_native_clash():
    """🚶 Check all occupied material and the existing800mm hall route against the actual flue."""
    m=api.load_model();g=api.building_geometry(m);pipe=api.service_geometry(CASE['tag'],m)
    shape=unary_union([Polygon(p['outline']) for p in pipe['parts']]);bounds=shape.bounds
    for category in ['walls','screens','slabs','pilotis','beams','foundations','stairs','railings','furnishings','details','fillings','services']:
        for item in g.get(category,[]):
            if item['tag']==CASE['tag']:continue
            for p in item.get('parts',[]):
                if min(CASE['top'],p['z1'])<=max(CASE['bottom'],p['z0'])+1e-10:continue
                ring=api.part_footprint(p)
                if not ring:continue
                xs,ys=zip(*ring)
                if max(xs)<bounds[0] or min(xs)>bounds[2] or max(ys)<bounds[1] or min(ys)>bounds[3]:continue
                assert shape.intersection(Polygon(ring)).area<1e-8,(category,item['tag'])
    route=json.loads(Path(__file__).with_name('🏠ground.json').read_text())['northern_circulation']
    body=LineString(route['path']).buffer(route['body_width']/2,cap_style=2)
    assert body.intersection(shape).area<1e-10
    for column in g['pilotis']:
        for p in column['parts']:
            if p['z1']<=0 or p['z0']>=CASE['top']:continue
            assert shape.distance(Polygon(api.part_footprint(p)))>=.25-1e-9,column['tag']

def test_actual_production_catalogue_and_storey_projections_keep_spanning_chimney():
    """🎨 Exercise actual catalogue, plan, section and axon code at every occupied storey."""
    h=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.🧪️ortho-remodel.test_frustums')
    m=api.load_model();data=api.service_geometry(CASE['tag'],m);g=api.building_geometry(m)
    root=h.ROOT;scope=h._scope();core={'geometry':lambda model:g,'disciplines_of':lambda *a:['services'],'KIND_CLASSES':{**{k:k for k in g},'curtain':'IfcCurtainWall','ramp':'IfcRamp','openings':'IfcOpeningElement'}}
    exec(compile(ast.Module(body=h._definitions(root/'🧱️core/🐍️.py',{'catalogue'}),type_ignores=[]),str(root),'exec'),core)
    catalogue=core['catalogue'](m);item=next(x for x in catalogue if x['tag']==CASE['tag'])
    assert item['kind']=='services' and item['data']['parts']
    plan=scope['Plan']();plan._pair=lambda *a:(0,0,1,0)
    expected=unary_union([Polygon(p['outline']) for p in data['parts']])
    for level,cut in CASE['cuts'].items():
        recorded=[];ctx={'look':None,'bands':{level:(cut-.5,cut+.5)},'level':level,'window':None,'catalogue':[item],'cut_z':cut,'screen':lambda ring:np.c_[ring,np.zeros(len(ring))],'put':lambda *a:recorded.append(a[-1])}
        plan._plan_services(ctx)
        actual=unary_union([p for group in recorded for p in h._polygons(group) if p.area>1e-12])
        assert actual.symmetric_difference(expected).area<1e-10,level
    section=scope['Section']();section._pair=lambda *a:(0,0,1,0)
    plane=SimpleNamespace(index=0,value=CASE['axis'][0],extent=lambda ring:(min(p[0] for p in ring),max(p[0] for p in ring)),classify=lambda lo,hi:'cut' if lo<=CASE['axis'][0]<=hi else 'removed',hrange=lambda ring:(min(p[1] for p in ring),max(p[1] for p in ring)))
    plane.intervals=lambda rings:scope['profile_cut'](rings[0],0,plane.value)
    recorded=[];section._sec_services({'plane':plane,'to':lambda h,z:np.array([h,z,0]),'look':None,'catalogue':[item],'put':lambda *a:recorded.append(a[-1])})
    actual=unary_union([p for group in recorded for p in h._polygons(group) if p.area>1e-12])
    expected=unary_union([Polygon(ring) for p in data['parts'] for ring in api.part_sections(p,plane.value,0)])
    assert actual.symmetric_difference(expected).area<1e-10
    prims=scope['_service_prims'](data,{},0,9.49)
    volume=sum(ConvexHull(np.array([v for kind,rings,normal,role in prim.faces for ring in rings for v in ring])).volume for prim in prims)
    assert math.isclose(volume,data['volume'],abs_tol=1e-10)

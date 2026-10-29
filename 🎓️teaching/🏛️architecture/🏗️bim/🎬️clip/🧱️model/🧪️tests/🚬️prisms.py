"""🧱 Roof chimney composite material and finish-only ownership contracts."""
from copy import deepcopy
import importlib,json,math
from pathlib import Path
import numpy as np
from scipy.spatial import ConvexHull
from shapely.geometry import Polygon,box,Point
from shapely.ops import unary_union
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
CASE=json.loads(Path(__file__).with_suffix('.json').read_text())

def synthetic():
    """🧪 Isolate a four-part native material owner above a simple regional roof."""
    m=api.load_model();owner=deepcopy(api.element('BOILER-FLUE-VISIBLE',m));owner.update(tag='TEST-CHIMNEY',guid=api.global_id('TEST-CHIMNEY'),storey='roof',material='MAT-HOLLOW-CLAY')
    owner['geometry']={'kind':'prismatic_chimney','axis':[0,0],'base_level':'roof','parts':deepcopy(CASE['synthetic_parts'])};owner['attributes']={}
    slab=deepcopy(api.element('SL-R',m));slab.update(tag='TEST-SLAB',guid=api.global_id('TEST-SLAB'))
    slab['geometry']={'outline':[[-1,-1],[1,-1],[1,1],[-1,1]],'voids':[],'level':'roof','top_offset':{'value':-.09,'unit':'m','basis':'inferred'},'regions':[{'tag':'TEST-REGION','outline':[[-1,-1],[1,-1],[1,1],[-1,1]],'thickness':{'value':.17,'unit':'m','basis':'inferred'},'top_offset':{'value':-.09,'unit':'m','basis':'inferred'},'material':'MAT-RC'}],'finish_voids':[{'tag':'TEST-FINISH-RESERVE','owner':owner['tag'],'outline':[[-.175,-.175],[.175,-.175],[.175,.175],[-.175,.175]]}]}
    opening=deepcopy(api.element('OP-FLUE-ROOF',m));opening.update(tag='TEST-BORE',guid=api.global_id('TEST-BORE'));opening['geometry']={'slab':slab['tag'],'outline':[[-.075,-.075],[.075,-.075],[.075,.075],[-.075,.075]]}
    m['elements'] += [owner,slab,opening]
    return m,owner,slab

def test_native_composite_service_and_finish_owner_partition():
    """📐 Independent GEOS and SciPy verify empty bore, full bearing and exact displaced finish."""
    m,owner,slab=synthetic();pipe=api.service_geometry(owner['tag'],m);g=api._slab_geometry(m,slab)
    ring=box(-.175,-.175,.175,.175).difference(box(-.075,-.075,.075,.075))
    shape=unary_union([Polygon(p['outline']) for p in pipe['parts']]);assert shape.symmetric_difference(ring).area<1e-12
    assert math.isclose(sum(ConvexHull(np.array(api.part_vertices(p))).volume for p in pipe['parts']),.109,abs_tol=1e-10)
    structural=unary_union([Polygon(p['outline']) for p in g['parts'] if math.isclose(p['z1'],6.51,abs_tol=1e-10)])
    assert math.isclose(structural.intersection(ring).area,.1,abs_tol=1e-10)
    finish=unary_union([Polygon(p['outline']) for p in g['finish_parts']]);assert finish.intersection(ring).area<1e-10
    shared=g['shared_finish_parts'];assert math.isclose(sum(api._area(p['outline'])*(p['z1']-p['z0']) for p in shared),.009,abs_tol=1e-10)
    assert all(p['owner']==owner['tag'] and p['material']=='MAT-HOLLOW-CLAY' for p in shared)
    exclusive=sum(api._area(p['outline'])*(p['z1']-p['z0']) for p in g['finish_parts'])
    full=sum(api._area(p['outline'])*(p['z1']-p['z0']) for p in g['full_finish_parts'])
    assert math.isclose(exclusive+.009,full,abs_tol=1e-10)
    assert not any(Polygon(p['outline']).contains(Point(0,0)) for p in g['parts']+g['finish_parts'])

def test_finish_reserve_rejects_missing_or_incomplete_native_owner():
    """🚧 A named reserve cannot silently delete weathering without matching material ownership."""
    for defect in ['absent','incomplete']:
        m,owner,slab=synthetic()
        if defect=='absent':slab['geometry']['finish_voids'][0]['owner']='MISSING'
        else:owner['geometry']['parts'].pop()
        try:api._slab_geometry(m,slab)
        except (ValueError,KeyError):pass
        else:raise AssertionError(defect+' accepted')

def test_actual_registered_roof_chimney_sections_and_support():
    """🏛️ The two modern-author registered terminals use bounded roof-only native hollow construction."""
    m=api.load_model();slab=api._slab_geometry(m,api.element('SL-R',m))
    for case in CASE['terminals']:
        obj=api.element(case['tag'],m);g=api.service_geometry(case['tag'],m)
        ring=box(case['axis'][0]-.175,case['axis'][1]-.175,case['axis'][0]+.175,case['axis'][1]+.175).difference(box(case['axis'][0]-.075,case['axis'][1]-.075,case['axis'][0]+.075,case['axis'][1]+.075))
        assert obj['class']=='IfcChimney' and obj['storey']=='roof'
        assert len(g['parts'])==12
        actual=unary_union([Polygon(p['outline']) for p in g['parts']]);assert actual.symmetric_difference(ring).area<1e-10
        assert min(p['z0'] for p in g['parts'])==CASE['structural_top']
        assert max(p['z1'] for p in g['parts'])==case['top']
        hull=sum(ConvexHull(np.array(api.part_vertices(p))).volume for p in g['parts']);assert math.isclose(hull,case['material_volume'],abs_tol=1e-10)
        assert math.isclose(api.quantities(case['tag'],m)['volume'],hull,abs_tol=1e-10)
        materials={q['material']:q['volume'] for q in api.quantities(case['tag'],m)['material_quantities']}
        for layer in CASE['layers']:
            assert math.isclose(materials[layer['material']],(layer['outer']**2-layer['inner']**2)*(case['top']-6.51),abs_tol=1e-10)
        bearing=unary_union([Polygon(p['outline']) for p in slab['parts'] if math.isclose(p['z1'],6.51,abs_tol=1e-10)])
        assert ring.difference(bearing).area<1e-10
        assert sum(ring.intersection(Polygon(p['outline'])).area for p in slab['finish_parts'])<1e-9
        opening=api.element(case['opening'],m);assert opening['geometry']['slab']=='SL-R'
        assert math.isclose(Polygon(opening['geometry']['outline']).area,.15**2,abs_tol=1e-10)
        assert not obj['geometry'].get('points')
        assert obj['historical_evidence']['status']=='inferred_reconstruction'

def test_production_roof_profile_and_above_cut_wire_keep_material_scope():
    """🎨 Roof cuts show the ring; a neck above a first-floor cut remains an outline."""
    h=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.🧪️ortho-remodel.test_frustums')
    m=api.load_model();scope=h._scope();scene=scope['Plan']();scene._pair=lambda *a:(0,0,1,0)
    for case in CASE['terminals']:
        data=api.service_geometry(case['tag'],m);item={'kind':'services','data':data,'tag':case['tag']}
        for level,band,cut,area in [('first',(3.17,6.6),4.37,0),('roof',(6.6,9.6),7.8,.1)]:
            recorded=[];ctx={'look':None,'bands':{level:band},'level':level,'window':None,'catalogue':[item],'cut_z':cut,'screen':lambda ring:np.c_[ring,np.zeros(len(ring))],'put':lambda *a:recorded.append(a[-1])}
            scene._plan_services(ctx)
            actual=unary_union([p for group in recorded for p in h._polygons(group) if p.area>1e-12])
            assert math.isclose(actual.area,area,abs_tol=1e-10),(case['tag'],level,actual.area)
        for mode in ['axis','footprint','plan','section','elevation','3d','body']:
            assert api.represent(case['tag'],mode,m)['primitives']

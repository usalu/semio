"""🚪 Verify the original external timber solarium leaf and registered access side."""
import importlib,json
from pathlib import Path
from shapely.geometry import LineString,Polygon
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
CASE=json.loads(Path(__file__).with_suffix('.json').read_text())

def test_roof_exit_leaf_has_its_own_external_timber_evidence():
    """🪵 The roof leaf cannot inherit another salon door's glass or attribution."""
    model=api.load_model()
    door=next(e for e in model['elements'] if e['tag']==CASE['door'])
    kind=next(t for t in model['types'] if t['tag']==door['type'])
    assert kind['material']==CASE['leaf']['material']
    assert kind['filling']['panel_material']==CASE['leaf']['material']
    assert kind['psets']['Pset_DoorCommon']['IsExternal']['value'] is CASE['leaf']['is_external']
    evidence=json.dumps(kind,ensure_ascii=False)
    assert all(text not in evidence for text in CASE['leaf']['forbidden_evidence'])
    layout=kind['filling']['panel_layout']
    expected=CASE['candidate']['layout']
    normalized=lambda cell:(cell['tag'],tuple(q['value'] for q in cell['u']),tuple(q['value'] for q in cell['v']),cell['material'],cell['thickness']['value'],cell['operation'])
    assert [normalized(cell) for cell in layout['cells']]==[normalized(cell) for cell in expected['cells']]
    assert all(q['basis']=='inferred' for cell in layout['cells'] for q in [*cell['u'],*cell['v'],cell['thickness']])
    geometry=api.filling_geometry(CASE['door'],model)
    leaves=[p for p in geometry['parts'] if p['role']=='panel']
    assert leaves and {p['material'] for p in leaves}==set(CASE['leaf']['face_materials'])
    assert any(p['role']=='batten' and p['material']==CASE['leaf']['batten_material'] for p in geometry['parts'])
    assert not any(p['material']=='MAT-AIR' for p in geometry['parts'])

def test_roof_access_is_on_the_registered_vertical_host_segment():
    """🧭 Public opening material must reach the author-plan side, not the north wall."""
    model=api.load_model()
    opening=api.opening_geometry(CASE['opening'],model)
    assert opening['host']==CASE['host']
    polygons=[Polygon(p['outline']) for p in opening['parts']]
    bounds=[p.bounds for p in polygons]
    x0,y0,x1,y1=min(b[0] for b in bounds),min(b[1] for b in bounds),max(b[2] for b in bounds),max(b[3] for b in bounds)
    register=CASE['register']
    line=LineString([(register['x'],register['y_interval'][0]),(register['x'],register['y_interval'][1])])
    assert all(poly.distance(line)<=register['uncertainty'] for poly in polygons)
    assert y1-y0>x1-x0
    assert y0>=register['y_interval'][0]-register['uncertainty']
    assert y1<=register['y_interval'][1]+register['uncertainty']

def synthetic_layout():
    """🧩 Test the material-cell interface on an independent two-metre straight host."""
    from copy import deepcopy
    model=deepcopy(api.load_model())
    host=next(e for e in model['elements'] if e['tag']==CASE['host'])
    host['geometry']['path']=[[0.,0.],[2.,0.]]
    opening=next(e for e in model['elements'] if e['tag']==CASE['opening'])
    for key,value in {'start':0.,'end':2.,'sill':0.,'head':CASE['candidate']['head']}.items():
        opening['attributes'][key]['value']=value
    kind=next(e for e in model['types'] if e['tag']==CASE['type'])
    spec=kind['filling']
    for key in ('panel_stations','sash_width','sash_depth','mullion_width','side_wings','track_web'):
        spec.pop(key,None)
    spec.update(operation='mixed',panel_material='LS-DOOR-LEAF',core_edge_width={'value':.04,'unit':'m','basis':'inferred'},core_edge_material='MAT-WOOD',panel_thickness={'value':.035,'unit':'m','basis':'inferred'},panel_layout=deepcopy(CASE['candidate']['layout']))
    return model

def test_mixed_filling_cells_conserve_independent_material_and_clear_leaf():
    """📐 GEOS and SciPy independently integrate non-overlapping native material cells."""
    import math,numpy as np
    from scipy.spatial import ConvexHull
    from shapely.geometry import box
    from shapely.ops import unary_union
    model=synthetic_layout()
    geometry=api.filling_geometry(CASE['door'],model)
    spec=next(t for t in model['types'] if t['tag']==CASE['type'])['filling']
    w,d,b=spec['frame_width']['value'],spec['frame_depth']['value'],CASE['candidate']['layout']['member_width']['value']
    x0,x1,z0,z1=w,2-w,6.6,6.6+CASE['candidate']['head']-w
    cells=[];bars=[]
    for cell in CASE['candidate']['layout']['cells']:
        u0,u1=[x0+(x1-x0)*q['value'] for q in cell['u']]
        v0,v1=[z0+(z1-z0)*q['value'] for q in cell['v']]
        cells.append((cell,box(u0,v0,u1,v1)))
        for u in (u0,u1):
            if x0<u<x1: bars.append(box(u-b/2,v0,u+b/2,v1))
        for v in (v0,v1):
            if z0<v<z1: bars.append(box(u0,v-b/2,u1,v+b/2))
    metal=unary_union(bars).intersection(box(x0,z0,x1,z1))
    expected={'MAT-METAL':(2*CASE['candidate']['head']-(x1-x0)*(z1-z0)+metal.area)*d}
    leaf=None
    for cell,area in cells:
        occupied=area.difference(metal)
        material=next(m for m in model['materials'] if m['tag']==cell['material'])
        for layer in material.get('layers',[{'material':cell['material'],'thickness':cell['thickness']}]):
            if layer.get('is_void'):
                area=occupied.area-occupied.buffer(-spec['core_edge_width']['value'],join_style='mitre').area
                tag=spec['core_edge_material']
            else:
                area,tag=occupied.area,layer['material']
            expected[tag]=expected.get(tag,0.)+area*layer['thickness']['value']
        if cell['operation']!='fixed': leaf=occupied
    actual={}
    bounds=[]
    for part in geometry['parts']:
        vertices=api.part_vertices(part)
        volume=ConvexHull(np.asarray(vertices)).volume
        actual[part['material']]=actual.get(part['material'],0.)+volume
        bounds.append((np.min(vertices,axis=0),np.max(vertices,axis=0)))
    for material,volume in expected.items(): assert math.isclose(actual[material],volume,abs_tol=1e-11)
    for i,(low,high) in enumerate(bounds):
        for low2,high2 in bounds[i+1:]: assert np.prod(np.maximum(0,np.minimum(high,high2)-np.maximum(low,low2)))<1e-12
    assert math.isclose(geometry['clear_width'],leaf.bounds[2]-leaf.bounds[0],abs_tol=1e-12)
    assert math.isclose(geometry['clear_height'],leaf.bounds[3]-leaf.bounds[1],abs_tol=1e-12)
    for representation in ('plan','section','elevation','3d','axis','footprint','body'):
        result=api.represent(CASE['door'],representation,model)
        assert result['key']==CASE['door'] and result['primitives']

def test_mixed_filling_rejects_uncovered_or_overlapping_layout_cells():
    """🧱 Missing and doubled material domains fail rather than being silently filled."""
    from copy import deepcopy
    for change in ('gap','overlap'):
        model=synthetic_layout()
        spec=next(t for t in model['types'] if t['tag']==CASE['type'])['filling']
        if change=='gap': spec['panel_layout']['cells'].pop()
        else: spec['panel_layout']['cells'].append(deepcopy(spec['panel_layout']['cells'][0]))
        try: api.filling_geometry(CASE['door'],model)
        except ValueError: pass
        else: raise AssertionError('Invalid material partition accepted: '+change)

def test_corner_aperture_keeps_finite_station_end_planes():
    """📏 An opening at a host corner ends at its authored station plane, not the wall miter."""
    from copy import deepcopy
    from shapely.ops import unary_union
    model=synthetic_layout()
    case=CASE['corner_aperture']
    host=next(e for e in model['elements'] if e['tag']==CASE['host'])
    host['geometry']['path']=deepcopy(case['path'])
    host['geometry']['path_offset']['value']=case['offset']
    host['attributes']['thickness']['value']=case['thickness']
    opening=next(e for e in model['elements'] if e['tag']==CASE['opening'])
    for key in ('start','end'): opening['attributes'][key]['value']=case[key]
    result=api.opening_geometry(CASE['opening'],model)
    actual=unary_union([Polygon(part['outline']) for part in result['parts']])
    expected=Polygon(case['outline'])
    assert actual.symmetric_difference(expected).area<1e-12
    wall=api.path_wall_geometry(model,host)
    occupied=unary_union([Polygon(p['outline']) for p in wall['parts']+wall['cavity_parts'] if p['z0']<7.<p['z1']])
    gross=Polygon([[-.25,-.25],[1.25,-.25],[1.25,1.25],[-.25,1.25]],holes=[[[0,0],[1,0],[1,1],[0,1]]])
    assert occupied.symmetric_difference(gross.difference(expected)).area<1e-10
    assert occupied.intersection(Polygon([[-.25,1],[-.125,1],[-.125,1.25],[-.25,1.25]])).area>.02

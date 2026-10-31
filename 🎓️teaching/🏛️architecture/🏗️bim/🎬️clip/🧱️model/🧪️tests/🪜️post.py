"""🪜 Native plane-capped profiles and the original L stair support bearing."""
from copy import deepcopy
import importlib,json,math
from pathlib import Path
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
CASE=json.loads(Path(__file__).with_suffix('.json').read_text())

def synthetic():
    """🧪 Place one source-neutral profile with a declared sloping upper end plane."""
    m=api.load_model();obj=next(e for e in m['elements'] if e['tag']=='19');case=CASE['synthetic'];kind=next(t for t in m['types'] if t['tag']==obj['type'])
    kind['profile']={'shape':'polygon','outline':deepcopy(case['outline'])}
    obj['geometry']={'axis':[0.,0.],'base_level':case['base'],'top_level':case['top'],'end_plane':deepcopy(case['plane'])}
    return m,obj

def test_native_column_end_plane_preserves_closed_mesh_and_exact_volume():
    """📏 Compare a sloping cap with independent plane integration and SciPy hull volume."""
    import numpy as np
    from scipy.spatial import ConvexHull
    m,obj=synthetic();g=api.column_geometry(m,obj);n=np.asarray(CASE['synthetic']['plane']['normal']);origin=np.asarray(CASE['synthetic']['plane']['point']);parts=g['parts']
    assert all('mesh' in p for p in parts)
    vertices=[v for p in parts for v in api.part_vertices(p)]
    assert all(abs(v[2]-CASE['synthetic']['base'])<1e-10 or abs(np.dot(n,np.asarray(v)-origin))<1e-10 for v in vertices)
    assert math.isclose(sum(ConvexHull(np.asarray(api.part_vertices(p))).volume for p in parts),CASE['synthetic']['volume'],abs_tol=CASE['tolerance'])
    assert math.isclose(api.quantities(obj['tag'],m)['volume'],CASE['synthetic']['volume'],abs_tol=CASE['tolerance'])
    mesh=api.mesh(obj['tag'],model=m)
    from collections import Counter
    edges=Counter(tuple(sorted((face[i],face[(i+1)%3]))) for face in mesh['faces'] for i in range(3))
    assert edges and set(edges.values())=={2}
    assert math.isclose(abs(sum(np.dot(np.asarray(mesh['vertices'][f[0]]),np.cross(np.asarray(mesh['vertices'][f[1]]),np.asarray(mesh['vertices'][f[2]])))/6 for f in mesh['faces'])),CASE['synthetic']['volume'],abs_tol=1e-10)

def test_native_column_end_plane_rejects_parallel_or_nonpositive_caps():
    """🚧 Reject upper planes that cannot bound a positive vertical profile extrusion."""
    for plane in ({'normal':[1,0,0],'point':[0,0,3]},{'normal':[0,0,0],'point':[0,0,3]},{'normal':[0,0,1],'point':[0,0,-1]}):
        m,obj=synthetic();obj['geometry']['end_plane']=plane
        try:api.column_geometry(m,obj)
        except ValueError:pass
        else:raise AssertionError('Invalid column cap accepted: '+str(plane))

def test_original_l_support_meets_actual_structural_guard_in_positive_area():
    """🏗️ Independently integrate the source L profile and actual sloped concrete bearing faces."""
    import numpy as np
    from scipy.spatial import ConvexHull
    from shapely.geometry import Polygon
    from shapely.ops import unary_union
    m=api.load_model();case=CASE['source'];assert any(e['tag']==case['tag'] for e in m['elements'])
    obj=api.element(case['tag'],m);g=api.column_geometry(m,obj);n=np.asarray(case['plane']['normal']);origin=np.asarray(case['plane']['point'])
    expected=Polygon([[x+case['axis'][0],y+case['axis'][1]] for x,y in case['outline']]);footprint=unary_union([Polygon(p['outline']) for p in g['parts']])
    assert expected.symmetric_difference(footprint).area<CASE['tolerance']
    assert math.isclose(expected.area,.0228,abs_tol=1e-12)
    independent=sum(ConvexHull(np.asarray(api.part_vertices(p))).volume for p in g['parts'])
    assert math.isclose(independent,case['volume'],abs_tol=CASE['tolerance'])
    assert math.isclose(api.quantities(case['tag'],m)['volume'],independent,abs_tol=CASE['tolerance'])
    guard=api.solid_geometry(case['guard'],m);contacts=[]
    for part in guard['parts']:
        if part.get('material')!='MAT-RC' or expected.intersection(Polygon(part['outline'])).area<1e-12:continue
        vs=api.part_vertices(part)
        assert min(np.dot(n,np.asarray(v)-origin) for v in vs)>=-CASE['tolerance']
        for face in api.part_faces(part):
            ps=[vs[i] for i in face]
            if all(abs(np.dot(n,np.asarray(v)-origin))<CASE['tolerance'] for v in ps):
                ring=Polygon([[v[0],v[1]] for v in ps])
                if ring.area>1e-12:contacts.append(ring)
    assert math.isclose(unary_union(contacts).intersection(expected).area,case['projected_bearing'],abs_tol=CASE['tolerance'])
    assert obj['historical_evidence']['status']=='inferred_reconstruction'

def test_stair_l_has_positive_floor_bearing_without_a_fictitious_material_reserve():
    """🧱 Compare actual lower face bearing while preserving the slab without invented finish cuts."""
    from shapely.geometry import Polygon
    from shapely.ops import unary_union
    m=api.load_model();case=CASE['source'];slab=api.element('SL-1',m);g=api._slab_geometry(m,slab);column=api.column_geometry(m,api.element(case['tag'],m))
    footprint=unary_union([Polygon(p['outline']) for p in column['parts']])
    bearing=unary_union([Polygon(p['outline']) for p in g['full_parts'] if math.isclose(p['z1'],case['base'],abs_tol=1e-10)])
    assert math.isclose(footprint.intersection(bearing).area,.0096,abs_tol=CASE['tolerance'])
    assert case['tag'] not in slab['geometry'].get('column_reserves',[])
    assert not g['shared_parts'] and not g['finish_parts'] and not g['shared_finish_parts']
    assert math.isclose(sum(Polygon(p['outline']).area*(p['z1']-p['z0']) for p in g['parts']),sum(Polygon(p['outline']).area*(p['z1']-p['z0']) for p in g['full_parts']),abs_tol=CASE['tolerance'])
                                                         
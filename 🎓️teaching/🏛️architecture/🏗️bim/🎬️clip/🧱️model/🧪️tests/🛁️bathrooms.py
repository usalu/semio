"""🛁 Original shared sanitary programme and actual capsule niche contracts."""
import importlib,json,math
from pathlib import Path
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
CASE=json.loads(Path(__file__).with_suffix('.json').read_text())

def test_original_shared_bathroom_has_distinct_bidet_basin_and_capsule_tub():
    """📚 Check source-attested fixture identities and a true curved niche rather than a straight cut wall."""
    from shapely.geometry import Polygon,Point
    m=api.load_model();assert {w['tag'] for w in CASE['fixtures']}<={e['tag'] for e in m['elements']}
    for witness in CASE['fixtures']:
        obj=api.element(witness['tag'],m)
        assert obj['class']=='IfcSanitaryTerminal' and obj['predefined_type']==witness['predefined']
        assert math.dist(obj['geometry']['axis'],witness['axis'])<CASE['uncertainty_m']
    wall=api.element('W-SHARED-BATH-N',m)
    assert len(wall['geometry']['path'])>8
    assert max(p[1] for p in wall['geometry']['path'])>16.1
    bath=next(s for s in m['spaces'] if s['tag']=='SP-SHARED-BATH');son=next(s for s in m['spaces'] if s['tag']=='SP-SON')
    assert Polygon(bath['outline']).intersection(Polygon(son['outline'])).area<CASE['tolerance']
    assert Polygon(bath['outline']).covers(Point(*CASE['fixtures'][-1]['axis']))

def test_shared_sanitary_cavities_and_native_material_volumes_match_independent_solids():
    """🫧 Compare all explicit native material pieces and actual open fixture cavities with GEOS and SciPy."""
    import numpy as np
    from scipy.spatial import ConvexHull
    from shapely.geometry import Polygon,Point
    from shapely.ops import unary_union
    m=api.load_model()
    for witness in CASE['fixtures']:
        g=api.service_geometry(witness['tag'],m);parts=g['parts'];z=CASE['floor']+witness['cavity_z']
        cut=unary_union([Polygon(r) for p in parts if p['z0']<=z<p['z1'] for r in api.part_band_footprints(p,z,z+1e-7)])
        assert not cut.covers(Point(*witness['axis']))
        independent=sum(ConvexHull(np.asarray(api.part_vertices(p))).volume for p in parts)
        assert math.isclose(api.quantities(witness['tag'],m)['volume'],independent,abs_tol=CASE['tolerance'])
        assert all(p['material'] in {'MAT-SANITARY-CERAMIC','MAT-METAL','MAT-BRICK'} for p in parts)
        for kind in ('axis','footprint','plan','section','elevation','3d','body'):
            assert api.represent(witness['tag'],kind,m)['primitives']

def test_shared_bathroom_two_actual_portal_routes_clear_all_fixed_material():
    """🚶 Test both original doorway functions against occupied walls, structure, fixtures and curved niche."""
    from shapely.geometry import Polygon,LineString
    m=api.load_model();g=api.building_geometry(m);lo,hi=CASE['floor'],CASE['floor']+CASE['body_height'];obstacles=[]
    for category in ('walls','pilotis','screens','beams','details','furnishings','railings','services','stairs'):
        for item in g[category]:
            for p in item['parts']:
                if p['z0']<hi-1e-10 and p['z1']>lo+1e-10:
                    obstacles.extend(Polygon(r) for r in api.part_band_footprints(p,max(lo,p['z0']),min(hi,p['z1'])))
    for witness in CASE['door_cases']:
        filling=api.filling_geometry(witness['tag'],m)
        assert filling['clear_width']>=.8-1e-9 and filling['clear_height']>=CASE['body_height']
        fixed=obstacles+[Polygon(p['outline']) for p in filling['parts'] if p['role']=='frame' and p['z0']<hi and p['z1']>lo]
        route=LineString(witness['route']).buffer(CASE['body_width']/2,cap_style=1,join_style=1)
        intersections=[route.intersection(p).area for p in fixed if route.bounds[0]<p.bounds[2] and p.bounds[0]<route.bounds[2] and route.bounds[1]<p.bounds[3] and p.bounds[1]<route.bounds[3]]
        assert sum(intersections)<CASE['tolerance'],(witness['tag'],sum(intersections))

def test_guest_washbasin_has_a_real_drain_and_hollow_tap_bore():
    """🚰 Keep the registered guest washroom envelope while exposing the actual drain and tap lumen."""
    from shapely.geometry import Polygon,Point
    m=api.load_model()
    def occupied(tag,probe):
        """🔬 Query occupied horizontal material with the independent polygon library."""
        x,y,z=probe;z+=CASE['floor'];obj=api.element(tag,m)
        g=api.service_geometry(tag,m) if obj['class']=='IfcSanitaryTerminal' else api.solid_geometry(tag,m)
        return any(Polygon(r).covers(Point(x,y)) for p in g['parts'] if p['z0']<z<p['z1'] for r in api.part_regions(p,z))
    assert not occupied('FIX-GUEST-BASIN',CASE['guest']['drain_probe'])
    assert occupied('FIX-GUEST-BASIN',CASE['guest']['floor_probe'])
    assert not occupied('TAP-GUEST',CASE['guest']['tap_bore'])
    assert occupied('TAP-GUEST',CASE['guest']['tap_metal'])

def test_original_guest_niche_routes_keep_full_operated_leaf_and_actual_torso_clear():
    """🚶 Prove the wide room entry and honest sideways torso movement through the original narrow wash niche."""
    from shapely.geometry import Polygon,LineString,MultiPoint
    m=api.load_model();g=api.building_geometry(m);case=CASE['guest'];base=CASE['floor'];top=base+case['body_height'];fixed=[]
    for category in ('walls','pilotis','screens','beams','details','furnishings','railings','services','stairs','fillings'):
        for item in g[category]:
            if item['tag']==case['door']:continue
            for p in item['parts']:
                if p['z0']<top and p['z1']>base:
                    fixed.extend((item['tag'],Polygon(r)) for r in api.part_band_footprints(p,max(base,p['z0']),min(top,p['z1'])))
    filling=api.filling_geometry(case['door'],m);frames=[Polygon(p['outline']) for p in filling['parts'] if p['role']=='frame' and p['z0']<top and p['z1']>base];leaves=[p for p in filling['parts'] if p['role']!='frame']
    points=[v for p in leaves for v in api.part_vertices(p)];hx=min(v[0] for v in points);hy=max(v[1] for v in points)
    opened=[Polygon([[hx+(y-hy),hy-(x-hx)] for x,y in p['outline']]) for p in leaves]
    fixed.extend((case['door'],p) for p in frames)
    for pose,polygons in [('closed',[Polygon(p['outline']) for p in leaves]),('open90',opened)]:
        for shape in polygons:
            collisions=[(tag,shape.intersection(other).area) for tag,other in fixed if shape.bounds[0]<other.bounds[2] and other.bounds[0]<shape.bounds[2] and shape.bounds[1]<other.bounds[3] and other.bounds[1]<shape.bounds[3]]
            assert sum(a for _,a in collisions)<CASE['tolerance'],(pose,[(t,a)for t,a in collisions if a>1e-10])
    fixed.extend((case['door'],p) for p in opened)
    routes=[LineString(case['entry_route']).buffer(case['entry_width']/2,cap_style=1,join_style=1)]
    for a,b in zip(case['niche_route'],case['niche_route'][1:]):
        routes.append(MultiPoint([(p[0]+sx*case['torso_width_x']/2,p[1]+sy*case['shoulders_y']/2) for p in (a,b) for sx in (-1,1)for sy in (-1,1)]).convex_hull)
    for route in routes:
        collisions=[(tag,route.intersection(other).area) for tag,other in fixed if route.bounds[0]<other.bounds[2] and other.bounds[0]<route.bounds[2] and route.bounds[1]<other.bounds[3] and other.bounds[1]<route.bounds[3]]
        assert sum(a for _,a in collisions)<CASE['tolerance'],[(t,a)for t,a in collisions if a>1e-10]

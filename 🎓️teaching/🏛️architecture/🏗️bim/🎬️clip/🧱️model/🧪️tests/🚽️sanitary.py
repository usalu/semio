"""🚽 Original sanitary programme, native material and fixed-obstacle access contracts."""
import importlib,json,math
from pathlib import Path
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
CASE=json.loads(Path(__file__).with_suffix('.json').read_text())

def test_original_two_first_floor_wcs_have_real_rooms_and_native_sanitary_identity():
    """📋 Validate original WC programme and native schema without furniture-class substitution."""
    import jsonschema
    m=api.load_model();tags={e['tag'] for e in m['elements']}
    assert {w['tag'] for w in CASE['fixtures']}<=tags
    schema=json.loads((Path(api.__file__).parent/'🧬️schema/🔣️.schema.json').read_text())
    jsonschema.Draft202012Validator(schema).validate(m)
    assert m['classes'][CASE['class']]=={'supertype':'IfcFlowTerminal','adds':['PredefinedType']}
    for witness in CASE['fixtures']:
        obj=api.element(witness['tag'],m)
        assert obj['class']==CASE['class'] and obj['predefined_type']==CASE['predefined_type']
        assert obj['historical_evidence']['state']==CASE['historical_state']
        assert any(s['tag']==witness['space'] for s in m['spaces'])
        for lang in ('en','de'):
            exported=api.step_excerpt(obj['tag'],m,language=lang)
            assert any('=IFCSANITARYTERMINAL(' in line for line in exported['lines'])

def test_wc_bowls_cisterns_and_seats_are_hollow_with_independent_material_volume():
    """🫧 Compare real ceramic cavities and every convex material solid with GEOS and SciPy."""
    import numpy as np
    from scipy.spatial import ConvexHull
    from shapely.geometry import Point,Polygon
    from shapely.ops import unary_union
    m=api.load_model()
    for witness in CASE['fixtures']:
        obj=api.element(witness['tag'],m);g=api.service_geometry(obj['tag'],m);x,y=witness['axis'];parts=g['parts']
        assert {'bowl_shell','pan_base','seat','cistern_wall','cistern_lid','flush_pipe'}<={p['role'] for p in parts}
        for z,point in [(CASE['floor']+.34,(x,y)),(CASE['floor']+.80,(x,y-.24))]:
            occupied=unary_union([Polygon(r) for p in parts for r in api.part_band_footprints(p,z,z+1e-7) if p['z0']<=z<p['z1']])
            assert not occupied.covers(Point(*point))
        independent=sum(ConvexHull(np.asarray(api.part_vertices(p))).volume for p in parts)
        assert independent>0
        assert math.isclose(independent,api.quantities(obj['tag'],m)['volume'],abs_tol=CASE['tolerance'])
        assert {q['material'] for q in api.quantities(obj['tag'],m)['material_quantities']}=={'MAT-SANITARY-CERAMIC','MAT-WOOD','MAT-METAL'}
        for kind in ('axis','footprint','plan','section','elevation','3d','body'):
            assert api.represent(obj['tag'],kind,m)['primitives']

def test_first_floor_wc_side_portals_and_body_routes_clear_actual_fixed_material():
    """🚶 Compare actual closed fixtures and open-door frame routes against native GEOS obstacles."""
    from shapely.geometry import LineString,Polygon,Point
    from shapely.ops import unary_union
    m=api.load_model();g=api.building_geometry(m);lo,hi=CASE['floor'],CASE['floor']+CASE['body_height'];obstacles=[]
    for category in ('walls','pilotis','screens','beams','details','furnishings','railings','services','stairs'):
        for item in g[category]:
            for p in item['parts']:
                if p['z0']<hi-1e-10 and p['z1']>lo+1e-10:
                    obstacles.extend(Polygon(r) for r in api.part_band_footprints(p,max(lo,p['z0']),min(hi,p['z1'])))
    for witness in CASE['fixtures']:
        filling=api.filling_geometry(witness['door'],m)
        assert math.isclose(filling['clear_width'],witness['door_clear_width'],abs_tol=1e-10)
        assert filling['clear_height']>=CASE['body_height']
        frame=[p for p in filling['parts'] if p['role']=='frame']
        fixed=obstacles+[Polygon(p['outline']) for p in frame if p['z0']<hi and p['z1']>lo]
        route=LineString(witness['route']).buffer(CASE['body_width']/2,cap_style=2,join_style=2)
        collisions=[route.intersection(p).area for p in fixed if route.bounds[0]<p.bounds[2] and p.bounds[0]<route.bounds[2] and route.bounds[1]<p.bounds[3] and p.bounds[1]<route.bounds[3]]
        assert sum(collisions)<CASE['tolerance'],(witness['tag'],sum(collisions))
        room=next(s for s in m['spaces'] if s['tag']==witness['space'])
        assert Polygon(api._expand(m,room['outline'])).covers(Point(*witness['axis']))
        hall=next(s for s in m['spaces'] if s['tag']=='SP-FIRST-HALL')
        assert Polygon(room['outline']).intersection(Polygon(api._expand(m,hall['outline']))).area<CASE['tolerance']

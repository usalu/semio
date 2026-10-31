"""💡 Exterior inferred lamp azimuths retain actual casing mounts while clearing true concrete shafts."""
import importlib,json
from pathlib import Path
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
CASE=json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf8'))

def test_light_clearance_contract_matches_actual_analytic_source_posts():
    """🧬 Bind neutral controls to the actual circular shaft and strict native positive witnesses."""
    from jsonschema import Draft202012Validator
    Draft202012Validator(json.loads(Path(__file__).with_suffix('.schema.json').read_text(encoding='utf8'))).validate(CASE)
    source=api.load_model()
    for row in CASE['fixtures']:
        post=api._find(source,row['post']);assert post['geometry']['axis']==row['axis']
        solids=api.column_geometry(source,post)['parts']
        assert any(p.get('profile',{}).get('shape')=='circle' and p['radius']==row['radius'] and p['z0']<row['witness'][2]<p['z1'] for p in solids)
        assert sum((a-b)**2 for a,b in zip(row['witness'],row['axis']))<row['radius']**2

def test_every_exterior_luminaire_cell_clears_the_analytic_post_and_old_shared_point():
    """🔬 Use independent convex-cell projection distance to the true cylinder rather than its inscribed facets."""
    import numpy as np
    from scipy.spatial import ConvexHull
    from shapely.geometry import MultiPoint,Point
    source=api.load_model()
    for row in CASE['fixtures']:
        parts=api.service_geometry(row['tag'],source)['parts'];shaft=next(p for p in api.column_geometry(source,api._find(source,row['post']))['parts'] if p.get('profile',{}).get('shape')=='circle')
        for part in parts:
            vertices=np.asarray(api.part_vertices(part));hull=ConvexHull(vertices)
            assert shaft['z0']<vertices[:,2].min()<=vertices[:,2].max()<shaft['z1']
            assert MultiPoint(vertices[:,:2]).convex_hull.distance(Point(row['axis']))>=row['radius']-CASE['absolute_distance_tolerance'],(row['tag'],part.get('role'))
            equations=hull.equations
            assert np.max(equations[:,:3]@row['witness']+equations[:,3])>=0,(row['tag'],'old shared point remains occupied')

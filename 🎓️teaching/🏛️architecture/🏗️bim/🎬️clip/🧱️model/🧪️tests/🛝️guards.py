"""🛝️ Continuous concrete ramp guards own material exclusively at real wall junctions."""
import importlib,json,math
from pathlib import Path
model=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
CASE=json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf8'))

def _occupied(part,point):
    """🔬 Query strict native material using independent GEOS prisms or SciPy convex planes."""
    from shapely.geometry import Polygon,Point
    if not part['z0']<point[2]<part['z1']:return False
    if not any(k in part for k in ('mesh','section','top_outline')):
        return Polygon(part['outline'],part.get('holes',())).contains(Point(point[:2]))
    import numpy as np
    from scipy.spatial import ConvexHull
    vertices=np.asarray(model.part_vertices(part));origin=vertices.min(axis=0);spans=vertices.max(axis=0)-origin
    if np.any(spans<=0):return False
    if any(point[k]<origin[k] or point[k]>origin[k]+spans[k] for k in range(3)):return False
    equations=ConvexHull((vertices-origin)/spans).equations
    distance=(equations[:,:3]@((np.asarray(point)-origin)/spans)+equations[:,3])/np.linalg.norm(equations[:,:3]/spans,axis=1)
    return bool(np.max(distance) < -CASE['comparison']['point_margin'])

def _guards():
    """🧱 Read each current authored product rather than a captured former shape."""
    source=model.load_model()
    return {tag:model._solid_geometry(source,model._find(source,tag))['parts'] for tag in ('GUARD-RM-1','GUARD-RM-2')}

def test_guard_contract_is_schema_valid_and_reproduces_original_positive_intersections():
    """🧬 Verify every retained defect witness against independent original occupied geometry."""
    from jsonschema import Draft202012Validator
    from shapely.geometry import Polygon
    Draft202012Validator(json.loads(Path(__file__).with_suffix('.schema.json').read_text(encoding='utf8'))).validate(CASE)
    for row in CASE['cases']:
        a,b=row['previous_guard'],row['wall_part']
        overlap=Polygon(a['outline']).intersection(Polygon(b['outline'])).area*(min(a['z1'],b['z1'])-max(a['z0'],b['z0']))
        assert abs(overlap-row['previous_volume'])<=CASE['comparison']['volume_tolerance'],row['id']
        assert _occupied(a,row['witness']) and _occupied(b,row['witness']),row['id']

def test_archive_concrete_guard_owns_every_original_masonry_joint_witness():
    """🚧 Keep the archive-attested concrete continuous and partition the inferred masonry once."""
    guards=_guards();source=model.load_model();walls={tag:model._wall_geometry(source,model._find(source,tag))['parts'] for tag in ('W-G-GARAGE','W-PARENT-BATH')}
    for row in CASE['cases']:
        assert _occupied(row['wall_part'],row['witness']),row['id']
        assert any(_occupied(p,row['witness']) for p in guards[row['guard']]),(row['id'],'archive concrete removed')
        assert not any(_occupied(p,row['witness']) for p in walls[row['wall']]),(row['id'],'two materials own the same point')
    for row in CASE['rc_core']:
        assert any(_occupied(p,row['point']) for p in guards[row['guard']]),row
        assert not any(_occupied(p,row['point']) for p in walls[row['wall']]),row
    for row in CASE['masonry_controls']:
        assert any(_occupied(p,row['point']) for p in walls[row['wall']]),(row,'masonry outside actual RC was overcut')

def test_parapet_tops_are_continuous_inclined_planes_instead_of_midpoint_stairs():
    """📏 Check both sides of the nominal inclined top at four unobstructed original flight edges."""
    guards=_guards()
    for row in CASE['continuity']:
        assert any(_occupied(p,row['below']) for p in guards[row['guard']]),row
        assert not any(_occupied(p,row['above']) for p in guards[row['guard']]),row

def test_every_guard_part_is_closed_convex_positive_and_matches_independent_volume():
    """🔻 Compare every actual native guard cell with independent SciPy volume and closed edge incidence."""
    from collections import Counter
    independent=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧪️tests.🔻️volume').independent_volume
    for tag,parts in _guards().items():
        assert parts,tag
        for part in parts:
            vertices=model.part_vertices(part);faces=model.part_faces(part)
            counts=Counter(tuple(sorted((a,b))) for face in faces for a,b in zip(face,face[1:]+face[:1]))
            assert counts and set(counts.values())=={2},tag
            expected=independent(vertices)
            assert expected>0 and model._polyhedron_volume(part)>0 and math.isclose(model._polyhedron_volume(part),expected,rel_tol=1e-7,abs_tol=1e-24),tag
                                                                                 
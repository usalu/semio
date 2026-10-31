"""🚰 Source-correct pedestal ellipses, real paired taps and mounting material."""
import importlib,json,math
from pathlib import Path
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
CASE=json.loads(Path(__file__).with_suffix('.json').read_text())

def _shape(parts,z,material=None):
    """🔬 Resolve a true material slice including lofts, mesh tubes and profile holes."""
    from shapely.geometry import Polygon
    from shapely.ops import unary_union
    return unary_union([Polygon(r) for p in parts if p['z0']<z<p['z1'] and (material is None or p['material']==material) for r in api.part_regions(p,z)])

def test_original_pedestal_ellipses_and_paired_taps_follow_both_plan_sources():
    """📚 Enforce fixed axes, long-Y source traces and two real independent fittings per basin."""
    from jsonschema import Draft202012Validator
    Draft202012Validator(json.loads(Path(__file__).with_suffix('.schema.json').read_text())).validate(CASE)
    m=api.load_model()
    for row in CASE['rows']:
        obj=api.element(row['basin'],m);assert obj['geometry']['axis']==row['axis']
        g=api.service_geometry(row['basin'],m);parts=[p for p in g['parts'] if p['role']=='basin_shell'];points=[v for p in parts for v in api.part_vertices(p)]
        size=[max(p[i] for p in points)-min(p[i] for p in points) for i in (0,1)]
        assert all(abs(a-b)<1e-8 for a,b in zip(size,row['size'])),(row['basin'],size)
        raw=api.element(row['tap'],m)['geometry']['parts'];assert len([p for p in raw if p.get('role')=='tap_stem'])==CASE['segments']*row['pairs']
        assert any(p.get('role')=='tap_cross_handle' for p in raw)

def test_paired_tap_lumens_and_mounting_holes_are_real_occupied_material_interfaces():
    """🫧 Verify open bores, positive metal walls, actual ceramic contact and conserved disjoint bodies."""
    import numpy as np
    from scipy.spatial import ConvexHull
    from shapely.geometry import Point
    m=api.load_model()
    for row in CASE['rows']:
        basin=api.service_geometry(row['basin'],m);tap=basin if row['owned'] else api.solid_geometry(row['tap'],m);z=row['floor']+CASE['probe_z'];slice_=_shape(tap['parts'],z,'MAT-METAL');mountz=row['floor']+row['rim']+.01
        ceramic=_shape(basin['parts'],mountz,'MAT-SANITARY-CERAMIC');metal=_shape(tap['parts'],mountz,'MAT-METAL')
        assert ceramic.intersection(metal).area<1e-9
        assert ceramic.boundary.intersection(metal.boundary.buffer(CASE['contact_tolerance_m'])).length>.10,(row['basin'],'floating stem')
        for off in CASE['stem_offsets_y']:
            x,y=row['axis'][0]+CASE['stem_offset_x'],row['axis'][1]+off
            assert not slice_.covers(Point(x,y));assert slice_.covers(Point(x+.011,y))
            assert not _shape(basin['parts'],row['floor']+.80).covers(Point(row['axis'][0]-.06,y))
        for tag,geometry in [(row['basin'],basin)]+([] if row['owned'] else [(row['tap'],tap)]):
            volume=sum(ConvexHull(np.asarray(api.part_vertices(p))).volume for p in geometry['parts'])
            assert math.isclose(api.quantities(tag,m)['volume'],volume,abs_tol=CASE['tolerance']),(tag,volume)
            for language in ('en','de'):assert api.step_excerpt(tag,m,language=language)['lines']

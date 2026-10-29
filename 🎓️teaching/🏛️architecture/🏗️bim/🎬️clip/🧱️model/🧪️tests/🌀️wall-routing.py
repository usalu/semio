"""🌀 Route wall geometry by its native path rather than its IFC classification."""
import importlib,json,math
from functools import lru_cache
from pathlib import Path
from shapely.geometry import Polygon
from shapely.ops import unary_union
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
exchange=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🔌️exchange.🐍️')

@lru_cache(maxsize=1)
def example():
    """🗺️ Capture one source cohort and its actual shared geometry catalogue."""
    case=json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf8'));source=api.load_model();geometry=api.building_geometry(source)
    return case,source,geometry

def test_standard_curved_wall_reaches_complete_shared_catalogue():
    """🏰 Preserve the complete enclosure without reclassifying its native wall type."""
    case,source,geometry=example();wall=next(e for e in source['elements'] if e['tag']==case['tag'])
    assert wall['predefined_type']==case['predefined_type']
    assert not any(e['tag']==case['tag'] for e in geometry['walls'])
    matches=[e for e in geometry['screens'] if e['tag']==case['tag']];assert len(matches)==1
    assert max(p[1] for part in matches[0]['parts'] for p in api.part_footprint(part))>case['minimum_y_extent']
    assert {p['material'] for p in matches[0]['parts']}==set(case['materials'])

def test_curved_standard_wall_public_views_keep_material_domain():
    """👁️ Compare all public view quantities with independent occupied GEOS slices."""
    case,source,_=example();native=api.path_wall_geometry(source,next(e for e in source['elements'] if e['tag']==case['tag']));parts=native['parts']
    levels=sorted({p[k] for p in parts for k in ('z0','z1')})
    volume=sum(unary_union([Polygon(api.part_outline_at(p,(lo+hi)/2)) for p in parts if p['z0']<(lo+hi)/2<p['z1']]).area*(hi-lo) for lo,hi in zip(levels,levels[1:]))
    for kind in case['views']:
        value=api.represent(case['tag'],kind,source)
        assert math.isclose(value['derived']['volume'],volume,abs_tol=1e-8),kind
    primitives=api.represent(case['tag'],'3d',source)['primitives']
    assert max(p[1] for item in primitives for p in item['polygon'])>case['minimum_y_extent']
    assert {p['material'] for p in primitives}==set(case['materials'])

def test_curved_standard_wall_native_step_preserves_whole_body():
    """📄 Export the complete native layered path wall rather than its first chord."""
    case,source,geometry=example();native=api.path_wall_geometry(source,next(e for e in source['elements'] if e['tag']==case['tag']))
    for language in ('en','de'):
        result=exchange.step_excerpt(case['tag'],source,language=language)
        assert result['class']=='IfcWall'
        assert sum('=IFCEXTRUDEDAREASOLID(' in line for line in result['lines'])==len(native['parts'])
        assert 'MAT-AIR' not in '\n'.join(result['lines'])

def test_explicit_closed_wall_keeps_its_final_material_segment():
    """🔒 Preserve occupied west closure and the separate interior void in the public plan."""
    from shapely.geometry import Point
    case=json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf8'));source=api.load_model()
    primitives=api.represent(case['tag'],'plan',source)['primitives'];shapes=[Polygon(p['points']) for p in primitives]
    assert any(p.covers(Point(case['closed_boundary']['occupied'])) for p in shapes)
    assert not any(p.intersects(Point(case['closed_boundary']['empty'])) for p in shapes)

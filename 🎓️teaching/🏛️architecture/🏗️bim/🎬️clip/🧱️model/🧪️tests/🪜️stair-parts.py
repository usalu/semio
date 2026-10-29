"""🪜 Verify complete native stair material through public views and IFC export."""
import importlib,json,math
from copy import deepcopy
from pathlib import Path
import numpy as np
from scipy.spatial import ConvexHull
from shapely.geometry import Polygon
from shapely.ops import unary_union
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
exchange=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🔌️exchange.🐍️')

def example():
    """🧩 Place an isolated material-part contract on an existing native stair identity."""
    case=json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf8'));source=deepcopy(api.load_model());stair=next(e for e in source['elements'] if e['tag']==case['stair'])
    stair['geometry'].update(kind='curved_straight',step_count=1,step_parts=[case['step']],landing_parts=[case['landing']],structural_parts=[case['structure']])
    return case,source

def test_public_stair_parts_preserve_mesh_and_material():
    """🧱 Keep a structural waist beside semantic tread and landing material in every physical view."""
    case,source=example();geometry=api.stair_geometry(case['stair'],source)
    assert len(geometry['parts'])==case['expected']['parts']
    for view in case['views']:
        result=api.represent(case['stair'],view,source)
        assert math.isclose(result['derived']['volume'],case['expected']['volume'],abs_tol=1e-9)
        if view!='axis':assert any(p.get('role')==case['expected']['mesh_role'] and p.get('material')==case['expected']['mesh_material'] for p in result['primitives']),view
    for view,kind in [('3d','polyhedron'),('body','faceted_brep')]:
        result=api.represent(case['stair'],view,source)
        assert result['type']=='SolidModel'
        assert sum(p['kind']==kind for p in result['primitives'])==1

def test_stair_occupied_volume_matches_independent_hull_and_slices():
    """📐 Check a continuous structural body independently with SciPy and GEOS."""
    case,source=example();expected=ConvexHull(np.asarray(case['structure']['mesh']['vertices'])).volume
    assert math.isclose(expected,case['expected']['mesh_volume'],abs_tol=1e-12)
    geometry=api.stair_geometry(case['stair'],source);zs=sorted({p[k] for p in geometry['parts'] for k in ('z0','z1')})
    volume=sum(unary_union([Polygon(api.part_outline_at(p,(lo+hi)/2)) for p in geometry['parts'] if p['z0']<(lo+hi)/2<p['z1']]).area*(hi-lo) for lo,hi in zip(zs,zs[1:]))
    assert math.isclose(volume,case['expected']['volume'],abs_tol=1e-9)
    assert math.isclose(api.quantities(case['stair'],source)['volume'],volume,abs_tol=1e-9)

def test_native_stair_step_preserves_all_material_parts():
    """📄 Export the actual native stair product with a brep waist and separate finish solids."""
    case,source=example()
    for language in ('en','de'):
        fragment=exchange.step_excerpt(case['stair'],source,language=language);text='\n'.join(fragment['lines'])
        assert 'IFCSTAIR(' in text
        assert text.count('IFCFACETEDBREP(')==1
        assert text.count('IFCEXTRUDEDAREASOLID(')==2
        assert 'SolidModel' in text
        assert text.count('IFCMATERIAL(')==2
        assert text.count('IFCMATERIALCONSTITUENT(')==3

def test_polyhedron_boundary_planes_keep_all_vertices():
    """💠 Preserve both endpoints when a section plane coincides with a complete mesh face."""
    case,source=example();part=api.stair_geometry(case['stair'],source)['structural_parts'][0]
    vertices=np.asarray(part['mesh']['vertices'])
    for axis,expected in enumerate(case['boundary_areas']):
        for value in (vertices[:,axis].min(),vertices[:,axis].max()):
            rings=[api.part_outline_at(part,float(value))] if axis==2 else api.part_sections(part,float(value),axis)
            assert math.isclose(unary_union([Polygon(r) for r in rings]).area,expected,abs_tol=1e-12),(axis,value)

def test_native_solid_mesh_keeps_authored_material_role():
    """🎨 Preserve material-domain identity when normalizing a native structural mesh."""
    case,source=example();solid=next(e for e in source['elements'] if e['class']==case['solid']['class'])
    solid['geometry']={'kind':case['solid']['kind'],'base_level':case['solid']['base_level'],'parts':[case['structure']]}
    part=api.solid_geometry(solid['tag'],source)['parts'][0]
    assert part['material']==case['expected']['mesh_material']
    assert part['role']==case['expected']['mesh_role']
    assert math.isclose(ConvexHull(np.asarray(part['mesh']['vertices'])).volume,case['expected']['mesh_volume'],abs_tol=1e-12)

def test_stair_geometry_requires_explicit_occupied_parts():
    """🚫 Reject missing authored tread or structural material instead of manufacturing circular blocks."""
    case,source=example()
    for key in case['required_geometry']:
        candidate=deepcopy(source);geometry=next(e for e in candidate['elements'] if e['tag']==case['stair'])['geometry'];del geometry[key]
        try:api.stair_geometry(case['stair'],candidate)
        except KeyError as error:assert error.args[0]==key
        else:raise AssertionError('Missing explicit stair material: '+key)

def test_stair_source_has_no_unused_turn_synthesis_fields():
    """🧭 Keep the legitimate U-return controls while dropping obsolete alternate construction inputs."""
    case,source=example()
    for stair in [e for e in source['elements'] if e['class']=='IfcStair']:
        assert all(key not in stair['geometry'] for key in case['obsolete_geometry'])
        assert all(key in stair['geometry'] for key in case['required_geometry'])
        assert stair['geometry']['radius']>stair['geometry']['inner_radius']>0

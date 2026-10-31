"""🪟 Native fillings retain material only outside independently owned structural bodies."""
from pathlib import Path
from copy import deepcopy
import importlib,json,math
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
CASE=json.loads(Path(__file__).with_suffix('.json').read_text())

def overlap(first,second):
    """📐 GEOS measures actual occupied planar regions at each strict common height interval."""
    from shapely.geometry import Polygon
    return sum(Polygon(a['outline'],a.get('holes',[])).intersection(Polygon(b['outline'],b.get('holes',[]))).area*max(0,min(a['z1'],b['z1'])-max(a['z0'],b['z0'])) for a in first for b in second)

def test_native_ribbon_window_keeps_all_owned_material_outside_its_real_concrete_jamb():
    """🧱 A captured positive rectangle/prism control remains RED until the actual native filling relinquishes concrete-owned material."""
    from jsonschema import Draft202012Validator
    Draft202012Validator(json.loads(Path(__file__).with_suffix('.schema.json').read_text())).validate(CASE)
    control=CASE['neutral'];assert abs(overlap([control['part']],[control['cutter']])-control['intersection_volume'])<1e-14
    m=api.load_model();filling=api._filling_geometry(m,api._find(m,'RW-E'));column=api.column_geometry(m,api._find(m,'1-FIRST'))
    actual=overlap(filling['parts'],column['parts'])
    assert actual==CASE['expected_overlap'],('RW-E','1-FIRST',actual)

def test_real_receiving_post_keeps_complete_sourced_perimeter_jamb_stock():
    """🪵 The correction retains the full80×40mm member against the real RC receiving face instead of deleting it."""
    from shapely.geometry import Polygon
    row=CASE['jamb'];m=api.load_model();filling=api._filling_geometry(m,api._find(m,row['filling']));column=api.column_geometry(m,api._find(m,row['column']))
    assert filling['source_rough_width']>filling['rough_width']
    assert filling['frame_width']==row['frame_width'] and filling['frame_depth']==row['frame_depth']
    concrete=Polygon(column['parts'][0]['outline']);contact=0.
    for part in filling['parts']:
        if part['role']!='frame':continue
        polygon=Polygon(part['outline']);xmin,ymin,xmax,ymax=polygon.bounds
        if math.isclose(ymax-ymin,row['frame_width'],rel_tol=0,abs_tol=1e-12) and math.isclose(xmax-xmin,row['frame_depth'],rel_tol=0,abs_tol=1e-12):
            contact+=polygon.boundary.intersection(concrete.boundary).length*max(0,min(part['z1'],column['parts'][0]['z1'])-max(part['z0'],column['parts'][0]['z0']))
    assert abs(contact-row['expected_contact_area'])<1e-12,contact

def test_all_proven_filling_column_and_beam_pairs_have_no_occupied_prism_overlap():
    """🧱 All independently diagnosed source pairs retain zero actual prism overlap after physical ownership resolution."""
    m=api.load_model();fillings={}
    for row in CASE['pairs']:
        tag=row['filling'];obj=api._find(m,tag)
        if tag not in fillings:fillings[tag]=api._curtain_filling_geometry(m,obj) if obj['class']=='IfcCurtainWall' else api._filling_geometry(m,obj)
        host=api._find(m,row['structure']);body=api.column_geometry(m,host) if host['class']=='IfcColumn' else api._solid_geometry(m,host,resolve_embedded=False)
        assert not any(any(key in p for key in ('mesh','section','top_outline')) for p in [*fillings[tag]['parts'],*body['parts']]),'These independently captured pairs require their actual prism oracle'
        value=overlap(fillings[tag]['parts'],body['parts']);assert value==0,(row,value)

def test_spatial_filling_ownership_preserves_sloped_structure_and_full_input_output_isolation():
    """🧊 Exact3D cuts match independent SciPy volume, retain stock metadata and follow complete-source mutations."""
    import numpy as np
    from scipy.spatial import ConvexHull
    row=CASE['spatial'];m=api.load_model();beam=deepcopy(api._find(m,'BEAM-PRIMARY-F-3-34'));cutter={**api._polyhedron_part(row['vertices'],row['faces']),'material':'MAT-RC','role':'beam'}
    beam['geometry']={'kind':'beam','base_level':'ground','parts':[cutter]};m['elements']=[beam];stock=deepcopy(row['part']);original=deepcopy(stock)
    owned=api._filling_owned_parts(m,[stock]);assert stock==original
    assert abs(sum(api._polyhedron_volume(p) for p in owned)-row['expected_volume'])<1e-12
    for part in owned:
        api._polyhedron_part(part['mesh']['vertices'],part['mesh']['faces']);vertices=np.asarray(part['mesh']['vertices'])
        assert abs(ConvexHull(vertices-vertices[0]).volume-api._polyhedron_volume(part))<1e-12
        assert part['role']=='frame' and part['material']=='MAT-WOOD' and part['panel_index']==7
    owned[0]['mesh']['vertices'][0][0]+=5
    fresh=api._filling_owned_parts(m,[stock]);assert abs(sum(api._polyhedron_volume(p) for p in fresh)-row['expected_volume'])<1e-12
    for point in beam['geometry']['parts'][0]['mesh']['vertices']:point[2]+=.1
    beam['geometry']['parts'][0]['z0']+=.1;beam['geometry']['parts'][0]['z1']+=.1
    changed=api._filling_owned_parts(m,[stock]);assert abs(sum(api._polyhedron_volume(p) for p in changed)-row['translated_volume'])<1e-12

"""🛝️ Multi-storey ramp wearing remains visible at its real upper-storey arrival."""
import ast,json
from pathlib import Path

def test_actual_plan_predicate_keeps_only_occupied_ramp_arrivals_across_owner_storeys():
    """🌐 Compare neutral arrival/air/bedding controls with the production helper and independent NumPy intervals."""
    import numpy as np
    from jsonschema import Draft202012Validator
    directory=Path(__file__).parent;case=json.loads((directory/'ramp_arrivals.json').read_text(encoding='utf8'));Draft202012Validator(json.loads((directory/'ramp_arrivals.schema.json').read_text(encoding='utf8'))).validate(case)
    source=directory.parent/'📋️plan/🐍️.py';tree=ast.parse(source.read_text(encoding='utf8'));module=ast.Module(body=[node for node in tree.body if isinstance(node,ast.FunctionDef)and node.name in ('plan_part_rank','plan_part_visible')],type_ignores=[]);ns={};exec(compile(module,str(source),'exec'),ns)
    for row in case['cases']:
        data={'kind':row['kind'],'class':'IfcCovering','predefined_type':'FLOORING','storey':row['owner']};part={'z0':row['z'][0],'z1':row['z'][1],'role':row['role']};actual=ns['plan_part_visible'](data,part,row['level'],row['band'],True)
        height=bool(np.less(row['z'][0],row['band'][1])and np.greater_equal(row['z'][1],row['band'][0]));oracle=row['role']=='wearing'and row['z'][0]<row['band'][1]and(height if row['kind']=='ramp_finish'else row['owner']==row['level']);assert oracle==row['visible'];assert actual==row['visible'],row['id']


def test_actual_plan_solid_loop_draws_arrival_wearing_and_hides_bedding_and_remote_air():
    """🖌️ Exercise production drawing selection through the recording interface and independent GEOS contours."""
    import importlib
    from shapely.geometry import Polygon
    provider=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🎨️visuals.🏛️savoye.🧪️ortho-remodel.test_visibility');case=json.loads((Path(__file__).parent/'ramp_arrivals.json').read_text(encoding='utf8'));outline=[[0,0],[1,0],[1,1],[0,1]]
    for row in case['cases']:
        part={'outline':outline,'z0':row['z'][0],'z1':row['z'][1],'role':row['role'],'material':'MAT-RAMP-RESILIENT'};data={'tag':'ARRIVAL','guid':'arrival','class':'IfcCovering','predefined_type':'FLOORING','domains':['architecture'],'kind':row['kind'],'storey':row['owner'],'parts':[part]};view={'level':row['level'],'band':row['band'],'cut':sum(row['band'])/2};actual=provider.plan_output(data,view);assert(not actual.is_empty)==row['visible'],row['id']
        if row['visible']:assert actual.symmetric_difference(Polygon(outline)).area==0,row['id']

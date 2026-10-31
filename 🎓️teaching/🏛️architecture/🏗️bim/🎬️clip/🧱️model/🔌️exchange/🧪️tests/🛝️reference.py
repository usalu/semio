"""🛝️ Native ramp Body contains only exclusive material while Reference retains its uncut product stock."""
import copy,importlib,json,math
from pathlib import Path
A=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
T=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧪️tests.🐍️')
CASE=json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf8'))


def fixture():
    """📐 Isolate an actual affine flight and an explicit independent owner without borrowed landing products."""
    m=A.load_model();flight=copy.deepcopy(A._find(m,'RF-1A'));parent=copy.deepcopy(A._find(m,'RM-1'));owner=copy.deepcopy(A._by_class(m,'IfcBuildingElementProxy')[0]);q=lambda value:{'value':value,'unit':'m','basis':'assumed'}
    flight.update(tag='NEUTRAL-FLIGHT',guid=A.global_id('NEUTRAL-FLIGHT'),material='MAT-RC',storey='ground');flight.pop('type',None);flight['attributes']['width']=q(CASE['offsets'][1]-CASE['offsets'][0])
    flight['geometry']={'ramp':'NEUTRAL-RAMP','path':copy.deepcopy(CASE['path']),'structural_offsets':[q(value) for value in CASE['offsets']],'walking_offsets':[q(value) for value in CASE['offsets']],'from_level':CASE['from_level'],'to_level':CASE['to_level'],'portion':[0,1],'finish_thickness':q(CASE['finish']),'finish_direction':'vertical','thickness':q(CASE['depth']),'thickness_direction':'vertical','interface_owners':['NEUTRAL-OWNER']}
    parent.update(tag='NEUTRAL-RAMP',guid=A.global_id('NEUTRAL-RAMP'),material='MAT-RC',storey='ground');parent.pop('type',None);parent['attributes']['width']=flight['attributes']['width'];parent['geometry']={'from_level':CASE['from_level'],'to_level':CASE['to_level'],'landings':[],'finish_thickness':q(CASE['finish']),'finish_direction':'vertical','thickness':q(CASE['depth']),'thickness_direction':'vertical'}
    owner.update(tag='NEUTRAL-OWNER',guid=A.global_id('NEUTRAL-OWNER'),material='MAT-RC',storey='ground',**{'class':'IfcFooting'});owner.pop('type',None);owner['geometry']={'kind':'structural_member','base_level':0,'parts':[{**copy.deepcopy(CASE['owner']),'material':'MAT-RC','role':'structural_member'}]}
    m['elements']=[flight,parent,owner];m['spaces']=[];m['relations']=[]
    return m


def test_native_flight_body_and_full_reference_are_separate_exact_products():
    """🧭 Compare actual owned and original affine stock against independent SciPy volumes and native representation labels."""
    from scipy.spatial import ConvexHull
    import numpy as np
    from jsonschema import Draft202012Validator
    Draft202012Validator(json.loads(Path(__file__).with_suffix('.schema.json').read_text(encoding='utf8'))).validate(CASE)
    m=fixture();physical=A.ramp_geometry('NEUTRAL-FLIGHT',m);reference=A.reference_parts('NEUTRAL-FLIGHT',m)
    for parts,expected in ((physical['parts'],CASE['owned_volume']),(reference,CASE['reference_volume'])):
        volume=sum(ConvexHull(np.asarray(A.part_vertices(part),float)).volume for part in parts);assert math.isclose(volume,expected,abs_tol=1e-12),(volume,expected)
    for language in CASE['languages']:
        fragment=A.step_excerpt('NEUTRAL-FLIGHT',m,language=language,spatial=False);entities=T.step_parse(fragment['lines'])['entities'];_,product=entities[int(fragment['ids']['element'][1:])];_,shape=entities[product[6][1]]
        labels=[entities[ref[1]][1][1] for ref in shape[2]];assert labels==['Body','Reference'],labels

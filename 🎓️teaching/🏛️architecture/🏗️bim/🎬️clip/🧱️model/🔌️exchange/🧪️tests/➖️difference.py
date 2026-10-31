"""➖ Native analytic differences retain exact stock, curves, finite heights and nested ownership operands."""
import importlib,json,copy
from pathlib import Path
E=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🔌️exchange.🐍️')
T=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧪️tests.🐍️')
VECTORS=Path(E.__file__).parent.parent/'🧊️solids/➖️difference/🧪️tests/🔣️.json'
CASE=json.loads(VECTORS.read_text(encoding='utf8'))


def boolean_count(operand):
    """🧮 Count declared recursive set differences independently of native entity allocation."""
    if operand.get('operator')!='difference':return 0
    return len(operand['cutters'])+boolean_count(operand['stock'])+sum(boolean_count(row['operand']) for row in operand['cutters'])


def fragment(operand):
    """📄 Produce a complete self-contained geometry fragment through the normal native writer."""
    plan=E._analytic_geometry_plan(operand,'solid0');keys=plan.keys+['world_point','world_axis','context']+(['operand_representation','mapping_origin','mapping','mapping_point','mapping_target','mapped'] if plan.frame['conditioned'] else []);writer=E._Writer(101,keys)
    writer.lines=['ISO-10303-21;','HEADER;',"FILE_DESCRIPTION(('Analytic operand contract'),'2;1');",f"FILE_NAME('{E.FILENAME}','{E.STAMP}',('Teaching'),('semio'),'','','');",f"FILE_SCHEMA(('{E.SCHEMA}'));",'ENDSEC;','DATA;']
    plan.write(writer);writer.add('world_point','IFCCARTESIANPOINT',E._coords([0.,0.,0.]));writer.add('world_axis','IFCAXIS2PLACEMENT3D',writer.ref('world_point'),'$','$');writer.add('context','IFCGEOMETRICREPRESENTATIONCONTEXT','$',E._str('Model'),'3','1.E-05',writer.ref('world_axis'),'$');writer.world_root=plan.root
    if plan.frame['conditioned']:
        writer.add('operand_representation','IFCSHAPEREPRESENTATION',writer.ref('context'),'$',E._str('CSG' if operand.get('operator')=='difference' else 'SolidModel'),E._tuple(writer.ref(plan.root)));writer.add('mapping_origin','IFCAXIS2PLACEMENT3D',writer.ref('world_point'),'$','$');writer.add('mapping','IFCREPRESENTATIONMAP',writer.ref('mapping_origin'),writer.ref('operand_representation'));writer.add('mapping_point','IFCCARTESIANPOINT',E._coords(plan.frame['origin']));writer.add('mapping_target','IFCCARTESIANTRANSFORMATIONOPERATOR3D','$','$',writer.ref('mapping_point'),E._real(1/plan.frame['scale']),'$');writer.add('mapped','IFCMAPPEDITEM',writer.ref('mapping'),writer.ref('mapping_target'));writer.world_root='mapped'
    writer.lines+=['ENDSEC;','END-ISO-10303-21;']
    return plan,writer,T.step_parse(writer.lines)['entities']


def test_all_shared_analytic_cases_have_exact_native_difference_trees():
    """🌳 Emit general Boolean differences with original analytic profile operands, never cylinder facets."""
    for row in CASE['cases']:
        operand=row['part']['csg'];plan,writer,entities=fragment(operand)
        assert entities[writer.ids[plan.root]][0]=='IFCCSGSOLID',row['id']
        booleans=[arguments for name,arguments in entities.values() if name=='IFCBOOLEANRESULT']
        assert len(booleans)==boolean_count(operand) and all(arguments[0]==('enum','DIFFERENCE') for arguments in booleans),row['id']
        if 'profile_extrusion' in json.dumps(operand):assert any(name in ('IFCCIRCLEPROFILEDEF','IFCCIRCLE','IFCELLIPSEPROFILEDEF','IFCELLIPSE') for name,_ in entities.values()),row['id']
        assert not any(name in ('IFCBOOLEANCLIPPINGRESULT','IFCPOLYGONALFACESET','IFCTRIANGULATEDFACESET') for name,_ in entities.values()),row['id']
        if 'polyhedron' not in json.dumps(operand):assert not any(name=='IFCFACETEDBREP' for name,_ in entities.values()),row['id']


def test_reference_plan_preserves_nominal_stock_without_borrowed_body_operands():
    """🧭 Full Reference serializes declared original stock rather than copying its outer ownership cuts."""
    for row in CASE['cases']:
        stock=row['part']['csg']['stock'];plan,writer,entities=fragment(stock)
        assert sum(name=='IFCBOOLEANRESULT' for name,_ in entities.values())==boolean_count(stock),row['id']
        assert entities[writer.ids[plan.root]][0] in ('IFCCSGSOLID','IFCEXTRUDEDAREASOLID','IFCFACETEDBREP'),row['id']


def test_native_service_parts_keep_analytic_authority_without_a_display_profile():
    """📦 The native product view retains the actual CSG part instead of manufacturing a sampled profile."""
    for row in CASE['cases']:
        part=row['part'];body=E._service({'tag':'NEUTRAL'},{'details':[{'tag':'NEUTRAL','parts':[part]}]})
        assert body['parts'][0]==part and body['depth']>0,row['id']


def test_native_polyhedron_rejects_open_warped_or_inconsistently_oriented_faces():
    """🔒 Invalid source boundary topology is rejected instead of projected, healed or replaced by a hull."""
    backend=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🔌️exchange.➖️difference.🐍️');source=next(primitive for row in CASE['cases'] for primitive in backend.primitives(row['part']['csg']) if primitive['kind']=='polyhedron')
    invalid=[];opened=copy.deepcopy(source);opened['faces'].pop();invalid.append(opened);reversed_face=copy.deepcopy(source);reversed_face['faces'][0].reverse();invalid.append(reversed_face);warped=copy.deepcopy(source);warped['vertices'][warped['faces'][0][0]][2]+=.001;invalid.append(warped)
    for operand in invalid:
        try:E._analytic_geometry_plan(operand,'solid0')
        except ValueError:continue
        raise AssertionError('An invalid native polyhedron was silently accepted')


def test_native_anonymous_construction_boundaries_do_not_invent_owner_identities():
    """🫥 Geometric clipping authority remains distinct from a physical material or reserved-air owner."""
    plans=[E._analytic_geometry_plan(row['part']['csg'],'solid0') for row in CASE['cases']];boundaries=[operation for plan in plans for operation in plan.operations if operation['role']=='construction_boundary'];assert boundaries
    assert all('owner' not in operation and 'owner_guid' not in operation for operation in boundaries)

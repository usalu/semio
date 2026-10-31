"""🔗 Source material associations follow authored assemblies and their real constituent references."""
import copy,importlib,json
from pathlib import Path
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')

def _material_owners(source,obj):
    """🌿 Resolve only schema material-reference fields, typed construction and nonvoid material layers."""
    registry=api._registry(source);materials={r['tag']for r in source['materials']};found=set();seen=set()
    def visit(value):
        """🧵 Traverse declared material references without accepting arbitrary labels or evidence prose."""
        if isinstance(value,list):
            for item in value:visit(item)
        elif isinstance(value,dict)and not value.get('is_void',False):
            for key,item in value.items():
                if isinstance(item,str)and(key=='material'or key.endswith('_material')or key=='substrate')and item in materials:
                    found.add(item)
                    if item not in seen:seen.add(item);visit(registry[item])
                elif isinstance(item,(list,dict)):visit(item)
    parts=obj.get('geometry',{}).get('parts',[])
    explicit=bool(parts)and all(p.get('material')for p in parts)
    visit({k:v for k,v in obj.items()if not explicit or k not in ('material','type')})
    if not explicit and obj.get('type'):visit(registry[obj['type']])
    nominal=obj.get('material')
    if explicit and nominal in registry and registry[nominal]['class']=='IfcMaterialLayerSet':
        found.add(nominal);visit(registry[nominal])
    return found

def _assert_relations(source):
    """🛡️ Reject every material association that is absent from its target's authored construction."""
    registry=api._registry(source);owners={};invalid=[]
    for relation in source['relations']:
        if relation['class']!='IfcRelAssociatesMaterial':continue
        assert relation['relating']in registry,relation['tag']
        for tag in relation['related']:
            assert tag in registry,(relation['tag'],tag)
            if tag not in owners:owners[tag]=_material_owners(source,registry[tag])
            if relation['relating']not in owners[tag]:invalid.append({'relation':relation['tag'],'target':tag,'material':relation['relating'],'actual':sorted(owners[tag])})
    assert not invalid,invalid

def test_material_ownership_portable_cases_reject_stale_scalar_assignments():
    """🧬 Independently validate typed composite/assembly examples and explicit negative material references."""
    from jsonschema import Draft202012Validator
    case=json.loads(Path(__file__).with_suffix('.json').read_text());Draft202012Validator(json.loads(Path(__file__).with_suffix('.schema.json').read_text())).validate(case);source=api.load_model();registry=api._registry(source)
    for row in case['rows']:
        actual=_material_owners(source,registry[row['tag']]);assert set(row['accepted'])<=actual,(row['tag'],actual);assert not set(row['rejected'])&actual,(row['tag'],actual)
        if 'forged_nominal'in row:
            forged=copy.deepcopy(registry[row['tag']]);forged['material']=row['forged_nominal'];assert row['forged_nominal']not in _material_owners(source,forged),(row['tag'],'unused scalar nominal cannot legitimise a stale association')

def test_all_source_material_relations_refer_to_authored_assemblies_or_constituents():
    """🧱 Preserve valid mixed constructions while rejecting stale wall layer-set references globally."""
    _assert_relations(api.load_model())


def test_material_inheritance_uses_the_exact_authored_occurrence_type_graph():
    """🌳 Preserve a single exact type owner for every occurrence before inheriting its material assembly."""
    source=api.load_model();registry=api._registry(source);actual={}
    for relation in source['relations']:
        if relation['class']=='IfcRelDefinesByType':
            for tag in relation['related']:actual.setdefault(tag,[]).append(relation['relating'])
    invalid=[{'target':tag,'linked':types,'authored':registry[tag].get('type')}for tag,types in actual.items()if types!=[registry[tag].get('type')]]
    assert not invalid,invalid
    for obj in source['elements']:
        if obj.get('type'):assert actual.get(obj['tag'])==[obj['type']],obj['tag']
    for row in json.loads(Path(__file__).with_suffix('.json').read_text())['typed_occurrences']:assert actual[row['tag']]==[row['type']],row['tag']

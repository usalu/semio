"""🎨 Source-domain architectural coverings retain native material bodies outside MEP dispatch."""
from copy import deepcopy
import ast,importlib,json
from pathlib import Path
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')

def test_architectural_covering_dispatch_uses_source_domains_and_native_material():
    """🧱 GEOS independently verifies the same-class floor/skirting/paint and service-casing bodies."""
    from jsonschema import Draft202012Validator
    from shapely.geometry import Polygon
    oracle=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧪️tests.🐍️')
    case=json.loads(Path(__file__).with_suffix('.json').read_text())
    Draft202012Validator(json.loads(Path(__file__).with_suffix('.schema.json').read_text())).validate(case)
    m=api.load_model();template=api._find(m,'RAIN-KITCHEN-PATIO-9-COVER');objects=[]
    for row in case['cases']:
        obj=deepcopy(template);obj.update(tag=row['tag'],guid=api.global_id(row['tag']),predefined_type=row['predefined_type'],geometry={'kind':'flooring' if row['predefined_type']=='FLOORING' else 'synthetic_covering','base_level':'ground','parts':[deepcopy(case['part'])]},attributes={},psets={})
        obj['metadata']['domains']=row['domains'];obj['name']={'en':'Neutral covering','de':'Neutraler Belag'}
        if row['system'] is None:obj.pop('system',None)
        else:obj['system']=row['system']
        assert api.is_architectural_covering(obj)==row['architectural'],row['tag']
        objects.append(obj)
    m['elements']+=objects
    element_schema=json.loads((Path(api.__file__).parent/'🧬️schema/🔣️.schema.json').read_text())
    validator=Draft202012Validator({'$schema':element_schema['$schema'],'$defs':element_schema['$defs'],'$ref':'#/$defs/element'})
    for obj,row in zip(objects,case['cases']):
        validator.validate(obj)
        volume=Polygon(case['part']['outline']).area*(case['part']['z1']-case['part']['z0'])
        assert abs(volume-case['expected_volume'])<1e-12
        assert abs(api.quantities(obj['tag'],m)['volume']-volume)<1e-12
        for kind in ('axis','footprint','plan','section','elevation','3d','body'):
            representation=api.represent(obj['tag'],kind,m)
            assert representation['primitives']
            assert ('historical_state' not in representation['derived'])==row['architectural']
        for language in ('en','de'):
            fragment=api.step_excerpt(obj['tag'],m,language=language)
            parsed=oracle.step_parse(fragment['lines'])['entities']
            entity,attributes=parsed[int(fragment['ids']['element'][1:])]
            assert entity=='IFCCOVERING' and len(attributes)==9
            assert attributes[8]==('enum',row['predefined_type'])
    invalid=deepcopy(objects[1]);rain=api._find(m,'DP-1');invalid['geometry']=deepcopy(rain['geometry']);invalid['attributes']=deepcopy(rain['attributes'])
    assert not validator.is_valid(invalid),'Architectural coverings require owned solid parts, not a MEP tube'
    tree=ast.parse(Path(api.__file__).read_text());node=next(n for n in tree.body if isinstance(n,ast.FunctionDef) and n.name=='_building_geometry_uncached');returned=next(n.value for n in node.body if isinstance(n,ast.Return))
    local={**m,'elements':objects};scope={'m':local,'_solid_geometry':api._solid_geometry,'_service':api._service,'is_architectural_covering':api.is_architectural_covering}
    for key in ('details','services'):
        expression=next(value for name,value in zip(returned.keys,returned.values) if isinstance(name,ast.Constant) and name.value==key)
        selected=eval(compile(ast.Expression(expression),str(api.__file__),'eval'),scope)
        assert {o['tag'] for o in selected}=={r['tag'] for r in case['cases'] if r['architectural']==(key=='details')}

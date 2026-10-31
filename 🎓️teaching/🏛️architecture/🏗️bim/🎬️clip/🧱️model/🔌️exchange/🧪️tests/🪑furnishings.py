"""🪑 Native furnishing identity and owned material quantities follow the real IFC entity."""
import importlib,json,math,unittest
from copy import deepcopy
from pathlib import Path
from scipy.spatial import ConvexHull
from shapely.geometry import Polygon,Point
from shapely.ops import unary_union
prefix='🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.'
api=importlib.import_module(prefix+'🧱️model.🐍️');oracle=importlib.import_module(prefix+'🧪️tests.🐍️')
class NativeFurnishingContract(unittest.TestCase):
    def setUp(self):
        self.case=json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf8'));self.model=api.load_model()
    def _fragment(self,tag,model,language):
        fragment=api.step_excerpt(tag,model,language=language);parsed=oracle.step_parse(fragment['lines'])['entities']
        def record(key):return parsed[int(fragment['ids'][key][1:])]
        entity,attributes=record('element');obj=api.element(tag,model)
        self.assertEqual(entity,'IFCFURNISHINGELEMENT');self.assertEqual(len(attributes),len(self.case['attributes']))
        self.assertEqual(attributes[0],obj['guid']);self.assertEqual(oracle.step_decode(attributes[2]),obj['name'][language]);self.assertEqual(attributes[7],tag)
        self.assertFalse(any(s['role']=='predefined' for s in fragment['spans']))
        kind,values=record('quantity_volume');self.assertEqual(kind,'IFCQUANTITYVOLUME');self.assertEqual(values[0],self.case['quantity']['name']);self.assertEqual(len(values),5)
        self.assertAlmostEqual(values[3],api.quantities(tag,model)['volume'],places=8)
        kind,unit=record('quantity_unit');self.assertEqual(kind,'IFCSIUNIT');self.assertEqual(unit,['*',('enum','VOLUMEUNIT'),None,('enum','CUBIC_METRE')])
        kind,qset=record('quantity_set');self.assertEqual(kind,'IFCELEMENTQUANTITY');self.assertEqual(qset[2],self.case['quantity_set']);self.assertEqual(len(qset),6)
        self.assertIn(int(fragment['ids']['quantity_volume'][1:]),oracle.step_refs(qset[5]))
        kind,relation=record('quantity_relation');self.assertEqual(kind,'IFCRELDEFINESBYPROPERTIES');self.assertIn(int(fragment['ids']['element'][1:]),oracle.step_refs(relation[4]));self.assertEqual(relation[5],('ref',int(fragment['ids']['quantity_set'][1:])))
        self.assertIn('rel_storey',fragment['ids']);self.assertIn('rel_material',fragment['ids'])
        return fragment,parsed
    def test_schema_and_information_have_no_fabricated_predefined_attribute(self):
        import jsonschema
        schema=json.loads((Path(__file__).parents[2]/'🧬️schema/🔣️.schema.json').read_text(encoding='utf8'))
        self.assertEqual(self.model['classes'][self.case['entity']]['adds'],[])
        for obj in self.model['elements']:
            if obj['class']==self.case['entity']:self.assertNotIn('predefined_type',obj)
        jsonschema.Draft202012Validator(schema).validate(self.model)
        invalid=deepcopy(self.model);next(e for e in invalid['elements'] if e['class']==self.case['entity'])['predefined_type']='USERDEFINED'
        self.assertFalse(jsonschema.Draft202012Validator(schema).is_valid(invalid))
        with self.assertRaisesRegex(ValueError,'PredefinedType'):api.information(self.case['source_cases'][0]['tag'],'PredefinedType','en',self.model)
        self.assertEqual(api.information(self.case['source_cases'][0]['tag'],'GlobalId','de',self.model)['value'],api.element(self.case['source_cases'][0]['tag'],self.model)['guid'])
    def test_mixed_owned_solids_have_native_materials_and_explicit_volume_units(self):
        case=self.case['synthetic'];model=deepcopy(self.model);obj={'tag':case['tag'],'guid':api.global_id(case['tag']),'class':self.case['entity'],'name':{'en':'Synthetic owned furnishing','de':'Synthetische Ausstattung'},'storey':'ground','material':case['fallback_material'],'geometry':deepcopy(case['geometry']),'attributes':{},'psets':{},'metadata':{'author':'portable fixture','revision':1,'domains':['architecture']}}
        model['elements'].append(obj);parts=api.solid_geometry(case['tag'],model)['parts'];volume=sum(ConvexHull(api.part_vertices(p)).volume for p in parts)
        self.assertAlmostEqual(volume,case['volume'],places=10);self.assertAlmostEqual(api.quantities(case['tag'],model)['volume'],volume,places=9)
        for language in ('en','de'):
            fragment,parsed=self._fragment(case['tag'],model,language);self.assertIn('constituentset',fragment['ids']);self.assertEqual(sum(entity=='IFCMATERIALCONSTITUENT' for entity,_ in parsed.values()),3)
        for part in obj['geometry']['parts']:part.setdefault('material',obj['material'])
        obj.pop('material')
        self._fragment(case['tag'],model,'en')
    def test_source_hollow_furnishings_keep_real_native_parts_and_volume(self):
        cavity=json.loads((Path(api.__file__).parent/self.case['cavity_fixture']).read_text(encoding='utf8'))
        witnesses={c['tag']:c for c in cavity['cabinets']}
        for case in self.case['source_cases']:
            tag=case['tag'];obj=api.element(tag,self.model)
            self.assertEqual(obj['class'],self.case['entity'])
            parts=api.solid_geometry(tag,self.model)['parts']
            self.assertEqual({p['material'] for p in parts},set(cavity['materials']))
            self.assertTrue(set(cavity['roles']).issubset({p['role'] for p in parts}))
            levels=sorted({p[k] for p in parts for k in ('z0','z1')})
            volume=sum(unary_union([Polygon(p['outline'],p.get('holes',[])) for p in parts if p['z0']<(a+b)/2<p['z1']]).area*(b-a) for a,b in zip(levels,levels[1:]))
            self.assertGreater(volume,0)
            self.assertAlmostEqual(api.quantities(tag,self.model)['volume'],volume,places=9)
            floor=next(row['elevation']['value'] for row in self.model['storeys'] if row['id']==obj['geometry']['base_level'])
            for x,y,z in witnesses[tag]['cavities']:
                self.assertFalse(any(p['z0']<floor+z<p['z1'] and Polygon(p['outline'],p.get('holes',[])).covers(Point(x,y)) for p in parts))
            for language in ('en','de'):
                fragment,parsed=self._fragment(tag,self.model,language)
                self.assertEqual(sum(entity=='IFCEXTRUDEDAREASOLID' for entity,_ in parsed.values()),len(parts))

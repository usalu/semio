"""🏗️ Complete native construction export contracts for source-owned occupied parts."""
import csv,importlib,json,math,re,unittest
from pathlib import Path
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
CASE=json.loads(Path(__file__).with_suffix('.json').read_text())

class ConstructionExport(unittest.TestCase):
    """🧱 Keep footing and railing identities and solids in both native exchange languages."""
    def test_all_source_foundations_and_railings_have_native_bodies(self):
        """📄 Compare actual native solid counts and schema attributes against every authored occurrence."""
        m=api.load_model()
        for obj in m['elements']:
            if obj['class'] not in CASE['physical_classes']:continue
            parts=api.solid_geometry(obj['tag'],m)['parts']
            for language in CASE['languages']:
                export=api.step_excerpt(obj['tag'],m,language=language)
                line=export['lines'][export['entities']['element']]
                values=next(csv.reader([line[line.index('(')+1:-2]],delimiter=',',quotechar="'"))
                self.assertEqual(len(values),CASE['attributes'],obj['tag'])
                self.assertEqual(values[0],obj['guid']);self.assertEqual(re.sub(r'\\X2\\([0-9A-F]+)\\X0\\',lambda hit:bytes.fromhex(hit[1]).decode('utf-16-be'),values[2]),obj['name'][language]);self.assertEqual(values[7],obj['tag'])
                self.assertEqual(values[8],'.'+obj['predefined_type']+'.')
                self.assertEqual(len([key for key in export['ids'] if key.startswith('solid')]),len(parts))
                self.assertIn('rel_storey',export['ids']);self.assertIn('rel_material',export['ids'])
    def test_native_construction_parts_match_independent_convex_volumes(self):
        """📐 Validate closed construction pieces with independent SciPy three-dimensional hulls."""
        import numpy as np
        from scipy.spatial import ConvexHull
        from shapely.geometry import Polygon
        m=api.load_model()
        for obj in m['elements']:
            if obj['class'] not in CASE['physical_classes']:continue
            g=api.solid_geometry(obj['tag'],m)
            volume=sum(ConvexHull(np.asarray(api.part_vertices(p))).volume if any(k in p for k in ('mesh','section','top_outline')) else Polygon(p['outline'],p.get('holes',[])).area*(p['z1']-p['z0']) for p in g['parts'])
            self.assertTrue(math.isclose(volume,sum(api._polyhedron_volume(p) for p in g['parts']),abs_tol=CASE['tolerance']),(obj['tag'],volume))

    def test_native_construction_materials_include_every_owned_part(self):
        """🧵 Preserve mixed material guards rather than replacing their RC parts with the envelope material."""
        m=api.load_model()
        for obj in m['elements']:
            if obj['class'] not in CASE['physical_classes']:continue
            parts=api.solid_geometry(obj['tag'],m)['parts'];materials={p.get('material',obj.get('material')) for p in parts}
            expected={next(v for v in m['materials'] if v['tag']==tag)['name']['en'] for tag in materials}
            export=api.step_excerpt(obj['tag'],m)
            names={next(csv.reader([line[line.index('(')+1:-2]],delimiter=',',quotechar="'"))[0] for line in export['lines'] if '=IFCMATERIAL(' in line}
            self.assertEqual(names,expected,obj['tag'])

    def test_opening_exports_preserve_native_subtraction_and_host_ownership(self):
        """🕳️ Export actual wall, curved wall, slab and curtain voids without occupied material or spatial containment."""
        exchange=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🔌️exchange.🐍️')
        self.assertIn(CASE['void_class'],exchange.BODIES)
        m=api.load_model()
        for tag in CASE['opening_witnesses']:
            obj=api.element(tag,m);host=next(obj['geometry'][k] for k in ('wall','slab','curtain_wall') if k in obj['geometry'])
            relation=next(r for r in m['relations'] if r['class']==CASE['void_relationship'] and r['relating']==host and r['related']==[tag])
            for language in CASE['languages']:
                export=api.step_excerpt(tag,m,language=language);line=export['lines'][export['entities']['void_relation']]
                self.assertIn("'"+relation['guid']+"'",line)
                self.assertIn(export['ids']['host.element'],line);self.assertIn(export['ids']['element'],line)
                self.assertIn('=IFCRELVOIDSELEMENT(',line)
                self.assertIn("'Reference'",export['lines'][export['entities']['representation']])
                self.assertNotIn('rel_storey',export['ids']);self.assertNotIn('rel_material',export['ids'])
                self.assertIn('host.rel_storey',export['ids']);self.assertEqual(export['scope'],'selected_subtraction_and_native_host')

    def test_source_native_guids_follow_the_canonical_tag_namespace(self):
        """🪪 Keep every native source identity consistent with the stable independently recomputed tag contract."""
        import uuid
        case=CASE['identity'];m=api.load_model();objects=[obj for group in case['groups'] for obj in m[group]]+[m[key] for key in case['roots']]
        for obj in objects:
            value=uuid.uuid5(uuid.UUID(case['namespace']),case['seed']+obj['tag']).int;digits=[]
            for _ in range(22):
                value,digit=divmod(value,64);digits.append(case['alphabet'][digit])
            expected=''.join(reversed(digits))
            self.assertEqual(obj['guid'],expected,obj['tag']);self.assertEqual(api.global_id(obj['tag']),expected)

    def test_portable_construction_acceptance_matches_its_schema(self):
        """🧬 Validate language-neutral native exchange cases with the independent JSON Schema implementation."""
        from jsonschema import Draft202012Validator
        schema=json.loads(Path(__file__).with_suffix('.schema.json').read_text())
        Draft202012Validator.check_schema(schema);Draft202012Validator(schema).validate(CASE)

    def test_opening_source_spatial_ownership_is_exclusively_its_authored_host(self):
        """🕳️ Enforce native IFC4.3 opening ownership without a conflicting direct storey relationship."""
        m=api.load_model();registry={e['tag']:e for e in m['elements']}
        for obj in m['elements']:
            if obj['class']!=CASE['void_class']:continue
            contained=[r for r in m['relations'] if r['class']=='IfcRelContainedInSpatialStructure' and obj['tag'] in r['related']]
            self.assertEqual(bool(contained),CASE['void_spatial_containment'],obj['tag'])
            host=next(obj['geometry'][k] for k in ('wall','slab','curtain_wall') if k in obj['geometry'])
            voids=[r for r in m['relations'] if r['class']==CASE['void_relationship'] and r['related']==[obj['tag']]]
            self.assertEqual(len(voids),1,obj['tag']);self.assertEqual(voids[0]['relating'],host)
            self.assertIn(host,registry)
            host_containment=[r for r in m['relations'] if r['class']=='IfcRelContainedInSpatialStructure' and host in r['related']]
            self.assertEqual(len(host_containment),1,host)

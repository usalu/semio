"""📐 Native material subtraction preserves simple rings and occupied source domains."""
import importlib,json,unittest
from pathlib import Path
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
analysis=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧮️analysis.🐍️')
CASES=json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf8'))

class MaterialSubtraction(unittest.TestCase):
    """🪚 Validate published residuals against independent GEOS without geometric repair."""
    def test_language_neutral_subtraction_contract(self):
        """🧬 Validate every exact source operand before invoking the first-party implementation."""
        from jsonschema import Draft202012Validator
        schema=json.loads(Path(__file__).with_suffix('.schema.json').read_text(encoding='utf8'))
        Draft202012Validator(schema).validate(CASES)

    def test_simple_ring_predicate_matches_independent_topology(self):
        """🔬 Detect microscopic crossings while retaining finite nanometre material."""
        from shapely.geometry import Polygon
        for case in CASES['rings']:
            with self.subTest(case=case['id']):
                self.assertEqual(Polygon(case['ring']).is_valid,case['valid'])
                self.assertEqual(analysis._simple(case['ring']),case['valid'])

    def test_replayed_roof_residuals_have_valid_disjoint_native_rings(self):
        """🏠 Reproduce both native roof failures using ordered source operands and independent GEOS."""
        from shapely.geometry import Polygon
        from shapely.ops import unary_union
        for case in CASES['cases']:
            with self.subTest(case=case['id']):
                result=api._subtract_prisms(case['parts'],case['cutters'])
                self.assertTrue(result)
                self.assertTrue(all(Polygon(p['outline']).is_valid for p in result))
                levels=sorted({p[k] for p in [*case['parts'],*case['cutters']] for k in ('z0','z1')})
                for lo,hi in zip(levels,levels[1:]):
                    z=(lo+hi)/2
                    original=unary_union([Polygon(p['outline'],p.get('holes',())) for p in case['parts'] if p['z0']<z<p['z1']])
                    removed=unary_union([Polygon(p['outline'],p.get('holes',())) for p in case['cutters'] if p['z0']<z<p['z1']])
                    rings=[Polygon(p['outline']) for p in result if p['z0']<z<p['z1']]
                    actual=unary_union(rings)
                    self.assertLessEqual(actual.symmetric_difference(original.difference(removed)).area,1e-10)
                    self.assertAlmostEqual(sum(p.area for p in rings),actual.area,delta=1e-10)
                self.assertTrue(all((p.get('region'),p.get('layer')) in {(q.get('region'),q.get('layer')) for q in case['parts']} for p in result))

    def test_exact_replay_preserves_perforated_cutter_material(self):
        """🕳️ Preserve real cutter cavities in exact source arithmetic and compare independent GEOS occupancy."""
        from shapely.geometry import Polygon
        from shapely.ops import unary_union
        case=next(c for c in CASES['cases'] if c['id']=='perforated-z-bands')
        outline=case['parts'][0]['outline'];cutter=case['cutters'][0]
        rings=api._prism_residuals(outline,[cutter],exact=True)
        self.assertTrue(all(Polygon(p).is_valid for p in rings))
        actual=unary_union([Polygon(p) for p in rings])
        expected=Polygon(outline).difference(Polygon(cutter['outline'],cutter['holes']))
        self.assertLessEqual(actual.symmetric_difference(expected).area,1e-12)
        self.assertAlmostEqual(sum(Polygon(p).area for p in rings),expected.area,delta=1e-12)

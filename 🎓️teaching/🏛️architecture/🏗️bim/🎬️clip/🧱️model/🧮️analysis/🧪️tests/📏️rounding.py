"""📏 Exact ring normalization preserves every resolvable finite authored facet."""
import importlib,json,unittest
from pathlib import Path
analysis=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧮️analysis.🐍️')
CASE=json.loads(Path(__file__).with_suffix('.json').read_text())

class RoundedRings(unittest.TestCase):
    """✂️ Compare exact occupied facets with independent GEOS at nanometre resolution."""
    def test_exact_normalization_preserves_two_nanometre_positive_facet(self):
        """🔬 Retain every vertex/order of a resolvable2nm triangle against independent GEOS area."""
        from shapely.geometry import Polygon
        case=CASE["facet"]
        for reverse in (False,True):
            for offset in range(len(case["ring"])):
                ring=case["ring"][offset:]+case["ring"][:offset]
                result=analysis._ccw(list(reversed(ring)) if reverse else ring)
                self.assertEqual(len(result),3)
                shape=Polygon(result)
                self.assertTrue(shape.is_valid)
                self.assertAlmostEqual(shape.area,case["area"],delta=case["absolute_tolerance"])
                self.assertAlmostEqual(analysis._area(result),shape.area,delta=case["absolute_tolerance"])

    def test_quantization_cleanup_preserves_resolvable_thin_material(self):
        """🔬 Preserve a ten-nanometre strip through exact ring normalization."""
        from shapely.geometry import Polygon
        ring=analysis._ccw(CASE['thin'])
        self.assertEqual(len(ring),4)
        self.assertEqual(Polygon(ring).area,Polygon(CASE['thin']).area)
                                                                                                                                                                                          
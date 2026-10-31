"""📏 Preserve original mitered material when stations lie close to path corners."""
import importlib,json,unittest
from pathlib import Path
API=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
ANALYSIS=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧮️analysis.🐍️')
CASES=json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf-8'))
class PathStationContract(unittest.TestCase):
    """🧱 Validate the actual producer against GEOS independently of native profile formatting."""
    def test_near_corner_stations_keep_valid_disjoint_full_material(self):
        """📐 Partition original mitered strips without folding short station bands."""
        import jsonschema
        from shapely.geometry import Polygon
        from shapely.ops import unary_union
        jsonschema.validate(CASES,json.loads(Path(__file__).with_suffix('.schema.json').read_text(encoding='utf-8')))
        for case in CASES['cases']:
            with self.subTest(case=case['id']):
                base=API._path_bands(case['path'],case['thickness'])
                parts=API._path_bands(case['path'],case['thickness'],case['stations'])
                polygons=[Polygon(p['outline']) for p in parts]
                self.assertTrue(all(p.is_valid and p.area>0 for p in polygons))
                expected=unary_union([Polygon(p['outline']) for p in base])
                actual=unary_union(polygons)
                self.assertLess(actual.symmetric_difference(expected).area,CASES['tolerance'])
                self.assertLess(abs(sum(p.area for p in polygons)-actual.area),CASES['tolerance'])
                boundaries={p[k] for p in parts for k in ('start','end')}
                self.assertTrue(set(case['stations']).issubset(boundaries))
    def test_layers_and_authoritative_aperture_preserve_independent_area(self):
        """🚪 Keep each material strip and the perpendicular aperture independent of subdivision."""
        from shapely.geometry import Polygon
        from shapely.ops import unary_union
        for case in CASES['cases']:
            for lo,hi in ((0.,.25),(.25,.75),(.75,1.)):
                sets=[]
                for stations in ((),case['stations']):
                    pieces=[]
                    material=[]
                    for band in API._path_bands(case['path'],case['thickness'],stations):
                        a,b,c,d=band['outline']
                        ring=[[p[i]+f*(q[i]-p[i]) for i in (0,1)] for p,q,f in ((a,d,lo),(b,c,lo),(b,c,hi),(a,d,hi))]
                        material.append(Polygon(ring))
                        pieces.extend(ANALYSIS._subtract(ring,case['aperture'],digits=12,tolerance=1e-12))
                    self.assertTrue(all(Polygon(p).is_valid for p in pieces))
                    sets.append(unary_union([Polygon(p) for p in pieces]))
                    expected=unary_union(material).difference(Polygon(case['aperture']))
                    self.assertLess(sets[-1].symmetric_difference(expected).area,CASES['tolerance'])
                self.assertLess(sets[0].symmetric_difference(sets[1]).area,CASES['tolerance'])
    def test_authored_shared_wall_prisms_survive_native_decimal_profile_precision(self):
        """🏛️ Keep source brick/plaster and the real door cut valid before and after nine-decimal serialization."""
        from shapely.geometry import Polygon
        model=API.load_model()
        wall=API._wall_geometry(model,API.element(CASES['source_wall'],model))
        self.assertTrue(any(o['tag']==CASES['source_opening'] for o in wall['openings']))
        for index,part in enumerate(wall['parts']):
            with self.subTest(part=index,material=part['material']):
                raw=Polygon(part['outline'])
                native=Polygon([[float(f'{v:.{CASES["digits"]}f}') for v in point] for point in part['outline']])
                self.assertTrue(raw.is_valid)
                self.assertTrue(native.is_valid)
                self.assertGreater(raw.area,0)
                self.assertLess(raw.symmetric_difference(native).area,CASES['tolerance'])
if __name__=='__main__':
    unittest.main()

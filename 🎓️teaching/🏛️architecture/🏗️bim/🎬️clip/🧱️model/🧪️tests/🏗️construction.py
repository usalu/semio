"""🏗️ Check sourced construction details and independently conserved occupied volumes."""
import importlib,json,math,unittest
from pathlib import Path
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
class ConstructionDetails(unittest.TestCase):
    def test_exterior_ramp_soffit_has_battens_and_board(self):
        from scipy.spatial import ConvexHull
        source=api.load_model();case=json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf8'))
        for tag in case['soffits']:
            obj=api._find(source,tag);parts=api._solid_geometry(source,obj)['parts']
            self.assertEqual({p['role'] for p in parts},{'soffit_board_inferred','timber_batten_35mm'})
            self.assertIn('inferred',obj['historical_evidence']['status'])
            self.assertTrue(api.is_architectural_covering(obj))
            self.assertTrue(all(api._occupied_volume([p])>0 for p in parts))
            for p in parts:
                api._polyhedron_part(api.part_vertices(p),api.part_faces(p))
                self.assertAlmostEqual(api._occupied_volume([p]),ConvexHull(api.part_vertices(p)).volume,places=9)
            row=next(r for r in case['soffit_interfaces']if r['cover']==tag)
            self.assertEqual(len(parts),row['parts']);self.assertAlmostEqual(sum(ConvexHull(api.part_vertices(p)).volume for p in parts),row['retained_volume'],places=9)
    def test_stair_nosings_are_flush_and_partition_tile(self):
        from shapely.geometry import Polygon
        from shapely.ops import unary_union
        source=api.load_model();case=json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf8'))
        count=0
        for tag in case['stairs']:
            g=api.stair_geometry(tag,source);count+=len(g['nosings'])//2
            self.assertEqual(len(g['nosings']),2*len(g['steps']))
            for step in g['steps']:
                top=step['z1'];zs=[top-.03,top-.025,top-.004,top]
                for lo,hi in zip(zs,zs[1:]):
                    z=(lo+hi)/2;own=[p for p in g['finish_parts']+g['nosings'] if p['z0']<z<p['z1']]
                    outlines=[Polygon(api.part_outline_at(p,z)) for p in own]
                    union=unary_union(outlines);gross=unary_union([Polygon(p['outline']) for p in g['steps'] if p['z0']<z<p['z1']])
                    self.assertAlmostEqual(sum(p.area for p in outlines),union.area,places=8)
                    self.assertAlmostEqual(union.area,gross.area,places=8)
        self.assertEqual(count,case['mapped_nosings'])
    def test_pilotis_gravel_is_bounded_under_building(self):
        source=api.load_model();g=api._solid_geometry(source,api._find(source,'FIN-PILOTIS-GRAVEL'))
        from shapely.geometry import Polygon
        from shapely.ops import unary_union
        self.assertTrue(g['parts'])
        polys=[Polygon(p['outline'])for p in g['parts']]
        self.assertAlmostEqual(sum(api._area(p['outline'])for p in g['parts']),unary_union(polys).area,places=7)
        for p in g['parts']:
            self.assertEqual(p['material'],'MAT-GRAVEL')
            self.assertLessEqual(p['z1'],0)
            self.assertTrue(all(0<=x<=19 and 0<=y<=21.5 for x,y in p['outline']))
    def test_central_tube_is_hollow_black_and_distinct_from_drain(self):
        source=api.load_model();obj=api._find(source,'TUBE-STAIR-CENTRAL');g=api._solid_geometry(source,obj)
        self.assertEqual(obj['attributes']['diameter']['value'],.055)
        self.assertEqual(obj['material'],'MAT-STAIR-BLACK-METAL')
        self.assertGreater(len(g['parts']),32)
        self.assertTrue(all(p['z0']==-2.4 for p in g['parts']))
        self.assertNotEqual(obj['geometry'].get('axis'),api._find(source,'SOIL-STAIR')['geometry'].get('axis'))
    def test_original_roof_beds_have_true_shell_and_soil(self):
        from shapely.geometry import Polygon
        from shapely.ops import unary_union
        source=api.load_model()
        for tag in ('FIX-ROOF-BED-1','FIX-ROOF-BED-2'):
            obj=api._find(source,tag);parts=api._solid_geometry(source,obj)['parts'];z=6.6+.15
            active=[Polygon(api.part_outline_at(p,z))for p in parts if p['z0']<z<p['z1']]
            self.assertAlmostEqual(sum(p.area for p in active),unary_union(active).area,places=8)
            self.assertTrue(any(p['material']=='MAT-PLANTING-SOIL'for p in parts))
            self.assertTrue(any(p['role']=='planter_rim_inferred' and p['z1']>6.9 for p in parts))

"""🧱 Preserve inferred wall material outside source-authoritative inclined RC solids."""
import copy,importlib,json,math,unittest
from pathlib import Path
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
class WallInterfaces(unittest.TestCase):
    def test_inclined_interface_keeps_real_slope_and_conserves_material(self):
        from scipy.spatial import ConvexHull
        from shapely.geometry import Point,Polygon
        case=json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf8'));source=api.load_model();wall=copy.deepcopy(api._find(source,case['wall']));wall['geometry']['interface_owners']=[case['owner']]
        source['elements'].append({'tag':case['owner'],'guid':api.global_id(case['owner']),'class':'IfcRailing','name':{'en':'Test RC','de':'Testbeton'},'storey':'ground','material':'MAT-RC','geometry':{'kind':'guard','base_level':'ground','parts':[case['cutter']]},'metadata':{'domains':['architecture']}})
        raw=copy.deepcopy(case['stock']);parts,removed=api._wall_interface_parts(source,wall,[raw])
        self.assertEqual(raw,case['stock'])
        self.assertAlmostEqual(ConvexHull(case['cutter']['mesh']['vertices']).volume,case['expected']['cutter_volume'],places=12)
        native=sum(api._polyhedron_volume(p)if'mesh'in p else api._partition_volume([p])for p in parts)
        oracle=sum(ConvexHull(api.part_vertices(p)).volume for p in parts)
        self.assertAlmostEqual(native,oracle,places=12);self.assertAlmostEqual(native,case['expected']['retained_volume'],places=12);self.assertAlmostEqual(removed,case['expected']['cutter_volume'],places=12)
        for point in case['expected']['retained_points']:
            self.assertTrue(any(p['z0']<=point[2]<=p['z1'] and Polygon(api.part_outline_at(p,point[2])).covers(Point(point[:2]))for p in parts))
        for point in case['expected']['removed_points']:
            self.assertFalse(any(p['z0']<point[2]<p['z1'] and Polygon(api.part_outline_at(p,point[2])).contains(Point(point[:2]))for p in parts))
        self.assertTrue(all(p['material']==raw['material'] and p['layer']==raw['layer'] and p['region']==raw['region']for p in parts))
    def test_explicit_native_owner_is_not_precarved_as_vertical_shadow(self):
        from unittest.mock import patch
        case=json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf8'));source=api.load_model()
        shadow={'outline':[[0,.2],[1,.2],[1,.8],[0,.8]],'z0':.2,'z1':.8,'owner':case['owner']}
        with patch.object(api,'_wall_cutters',return_value=[shadow]):
            parts,removed,volume=api._wall_owned_parts(source,[case['stock']],[case['owner']])
        self.assertEqual(parts,[case['stock']]);self.assertEqual(removed,0);self.assertEqual(volume,case['expected']['stock_volume'])
    def test_global_native_cutter_keeps_inclined_material_instead_of_its_shadow(self):
        from unittest.mock import patch
        from shapely.geometry import Polygon
        from shapely.ops import unary_union
        case=json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf8'));source=api.load_model()
        for row in case['native_shapes']:
            with self.subTest(shape=row['id']):
                native=copy.deepcopy(row['part']);shape=api._polyhedron_part(api.part_vertices(native),api.part_faces(native))
                for key in ('outline','z0','z1'):native.setdefault(key,shape[key])
                native['owner']=case['owner']
                with patch.object(api,'_wall_cutters',return_value=[native]):
                    parts,removed,volume=api._wall_owned_parts(source,[case['stock']])
                self.assertAlmostEqual(removed,row['removed_volume'],places=12)
                self.assertAlmostEqual(volume,row['retained_volume'],places=12)
                expected=Polygon(case['stock']['outline']).difference(Polygon(api.part_outline_at(native,.5)))
                actual=unary_union([Polygon(api.part_outline_at(p,.5))for p in parts if p['z0']<.5<p['z1']])
                self.assertLess(actual.symmetric_difference(expected).area,1e-10)
                self.assertTrue(any('mesh'in p for p in parts))

    def test_wall_material_does_not_fill_reserved_column_bore_air(self):
        from unittest.mock import patch
        from shapely.geometry import Polygon
        from shapely.ops import unary_union
        case=json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf8'));source=api.load_model();column=copy.deepcopy(next(e for e in source['elements']if e['class']=='IfcColumn'))
        column['geometry']={'axis':[.5,.5],'base_level':0,'top_level':1};source['elements']=[column]
        with patch.object(api,'column_geometry',return_value={'parts':[case['column_bore']['part']]}):
            parts,removed,volume=api._wall_owned_parts(source,[case['stock']])
        expected=Polygon(case['stock']['outline']).difference(Polygon(case['column_bore']['part']['outline']))
        actual=unary_union([Polygon(api.part_outline_at(p,.5))for p in parts])
        self.assertLess(actual.symmetric_difference(expected).area,1e-10)
        self.assertAlmostEqual(volume,case['column_bore']['expected_wall_volume'],places=12)
    def test_native_difference_preserves_a_cutters_real_hole_without_reserving_air(self):
        from shapely.geometry import Polygon
        from shapely.ops import unary_union
        from scipy.spatial import ConvexHull
        case=json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf8'));cutter=case['column_bore']['part']
        parts,removed=api._subtract_native_material([case['stock']],[cutter])
        expected=Polygon(case['stock']['outline']).difference(Polygon(cutter['outline'],cutter['holes']))
        actual=unary_union([Polygon(api.part_outline_at(p,.5))for p in parts if p['z0']<.5<p['z1']])
        self.assertLess(actual.symmetric_difference(expected).area,1e-10)
        volume=sum(ConvexHull(api.part_vertices(p)).volume for p in parts)
        self.assertAlmostEqual(volume,case['column_bore']['expected_physical_difference_volume'],places=12)
        self.assertAlmostEqual(removed,1-volume,places=12)
        json.dumps(parts)
    def test_native_cache_tracks_geometry_producer_identity(self):
        from unittest.mock import patch
        case=json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf8'));backend=api.surface_finishes();source=api.load_model()
        native={**case['cutter'],**api._polyhedron_part(case['cutter']['mesh']['vertices'],case['cutter']['mesh']['faces']),'owner':case['owner']}
        for name in case['cache_producers']:
            for scope in ('native','wall'):
                with self.subTest(producer=name,scope=scope),patch.object(api,'_wall_cutters',return_value=[native]):
                    api._WALL_INTERFACES.clear();api._WALL_OWNERSHIP.clear()
                    def query():
                        return api._subtract_native_material([case['stock']],[case['cutter']])if scope=='native'else api._wall_owned_parts(source,[case['stock']])
                    query();module=api if hasattr(api,name)else backend;original=getattr(module,name);calls=[]
                    def tracked(*args,**kwargs):
                        calls.append(1)
                        return original(*args,**kwargs)
                    with patch.object(module,name,tracked):query()
                    self.assertTrue(calls,'A changed producer must not reuse old native geometry')
    def test_exact_native_inputs_keep_lossless_distinct_cache_keys(self):
        from fractions import Fraction
        from unittest.mock import patch
        case=json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf8'));stock=copy.deepcopy(case['stock']);stock['z1']=Fraction(1);calls=[];original=api._subtract_convex_material
        def tracked(*args,**kwargs):
            calls.append(1)
            return original(*args,**kwargs)
        api._WALL_INTERFACES.clear()
        with patch.object(api,'_subtract_convex_material',tracked):
            first=api._subtract_native_material([stock],[case['cutter']])
            exact=case['exact_cache'];stock['z1']=Fraction(int(exact['numerator']),int(exact['denominator']))
            self.assertEqual(float(stock['z1']),exact['float_value'])
            second=api._subtract_native_material([stock],[case['cutter']])
        self.assertEqual(len(calls),exact['kernel_calls'])
        json.dumps(first);json.dumps(second)
        untouched=copy.deepcopy(stock);remote=copy.deepcopy(case['cutter'])
        remote['mesh']['vertices']=[[x+5,y,z]for x,y,z in remote['mesh']['vertices']]
        for cutters in ([],[remote]):
            result,removed=api._subtract_native_material([stock],cutters)
            self.assertEqual(removed,0);self.assertEqual(stock,untouched)
            self.assertEqual(set(result[0]),set(stock));self.assertIsInstance(result[0]['z1'],float)
            json.dumps(result)
    def test_global_and_explicit_native_planes_publish_each_wall_stock_once(self):
        from unittest.mock import patch
        case=json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf8'));source=api.load_model();row=case['combined_native']
        primary={**case['cutter'],**api._polyhedron_part(case['cutter']['mesh']['vertices'],case['cutter']['mesh']['faces']),'owner':'PRIMARY-RC'}
        extra=copy.deepcopy(case['cutter']);extra['mesh']['vertices']=[[x,y+row['extra_y_shift'],z]for x,y,z in extra['mesh']['vertices']]
        with patch.object(api,'_wall_cutters',return_value=[primary]),patch.object(api,'_subtract_native_material',wraps=api._subtract_native_material)as native:
            parts,removed,volume=api._wall_owned_parts(source,[case['stock']],(),[extra])
        self.assertEqual(native.call_count,row['kernel_batches']);self.assertEqual(len(native.call_args.args[1]),row['cutters'])
        self.assertAlmostEqual(removed,row['removed_volume'],places=12);self.assertAlmostEqual(volume,row['retained_volume'],places=12)
        json.dumps(parts)

    def test_vertical_and_inclined_interfaces_keep_one_original_stock(self):
        from unittest.mock import patch
        from scipy.spatial import ConvexHull
        case=json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf8'));source=api.load_model();row=case['combined_prism']
        with patch.object(api,'_wall_cutters',return_value=[row['cutter']]),patch.object(api,'_subtract_native_material',wraps=api._subtract_native_material)as native:
            parts,removed,volume=api._wall_owned_parts(source,[case['stock']],(),[case['cutter']])
        self.assertEqual(native.call_count,row['kernel_batches']);self.assertEqual(len(native.call_args.args[0]),1);self.assertEqual(len(native.call_args.args[1]),row['cutters'])
        self.assertAlmostEqual(removed,row['removed_volume'],places=12);self.assertAlmostEqual(volume,row['retained_volume'],places=12)
        self.assertAlmostEqual(sum(ConvexHull(api.part_vertices(p)).volume for p in parts),row['retained_volume'],places=12)

    def test_original_guard_and_structural_union_matches_exclusive_guard_material(self):
        from scipy.spatial import ConvexHull
        case=json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf8'))['guard_union']
        exclusive,_=api._subtract_native_material([case['guard']],[case['structure']])
        a,_=api._subtract_native_material([case['wall']],[case['guard'],case['structure']])
        b,_=api._subtract_native_material([case['wall']],[*exclusive,case['structure']])
        for parts in (a,b):self.assertAlmostEqual(sum(ConvexHull(api.part_vertices(p)).volume for p in parts),case['retained_volume'],places=12)

    def test_guard_reference_stocks_keep_source_affine_controls(self):
        from fractions import Fraction
        row=json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf8'))['guard_reference'];source=api.load_model();guard=copy.deepcopy(api._find(source,row['guard']));guard['geometry']['interface_construction']=row['construction']
        from jsonschema import Draft202012Validator
        schema=json.loads(Path(api.__file__).with_name('🧬️schema').joinpath('🔣️.schema.json').read_text(encoding='utf8'));Draft202012Validator({'$ref':'#/$defs/geometry', '$defs':schema['$defs']}).validate(guard['geometry'])
        parts=api._guard_reference_stocks(source,guard);self.assertEqual(len(parts),row['parts'])
        self.assertEqual(parts[0]['mesh']['vertices'][1][0]-parts[0]['mesh']['vertices'][0][0],Fraction(*map(int,row['core_width'])))
        self.assertTrue(all(isinstance(v,Fraction)for p in parts for row in p['mesh']['vertices']for v in row))
        self.assertEqual(parts[1]['mesh']['vertices'][0][0],Fraction(*map(int,row['cap_first_x'])))

    def test_interface_wall_stocks_keep_private_source_affine_coordinates(self):
        from fractions import Fraction
        from unittest.mock import patch
        case=json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf8'))['wall_affine'];source=api.load_model();wall=copy.deepcopy(api._find(source,case['wall']));wall['geometry']['interface_owners']=['GUARD-RM-2'];seen=[]
        original=api._wall_owned_parts
        def capture(model,parts,*args):
            seen.extend(parts)
            return original(model,parts,(),()) if not parts else (api._native_public_value(parts),0.,sum(api._polyhedron_volume(p)for p in parts))
        with patch.object(api,'_wall_owned_parts',side_effect=capture):api._wall_geometry(source,wall)
        self.assertTrue(seen);self.assertTrue(all(isinstance(v,Fraction)for p in seen for row in p['outline']for v in row))

    def test_stringer_reference_keeps_original_flight_plane_and_section(self):
        from fractions import Fraction
        row=json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf8'))['stringer_reference'];source=api.load_model();beam=copy.deepcopy(api._find(source,row['beam']));beam['geometry']['interface_construction']=row['construction']
        part=api._ramp_stringer_reference_stock(source,beam);flight=api._ramp_reference_stock(source,api._find(source,row['construction']['flight']))
        self.assertEqual(part['mesh']['vertices'][0][0],max(p[0]for p in flight['mesh']['vertices']))
        self.assertEqual(part['mesh']['vertices'][1][0]-part['mesh']['vertices'][0][0],Fraction(*map(int,row['width'])))
        self.assertEqual(part['mesh']['vertices'][4][2]-part['mesh']['vertices'][0][2],Fraction(*map(int,row['depth'])))
        self.assertEqual(part['mesh']['vertices'][4][2],flight['mesh']['vertices'][0][2])

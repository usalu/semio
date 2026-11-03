"""🪈️ Preserve hollow tube material and shared native miters with an independent scientific oracle."""
import importlib,json,math,unittest
from pathlib import Path
api=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🐍️')
profiles=importlib.import_module('🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.🧱️model.🧊️profiles.🐍️')
class HollowTubes(unittest.TestCase):
    def test_shared_miters_preserve_normal_radius_air_and_conserved_metal(self):
        from jsonschema import Draft202012Validator
        import numpy as np
        from scipy.spatial import ConvexHull
        case=json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf8'));Draft202012Validator(json.loads(Path(__file__).with_suffix('.schema.json').read_text(encoding='utf8'))).validate(case)
        r,inner=case['diameter']/2,case['diameter']/2-case['gauge'];n=case['segments']
        for row in case['cases']:
            with self.subTest(row=row['id']):
                stock=profiles.hollow_polyline(row['points'],case['diameter'],case['gauge'],n)
                json.dumps(stock);self.assertEqual(len(stock['metal']),n*(len(row['points'])-1))
                expected=n*math.sin(2*math.pi/n)/2*(r*r-inner*inner)*row['length'];actual=0.
                for part in stock['metal']:
                    native=api._polyhedron_part(part['mesh']['vertices'],part['mesh']['faces']);oracle=ConvexHull(part['mesh']['vertices']).volume
                    self.assertGreater(oracle,0);self.assertTrue(math.isclose(api._polyhedron_volume(native),oracle,rel_tol=case['comparison']['volume_relative']));actual+=oracle
                self.assertTrue(math.isclose(actual,expected,rel_tol=case['comparison']['volume_relative']))
                for index,(a,b) in enumerate(zip(row['points'],row['points'][1:])):
                    axis=np.asarray(b)-np.asarray(a);axis=axis/np.linalg.norm(axis)
                    for ring,center in ((stock['rings'][index],a),(stock['rings'][index+1],b)):
                        delta=np.asarray(ring)-np.asarray(center);normal=delta-np.outer(delta@axis,axis)
                        self.assertLessEqual(float(np.max(np.abs(np.linalg.norm(normal,axis=1)-r))),case['comparison']['radius_absolute'])
                    midpoint=(np.asarray(a)+np.asarray(b))/2
                    for part in stock['metal'][index*n:(index+1)*n]:
                        eq=ConvexHull(part['mesh']['vertices']).equations
                        self.assertGreater(float(np.max(eq[:,:3]@midpoint+eq[:,3])),case['comparison']['clearance_margin'])
                for left,right in zip(stock['outer'],stock['outer'][1:]):self.assertEqual(left['mesh']['vertices'][n:],right['mesh']['vertices'][:n])
    def test_private_exact_path_does_not_publish_rational_scalars(self):
        from fractions import Fraction
        stock=profiles.hollow_polyline([[0,0,0],[1,0,0],[1,1,1]],.035,.002,24,exact=True)
        self.assertTrue(all(isinstance(v,Fraction)for ring in stock['rings']for point in ring for v in point))
        json.dumps(profiles.hollow_polyline([[0,0,0],[1,0,0],[1,1,1]],.035,.002,24))

    def test_source_handrails_keep_positive_metal_and_exposed_air_cores(self):
        import numpy as np
        from scipy.spatial import ConvexHull
        case=json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf8'));source=api.load_model()
        for tag in case['source_products']:
            obj=api._find(source,tag);self.assertTrue('tube_construction'in obj['geometry'],tag);construction=obj['geometry']['tube_construction']
            self.assertEqual(construction['wall_thickness']['basis'],'inferred');self.assertEqual(construction['diameter']['value'],case['diameter'])
            parts=api._solid_geometry(source,obj,resolve_embedded=False)['parts'];prepared=[]
            for p in parts:
                native=api._polyhedron_part(api.part_vertices(p),api.part_faces(p));vertices=np.asarray(api.part_vertices(p));origin=vertices.min(axis=0);span=vertices.max(axis=0)-origin
                self.assertTrue(np.all(span>0));hull=ConvexHull((vertices-origin)/span);oracle=hull.volume*float(np.prod(span));self.assertGreater(oracle,0);self.assertTrue(math.isclose(api._polyhedron_volume(native),oracle,rel_tol=1e-7,abs_tol=1e-24))
                prepared.append((origin,span,hull.equations,p.get('role')))
            self.assertIn(tag,case['source_air_controls']);self.assertTrue(case['source_air_controls'][tag])
            for point in case['source_air_controls'][tag]:
                for origin,span,eq,role in prepared:
                    if np.any(np.asarray(point)<origin)or np.any(np.asarray(point)>origin+span):continue
                    distance=(eq[:,:3]@((np.asarray(point)-origin)/span)+eq[:,3])/np.linalg.norm(eq[:,:3]/span,axis=1)
                    self.assertGreaterEqual(float(np.max(distance)),-case['comparison']['clearance_margin'],(tag,point))

            for row in (r for r in case['source_bearing_controls']if r['product']==tag):
                x,y=row['xy'];intervals=[]
                for origin,span,eq,role in prepared:
                    if role!='ramp_guard_post'or not(origin[0]<=x<=origin[0]+span[0]and origin[1]<=y<=origin[1]+span[1]):continue
                    normals=eq[:,:3]/span;world=np.column_stack([normals,eq[:,3]-normals@origin]);lo,hi=-math.inf,math.inf;outside=False
                    for nx,ny,nz,d in world:
                        offset=nx*x+ny*y+d
                        if nz==0:
                            if offset>1e-10:outside=True;break
                        elif nz>0:hi=min(hi,-offset/nz)
                        else:lo=max(lo,-offset/nz)
                    if not outside and hi>lo+1e-12:intervals.append(lo)
                self.assertTrue(intervals,(tag,row['route'],row['xy']));self.assertLessEqual(abs(min(intervals)-row['expected_foot']),case['bearing_tolerance'])
            if tag=='TUBE-RM-2':
                terminal=next(r for r in construction['routes']if r['id']=='post_6');self.assertEqual(terminal['points'][0][1],8.2475);self.assertEqual(terminal['terminal_inset']['value'],.0175);self.assertEqual(terminal['terminal_inset']['basis'],'inferred')

    def test_circular_samples_preserve_exact_common_cartesian_symmetry(self):
        from fractions import Fraction
        case=json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf8'));n=case['segments'];stock=profiles.hollow_polyline(case['symmetry_path'],case['diameter'],case['gauge'],n,exact=True);ring=stock['rings'][0];r=Fraction(str(case['diameter']))/2
        self.assertEqual(ring[0][0],r);self.assertEqual(ring[n//4][0],0)
        for i in range(n//2):self.assertEqual([ring[i][k]+ring[i+n//2][k]for k in range(3)],[0,0,0])
        for row in case['cases']:
            route=profiles.hollow_polyline(row['points'],case['diameter'],case['gauge'],n,exact=True)
            for a,planes in zip(route['outer'],route['cap_planes'][1:]):
                self.assertTrue(any(all(sum(plane['normal'][k]*(p[k]-plane['point'][k])for k in range(3))>=0 for p in a['mesh']['vertices'])for plane in planes))
                self.assertTrue(api.surface_finishes().stock_separated(a,planes))

    def test_route_anchor_preserves_exact_rail_intersection_after_terminal_inset(self):
        from fractions import Fraction
        import numpy as np
        case=json.loads(Path(__file__).with_suffix('.json').read_text(encoding='utf8'))['route_binding'];routes=profiles.tube_routes(case['routes'],exact=True);a,b=routes[0]['points'];point=routes[1]['points'][-1]
        parameter=(point[1]-a[1])/(b[1]-a[1]);self.assertEqual(point[2],a[2]+parameter*(b[2]-a[2]));self.assertEqual(point[:2],list(map(Fraction,case['expected_top'])));self.assertEqual(point[2],Fraction(*map(int,case['expected_fraction'])))
        oracle=np.interp(float(parameter),[0,1],[float(a[2]),float(b[2])]);self.assertAlmostEqual(float(point[2]),float(oracle),places=14)
        json.dumps(profiles.tube_routes(case['routes']))

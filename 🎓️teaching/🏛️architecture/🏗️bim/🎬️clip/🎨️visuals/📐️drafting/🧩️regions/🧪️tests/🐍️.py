"""🧩 Exact owned polygon unions and production boundary ink, independently checked with GEOS."""
import importlib,json,math,unittest
from pathlib import Path
from types import SimpleNamespace
from unittest.mock import patch
from threading import RLock
import numpy as np
from jsonschema import Draft202012Validator
from shapely.geometry import Polygon,LineString
from shapely.ops import unary_union
PREFIX='🎓️teaching.🏛️architecture.🏗️bim.🎬️clip.'
U=importlib.import_module(PREFIX+'🎨️visuals.📐️drafting.🧩️regions.🐍️')
RESULTS=[]
BASE=Path(__file__).parent
FIXTURE=json.loads((BASE/'🔣️.json').read_text(encoding='utf8'))

def oracle(polygons):
    """🌐 Build independently noded GEOS material from the same original operands."""
    return unary_union([Polygon(ring) for ring in polygons])

def material(regions):
    """🕳️ Interpret every explicit outer and hole ring independently of the implementation."""
    return unary_union([Polygon(row['outer'],row['holes']) for row in regions])

class OwnedUnion(unittest.TestCase):
    """📋 Preserve all authored material while removing internal rendering ink."""
    def test_neutral_exact_union_and_complete_boundary(self):
        """📐 Match GEOS area, complete boundary and declarative independent metrics."""
        Draft202012Validator(json.loads((BASE/'🔣️.schema.json').read_text(encoding='utf8'))).validate(FIXTURE)
        for row in FIXTURE['cases']:
            with self.subTest(case=row['id']):
                result=U.polygon_union(row['polygons']);actual=material(result);expected=oracle(row['polygons'])
                self.assertLess(actual.symmetric_difference(expected).area,1e-24)
                self.assertLess(actual.boundary.symmetric_difference(expected.boundary).length,1e-12)
                self.assertAlmostEqual(actual.area,row['area'],places=11)
                self.assertAlmostEqual(actual.length,row['length'],places=11)
                self.assertEqual(len(result),row['components']);self.assertEqual(sum(len(r['holes']) for r in result),row['holes'])
    def test_progress_cancellation_cache_isolation_and_producer_identity(self):
        """🛑 Cancellation interrupts work and cache copies cannot hide changed producers."""
        polygons=FIXTURE['cases'][0]['polygons'];events=[];U.polygon_union(polygons,progress=events.append)
        self.assertEqual(events[-1]['phase'],'complete')
        with self.assertRaises(U.UnionCancelled):U.polygon_union(polygons,cancelled=lambda:True)
        stopped=[]
        def stop(event):
            """🚦 Cancel after actual noding work begins rather than only before entry."""
            stopped.append(event['phase'])
        shifted=[[[p[0]+.125,p[1]+.125] for p in ring] for ring in polygons]
        with self.assertRaises(U.UnionCancelled):U.polygon_union(shifted,progress=stop,cancelled=lambda:'node' in stopped)
        self.assertIn('node',stopped)
        first=U.polygon_union(polygons);first[0]['outer'][0][0]=999
        self.assertNotEqual(U.polygon_union(polygons)[0]['outer'][0][0],999)
        original=U._arrangement;calls=[]
        def replacement(*args):
            calls.append(True);return original(*args)
        with patch.object(U,'_arrangement',replacement):U.polygon_union(polygons)
        self.assertTrue(calls)
    def test_source_affine_tjunction_matches_integer_geos_before_publication(self):
        """🧮 Preserve an actual source-affine partition that develops false ink after independent coordinate rounding."""
        from fractions import Fraction
        from shapely.affinity import scale
        case=json.loads((BASE/'🧮️source.json').read_text(encoding='utf8'));Draft202012Validator(json.loads((BASE/'🧮️source.schema.json').read_text(encoding='utf8'))).validate(case)
        def scalar(value):
            """🔢 Decode a portable exact source scalar without converting it through binary64."""
            return Fraction(value['numerator'],value['denominator']) if isinstance(value,dict) else Fraction(value)
        polygons=[[[scalar(value) for value in point] for point in ring] for ring in case['polygons']]
        transformed=[[[x*case['scale'][0],y*case['scale'][1]] for x,y in ring] for ring in polygons]
        self.assertTrue(all(value.denominator==1 for ring in transformed for point in ring for value in point))
        expected=scale(oracle([[[int(value) for value in point] for point in ring] for ring in transformed]),xfact=1/case['scale'][0],yfact=1/case['scale'][1],origin=(0,0))
        result=U.polygon_union(polygons);actual=material(result)
        self.assertEqual(len(result),case['expected']['components']);self.assertEqual(actual.area,case['expected']['area']);self.assertEqual(actual.length,case['expected']['length'])
        self.assertEqual(actual.symmetric_difference(expected).area,0);self.assertEqual(actual.boundary.symmetric_difference(expected.boundary).length,0)

    def test_exact_closed_material_projection_matches_all_face_geos(self):
        """🧊 Preserve inclined shadows, concavity, holes, disconnected material and source-affine joins."""
        from fractions import Fraction
        from shapely.affinity import scale
        case_file=BASE/'🧊️projection.json';data=json.loads(case_file.read_text(encoding='utf8'));Draft202012Validator(json.loads((BASE/'🧊️projection.schema.json').read_text(encoding='utf8'))).validate(data)
        def scalar(value):
            """🔢 Decode an exact neutral mesh scalar before any projection or publication."""
            return Fraction(value['numerator'],value['denominator']) if isinstance(value,dict) else Fraction(value)
        for case in data['cases']:
            with self.subTest(case=case['id']):
                meshes=[{'vertices':[[scalar(value) for value in p] for p in mesh['vertices']],'faces':mesh['faces']} for mesh in case['meshes']];faces=[]
                for mesh in meshes:
                    for face in mesh['faces']:
                        ring=[mesh['vertices'][index][:2] for index in face]
                        signed=sum(a[0]*b[1]-a[1]*b[0] for a,b in zip(ring,ring[1:]+ring[:1]))
                        if signed==0:continue
                        chart=[[p[i]*case['chart'][i] for i in (0,1)] for p in ring]
                        self.assertTrue(all(value.denominator==1 for p in chart for value in p))
                        faces.append(Polygon([[int(value) for value in p] for p in chart]))
                expected=scale(unary_union(faces),xfact=1/case['chart'][0],yfact=1/case['chart'][1],origin=(0,0));events=[];result=U.mesh_projection(meshes,progress=events.append);actual=material(result)
                self.assertTrue(actual.is_valid);self.assertEqual(actual.symmetric_difference(expected).area,0);self.assertEqual(actual.boundary.symmetric_difference(expected.boundary).length,0)
                self.assertAlmostEqual(actual.area,case['expected']['area'],places=11);self.assertAlmostEqual(actual.length,case['expected']['length'],places=11)
                self.assertEqual(len(result),case['expected']['components']);self.assertEqual(sum(len(row['holes']) for row in result),case['expected']['holes'])
                self.assertIn('project',[event['phase'] for event in events]);self.assertEqual(events[-1]['phase'],'complete');json.dumps(result,allow_nan=False)
                stopped=[]
                with self.assertRaises(U.UnionCancelled):U.mesh_projection(meshes,progress=lambda event:stopped.append(event['phase']),cancelled=lambda:'project' in stopped)

    def test_actual_production_ramp_loop_records_only_owned_union_ink(self):
        """🖌️ Record actual source-owned flight boundaries and independently certify their affine GEOS union."""
        from fractions import Fraction
        H=importlib.import_module(PREFIX+'🎨️visuals.🏛️savoye.🧪️ortho-remodel.test_frustums');C=importlib.import_module(PREFIX+'🎨️visuals.🏛️savoye.🧱️core.📐️geometry');scope=H._scope();scope['_regions']=U;scope['_RAMP_REGIONS_CACHE']={};scope['_RAMP_REGIONS_LOCK']=RLock();scope['deepcopy']=importlib.import_module('copy').deepcopy;A=H.MODEL;model=A.load_model();source=C.source_model(model)
        fills=[];ink=[]
        def region(rings,*args,**kwargs):
            """🎨 Record the real compound-fill interface, including selection opacity."""
            self.assertEqual(args[0],1);self.assertAlmostEqual(args[1],.06);self.assertEqual(kwargs,{})
            fills.append(Polygon(np.asarray(rings[0])[:,:2],[np.asarray(r)[:,:2] for r in rings[1:]]));return H.Shape()
        def polygon(points,*args,**kwargs):
            """⬟ Retain the pre-repair polygon interface so the regression detects old ink."""
            if len(args)>2 and args[2] is not None:ink.append(LineString(list(np.asarray(points)[:,:2])+[np.asarray(points)[0,:2]]))
            fills.append(Polygon(np.asarray(points)[:,:2]));return H.Shape()
        def line(points,*args,**kwargs):
            """🖊️ Record every emitted boundary segment and its selected pen."""
            self.assertEqual(args[0],2);self.assertAlmostEqual(args[1],1.2);self.assertAlmostEqual(args[2],.54);self.assertTrue(kwargs.get('closed'))
            ink.append(LineString(list(np.asarray(points)[:,:2])+[np.asarray(points)[0,:2]]));return H.Shape()
        scope['_d'].region=region;scope['_d'].polygon=polygon;scope['_d'].polyline=line
        scene=scope['Plan']();scene._pair=lambda *args:(1,2,.6,.2)
        flights=[flight for ramp in [A.ramp_geometry(tag,model) for tag in ('RM-1','RM-2')] for flight in ramp['flights']];expected_by_tag={};neutral_meshes={};affine_metrics={}
        def signatures(parts):
            """🪪 Require every displayed source cell to match the actual public material."""
            return sorted(tuple(sorted(tuple(float(x) for x in p) for p in A.part_vertices(part))) for part in parts)
        for flight in flights:
            cells=A._ramp_owned_reference_parts(source,A._find(source,flight['tag']));self.assertEqual(signatures(cells),signatures(flight['parts']));upper=[];all_faces=[]
            for cell in cells:
                vertices=A.part_vertices(cell)
                for face in A.part_faces(cell):
                    ring=[tuple(vertices[i][:2]) for i in face];signed=sum(a[0]*b[1]-a[1]*b[0] for a,b in zip(ring,ring[1:]+ring[:1]))
                    if signed:all_faces.append(ring)
                    if signed>0:upper.append(ring)
            exact=U._arrangement(tuple(tuple(ring) for ring in upper),lambda *args:None,lambda:None);origin=[min(p[i] for ring in all_faces for p in ring) for i in (0,1)]
            def chart(ring):
                """📏 Transform exact source coordinates before independent binary64 evaluation."""
                return [[float((p[i]-origin[i])*1000) for i in (0,1)] for p in ring]
            geos=unary_union([Polygon(chart(ring)) for ring in all_faces]);own=unary_union([Polygon(chart(r['outer']),[chart(ring) for ring in r['holes']]) for r in exact]);edges=unary_union([LineString(chart(ring+[ring[0]])) for r in exact for ring in (r['outer'],*r['holes'])])
            self.assertEqual(own.symmetric_difference(geos).area,0);self.assertEqual(edges.symmetric_difference(geos.boundary).length,0)
            affine_metrics[flight['tag']]={'area_difference_m2':own.symmetric_difference(geos).area/1e6,'boundary_difference_m':edges.symmetric_difference(geos.boundary).length/1000,'perimeter_m':geos.length/1000,'every_public_cell_equal':True}
            expected_by_tag[flight['tag']]=[{'outer':[[float(x),float(y)] for x,y in r['outer']],'holes':[[[float(x),float(y)] for x,y in ring] for ring in r['holes']]} for r in exact]
        for row in FIXTURE['cases']:
            tag='NEUTRAL-'+row['id'];meshes=[]
            for ring in row['polygons']:
                n=len(ring);meshes.append({'vertices':[[*p,0] for p in ring]+[[*p,1] for p in ring],'faces':[list(reversed(range(n))),list(range(n,2*n))]+[[i,(i+1)%n,n+(i+1)%n,n+i] for i in range(n)]})
            neutral_meshes[tag]=meshes;expected_by_tag[tag]=U.polygon_union(row['polygons']);flights.append({'tag':tag,'z0':0,'z1':1,'start':[0,0],'end':[1,1],'parts':[{'outline':ring,'z0':0,'z1':1} for ring in row['polygons']]})
        original=scope.get('ramp_plan_regions')
        def projection(flight,model,progress=None,cancelled=None):
            """🧪 Supply neutral closed material through the same projection boundary as native flights."""
            if flight['tag'] in neutral_meshes:return U.mesh_projection(neutral_meshes[flight['tag']],progress=progress,cancelled=cancelled)
            return original(flight,model,progress=progress,cancelled=cancelled)
        scope['ramp_plan_regions']=projection
        for flight in flights:
            with self.subTest(flight=flight['tag']):
                RESULTS.append({'flight':flight['tag'],'footprints':[A.part_footprint(p) for p in flight['parts']],'source_affine_geos':affine_metrics.get(flight['tag'])});fills.clear();ink.clear();ctx={'model':source,'g':{'ramp':{'ramps':[{'tag':'R','guid':'r','flights':[flight],'landings':[]}]}},'view':lambda x,y,z:np.array([x,y,z]),'look':None,'level':'L','w':1,'bands':{'L':(-10,20)},'catalogue':[],'put':lambda *args:None}
                scene._plan_ramps(ctx,False);expected=material(expected_by_tag[flight['tag']]);actual=unary_union(fills);lines=unary_union(ink)
                RESULTS[-1].update({'parts':len(flight['parts']),'fills':len(fills),'ink_rings':len(ink),'area':expected.area,'boundary_length':expected.length,'fill_overlap':sum(p.area for p in fills)-expected.area,'ink_excess_length':lines.difference(expected.boundary).length,'ink_missing_length':expected.boundary.difference(lines).length})
                self.assertLess(actual.symmetric_difference(expected).area,1e-13);self.assertLess(abs(sum(p.area for p in fills)-expected.area),1e-12)
                self.assertEqual(lines.symmetric_difference(expected.boundary).length,0);self.assertLess(abs(sum(p.length for p in ink)-expected.length),1e-11)
                self.assertEqual(len(ink),sum(1+len(r['holes']) for r in expected_by_tag[flight['tag']]))
                if flight['tag']=='RF-1A' and original:
                    first=original(flight,source);first[0]['outer'][0][0]+=100;self.assertNotEqual(original(flight,source)[0]['outer'][0][0],first[0]['outer'][0][0]);calls=[];producer=U._arrangement
                    def replacement(*args):
                        """🔁 Require a changed union producer to bypass the warm drawing cache."""
                        calls.append(True);return producer(*args)
                    with patch.object(U,'_arrangement',replacement):
                        original(flight,source);self.assertTrue(calls);calls.clear();original(flight,source);self.assertFalse(calls)
                    with self.assertRaises(U.UnionCancelled):original(flight,source,cancelled=lambda:True)


if __name__=='__main__':unittest.main()

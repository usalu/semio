"""✂️ Validate fragment material, source rays and cancellation through independent numerical/raster oracles."""
import importlib.util
import json
from pathlib import Path
import sys
import unittest
import numpy as np
from PIL import Image,ImageDraw
BASE=Path(__file__).resolve().parents[1]
SPEC=importlib.util.spec_from_file_location('fragment_kernel',BASE/'🐍️.py')
K=importlib.util.module_from_spec(SPEC)
sys.modules[SPEC.name]=K
SPEC.loader.exec_module(K)
C=json.loads((BASE/'🧪️cases/🔣️.json').read_text(encoding='utf-8'))
def faces(case):
    """🧩 Construct project-owned immutable faces from the portable contract."""
    return tuple(K.Face(row['source'],row['polygon'],tuple(map(tuple,row['vertices'])),tuple(row['normal'])) for row in case['faces'])
def area(face):
    """📏 Use NumPy vector area independently of the runtime kernel."""
    vertices=np.asarray(face.vertices,float)
    return float(np.linalg.norm(np.cross(vertices,np.roll(vertices,-1,axis=0)).sum(axis=0))/2)
def hits(rows,point):
    """🔬 Solve actual plane rays and use independent even-odd material membership."""
    result=[]
    for face in rows:
        vertices=np.asarray(face.vertices,float)
        normal=np.asarray(face.normal,float)
        if abs(normal[2])<1e-12:
            continue
        p=np.linalg.solve(np.array([[1.,0,0],[0,1.,0],normal]),[*point,float(normal@vertices[0])])
        polygon=vertices[:,:2]
        other=np.roll(polygon,-1,axis=0)
        edge=other-polygon
        delta=p[:2]-polygon
        cross=edge[:,0]*delta[:,1]-edge[:,1]*delta[:,0]
        along=(delta*edge).sum(axis=1)
        length=np.linalg.norm(edge,axis=1)
        epsilon=64*np.spacing(float(np.abs(np.vstack((polygon,p[:2]))).max()) or 1.0)
        boundary=((length>0)&(np.abs(cross)<=epsilon*length)&(along>=-epsilon*length)&(along<=(edge*edge).sum(axis=1)+epsilon*length)).any()
        boundary|=((length==0)&(np.linalg.norm(delta,axis=1)<=epsilon)).any()
        mask=(polygon[:,1]>p[1])!=(other[:,1]>p[1])
        crossings=polygon[mask,0]+(p[1]-polygon[mask,1])*edge[mask,0]/edge[mask,1]
        if boundary or int((crossings>p[0]).sum())%2:
            result.append((face.source,float(p[2])))
    return result
class FragmentKernel(unittest.TestCase):
    def test_ray_boundary_does_not_inflate_tiny_edges(self):
        case=C['ray_boundary']
        polygon=np.asarray(case['polygon'])
        point=np.asarray(case['point'])
        edge=np.roll(polygon,-1,axis=0)-polygon
        fraction=np.clip(np.einsum('ij,ij->i',point-polygon,edge)/np.einsum('ij,ij->i',edge,edge),0,1)
        distance=np.linalg.norm(point-polygon-fraction[:,None]*edge,axis=1).min()
        self.assertGreater(distance,case['minimum_distance'])
        self.assertTrue(np.any((point<polygon.min(axis=0))|(point>polygon.max(axis=0))))
        face=K.Face(0,0,tuple((x,y,0.) for x,y in polygon),(0.,0.,1.))
        self.assertEqual(bool(hits((face,),point)),case['inside'])

    def test_concave_ears_keep_the_validated_original_origin(self):
        from unittest.mock import patch
        case=next(row for row in C['cases'] if row['id']=='concave-u-split')
        original=next(face for face in faces(case) if len(K._prepare(face,lambda:None))>1)
        with patch.object(K,'_split',wraps=K._split) as split:
            ordered=K.ordered_fragments((original,),(0,0,1))
        self.assertTrue(C['lineage']['preserve_original_origin'])
        self.assertEqual(tuple(split.call_args_list[0].args[1]),tuple(original.vertices[0]))
        self.assertAlmostEqual(sum(area(face) for face in ordered),area(original),places=12)

    def test_original_plane_lineage_consumes_every_public_input_node(self):
        from unittest.mock import patch
        original=faces(C['lineage'])
        self.assertEqual(len(original),C['lineage']['expected_input_faces'])
        counts={'prepared':0,'splits':0}
        prepare,split=K._prepare,K._split
        def counted_prepare(*args):
            rows=prepare(*args)
            counts['prepared']+=len(rows)
            return rows
        def counted_split(*args):
            row=split(*args)
            counts['splits']+=row[0]==2
            return row
        def progress(event):
            if event['phase']=='partition':
                self.assertLessEqual(event['completed'],counts['prepared']+counts['splits'],'Every node must consume at least one material fragment')
        with patch.object(K,'_prepare',side_effect=counted_prepare),patch.object(K,'_split',side_effect=counted_split):
            ordered=K.ordered_fragments(original,tuple(C['lineage']['towards']),progress=progress)
        reused=tuple(K.Face(0,0,f.vertices,f.normal) for f in original)
        repeated=K.ordered_fragments(reused,tuple(C['lineage']['towards']))
        self.assertEqual(sorted((f.vertices,f.normal) for f in ordered),sorted((f.vertices,f.normal) for f in repeated))
        for identity in {(f.source,f.polygon) for f in original}:
            self.assertAlmostEqual(sum(area(f) for f in original if (f.source,f.polygon)==identity),sum(area(f) for f in ordered if (f.source,f.polygon)==identity),places=12)
        for output in ordered:
            candidates=[f for f in original if (f.source,f.polygon,f.normal)==(output.source,output.polygon,output.normal)]
            self.assertTrue(candidates)
            distances=[np.abs((np.asarray(output.vertices)-f.vertices[0])@np.asarray(f.normal)).max() for f in candidates]
            self.assertLess(min(distances),1e-13)
        towards=np.asarray(C['lineage']['towards'],float)
        towards/=np.linalg.norm(towards)
        seed=np.eye(3)[int(np.argmin(np.abs(towards)))]
        horizontal=np.cross(seed,towards)
        horizontal/=np.linalg.norm(horizontal)
        matrix=np.asarray([horizontal,np.cross(towards,horizontal),towards])
        def camera(rows):
            return tuple(K.Face(f.source,f.polygon,tuple(map(tuple,np.asarray(f.vertices)@matrix.T)),tuple(np.asarray(f.normal)@matrix.T)) for f in rows)
        physical_faces,painted_faces=camera(original),camera(ordered)
        projected=np.concatenate([np.asarray(f.vertices)[:,:2] for f in physical_faces])
        low,high=projected.min(axis=0),projected.max(axis=0)
        for x in np.linspace(low[0],high[0],31)[1:-1]:
            for y in np.linspace(low[1],high[1],29)[1:-1]:
                physical,painted=hits(physical_faces,(x,y)),hits(painted_faces,(x,y))
                self.assertEqual(bool(physical),bool(painted))
                if physical:
                    self.assertLess(abs(painted[-1][1]-max(depth for _,depth in physical)),1e-10)
    def test_conditioned_normal_uses_local_material_plane(self):
        row=C['conditioned_plane']
        raw=K.Face(row['source'],row['polygon'],tuple(map(tuple,row['vertices'])),tuple(row['normal']))
        with self.assertRaises(ValueError):
            K._prepare(raw,lambda:None)
        vertices=np.asarray(raw.vertices)
        vectors=vertices[1:]-vertices[0]
        candidates=np.cross(vectors[:,None,:],vectors[None,:,:]).reshape(-1,3)
        normal=candidates[np.argmax(np.linalg.norm(candidates,axis=1))]
        normal/=np.linalg.norm(normal)
        if normal@raw.normal<0:
            normal=-normal
        face=K.Face(raw.source,raw.polygon,raw.vertices,tuple(normal))
        prepared=K._prepare(face,lambda:None)
        self.assertEqual(len(prepared),1)
        self.assertEqual(prepared[0].normal,face.normal)
        origin,normal=K._plane(face.vertices,face.normal,lambda:None)
        _,_,vectors=np.linalg.svd(vertices-vertices.mean(axis=0))
        oracle=vectors[-1]
        self.assertLess(abs(abs(np.dot(oracle,normal))-1),1e-12)
        self.assertLess(float(np.abs((vertices-origin)@normal).max()),K._epsilon(vertices))
        self.assertEqual(K.ordered_fragments((face,),(1,1,1))[0].normal,face.normal)
    def test_quadratic_validation_cancels_before_pair_work(self):
        from unittest.mock import patch
        points=tuple((float(np.cos(value)),float(np.sin(value))) for value in np.linspace(0,2*np.pi,12,endpoint=False))
        checks=[0]
        def check():
            checks[0]+=1
            if checks[0]>=C['control']['validation_cancel_after_checks']:
                raise K.FragmentCancelled()
        with patch.object(K,'_turn',wraps=K._turn) as turns:
            with self.assertRaises(K.FragmentCancelled):
                K._simple(points,1e-14,1e-14,check)
            self.assertEqual(turns.call_count,C['control']['validation_expected_turns'])
    def test_invalid_material_and_nonzero_camera_are_rejected(self):
        for case in C['invalid_inputs']:
            with self.subTest(case=case['id']):
                row=K.Face(0,0,tuple(map(tuple,case['vertices'])),tuple(case['normal']))
                with self.assertRaises(ValueError):
                    K.ordered_fragments((row,),(0,0,1))
        with self.assertRaises(ValueError):
            K.ordered_fragments((),(0,0,0))
        self.assertTrue(all(path.is_file() for path in K.source_files()))
    def test_rotated_orthographic_cameras_preserve_source_ray_order(self):
        original=faces(C['cases'][0])
        for values in C['control']['orthographic_rotations']:
            matrix=np.asarray(values,float)
            rotated=tuple(K.Face(face.source,face.polygon,tuple(map(tuple,np.asarray(face.vertices)@matrix.T)),tuple(matrix@np.asarray(face.normal))) for face in original)
            ordered=K.ordered_fragments(rotated,tuple(matrix@np.array([0.,0.,1.])))
            restored=tuple(K.Face(face.source,face.polygon,tuple(map(tuple,np.asarray(face.vertices)@matrix)),tuple(np.asarray(face.normal)@matrix)) for face in ordered)
            for point in ((-.5,.2),(.5,.2)):
                physical=hits(original,point)
                painter=hits(restored,point)
                self.assertAlmostEqual(painter[-1][1],max(depth for _,depth in physical),places=10)
    def test_portable_ray_identities_and_all_source_area(self):
        for case in C['cases']:
            with self.subTest(case=case['id']):
                original=faces(case)
                ordered=K.ordered_fragments(original,tuple(case['towards']))
                self.assertIsInstance(ordered,tuple)
                self.assertGreaterEqual(len(ordered),case['minimum_fragments'])
                for identity in {(face.source,face.polygon) for face in original}:
                    self.assertAlmostEqual(sum(area(face) for face in original if (face.source,face.polygon)==identity),sum(area(face) for face in ordered if (face.source,face.polygon)==identity),places=10)
                for face in ordered:
                    self.assertTrue(all(np.isfinite(face.vertices).flat))
                    self.assertGreater(area(face),0)
                    source=next(row for row in original if (row.source,row.polygon)==(face.source,face.polygon))
                    self.assertEqual(face.normal,source.normal)
                    normal=np.asarray(source.normal,float)
                    self.assertLess(float(np.abs((np.asarray(face.vertices)-source.vertices[0])@normal).max()),1e-10)
                for probe in case['probes']:
                    original_hits=hits(original,probe['point'])
                    nearest=[] if not original_hits else sorted({source for source,depth in original_hits if max(value for _,value in original_hits)-depth<1e-9})
                    self.assertEqual(nearest,probe['nearest'])
                    painter=hits(ordered,probe['point'])
                    self.assertEqual(bool(painter),bool(nearest))
                    if painter:
                        self.assertIn(painter[-1][0],nearest)
                self.assertEqual(ordered,K.ordered_fragments(original,tuple(case['towards'])))
    def test_pillow_union_coverage_and_dense_numpy_rays(self):
        for case in C['cases']:
            original=faces(case)
            ordered=K.ordered_fragments(original,tuple(case['towards']))
            for source in {row.source for row in original}:
                masks=[]
                for group in (original,ordered):
                    image=Image.new('L',(512,512),0)
                    for face in group:
                        if face.source==source and abs(face.normal[2])>1e-12:
                            ImageDraw.Draw(image).polygon([(256+64*x,256-64*y) for x,y,z in face.vertices],fill=255)
                    masks.append(np.asarray(image)>0)
                np.testing.assert_array_equal(*masks,err_msg=case['id'])
            for x in np.linspace(-1.91,1.91,23):
                for y in np.linspace(-1.87,1.87,21):
                    physical=hits(original,(x,y))
                    painted=hits(ordered,(x,y))
                    self.assertEqual(bool(physical),bool(painted))
                    if physical:
                        depth=max(value for _,value in physical)
                        self.assertLess(abs(painted[-1][1]-depth),1e-9,(case['id'],x,y))
    def test_progress_cancellation_and_iterative_depth(self):
        case=C['cases'][0]
        events=[]
        result=K.ordered_fragments(faces(case),tuple(case['towards']),progress=events.append)
        self.assertEqual(events[-1],{'phase':'complete','completed':len(result),'total':len(result)})
        with self.assertRaises(K.FragmentCancelled):
            K.ordered_fragments(faces(case),tuple(case['towards']),cancelled=lambda:True)
        stop=[False]
        def progress(event):
            stop[0]=event['phase']=='partition'
        with self.assertRaises(K.FragmentCancelled):
            K.ordered_fragments(faces(case),tuple(case['towards']),progress=progress,cancelled=lambda:stop[0])
        deep=tuple(K.Face(i,0,((0.,0.,float(i)),(1.,0.,float(i)),(1.,1.,float(i)),(0.,1.,float(i))),(0.,0.,1.)) for i in range(C['control']['deep_stack_count']))
        ordered=K.ordered_fragments(deep,(0.,0.,1.))
        self.assertEqual([row.source for row in ordered],list(range(len(deep))))
        with self.assertRaises(Exception):
            result[0].source=100
if __name__=='__main__':
    unittest.main(verbosity=2)

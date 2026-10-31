"""👁️ Prove painter visibility with portable faces, NumPy ray geometry and the actual Cairo camera."""
import importlib
import json
import inspect
import os
from pathlib import Path
import sys
import unittest
from collections import Counter
import numpy as np
import cairo
from manim import Camera,tempconfig,VGroup
from PIL import Image,ImageDraw,ImageFilter

ROOT=next(p for p in Path(__file__).resolve().parents if (p/'🧰️framework').is_dir())
sys.path.insert(0,str(ROOT))
V=importlib.import_module('🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️')
C=json.loads((ROOT/'🎓️teaching/🏛️architecture/🏢️nakagin/🧪️cases/👁️occlusion/🔣️.json').read_text(encoding='utf-8'))

class ExactCamera(Camera):
    """🔎 Disable native Cairo AA to isolate actual source-face visibility."""
    def get_cairo_context(self,pixel_array):
        context=super().get_cairo_context(pixel_array)
        context.set_antialias(cairo.ANTIALIAS_NONE)
        return context

def ray_faces(items, view, screens):
    """🔬 Solve actual source polygon planes with NumPy SVD and original even-odd boundaries."""
    basis,towards=V._basis(view)
    records=[]
    for index,item in enumerate(items):
        if item.normal is None or item.normal@towards<=1e-6:
            continue
        for polygon in item.polygons or (item.verts,):
            world=np.asarray(polygon)
            normal=np.linalg.svd(world-world[0],full_matrices=False)[2][-1]
            normal=normal if normal@item.normal>=0 else -normal
            if normal@towards<=1e-6:
                continue
            matrix=np.vstack((basis,normal))
            right=np.diag([1.,1.,float(normal@world[0])])
            coefficient=towards@np.linalg.solve(matrix,right)
            points=world@basis.T
            records.append((index,coefficient,points,points.min(axis=0),points.max(axis=0)))
    output=[]
    for screen in screens:
        point=(np.asarray(screen)-np.asarray(view.point)[:2])/view.scale+basis@np.asarray(view.centre)
        hits={}
        for index,coefficient,polygon,lower,upper in records:
            if (point<lower-1e-10).any() or (point>upper+1e-10).any():
                continue
            following=np.roll(polygon,-1,axis=0)
            edges=following-polygon
            delta=point-polygon
            cross=edges[:,0]*delta[:,1]-edges[:,1]*delta[:,0]
            distance=(delta*edges).sum(axis=1)
            boundary=(np.abs(cross)<=1e-10)&(distance>=-1e-10)&(distance<=(edges*edges).sum(axis=1)+1e-10)
            crossing=(polygon[:,1]>point[1])!=(following[:,1]>point[1])
            x=polygon[crossing,0]+(point[1]-polygon[crossing,1])*edges[crossing,0]/edges[crossing,1]
            if boundary.any() or int((x>point[0]).sum())%2:
                hits[index]=max(hits.get(index,float('-inf')),float(coefficient@np.array([*point,1.])))
        output.append(hits)
    return output

class Occlusion(unittest.TestCase):
    def test_cyclic_refinement_can_cancel_before_component_sort(self):
        import networkx as nx
        from unittest.mock import patch
        case=C['refinement_progress']
        component=next(iter(nx.strongly_connected_components(nx.DiGraph(case['edges']))))
        self.assertEqual(component,{0,1})
        state={'cancel':False}
        events=[]
        def progress(event):
            events.append(event)
            if event['phase']==case['phase']:
                state['cancel']=True
        view=V.make_view(tuple(C['view']),(0,0,0),1,(0,0,0))
        with patch.object(V,'_cyclic_components',return_value=iter((component,))),patch.object(V,'_dependency_order',side_effect=AssertionError('Component sorting preceded refinement cancellation')):
            with self.assertRaises(V._fragments.FragmentCancelled):
                V._section_order(self.items(),view,progress=progress,cancelled=lambda:state['cancel'])
        self.assertTrue(any(event['phase']==case['phase'] for event in events))

    def test_sparse_polygon_candidates_match_independent_spatial_index(self):
        from scipy.spatial import KDTree
        case=C['overlap_candidates']
        class Bounds:
            def __init__(self,box):
                self.box=np.asarray(box,float)
                self.reads=0
            @property
            def lower(self):
                self.reads+=1
                return self.box[:2]
            @property
            def upper(self):
                self.reads+=1
                return self.box[2:]
        for fixture in case['cases']:
            with self.subTest(case=fixture['name']):
                first=[Bounds(box) for box in fixture['first']]
                second=[Bounds(box) for box in fixture['second']]
                centres=np.asarray([(np.asarray(box[:2])+box[2:])/2 for box in fixture['second']]).reshape(-1,2)
                tree=KDTree(centres) if len(centres) else None
                extent=max((max((np.asarray(box[2:])-box[:2])/2) for box in fixture['second']),default=0)
                for strict in (False,True):
                    actual=list(V._overlap_candidates(first,second,strict=strict,margin=case['margin'] if strict else 0))
                    expected=fixture['strict' if strict else 'inclusive']
                    oracle=[]
                    for i,box in enumerate(fixture['first']):
                        radius=max((np.asarray(box[2:])-box[:2])/2)+extent
                        candidates=sorted(tree.query_ball_point((np.asarray(box[:2])+box[2:])/2,radius,p=np.inf)) if tree else []
                        for j in candidates:
                            other=fixture['second'][j]
                            margin=case['margin'] if strict else 0
                            overlaps=all(box[k+2]>other[k]+margin and other[k+2]>box[k]+margin for k in (0,1)) if strict else all(box[k+2]>=other[k] and other[k+2]>=box[k] for k in (0,1))
                            if overlaps:
                                oracle.append([i,j])
                    self.assertEqual([list(pair) for pair in actual],expected)
                    self.assertEqual(oracle,expected)
                self.assertLessEqual(sum(bound.reads for bound in first+second),4*(len(first)+len(second)))

    def test_expanded_cover_bound_contains_independent_line_intersections(self):
        for case in C['single_cover_bounds']['cases']:
            with self.subTest(case=case['id']):
                cover=V._prepare_polygon(np.asarray(case['cover'],float))
                lower,upper=V._single_cover_bounds(cover)
                points=cover.points
                edges=np.roll(points,-1,axis=0)-points
                signed=float((points[:,0]*np.roll(points[:,1],-1)-points[:,1]*np.roll(points[:,0],-1)).sum())
                inward=(1 if signed>=0 else -1)*np.column_stack((-edges[:,1],edges[:,0]))/np.maximum(np.linalg.norm(edges,axis=1),1e-15)[:,None]
                levels=np.einsum('ij,ij->i',inward,points)-1e-10
                accepted=[]
                for first in range(len(points)):
                    for second in range(first+1,len(points)):
                        matrix=inward[[first,second]]
                        if np.linalg.det(matrix)==0:
                            continue
                        vertex=np.linalg.solve(matrix,levels[[first,second]])
                        rounding=64*np.finfo(float).eps*max(1,float(np.abs(vertex).max()))
                        if np.all(inward@vertex>=levels-rounding):
                            accepted.append(vertex)
                self.assertGreaterEqual(len(accepted),3)
                self.assertTrue((np.asarray(accepted)>=lower).all())
                self.assertTrue((np.asarray(accepted)<=upper).all())

    def test_expanded_single_cover_bbox_skips_only_impossible_halfplanes(self):
        from unittest.mock import patch
        for case in C['single_cover_bounds']['cases']:
            with self.subTest(case=case['id']):
                points=np.asarray(case['footprint'],float)
                cover=V._prepare_polygon(np.asarray(case['cover'],float))
                area=float((cover.points[:,0]*np.roll(cover.points[:,1],-1)-cover.points[:,1]*np.roll(cover.points[:,0],-1)).sum())
                orientation=1 if area>=0 else -1
                delta=points[:,None,:]-cover.points
                distance=orientation*(cover.edges[:,0]*delta[:,:,1]-cover.edges[:,1]*delta[:,:,0])/np.maximum(np.linalg.norm(cover.edges,axis=1),1e-15)
                oracle=bool((distance>=-1e-10).all())
                self.assertEqual(oracle,case['expected'])
                with patch.object(V,'_cross2',wraps=V._cross2) as halfplanes:
                    actual=V._single_cover([points],cover)
                self.assertEqual(actual,oracle)
                self.assertEqual(halfplanes.call_count==0,case['skip_halfplanes'])

    def test_area_first_blockers_preserve_exact_union_identity(self):
        from unittest.mock import patch
        case=C['single_cover_bounds']['candidate_order']
        view=V.make_view((90,0),(0,0,0),1,(0,0,0))
        items=[]
        for index,row in enumerate([case['target'],*case['blockers']]):
            a,b=row['x'];c,d=row['z'];y=row['y']
            points=np.asarray([[a,y,c],[b,y,c],[b,y,d],[a,y,d]])
            items.append(V.Item(0,points.mean(axis=0),points,lambda view,polygons=None:None,np.array([0.,1.,0.]),(points,),1,'shell',part_key=row.get('key','target')))
        attempts=[]
        original=V._single_cover
        def attempt(footprints,cover,*arguments):
            attempts.append((cover.lower+cover.upper)/2)
            return original(footprints,cover,*arguments)
        V._OPAQUE_VISIBILITY.clear()
        with patch.object(V,'_single_cover',side_effect=attempt):
            optimized=V._opaque_visible(items,view)
        self.assertEqual(len(attempts),case['expected_attempts'])
        np.testing.assert_allclose(attempts[0],case['expected_first_center'],atol=1e-12,rtol=0)
        V._OPAQUE_VISIBILITY.clear()
        with patch.object(V,'_single_cover',return_value=False),patch.object(V,'_coverage_samples_possible',return_value=True):
            exact=V._opaque_visible(items,view)
        self.assertEqual([id(item) for item in optimized],[id(item) for item in exact])
        self.assertEqual([item.part_key for item in optimized],case['expected_retained'])

    def test_single_cover_matches_convex_numpy_and_pillow_oracles(self):
        for case in C['single_cover']:
            with self.subTest(case=case['id']):
                polygon=np.asarray(case['cover'],float)
                footprints=[np.asarray(row,float) for row in case['footprints']]
                edges=np.roll(polygon,-1,axis=0)-polygon
                turns=edges[:,0]*np.roll(edges,-1,axis=0)[:,1]-edges[:,1]*np.roll(edges,-1,axis=0)[:,0]
                convex=bool((turns>=-1e-12).all() or (turns<=1e-12).all())
                area=float((polygon[:,0]*np.roll(polygon[:,1],-1)-polygon[:,1]*np.roll(polygon[:,0],-1)).sum())
                points=np.vstack(footprints)
                delta=points[:,None,:]-polygon
                distances=(edges[:,0]*delta[:,:,1]-edges[:,1]*delta[:,:,0])*(1 if area>=0 else -1)/np.linalg.norm(edges,axis=1)
                oracle=convex and bool((distances>=-1e-10).all())
                self.assertEqual(oracle,case['expected'])
                actual=V._single_cover(footprints,V._prepare_polygon(polygon))
                self.assertIsInstance(actual,bool)
                self.assertEqual(actual,oracle)
                if case['pillow']:
                    masks=[]
                    for rows in ([polygon],footprints):
                        image=Image.new('L',(512,512),0)
                        draw=ImageDraw.Draw(image)
                        for row in rows:
                            draw.polygon([(64*(x+3),64*(y+3)) for x,y in row],fill=255)
                        masks.append(np.asarray(image)>0)
                    self.assertEqual(bool(not (masks[1]&~masks[0]).any()),case['expected'])

    def test_single_cover_shortcut_preserves_strict_depth_and_opacity(self):
        from unittest.mock import patch
        view=V.make_view((90,0),(0,0,0),1,(0,0,0))
        def item(y,extent,opacity,key):
            points=np.array([[-extent,y,-extent],[extent,y,-extent],[extent,y,extent],[-extent,y,extent]],float)
            return V.Item(0,points.mean(axis=0),points,lambda v,polygons=None:VGroup(),np.array([0.,1.,0.]),(points,),opacity,'shell',part_key=key)
        target=item(0,1,1,'target')
        for depth,opacity,hidden in ((1,1,True),(1,.25,False),(-1,1,False),(0,1,False)):
            cover=item(depth,2,opacity,'cover')
            V._OPAQUE_VISIBILITY.clear()
            with patch.object(V,'_convex_difference',wraps=V._convex_difference) as clipping:
                result=V._opaque_visible([target,cover],view)
            self.assertEqual(all(row is not target for row in result),hidden,(depth,opacity))
            if hidden:
                self.assertEqual(clipping.call_count,0,'A proven single cover needs no polygon subtraction')

    def test_dependency_order_preserves_residual_component_dependencies(self):
        import networkx as nx
        for case in C['dependency_order']:
            with self.subTest(case=case['id']):
                edges=[set(targets) for targets in case['edges']]
                untouched=[set(targets) for targets in edges]
                actual=V._dependency_order(edges,case['depths'])
                self.assertEqual(actual,case['expected'])
                self.assertEqual(sorted(actual),list(range(len(edges))))
                self.assertEqual(edges,untouched)
                graph=nx.DiGraph()
                graph.add_nodes_from(range(len(edges)))
                graph.add_edges_from((source,target) for source,targets in enumerate(edges) for target in targets)
                components={node:index for index,component in enumerate(nx.strongly_connected_components(graph)) for node in component}
                rank={node:index for index,node in enumerate(actual)}
                for source,target in graph.edges:
                    if components[source]!=components[target]:
                        self.assertLess(rank[source],rank[target])
                remaining=set(graph)
                for node in actual:
                    residual=graph.subgraph(remaining)
                    component=next(group for group in nx.strongly_connected_components(residual) if node in group)
                    self.assertFalse(any(source not in component for target in component for source in residual.predecessors(target)))
                    remaining.remove(node)

    def test_coverage_witness_matches_expanded_numpy_halfplanes(self):
        spec=C['coverage_witness']
        for case in spec['cases']:
            with self.subTest(case=case['id']):
                footprint=np.asarray(case['footprint'],float)
                probes=np.vstack((footprint,footprint.mean(axis=0)))
                covered=np.zeros(len(probes),bool)
                for points in case['covers']:
                    polygon=np.asarray(points,float)
                    edges=np.roll(polygon,-1,axis=0)-polygon
                    orientation=np.sign(np.linalg.det(np.column_stack((edges[0],edges[1]))))
                    outward=orientation*np.column_stack((edges[:,1],-edges[:,0]))
                    outward/=np.linalg.norm(outward,axis=1)[:,None]
                    covered|=np.all(np.einsum('pki,ki->pk',probes[:,None,:]-polygon,outward)<=spec['expansion'],axis=1)
                self.assertEqual(bool(covered.all()),case['possible'])
                actual=V._coverage_samples_possible(footprint,[V._prepare_polygon(np.asarray(p,float)) for p in case['covers']])
                self.assertEqual(actual,case['possible'])

    def test_coverage_witness_retains_holes_and_skips_only_impossible_clipping(self):
        from unittest.mock import patch
        view=V.make_view((90,0),(0,0,0),1,(0,0,0))
        for case in C['coverage_witness']['cases'][:5]:
            with self.subTest(case=case['id']):
                items=[]
                for name,polygons,depth in (('cover',case['covers'],0),('subject',[case['footprint']],-.1)):
                    world=tuple(np.asarray([(x,depth,z) for x,z in polygon]) for polygon in polygons)
                    vertices=np.vstack(world)
                    items.append(V.Item(0,vertices.mean(axis=0),vertices,lambda v,polygons=None,name=name:name,np.array([0.,1.,0.]),world,1,'shell'))
                images=[]
                for polygons in (case['covers'],[case['footprint']]):
                    image=Image.new('L',(256,256),0)
                    for polygon in polygons:
                        ImageDraw.Draw(image).polygon([(20+100*x,20+100*y) for x,y in polygon],fill=255)
                    images.append(np.asarray(image)>0)
                self.assertEqual(not bool((images[1]&~images[0]).any()),case['hidden'])
                V._OPAQUE_VISIBILITY.clear()
                with patch.object(V,'_convex_difference',wraps=V._convex_difference) as clipping:
                    visible=V._opaque_visible(items,view)
                self.assertEqual(any(item is items[1] for item in visible),not case['hidden'])
                self.assertEqual(clipping.call_count==0,not case['possible'])
                V._OPAQUE_VISIBILITY.clear()
                with patch.object(V,'_coverage_samples_possible',return_value=True):
                    reference=V._opaque_visible(items,view)
                self.assertEqual([id(item) for item in visible],[id(item) for item in reference])
        case=C['coverage_witness']['cases'][0]
        prepared=[V._prepare_polygon(np.asarray(p,float)) for p in case['covers']]
        footprint=np.asarray(case['footprint'],float)
        lower,upper=footprint.min(axis=0)-.05,footprint.max(axis=0)+.05
        padded=np.asarray([lower,[upper[0],lower[1]],upper,[lower[0],upper[1]]])
        self.assertFalse(V._coverage_samples_possible(padded,prepared))

    def test_empty_or_background_only_ordering_needs_no_support_projection(self):
        view=V.make_view(tuple(C['view']),(0,0,0),1,(0,0,0))
        self.assertEqual(V._section_order([],view),[])
        vertices=np.asarray(C['wall']['vertices'],float)
        background=V.Item(-1,vertices.mean(axis=0),vertices,lambda view,polygons=None:None,None)
        ordered=V._section_order([background],view)
        self.assertEqual(len(ordered),1)
        self.assertIs(ordered[0],background)

    def test_sparse_support_intervals_match_portable_values_and_dense_numpy_oracle(self):
        spec=C['support_intervals']
        polygons=[np.asarray(points,float) for points in spec['polygons']]
        vertices=np.vstack(polygons)
        lengths=np.asarray([len(points) for points in polygons],int)
        starts=np.cumsum(np.r_[0,lengths[:-1]])
        normals=np.asarray(spec['normals'],float)
        dense=vertices@normals.T
        minimum=np.minimum.reduceat(dense,starts,axis=0)
        maximum=np.maximum.reduceat(dense,starts,axis=0)
        for case in spec['cases']:
            with self.subTest(case=case['id']):
                candidates=np.asarray(case['candidates']*case['repeat'],int)
                other=case['other']
                expected=np.tile(np.asarray(case['expected'],float),(1,case['repeat']))
                oracle=np.asarray([minimum[candidates,other],maximum[candidates,other],minimum[other,candidates],maximum[other,candidates]])
                np.testing.assert_allclose(oracle,expected,atol=1e-12,rtol=0)
                actual=V._candidate_support(vertices,starts,lengths,candidates,polygons[other],normals,normals[other])
                np.testing.assert_allclose(actual,oracle,atol=1e-12,rtol=0)

    def test_actual_overlap_depth_orders_planes_when_whole_face_intervals_cross(self):
        case=C['overlap_plane_order']
        view=V.make_view(tuple(case['view']),(0,0,0),1,(0,0,0))
        items=[]
        for key in ('back','front'):
            vertices=np.asarray(case[key],float)
            def draw(v,polygons=(vertices,),key=key):
                return V._shape([V._screen(v,p) for p in polygons],'#FF0000' if key=='back' else '#0000FF',1,width=0)
            items.append(V.Item(0,vertices.mean(axis=0),vertices,draw,np.asarray(case[key+'_normal'],float),(vertices,),1,'shell',part_key=key))
        towards=np.asarray(view.towards)
        self.assertGreater(float(np.dot(np.subtract(case['front_probe'],case['back_probe']),towards)),0)
        np.testing.assert_allclose(V._screen(view,np.array([case['front_probe']])),V._screen(view,np.array([case['back_probe']])),atol=1e-12)
        basis,_=V._basis(view)
        polygons=[np.asarray(case[key])@basis.T for key in ('back','front')]
        def inside(point,polygon):
            edges=np.roll(polygon,-1,axis=0)-polygon
            delta=point-polygon
            values=edges[:,0]*delta[:,1]-edges[:,1]*delta[:,0]
            return bool((values>=-1e-12).all() or (values<=1e-12).all())
        probes=[point for i,polygon in enumerate(polygons) for point in polygon if inside(point,polygons[1-i])]
        for i,start in enumerate(polygons[0]):
            end=polygons[0][(i+1)%len(polygons[0])]
            for j,other in enumerate(polygons[1]):
                following=polygons[1][(j+1)%len(polygons[1])]
                matrix=np.column_stack((end-start,other-following))
                if abs(np.linalg.det(matrix))>1e-12:
                    fractions=np.linalg.solve(matrix,other-start)
                    if ((fractions>=0)&(fractions<=1)).all():
                        probes.append(start+fractions[0]*(end-start))
        differences=[]
        for point in probes:
            depths=[]
            for item in items:
                hit=np.linalg.solve(np.vstack((basis,item.normal)),[*point,float(item.normal@item.verts[0])])
                depths.append(float(hit@towards))
            differences.append(depths[1]-depths[0])
        self.assertGreater(min(differences),0)
        ordered=V._section_order(items,view)
        self.assertEqual([item.part_key for item in ordered],case['expected'])
        with tempconfig({'pixel_width':192,'pixel_height':192,'frame_width':6,'frame_height':6,'renderer':'cairo','dry_run':True}):
            camera=ExactCamera(background_color='#000000')
            camera.capture_mobjects([VGroup(*[item.draw(view) for item in ordered])])
            point=V._screen(view,np.asarray([case['front_probe']]))[0]
            column,row=int((point[0]+3)*32),int((3-point[1])*32)
            np.testing.assert_array_equal(camera.pixel_array[row,column,:3],[0,0,255])

    def test_real_glass_faces_composite_in_physical_order_against_pillow(self):
        case=C['glazing']
        parts=V._model.Assembly((V._model.NamedPart('backing','window_blind',V._model._box(-1,1,case['backing_y']-.01,case['backing_y'],0,2),'paper'),V._model.NamedPart('pane','glazing',V._model._box(-1,1,case['glass_y'],case['glass_y']+.006,0,2),'glass')))
        groups={}
        items=V._assembly_items(V._parts(parts),base=V.CAPSULE,groups=groups,glass_opacity=case['opacity'])
        with tempconfig({'pixel_width':128,'pixel_height':128,'frame_width':4,'frame_height':4,'renderer':'cairo','dry_run':True}):
            drawing=V.NakaginDrawing()._compose(items,tuple(case['view']),(0,0,0),1)
            camera=Camera(background_color=V.BACKGROUND)
            camera.capture_mobjects([drawing])
            screen=drawing.view(0,case['glass_y'],1)
            row=round((camera.frame_height/2-screen[1])*camera.pixel_height/camera.frame_height)
            col=round((screen[0]+camera.frame_width/2)*camera.pixel_width/camera.frame_width)
            backing=groups['backing'][0].get_fill_color().to_rgb()
            pane=groups['pane'][0].get_fill_color().to_rgb()
            under=Image.new('RGBA',(1,1),tuple(round(x*255) for x in backing)+(255,))
            over=Image.new('RGBA',(1,1),tuple(round(x*255) for x in pane)+(round(case['opacity']*255),))
            expected=np.asarray(Image.alpha_composite(under,over))[0,0,:3]
            np.testing.assert_allclose(camera.pixel_array[row,col,:3],expected,atol=case['tolerance'],rtol=0)
            self.assertAlmostEqual(groups['pane'][0].get_fill_opacity(),case['opacity'])

    def test_opaque_union_coverage_preserves_real_holes_partial_faces_and_glass(self):
        view=V.make_view((90,0),(0,0,0),1,(0,0,0))
        for case in C['opaque_coverage']:
            with self.subTest(case=case['id']):
                items=[]
                masks=[]
                for name,polygons,depth,opaque in (('wall',case['walls'],0,case['opaque']),('beam',[case['beam']],-.1,True)):
                    world=tuple(np.asarray([(x,depth,z) for x,z in polygon]) for polygon in polygons)
                    vertices=np.vstack(world)
                    item=V.Item(0,vertices.mean(axis=0),vertices,lambda v,polygons=None,name=name:name,np.array([0.,1.,0.]),world,1 if opaque else .25,"shell" if opaque else "glass")
                    items.append(item)
                    mask=Image.new('L',(512,384),0)
                    painter=ImageDraw.Draw(mask)
                    for polygon in polygons:
                        painter.polygon([(64*(x+3),64*(z+1)) for x,z in polygon],fill=255)
                    masks.append(np.asarray(mask)>0)
                oracle=case['opaque'] and not (masks[1]&~masks[0]).any()
                self.assertEqual(oracle,'beam' not in case['expected'])
                visible=V._opaque_visible(items,view)
                self.assertEqual([item.draw(view) for item in visible],case['expected'])
                self.assertTrue(all(any(item is current for current in items) for item in visible))

    def test_unknown_opacity_is_not_an_occluder(self):
        case=C['opaque_coverage'][0]
        view=V.make_view((90,0),(0,0,0),1,(0,0,0))
        items=[]
        for name,polygons,depth in (('wall',case['walls'],0),('beam',[case['beam']],-.1)):
            world=tuple(np.asarray([(x,depth,z) for x,z in polygon]) for polygon in polygons)
            vertices=np.vstack(world)
            items.append(V.Item(0,vertices.mean(axis=0),vertices,lambda v,polygons=None,name=name:name,np.array([0.,1.,0.]),world))
        self.assertEqual([item.draw(view) for item in V._opaque_visible(items,view)],['wall','beam'])

    def test_opaque_coverage_does_not_change_actual_cairo_pixels(self):
        with tempconfig({'pixel_width':192,'pixel_height':128,'frame_width':6,'frame_height':4,'renderer':'cairo','dry_run':True}):
            view=V.make_view((90,0),(0,0,0),1,(0,0,1))
            for case in C['opaque_coverage']:
                with self.subTest(case=case['id']):
                    items=[]
                    for name,polygons,depth,opacity in (('wall',case['walls'],0,1 if case['opaque'] else .25),('beam',[case['beam']],-.1,1)):
                        world=tuple(np.asarray([(x,depth,z) for x,z in polygon]) for polygon in polygons)
                        vertices=np.vstack(world)
                        def draw(v,polygons=world,name=name,opacity=opacity):
                            return V._shape([V._screen(v,p) for p in polygons],'#00FF00' if name=='wall' else '#FF0000',opacity,'#000000',0)
                        items.append(V.Item(0,vertices.mean(axis=0),vertices,draw,np.array([0.,1.,0.]),world,opacity,'glass' if opacity<1 else 'shell'))
                    images=[]
                    for selection in (items,V._opaque_visible(items,view)):
                        drawing=VGroup(*[item.draw(view) for item in V._section_order(selection,view)])
                        camera=Camera(background_color=V.BACKGROUND)
                        camera.capture_mobjects([drawing])
                        images.append(camera.pixel_array.copy())
                    np.testing.assert_array_equal(images[0],images[1])

    def test_actual_opaque_capsule_culling_preserves_cairo_and_all_semantic_parts(self):
        prototype=V._model.capsule_type('A')
        parts=V._cap_parts(V._cap(prototype))
        view=V.make_view((55,24),(0,0,0),1,(0,2,1.25))
        groups={}
        items=V._assembly_items(parts,base=V.CAPSULE,groups=groups,lod=True,glass_opacity=1)
        self.assertEqual(set(groups),{part.key for part,mesh in parts})
        full=V._section_order(items,view)
        indices={id(item):i+1 for i,item in enumerate(items)}
        for resolution in C['actual_visibility']['resolutions']:
            with tempconfig({'pixel_width':resolution,'pixel_height':resolution,'frame_width':6,'frame_height':6,'renderer':'cairo','dry_run':True}):
                filtered=V._opaque_visible(items,view,padding=V._raster_footprint(1,.45))
                self.assertLess(len(filtered),len(items))
                allowed={id(item) for item in filtered}
                orders=(full,[item for item in full if id(V._source_item(item)) in allowed],V._section_order(filtered,view))
                for mode in C['actual_visibility']['modes']:
                    images=[]
                    for ordered in orders:
                        for group in groups.values():
                            group.remove(*group.submobjects)
                        shapes=[]
                        for item in ordered:
                            shape=item.draw(view).set_stroke(width=0)
                            if mode=='silhouette':
                                shape.set_fill('#FFFFFF',1)
                            elif mode=='source-id':
                                shape.set_fill('#'+format(indices[id(V._source_item(item))],'06x'),1)
                            shapes.append(shape)
                        camera=ExactCamera(background_color='#000000' if mode=='source-id' else V.BACKGROUND)
                        camera.capture_mobjects([VGroup(*shapes)])
                        images.append(camera.pixel_array.copy())
                    np.testing.assert_array_equal(images[0],images[1],err_msg=f'{resolution} {mode} stable-removal')
                    if mode=='silhouette':
                        np.testing.assert_array_equal(images[0],images[2],err_msg=f'{resolution} silhouette')
                    if mode=='source-id':
                        pixels={tuple(pixel) for pixel in np.argwhere((images[0]!=images[2]).any(axis=2))}
                        witnesses=[row for row in C['actual_visibility']['reorder_witnesses'] if row['resolution']==resolution]
                        pixels.update((row['pixel'][1],row['pixel'][0]) for row in witnesses)
                        ordered_ids=[indices[id(V._source_item(item))]-1 for item in orders[2]]
                        for row,column in pixels:
                            rgb=images[2][row,column,:3].astype(int)
                            actual=int(rgb@np.array([65536,256,1]))-1
                            offsets=C['actual_visibility']['subpixels']
                            screens=[((column+x)*6/resolution-3,3-(row+y)*6/resolution) for x in offsets for y in offsets]
                            hits=ray_faces(items,view,screens)
                            painted=ray_faces(orders[2],view,screens)
                            for sample,paint in zip(hits,painted):
                                self.assertTrue(sample,(resolution,row,column))
                                nearest=max(sample.values())
                                remaining={i:depth for i,depth in sample.items() if id(items[i]) in allowed}
                                self.assertAlmostEqual(max(remaining.values()),nearest,places=9)
                                self.assertTrue(paint,(resolution,row,column))
                                painter=ordered_ids[max(paint)]
                                self.assertLessEqual(nearest-sample[painter],1e-9,(resolution,row,column,painter))
                            centre=hits[len(hits)//2]
                            self.assertIn(actual,centre)
                            self.assertLessEqual(max(centre.values())-centre[actual],1e-9,(resolution,row,column,actual))
                            for witness in witnesses:
                                if witness['pixel']==[column,row]:
                                    self.assertEqual(actual,witness['nearest'])
                                    self.assertEqual(items[actual].part_key,witness['key'])
        self.assertEqual(set(groups),{part.key for part,mesh in parts})

    def test_named_window_controls_and_between_pane_fan_have_actual_cairo_pixels(self):
        case=C['window_controls']
        state=V._model.WindowUnitState(**case['state'])
        drawing=V.NakaginDrawing().window_unit_axon(V._model.capsule_type('A'),state=state,view=tuple(case['view']),glass_opacity=.25)
        self.assertTrue(set(case['parts']).issubset(drawing.semantic_parts))
        for shape in drawing:
            color=case['parts'].get(shape.semantic_key)
            if color:
                shape.set_fill(color,1).set_stroke(width=0)
        with tempconfig({'pixel_width':512,'pixel_height':512,'frame_width':3,'frame_height':3,'renderer':'cairo','dry_run':True}):
            camera=Camera(background_color=V.BACKGROUND)
            camera.capture_mobjects([drawing])
            rgb=camera.pixel_array[:,:,:3].astype(int)
            for key,color in case['parts'].items():
                channel=int(np.argmax([int(color[i:i+2],16) for i in (1,3,5)]))
                other=[i for i in range(3) if i!=channel]
                pixels=(rgb[:,:,channel]>80)&((rgb[:,:,channel]-rgb[:,:,other].max(axis=2))>50)
                self.assertGreaterEqual(int(pixels.sum()),case['minimum_pixels'],key)

    def test_material_compound_underpaint_preserves_source_holes_and_transparency(self):
        spec=C['material_underpaint']
        case=next(case for case in C['opaque_coverage'] if case['id']==spec['frame_case'])
        view=V.make_view(tuple(spec['view']),(0,0,0),1,(0,0,1))
        for opacity in (1,.25):
            owner=VGroup()
            items=[]
            for index,polygon in enumerate(case['walls']):
                world=np.asarray([(x,0,z) for x,z in polygon])
                def draw(v,polygons=(world,)):
                    return V._shape([V._screen(v,p) for p in polygons],'#FFFFFF',opacity,'#FFFFFF',0)
                items.append(V.Item(0,world.mean(axis=0),world,draw,np.array([0.,1.,0.]),(world,),opacity,'shell',paint_owner=owner,paint_color='#FFFFFF',part_key=str(index)))
            shapes=V._paint_ordered(V._section_order(items,view),view)
            coverage=[shape for shape in shapes if getattr(shape,'semantic_role',None)=='material_coverage']
            self.assertEqual(len(coverage),1 if opacity==1 else 0)
            if coverage:
                self.assertEqual(set(coverage[0].semantic_sources),set(map(str,range(len(items)))))
                self.assertEqual(len(coverage[0].world_facets),len(items))
                self.assertAlmostEqual(coverage[0].world_depth,V._order(view,coverage[0].world_vertices.mean(axis=0)))
            with tempconfig({'pixel_width':192,'pixel_height':128,'frame_width':6,'frame_height':4,'renderer':'cairo','dry_run':True}):
                camera=Camera(background_color=V.BACKGROUND)
                camera.capture_mobjects([VGroup(*shapes)])
                pixels=[]
                for key in ('hole','material','background'):
                    x,z=spec[key]
                    p=view(x,0,z)
                    row=round((2-p[1])*128/4)
                    col=round((p[0]+3)*192/6)
                    pixels.append(camera.pixel_array[row,col,:3].astype(int))
                np.testing.assert_array_equal(pixels[0],pixels[2])
                self.assertGreater(float(pixels[1].mean()),float(pixels[2].mean())+40)

    def test_fully_occluded_hardware_cannot_colour_opaque_material_facet_joins(self):
        case=C['hardware_sentinel']
        with tempconfig({'pixel_width':case['pixel_size'],'pixel_height':case['pixel_size'],'frame_width':6,'frame_height':6,'renderer':'cairo','dry_run':True}):
            view=V.make_view(tuple(case['view']),(0,0,0),1,(0,2,1.25))
            groups={}
            items=V._assembly_items(V._cap_parts(V._cap(V._model.capsule_type('A'))),base=V.CAPSULE,groups=groups,lod=True,glass_opacity=1)
            retained=V._opaque_visible(items,view,padding=V._raster_footprint(1,.45))
            visible={item.part_key for item in retained}
            facing={item.part_key for item in items if item.normal@V._basis(view)[1]>1e-6}
            hidden={key for key in facing-visible if any(key.startswith(prefix) for prefix in case['prefixes'])}
            self.assertGreaterEqual(len(hidden),case['minimum_hidden_parts'])
            original=V._section_order(items,view)
            changed=[]
            for item in items:
                if item.part_key in hidden:
                    def draw(v,polygons=item.polygons or (item.verts,),original=item.draw):
                        return original(v,polygons=polygons).set_fill(case['colour'],1).set_stroke(case['colour'])
                    item=item._replace(draw=draw,paint_color=case['colour'])
                changed.append(item)
            current=V._opaque_visible(changed,view,padding=V._raster_footprint(1,.45))
            source_indices={id(item):i for i,item in enumerate(items)}
            changed_indices={id(item):i for i,item in enumerate(changed)}
            self.assertEqual([source_indices[id(item)] for item in retained],[changed_indices[id(item)] for item in current])
            self.assertTrue(all(item is changed[changed_indices[id(item)]] for item in current))
            unchanged_order=V._section_order(retained,view)
            changed_order=V._section_order(current,view)
            identity=lambda item,index:(index[id(V._source_item(item))],item.polygon,tuple(tuple(map(tuple,p)) for p in item.polygons),tuple(item.normal))
            self.assertEqual([identity(item,source_indices) for item in unchanged_order],[identity(item,changed_indices) for item in changed_order])
            changed=V._section_order(changed,view)
            silhouette=VGroup(*[item.draw(view).set_fill('#FFFFFF',1).set_stroke(width=0) for item in original])
            camera=ExactCamera(background_color='#000000')
            camera.capture_mobjects([silhouette])
            mask=Image.fromarray((camera.pixel_array[:,:,:3].min(axis=2)>250).astype('uint8')*255)
            interior=np.asarray(mask.filter(ImageFilter.MinFilter(case['erosion'])))>0
            self.assertTrue(interior.any())
            controls={}
            for material in (False,True):
                pictures=[]
                for ordered in ((unchanged_order,changed_order) if material else (original,changed)):
                    for group in groups.values():
                        group.remove(*group.submobjects)
                        group.material_underpaint.remove(*group.material_underpaint.submobjects)
                    shapes=V._paint_ordered(ordered,view) if material else [item.draw(view) for item in ordered]
                    camera=Camera(background_color=V.BACKGROUND)
                    camera.capture_mobjects([VGroup(*shapes)])
                    pictures.append(camera.pixel_array.copy())
                controls[material]=int((pictures[0]!=pictures[1])[interior].sum())
            self.assertGreater(controls[False],0,'The old AA path must expose the sentinel to prove this control is meaningful')
            self.assertEqual(controls[True],0,controls)

    def test_lod_never_discards_real_thin_or_concave_reveal_faces(self):
        for case in C['visible_panels']:
            bounds=case['bounds']
            solid=V._model._box(*[value for axis in bounds for value in axis])
            for role in case['roles']:
                with self.subTest(panel=case['id'],role=role):
                    part=V._model.NamedPart('actual-panel',role,solid,'shell')
                    source=V.solid_mesh(solid)
                    expected=[source.V[face] for face,normal in zip(source.faces,source.N) if np.dot(normal,case['normal'])>1-1e-10]
                    self.assertTrue(expected)
                    items=V._assembly_items(((part,source),),base=V.CAPSULE,groups={},lod=True)
                    actual=[polygon for item in items if np.dot(item.normal,case['normal'])>1-1e-10 for polygon in item.polygons]
                    self.assertTrue(actual,'A style LOD cannot infer hidden geometry from part span or body-centre normals')
                    self.assertAlmostEqual(sum(float(np.linalg.norm(np.cross(p[1]-p[0],p[2]-p[0]))/2) for p in actual),sum(float(np.linalg.norm(np.cross(p[1]-p[0],p[2]-p[0]))/2) for p in expected))

    def test_previsibility_lod_preserves_every_actual_source_face(self):
        case=C['previsibility_faces']
        batches=[]
        for detail in case['details']:
            batches.extend((layout,detail,V._parts(V._model.capsule_assembly(V._model.capsule_type('A'),detail,layout))) for layout in case['families'])
            batches.extend((tower.key,detail,V._parts(V._model.core_assembly(tower,detail=detail))) for tower in V._model.towers() if tower.index in case['towers'])
        for name,detail,parts in batches:
            with self.subTest(source=name,detail=detail):
                expected=Counter((part.key,np.asarray(mesh.V[face],float).tobytes()) for part,mesh in parts for face in mesh.faces)
                items=V._assembly_items(parts,base=V.CAPSULE,groups={},lod=True)
                actual=Counter((item.part_key,np.asarray(polygon,float).tobytes()) for item in items for polygon in item.polygons)
                self.assertEqual(actual,expected)
                self.assertEqual(sum(actual.values()),sum(len(mesh.faces) for part,mesh in parts))

    def test_identical_geometry_reuses_only_order_not_old_palette_or_semantic_objects(self):
        drawings=[]
        for identity,palette in enumerate(C['palettes']):
            drawing=V.NakaginDrawing()._compose(self.items(palette,identity),tuple(C['view']),(0,0,0),1)
            self.assertEqual([shape.tag for shape in drawing],C['expected_order'])
            self.assertTrue(all(shape.semantic_owner==identity for shape in drawing))
            for shape in drawing:
                self.assertEqual(str(shape.get_fill_color()).upper(),palette[shape.tag])
            drawings.append(drawing)
        self.assertFalse({id(shape) for shape in drawings[0]}&{id(shape) for shape in drawings[1]})

    def test_lod_ordering_uses_real_facet_planes_not_artist_axis_normals(self):
        spec=C['sloped_shell']
        solid=V._model._face_slab(tuple(map(tuple,spec['vertices'])),spec['thickness'])
        part=V._model.NamedPart('sloped-skin','shell',solid,'shell')
        items=V._assembly_items(V._parts(V._model.Assembly((part,))),base=V.CAPSULE,groups={},lod=True)
        self.assertTrue(any(np.dot(item.normal,spec['normal'])>1-1e-10 for item in items))
        for item in items:
            self.assertLessEqual(float(np.abs((item.verts-item.verts[0])@item.normal).max()),1e-8)

    def test_projected_overlap_matches_independent_pillow_masks(self):
        for pair in C['overlap_pairs']:
            with self.subTest(case=pair['id']):
                actual=V._projected_overlap([np.asarray(pair['a'],float)],[np.asarray(pair['b'],float)])
                masks=[]
                for polygon in (pair['a'],pair['b']):
                    mask=Image.new('L',(384,384),0)
                    ImageDraw.Draw(mask).polygon([(64*(x+1),64*(y+1)) for x,y in polygon],fill=255)
                    masks.append(np.asarray(mask.filter(ImageFilter.MinFilter(3)))>0)
                oracle=bool((masks[0]&masks[1]).any())
                self.assertEqual(oracle,pair['expected'])
                self.assertEqual(actual,oracle)

    def items(self,palette=None,identity=None):
        items=[]
        for name in ('wall','beam'):
            case=C[name]
            vertices=np.asarray(case['vertices'],float)
            def draw(view,polygons=(vertices,),name=name,color=(palette or {}).get(name,case['color']),identity=identity):
                shape=V._shape([V._screen(view,p) for p in polygons],color)
                shape.tag=name
                shape.semantic_owner=identity
                return shape
            items.append(V.Item(0,vertices.mean(axis=0),vertices,draw,np.asarray(case['normal'],float)))
        return items

    def test_wall_hides_long_inside_beam_despite_its_nearer_centroid(self):
        view=V.make_view(tuple(C['view']),(0,0,0),1,(0,0,0))
        items=self.items()
        self.assertGreater(V._order(view,items[1].centre),V._order(view,items[0].centre))
        drawing=V.NakaginDrawing()._compose(items,tuple(C['view']),(0,0,0),1)
        self.assertEqual([shape.tag for shape in drawing],C['expected_order'])
        point=np.asarray([.1,-1.5,1.05])
        direction=np.asarray(view.towards)
        distance=-point[0]/direction[0]
        crossing=point+distance*direction
        self.assertGreater(distance,0)
        self.assertTrue(-2<crossing[1]<-1 and 0<crossing[2]<2)
        camera=Camera(background_color=V.BACKGROUND)
        camera.capture_mobjects([drawing])
        screen=drawing.view(*point)
        row=round((camera.frame_height/2-screen[1])*camera.pixel_height/camera.frame_height)
        col=round((screen[0]+camera.frame_width/2)*camera.pixel_width/camera.frame_width)
        pixel=camera.pixel_array[row,col,:3]
        self.assertGreater(int(pixel[1]),200)
        self.assertLess(int(pixel[0]),30)

    def test_closed_capital_has_no_pixels_of_contained_frame(self):
        spec=C['capital']
        with tempconfig({'pixel_width':spec['pixel_size'],'pixel_height':spec['pixel_size'],'frame_width':8,'frame_height':8,'renderer':'cairo','dry_run':True}):
            for index in spec['towers']:
                tower=V._model.towers()[index]
                envelope=V.solid_mesh(V._model.capital_solid(tower))
                groups={}
                parts=V._capital_parts(index,tower.height,tower.capital_height,tower.capital_overhang,tower.core_side)
                frames=[mesh for part,mesh in parts if part.role==spec['hidden_role']]
                self.assertTrue(frames)
                for mesh in frames:
                    points=mesh.V
                    self.assertTrue(np.all(points[:,:2]>=envelope.V[:,:2].min(axis=0)-1e-8))
                    self.assertTrue(np.all(points[:,:2]<=envelope.V[:,:2].max(axis=0)+1e-8))
                    self.assertGreaterEqual(float(points[:,2].min()),float(envelope.V[:,2].min())-1e-8)
                    covered=np.zeros(len(points),bool)
                    for face,normal in zip(envelope.faces,envelope.N):
                        if normal[2]<=0:
                            continue
                        triangle=envelope.V[face]
                        weights=np.linalg.solve(np.vstack((triangle[:,:2].T,np.ones(3))),np.vstack((points[:,:2].T,np.ones(len(points))))).T
                        inside=(weights>=-1e-8).all(axis=1)
                        covered|=inside&(points[:,2]<=weights@triangle[:,2]+1e-8)
                    self.assertTrue(covered.all())
                for direction in spec['views']:
                    with self.subTest(tower=index,view=direction):
                        raw=V._assembly_items(parts,base=V.CAPITAL,groups=groups)
                        items=[]
                        for item in raw:
                            def draw(view,polygons=item.polygons or (item.verts,),original=item.draw):
                                shape=original(view,polygons=polygons)
                                if getattr(shape,'semantic_role',None)==spec['hidden_role']:
                                    shape.set_fill('#FF0000',1).set_stroke(width=0)
                                if getattr(shape,'semantic_role',None)==spec['visible_role']:
                                    shape.set_fill('#0000FF',1).set_stroke(width=0)
                                return shape
                            role=inspect.signature(item.draw).parameters["role"].default
                            paint="#FF0000" if role==spec["hidden_role"] else ("#0000FF" if role==spec["visible_role"] else item.paint_color)
                            items.append(item._replace(draw=draw,paint_color=paint))
                        drawing=V.NakaginDrawing()._compose(items,tuple(direction),(0,0,0),.5)
                        camera=Camera(background_color=V.BACKGROUND)
                        camera.capture_mobjects([drawing])
                        rgb=camera.pixel_array[:,:,:3]
                        red=(rgb[:,:,0]>150)&(rgb[:,:,1]<40)&(rgb[:,:,2]<40)
                        self.assertLessEqual(int(red.sum()),spec['maximum_red_pixels'])
                        blue=(rgb[:,:,2]>60)&(rgb[:,:,0]<30)&(rgb[:,:,1]<30)
                        self.assertGreaterEqual(int(blue.sum()),spec['minimum_seam_pixels'])

if __name__=='__main__':
    unittest.main(verbosity=2)

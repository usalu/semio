"""✂️ Preserve source surfaces, actual depth and styling through view-specific face fragments."""
import importlib
import json
import gc
import weakref
from pathlib import Path
import sys
import unittest
from unittest.mock import patch
import numpy as np
from manim import Camera,VGroup,tempconfig
from PIL import Image,ImageDraw,ImageFilter
root=next(p for p in Path(__file__).resolve().parents if (p/'🧰️framework').is_dir())
sys.path.insert(0,str(root))
V=importlib.import_module('🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️')
C=json.loads((root/'🎓️teaching/🏛️architecture/🏢️nakagin/🧪️cases/👁️occlusion/✂️fragments/🔣️.json').read_text(encoding='utf-8'))

class FragmentRendering(unittest.TestCase):
    def test_actual_warped_quad_has_canonical_planar_facets(self):
        import rhino3dm
        case=C['warped_quad']
        points=np.asarray(case['vertices'],float)
        actual=V._polygon_facets(points)
        self.assertEqual(len(actual),2)
        mesh=rhino3dm.Mesh()
        for point in points:
            mesh.Vertices.Add(*point)
        mesh.Faces.AddFace(0,1,2,3)
        mesh.Faces.ConvertQuadsToTriangles()
        expected=tuple(points[row] for row in case['triangles'])
        for face,witness in zip(actual,expected):
            np.testing.assert_array_equal(face,witness)
        self.assertEqual(tuple(tuple(mesh.Faces[i][:3]) for i in range(2)),tuple(tuple(row) for row in case['triangles']))
        normal=np.cross(points[1]-points[0],points[3]-points[0])
        for face in actual:
            self.assertGreater(float(np.cross(face[1]-face[0],face[2]-face[0])@normal),0)
        planar=np.asarray(case['planar'],float)
        unchanged=V._polygon_facets(planar)
        self.assertEqual(len(unchanged),1)
        np.testing.assert_array_equal(unchanged[0],planar)
        np.testing.assert_array_equal(points,np.asarray(case['vertices']))

    def test_warped_boundary_enters_fragments_before_any_uniform_plane_order(self):
        case=C['warped_overlap']
        items=self.items(case)
        view=V.make_view((0,90),(0,0,0),1,(0,0,0))
        basis,towards=V._basis(view)
        projected=[tuple(V._prepare_polygon(p@basis.T) for p in item.polygons) for item in items]
        self.assertEqual(V._overlap_plane_order(*items,*projected,basis,towards),2)
        ordered=V._section_order(items,view)
        self.assertGreater(len(ordered),len(items))
        self.assertEqual({id(V._source_item(item)) for item in ordered},{id(item) for item in items})
        for probe in case['probes']:
            screen=V._screen(view,np.asarray(probe['point']))
            point=np.asarray(screen).reshape(-1)[:2]
            hits=[]
            for item in ordered:
                for polygon in item.polygons:
                    vertices=V._screen(view,polygon)[:,:2]
                    delta=np.roll(vertices,-1,axis=0)-vertices
                    offset=point-vertices
                    cross=delta[:,0]*offset[:,1]-delta[:,1]*offset[:,0]
                    if (cross>=-1e-9).all() or (cross<=1e-9).all():
                        hits.append(item)
            self.assertTrue(hits)
            self.assertIs(V._source_item(hits[-1]),items[probe['source']])

    def test_isolated_warped_boundary_is_always_rendered_as_canonical_facets(self):
        source=self.items(C['warped_overlap'])[0]
        view=V.make_view((0,90),(0,0,0),1,(0,0,0))
        ordered=V._section_order([source],view)
        self.assertEqual(len(ordered),2)
        for item in ordered:
            self.assertIs(V._source_item(item),source)
            self.assertEqual(item.polygon,0)
            self.assertEqual(len(item.polygons),1)
            self.assertEqual(len(item.polygons[0]),3)
            np.testing.assert_allclose((item.verts-item.verts[0])@item.normal,0,atol=1e-14)

    def test_projected_self_crossing_singleton_uses_actual_facet_visibility(self):
        case=C['folded_singleton']
        source=self.items({'items':[case]})[0]
        view=V.make_view((0,90),(0,0,0),1,(0,0,0))
        self.assertAlmostEqual(float(source.normal@V._basis(view)[1]),0,places=12)
        self.assertEqual(V._opaque_visible([source],view),[source])
        ordered=V._section_order([source],view)
        self.assertEqual(len(ordered),1)
        self.assertIs(V._source_item(ordered[0]),source)
        self.assertEqual(ordered[0].polygon,0)
        np.testing.assert_array_equal(ordered[0].verts,source.verts[case['visible_indices']])
        normal=np.cross(ordered[0].verts[1]-ordered[0].verts[0],ordered[0].verts[2]-ordered[0].verts[0])
        np.testing.assert_allclose(ordered[0].normal,normal/np.linalg.norm(normal))

    def test_backward_shader_normal_on_warped_material_cannot_hide_nearer_geometry(self):
        case=C['occluder_guard']
        items=self.items(case)
        view=V.make_view((0,90),(0,0,0),1,(0,0,0))
        self.assertLess(float(items[0].normal@V._basis(view)[1]),0)
        self.assertIsNone(V._item_plane(items[0]))
        retained=V._opaque_visible(items,view)
        self.assertEqual([id(item) for item in retained],[id(item) for item in items])
        ordered=V._section_order(retained,view)
        self.assertIs(V._source_item(ordered[-1]),items[1])

    def progress_items(self):
        base=self.items(C['warped_overlap'])[1]
        result=[]
        for index in range(C['progress']['count']):
            offset=np.array([index*4,0,0])
            result.append(base._replace(centre=base.centre+offset,verts=base.verts+offset,polygons=tuple(p+offset for p in base.polygons)))
        return result

    def test_preparation_and_overlap_progress_counts_are_truthful_and_bounded(self):
        items=self.progress_items()
        view=V.make_view((0,90),(0,0,0),1,(0,0,0))
        V._OCCLUSION_ORDERS.clear()
        events=[]
        ordered=V._section_order(items,view,progress=events.append)
        self.assertEqual([id(item) for item in ordered],[id(item) for item in items])
        for phase in C['progress']['phases']:
            rows=[row for row in events if row['phase']==phase]
            self.assertTrue(rows,phase)
            self.assertEqual(rows[0]['completed'],0)
            self.assertEqual(rows[-1]['completed'],len(items))
            self.assertTrue(all(row['total']==len(items) for row in rows))
            self.assertTrue(all(0<=b['completed']-a['completed']<=C['progress']['interval'] for a,b in zip(rows,rows[1:])))
        unchanged=V._section_order(items,view)
        self.assertEqual([id(item) for item in unchanged],[id(item) for item in ordered])

    def test_cancellation_during_preparation_precedes_component_ordering(self):
        items=self.progress_items()
        view=V.make_view((0,90),(0,0,0),1,(0,0,0))
        V._OCCLUSION_ORDERS.clear()
        state={'cancel':False}
        events=[]
        def progress(event):
            events.append(event)
            if event['phase']==C['progress']['cancel_phase']:
                state['cancel']=True
        with patch.object(V,'_dependency_order',side_effect=AssertionError('Component ordering must not start')):
            with self.assertRaises(V._fragments.FragmentCancelled):
                V._section_order(items,view,progress=progress,cancelled=lambda:state['cancel'])
        self.assertTrue(any(row['phase']==C['progress']['cancel_phase'] for row in events))
        self.assertFalse(any(row['phase']=='occlusion-components' for row in events))

    def test_fragment_draw_projects_only_emitted_geometry_with_current_source_style(self):
        case=C['draw_override']
        view=V.make_view(tuple(case['view']),(0,0,0),1,(0,2,1.25))
        groups={}
        parts=V._cap_parts(V._cap(V._model.capsule_type('A')))
        items=V._assembly_items(parts,base=V.CAPSULE,groups=groups,outline='#000000',glass_opacity=1)
        source=next(item for item in items if item.part_key==case['part'] and len(item.polygons)>=case['minimum_polygons'] and item.normal@V._basis(view)[1]>1e-6)
        original=source.draw(view)
        polygon=source.polygons[case['polygon']]
        fragment=np.asarray([polygon[0],polygon[0]+case['fraction']*(polygon[1]-polygon[0]),polygon[0]+case['fraction']*(polygon[2]-polygon[0])])
        recipe=V._FragmentRecipe(0,case['polygon'],(tuple(map(tuple,fragment)),),tuple(V._polygon_normal(fragment,source.normal)))
        shapes=[]
        projections=[]
        original_shape,original_screen=V._shape,V._screen
        def shape(polygons,*args,**kwargs):
            shapes.append((len(polygons),sum(len(p) for p in polygons)))
            return original_shape(polygons,*args,**kwargs)
        def screen(v,points):
            projections.append(len(np.asarray(points).reshape(-1,3)))
            return original_screen(v,points)
        with patch.object(V,'_shape',side_effect=shape),patch.object(V,'_screen',side_effect=screen):
            item=V._fragment_item([source],recipe)
            drawn=item.draw(view)
        self.assertEqual(shapes,[(1,case['expected_fragment_vertices'])])
        self.assertEqual(sum(projections),case['expected_fragment_vertices']+4)
        np.testing.assert_array_equal(drawn.world_vertices,fragment)
        expected=V._engine.Polygon(*V._screen(view,fragment))
        np.testing.assert_allclose(drawn.points,expected.points,atol=1e-12,rtol=0)
        self.assertEqual(drawn.get_fill_color(),original.get_fill_color())
        self.assertEqual(drawn.get_fill_opacity(),original.get_fill_opacity())
        self.assertEqual(drawn.semantic_key,source.part_key)
        self.assertEqual(drawn.source_polygon,case['polygon'])
        self.assertIn(drawn,groups[source.part_key].submobjects)
        self.assertEqual(drawn.get_stroke_width(),0)
        self.assertEqual(len(drawn.submobjects),1)
        self.assertEqual(len(drawn.submobjects[0].world_vertices),4)

    def test_tiny_fragment_metadata_uses_validated_original_plane_against_numpy(self):
        case=C['plane_lineage']
        carrier=np.asarray(case['carrier'],float)
        fragment=np.asarray(case['fragment'],float)
        normal=np.asarray(case['normal'],float)
        source=V.Item(0,carrier.mean(axis=0),carrier,lambda view,polygons=None:None,normal,(carrier,),1,'metal')
        recipe=V._FragmentRecipe(0,0,(tuple(map(tuple,fragment)),),tuple(normal))
        item=V._fragment_item([source],recipe)
        actual=V._paint_normal(item,fragment)
        np.testing.assert_array_equal(actual,normal)
        _,_,vectors=np.linalg.svd(carrier-carrier.mean(axis=0))
        oracle=vectors[-1]
        if oracle@normal<0:
            oracle=-oracle
        np.testing.assert_allclose(actual,oracle,atol=1e-12,rtol=0)
        self.assertLessEqual(float(np.max(np.abs((fragment-carrier[0])@np.asarray(actual)))),case['epsilon'])
        original=np.asarray(C['compound']['polygon'],float)
        unsplit=source._replace(origin=None,normal=np.array([1.,0,0]))
        np.testing.assert_allclose(V._paint_normal(unsplit,original),C['compound']['normal'],atol=1e-14,rtol=0)

    def items(self,case,owner=None,reverse=False,opacity=None):
        result=[]
        for row in case['items']:
            polygons=tuple(np.asarray(p,float) for p in row['polygons'])
            vertices=np.vstack(polygons)
            color=('#00FF00' if row['color']=='#FF0000' else '#FF00FF') if reverse else row['color']
            alpha=row.get("opacity",1) if opacity is None else opacity
            def draw(view,polygons=polygons,color=color,owner=owner,alpha=alpha):
                shape=V._shape([V._screen(view,p) for p in polygons],color,alpha,'#000000',2)
                shape.semantic_owner=owner
                return shape
            result.append(V.Item(0,vertices.mean(axis=0),vertices,draw,np.asarray(row['normal'],float),polygons,alpha,'shell',part_key=row['key']))
        return result

    def area(self,polygon,normal):
        points=np.asarray(polygon)-np.asarray(polygon)[0]
        return abs(float(np.cross(points,np.roll(points,-1,axis=0)).sum(axis=0)@normal))/2

    def test_actual_fragment_area_depth_and_original_boundaries(self):
        for case in C['cases']:
            with self.subTest(case=case['id']):
                items=self.items(case)
                view=V.make_view(tuple(C['view']),(0,0,0),1,(0,1,0))
                ordered=V._section_order(items,view)
                self.assertGreater(len(ordered),len(items))
                for source in items:
                    fragments=[item for item in ordered if V._source_item(item) is source]
                    self.assertTrue(fragments)
                    expected=sum(self.area(p,source.normal) for p in source.polygons)
                    actual=sum(self.area(p,item.normal) for item in fragments for p in item.polygons)
                    self.assertAlmostEqual(actual,expected,places=9)
                    for item in fragments:
                        self.assertIn(item.polygon,range(len(source.polygons)))
                        np.testing.assert_allclose(item.verts@item.normal,source.verts[0]@item.normal,atol=1e-10)
                with tempconfig({'pixel_width':C['pixels'],'pixel_height':C['pixels'],'frame_width':C['frame'],'frame_height':C['frame'],'renderer':'cairo','dry_run':True}):
                    camera=Camera(background_color='#FFFFFF')
                    drawing=VGroup(*V._paint_ordered(ordered,view))
                    camera.capture_mobjects([drawing])
                    for probe in case['probes']:
                        point=view(*probe['point'])
                        column,row=int((point[0]+4)*32),int((4-point[1])*32)
                        expected=np.array([int(case['items'][probe['source']]['color'][i:i+2],16) for i in (1,3,5)])
                        np.testing.assert_array_equal(camera.pixel_array[row,column,:3],expected)
                    if 'void_probe' in case:
                        point=view(*case['void_probe'])
                        column,row=int((point[0]+4)*32),int((4-point[1])*32)
                        np.testing.assert_array_equal(camera.pixel_array[row,column,:3],[255,255,255])
                    if 'no_stroke_probe' in case:
                        point=view(*case['no_stroke_probe'])
                        column,row=int((point[0]+4)*32),int((4-point[1])*32)
                        pixels=camera.pixel_array[row-1:row+2,column-5:column+6,:3]
                        self.assertFalse((pixels.max(axis=2)<40).any(),'Artificial split edges must not become strokes')
                for item in ordered:
                    shape=item.draw(view)
                    np.testing.assert_allclose(shape.world_vertices,item.verts)
                    self.assertAlmostEqual(shape.world_depth,V._order(view,item.verts.mean(axis=0)))

    def test_fragment_recipe_cache_keeps_current_styles_and_source_objects(self):
        case=C['cases'][0]
        view=V.make_view(tuple(C['view']),(0,0,0),1,(0,1,0))
        first=self.items(case,object())
        V._section_order(first,view)
        owner=object()
        current=self.items(case,owner,True,opacity=.25)
        result=V._section_order(current,view)
        for item in result:
            self.assertTrue(any(V._source_item(item) is source for source in current))
            shape=item.draw(view)
            self.assertIs(shape.semantic_owner,owner)
            self.assertIn(shape.get_fill_color().to_hex(),('#00FF00','#FF00FF'))
            self.assertAlmostEqual(shape.get_fill_opacity(),.25)

    def test_cyclic_transparency_composites_actual_material_against_pillow(self):
        case=C['transparent']
        view=V.make_view(tuple(C['view']),(0,0,0),1,(0,1,0))
        items=self.items(case)
        ordered=V._section_order(items,view)
        self.assertGreater(len(ordered),len(items))
        with tempconfig({'pixel_width':256,'pixel_height':256,'frame_width':8,'frame_height':8,'renderer':'cairo','dry_run':True}):
            camera=Camera(background_color='#FFFFFF')
            camera.capture_mobjects([VGroup(*V._paint_ordered(ordered,view))])
            for probe in case['probes']:
                expected=Image.new('RGBA',(1,1),(255,255,255,255))
                for source in probe['layers']:
                    row=case['items'][source]
                    rgb=tuple(int(row['color'][i:i+2],16) for i in (1,3,5))
                    expected=Image.alpha_composite(expected,Image.new('RGBA',(1,1),(*rgb,round(row['opacity']*255))))
                point=view(*probe['point'])
                column,row=int((point[0]+4)*32),int((4-point[1])*32)
                np.testing.assert_array_equal(camera.pixel_array[row,column,:3],np.asarray(expected)[0,0,:3])
        for item in ordered:
            self.assertAlmostEqual(item.draw(view).get_fill_opacity(),.25)

    def test_consecutive_compound_transparency_has_no_internal_double_fill(self):
        case=C['compound']
        polygon=np.asarray(case['polygon'],float)
        view=V.make_view((90,20),(0,0,0),1,(0,1,0))
        def draw(v,polygons=(polygon,)):
            return V._shape([V._screen(v,p) for p in polygons],case['color'],case['opacity'],width=0)
        source=V.Item(0,polygon.mean(axis=0),polygon,draw,np.asarray(case['normal'],float),(polygon,),case['opacity'],'glass',part_key='pane')
        faces=[V._fragments.Face(0,0,tuple(map(tuple,row)),tuple(case['normal'])) for row in case['triangles']]
        recipes=V._coalesce_fragments(faces)
        self.assertEqual(len(recipes),1)
        self.assertEqual(len(recipes[0].polygons),2)
        separated=V._coalesce_fragments([faces[0],1,faces[1]])
        self.assertEqual(len(separated),3)
        fragment=V._restore_order([source],recipes)[0]
        with tempconfig({'pixel_width':256,'pixel_height':256,'frame_width':8,'frame_height':8,'renderer':'cairo','dry_run':True}):
            images=[]
            for item in (source,fragment):
                camera=Camera(background_color='#FFFFFF')
                camera.capture_mobjects([item.draw(view)])
                images.append(camera.pixel_array.copy())
            np.testing.assert_array_equal(*images)

    def test_rendered_shapes_and_geometry_cache_do_not_retain_source_groups(self):
        case=C['cases'][0]
        view=V.make_view(tuple(C['view']),(0,0,0),1,(0,1,0))
        def build():
            groups=[VGroup() for row in case['items']]
            items=[]
            for row,group in zip(case['items'],groups):
                polygons=tuple(np.asarray(p,float) for p in row['polygons'])
                vertices=np.vstack(polygons)
                def draw(v,polygons=polygons,group=group):
                    shape=V._shape([V._screen(v,p) for p in polygons],'#FF0000')
                    group.add(shape)
                    return shape
                items.append(V.Item(0,vertices.mean(axis=0),vertices,draw,np.asarray(row['normal'],float),polygons,1,'shell'))
            ordered=V._section_order(items,view)
            shapes=[item.draw(view) for item in ordered]
            return shapes,[weakref.ref(group) for group in groups]
        shapes,owners=build()
        gc.collect()
        self.assertTrue(shapes)
        self.assertTrue(all(owner() is None for owner in owners))

if __name__=='__main__':
    unittest.main(verbosity=2)

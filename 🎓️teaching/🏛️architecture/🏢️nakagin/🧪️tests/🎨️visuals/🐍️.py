"""🔍 Portable rendering contracts checked against NumPy vectors and the Manim camera."""
import importlib
import json
from pathlib import Path
import sys
import unittest
from unittest.mock import patch
import numpy as np
root=next(p for p in Path(__file__).resolve().parents if (p/'🧰️framework').is_dir())
sys.path.insert(0,str(root))
V=importlib.import_module('🎓️teaching.🏛️architecture.🏢️nakagin.🎨️visuals.🐍️')
M=V._model
C=json.loads((root/'🎓️teaching/🏛️architecture/🏢️nakagin/🧪️cases/🎨️visuals/🔣️.json').read_text(encoding='utf-8'))
class VisualContracts(unittest.TestCase):
    def test_default_reference_drawings_keep_all_physical_attachment_bolts(self):
        case=C['reference_hardware']
        t=M.capsule_type(case['type'])
        for layout in case['layouts']:
            with self.subTest(layout=layout):
                expected={part.key:part for part in M.capsule_assembly(t,'full',layout).parts if part.role in case['roles']}
                self.assertEqual({role:sum(part.role==role for part in expected.values()) for role in case['roles']},case['roles'])
                items=[]
                with patch.object(V,'_section_order',side_effect=lambda source,view:(items.extend(source) or [])):
                    V.NakaginDrawing().capsule_axon(t,layout=layout)
                actual={key:np.vstack([item.verts for item in items if item.part_key==key]) for key in expected if any(item.part_key==key for item in items)}
                self.assertEqual(set(actual),set(expected))
                for key,points in actual.items():
                    vertices=np.asarray(expected[key].solid.vertices)
                    np.testing.assert_allclose((points.min(axis=0),points.max(axis=0)),(vertices.min(axis=0),vertices.max(axis=0)),atol=case['tolerance'],rtol=0)
                items=[]
                with patch.object(V,'_section_order',side_effect=lambda source,view:(items.extend(source) or [])):
                    V.NakaginDrawing().capsule_axon(t,layout=layout,bolts=False)
                self.assertFalse(set(expected)&{item.part_key for item in items})

    def test_annotated_views_keep_the_requested_clearance_between_all_ink(self):
        case=C['view_annotation_layout']
        class Sheet(V.NakaginDrawing,V._d.Drafting):
            def number(self,value,digits=2):
                return f'{value:.{digits}f}'.replace('.',',' if self.language=='de' else '.')
            def capsule_axon(self,t,*,at,scale,view,**options):
                casing=M.capsule_casing(t)
                width=casing.width if view=='north' else casing.length
                centre=((casing.left+casing.right)/2,(casing.back+casing.front)/2,(casing.bottom+casing.top)/2)
                drawing=V.NakaginDrawing3D(V.make_view(view,at,scale,centre))
                drawing.add(V._engine.Rectangle(width=width*scale,height=casing.height*scale).move_to(at))
                return drawing
        for language in case['languages']:
            for layout in case['layouts']:
                for scale in case['scales']:
                    with self.subTest(language=language,layout=layout,scale=scale):
                        sheet=Sheet()
                        sheet.language=language
                        with patch.object(V,'_cap_parts',return_value=[]):
                            drawing=sheet.capsule_views(M.capsule_type('A'),layout=layout,scale=scale,gap=case['gap'])
                        bounds={name:np.asarray((group.get_all_points().min(axis=0),group.get_all_points().max(axis=0))) for name,group in drawing.parts.items()}
                        self.assertGreaterEqual(bounds['side'][0,0]-bounds['front'][1,0],case['gap']-case['tolerance'])
                        self.assertGreaterEqual(bounds['plan'][0,1]-max(bounds['front'][1,1],bounds['side'][1,1]),case['gap']-case['tolerance'])
                        for name in ('front','side'):
                            group=drawing.parts[name]
                            np.testing.assert_allclose(group.at(*group.view.centre),group[0].get_center(),atol=case['tolerance'],rtol=0)

    def test_plan_boundary_matches_actual_rounded_shell_projection(self):
        from scipy.spatial import ConvexHull
        case=C['plan_boundary']
        for quarters in case['quarter_segments']:
            with self.subTest(quarters=quarters):
                parameters={'dimensional_basis':'nominal','key':case['type'],'corner_segments':quarters}
                sheet=V.NakaginDrawing().capsule_views(parameters,layout=case['layout'],scale=case['scale'],dims=False,glass_opacity=1)
                casing=M.capsule_casing(parameters)
                source=np.asarray([point[:2] for part in M.capsule_assembly(parameters,'full',case['layout']).parts if part.role in ('shell','shell_back') for point in part.solid.vertices])
                hull=source[ConvexHull(source).vertices]
                outer=sheet.parts['plan'][0]
                centre=outer.get_center()[:2]
                expected=centre+np.column_stack((-hull[:,0],(casing.back+casing.front)/2-hull[:,1]))*case['scale']
                actual=outer.get_all_points()[:,:2]
                actual=actual[ConvexHull(actual).vertices]
                expected=np.unique(np.round(expected,11),axis=0)
                actual=np.unique(np.round(actual,11),axis=0)
                np.testing.assert_allclose(actual,expected,atol=case['tolerance'],rtol=0)

    def test_building_retains_every_source_exterior_part_and_service_geometry(self):
        slot=M.slots()[0]
        key=(*slot.path,slot.item)
        parts=V._slot_parts(slot)
        capsules={}
        items=V.NakaginDrawing()._tower_items(lit=None,tint=None,types=False,capital=False,podium=False,where=lambda s:s==slot,core=False,ground=False,segments=2,capsules=capsules,cores={},capitals={},base=V.CAPSULE,towers=(slot.tower,),storeys=None)
        self.assertEqual(set(capsules[key].semantic_parts),{part.key for part,mesh in parts})
        actual=np.vstack([item.verts for item in items])
        for part,mesh in parts:
            if part.role in C['exterior_details']['roles']:
                world=V._world(mesh,slot.plane).V
                for point in world:
                    self.assertLess(float(np.linalg.norm(actual-point,axis=1).min()),1e-8,part.key)
        self.assertTrue(set(C['exterior_details']['roles']).issubset({part.role for part,mesh in parts}))

    def test_all_physical_families_keep_actual_apertures_in_inspection_and_orthographics(self):
        class Sheet(V.NakaginDrawing,V._d.Drafting):
            language='en'
            def number(self,value,digits=2):
                return f'{value:.{digits}f}'
        with self.assertRaises(ValueError):
            V._cap({'dimensional_basis':'nominal','layout':'P'})
        for layout in C['inspection_families']:
            with self.subTest(layout=layout):
                drawing=V.NakaginDrawing().capsule_axon(M.capsule_type('A'),layout=layout,view=(55,24))
                expected=M.capsule_window(M.capsule_type('A'),layout)
                np.testing.assert_allclose(drawing.aperture.center,expected.center)
                np.testing.assert_allclose(drawing.aperture.normal,expected.normal)
                self.assertEqual(drawing.layout,layout)
                sheet=Sheet().capsule_views(M.capsule_type("A"),layout=layout,dims=False,glass_opacity=1)
                self.assertEqual(sheet.layout,layout)
                np.testing.assert_allclose(sheet.aperture.center,expected.center,atol=1e-10,rtol=0)
                np.testing.assert_allclose(sheet.aperture.normal,expected.normal,atol=1e-10,rtol=0)
                casing,usable=M.capsule_casing(M.capsule_type('A')),M.capsule_interior(M.capsule_type('A'))
                radius=expected.radius
                bounds=np.asarray(((usable.right,expected.center[1]-radius),(casing.right,expected.center[1]+radius))) if expected.normal[0] else np.asarray(((expected.center[0]-radius,usable.front),(expected.center[0]+radius,casing.front)))
                scale=.58
                outer=sheet.parts['plan'][0].get_center()
                centre=outer+np.asarray((-bounds[:,0].mean()*scale,((casing.back+casing.front)/2-bounds[:,1].mean())*scale,0))
                points=sheet.parts['plan'].window.get_all_points()
                np.testing.assert_allclose((points.min(axis=0)+points.max(axis=0))/2,centre,atol=1e-10,rtol=0)
                np.testing.assert_allclose(np.ptp(points,axis=0),(*((bounds[1]-bounds[0])*scale),0),atol=1e-10,rtol=0)
                bolts={part.key for part in M.capsule_assembly('A','full',layout).parts if part.role in ('bolt_shaft','bolt_head')}
                for name in ('front','side'):
                    self.assertTrue(bolts<=set(sheet.parts[name].semantic_parts))
                for name in ("front","side"):
                    np.testing.assert_allclose(sheet.parts[name].aperture.center,expected.center)
                expected_keys={part.key for part in M.capsule_assembly('A','full',layout).parts}
                self.assertEqual(set(drawing.semantic_parts),expected_keys)
                self.assertEqual({part.key for part,mesh in V._cap_parts(V._cap(M.capsule_type('A')),layout=layout)},{part.key for part in M.capsule_assembly('A','exterior',layout).parts})

    def test_physical_reference_corner_slider_keeps_finite_actual_geometry(self):
        for radius in C['physical_round_controls']:
            with self.subTest(radius=radius):
                parameters={**C['dimensional_basis']['physical'],'corner_radius':radius}
                parts=V._cap_parts(V._cap(parameters))
                points=np.vstack([mesh.V for part,mesh in parts if part.role in ('shell','shell_back')])
                self.assertTrue(np.isfinite(points).all())
                np.testing.assert_allclose(np.ptp(points,axis=0),C['dimension_reference']['physical'],atol=1e-8)
                for part,mesh in parts:
                    self.assertTrue(np.isfinite(mesh.N).all(),part.key)

    def test_standalone_window_unit_preserves_actual_inventory_and_authored_states(self):
        for layout in C['window_inspection']['layouts']:
            for values in C['window_inspection']['states']:
                with self.subTest(layout=layout,state=values):
                    state=M.WindowUnitState(**values)
                    drawing=V.NakaginDrawing().window_unit_axon(M.capsule_type('A'),layout=layout,state=state)
                    expected=M.capsule_window_unit(M.capsule_type('A'),layout,state)
                    self.assertEqual(set(drawing.semantic_parts),{part.key for part in expected.parts})
                    for source,actual in zip(expected.parts,drawing.source_assembly.parts):
                        np.testing.assert_allclose(actual.solid.vertices,source.solid.vertices)
                    self.assertEqual(drawing.unit_state,values)
                    self.assertTrue(all((shape.semantic_key in drawing.semantic_parts) if shape.semantic_key is not None else set(shape.semantic_sources).issubset(drawing.semantic_parts) for shape in drawing))

    def test_default_and_explicit_tessellation_match_shared_detail_policy(self):
        case=C['mesh_policy']
        physical=C['dimensional_basis']['physical']
        controls=[(M.capsule_type('A'),M.capsule_type('A')),(physical,M.capsule_type('A')),(M.capsule_parameters_from_casing(physical),M.capsule_type('A'))]
        controls.extend(({'dimensional_basis':'nominal','key':'A','corner_segments':q},{'key':'A','corner_segments':q}) for q in case['explicit_quarters'])
        for parameters,source in controls:
            for detail in case['details']:
                with self.subTest(input=type(parameters).__name__,detail=detail):
                    expected=M.capsule_assembly(source,detail,case['family'])
                    actual=V._cap_parts(V._cap(parameters),detail,layout=case['family'])
                    self.assertEqual([part.key for part,mesh in actual],[part.key for part in expected.parts])
                    for (part,mesh),shared in zip(actual,expected.parts):
                        self.assertEqual(len(mesh.faces),len(shared.solid.faces),part.key)
                        np.testing.assert_allclose(mesh.V,np.asarray(shared.solid.vertices),rtol=0,atol=1e-12,err_msg=part.key)

    def test_source_exterior_core_inventory_is_not_replaced_by_a_role_whitelist(self):
        tower=M.towers()[C['core_exterior']['tower']]
        expected=[part for part in M.core_assembly(tower,detail='full').parts if np.asarray(part.solid.vertices)[:,2].max()>=0]
        groups={}
        items=V.NakaginDrawing()._core_items(tower,storeys=tower.storeys,side=tower.core_side,pitch=tower.storey_height,z0=0,tag=tower.index,lod=True,drawing=lambda tag,group:groups.update(group.semantic_parts))
        self.assertEqual(set(groups),{part.key for part in expected})
        self.assertTrue(set(C['core_exterior']['required_roles']).issubset({part.role for part in expected}))
        self.assertTrue(items)
        slot=M.slots()[0]
        actual=V._slot_parts(slot)
        direct=M.capsule_assembly(M.capsule_type(slot.type),'exterior',slot.layout)
        for (part,mesh),source in zip(actual,direct.parts):
            self.assertEqual(part.key,source.key)
            np.testing.assert_array_equal(mesh.V,np.asarray(source.solid.vertices))
            self.assertEqual(len(mesh.faces),len(source.solid.faces))

    def test_physical_depth_includes_height(self):
        view=V.make_view(tuple(C['depth']['view']),(0,0,0),1,(0,0,0))
        points=C['depth']['points']
        actual=[V._order(view,p) for p in points]
        reference=np.dot(np.array(points,dtype=float),np.array(view.towards,dtype=float))
        self.assertTrue(np.allclose(actual,reference))
        self.assertEqual(list(np.argsort(actual)),C['depth']['order'])
        self.assertGreater(actual[2],actual[1])
    def test_dimensions_measure_drawn_physical_casing_not_nominal_program_box(self):
        class Sheet(V.NakaginDrawing,V._d.Drafting):
            language='en'
            def number(self,value,digits=2):
                return f'{value:.{digits}f}'
        spec=C['dimension_reference']
        prototype=M.capsule_type('A')
        np.testing.assert_allclose([prototype.width,prototype.length,prototype.height],spec['nominal'],atol=1e-10,rtol=0)
        shell=[np.asarray(part.solid.vertices) for part in M.capsule_assembly(prototype,detail='exterior').parts if part.role in ('shell','shell_back')]
        points=np.vstack(shell)
        measured=np.ptp(points,axis=0)
        np.testing.assert_allclose(measured,spec['physical'],atol=1e-8,rtol=0)
        drawing=Sheet().capsule_axon(prototype,dims=('width','length','height'))
        for name,value in zip(('width','length','height'),spec['physical']):
            dimension=drawing.dims[name]
            self.assertAlmostEqual(dimension.measurement_value,value,places=8)
            self.assertAlmostEqual(float(np.linalg.norm(np.diff(dimension.world_endpoints,axis=0))),value,places=8)
        views=Sheet().capsule_views(prototype,dims=True)
        for group,names in ((views.parts['plan'],('width','length')),(views.parts['front'],('width','height')),(views.parts['side'],('length','height'))):
            for name in names:
                self.assertAlmostEqual(group.dims[name].measurement_value,dict(zip(('width','length','height'),spec['physical']))[name],places=8)

    def test_custom_view_lining_preserves_missing_explicit_and_typed_interiors(self):
        from dataclasses import replace
        spec=C['interior_controls']
        typed=replace(M.capsule_type('A'),**{key:value for key,value in spec['parameters'].items() if key not in ('key','dimensional_basis')})
        explicit={**spec['parameters'],**dict(zip(('interior_width','interior_length','interior_height'),spec['explicit']))}
        thickness=M.load()['detail']['lining_thickness']['value']
        for parameters,expected in ((spec['parameters'],spec['automatic']),(explicit,spec['explicit']),(typed,spec['explicit'])):
            with self.subTest(input=type(parameters).__name__,expected=expected):
                source=np.vstack([mesh.V for part,mesh in V._cap_parts(V._cap(parameters),'full') if part.role=='lining'])
                outer=np.asarray(expected)+2*thickness
                np.testing.assert_allclose(np.ptp(source,axis=0),outer,atol=1e-8,rtol=0)
                drawing=V.NakaginDrawing().capsule_axon(parameters,cutaway=True)
                points=np.vstack([shape.world_vertices for shape in drawing if getattr(shape,'semantic_role',None)=='lining'])
                cut=outer.copy()
                cut[[0,2]]-=spec['cut_depth']+thickness
                np.testing.assert_allclose(np.ptp(points,axis=0),cut,atol=1e-8,rtol=0)

    def test_dimension_basis_is_explicit_and_physical_controls_are_applied_once(self):
        physical=C['dimensional_basis']['physical']
        ambiguous={key:value for key,value in physical.items() if key!='dimensional_basis'}
        with self.assertRaises(ValueError):
            V._cap(ambiguous)
        nominal=M.capsule_parameters_from_casing(physical)
        for controls in (physical,nominal):
            cap=V._cap(controls)
            np.testing.assert_allclose([cap[key] for key in ('w','l','h')],C['dimensional_basis']['expected_nominal'],atol=1e-10,rtol=0)
            parts=V._cap_parts(cap)
            points=np.vstack([mesh.V for part,mesh in parts if part.role in ('shell','shell_back')])
            np.testing.assert_allclose(np.ptp(points,axis=0),C['dimension_reference']['physical'],atol=1e-8,rtol=0)

    def test_analytical_volume_box_has_exact_dimensions_without_casing_allowances(self):
        class Sheet(V.NakaginDrawing,V._d.Drafting):
            language='en'
            def number(self,value,digits=2):
                return f'{value:.{digits}f}'
        dimensions=C['analytical_volume']
        drawing=Sheet().analytical_box_axon(*dimensions,dims=True)
        points=np.vstack([shape.world_vertices for shape in drawing if hasattr(shape,'world_vertices')])
        np.testing.assert_allclose(np.ptp(points,axis=0),dimensions,atol=1e-9,rtol=0)
        for key,value in zip(('width','length','height'),dimensions):
            self.assertAlmostEqual(drawing.dims[key].measurement_value,value,places=8)

    def test_capsule_parts_are_shared_geometry(self):
        scene=V.NakaginDrawing()
        capsule=scene.capsule_axon(M.capsule_type(C['capsule']['type']),bolts=True)
        roles={shape.semantic_role for shape in capsule if hasattr(shape,'semantic_role')}
        self.assertTrue(set(C['capsule']['required_roles'])<=roles,roles)
        cut=scene.capsule_axon(M.capsule_type('A'),cutaway=True)
        roles={shape.semantic_role for shape in cut if hasattr(shape,'semantic_role')}
        self.assertTrue(set(C['capsule']['cutaway_roles'])<=roles,roles)
    def test_tower_depth_selections_and_cached_transform(self):
        scene=V.NakaginDrawing()
        first=scene.tower_axon(types=False)
        self.assertEqual(len(first.capsules),C['capsules'])
        self.assertTrue(first.bridges.submobjects)
        shapes=[shape for shape in first if hasattr(shape,'world_depth')]
        self.assertTrue(shapes)
        depths=np.asarray([shape.world_depth for shape in shapes])
        expected=np.asarray([V._order(first.view,np.asarray(shape.world_vertices).mean(axis=0)) for shape in shapes])
        self.assertTrue(np.isfinite(depths).all())
        np.testing.assert_allclose(depths,expected,atol=1e-8,rtol=0)
        second=scene.tower_axon(types=False,at=(2,1,0),scale=.2)
        key=next(iter(first.capsules))
        a=first.capsules[key].get_center()
        b=second.capsules[key].get_center()
        self.assertTrue(np.allclose(b,2*a+np.array([2,1,0])))
        second.shift((1,0,0))
        third=scene.tower_axon(types=False)
        self.assertTrue(np.allclose(first.get_center(),third.get_center()))
    def test_window_toggle_changes_the_actual_visible_opening(self):
        from manim import Camera,tempconfig
        spec=C['window_cut']
        pixels=[]
        with tempconfig({'pixel_width':spec['pixel_size'],'pixel_height':spec['pixel_size'],'frame_width':6,'frame_height':6}):
            for opened in (False,True):
                drawing=V.NakaginDrawing().capsule_axon(M.capsule_type('A'),view=spec['view'],scale=1,window=opened)
                camera=Camera(background_color=V.BACKGROUND)
                camera.capture_mobjects([drawing])
                pixels.append(camera.pixel_array[spec['pixel_size']//2,spec['pixel_size']//2,:3].astype(float))
        self.assertGreater(np.linalg.norm(pixels[0]-pixels[1]),spec['minimum_rgb_difference'])

    def test_program_study_cache_keeps_type_identity(self):
        """🛋 Preserve each program's independently specified furniture through the cached renderer."""
        scene=V.NakaginDrawing()
        for spec in C['study_variants']:
            with self.subTest(program=spec['type']):
                capsule=scene.capsule_axon(M.capsule_type(spec['type']),cutaway=True,interior='study')
                roles={shape.semantic_role for shape in capsule if hasattr(shape,'semantic_role')}
                self.assertTrue(set(spec['required_roles'])<=roles,roles)
                self.assertFalse(set(spec['excluded_roles'])&roles,roles)

    def test_live_core_dimensions_change(self):
        scene=V.NakaginDrawing()
        short=scene.core_axon(0,storeys=3)
        tall=scene.core_axon(0,storeys=6)
        self.assertGreater(tall.height,short.height)
        narrow=scene.core_axon(0,side=4)
        wide=scene.core_axon(0,side=7)
        self.assertGreater(wide.width,narrow.width)
    def test_partial_base_storeys_follow_actual_podium_datums(self):
        scene=V.NakaginDrawing()
        original=V._core_parts
        for count in C["podium_only"]["storeys"]:
            with self.subTest(storeys=count):
                walls=[]
                def measured(*args,**kwargs):
                    parts=original(*args,**kwargs)
                    walls.extend(mesh.V for part,mesh in parts if part.role=="core_wall")
                    return parts
                cores,capsules,capitals={},{},{}
                with patch.object(V,"_core_parts",side_effect=measured):
                    items=scene._tower_items(lit=None,tint=None,types=False,capital=True,podium=False,where=None,core=True,ground=False,segments=2,capsules=capsules,cores=cores,capitals=capitals,base=V.CAPSULE,towers=(0,),storeys=count)
                self.assertEqual(len(capsules),C["podium_only"]["capsules"])
                self.assertEqual(len(capitals),C["podium_only"]["capitals"])
                if count==0:
                    self.assertFalse(items)
                    self.assertFalse(walls)
                else:
                    self.assertTrue(walls)
                    self.assertAlmostEqual(np.vstack(walls)[:,2].max(),M.load()["podium"]["levels"][count]["value"],places=6)

    def test_bridge_drawing_retains_physical_frames_and_return(self):
        parts=V._bridge_parts()
        self.assertTrue(set(C["bridge_roles"]) <= {part.role for part,mesh in parts})
        expected={part.key:part for level in M.load()["detail"]["bridge_storeys"] for part in M.bridge_assembly(level).parts}
        self.assertEqual({part.key for part,mesh in parts},set(expected))
        for part,mesh in parts:
            np.testing.assert_allclose(mesh.V,np.asarray(expected[part.key].solid.vertices),atol=1e-10,rtol=0)

    def test_empty_construction_view_is_render_safe(self):
        for ground in C["podium_only"]["empty_ground_flags"]:
            drawing=V.NakaginDrawing().tower_axon(storeys=0,podium=False,ground=ground)
            self.assertEqual(np.asarray(drawing.get_all_points()).size,0)
            self.assertFalse(drawing.cores)
            self.assertFalse(drawing.capitals)
            self.assertFalse(drawing.capsules)
            np.testing.assert_allclose(drawing.view.centre,np.zeros(3),atol=1e-12,rtol=0)

if __name__=='__main__':
    unittest.main(verbosity=2)

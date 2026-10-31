"""📦 Verify physical fabrication bounds separately from nominal study and usable interior dimensions."""
import dataclasses
import importlib
import json
from pathlib import Path
import sys
import unittest
import numpy as np
import rhino3dm as rh

ROOT=next(p for p in Path(__file__).resolve().parents if (p/'🧰️framework').is_dir())
sys.path.insert(0,str(ROOT))
BASE=ROOT/'🎓️teaching/🏛️architecture/🏢️nakagin'
M=importlib.import_module('🎓️teaching.🏛️architecture.🏢️nakagin.🐍️')
C=json.loads((BASE/'🧪️cases/📦️casing/🔣️.json').read_text(encoding='utf8'))

def material_segment_hit(parts,start,end):
    """🧮 Independently intersect finite rays with actual material triangles using NumPy."""
    triangles=[]
    for part in parts:
        for face in part.solid.faces:
            triangles.extend((part.solid.vertices[face[0]],part.solid.vertices[face[i]],part.solid.vertices[face[i+1]]) for i in range(1,len(face)-1))
    triangles=np.asarray(triangles)
    direction=np.asarray(end)-start
    edge1,edge2=triangles[:,1]-triangles[:,0],triangles[:,2]-triangles[:,0]
    h=np.cross(direction,edge2)
    determinant=np.einsum('ij,ij->i',edge1,h)
    valid=np.abs(determinant)>1e-12
    inv=np.divide(1,determinant,out=np.zeros_like(determinant),where=valid)
    offset=np.asarray(start)-triangles[:,0]
    u=np.einsum('ij,ij->i',offset,h)*inv
    q=np.cross(offset,edge1)
    v=q@direction*inv
    distance=np.einsum('ij,ij->i',edge2,q)*inv
    return bool(np.any(valid&(u>=-1e-10)&(v>=-1e-10)&(u+v<=1+1e-10)&(distance>=0)&(distance<=1)))


class PhysicalCasing(unittest.TestCase):
    def test_reference_and_live_fabrication_dimensions(self):
        """📏 Preserve explicit fabrication allowances under nominal control changes."""
        t=M.capsule_type('A')
        casing=M.capsule_casing(t)
        self.assertEqual([casing.width,casing.length,casing.height],C['dimensions'])
        np.testing.assert_allclose([casing.left,casing.right,casing.back,casing.front,casing.bottom,casing.top],C['bounds'],atol=C['tolerance'],rtol=0)
        live=M.capsule_casing({**dataclasses.asdict(t),**C['prototype']})
        np.testing.assert_allclose([live.width,live.length,live.height],C['prototype_physical'],atol=C['tolerance'],rtol=0)
        self.assertEqual([t.width,t.length,t.height],[2.5,4,2.5])
    def test_actual_body_surfaces_have_physical_bounds_and_rounded_profiles(self):
        """🔲 Inspect real shell vertices with a NumPy rounded-box signed-distance oracle."""
        for family in C['families']:
            parts=M.capsule_assembly('A','exterior',family).parts
            points=np.array([v for p in parts if p.role in ('shell','shell_back') for v in p.solid.vertices])
            np.testing.assert_allclose(points.min(axis=0),np.array(C['bounds'])[::2],atol=C['tolerance'],rtol=0)
            np.testing.assert_allclose(points.max(axis=0),np.array(C['bounds'])[1::2],atol=C['tolerance'],rtol=0)
            q=np.abs(points[:,[0,2]]-[0,1.25])-[1.05,1]
            sdf=np.linalg.norm(np.maximum(q,0),axis=1)+np.minimum(np.max(q,axis=1),0)-.25
            self.assertLessEqual(sdf.max(),C['tolerance'])
    def test_lining_void_matches_independent_usable_dimensions(self):
        """🏠 Locate actual inner side/end/roof faces and the supported finished floor."""
        parts={p.key:p.solid for p in M.capsule_assembly('A').parts}
        bounds=lambda key: np.array([parts[key].vertices]).reshape(-1,3)
        width,length,height=C['interior']
        self.assertAlmostEqual(bounds('lining-left')[:,0].max(),-width/2)
        self.assertAlmostEqual(bounds('lining-right')[:,0].min(),width/2)
        self.assertAlmostEqual(bounds('lining-back')[:,1].max(),2-length/2)
        self.assertAlmostEqual(bounds('lining-front')[:,1].min(),2+length/2)
        self.assertAlmostEqual(bounds('lining-roof')[:,2].min(),C['finish']+height)
        self.assertAlmostEqual(bounds('floor-finish')[:,2].max(),C['finish'])
    def test_entries_and_collision_envelopes_match_all140_actual_poses(self):
        """🧭 Independently transform physical bounds and entry planes with Rhino."""
        self.assertEqual(len(M.slots()),C['total'])
        for slot in M.slots():
            t=M.capsule_type(slot.type)
            local=M.capsule_casing(t)
            plane=rh.Plane(rh.Point3d(*slot.plane.origin),rh.Vector3d(*slot.plane.x),rh.Vector3d(*slot.plane.y))
            transform=rh.Transform.PlaneToPlane(rh.Plane.WorldXY(),plane)
            corners=[]
            for x in (local.left,local.right):
                for y in (local.back,local.front):
                    for z in (local.bottom,local.top):
                        point=rh.Point3d(x,y,z)
                        point.Transform(transform)
                        corners.append([point.X,point.Y,point.Z])
            envelope=np.asarray(M.slot_solid(slot).vertices)
            np.testing.assert_allclose(envelope.min(axis=0),np.min(corners,axis=0),atol=C['tolerance'],rtol=0)
            np.testing.assert_allclose(envelope.max(axis=0),np.max(corners,axis=0),atol=C['tolerance'],rtol=0)
            entry=M.capsule_entry(t,slot.layout)
            point=rh.Point3d(*entry.origin)
            point.Transform(transform)
            np.testing.assert_allclose([point.X,point.Y,point.Z+C['finish']],slot.facade_plane.origin,atol=C['tolerance'],rtol=0)
    def test_physical_window_midplanes_follow_actual_surface_bounds(self):
        """🪟 Verify family glazing planes against casing faces rather than nominal analysis bounds."""
        data=M.load()
        front=C['recess_front']
        side=C['recess_side']
        for family in C['families']:
            circle=M.capsule_window('A',family)
            expected=(1.3-side,2+C['side_offset'],1.25) if family in ('P','S') else (0,4.1-front,1.25)
            np.testing.assert_allclose(circle.center,expected,atol=C['tolerance'],rtol=0)

    def test_collision_envelope_contains_every_actual_body_facet(self):
        """🛡️ Independently certify every actual body vertex against outward envelope planes."""
        for family in C['families']:
            slot=next(s for s in M.slots() if s.layout==family)
            envelope=M.slot_solid(slot)
            body=np.array([v for p in M.slot_assembly(slot,'exterior').parts if p.role in ('shell','shell_back') for v in p.solid.vertices])
            for face in envelope.faces:
                polygon=np.array([envelope.vertices[i] for i in face])
                normal=next(np.cross(polygon[i]-polygon[0],polygon[i+1]-polygon[0]) for i in range(1,len(polygon)-1) if np.linalg.norm(np.cross(polygon[i]-polygon[0],polygon[i+1]-polygon[0]))>1e-10)
                normal=normal/np.linalg.norm(normal)
                self.assertLessEqual(np.max((body-polygon[0])@normal),C['tolerance'],family)
    def test_exterior_reveal_has_native_aperture_and_source_glazing_recess(self):
        """🔎 Verify finite steel reveal geometry, actual aperture radius and physical glass midplane."""
        for family in C['families']:
            circle=M.capsule_window('A',family)
            for part in M.capsule_assembly('A','full',family).parts:
                points=np.asarray(part.solid.vertices)
                relative=points-np.asarray(circle.center)
                depth=relative@np.asarray(circle.normal)
                radial=np.linalg.norm(relative-depth[:,None]*np.asarray(circle.normal),axis=1)
                if part.role in ('window_rim','window_frame'):
                    self.assertAlmostEqual(radial.max(),C['aperture_radius']+C['steel_skin'])
                    self.assertAlmostEqual(radial.min(),C['aperture_radius'])
                    front=1.3 if family in ('P','S') else 4.1
                    projection=points@np.asarray(circle.normal)
                    near=front-C['reveal_depth'] if part.role=='window_frame' else front-C['steel_skin']
                    far=front-C['steel_skin'] if part.role=='window_frame' else front
                    self.assertAlmostEqual(projection.min(),near)
                    self.assertAlmostEqual(projection.max(),far)
                if part.role=='window_lining':
                    self.assertLessEqual(radial.max(),C['aperture_radius']+M.load()['detail']['frame_width']['value']+C['tolerance'])
                if part.role=='glazing':
                    self.assertAlmostEqual((depth.min()+depth.max())/2,0)

    def test_explicit_equal_reference_interior_is_authoritative(self):
        """🎛️ Preserve present interior controls even when equal to the reference, while missing fields derive."""
        auto=M.capsule_interior(C['prototype'])
        np.testing.assert_allclose([auto.width,auto.length,auto.height],C['prototype_interior'],atol=C['tolerance'],rtol=0)
        given={**C['prototype'],**dict(zip(('interior_width','interior_length','interior_height'),C['explicit_reference_interior']))}
        explicit=M.capsule_interior(given)
        np.testing.assert_allclose([explicit.width,explicit.length,explicit.height],C['explicit_reference_interior'],atol=C['tolerance'],rtol=0)
        typed=M.capsule_interior(dataclasses.replace(M.capsule_type('A'),**C['prototype']))
        np.testing.assert_allclose([typed.width,typed.length,typed.height],C['explicit_reference_interior'],atol=C['tolerance'],rtol=0)

    def test_reference_fittings_remain_inside_and_contact_usable_floor_ceiling(self):
        """🛋️ Check actual fixture vertices, supported floor objects and ceiling mounting faces."""
        for spec in C['fitting_specs']:
            parameters={**dataclasses.asdict(M.capsule_type('A')),**spec}
            if spec and 'interior_width' not in spec:
                for key in ('interior_width','interior_length','interior_height'):
                    parameters.pop(key)
            usable=M.capsule_interior(parameters)
            for family in C['families']:
                parts={p.key:p for p in M._reference_interior(parameters,family)}
                for part in parts.values():
                    points=np.asarray(part.solid.vertices)
                    self.assertTrue(np.all(points.min(axis=0)>=[usable.left-C['tolerance'],usable.back-C['tolerance'],usable.bottom-C['tolerance']]),(spec,family,part.key,'lower'))
                    self.assertTrue(np.all(points.max(axis=0)<=[usable.right+C['tolerance'],usable.front+C['tolerance'],usable.top+C['tolerance']]),(spec,family,part.key,'upper'))
                for key in C['floor_contacts']:
                    self.assertAlmostEqual(min(v[2] for v in parts[key].solid.vertices),usable.bottom)
                for key in C['ceiling_contacts']:
                    self.assertAlmostEqual(max(v[2] for v in parts[key].solid.vertices),usable.top)
    def test_study_fittings_honor_explicit_usable_bounds_without_resizing_areas(self):
        """📐 Inspect all actual study parts under fixed-interior and automatic wider-control cases."""
        for key in ('A','B','C','D'):
            for spec in C['fitting_specs']:
                parameters={**dataclasses.asdict(M.capsule_type(key)),**spec}
                if spec and 'interior_width' not in spec:
                    for name in ('interior_width','interior_length','interior_height'):
                        parameters.pop(name)
                usable=M.capsule_interior(parameters)
                for family in C['families']:
                    for part in M._study_interior(parameters,family):
                        points=np.asarray(part.solid.vertices)
                        self.assertTrue(np.all(points.min(axis=0)>=[usable.left-C['tolerance'],usable.back-C['tolerance'],usable.bottom-C['tolerance']]),(key,spec,family,part.key,'lower'))
                        self.assertTrue(np.all(points.max(axis=0)<=[usable.right+C['tolerance'],usable.front+C['tolerance'],usable.top+C['tolerance']]),(key,spec,family,part.key,'upper'))
                        if part.solid.meta.get('primary_footprint'):
                            authored=next(f for f in parameters['fittings'] if f['key']==part.solid.meta['program_fitting'])
                            self.assertAlmostEqual(float(np.prod(np.ptp(points[:,:2],axis=0))),authored['width']*authored['length'])

    def test_actual_steel_has_native_full3d_rounding_and_inner_offset(self):
        """🌐 Apply the independent Minkowski-box SDF to every actual steel patch vertex."""
        for family in C['families']:
            parts=[p for p in M.capsule_assembly('A','full',family).parts if p.role in ('shell','shell_back')]
            points=np.array([v for p in parts for v in p.solid.vertices])
            q=np.abs(points-C['rounded_centre'])-C['rounded_core']
            sdf=np.linalg.norm(np.maximum(q,0),axis=1)+np.minimum(np.max(q,axis=1),0)-C['fillet_radius']
            self.assertLessEqual(sdf.max(),C['tolerance'],family)
            self.assertGreaterEqual(sdf.min(),-C['steel_skin']-C['tolerance'],family)
            for witness in (C['spherical_witness'],C['longitudinal_witness']):
                self.assertLessEqual(np.min(np.linalg.norm(points-witness,axis=1)),C['tolerance'],(family,witness))
    def test_all_internal_components_stay_within_actual_3d_casing(self):
        """🏗️ Reject hidden truss/floor/lining/fitting spikes; exempt only individually explained external hardware."""
        for family in C['families']:
            for part in M.capsule_assembly('A','full',family).parts:
                if any(part.key.startswith(prefix) for prefix in C['external_hardware']):
                    continue
                points=np.asarray(part.solid.vertices)
                q=np.abs(points-C['rounded_centre'])-C['rounded_core']
                sdf=np.linalg.norm(np.maximum(q,0),axis=1)+np.minimum(np.max(q,axis=1),0)-C['fillet_radius']
                self.assertLessEqual(sdf.max(),C['tolerance'],(family,part.key))

    def test_curved_material_faces_are_planar_and_safe_to_triangulate(self):
        """🔺 Reject concave annular caps or warped quads that a consumer could incorrectly fan-fill."""
        for part in M.capsule_assembly('A','full','Slash').parts:
            if part.role not in ('shell','shell_back'):
                continue
            for face in part.solid.faces:
                points=np.array([part.solid.vertices[i] for i in face])
                normal=next(np.cross(points[i]-points[0],points[i+1]-points[0]) for i in range(1,len(points)-1) if np.linalg.norm(np.cross(points[i]-points[0],points[i+1]-points[0]))>1e-12)
                normal=normal/np.linalg.norm(normal)
                self.assertLessEqual(np.max(np.abs((points-points[0])@normal)),C['tolerance'])
                turns=[np.dot(np.cross(points[(i+1)%len(points)]-points[i],points[(i+2)%len(points)]-points[(i+1)%len(points)]),normal) for i in range(len(points))]
                self.assertTrue(all(v>=-C['tolerance'] for v in turns) or all(v<=C['tolerance'] for v in turns),part.key)

    def test_full_door_width_cuts_both_steel_and_lining_through_lower3d_roll(self):
        """🚪 Check every family at the.2001m sill and both jambs, with positive solid-wall controls."""
        width=M.load()['detail']['door_width']['value']
        for family in C['families']:
            entry=M.capsule_entry('A',family)
            parts=[p for p in M.capsule_assembly('A','full',family).parts if p.role in ('shell','shell_back','lining')]
            def ray(fraction,height):
                point=np.asarray(entry.origin)+np.asarray(entry.x)*width*fraction+np.asarray(entry.z)*height
                return material_segment_hit(parts,point-np.asarray(entry.y)*C['door_rays']['outside_depth'],point+np.asarray(entry.y)*C['door_rays']['inside_depth'])
            for fraction in C['door_rays']['width_fractions']:
                for height in C['door_rays']['heights']:
                    self.assertFalse(ray(fraction,height),(family,fraction,height))
            for fraction in C['door_rays']['material_controls']:
                self.assertTrue(ray(fraction,1.1),(family,fraction,'material'))

    def test_actual_curved_patches_are_watertight_positive_and_honor_real_lod(self):
        """🧊 Reproduce actual closed material meshes with Rhino under three geometric tessellations."""
        counts=[]
        for segments in C['quarter_lods']:
            parameters={**dataclasses.asdict(M.capsule_type('A')),'corner_segments':segments}
            parts=[p for p in M.capsule_assembly(parameters,'exterior','Slash').parts if p.role in ('shell','shell_back')]
            counts.append(sum(len(p.solid.vertices) for p in parts))
            for part in parts:
                mesh=rh.Mesh()
                for vertex in part.solid.vertices:
                    mesh.Vertices.Add(*vertex)
                triangles=[]
                for face in part.solid.faces:
                    self.assertIn(len(face),(3,4))
                    mesh.Faces.AddFace(*face)
                    triangles.extend((part.solid.vertices[face[0]],part.solid.vertices[face[i]],part.solid.vertices[face[i+1]]) for i in range(1,len(face)-1))
                self.assertTrue(mesh.IsValid,(segments,part.key))
                self.assertTrue(mesh.IsClosed,(segments,part.key))
                triangles=np.asarray(triangles)
                volume=np.einsum('ij,ij->i',triangles[:,0],np.cross(triangles[:,1],triangles[:,2])).sum()/6
                self.assertGreater(volume,1e-12,(segments,part.key))
        self.assertTrue(all(a<b for a,b in zip(counts,counts[1:])),counts)
    def test_floor_build_up_has_actual_face_contact_with_inner_steel(self):
        """🧱 Measure positive coplanar bearing area between actual floor and steel faces."""
        parts={p.key:p for p in M.capsule_assembly('A').parts}
        rectangles=[]
        for key in ('floor-finish','shell-floor'):
            faces=[]
            for face in parts[key].solid.faces:
                points=np.array([parts[key].solid.vertices[i] for i in face])
                if np.all(np.abs(points[:,2]-C['steel_skin'])<=C['tolerance']):
                    faces.append((points[:,:2].min(axis=0),points[:,:2].max(axis=0)))
            rectangles.append(faces)
        area=max(np.prod(np.maximum(np.minimum(a1,b1)-np.maximum(a0,b0),0)) for a0,a1 in rectangles[0] for b0,b1 in rectangles[1])
        self.assertAlmostEqual(float(area),C['floor_bearing_area'])

    def test_explicit_interior_lining_faces_are_simple_closed_positive_material(self):
        """🧩 Reject the negative-inner-radius self-crossings under fixed interior overrides."""
        parameters={**dataclasses.asdict(M.capsule_type('A')),**C['fitting_specs'][1]}
        parts=[p for p in M.capsule_assembly(parameters,'full','Slash').parts if p.role=='lining']
        for part in parts:
            mesh=rh.Mesh()
            triangles=[]
            for vertex in part.solid.vertices:
                mesh.Vertices.Add(*vertex)
            for face in part.solid.faces:
                points=np.asarray([part.solid.vertices[i] for i in face])
                normal=np.sum(np.cross(points,np.roll(points,-1,axis=0)),axis=0)
                uv=np.delete(points,int(np.argmax(np.abs(normal))),axis=1)
                def cross(a,b,c):
                    return (b[0]-a[0])*(c[1]-a[1])-(b[1]-a[1])*(c[0]-a[0])
                for i in range(len(uv)):
                    a,b=uv[i],uv[(i+1)%len(uv)]
                    for j in range(i+2,len(uv)):
                        if i==0 and j==len(uv)-1:
                            continue
                        c,d=uv[j],uv[(j+1)%len(uv)]
                        self.assertFalse(cross(a,b,c)*cross(a,b,d)<-1e-16 and cross(c,d,a)*cross(c,d,b)<-1e-16,(part.key,i,j))
                for i in range(1,len(face)-1):
                    mesh.Faces.AddFace(face[0],face[i],face[i+1])
                    triangles.append([part.solid.vertices[face[0]],part.solid.vertices[face[i]],part.solid.vertices[face[i+1]]])
            self.assertTrue(mesh.IsValid,part.key)
            self.assertTrue(mesh.IsClosed,part.key)
            triangles=np.asarray(triangles)
            self.assertGreater(np.einsum('ij,ij->i',triangles[:,0],np.cross(triangles[:,1],triangles[:,2])).sum()/6,1e-12,part.key)
    def test_clear_interior_profile_matches_actual_lining_facets(self):
        """📐 Match the project cavity polygon with real lining inner-corner vertices."""
        for spec in C['fitting_specs']:
            parameters={**dataclasses.asdict(M.capsule_type('A')),**spec}
            if spec and 'interior_width' not in spec:
                for key in ('interior_width','interior_length','interior_height'):
                    parameters.pop(key)
            profile=np.asarray(M.capsule_interior_profile(parameters,'full'))
            usable=M.capsule_interior(parameters)
            np.testing.assert_allclose(profile.min(axis=0),[usable.left,usable.bottom],atol=C['tolerance'],rtol=0)
            np.testing.assert_allclose(profile.max(axis=0),[usable.right,usable.top],atol=C['tolerance'],rtol=0)
            points=np.array([(x,z) for p in M.capsule_assembly(parameters,'full','Slash').parts if p.key.startswith('lining-corner-') for x,y,z in p.solid.vertices])
            for point in profile:
                self.assertLessEqual(np.min(np.linalg.norm(points-point,axis=1)),C['tolerance'])

    def test_physical_control_factory_converts_once_and_preserves_explicit_fields(self):
        """🎚️ Match physical controls with one declared allowance conversion and no interior backfill."""
        for case in C['control_conversions']:
            source=dict(case['physical'])
            nominal=M.capsule_parameters_from_casing(source)
            self.assertEqual(source,case['physical'])
            self.assertEqual(nominal['dimensional_basis'],'nominal')
            with self.assertRaises(ValueError):
                M.capsule_parameters_from_casing(nominal)
            np.testing.assert_allclose([nominal[k] for k in ('width','length','height')],case['nominal'],atol=C['tolerance'],rtol=0)
            restored=M.capsule_casing(nominal)
            np.testing.assert_allclose([restored.width,restored.length,restored.height],[source[k] for k in ('width','length','height')],atol=C['tolerance'],rtol=0)
            for key,value in source.items():
                if key not in ('width','length','height'):
                    self.assertEqual(nominal[key],value)
            for key in ('interior_width','interior_length','interior_height'):
                self.assertEqual(key in nominal,key in source)

    def test_both_entry_jambs_and_head_use_the_finished_sill_datum(self):
        """🚪 Check actual left/right jamb bases and crowns against the same explicit floor datum."""
        finish=M.load()['detail']['floor_finish_height']['value']
        height=M.load()['detail']['door_height']['value']
        for family in C['families']:
            parts={p.key:p for p in M.capsule_assembly('A','exterior',family).parts}
            for key in ('entry-frame-left','entry-frame-right'):
                z=np.asarray(parts[key].solid.vertices)[:,2]
                self.assertAlmostEqual(z.min(),finish,msg=(family,key))
                self.assertAlmostEqual(z.max(),finish+height+.035,msg=(family,key))
            self.assertAlmostEqual(min(v[2] for v in parts['entry-frame-head'].solid.vertices),finish+height)

    def test_zero_and_tiny_rounding_controls_keep_full_material_inside_the_physical_box(self):
        """📦 Preserve the actual sharp Box stage and continuous finite-wall transitions without truss spikes."""
        for radius in C['rounding_controls']:
            parameters={**dataclasses.asdict(M.capsule_type('A')),'corner_radius':radius}
            core=np.array(C['dimensions'])/2-radius
            for family in C['families']:
                parts=M.capsule_assembly(parameters,'full',family).parts
                for part in parts:
                    if any(part.key.startswith(prefix) for prefix in C['external_hardware']):
                        continue
                    points=np.asarray(part.solid.vertices)
                    q=np.abs(points-C['rounded_centre'])-core
                    sdf=np.linalg.norm(np.maximum(q,0),axis=1)+np.minimum(np.max(q,axis=1),0)-radius
                    self.assertLessEqual(sdf.max(),C['tolerance'],(radius,family,part.key))
                for part in (p for p in parts if p.role in ('shell','shell_back')):
                    mesh=rh.Mesh()
                    for vertex in part.solid.vertices:
                        mesh.Vertices.Add(*vertex)
                    for face in part.solid.faces:
                        for i in range(1,len(face)-1):
                            mesh.Faces.AddFace(face[0],face[i],face[i+1])
                    self.assertTrue(mesh.IsValid,(radius,family,part.key))
                    self.assertTrue(mesh.IsClosed,(radius,family,part.key))

    def test_every_shell_patch_has_two_incident_material_faces_per_edge(self):
        """🧩 Verify actual welded manifold topology independently of Rhino's closedness flag."""
        contract=C['material_topology']
        for radius in contract['radii']:
            parameters={**dataclasses.asdict(M.capsule_type('A')),'corner_radius':radius}
            for detail in contract['details']:
                for family in C['families']:
                    for part in M.capsule_assembly(parameters,detail,family).parts:
                        if part.role not in ('shell','shell_back'):
                            continue
                        edges=np.asarray([tuple(sorted((a,b))) for face in part.solid.faces for a,b in zip(face,face[1:]+face[:1])])
                        _,counts=np.unique(edges,axis=0,return_counts=True)
                        self.assertTrue(np.all(counts==contract['edge_incidence']),(radius,detail,family,part.key,sorted(set(counts))))
                        directed={}
                        for face in part.solid.faces:
                            for a,b in zip(face,face[1:]+face[:1]):
                                edge=tuple(sorted((a,b)))
                                directed[edge]=directed.get(edge,0)+(1 if a<b else -1)
                        self.assertEqual(set(directed.values()),{contract['directed_edge_balance']},(radius,detail,family,part.key))

if __name__=='__main__':
    unittest.main()

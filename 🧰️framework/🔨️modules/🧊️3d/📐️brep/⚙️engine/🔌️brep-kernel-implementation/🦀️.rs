//! 🔌️ B-Rep kernel implementation: the kernel trait implementation of the session that delegates every operation to the synchronous API. See [`super`].

use super::*;

// #region 🔖️BrepKernelImpl
impl BrepKernel for Brep {
    fn box_prim(&mut self, width: f64, depth: f64, height: f64) -> Result<GeometryHandle, BrepError> {
        self.box_prim_sync(width, depth, height)
    }
    fn sphere_prim(&mut self, radius: f64) -> Result<GeometryHandle, BrepError> {
        self.sphere_prim_sync(radius)
    }
    fn cylinder_prim(&mut self, radius: f64, height: f64) -> Result<GeometryHandle, BrepError> {
        self.cylinder_prim_sync(radius, height)
    }
    fn cone_prim(&mut self, radius: f64, height: f64) -> Result<GeometryHandle, BrepError> {
        self.cone_prim_sync(radius, height)
    }
    fn torus_prim(&mut self, major: f64, minor: f64) -> Result<GeometryHandle, BrepError> {
        self.torus_prim_sync(major, minor)
    }
    fn convex_hull(&mut self, points: &[EVec3]) -> Result<GeometryHandle, BrepError> {
        self.convex_hull_sync(points)
    }
    fn line_curve(&mut self, start: EVec3, end: EVec3) -> Result<GeometryHandle, BrepError> {
        self.line_curve_sync(start, end)
    }
    fn circle_curve(&mut self, center: EVec3, normal: EVec3, radius: f64) -> Result<GeometryHandle, BrepError> {
        self.circle_curve_sync(center, normal, radius)
    }
    fn arc_curve(&mut self, center: EVec3, normal: EVec3, radius: f64, start_angle: f64, end_angle: f64) -> Result<GeometryHandle, BrepError> {
        self.arc_curve_sync(center, normal, radius, start_angle, end_angle)
    }
    fn ellipse_curve(&mut self, center: EVec3, normal: EVec3, semi_major: f64, semi_minor: f64) -> Result<GeometryHandle, BrepError> {
        self.ellipse_curve_sync(center, normal, semi_major, semi_minor)
    }
    fn polyline_wire(&mut self, points: &[EVec3]) -> Result<GeometryHandle, BrepError> {
        self.polyline_wire_sync(points)
    }
    fn rectangle_wire(&mut self, width: f64, height: f64) -> Result<GeometryHandle, BrepError> {
        self.rectangle_wire_sync(width, height)
    }
    fn regular_polygon_wire(&mut self, radius: f64, sides: usize) -> Result<GeometryHandle, BrepError> {
        self.regular_polygon_wire_sync(radius, sides)
    }
    fn interpolate_curve(&mut self, points: &[EVec3], degree: usize) -> Result<GeometryHandle, BrepError> {
        self.interpolate_curve_sync(points, degree)
    }
    fn approximate_curve(&mut self, points: &[EVec3], degree: usize, control_points: usize) -> Result<GeometryHandle, BrepError> {
        self.approximate_curve_sync(points, degree, control_points)
    }
    fn helix_curve(&mut self, origin: EVec3, axis: EVec3, radius: f64, pitch: f64, turns: f64) -> Result<GeometryHandle, BrepError> {
        self.helix_curve_sync(origin, axis, radius, pitch, turns)
    }
    fn plane_surface(&mut self, origin: EVec3, normal: EVec3) -> Result<GeometryHandle, BrepError> {
        self.plane_surface_sync(origin, normal)
    }
    fn planar_face_from_points(&mut self, points: &[EVec3]) -> Result<GeometryHandle, BrepError> {
        self.planar_face_from_points_sync(points)
    }
    fn planar_face_from_wire(&mut self, wire: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.planar_face_from_wire_sync(wire)
    }
    fn nurbs_surface_from_grid(&mut self, points: &[Vec<EVec3>], degree_u: usize, degree_v: usize) -> Result<GeometryHandle, BrepError> {
        self.nurbs_surface_from_grid_sync(points, degree_u, degree_v)
    }
    fn coons_patch(&mut self, curves: &[Vec<EVec3>]) -> Result<GeometryHandle, BrepError> {
        self.coons_patch_sync(curves)
    }
    fn offset_face(&mut self, face: &GeometryHandle, distance: f64) -> Result<GeometryHandle, BrepError> {
        self.offset_face_sync(face, distance)
    }
    fn thicken_face(&mut self, face: &GeometryHandle, thickness: f64) -> Result<GeometryHandle, BrepError> {
        self.thicken_face_sync(face, thickness)
    }
    fn extrude_wire(&mut self, wire: &GeometryHandle, vector: EVec3) -> Result<GeometryHandle, BrepError> {
        self.extrude_wire_sync(wire, vector)
    }
    fn extrude(&mut self, face: &GeometryHandle, direction: EVec3, distance: f64) -> Result<GeometryHandle, BrepError> {
        self.extrude_sync(face, direction, distance)
    }
    fn revolve(&mut self, face: &GeometryHandle, axis_origin: EVec3, axis_direction: EVec3, angle: f64) -> Result<GeometryHandle, BrepError> {
        self.revolve_sync(face, axis_origin, axis_direction, angle)
    }
    fn loft(&mut self, profiles: &[GeometryHandle], smooth: bool) -> Result<GeometryHandle, BrepError> {
        self.loft_sync(profiles, smooth)
    }
    fn sweep(&mut self, profile: &GeometryHandle, path: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.sweep_sync(profile, path)
    }
    fn pipe(&mut self, profile: &GeometryHandle, path: &GeometryHandle, guide: Option<&GeometryHandle>) -> Result<GeometryHandle, BrepError> {
        self.pipe_sync(profile, path, guide)
    }
    fn helical_sweep(&mut self, profile: &GeometryHandle, axis_origin: EVec3, axis_dir: EVec3, radius: f64, pitch: f64, turns: f64) -> Result<GeometryHandle, BrepError> {
        self.helical_sweep_sync(profile, axis_origin, axis_dir, radius, pitch, turns)
    }
    fn fuse(&mut self, a: &GeometryHandle, b: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.fuse_sync(a, b)
    }
    fn cut(&mut self, a: &GeometryHandle, b: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.cut_sync(a, b)
    }
    fn intersect(&mut self, a: &GeometryHandle, b: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.intersect_sync(a, b)
    }
    fn compound_cut(&mut self, target: &GeometryHandle, tools: &[GeometryHandle]) -> Result<GeometryHandle, BrepError> {
        self.compound_cut_sync(target, tools)
    }
    fn translate(&mut self, shape: &GeometryHandle, offset: EVec3) -> Result<GeometryHandle, BrepError> {
        self.translate_sync(shape, offset)
    }
    fn rotate(&mut self, shape: &GeometryHandle, axis: EVec3, angle: f64) -> Result<GeometryHandle, BrepError> {
        self.rotate_sync(shape, axis, angle)
    }
    fn rotate_about(&mut self, shape: &GeometryHandle, origin: EVec3, axis: EVec3, angle: f64) -> Result<GeometryHandle, BrepError> {
        self.rotate_about_sync(shape, origin, axis, angle)
    }
    fn scale(&mut self, shape: &GeometryHandle, factor: f64, center: EVec3) -> Result<GeometryHandle, BrepError> {
        self.scale_sync(shape, factor, center)
    }
    fn scale_axes(&mut self, shape: &GeometryHandle, factors: EVec3, center: EVec3) -> Result<GeometryHandle, BrepError> {
        self.scale_axes_sync(shape, factors, center)
    }
    fn mirror(&mut self, shape: &GeometryHandle, origin: EVec3, normal: EVec3) -> Result<GeometryHandle, BrepError> {
        self.mirror_sync(shape, origin, normal)
    }
    fn copy_shape(&mut self, shape: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.copy_shape_sync(shape)
    }
    fn linear_pattern(&mut self, shape: &GeometryHandle, direction: EVec3, spacing: f64, count: usize) -> Result<GeometryHandle, BrepError> {
        self.linear_pattern_sync(shape, direction, spacing, count)
    }
    fn circular_pattern(&mut self, shape: &GeometryHandle, axis: EVec3, count: usize) -> Result<GeometryHandle, BrepError> {
        self.circular_pattern_sync(shape, axis, count)
    }
    fn grid_pattern(&mut self, shape: &GeometryHandle, dir_x: EVec3, dir_y: EVec3, spacing_x: f64, spacing_y: f64, count_x: usize, count_y: usize) -> Result<GeometryHandle, BrepError> {
        self.grid_pattern_sync(shape, dir_x, dir_y, (spacing_x, spacing_y), count_x, count_y)
    }
    fn fillet(&mut self, shape: &GeometryHandle, radius: f64) -> Result<GeometryHandle, BrepError> {
        self.fillet_sync(shape, radius)
    }
    fn fillet_variable(&mut self, shape: &GeometryHandle, radius_start: f64, radius_end: f64) -> Result<GeometryHandle, BrepError> {
        self.fillet_variable_sync(shape, radius_start, radius_end)
    }
    fn fillet_edges(&mut self, shape: &GeometryHandle, edges: &[GeometryHandle], radius: f64) -> Result<GeometryHandle, BrepError> {
        self.fillet_edges_sync(shape, edges, radius)
    }
    fn chamfer(&mut self, shape: &GeometryHandle, distance: f64) -> Result<GeometryHandle, BrepError> {
        self.chamfer_sync(shape, distance)
    }
    fn chamfer_asymmetric(&mut self, shape: &GeometryHandle, d1: f64, d2: f64) -> Result<GeometryHandle, BrepError> {
        self.chamfer_asymmetric_sync(shape, d1, d2)
    }
    fn chamfer_edges(&mut self, shape: &GeometryHandle, edges: &[GeometryHandle], distance: f64) -> Result<GeometryHandle, BrepError> {
        self.chamfer_edges_sync(shape, edges, distance)
    }
    fn shell(&mut self, shape: &GeometryHandle, thickness: f64, open_faces: &[GeometryHandle]) -> Result<GeometryHandle, BrepError> {
        self.shell_sync(shape, thickness, open_faces)
    }
    fn draft(&mut self, shape: &GeometryHandle, faces: &[GeometryHandle], pull_direction: EVec3, neutral_point: EVec3, angle: f64) -> Result<GeometryHandle, BrepError> {
        self.draft_sync(shape, faces, pull_direction, neutral_point, angle)
    }
    fn offset_solid(&mut self, shape: &GeometryHandle, distance: f64) -> Result<GeometryHandle, BrepError> {
        self.offset_solid_sync(shape, distance)
    }
    fn defeature(&mut self, shape: &GeometryHandle, faces: &[GeometryHandle]) -> Result<GeometryHandle, BrepError> {
        self.defeature_sync(shape, faces)
    }
    fn section(&mut self, solid: &GeometryHandle, plane_origin: EVec3, plane_normal: EVec3) -> Result<Vec<GeometryHandle>, BrepError> {
        self.section_sync(solid, plane_origin, plane_normal)
    }
    fn split(&mut self, solid: &GeometryHandle, plane_origin: EVec3, plane_normal: EVec3) -> Result<(GeometryHandle, GeometryHandle), BrepError> {
        self.split_sync(solid, plane_origin, plane_normal)
    }
    fn curve_curve_intersect(&mut self, a: &GeometryHandle, b: &GeometryHandle, tolerance: f64) -> Result<Vec<EVec3>, BrepError> {
        self.curve_curve_intersect_sync(a, b, tolerance)
    }
    fn curve_surface_intersect(&mut self, curve: &GeometryHandle, surface: &GeometryHandle, tolerance: f64) -> Result<Vec<EVec3>, BrepError> {
        self.curve_surface_intersect_sync(curve, surface, tolerance)
    }
    fn surface_surface_intersect(&mut self, a: &GeometryHandle, b: &GeometryHandle, tolerance: f64) -> Result<Vec<GeometryHandle>, BrepError> {
        self.surface_surface_intersect_sync(a, b, tolerance)
    }
    fn curve_point(&self, curve: &GeometryHandle, parameter: f64) -> Result<EVec3, BrepError> {
        self.curve_point_sync(curve, parameter)
    }
    fn curve_tangent(&self, curve: &GeometryHandle, parameter: f64) -> Result<EVec3, BrepError> {
        self.curve_tangent_sync(curve, parameter)
    }
    fn curve_domain(&self, curve: &GeometryHandle) -> Result<ParamDomain, BrepError> {
        self.curve_domain_sync(curve)
    }
    fn curve_curvature(&self, curve: &GeometryHandle, parameter: f64) -> Result<f64, BrepError> {
        self.curve_curvature_sync(curve, parameter)
    }
    fn surface_point(&self, surface: &GeometryHandle, u: f64, v: f64) -> Result<EVec3, BrepError> {
        self.surface_point_sync(surface, u, v)
    }
    fn surface_normal(&self, surface: &GeometryHandle, u: f64, v: f64) -> Result<EVec3, BrepError> {
        self.surface_normal_sync(surface, u, v)
    }
    fn curve_closest_parameter(&self, curve: &GeometryHandle, point: EVec3) -> Result<(f64, EVec3, f64), BrepError> {
        self.curve_closest_parameter_sync(curve, point)
    }
    fn surface_closest_uv(&self, surface: &GeometryHandle, point: EVec3) -> Result<(f64, f64, EVec3, f64), BrepError> {
        self.surface_closest_uv_sync(surface, point)
    }
    fn volume(&self, shape: &GeometryHandle) -> Result<f64, BrepError> {
        self.volume_sync(shape)
    }
    fn area(&self, shape: &GeometryHandle) -> Result<f64, BrepError> {
        self.area_sync(shape)
    }
    fn length(&self, shape: &GeometryHandle) -> Result<f64, BrepError> {
        self.length_sync(shape)
    }
    fn center_of_mass(&self, shape: &GeometryHandle) -> Result<EVec3, BrepError> {
        self.center_of_mass_sync(shape)
    }
    fn bounding_box(&mut self, shape: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.bounding_box_sync(shape)
    }
    fn distance(&self, a: &GeometryHandle, b: &GeometryHandle) -> Result<f64, BrepError> {
        self.distance_sync(a, b)
    }
    fn closest_point(&self, shape: &GeometryHandle, point: EVec3) -> Result<ClosestPoint, BrepError> {
        self.closest_point_sync(shape, point)
    }
    fn classify_point(&self, solid: &GeometryHandle, point: EVec3) -> Result<PointClassification, BrepError> {
        self.classify_point_sync(solid, point)
    }
    fn validate(&self, shape: &GeometryHandle) -> Result<String, BrepError> {
        self.validate_sync(shape)
    }
    fn vertex(&mut self, point: EVec3) -> Result<GeometryHandle, BrepError> {
        self.vertex_sync(point)
    }
    fn face_from_wire(&mut self, wire: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.face_from_wire_sync(wire)
    }
    fn sew_faces(&mut self, faces: &[GeometryHandle], tolerance: f64) -> Result<GeometryHandle, BrepError> {
        self.sew_faces_sync(faces, tolerance)
    }
    fn heal_solid(&mut self, shape: &GeometryHandle, tolerance: f64) -> Result<GeometryHandle, BrepError> {
        self.heal_solid_sync(shape, tolerance)
    }
    fn convert_to_nurbs(&mut self, shape: &GeometryHandle) -> Result<GeometryHandle, BrepError> {
        self.convert_to_nurbs_sync(shape)
    }
    fn deconstruct(&mut self, shape: &GeometryHandle) -> Result<BrepTopology, BrepError> {
        self.deconstruct_sync(shape)
    }
    
    fn export_stl(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
        self.export_stl_sync(shapes, deflection)
    }
    fn export_obj(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<String, BrepError> {
        self.export_obj_sync(shapes, deflection)
    }
    fn export_gltf(&self, shapes: &[GeometryHandle], deflection: f64) -> Result<Vec<u8>, BrepError> {
        self.export_gltf_sync(shapes, deflection)
    }
    
    fn import_stl(&mut self, data: &[u8], tolerance: f64) -> Result<GeometryHandle, BrepError> {
        self.import_stl_sync(data, tolerance)
    }
    fn import_obj(&mut self, data: &str, tolerance: f64) -> Result<GeometryHandle, BrepError> {
        self.import_obj_sync(data, tolerance)
    }
    fn export_mesh(&self, shapes: &[GeometryHandle], deflection: f64, exporter: &dyn semio_framework_mesh_engine::io::MeshExporter) -> Result<Vec<u8>, BrepError> {
        self.export_mesh_sync(shapes, deflection, exporter)
    }
    fn import_mesh(&mut self, data: &[u8], tolerance: f64, importer: &dyn semio_framework_mesh_engine::io::MeshImporter) -> Result<GeometryHandle, BrepError> {
        self.import_mesh_sync(data, tolerance, importer)
    }
    fn kind(&self, handle: &GeometryHandle) -> Result<GeometryKind, BrepError> {
        self.kind_sync(handle)
    }
    fn tessellate(&self, handle: &GeometryHandle, tolerance: f64) -> Result<MeshTransfer, BrepError> {
        self.tessellate_sync(handle, tolerance)
    }
    fn dispose(&mut self, handle: &GeometryHandle) {
        let _ = self.dispose_sync(handle);
    }
    /// ♻️ Equivalent to disposing every handle not in `live`, then compacting once — see
    /// [`Brep::compact_unreachable`].
    fn retain(&mut self, live: &std::collections::HashSet<String>) {
        let mut changed=false;for slot in 0..self.live.slot_count() {if let Some(original)=self.live.extract_slot_if(slot,|key,_|live.contains(key)){drop(original);changed=true;}}if changed{self.advance_live_revision();}
        self.compact_unreachable();
    }
    fn registry_len(&self) -> usize {
        self.live.len()
    }
    fn solid_shells(&mut self, solid: &GeometryHandle) -> Result<Vec<GeometryHandle>, BrepError> {
        self.solid_shells_sync(solid)
    }
    fn compound(&mut self, solids: &[GeometryHandle]) -> Result<GeometryHandle, BrepError> {
        self.compound_sync(solids)
    }
    fn explode(&mut self, compound: &GeometryHandle) -> Result<Vec<GeometryHandle>, BrepError> {
        self.explode_sync(compound)
    }
    fn label(&self, handle: &GeometryHandle) -> Option<u64> {
        self.label_of(handle).map(|l| l.0)
    }
}
// #endregion 🔖️BrepKernelImpl

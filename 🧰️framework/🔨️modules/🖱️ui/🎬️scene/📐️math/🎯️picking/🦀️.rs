//! 🎯️ Screen picking: ray and marquee selection of instances, components and vertices over leased meshes.
//!
//! Carved out of the former `semio-framework-ui-scene` math module by ticket `26/08/11/CLEAN-ARCHITECTURE-LAYERING-ENFORCEMENT`.

use semio_framework_geometry::{Mat4, Vec3};
use semio_framework_ui_scene_algebra::{vec3_from_array_m, vec3_new_m, Mat4Math, Vec3Math};
use semio_framework_ui_scene_camera::projection_spec_project_point;
use semio_framework_ui_scene_mesh::{Instance3d, Mesh3dFault, Mesh3dField, Mesh3dLease};
use semio_framework_ui_scene_pass::{transform_aabb, SceneDraw3d};
use semio_framework_ui_viewport::Viewport3dProjectionSpec;

//#region Picking
pub fn ray_aabb_slab(origin: Vec3, dir: Vec3, min: [f32; 3], max: [f32; 3]) -> Option<f32> {
    let mut t_min = f32::NEG_INFINITY;
    let mut t_max = f32::INFINITY;
    let axes = [origin.x, origin.y, origin.z];
    let dirs = [dir.x, dir.y, dir.z];
    let bounds = [(min[0], max[0]), (min[1], max[1]), (min[2], max[2])];
    for axis in 0..3 {
        if dirs[axis].abs() < 1e-8 {
            if axes[axis] < bounds[axis].0 || axes[axis] > bounds[axis].1 {
                return None;
            }
            continue;
        }
        let inv = 1.0 / dirs[axis];
        let mut t0 = (bounds[axis].0 - axes[axis]) * inv;
        let mut t1 = (bounds[axis].1 - axes[axis]) * inv;
        if t0 > t1 {
            std::mem::swap(&mut t0, &mut t1);
        }
        t_min = t_min.max(t0);
        t_max = t_max.min(t1);
        if t_max < t_min {
            return None;
        }
    }
    if t_max < 0.0 {
        return None;
    }
    Some(if t_min >= 0.0 { t_min } else { t_max })
}


pub fn ray_pick_instance(origin: Vec3, dir: Vec3, mesh: Mesh3dLease, instance: &Instance3d) -> Result<Option<f32>,Mesh3dFault> {
    let (min,max)=mesh.aabb()?;
    let (world_min,world_max)=transform_aabb(instance.model,min,max);
    if ray_aabb_slab(origin,dir,world_min,world_max).is_none(){return Ok(None);}
    let mut best=None;
    let triangles=mesh.schema()?.indices/3;
    for triangle in 0..triangles{
        let first=triangle.checked_mul(3).ok_or(Mesh3dFault::Schema)?;
        let points=std::array::from_fn::<_,3,_>(|index|mesh.u32(Mesh3dField::Indices,first+index as u32).and_then(|index|mesh.vec3(Mesh3dField::Positions,index)).map(|point|instance.model.transform_point_m(vec3_new_m(point[0],point[1],point[2]))));
        let [a,b,c]=points;
        if let Some(distance)=ray_triangle(origin,dir,a?,b?,c?){best=Some(best.map_or(distance,|previous:f32|previous.min(distance)));}
    }
    Ok(best)
}

pub struct RayMeshHit {
    pub distance: f32,
    pub triangle_index: usize,
    pub bary_u: f32,
    pub bary_v: f32,
    pub point: Vec3,
    pub normal: Vec3,
}


pub fn ray_pick_mesh_detail(origin:Vec3,dir:Vec3,mesh:Mesh3dLease,instance:&Instance3d)->Result<Option<RayMeshHit>,Mesh3dFault>{
    let(min,max)=mesh.aabb()?;
    let(world_min,world_max)=transform_aabb(instance.model,min,max);
    if ray_aabb_slab(origin,dir,world_min,world_max).is_none(){return Ok(None);}
    let mut best:Option<RayMeshHit>=None;
    let triangles=mesh.schema()?.indices/3;
    for triangle in 0..triangles{
        let first=triangle.checked_mul(3).ok_or(Mesh3dFault::Schema)?;
        let points=std::array::from_fn::<_,3,_>(|index|mesh.u32(Mesh3dField::Indices,first+index as u32).and_then(|index|mesh.vec3(Mesh3dField::Positions,index)).map(|point|instance.model.transform_point_m(vec3_new_m(point[0],point[1],point[2]))));
        let[a,b,c]=points;let(a,b,c)=(a?,b?,c?);
        if let Some((distance,u,v))=ray_triangle_barycentric(origin,dir,a,b,c){
            if best.as_ref().is_none_or(|hit|distance<hit.distance){
                let point=origin.add_m(dir.scale_m(distance));
                let mut normal=b.sub_m(a).cross_m(c.sub_m(a));
                if normal.length_m()>1e-6{normal=normal.normalize_m();}
                best=Some(RayMeshHit{distance,triangle_index:triangle as usize,bary_u:u,bary_v:v,point,normal});
            }
        }
    }
    Ok(best)
}

fn ray_triangle_barycentric(origin: Vec3, dir: Vec3, a: Vec3, b: Vec3, c: Vec3) -> Option<(f32, f32, f32)> {
    let edge1 = b.sub_m(a);
    let edge2 = c.sub_m(a);
    let h = dir.cross_m(edge2);
    let det = edge1.dot_m(h);
    if det.abs() < 1e-8 {
        return None;
    }
    let f = 1.0 / det;
    let s = origin.sub_m(a);
    let u = f * s.dot_m(h);
    if !(0.0..=1.0).contains(&u) {
        return None;
    }
    let q = s.cross_m(edge1);
    let v = f * dir.dot_m(q);
    if v < 0.0 || u + v > 1.0 {
        return None;
    }
    let t = f * edge2.dot_m(q);
    if t > 1e-4 {
        Some((t, u, v))
    } else {
        None
    }
}


pub fn interpolate_mesh_uv(mesh:Mesh3dLease,triangle_index:usize,bary_u:f32,bary_v:f32)->Result<Option<(f32,f32)>,Mesh3dFault>{
    let schema=mesh.schema()?;
    if schema.uvs==0||triangle_index>=schema.indices as usize/3{return Ok(None);}
    let first=u32::try_from(triangle_index).ok().and_then(|triangle|triangle.checked_mul(3)).ok_or(Mesh3dFault::Schema)?;
    let coordinates=std::array::from_fn::<_,3,_>(|index|mesh.u32(Mesh3dField::Indices,first+index as u32).and_then(|index|mesh.vec2(Mesh3dField::Uvs,index)));
    let[a,b,c]=coordinates;let(a,b,c)=(a?,b?,c?);
    let weight=1.0-bary_u-bary_v;
    Ok(Some((a[0]*weight+b[0]*bary_u+c[0]*bary_v,a[1]*weight+b[1]*bary_u+c[1]*bary_v)))
}

pub const SELECTION_DRAG_DIRECTION_THRESHOLD_PX: f32 = 2.0;

pub fn marquee_is_crossing(start_x: f32, end_x: f32) -> bool {
    end_x < start_x
}

pub fn marquee_is_crossing_from_path(path: &[[f32; 2]], is_lasso: bool) -> bool {
    let Some(start) = path.first() else {
        return false;
    };
    if is_lasso {
        for point in path.iter().skip(1) {
            let dx = point[0] - start[0];
            if dx.abs() >= SELECTION_DRAG_DIRECTION_THRESHOLD_PX {
                return dx < 0.0;
            }
        }
    }
    let end = path.last().copied().unwrap_or(*start);
    marquee_is_crossing(start[0], end[0])
}

fn orient2d(a: [f32; 2], b: [f32; 2], c: [f32; 2]) -> f32 {
    (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])
}

fn point_on_segment(p: [f32; 2], a: [f32; 2], b: [f32; 2]) -> bool {
    p[0] >= a[0].min(b[0]) && p[0] <= a[0].max(b[0]) && p[1] >= a[1].min(b[1]) && p[1] <= a[1].max(b[1])
}

fn segments_intersect(a0: [f32; 2], a1: [f32; 2], b0: [f32; 2], b1: [f32; 2]) -> bool {
    let o1 = orient2d(a0, a1, b0);
    let o2 = orient2d(a0, a1, b1);
    let o3 = orient2d(b0, b1, a0);
    let o4 = orient2d(b0, b1, a1);
    if o1 == 0.0 && point_on_segment(b0, a0, a1) {
        return true;
    }
    if o2 == 0.0 && point_on_segment(b1, a0, a1) {
        return true;
    }
    if o3 == 0.0 && point_on_segment(a0, b0, b1) {
        return true;
    }
    if o4 == 0.0 && point_on_segment(a1, b0, b1) {
        return true;
    }
    (o1 > 0.0) != (o2 > 0.0) && (o3 > 0.0) != (o4 > 0.0)
}

fn rect_corners(rect: [f32; 4]) -> [[f32; 2]; 4] {
    let min_x = rect[0].min(rect[2]);
    let max_x = rect[0].max(rect[2]);
    let min_y = rect[1].min(rect[3]);
    let max_y = rect[1].max(rect[3]);
    [[min_x, min_y], [max_x, min_y], [max_x, max_y], [min_x, max_y]]
}

fn segment_intersects_rect(a: [f32; 2], b: [f32; 2], rect: [f32; 4]) -> bool {
    let corners = rect_corners(rect);
    for index in 0..corners.len() {
        let c0 = corners[index];
        let c1 = corners[(index + 1) % corners.len()];
        if segments_intersect(a, b, c0, c1) {
            return true;
        }
    }
    false
}

fn segment_intersects_polygon(a: [f32; 2], b: [f32; 2], polygon: &[[f32; 2]]) -> bool {
    if point_in_polygon(a, polygon) || point_in_polygon(b, polygon) {
        return true;
    }
    for index in 0..polygon.len() {
        let j = if index == 0 { polygon.len() - 1 } else { index - 1 };
        if segments_intersect(a, b, polygon[index], polygon[j]) {
            return true;
        }
    }
    false
}

fn marquee_contains_point(point: [f32; 2], polygon: &[[f32; 2]], rectangle: bool, rect_bounds: Option<[f32; 4]>) -> bool {
    if rectangle {
        rect_bounds.is_some_and(|bounds| rect_contains(bounds, point))
    } else {
        point_in_polygon(point, polygon)
    }
}

fn marquee_segment_selected(a: [f32; 2], b: [f32; 2], polygon: &[[f32; 2]], rectangle: bool, rect_bounds: Option<[f32; 4]>, crossing: bool) -> bool {
    if crossing {
        if marquee_contains_point(a, polygon, rectangle, rect_bounds) || marquee_contains_point(b, polygon, rectangle, rect_bounds) {
            return true;
        }
        if rectangle {
            rect_bounds.is_some_and(|bounds| segment_intersects_rect(a, b, bounds))
        } else {
            segment_intersects_polygon(a, b, polygon)
        }
    } else {
        marquee_contains_point(a, polygon, rectangle, rect_bounds) && marquee_contains_point(b, polygon, rectangle, rect_bounds)
    }
}

fn marquee_triangle_selected(points: &[[f32; 2]; 3], polygon: &[[f32; 2]], rectangle: bool, rect_bounds: Option<[f32; 4]>, crossing: bool) -> bool {
    if crossing {
        if points.iter().any(|point| marquee_contains_point(*point, polygon, rectangle, rect_bounds)) {
            return true;
        }
        for index in 0..3 {
            let next = (index + 1) % 3;
            if marquee_segment_selected(points[index], points[next], polygon, rectangle, rect_bounds, true) {
                return true;
            }
        }
        false
    } else {
        points.iter().all(|point| marquee_contains_point(*point, polygon, rectangle, rect_bounds))
    }
}

fn marquee_rect_bounds(polygon: &[[f32; 2]]) -> Option<[f32; 4]> {
    let first = polygon.first()?;
    let mut min_x = first[0];
    let mut max_x = first[0];
    let mut min_y = first[1];
    let mut max_y = first[1];
    for point in polygon.iter().skip(1) {
        min_x = min_x.min(point[0]);
        max_x = max_x.max(point[0]);
        min_y = min_y.min(point[1]);
        max_y = max_y.max(point[1]);
    }
    Some([min_x, min_y, max_x, max_y])
}

/// 🎯️ Screen-space component (vertex/edge/face) picking within a marquee/lasso polygon; kept as a flat argument
/// list rather than a params struct because both `infinite/world/rs` call sites pass through the same shape
/// positionally and this crate must not restructure a signature consumed outside its own scope.
#[allow(clippy::too_many_arguments, reason = "flat picking-context args match the two infinite/world/rs call sites; a params struct would be a cross-crate signature change out of this crate's scope")]
pub fn screen_select_components(
    mesh_lookup: &std::collections::HashMap<String, Mesh3dLease>,
    draws: &[SceneDraw3d],
    view_proj: Mat4,
    projection_spec: Viewport3dProjectionSpec,
    width: f32,
    height: f32,
    polygon: &[[f32; 2]],
    rectangle: bool,
    granularity: &str,
    active_instance_id: Option<&str>,
    crossing: bool,
) -> Vec<String> {
    use std::collections::HashSet;
    let mut selected = HashSet::new();
    let local_polygon: Vec<[f32; 2]> = polygon.iter().map(|point| [point[0], point[1]]).collect();
    let rect_bounds = marquee_rect_bounds(&local_polygon);
    for draw in draws {
        let Some(&mesh) = mesh_lookup.get(&draw.mesh_key) else {
            continue;
        };
        let Ok(schema) = mesh.schema() else { continue };
        for instance in &draw.instances {
            if active_instance_id.is_some_and(|active| instance.id != active) {
                continue;
            }
            match granularity {
                "vertex" if schema.vertex_ids != 0 => {
                    for vertex_index in 0..schema.vertices {
                        let group = mesh.u32(Mesh3dField::VertexIds, vertex_index).unwrap_or(vertex_index);
                        if group == u32::MAX { continue; }
                        let Ok(point) = mesh.vec3(Mesh3dField::Positions, vertex_index) else { continue };
                        let world = instance.model.transform_point_m(vec3_new_m(point[0], point[1], point[2]));
                        let Some(screen) = projection_spec_project_point(view_proj, projection_spec, world, width, height) else {
                            continue;
                        };
                        let point = [screen[0], screen[1]];
                        let inside = marquee_contains_point(point, &local_polygon, rectangle, rect_bounds);
                        if inside {
                            let id = group.to_string();
                            selected.insert(id);
                        }
                    }
                }
                "edge" if schema.edges != 0 => {
                    for edge_index in 0..schema.edges {
                        let Ok(edge) = mesh.edge(edge_index) else { continue };
                        let a_world = instance.model.transform_point_m(vec3_new_m(edge[0][0], edge[0][1], edge[0][2]));
                        let b_world = instance.model.transform_point_m(vec3_new_m(edge[1][0], edge[1][1], edge[1][2]));
                        let (Some(a_screen), Some(b_screen)) = (projection_spec_project_point(view_proj, projection_spec, a_world, width, height), projection_spec_project_point(view_proj, projection_spec, b_world, width, height)) else {
                            continue;
                        };
                        if !marquee_segment_selected(a_screen, b_screen, &local_polygon, rectangle, rect_bounds, crossing) {
                            continue;
                        }
                        let id = mesh.u32(Mesh3dField::EdgeIds, edge_index).unwrap_or(edge_index).to_string();
                        selected.insert(id);
                    }
                }
                "face" => {
                    for tri_index in 0..schema.indices / 3 {
                        let Some(tri) = mesh_triangle(mesh, tri_index) else { continue };
                        let mut screens = [[0.0_f32; 2]; 3];
                        let mut visible = 0usize;
                        for (slot, index) in tri.iter().enumerate() {
                            let Some(point) = vertex(mesh, *index) else { continue };
                            let world = instance.model.transform_point_m(point);
                            if let Some(screen) = projection_spec_project_point(view_proj, projection_spec, world, width, height) {
                                screens[slot] = screen;
                                visible += 1;
                            }
                        }
                        if visible < 3 {
                            continue;
                        }
                        if !marquee_triangle_selected(&screens, &local_polygon, rectangle, rect_bounds, crossing) {
                            continue;
                        }
                        let id = mesh.u32(Mesh3dField::FaceIds, tri_index).unwrap_or(tri_index).to_string();
                        selected.insert(id);
                    }
                }
                _ => {
                    let Ok((min, max)) = mesh.aabb() else { continue };
                    let Some(projected) = projected_aabb_bounds(view_proj, projection_spec, instance.model, min, max, width, height) else {
                        continue;
                    };
                    if aabb_overlaps_marquee(projected, &local_polygon, rectangle) {
                        selected.insert(instance.id.clone());
                    }
                }
            }
        }
    }
    selected.into_iter().collect()
}

fn vertex(mesh: Mesh3dLease, index: u32) -> Option<Vec3> {
    let point = mesh.vec3(Mesh3dField::Positions, index).ok()?;
    Some(vec3_new_m(point[0], point[1], point[2]))
}

fn mesh_triangle(mesh: Mesh3dLease, triangle: u32) -> Option<[u32; 3]> {
    let base = triangle.checked_mul(3)?;
    Some([mesh.u32(Mesh3dField::Indices, base).ok()?, mesh.u32(Mesh3dField::Indices, base + 1).ok()?, mesh.u32(Mesh3dField::Indices, base + 2).ok()?])
}

pub fn ray_triangle(origin: Vec3, dir: Vec3, a: Vec3, b: Vec3, c: Vec3) -> Option<f32> {
    let edge1 = b.sub_m(a);
    let edge2 = c.sub_m(a);
    let h = dir.cross_m(edge2);
    let det = edge1.dot_m(h);
    if det.abs() < 1e-8 {
        return None;
    }
    let f = 1.0 / det;
    let s = origin.sub_m(a);
    let u = f * s.dot_m(h);
    if !(0.0..=1.0).contains(&u) {
        return None;
    }
    let q = s.cross_m(edge1);
    let v = f * dir.dot_m(q);
    if v < 0.0 || u + v > 1.0 {
        return None;
    }
    let t = f * edge2.dot_m(q);
    if t > 1e-4 {
        Some(t)
    } else {
        None
    }
}

pub fn project_point(view_proj: Mat4, point: Vec3, width: f32, height: f32) -> Option<[f32; 2]> {
    let clip = view_proj.transform_point_m(point);
    if clip.z < 0.0 || clip.z > 1.0 {
        return None;
    }
    Some([(clip.x * 0.5 + 0.5) * width, (1.0 - (clip.y * 0.5 + 0.5)) * height])
}

pub fn screen_segment_distance(px: f32, py: f32, ax: f32, ay: f32, bx: f32, by: f32) -> f32 {
    let abx = bx - ax;
    let aby = by - ay;
    let len_sq = abx * abx + aby * aby;
    if len_sq < 1e-6 {
        let dx = px - ax;
        let dy = py - ay;
        return (dx * dx + dy * dy).sqrt();
    }
    let t = ((px - ax) * abx + (py - ay) * aby) / len_sq;
    let t = t.clamp(0.0, 1.0);
    let cx = ax + abx * t;
    let cy = ay + aby * t;
    let dx = px - cx;
    let dy = py - cy;
    (dx * dx + dy * dy).sqrt()
}

pub fn point_in_polygon(point: [f32; 2], polygon: &[[f32; 2]]) -> bool {
    if polygon.len() < 3 {
        return false;
    }
    let mut winding = 0;
    for index in 0..polygon.len() {
        let a = polygon[index];
        let b = polygon[(index + 1) % polygon.len()];
        if a[1] <= point[1] {
            if b[1] > point[1] && cross2(a, b, point) > 0.0 {
                winding += 1;
            }
        } else if b[1] <= point[1] && cross2(a, b, point) < 0.0 {
            winding -= 1;
        }
    }
    winding != 0
}

fn cross2(a: [f32; 2], b: [f32; 2], c: [f32; 2]) -> f32 {
    (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0])
}

pub fn rect_contains(rect: [f32; 4], point: [f32; 2]) -> bool {
    let min_x = rect[0].min(rect[2]);
    let max_x = rect[0].max(rect[2]);
    let min_y = rect[1].min(rect[3]);
    let max_y = rect[1].max(rect[3]);
    point[0] >= min_x && point[0] <= max_x && point[1] >= min_y && point[1] <= max_y
}

pub fn projected_aabb_bounds(view_proj: Mat4, projection_spec: Viewport3dProjectionSpec, model: Mat4, min: [f32; 3], max: [f32; 3], width: f32, height: f32) -> Option<[f32; 4]> {
    let corners = [[min[0], min[1], min[2]], [max[0], min[1], min[2]], [min[0], max[1], min[2]], [max[0], max[1], min[2]], [min[0], min[1], max[2]], [max[0], min[1], max[2]], [min[0], max[1], max[2]], [max[0], max[1], max[2]]];
    let mut min_x = f32::INFINITY;
    let mut min_y = f32::INFINITY;
    let mut max_x = f32::NEG_INFINITY;
    let mut max_y = f32::NEG_INFINITY;
    let mut visible = false;
    for corner in corners {
        let world = model.transform_point_m(vec3_from_array_m(corner));
        if let Some(screen) = projection_spec_project_point(view_proj, projection_spec, world, width, height) {
            visible = true;
            min_x = min_x.min(screen[0]);
            min_y = min_y.min(screen[1]);
            max_x = max_x.max(screen[0]);
            max_y = max_y.max(screen[1]);
        }
    }
    visible.then_some([min_x, min_y, max_x, max_y])
}

fn aabb_overlaps_marquee(projected: [f32; 4], polygon: &[[f32; 2]], rectangle: bool) -> bool {
    if rectangle {
        let Some(marquee) = marquee_rect_bounds(polygon) else {
            return false;
        };
        let marquee_min_x = marquee[0].min(marquee[2]);
        let marquee_max_x = marquee[0].max(marquee[2]);
        let marquee_min_y = marquee[1].min(marquee[3]);
        let marquee_max_y = marquee[1].max(marquee[3]);
        return projected[2] >= marquee_min_x && projected[0] <= marquee_max_x && projected[3] >= marquee_min_y && projected[1] <= marquee_max_y;
    }
    let corners = [[projected[0], projected[1]], [projected[2], projected[1]], [projected[2], projected[3]], [projected[0], projected[3]]];
    corners.iter().any(|corner| point_in_polygon(*corner, polygon)) || polygon.iter().any(|point| point[0] >= projected[0] && point[0] <= projected[2] && point[1] >= projected[1] && point[1] <= projected[3])
}

/// 🎯️ Screen-space whole-instance picking within a marquee/lasso polygon; kept as a flat argument list rather
/// than a params struct for the same cross-crate-scope reason as `screen_select_components`.
#[allow(clippy::too_many_arguments, reason = "flat picking-context args match the two infinite/world/rs call sites; a params struct would be a cross-crate signature change out of this crate's scope")]
pub fn screen_select_instances(mesh_lookup: &std::collections::HashMap<String, Mesh3dLease>, draws: &[SceneDraw3d], view_proj: Mat4, projection_spec: Viewport3dProjectionSpec, width: f32, height: f32, polygon: &[[f32; 2]], rectangle: bool, crossing: bool) -> Vec<String> {
    let rect_bounds = marquee_rect_bounds(polygon);
    let mut selected = Vec::new();
    for draw in draws {
        let Some(&mesh) = mesh_lookup.get(&draw.mesh_key) else {
            continue;
        };
        let Ok(schema) = mesh.schema() else { continue };
        for instance in &draw.instances {
            if !crossing {
                let mut all_inside = true;
                let mut any_visible = false;
                for vertex_index in 0..schema.vertices {
                    let Ok(point) = mesh.vec3(Mesh3dField::Positions, vertex_index) else { continue };
                    let world = instance.model.transform_point_m(vec3_new_m(point[0], point[1], point[2]));
                    if let Some(screen) = projection_spec_project_point(view_proj, projection_spec, world, width, height) {
                        any_visible = true;
                        if !marquee_contains_point(screen, polygon, rectangle, rect_bounds) {
                            all_inside = false;
                            break;
                        }
                    }
                }
                if any_visible && all_inside {
                    selected.push(instance.id.clone());
                }
                continue;
            }
            let Ok((min, max)) = mesh.aabb() else { continue };
            let Some(projected) = projected_aabb_bounds(view_proj, projection_spec, instance.model, min, max, width, height) else {
                continue;
            };
            if !aabb_overlaps_marquee(projected, polygon, rectangle) {
                continue;
            }
            let mut covered = false;
            for triangle in 0..schema.indices / 3 {
                let Some(tri) = mesh_triangle(mesh, triangle) else { continue };
                let mut screens = [[0.0_f32; 2]; 3];
                let mut visible = 0usize;
                for (slot, &index) in tri.iter().enumerate() {
                    let Some(point) = vertex(mesh, index) else { continue };
                    let world = instance.model.transform_point_m(point);
                    if let Some(screen) = projection_spec_project_point(view_proj, projection_spec, world, width, height) {
                        screens[slot] = screen;
                        visible += 1;
                    }
                }
                if visible < 3 {
                    continue;
                }
                if marquee_triangle_selected(&screens, polygon, rectangle, rect_bounds, true) {
                    covered = true;
                    break;
                }
            }
            if covered {
                selected.push(instance.id.clone());
            }
        }
    }
    selected
}
//#endregion Picking

#[cfg(test)]
#[path = "../../🧪️tests/🔬️picking-unit/🦀️.rs"]
mod tests;

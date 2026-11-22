//! 📐️ Structural DWG drawing model of `stdio.dwg` (ac1024/✳️any): layers, entities and their geometry, plus
//! the pure mesh, path and SVG projections. The reader produces it, the writer and the snapshot consume it.

//#region 🔖️DwgStructuralCodec
// 📐 Relocated verbatim (ticket 26/08/12/DISSOLVE-KERNELS-AND-MODULES-INTO-EVENT-SOURCED-
// ARTIFACTS G2) from `🧰️framework/🔨️modules/🔺️mesh/🦀️.rs`, a framework module that
// despite its name held zero mesh code — a complete, self-contained, hand-rolled DWG binary
// codec (AC1015/R2000 file magic, its own section-locator/CRC/handle container; bit primitives
// BS/BL/BD/handle refs per https://www.opendesign.com/files/guestdownloads/OpenDesign_Specification_for_.dwg_files.pdf).
// Entity/header field layouts are a semio-defined subset chosen for lossless round-tripping
// through THIS codec; byte-exact third-party AutoCAD/ODA interop needs follow-up validation
// against a real DWG viewer — it is NOT the same pipeline as this file's own D1/D2 real
// R2004+ decoder above, which walks genuine AutoCAD-authored files at the section/page level
// without ever reaching entity bitcode. Kept here, not folded into `DwgSnapshot`, because it has
// zero `DwgSnapshot` dependency of its own — pure byte<->structured-value functions, per this
// same file's own `R2004FileHeaderDecrypt`/`Lz77Variant` precedent ("kept here per ... rule 6").
//#region DwgTypes
/// 📐️ Hand-rolled DWG codec: a self-contained, round-trippable binary interchange format using the AC1015 (R2000) file magic and an R2000-flavored section-locator/CRC/handle container (bit primitives BS/BL/BD/handle refs per https://www.opendesign.com/files/guestdownloads/OpenDesign_Specification_for_.dwg_files.pdf). Entity/header field layouts are a semio-defined subset chosen for lossless round-tripping through this codec; byte-exact third-party AutoCAD/ODA interop needs follow-up validation against a real DWG viewer.
#[derive(Clone, Debug, PartialEq, Default)]
pub struct DwgDrawing {
    pub layers: Vec<DwgLayer>,
    pub entities: Vec<DwgEntity>,
    pub extmin: [f64; 3],
    pub extmax: [f64; 3],
}
#[derive(Clone, Debug, PartialEq)]
pub struct DwgLayer {
    pub name: String,
    pub color: u8,
}
impl Default for DwgLayer {
    fn default() -> Self {
        Self { name: "0".to_string(), color: 7 }
    }
}
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum DwgColor {
    ByLayer,
    ByBlock,
    Index(u8),
}
#[derive(Clone, Debug, PartialEq)]
pub struct DwgEntity {
    pub layer: usize,
    pub color: DwgColor,
    pub geometry: DwgGeometry,
}
#[derive(Clone, Debug, PartialEq)]
pub enum DwgGeometry {
    Line { start: [f64; 3], end: [f64; 3] },
    Point { at: [f64; 3] },
    Circle { center: [f64; 3], radius: f64, normal: [f64; 3] },
    Arc { center: [f64; 3], radius: f64, start_angle: f64, end_angle: f64, normal: [f64; 3] },
    Ellipse { center: [f64; 3], major_axis: [f64; 3], ratio: f64, start_param: f64, end_param: f64, normal: [f64; 3] },
    LwPolyline { closed: bool, elevation: f64, vertices: Vec<[f64; 2]>, bulges: Vec<f64> },
    Spline { degree: u32, control_points: Vec<[f64; 3]>, knots: Vec<f64>, weights: Vec<f64> },
    Text { at: [f64; 3], height: f64, rotation: f64, content: String },
    Face3d { corners: [[f64; 3]; 4] },
    Polyline3d { closed: bool, vertices: Vec<[f64; 3]> },
    PolyfaceMesh { vertices: Vec<[f64; 3]>, faces: Vec<[i32; 4]> },
}
impl DwgDrawing {
    pub fn ensure_layer(&mut self, name: &str) -> usize {
        if let Some(index) = self.layers.iter().position(|layer| layer.name == name) {
            return index;
        }
        self.layers.push(DwgLayer { name: name.to_string(), color: 7 });
        self.layers.len() - 1
    }

    /// 📐️ Recomputes the drawing extents from every entity; the reader calls it after decoding.
    pub fn recompute_extents(&mut self) {
        let mut extents = DwgDrawingExtents::default();
        for entity in &self.entities {
            extents.include(&entity.geometry);
        }
        extents.apply(self);
    }
}

/// 📏️ Accumulates drawing extents without rescanning already projected geometry.
pub struct DwgDrawingExtents {
    min: [f64; 3],
    max: [f64; 3],
}

impl Default for DwgDrawingExtents {
    fn default() -> Self { Self { min: [f64::INFINITY; 3], max: [f64::NEG_INFINITY; 3] } }
}

impl DwgDrawingExtents {
    /// 📍️ Expands the bounds with one projected drawing position.
    pub fn include_point(&mut self, point: [f64; 3]) {
        for (axis, coordinate) in point.into_iter().enumerate() {
            self.min[axis] = self.min[axis].min(coordinate);
            self.max[axis] = self.max[axis].max(coordinate);
        }
    }

    pub fn include(&mut self, geometry: &DwgGeometry) {
        let mut touch = |point| self.include_point(point);
        match geometry {
            DwgGeometry::Line { start, end } => {
                touch(*start);
                touch(*end);
            }
            DwgGeometry::Point { at } => touch(*at),
            DwgGeometry::Circle { center, radius, .. } | DwgGeometry::Arc { center, radius, .. } => {
                touch([center[0] - radius, center[1] - radius, center[2]]);
                touch([center[0] + radius, center[1] + radius, center[2]]);
            }
            DwgGeometry::Ellipse { center, major_axis, .. } => {
                let r = (major_axis[0] * major_axis[0] + major_axis[1] * major_axis[1]).sqrt();
                touch([center[0] - r, center[1] - r, center[2]]);
                touch([center[0] + r, center[1] + r, center[2]]);
            }
            DwgGeometry::LwPolyline { vertices, elevation, .. } => {
                for v in vertices {
                    touch([v[0], v[1], *elevation]);
                }
            }
            DwgGeometry::Spline { control_points, .. } | DwgGeometry::Polyline3d { vertices: control_points, .. } => {
                for p in control_points {
                    touch(*p);
                }
            }
            DwgGeometry::PolyfaceMesh { vertices, .. } => {
                for p in vertices {
                    touch(*p);
                }
            }
            DwgGeometry::Text { at, .. } => touch(*at),
            DwgGeometry::Face3d { corners } => {
                for p in corners {
                    touch(*p);
                }
            }
        }
    }

    pub fn apply(&self, drawing: &mut DwgDrawing) {
        if self.min[0].is_finite() {
            drawing.extmin = self.min;
            drawing.extmax = self.max;
        }
    }
}
//#endregion DwgTypes
//#region DwgMeshBridge
/// 🔺️ Wraps mesh data as a single polyface-mesh drawing. Framework-`MeshData`-shaped (not
/// `DwgSnapshot`-shaped) — kept here, not folded into a `DwgSnapshot` field, for the same "pure
/// byte/structural conversion, zero snapshot dependency" reason as the rest of this region. Still
/// has real framework-layer callers (`🧊️3d/📐️brep/📦️mesh-io`, the `os`/`host` products' OS-media
/// DWG mesh handlers) that cannot depend on this plugin crate — see this wave's report for the
/// resulting cross-layer gate on deleting the old `🔺️mesh` module outright.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn mesh_to_dwg_drawing(mesh: &semio_framework_mesh_engine::MeshData) -> Result<DwgDrawing, String> {
    let (positions, position_tail) = mesh.positions.as_chunks::<3>();
    let (triangles, index_tail) = mesh.indices.as_chunks::<3>();
    if !position_tail.is_empty() || !index_tail.is_empty() {
        return Err("DWG mesh requires complete positions and triangles".into());
    }
    if !mesh.positions.iter().all(|value| value.is_finite()) {
        return Err("DWG mesh requires finite positions".into());
    }
    if mesh.indices.iter().any(|&index| index as usize >= positions.len()) {
        return Err("DWG triangle references a vertex outside the source mesh".into());
    }

    let mut drawing = DwgDrawing::default();
    let layer = drawing.ensure_layer("0");
    let mut emit = |vertices, faces| drawing.entities.push(DwgEntity {
        layer, color: DwgColor::ByLayer, geometry: DwgGeometry::PolyfaceMesh { vertices, faces },
    });
    let vertex_limit = i16::MAX as usize;
    let face_limit = u16::MAX as usize;
    if positions.len() <= vertex_limit && triangles.len() <= face_limit {
        let vertices = positions.iter().map(|position| position.map(f64::from)).collect();
        let faces = triangles.iter().map(|triangle| {
            let [a,b,c] = triangle.map(|index| index as i32 + 1);
            [a,b,c,0]
        }).collect();
        emit(vertices, faces);
    } else {
        let mut vertices = Vec::new();
        let mut faces = Vec::new();
        let mut local = std::collections::HashMap::new();
        let mut used = vec![false; positions.len()];
        for triangle in triangles {
            let additional = triangle.iter().enumerate()
                .filter(|(slot, index)| !local.contains_key(*index) && !triangle[..*slot].contains(*index)).count();
            if vertices.len() + additional > vertex_limit || faces.len() == face_limit {
                emit(std::mem::take(&mut vertices), std::mem::take(&mut faces));
                local.clear();
            }
            let [a,b,c] = triangle.map(|index| *local.entry(index).or_insert_with(|| {
                let index = index as usize;
                used[index] = true;
                vertices.push(positions[index].map(f64::from));
                vertices.len() as i32
            }));
            faces.push([a,b,c,0]);
        }
        for (position, used) in positions.iter().zip(used) {
            if used { continue; }
            if vertices.len() == vertex_limit {
                emit(std::mem::take(&mut vertices), std::mem::take(&mut faces));
            }
            vertices.push(position.map(f64::from));
        }
        if !vertices.is_empty() {
            emit(vertices, faces);
        }
    }
    drawing.recompute_extents();
    Ok(drawing)
}
/// 🔺️ Collects polyface-mesh and 3dface entities into mesh data.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dwg_drawing_to_mesh(drawing: &DwgDrawing) -> Result<semio_framework_mesh_engine::MeshData, String> {
    dwg_entities_to_mesh(&drawing.entities)
}

/// 🧱️ Projects selected drawing entities with checked positions and face indices.
pub fn dwg_entities_to_mesh<'a>(entities: impl IntoIterator<Item = &'a DwgEntity>) -> Result<semio_framework_mesh_engine::MeshData, String> {
    let mut projection = DwgMeshProjection::new(entities.into_iter());
    loop {
        if let DwgMeshProjectionStep::Complete = projection.step()? {
            return projection.take_mesh().ok_or_else(|| "DWG mesh projection is missing its result".into());
        }
    }
}

/// 🔖️ One completed projection unit before the finished mesh can transfer.
pub enum DwgMeshProjectionStep {
    Entity,
    Position,
    Face,
    Normal,
    Complete,
}

enum DwgMeshProjectionPhase {
    Entities,
    Normals,
    Complete,
    Failed(String),
}

/// 🏗️ Converts original drawing entities to a triangle mesh without publishing partial geometry.
pub struct DwgMeshProjection<Entities: Iterator> {
    entities: Entities,
    current: Option<Entities::Item>,
    phase: DwgMeshProjectionPhase,
    mesh: Option<semio_framework_mesh_engine::MeshData>,
    entity_total: usize,
    entities_done: usize,
    vertex: usize,
    face: usize,
    base: u32,
    vertex_count: u32,
    normal: usize,
}

impl<Entities: Iterator> DwgMeshProjection<Entities>
where
    Entities::Item: std::borrow::Borrow<DwgEntity>,
{
    /// 🧺️ Holds either the original entity sequence or a borrowed sequence selected by the caller.
    pub fn new(entities: Entities) -> Self {
        let entity_total = entities.size_hint().1.unwrap_or(0);
        Self {
            entities, current: None, phase: DwgMeshProjectionPhase::Entities,
            mesh: Some(Default::default()), entity_total, entities_done: 0, vertex: 0, face: 0, base: 0, vertex_count: 0, normal: 0,
        }
    }

    /// 📈️ Reports progress through entity geometry and final vertex-normal normalization.
    pub fn progress(&self) -> f32 {
        match self.phase {
            DwgMeshProjectionPhase::Entities => {
                let current = self.current.as_ref().map_or(0.0, |entity| {
                    let (positions, faces) = dwg_mesh_rows(&std::borrow::Borrow::<DwgEntity>::borrow(entity).geometry);
                    (self.vertex + self.face) as f32 / (positions.len() + faces.len()).max(1) as f32
                });
                0.75 * ((self.entities_done as f32 + current) / self.entity_total.max(1) as f32).min(1.0)
            }
            DwgMeshProjectionPhase::Normals => {
                let total = self.mesh.as_ref().map_or(0, |mesh| mesh.vertex_count());
                0.75 + 0.25 * (self.normal as f32 / total.max(1) as f32)
            }
            DwgMeshProjectionPhase::Complete => 1.0,
            DwgMeshProjectionPhase::Failed(_) => 0.0,
        }
    }

    /// ⏭️ Advances one entity, one coordinate triplet, one face, or one normal.
    pub fn step(&mut self) -> Result<DwgMeshProjectionStep, String> {
        let result = match &self.phase {
            DwgMeshProjectionPhase::Entities => self.advance_entity(),
            DwgMeshProjectionPhase::Normals => Ok(self.advance_normal()),
            DwgMeshProjectionPhase::Complete => Ok(DwgMeshProjectionStep::Complete),
            DwgMeshProjectionPhase::Failed(error) => return Err(error.clone()),
        };
        if let Err(error) = &result {
            self.phase = DwgMeshProjectionPhase::Failed(error.clone());
        }
        result
    }

    /// 📤️ Transfers the validated mesh after all position, face and normal rows are complete.
    pub fn take_mesh(&mut self) -> Option<semio_framework_mesh_engine::MeshData> {
        if matches!(self.phase, DwgMeshProjectionPhase::Complete) { self.mesh.take() } else { None }
    }

    fn advance_entity(&mut self) -> Result<DwgMeshProjectionStep, String> {
        let mesh = self.mesh.as_mut().expect("pending DWG mesh projection owns its output");
        if let Some(entity) = self.current.as_ref() {
            let (vertices, faces) = dwg_mesh_rows(&std::borrow::Borrow::<DwgEntity>::borrow(entity).geometry);
            if let Some(vertex) = vertices.get(self.vertex) {
                let coordinates = vertex.map(|value| value as f32);
                if !coordinates.iter().all(|coordinate| coordinate.is_finite()) {
                    return Err("DWG mesh position is outside the finite mesh coordinate range".into());
                }
                mesh.positions.try_reserve(3).map_err(|_| "DWG mesh position allocation failed")?;
                mesh.normals.try_reserve(3).map_err(|_| "DWG mesh normal allocation failed")?;
                mesh.positions.extend_from_slice(&coordinates);
                mesh.normals.extend_from_slice(&[0.0; 3]);
                self.vertex += 1;
                return Ok(DwgMeshProjectionStep::Position);
            }
            if let Some(face) = faces.get(self.face) {
                append_dwg_face(mesh, self.base, self.vertex_count, *face)?;
                self.face += 1;
                return Ok(DwgMeshProjectionStep::Face);
            }
            self.current = None;
            self.entities_done += 1;
        }
        let Some(entity) = self.entities.next() else {
            self.phase = DwgMeshProjectionPhase::Normals;
            return Ok(self.advance_normal());
        };
        let (vertices, _) = dwg_mesh_rows(&std::borrow::Borrow::<DwgEntity>::borrow(&entity).geometry);
        self.base = u32::try_from(mesh.vertex_count()).map_err(|_| "DWG mesh has too many vertices")?;
        self.vertex_count = u32::try_from(vertices.len()).map_err(|_| "DWG mesh has too many vertices")?;
        self.base.checked_add(self.vertex_count).ok_or("DWG mesh has too many vertices")?;
        self.current = Some(entity);
        self.vertex = 0;
        self.face = 0;
        Ok(DwgMeshProjectionStep::Entity)
    }

    fn advance_normal(&mut self) -> DwgMeshProjectionStep {
        let mesh = self.mesh.as_mut().expect("normal projection owns its output");
        if self.normal == mesh.vertex_count() {
            self.phase = DwgMeshProjectionPhase::Complete;
            return DwgMeshProjectionStep::Complete;
        }
        mesh.normalize_vertex_normal(self.normal);
        self.normal += 1;
        DwgMeshProjectionStep::Normal
    }
}

/// 🧾️ Returns source positions and local face records for mesh-producing entities.
pub fn dwg_mesh_rows(geometry: &DwgGeometry) -> (&[[f64; 3]], &[[i32; 4]]) {
    match geometry {
        DwgGeometry::PolyfaceMesh { vertices, faces } => (vertices, faces),
        DwgGeometry::Face3d { corners } if corners[3] == corners[2] => (&corners[..3], &[[1, 2, 3, 0]]),
        DwgGeometry::Face3d { corners } => (corners, &[[1, 2, 3, 4]]),
        _ => (&[], &[]),
    }
}

/// 🔺️ Validates local DWG indices and triangulates one triangle or quadrilateral.
pub fn dwg_face_triangles(base: u32, vertex_count: u32, face: [i32; 4]) -> Result<([u32; 3], Option<[u32; 3]>), String> {
    let mut corners = face.map(i32::unsigned_abs);
    let count = if corners[3] == 0 || corners[2] == corners[3] { 3 } else { 4 };
    for index in &mut corners[..count] {
        if *index == 0 || *index > vertex_count {
            return Err(format!("DWG polyface vertex index {index} exceeds the local range 1..={vertex_count}"));
        }
        *index = base.checked_add(*index - 1).ok_or("DWG mesh has too many vertices")?;
    }
    let [a, b, c, d] = corners;
    Ok(([a, b, c], (count == 4).then_some([a, c, d])))
}

fn append_dwg_face(mesh: &mut semio_framework_mesh_engine::MeshData, base: u32, vertex_count: u32, face: [i32; 4]) -> Result<(), String> {
    let (first, second) = dwg_face_triangles(base, vertex_count, face)?;
    mesh.indices.try_reserve(if second.is_some() { 6 } else { 3 }).map_err(|_| "DWG mesh index allocation failed")?;
    mesh.indices.extend_from_slice(&first);
    mesh.accumulate_triangle_normal(first);
    if let Some(second) = second {
        mesh.indices.extend_from_slice(&second);
        mesh.accumulate_triangle_normal(second);
    }
    Ok(())
}
//#endregion DwgMeshBridge
//#region DwgPathBridge
/// ✏️ Path segment mirror of the 2d kernel's PathSegment (kernel/2d/engine/rs/lib.rs), kept dependency-free.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum DwgPathSegment {
    Move { to: [f64; 2] },
    Line { to: [f64; 2] },
    Quad { ctrl: [f64; 2], to: [f64; 2] },
    Cubic { ctrl1: [f64; 2], ctrl2: [f64; 2], to: [f64; 2] },
    Arc { rx: f64, ry: f64, rotation: f64, large_arc: bool, sweep: bool, to: [f64; 2] },
    Close,
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn arc_bulge(from: [f64; 2], to: [f64; 2], radius: f64, sweep: bool) -> f64 {
    let dx = to[0] - from[0];
    let dy = to[1] - from[1];
    let chord = (dx * dx + dy * dy).sqrt();
    if chord < 1e-9 || radius < 1e-9 {
        return 0.0;
    }
    let included_angle = 2.0 * (chord * 0.5 / radius).clamp(-1.0, 1.0).asin();
    let bulge = (included_angle / 4.0).tan();
    if sweep {
        bulge
    } else {
        -bulge
    }
}
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn bulge_to_segment(from: [f64; 2], to: [f64; 2], bulge: f64) -> DwgPathSegment {
    if bulge.abs() < 1e-9 {
        return DwgPathSegment::Line { to };
    }
    let dx = to[0] - from[0];
    let dy = to[1] - from[1];
    let chord = (dx * dx + dy * dy).sqrt();
    let included_angle = 4.0 * bulge.atan();
    let denom = (2.0 * (included_angle / 2.0).sin()).abs();
    let radius = if denom > 1e-9 { chord / denom } else { 0.0 };
    DwgPathSegment::Arc { rx: radius, ry: radius, rotation: 0.0, large_arc: included_angle.abs() > std::f64::consts::PI, sweep: bulge > 0.0, to }
}
/// ✏️ Converts flattened path segments to dwg entities: line/close runs to lwpolylines with bulge arcs, curves to splines.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn paths_to_dwg_drawing(paths: &[Vec<DwgPathSegment>]) -> DwgDrawing {
    let mut drawing = DwgDrawing::default();
    let layer = drawing.ensure_layer("0");
    for path in paths {
        let mut vertices: Vec<[f64; 2]> = Vec::new();
        let mut bulges: Vec<f64> = Vec::new();
        let mut closed = false;
        let mut cursor = [0.0, 0.0];
        let mut start = [0.0, 0.0];
        for segment in path {
            match segment {
                DwgPathSegment::Move { to } => {
                    if !vertices.is_empty() {
                        drawing.entities.push(DwgEntity { layer, color: DwgColor::ByLayer, geometry: DwgGeometry::LwPolyline { closed, elevation: 0.0, vertices: vertices.clone(), bulges: bulges.clone() } });
                        vertices.clear();
                        bulges.clear();
                        closed = false;
                    }
                    vertices.push(*to);
                    bulges.push(0.0);
                    cursor = *to;
                    start = *to;
                }
                DwgPathSegment::Line { to } => {
                    vertices.push(*to);
                    bulges.push(0.0);
                    cursor = *to;
                }
                DwgPathSegment::Quad { ctrl, to } => {
                    let c1 = [cursor[0] + 2.0 / 3.0 * (ctrl[0] - cursor[0]), cursor[1] + 2.0 / 3.0 * (ctrl[1] - cursor[1])];
                    let c2 = [to[0] + 2.0 / 3.0 * (ctrl[0] - to[0]), to[1] + 2.0 / 3.0 * (ctrl[1] - to[1])];
                    let spline_points = [cursor, c1, c2, *to];
                    drawing.entities.push(DwgEntity {
                        layer,
                        color: DwgColor::ByLayer,
                        geometry: DwgGeometry::Spline { degree: 3, control_points: spline_points.iter().map(|p| [p[0], p[1], 0.0]).collect(), knots: vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], weights: vec![1.0; 4] },
                    });
                    cursor = *to;
                }
                DwgPathSegment::Cubic { ctrl1, ctrl2, to } => {
                    let spline_points = [cursor, *ctrl1, *ctrl2, *to];
                    drawing.entities.push(DwgEntity {
                        layer,
                        color: DwgColor::ByLayer,
                        geometry: DwgGeometry::Spline { degree: 3, control_points: spline_points.iter().map(|p| [p[0], p[1], 0.0]).collect(), knots: vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], weights: vec![1.0; 4] },
                    });
                    cursor = *to;
                }
                DwgPathSegment::Arc { rx, sweep, to, .. } => {
                    let bulge = arc_bulge(cursor, *to, *rx, *sweep);
                    if let Some(last) = bulges.last_mut() {
                        *last = bulge;
                    }
                    vertices.push(*to);
                    bulges.push(0.0);
                    cursor = *to;
                }
                DwgPathSegment::Close => {
                    closed = true;
                    cursor = start;
                }
            }
        }
        if !vertices.is_empty() {
            drawing.entities.push(DwgEntity { layer, color: DwgColor::ByLayer, geometry: DwgGeometry::LwPolyline { closed, elevation: 0.0, vertices, bulges } });
        }
    }
    drawing.recompute_extents();
    drawing
}
/// ✏️ Converts one entity's geometry to its path-segment form, when that geometry kind has one —
/// extracted out of `dwg_drawing_to_paths`'s own per-entity loop body (ticket 26/08/12/DISSOLVE-
/// KERNELS-AND-MODULES-INTO-EVENT-SOURCED-ARTIFACTS G2) so the new `✳️drawing` semio-mesh DWG
/// bridge leaf can walk entities one at a time (keeping each path's originating layer/entity
/// index), something the original all-entities-flattened `Vec<Vec<DwgPathSegment>>` return shape
/// could not offer since non-path geometry kinds are silently skipped, desyncing any by-index zip
/// against `drawing.entities`.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dwg_geometry_to_path_segments(geometry: &DwgGeometry) -> Option<Vec<DwgPathSegment>> {
    match geometry {
        DwgGeometry::LwPolyline { closed, vertices, bulges, .. } => {
            if vertices.is_empty() {
                return None;
            }
            let mut segments = vec![DwgPathSegment::Move { to: vertices[0] }];
            for i in 1..vertices.len() {
                let from = vertices[i - 1];
                let to = vertices[i];
                let bulge = bulges.get(i - 1).copied().unwrap_or(0.0);
                segments.push(bulge_to_segment(from, to, bulge));
            }
            if *closed && vertices.len() > 1 {
                let bulge = bulges.last().copied().unwrap_or(0.0);
                segments.push(bulge_to_segment(vertices[vertices.len() - 1], vertices[0], bulge));
                segments.push(DwgPathSegment::Close);
            }
            Some(segments)
        }
        DwgGeometry::Spline { degree, control_points, .. } if *degree == 3 && control_points.len() == 4 => Some(vec![
            DwgPathSegment::Move { to: [control_points[0][0], control_points[0][1]] },
            DwgPathSegment::Cubic { ctrl1: [control_points[1][0], control_points[1][1]], ctrl2: [control_points[2][0], control_points[2][1]], to: [control_points[3][0], control_points[3][1]] },
        ]),
        DwgGeometry::Circle { center, radius, normal } => {
            let mirror = planar_ocs_mirror(normal)?;
            let at = |x: f64, y: f64| [mirror * x, y];
            Some(vec![
                DwgPathSegment::Move { to: at(center[0] + radius, center[1]) },
                DwgPathSegment::Arc { rx: *radius, ry: *radius, rotation: 0.0, large_arc: true, sweep: mirror > 0.0, to: at(center[0] - radius, center[1]) },
                DwgPathSegment::Arc { rx: *radius, ry: *radius, rotation: 0.0, large_arc: true, sweep: mirror > 0.0, to: at(center[0] + radius, center[1]) },
                DwgPathSegment::Close,
            ])
        }
        DwgGeometry::Line { start, end } => Some(vec![DwgPathSegment::Move { to: [start[0], start[1]] }, DwgPathSegment::Line { to: [end[0], end[1]] }]),
        DwgGeometry::Polyline3d { closed, vertices } => {
            let (first, rest) = vertices.split_first()?;
            let mut segments = vec![DwgPathSegment::Move { to: [first[0], first[1]] }];
            segments.extend(rest.iter().map(|v| DwgPathSegment::Line { to: [v[0], v[1]] }));
            if *closed {
                segments.push(DwgPathSegment::Close);
            }
            Some(segments)
        }
        DwgGeometry::Arc { center, radius, start_angle, end_angle, normal } => {
            let mirror = planar_ocs_mirror(normal)?;
            elliptic_arc_segments(mirror, [center[0], center[1]], [*radius, 0.0], 1.0, *start_angle, *end_angle)
        }
        DwgGeometry::Ellipse { center, major_axis, ratio, start_param, end_param, normal } => {
            let mirror = planar_ocs_mirror(normal)?;
            elliptic_arc_segments(mirror, [center[0], center[1]], [major_axis[0], major_axis[1]], *ratio, *start_param, *end_param)
        }
        DwgGeometry::Point { .. } | DwgGeometry::Spline { .. } | DwgGeometry::Text { .. } | DwgGeometry::Face3d { .. } | DwgGeometry::PolyfaceMesh { .. } => None,
    }
}
/// 🧭️ The x factor mapping an entity's object coordinate system onto the world XY plane (DWG's
/// arbitrary-axis algorithm): `1` for the `+Z` extrusion, `−1` for `−Z` (its OCS x axis is world −x);
/// `None` for any tilted plane, which a 2D path cannot carry.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn planar_ocs_mirror(normal: &[f64; 3]) -> Option<f64> {
    let length = (normal[0] * normal[0] + normal[1] * normal[1] + normal[2] * normal[2]).sqrt();
    if length < 1e-12 || (normal[0] / length).abs() > 1e-9 || (normal[1] / length).abs() > 1e-9 {
        return None;
    }
    Some(normal[2].signum())
}
/// 🌀️ The counter-clockwise (in OCS) elliptic arc from parameter `start` to `end` of the ellipse
/// `centre + major·cos t + ratio·perp(major)·sin t`, as SVG-style arc segments (a full turn is two
/// halves); a circular arc is the `ratio = 1` case with angles as parameters.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
fn elliptic_arc_segments(mirror: f64, centre: [f64; 2], major: [f64; 2], ratio: f64, start: f64, end: f64) -> Option<Vec<DwgPathSegment>> {
    let rx = (major[0] * major[0] + major[1] * major[1]).sqrt();
    if rx < 1e-12 || ratio <= 0.0 {
        return None;
    }
    let tau = std::f64::consts::TAU;
    let sweep = (end - start).rem_euclid(tau);
    let sweep = if sweep < 1e-12 { tau } else { sweep };
    let point = |t: f64| [mirror * (centre[0] + major[0] * t.cos() - ratio * major[1] * t.sin()), centre[1] + major[1] * t.cos() + ratio * major[0] * t.sin()];
    let rotation = mirror * major[1].atan2(major[0]).to_degrees();
    let arc = |to: f64, span: f64| DwgPathSegment::Arc { rx, ry: ratio * rx, rotation, large_arc: span > std::f64::consts::PI, sweep: mirror > 0.0, to: point(to) };
    let mut segments = vec![DwgPathSegment::Move { to: point(start) }];
    if sweep > std::f64::consts::PI {
        segments.push(arc(start + sweep / 2.0, sweep / 2.0));
        segments.push(arc(start + sweep, sweep / 2.0));
    } else {
        segments.push(arc(start + sweep, sweep));
    }
    if (sweep - tau).abs() < 1e-12 {
        segments.push(DwgPathSegment::Close);
    }
    Some(segments)
}
/// ✏️ Converts drawing entities back to path segments, one path per entity.
// 🚫️async: E1 pure codec/computation helper (file verified I/O-free, consumed via Fn-bound combinator/Display) — see R9
pub fn dwg_drawing_to_paths(drawing: &DwgDrawing) -> Vec<Vec<DwgPathSegment>> {
    drawing.entities.iter().filter_map(|entity| dwg_geometry_to_path_segments(&entity.geometry)).collect()
}
//#endregion DwgPathBridge
//#endregion 🔖️DwgStructuralCodec
/// 📐️ Renders a DWG drawing back to flat SVG markup (lines and closed polygons), for the raster import path.
pub fn dwg_drawing_to_svg(drawing: &DwgDrawing) -> Result<(String, u32, u32), String> {
    let width = (drawing.extmax[0] - drawing.extmin[0]).max(1.0).ceil() as u32;
    let height = (drawing.extmax[1] - drawing.extmin[1]).max(1.0).ceil() as u32;
    let mut paths = String::new();
    for entity in &drawing.entities {
        if let DwgGeometry::LwPolyline { vertices, closed, .. } = &entity.geometry {
            if vertices.is_empty() {
                continue;
            }
            let mut d = format!("M {} {}", vertices[0][0] - drawing.extmin[0], drawing.extmax[1] - vertices[0][1]);
            for v in &vertices[1..] {
                d.push_str(&format!(" L {} {}", v[0] - drawing.extmin[0], drawing.extmax[1] - v[1]));
            }
            if *closed {
                d.push_str(" Z");
            }
            paths.push_str(&format!("<path d=\"{d}\" fill=\"none\" stroke=\"black\" stroke-width=\"1\"/>"));
        }
    }
    let svg = format!("<svg xmlns=\"http://www.w3.org/2000/svg\" width=\"{width}\" height=\"{height}\" viewBox=\"0 0 {width} {height}\">{paths}</svg>");
    Ok((svg, width, height))
}

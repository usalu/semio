//! 📐 Schema-neutral IFC representation projection in source length units.
#[path = "📖️graph/🦀️.rs"]
pub mod graph;
#[path="📏️units/🦀️.rs"]
pub mod units;
use graph::{GeometryEntity,GeometryGraph,GeometryStandard,GeometryValue};
#[path = "🧮️matrix/🦀️.rs"]
mod matrix;
#[path="🧱️primitive/🦀️.rs"]mod primitive;
#[path="📦️product/🦀️.rs"]mod product;
#[path="🧵️prepare/🦀️.rs"]pub mod preparation;
use matrix::Mat4;
use std::collections::{HashMap, HashSet};
#[path="🧊️boolean/🦀️.rs"]
pub mod boolean;
#[path="⭕️circle/🦀️.rs"]
pub mod circle;
#[path="🔲️convex/🦀️.rs"]
pub mod convex;

use semio_s_artifact_stdio_ifc_geometry_rings as rings;

type Point = [f64; 3];
pub const CURVE_RELATIVE_TOLERANCE: f64 = 0.001;
pub const CURVE_SEGMENTS: usize = 72;
type ProjectionResult<T> = Result<T, String>;

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GeometryMesh {
    pub product: u64,
    pub representation: u64,
    pub label: String,
    pub vertices: Vec<Point>,
    pub triangles: Vec<[u32; 3]>,
}

#[derive(Clone, Debug, PartialEq)]
pub struct GeometryIssue {
    pub entity: u64,
    pub reason: String,
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum GeometryStatus {
    #[default]
    Complete,
    Cancelled,
    Limited,
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum GeometryControl {
    Continue,
    Cancel,
    Limit,
}

#[derive(Clone, Debug, Default, PartialEq)]
pub struct GeometryScene {
    pub meshes: Vec<GeometryMesh>,
    pub issues: Vec<GeometryIssue>,
    pub status: GeometryStatus,
    pub work: usize,
}

#[derive(Clone, Copy, Debug)]
pub struct GeometryLimits {
    pub work: usize,
    pub vertices: usize,
    pub triangles: usize,
    pub depth: usize,
}

impl Default for GeometryLimits {
    fn default() -> Self {
        Self { work: 8_000_000, vertices: 200_000, triangles: 400_000, depth: 64 }
    }
}

#[derive(Clone, Copy, Debug)]
pub struct GeometryProgress {
    pub work: usize,
    pub budget: usize,
}

impl GeometryScene {
    /// 📷️ Refuses public mesh values that cannot produce finite indexed payloads and a camera.
    pub fn payload_issue(&self) -> Option<GeometryIssue> {
        if self.status != GeometryStatus::Complete { return None; }
        for mesh in &self.meshes {
            if mesh.vertices.iter().flatten().any(|value| !value.is_finite()) { return Some(GeometryIssue { entity: mesh.product, reason: "nonfinite-mesh-coordinate".into() }); }
            if mesh.triangles.iter().any(|triangle| triangle.iter().any(|&index| index as usize >= mesh.vertices.len()) || triangle[0] == triangle[1] || triangle[1] == triangle[2] || triangle[2] == triangle[0]) { return Some(GeometryIssue { entity: mesh.product, reason: "invalid-mesh-index".into() }); }
        }
        if payload_camera(self.bounds().unwrap_or([[0.; 3]; 2])).is_none() { return Some(GeometryIssue { entity: 0, reason: "unrepresentable-camera-range".into() }); }
        for mesh in &self.meshes {
            let origin = mesh.vertices.first().copied().unwrap_or([0.; 3]);
            let extent = mesh.vertices.iter().flat_map(|point| subtract(*point, origin)).map(f64::abs).fold(0., f64::max);
            for triangle in &mesh.triangles {
                let [a, b, c] = triangle.map(|index| mesh.vertices[index as usize]);
                let normal = cross(subtract(b, a).map(|value| value / extent), subtract(c, a).map(|value| value / extent));
                if normal.iter().any(|value| !value.is_finite()) || normal.iter().all(|value| *value == 0.) { return Some(GeometryIssue { entity: mesh.product, reason: "unrepresentable-mesh-normal".into() }); }
            }
        }
        None
    }
    pub fn meshes_json(&self) -> String {
        if self.status!=GeometryStatus::Complete || self.payload_issue().is_some(){return "[]".into();}
        let meshes: Vec<semio_framework_pack_json::Value> = self.meshes.iter().map(|mesh| {
            let mut normals = vec![[0.; 3]; mesh.vertices.len()];
            let origin = mesh.vertices.first().copied().unwrap_or([0.; 3]);
            let length = mesh.vertices.iter().flat_map(|point| subtract(*point, origin)).map(f64::abs).fold(0., f64::max);
            for triangle in &mesh.triangles {
                let a = mesh.vertices[triangle[0] as usize];
                let b = mesh.vertices[triangle[1] as usize];
                let c = mesh.vertices[triangle[2] as usize];
                let normal = cross(subtract(b, a).map(|value| value / length), subtract(c, a).map(|value| value / length));
                for &index in triangle { normals[index as usize] = add(normals[index as usize], normal); }
            }
            let positions: Vec<f64> = mesh.vertices.iter().flatten().copied().collect();
            let normals: Vec<f64> = normals.into_iter().flat_map(|normal| normalize(normal).unwrap_or([0.; 3])).collect();
            let indices: Vec<u32> = mesh.triangles.iter().flatten().copied().collect();
            semio_framework_pack_json::json!({ "id": format!("ifc-{}-{}", mesh.product, mesh.representation), "role": "solid", "data": { "positions": positions, "normals": normals, "indices": indices, "edgePositions": [] } })
        }).collect();
        semio_framework_pack_json::to_string(&semio_framework_pack_json::array(meshes))
    }

    pub fn instances_json(&self) -> String {
        if self.status!=GeometryStatus::Complete || self.payload_issue().is_some(){return "[]".into();}
        let instances: Vec<semio_framework_pack_json::Value> = self.meshes.iter().map(|mesh| semio_framework_pack_json::json!({
            "id": format!("ifc-{}-{}", mesh.product, mesh.representation), "interactionId": format!("ifc-{}", mesh.product),
            "meshId": format!("ifc-{}-{}", mesh.product, mesh.representation), "position": [0., 0., 0.],
            "rotation": [0., 0., 0., 1.], "scale": [1., 1., 1.], "label": mesh.label.clone(), "smoothShading": false
        })).collect();
        semio_framework_pack_json::to_string(&semio_framework_pack_json::array(instances))
    }

    pub fn camera(&self) -> (Point, Point) {
        let bounds = if self.status == GeometryStatus::Complete { self.bounds().unwrap_or([[0.; 3]; 2]) } else { [[0.; 3]; 2] };
        payload_camera(bounds).unwrap_or(([1.5, -1.5, 1.2], [0.; 3]))
    }

    pub fn bounds(&self) -> Option<[Point; 2]> {
        let mut points = self.meshes.iter().flat_map(|mesh| mesh.vertices.iter());
        let first = *points.next()?;
        let mut bounds = [first, first];
        for point in points { for axis in 0..3 {
            bounds[0][axis] = bounds[0][axis].min(point[axis]);
            bounds[1][axis] = bounds[1][axis].max(point[axis]);
        } }
        Some(bounds)
    }
}

fn payload_camera(bounds: [Point; 2]) -> Option<(Point, Point)> {
    let span = subtract(bounds[1], bounds[0]);
    if span.iter().any(|value| !value.is_finite()) { return None; }
    let target = std::array::from_fn(|axis| bounds[0][axis] + span[axis] / 2.);
    let extent = span.into_iter().fold(1., f64::max);
    let position: Point = std::array::from_fn(|axis| { let offset = [1.5, -1.5, 1.2][axis] * extent; let preferred = target[axis] + offset; if preferred.is_finite() { preferred } else { target[axis] - offset } });
    position.iter().chain(target.iter()).all(|value| value.is_finite()).then_some((position, target))
}

/// 🚦 Returns complete products only; callback false cancels and every graph, point and face step consumes budget.
pub fn project_with<G:GeometryGraph>(graph: &G, limits: GeometryLimits, progress: &mut dyn FnMut(GeometryProgress) -> bool) -> GeometryScene {
    project_controlled(graph, limits, &mut |value| if progress(value) { GeometryControl::Continue } else { GeometryControl::Cancel })
}

pub fn project_controlled<G:GeometryGraph>(graph: &G, limits: GeometryLimits, progress: &mut dyn FnMut(GeometryProgress) -> GeometryControl) -> GeometryScene {
    let mut projector = Projector { entities: HashMap::new(), limits, progress, scene: GeometryScene::default(), vertices: 0, triangles: 0, vertex_indices: HashMap::new(), openings:HashMap::new(), standard:graph.standard() };
    if projector.tick().is_err() { return projector.scene; }
    for index in 0..graph.len() {
        let Some(entity)=graph.entity(index) else {projector.scene.issues.push(GeometryIssue{entity:0,reason:"missing-graph-instance".into()});return projector.scene;};
        if projector.tick().is_err() { return projector.scene; }
        if projector.entities.insert(entity.id, entity).is_some() {
            projector.scene.issues.push(GeometryIssue { entity: entity.id, reason: "duplicate-entity-id".into() });
            return projector.scene;
        }
    }
    let mut excluded:HashMap<u64,String> = HashMap::new();
    for index in 0..graph.len() {
        let Some(relation)=graph.entity(index) else {continue;};
        if projector.tick().is_err() { return projector.scene; }
        if relation.name == "IFCRELVOIDSELEMENT" {
            let host=reference(relation,4);let opening=reference(relation,5);if let Some(host)=host{if relation.constituent_count!=1||opening.is_none(){excluded.insert(host,format!("invalid-opening-relation-#{}",relation.id));}else{let ids=projector.openings.entry(host).or_default();let id=opening.unwrap();if !ids.contains(&id){ids.push(id);}}}if let Some(id)=opening{excluded.insert(id,"opening-is-a-void-volume".into());}
        }
    }
    for index in 0..graph.len() {
        let Some(product)=graph.entity(index) else {continue;};
        if projector.tick().is_err() { break; }
        let Some(representation) = reference(product, 6) else { continue; };
        let shape = projector.entities.get(&representation).copied();
        let valid_shape = shape.is_some_and(|value| matches!(value.name, "IFCPRODUCTDEFINITIONSHAPE" | "IFCPRODUCTREPRESENTATION"));
        if !product_hint(graph.standard(),product) { continue; }
        if product.constituent_count!=1 {projector.scene.issues.push(GeometryIssue{entity:product.id,reason:"unsupported-complex-product".into()});continue;}
        let Some(shape) = shape else {
            projector.scene.issues.push(GeometryIssue { entity: product.id, reason: format!("missing-product-representation-#{representation}") });
            continue;
        };
        if !valid_shape {
            projector.scene.issues.push(GeometryIssue { entity: product.id, reason: format!("invalid-product-representation-{}-#{representation}", shape.name) });
            continue;
        }
        if let Some(reason) = excluded.get(&product.id).cloned().or_else(|| if matches!(product.name, "IFCOPENINGELEMENT" | "IFCOPENINGSTANDARDCASE") { Some("opening-is-a-void-volume".into()) } else { None }) {
            projector.scene.issues.push(GeometryIssue { entity: product.id, reason: reason.into() });
            continue;
        }
        let result = projector.product(product, shape);
        if let Err(reason) = result { projector.scene.issues.push(GeometryIssue { entity: product.id, reason }); }
        if projector.scene.status != GeometryStatus::Complete { break; }
    }
    projector.scene
}

pub fn project<G:GeometryGraph>(graph: &G) -> GeometryScene {
    project_with(graph, GeometryLimits::default(), &mut |_| true)
}

fn product_hint<V:GeometryValue>(standard:GeometryStandard,entity:GeometryEntity<'_,V>)->bool{product::is_geometry_product(standard,entity.name)}

struct Projector<'a, 'b,V:GeometryValue> {
    entities: HashMap<u64, GeometryEntity<'a,V>>,
    standard:GeometryStandard,
    openings:HashMap<u64,Vec<u64>>,
    limits: GeometryLimits,
    progress: &'b mut dyn FnMut(GeometryProgress) -> GeometryControl,
    scene: GeometryScene,
    vertices: usize,
    triangles: usize,
    vertex_indices: HashMap<[u64; 3], u32>,
}

impl<'a, 'b,V:GeometryValue> Projector<'a, 'b,V> {
    fn tick(&mut self) -> ProjectionResult<()> {
        if self.scene.work >= self.limits.work {
            self.scene.status = GeometryStatus::Limited;
            return Err("work-limit".into());
        }
        self.scene.work += 1;
        match (self.progress)(GeometryProgress { work: self.scene.work, budget: self.limits.work }) {
            GeometryControl::Continue => (),
            GeometryControl::Cancel => { self.scene.status = GeometryStatus::Cancelled; return Err("cancelled".into()); }
            GeometryControl::Limit => { self.scene.status = GeometryStatus::Limited; return Err("caller-work-limit".into()); }
        }
        Ok(())
    }

    fn entity(&mut self, id: u64) -> ProjectionResult<GeometryEntity<'a,V>> {
        self.tick()?;
        let entity=self.entities.get(&id).copied().ok_or_else(|| format!("missing-reference-#{id}"))?;
        if entity.constituent_count!=1{return Err(format!("unsupported-complex-representation-#{id}"));}
        Ok(entity)
    }

    fn references(&mut self, entity: GeometryEntity<'_,V>, index: usize) -> ProjectionResult<Vec<u64>> {
        let items = entity.args.get(index).and_then(V::as_aggregate).ok_or("missing-reference-list")?;
        let mut ids = Vec::new();
        for item in items {
            self.tick()?;
            ids.push(item.as_reference().ok_or("invalid-reference-list")?);
        }
        Ok(ids)
    }

    fn point(&mut self, id: u64, direction: bool) -> ProjectionResult<Point> {
        let entity = self.entity(id)?;
        if entity.name != if direction { "IFCDIRECTION" } else { "IFCCARTESIANPOINT" } { return Err(format!("invalid-point-#{id}")); }
        coordinates(entity.args.first().ok_or("missing-coordinates")?)
    }

    fn require_dimension(&mut self, id: u64, dimension: usize) -> ProjectionResult<()> {
        let entity = self.entity(id)?;
        if entity.args.first().and_then(V::as_aggregate).map(|values| values.len()) != Some(dimension) { return Err(format!("invalid-{dimension}d-coordinate-dimension")); }
        Ok(())
    }

    fn axis(&mut self, id: u64) -> ProjectionResult<Mat4> {
        let entity = self.entity(id)?;
        let name = entity.name.clone();
        let origin = reference(entity, 0).ok_or("missing-location")?;
        let a = optional_reference(entity, 1)?;
        let b = optional_reference(entity, 2)?;
        let origin = self.point(origin, false)?;
        match name {
            "IFCAXIS2PLACEMENT3D" => {
                self.require_dimension(reference(entity, 0).ok_or("missing-location")?, 3)?;
                if let Some(id) = a { self.require_dimension(id, 3)?; }
                if let Some(id) = b { self.require_dimension(id, 3)?; }
                if a.is_some() != b.is_some() { return Err("axis-and-ref-direction-provision".into()); }
                let z = normalize(if let Some(id) = a { self.point(id, true)? } else { [0., 0., 1.] })?;
                let hint = if let Some(id) = b { self.point(id, true)? } else if z[0].abs() < 0.9 { [1., 0., 0.] } else { [0., 1., 0.] };
                let x = normalize(subtract(hint, scale(z, dot(hint, z))))?;
                Ok(matrix(origin, x, cross(z, x), z))
            }
            "IFCAXIS2PLACEMENT2D" => {
                let location = self.entity(reference(entity, 0).ok_or("missing-location")?)?;
                if location.args.first().and_then(V::as_aggregate).map(|values| values.len()) != Some(2) { return Err("invalid-2d-coordinate-dimension".into()); }
                if let Some(id) = a {
                    let direction = self.entity(id)?;
                    if direction.args.first().and_then(V::as_aggregate).map(|values| values.len()) != Some(2) { return Err("invalid-2d-coordinate-dimension".into()); }
                }
                let x = normalize(if let Some(id) = a { self.point(id, true)? } else { [1., 0., 0.] })?;
                if x[2].abs() > 1e-9 { return Err("invalid-2d-axis".into()); }
                Ok(matrix(origin, x, [-x[1], x[0], 0.], [0., 0., 1.]))
            }
            _ => Err(format!("unsupported-placement-{name}")),
        }
    }

    fn placement(&mut self, id: u64, path: &mut HashSet<u64>) -> ProjectionResult<Mat4> {
        if path.len() >= self.limits.depth || !path.insert(id) { return Err("placement-cycle-or-depth".into()); }
        let entity = self.entity(id)?;
        if entity.name != "IFCLOCALPLACEMENT" { return Err(format!("unsupported-placement-{}", entity.name)); }
        let parent = reference(entity, 0);
        let axis = reference(entity, 1).ok_or("missing-relative-placement")?;
        let local = self.axis(axis)?;
        let result = if let Some(parent) = parent { self.placement(parent, path)?.mul(&local) } else { local };
        path.remove(&id);
        Ok(result)
    }

    fn product_meshes(&mut self, product: GeometryEntity<'_,V>, shape: GeometryEntity<'_,V>) -> ProjectionResult<Vec<GeometryMesh>> {
        if shape.constituent_count!=1{return Err(format!("unsupported-complex-representation-#{}",shape.id));}
        let placement = if let Some(id) = reference(product, 5) { self.placement(id, &mut HashSet::new())? } else { Mat4::identity() };
        let representations = self.references(shape, 2)?;
        let has_body = representations.iter().any(|id| self.entities.get(id).is_some_and(|entity| entity.args.get(1).and_then(V::as_str).is_some_and(|s| s.eq_ignore_ascii_case("Body"))));
        let mut meshes = Vec::new();
        for id in representations {
            let representation = self.entity(id)?;
            if !matches!(representation.name, "IFCSHAPEREPRESENTATION" | "IFCSHAPEMODEL") { return Err(format!("unsupported-product-representation-{}-#{id}", representation.name)); }
            let identifier = representation.args.get(1).and_then(V::as_str);
            if identifier.is_some_and(|value| !value.eq_ignore_ascii_case("Body")) || has_body && identifier.is_none() {
                self.scene.issues.push(GeometryIssue { entity: id, reason: "non-body-representation-skipped".into() });
                continue;
            }
            let items = self.references(representation, 3)?;
            let context = reference(representation, 0);
            let transform = if let Some(context) = context { self.context(context, 0)?.mul(&placement) } else { placement.clone() };
            self.vertex_indices.clear();
            let mut mesh = GeometryMesh { product: product.id, representation: id, label: product.args.get(2).and_then(V::as_str).unwrap_or(product.name).chars().take(256).collect(), ..GeometryMesh::default() };
            let mut path = HashSet::new();
            for item in items { self.item(item, &transform, &mut mesh, &mut path)?; }
            if !mesh.triangles.is_empty() { meshes.push(mesh); }
        }
        if meshes.is_empty() { return Err("no-supported-body-geometry".into()); }
        Ok(meshes)
    }

    fn product(&mut self,product:GeometryEntity<'_,V>,shape:GeometryEntity<'_,V>)->ProjectionResult<()>{
        let mut meshes=self.product_meshes(product,shape)?;
        for id in self.openings.get(&product.id).cloned().unwrap_or_default(){
            self.tick()?;let opening=self.entity(id)?;let shape=reference(opening,6).and_then(|representation|self.entities.get(&representation).copied()).filter(|shape|matches!(shape.name,"IFCPRODUCTDEFINITIONSHAPE"|"IFCPRODUCTREPRESENTATION")).ok_or_else(||format!("missing-opening-representation-#{id}"))?;
            let cutters:Vec<boolean::SolidMesh>=self.product_meshes(opening,shape).map_err(|reason|format!("opening-#{id}-{reason}"))?.into_iter().map(|mesh|boolean::SolidMesh{vertices:mesh.vertices,triangles:mesh.triangles}).collect();let mut next=Vec::new();
            for mut mesh in meshes{let mut result=boolean::SolidMesh{vertices:std::mem::take(&mut mesh.vertices),triangles:std::mem::take(&mut mesh.triangles)};for cutter in &cutters{self.tick()?;result=self.solid_operation(boolean::SolidBooleanOperation::Difference,&result,cutter)?;if result.vertices.len()>self.limits.vertices.saturating_sub(self.vertices)||result.triangles.len()>self.limits.triangles.saturating_sub(self.triangles){self.scene.status=GeometryStatus::Limited;return Err("boolean-output-limit".into());}self.vertices+=result.vertices.len();self.triangles+=result.triangles.len();}if !result.triangles.is_empty(){mesh.vertices=result.vertices;mesh.triangles=result.triangles;next.push(mesh);}}meshes=next;
        }
        self.scene.meshes.extend(meshes);Ok(())
    }

    fn context(&mut self, id: u64, depth: usize) -> ProjectionResult<Mat4> {
        if depth >= self.limits.depth { return Err("context-cycle-or-depth".into()); }
        let entity = self.entity(id)?;
        match entity.name {
            "IFCGEOMETRICREPRESENTATIONCONTEXT" => {
                let axis = reference(entity, 4);
                if let Some(axis) = axis { self.axis(axis) } else { Err("missing-context-placement".into()) }
            }
            "IFCGEOMETRICREPRESENTATIONSUBCONTEXT" => {
                let parent = reference(entity, 6).ok_or("missing-parent-context")?;
                self.context(parent, depth + 1)
            }
            _ => Err("unsupported-representation-context".into()),
        }
    }

    fn vertex(&mut self, mesh: &mut GeometryMesh, point: Point, transform: &Mat4) -> ProjectionResult<u32> {
        self.tick()?;
        let point = transform.transform_point(point);
        if !point.iter().all(|value| value.is_finite()) { return Err("nonfinite-vertex".into()); }
        let key = point.map(|value| if value == 0. { 0 } else { value.to_bits() });
        if let Some(&index) = self.vertex_indices.get(&key) { return Ok(index); }
        if self.vertices >= self.limits.vertices || mesh.vertices.len() >= u32::MAX as usize {
            self.scene.status = GeometryStatus::Limited;
            return Err("vertex-limit".into());
        }
        self.vertices += 1;
        let index = mesh.vertices.len() as u32;
        mesh.vertices.push(point);
        self.vertex_indices.insert(key, index);
        Ok(index)
    }

    fn triangle(&mut self, mesh: &mut GeometryMesh, triangle: [u32; 3]) -> ProjectionResult<()> {
        self.tick()?;
        if self.triangles >= self.limits.triangles {
            self.scene.status = GeometryStatus::Limited;
            return Err("triangle-limit".into());
        }
        if triangle.iter().any(|&index| index as usize >= mesh.vertices.len()) { return Err("invalid-triangle-index".into()); }
        self.triangles += 1;
        mesh.triangles.push(triangle);
        Ok(())
    }

    fn polygon(&mut self, mesh: &mut GeometryMesh, indices: &[u32]) -> ProjectionResult<()> {
        self.polygon_rings(mesh, &[indices.to_vec()])
    }

    fn polygon_rings(&mut self, mesh: &mut GeometryMesh, boundaries: &[Vec<u32>]) -> ProjectionResult<()> {
        let triangles = rings::triangulate_rings(&mesh.vertices, boundaries, &mut || self.tick())?;
        for triangle in triangles { self.triangle(mesh, triangle)?; }
        Ok(())
    }

    fn item(&mut self, id: u64, transform: &Mat4, mesh: &mut GeometryMesh, path: &mut HashSet<u64>) -> ProjectionResult<()> {
        if path.len() >= self.limits.depth || !path.insert(id) { return Err("representation-cycle-or-depth".into()); }
        let entity = self.entity(id)?;
        let name = entity.name.clone();
        let first_triangle = mesh.triangles.len();
        match name {
            "IFCTRIANGULATEDFACESET" | "IFCPOLYGONALFACESET" => self.face_set(id, transform, mesh)?,
            "IFCEXTRUDEDAREASOLID" => self.extrusion(id, transform, mesh)?,
            "IFCBOOLEANRESULT"=>self.boolean_result(entity,transform,mesh,path)?,
            "IFCBOOLEANCLIPPINGRESULT"=>self.clipping_result(entity,transform,mesh,path)?,
            "IFCBLOCK"=>self.block(entity,transform,mesh)?,
            "IFCCSGSOLID"=>{if entity.args.len()!=1{return Err("invalid-csg-argument-count".into());}let root=self.entity(reference(entity,0).ok_or("missing-csg-root")?)?;if !matches!(root.name,"IFCBOOLEANRESULT"|"IFCBOOLEANCLIPPINGRESULT"|"IFCBLOCK"){return Err(format!("unsupported-csg-root-{}",root.name));}self.item(root.id,transform,mesh,path)?;},
            "IFCFACETEDBREP" => {
                let shell = reference(entity, 0).ok_or("missing-shell")?;
                self.item(shell, transform, mesh, path)?;
            }
            "IFCCLOSEDSHELL" | "IFCOPENSHELL" | "IFCCONNECTEDFACESET" | "IFCSHELLBASEDSURFACEMODEL" | "IFCFACEBASEDSURFACEMODEL" => {
                let children = self.references(entity, 0)?;
                for child in children { self.item(child, transform, mesh, path)?; }
            }
            "IFCFACE" => self.face(id, transform, mesh)?,
            "IFCMAPPEDITEM" => {
                let source = reference(entity, 0).ok_or("missing-mapping-source")?;
                let target = reference(entity, 1).ok_or("missing-mapping-target")?;
                let source = self.entity(source)?;
                if source.name != "IFCREPRESENTATIONMAP" { return Err("invalid-representation-map".into()); }
                let origin = reference(source, 0).ok_or("missing-mapping-origin")?;
                let representation = reference(source, 1).ok_or("missing-mapped-representation")?;
                let origin = self.axis(origin)?;
                let mapped = transform.mul(&self.operator(target)?).mul(&origin);
                self.item(representation, &mapped, mesh, path)?;
            }
            "IFCSHAPEREPRESENTATION" => {
                let items = self.references(entity, 3)?;
                for item in items { self.item(item, transform, mesh, path)?; }
            }
            _ => return Err(format!("unsupported-representation-{name}-#{id}")),
        }
        if matches!(name, "IFCTRIANGULATEDFACESET" | "IFCPOLYGONALFACESET" | "IFCFACE" | "IFCEXTRUDEDAREASOLID" | "IFCBLOCK") && determinant(transform) < 0. {
            for triangle in &mut mesh.triangles[first_triangle..] { triangle.swap(1, 2); }
        }
        path.remove(&id);
        Ok(())
    }

    fn append_solid(&mut self,mesh:&mut GeometryMesh,result:boolean::SolidMesh)->ProjectionResult<()>{self.vertex_indices.clear();for(index,point)in mesh.vertices.iter().enumerate(){self.tick()?;self.vertex_indices.insert(point.map(|v|if v==0.{0}else{v.to_bits()}),index as u32);}let mut remap=Vec::new();for point in result.vertices{remap.push(self.vertex(mesh,point,&Mat4::identity())?);}for triangle in result.triangles{self.triangle(mesh,triangle.map(|i|remap[i as usize]))?;}Ok(())}

    fn clipping_result(&mut self,entity:GeometryEntity<'a,V>,transform:&Mat4,mesh:&mut GeometryMesh,path:&mut HashSet<u64>)->ProjectionResult<()>{
        if entity.args.len()!=3||entity.args.first().and_then(V::as_enum)!=Some("DIFFERENCE"){return Err("clipping-requires-difference".into());}let first=self.entity(reference(entity,1).ok_or("missing-clipping-first-operand")?)?;if !matches!(first.name,"IFCEXTRUDEDAREASOLID"|"IFCBOOLEANCLIPPINGRESULT"){return Err(format!("unsupported-clipping-first-operand-{}",first.name));}let half=self.entity(reference(entity,2).ok_or("missing-clipping-halfspace")?)?;if half.name!="IFCHALFSPACESOLID"{return Err(format!("unsupported-halfspace-{}",half.name));}if half.args.len()!=2{return Err("invalid-halfspace-argument-count".into());}let agreement=half.args.get(1).and_then(V::as_enum);if !matches!(agreement,Some("T"|"F")){return Err("invalid-halfspace-agreement".into());}let surface=self.entity(reference(half,0).ok_or("missing-halfspace-surface")?)?;if surface.name!="IFCPLANE"||surface.args.len()!=1{return Err(format!("unsupported-halfspace-surface-{}",surface.name));}let position=self.entity(reference(surface,0).ok_or("missing-plane-position")?)?;if position.name!="IFCAXIS2PLACEMENT3D"{return Err("plane-placement-must-be-3d".into());}let world=transform.mul(&self.axis(position.id)?);let mut columns=[[0.;3];3];for j in 0..3{let point=[world.0[0][j],world.0[1][j],world.0[2][j]];let extent=point.into_iter().map(f64::abs).fold(0.,f64::max);if extent<=0.||!extent.is_finite(){return Err("invalid-plane-transform".into());}columns[j]=point.map(|v|v/extent);}let mut normal=normalize(cross(columns[0],columns[1]))?;let side=dot(normal,columns[2]);if !side.is_finite()||side==0.{return Err("singular-plane-transform".into());}if(side<0.)!=(agreement==Some("T")){normal=scale(normal,-1.);}let plane=boolean::SolidPlane{origin:world.transform_point([0.;3]),normal};let mut temporary=GeometryMesh{product:mesh.product,representation:mesh.representation,label:mesh.label.clone(),..GeometryMesh::default()};self.vertex_indices.clear();self.item(first.id,transform,&mut temporary,path)?;let left=boolean::SolidMesh{vertices:temporary.vertices,triangles:temporary.triangles};let limits=boolean::SolidBooleanLimits{work:self.limits.work.saturating_sub(self.scene.work).max(1),polygons:self.limits.triangles.min(200_000).max(1),..boolean::SolidBooleanLimits::default()};let result=boolean::subtract_half_space(&left,&plane,limits,&mut ||self.tick());if let Err(reason)=&result{if matches!(reason.as_str(),"boolean-work-limit"|"boolean-polygon-limit"|"boolean-depth-limit"){self.scene.status=GeometryStatus::Limited;}}self.append_solid(mesh,result?)
    }

    fn solid_operation(&mut self,operation:boolean::SolidBooleanOperation,left:&boolean::SolidMesh,right:&boolean::SolidMesh)->ProjectionResult<boolean::SolidMesh>{let limits=boolean::SolidBooleanLimits{work:self.limits.work.saturating_sub(self.scene.work).max(1),polygons:self.limits.triangles.min(200_000).max(1),..boolean::SolidBooleanLimits::default()};let result=boolean::boolean_solid(operation,left,right,limits,&mut ||self.tick());if let Err(reason)=&result{if matches!(reason.as_str(),"boolean-work-limit"|"boolean-polygon-limit"|"boolean-depth-limit"){self.scene.status=GeometryStatus::Limited;}}result}

    fn block(&mut self,entity:GeometryEntity<'a,V>,transform:&Mat4,mesh:&mut GeometryMesh)->ProjectionResult<()>{
        if entity.args.len()!=4{return Err("invalid-block-argument-count".into());}let position=self.entity(reference(entity,0).ok_or("missing-block-position")?)?;if position.name!="IFCAXIS2PLACEMENT3D"{return Err("block-placement-must-be-3d".into());}let(x,y,z)=(required_real(entity,1)?,required_real(entity,2)?,required_real(entity,3)?);if x<=0.||y<=0.||z<=0.{return Err("invalid-block-dimensions".into());}let transform=transform.mul(&self.axis(position.id)?);let mut indices=Vec::new();for point in (0..8).map(|index|primitive::block_point([x,y,z],index)){indices.push(self.vertex(mesh,point,&transform)?);}for triangle in primitive::BLOCK_TRIANGLES{self.triangle(mesh,triangle.map(|i|indices[i as usize]))?;}Ok(())
    }

    fn boolean_result(&mut self,entity:GeometryEntity<'a,V>,transform:&Mat4,mesh:&mut GeometryMesh,path:&mut HashSet<u64>)->ProjectionResult<()>{
        if entity.args.len()!=3{return Err("invalid-boolean-argument-count".into());}let operation=match entity.args.first().and_then(V::as_enum){Some("UNION")=>boolean::SolidBooleanOperation::Union,Some("INTERSECTION")=>boolean::SolidBooleanOperation::Intersection,Some("DIFFERENCE")=>boolean::SolidBooleanOperation::Difference,_=>return Err("unsupported-boolean-operation".into())};
        let mut operands=Vec::new();for index in [1,2]{let id=reference(entity,index).ok_or("missing-boolean-operand")?;let operand=self.entity(id)?;if !matches!(operand.name,"IFCBOOLEANRESULT"|"IFCBOOLEANCLIPPINGRESULT"|"IFCCSGSOLID"|"IFCBLOCK"|"IFCEXTRUDEDAREASOLID"|"IFCFACETEDBREP"|"IFCTRIANGULATEDFACESET"|"IFCPOLYGONALFACESET"){return Err(format!("unsupported-boolean-operand-{}",operand.name));}if operand.name=="IFCFACETEDBREP"&&self.entity(reference(operand,0).ok_or("missing-shell")?)?.name!="IFCCLOSEDSHELL"{return Err("boolean-brep-requires-closed-shell".into());}if matches!(operand.name,"IFCTRIANGULATEDFACESET"|"IFCPOLYGONALFACESET")&&(self.standard!=GeometryStandard::Ifc4||operand.args.get(if operand.name=="IFCTRIANGULATEDFACESET"{2}else{1}).and_then(V::as_enum)!=Some("T")){return Err("boolean-tessellation-must-be-closed".into());}let mut temporary=GeometryMesh{product:mesh.product,representation:mesh.representation,label:mesh.label.clone(),..GeometryMesh::default()};self.vertex_indices.clear();self.item(id,transform,&mut temporary,path)?;operands.push(boolean::SolidMesh{vertices:temporary.vertices,triangles:temporary.triangles});}
        let result=self.solid_operation(operation,&operands[0],&operands[1])?;self.append_solid(mesh,result)
    }

    fn operator(&mut self, id: u64) -> ProjectionResult<Mat4> {
        let entity = self.entity(id)?;
        let name = entity.name.clone();
        if !matches!(name, "IFCCARTESIANTRANSFORMATIONOPERATOR3D" | "IFCCARTESIANTRANSFORMATIONOPERATOR3DNONUNIFORM") { return Err(format!("unsupported-mapping-{name}")); }
        let axes = [optional_reference(entity, 0)?, optional_reference(entity, 1)?, optional_reference(entity, 4)?];
        let location = reference(entity, 2).ok_or("missing-mapping-location")?;
        self.require_dimension(location, 3)?;
        for id in axes.into_iter().flatten() { self.require_dimension(id, 3)?; }
        let s = optional_real(entity, 3, 1.)?;
        let sy = if name.ends_with("NONUNIFORM") { optional_real(entity, 5, s)? } else { s };
        let sz = if name.ends_with("NONUNIFORM") { optional_real(entity, 6, s)? } else { s };
        if s <= 0. || sy <= 0. || sz <= 0. { return Err("invalid-mapping-scale".into()); }
        let z = normalize(if let Some(id) = axes[2] { self.point(id, true)? } else { [0., 0., 1.] })?;
        let hint = if let Some(id) = axes[0] { self.point(id, true)? } else if z == [1., 0., 0.] { [0., 1., 0.] } else { [1., 0., 0.] };
        let x = normalize(subtract(hint, scale(z, dot(hint, z))))?;
        let hint = if let Some(id) = axes[1] { self.point(id, true)? } else { [0., 1., 0.] };
        let y = normalize(subtract(subtract(hint, scale(z, dot(hint, z))), scale(x, dot(hint, x))))?;
        Ok(matrix(self.point(location, false)?, scale(x, s), scale(y, sy), scale(z, sz)))
    }

    fn face_set(&mut self, id: u64, transform: &Mat4, mesh: &mut GeometryMesh) -> ProjectionResult<()> {
        if self.standard!=GeometryStandard::Ifc4{return Err("representation-requires-ifc4".into());}
        let entity = self.entity(id)?;
        let triangulated = entity.name == "IFCTRIANGULATEDFACESET";
        let coords = reference(entity, 0).ok_or("missing-point-list")?;
        let faces = entity.args.get(if triangulated { 3 } else { 2 }).and_then(V::as_aggregate).ok_or("missing-face-indices")?;
        let pn = entity.args.get(if triangulated { 4 } else { 3 }).filter(|value| !value.is_absent());
        let point_list = self.entity(coords)?;
        if point_list.name != "IFCCARTESIANPOINTLIST3D" { return Err("invalid-point-list".into()); }
        let points = point_list.args.first().and_then(V::as_aggregate).ok_or("missing-point-list-coordinates")?;
        let mut vertices = HashMap::new();
        for face in faces {
            self.tick()?;
            let boundaries = if triangulated { vec![face.as_aggregate().ok_or("invalid-face-indices")?] } else {
                let indexed = self.entity(face.as_reference().ok_or("invalid-indexed-face")?)?;
                if !matches!(indexed.name, "IFCINDEXEDPOLYGONALFACE" | "IFCINDEXEDPOLYGONALFACEWITHVOIDS") { return Err(format!("unsupported-indexed-face-{}", indexed.name)); }
                let mut boundaries = vec![indexed.args.first().and_then(V::as_aggregate).ok_or("missing-polygon-indices")?];
                if indexed.name == "IFCINDEXEDPOLYGONALFACEWITHVOIDS" {
                    for hole in indexed.args.get(1).and_then(V::as_aggregate).ok_or("missing-polygon-inner-indices")? {
                        self.tick()?; boundaries.push(hole.as_aggregate().ok_or("invalid-polygon-inner-indices")?);
                    }
                }
                boundaries
            };
            let mut boundaries_indices = Vec::new();
            for values in boundaries {
                let mut indices = Vec::new();
                for value in values {
                    self.tick()?;
                    let mut index = index_value(value)?;
                    if let Some(pn) = pn { index = index_value(pn.as_aggregate().and_then(|list| list.get(index)).ok_or("invalid-pn-index")?)?; }
                    if index >= points.len() { return Err("face-index-out-of-range".into()); }
                    let vertex = if let Some(&vertex) = vertices.get(&index) { vertex } else {
                        let vertex = self.vertex(mesh, coordinates(&points[index])?, transform)?;
                        vertices.insert(index, vertex); vertex
                    };
                    indices.push(vertex);
                }
                boundaries_indices.push(indices);
            }
            if triangulated {
                let indices = &boundaries_indices[0];
                if indices.len() != 3 { return Err("triangle-needs-three-indices".into()); }
                self.triangle(mesh, [indices[0], indices[1], indices[2]])?;
            } else { self.polygon_rings(mesh, &boundaries_indices)?; }
        }
        Ok(())
    }

    fn face(&mut self, id: u64, transform: &Mat4, mesh: &mut GeometryMesh) -> ProjectionResult<()> {
        let entity = self.entity(id)?;
        let bounds = self.references(entity, 0)?;
        let mut boundaries = Vec::new();
        let mut outer = None;
        for id in bounds {
            let bound = self.entity(id)?;
            if !matches!(bound.name, "IFCFACEOUTERBOUND" | "IFCFACEBOUND") { return Err("invalid-face-bound".into()); }
            if bound.name == "IFCFACEOUTERBOUND" {
                if outer.is_some() { return Err("multiple-face-outer-bounds".into()); }
                outer = Some(boundaries.len());
            }
            let loop_id = reference(bound, 0).ok_or("missing-face-loop")?;
            let orientation = bound.args.get(1).and_then(V::as_enum).ok_or("invalid-bound-orientation")?;
            if !matches!(orientation, "T" | "F") { return Err("invalid-bound-orientation".into()); }
            let curve = self.entity(loop_id)?;
            if curve.name != "IFCPOLYLOOP" { return Err("unsupported-face-loop".into()); }
            let points = self.references(curve, 0)?;
            let mut indices = Vec::new();
            for id in points { let point = self.point(id, false)?; indices.push(self.vertex(mesh, point, transform)?); }
            if orientation == "F" { indices.reverse(); }
            boundaries.push(indices);
        }
        if boundaries.is_empty() { return Err("missing-face-bound".into()); }
        if outer.is_none() {
            let mut largest = -1.;
            for (i, boundary) in boundaries.iter().enumerate() {
                self.tick()?;
                let mut span: Point = [0.; 3];
                for axis in 0..3 {
                    let (mut low, mut high) = (f64::INFINITY, f64::NEG_INFINITY);
                    for &id in boundary { self.tick()?; low = low.min(mesh.vertices[id as usize][axis]); high = high.max(mesh.vertices[id as usize][axis]); }
                    span[axis] = high - low;
                }
                let size = span[0].hypot(span[1]).hypot(span[2]);
                if size > largest { largest = size; outer = Some(i); }
            }
        }
        boundaries.swap(0, outer.ok_or("missing-face-outer-bound")?);
        self.polygon_rings(mesh, &boundaries)
    }

    fn ellipse_profile(&mut self, a: f64, b: f64, position: Option<u64>) -> ProjectionResult<Vec<Point>> {
        if !a.is_finite() || !b.is_finite() || a <= 0. || b <= 0. { return Err("invalid-ellipse-dimensions".into()); }
        if let Some(id) = position {
            if self.entity(id)?.name != "IFCAXIS2PLACEMENT2D" { return Err("profile-placement-must-be-2d".into()); }
        }
        let transform = if let Some(id) = position { self.axis(id)? } else { Mat4::identity() };
        let mut points = Vec::new();
        for i in 0..CURVE_SEGMENTS {
            self.tick()?;
            let angle = std::f64::consts::TAU * i as f64 / CURVE_SEGMENTS as f64;
            let point = transform.transform_point([a * angle.cos(), b * angle.sin(), 0.]);
            if point[2].abs() > 1e-9 { return Err("profile-must-be-planar-2d".into()); }
            points.push(point);
        }
        Ok(points)
    }

    fn arc_points(&mut self,start:Point,middle:Point,end:Point)->ProjectionResult<Vec<Point>>{
        let u=subtract(middle,start);let v=subtract(end,start);let extent=u[0].abs().max(u[1].abs()).max(v[0].abs()).max(v[1].abs());
        if extent<=0.||!extent.is_finite(){return Err("invalid-profile-arc".into());}
        let a=[u[0]/extent,u[1]/extent];let b=[v[0]/extent,v[1]/extent];let determinant=2.*(a[0]*b[1]-a[1]*b[0]);
        if determinant==0.{return Ok(vec![middle,end]);}
        if determinant.abs()<128.*f64::EPSILON{return Err("ill-conditioned-profile-arc".into());}
        let aa=a[0]*a[0]+a[1]*a[1];let bb=b[0]*b[0]+b[1]*b[1];let center=[(aa*b[1]-bb*a[1])/determinant,(a[0]*bb-b[0]*aa)/determinant];let radius=center[0].hypot(center[1]);let orientation=determinant.signum();let tau=std::f64::consts::TAU;
        let begin=(-center[1]).atan2(-center[0]);let mid=(a[1]-center[1]).atan2(a[0]-center[0]);let last=(b[1]-center[1]).atan2(b[0]-center[0]);let mut points=Vec::new();
        for(from,to,exact)in [(begin,mid,middle),(mid,last,end)]{
            let angle=if orientation>0.{(to-from).rem_euclid(tau)}else{-(from-to).rem_euclid(tau)};let count=((angle.abs()/(tau/CURVE_SEGMENTS as f64)-1e-12).ceil() as usize).max(1);
            for i in 1..=count{self.tick()?;if i==count{points.push(exact);}else{let t=from+angle*i as f64/count as f64;points.push([start[0]+(center[0]+radius*t.cos())*extent,start[1]+(center[1]+radius*t.sin())*extent,0.]);}}
        }
        Ok(points)
    }

    fn indexed_profile_curve(&mut self,curve:GeometryEntity<'_,V>)->ProjectionResult<Vec<Point>>{
        if self.standard!=GeometryStandard::Ifc4{return Err(format!("unsupported-profile-curve-{}",curve.name));}
        let list=self.entity(reference(curve,0).ok_or("missing-profile-point-list")?)?;
        if list.name!="IFCCARTESIANPOINTLIST2D"{return Err("profile-point-list-must-be-2d".into());}
        let coordinates=list.args.first().and_then(V::as_aggregate).ok_or("missing-aggregate")?;let mut points=Vec::new();
        for row in coordinates{self.tick()?;let xy=row.as_aggregate().ok_or("missing-aggregate")?;if xy.len()!=2{return Err("invalid-2d-coordinate-dimension".into());}let x=xy[0].as_real().filter(|v|v.is_finite()).ok_or("missing-finite-number")?;let y=xy[1].as_real().filter(|v|v.is_finite()).ok_or("missing-finite-number")?;points.push([x,y,0.]);}
        let segments=curve.args.get(1);let mut polygon=Vec::new();
        if segments.is_some_and(V::is_unset){for point in points{self.tick()?;polygon.push(point);}}
        else{
            let mut first=None;let mut last=None;
            for segment in segments.and_then(V::as_aggregate).ok_or("missing-aggregate")?{
                self.tick()?;let(name,items)=segment.as_typed().ok_or("invalid-indexed-profile-segment")?;if items.len()!=1{return Err("invalid-indexed-profile-segment".into());}let mut indices=Vec::new();
                for value in items[0].as_aggregate().ok_or("missing-aggregate")?{self.tick()?;let index=index_value(value)?;if index>=points.len(){return Err("profile-index-out-of-range".into());}indices.push(index);}
                if name!="IFCLINEINDEX"&&name!="IFCARCINDEX"{return Err(format!("unsupported-indexed-profile-segment-{name}"));}
                if if name=="IFCARCINDEX"{indices.len()!=3}else{indices.len()<2}{return Err("invalid-indexed-profile-segment-size".into());}
                if first.is_none(){first=Some(indices[0]);polygon.push(points[indices[0]]);}else if Some(indices[0])!=last{return Err("disconnected-indexed-profile-segments".into());}
                if name=="IFCARCINDEX"{polygon.extend(self.arc_points(points[indices[0]],points[indices[1]],points[indices[2]])?);}else{for &index in &indices[1..]{self.tick()?;polygon.push(points[index]);}}
                last=indices.last().copied();
            }
            if first.is_none()||first!=last{return Err("indexed-profile-must-close-by-index".into());}
        }
        if polygon.is_empty()||polygon.first()!=polygon.last(){return Err("profile-must-be-closed".into());}polygon.pop();Ok(polygon)
    }

    fn profile_curve(&mut self, id: u64) -> ProjectionResult<Vec<Point>> {
        let curve = self.entity(id)?;
        if curve.name=="IFCINDEXEDPOLYCURVE"{return self.indexed_profile_curve(curve);}
        if curve.name == "IFCCIRCLE" || curve.name == "IFCELLIPSE" {
            let a = required_real(curve, 1)?;
            let b = if curve.name == "IFCCIRCLE" { a } else { required_real(curve, 2)? };
            let position = reference(curve, 0).ok_or("missing-profile-curve-placement")?;
            return self.ellipse_profile(a, b, Some(position));
        }
        if curve.name != "IFCPOLYLINE" { return Err(format!("unsupported-profile-curve-{}", curve.name)); }
        let points = self.references(curve, 0)?;
        let mut polygon = Vec::new();
        for id in points { polygon.push(self.point(id, false)?); }
        if polygon.is_empty() || polygon.first() != polygon.last() { return Err("profile-must-be-closed".into()); }
        polygon.pop();
        if polygon.iter().any(|point| point[2].abs() > 1e-9) { return Err("profile-must-be-planar-2d".into()); }
        Ok(polygon)
    }

    fn rounded_rectangle(&mut self,entity:GeometryEntity<'a,V>,position:Option<u64>)->ProjectionResult<Vec<Point>>{
        if entity.args.len()!=6||entity.args.first().and_then(V::as_enum)!=Some("AREA"){return Err("invalid-rounded-rectangle-profile".into());}let(x,y,r)=(required_real(entity,3)?/2.,required_real(entity,4)?/2.,required_real(entity,5)?);if x<=0.||y<=0.||r<=0.||r>x||r>y{return Err("invalid-rounded-rectangle-radius-or-dimensions".into());}if let Some(id)=position{if self.entity(id)?.name!="IFCAXIS2PLACEMENT2D"{return Err("profile-placement-must-be-2d".into());}}let transform=if let Some(id)=position{self.axis(id)?}else{Mat4::identity()};let centers=[[x-r,-y+r,0.],[x-r,y-r,0.],[-x+r,y-r,0.],[-x+r,-y+r,0.]];let cardinal=[[0.,-1.,0.],[1.,0.,0.],[0.,1.,0.],[-1.,0.,0.],[0.,-1.,0.]];let segments=CURVE_SEGMENTS/4;let mut points=Vec::new();for corner in 0..4{for i in 0..=segments{self.tick()?;let angle=(corner as f64-1.)*std::f64::consts::PI/2.+i as f64*std::f64::consts::PI/2./segments as f64;let d=if i==0{cardinal[corner]}else if i==segments{cardinal[corner+1]}else{[angle.cos(),angle.sin(),0.]};let point=transform.transform_point(add(centers[corner],scale(d,r)));if points.last()!=Some(&point){points.push(point);}}}if points.len()>1&&points.first()==points.last(){points.pop();}Ok(points)
    }

    fn profile(&mut self, id: u64) -> ProjectionResult<Vec<Point>> {
        let entity = self.entity(id)?;
        let name = entity.name.clone();
        let position = optional_reference(entity, 2)?;
        match name {
            "IFCROUNDEDRECTANGLEPROFILEDEF"=>self.rounded_rectangle(entity,position),
            "IFCCIRCLEPROFILEDEF" | "IFCELLIPSEPROFILEDEF" => {
                if entity.args.first().and_then(V::as_enum)!=Some("AREA") { return Err("profile-must-be-area".into()); }
                let a = required_real(entity, 3)?;
                let b = if name == "IFCCIRCLEPROFILEDEF" { a } else { required_real(entity, 4)? };
                self.ellipse_profile(a, b, position)
            }
            "IFCRECTANGLEPROFILEDEF" => {
                let x = required_real(entity, 3)? / 2.;
                let y = required_real(entity, 4)? / 2.;
                if x <= 0. || y <= 0. { return Err("invalid-rectangle-dimensions".into()); }
                let transform = if let Some(id) = position { self.axis(id)? } else { Mat4::identity() };
                Ok((0..4).map(|index|transform.transform_point(primitive::rectangle_point(2.*x,2.*y,index))).collect())
            }
            "IFCARBITRARYCLOSEDPROFILEDEF" => {
                if entity.args.first().and_then(V::as_enum) != Some("AREA") { return Err("profile-must-be-area".into()); }
                self.profile_curve(position.ok_or("missing-profile-curve")?)
            }
            _ => Err(format!("unsupported-profile-{name}")),
        }
    }

    fn profile_rings(&mut self, id: u64) -> ProjectionResult<Vec<Vec<Point>>> {
        let entity = self.entity(id)?;
        if entity.args.first().and_then(V::as_enum) != Some("AREA") { return Err("profile-must-be-area".into()); }
        if entity.name == "IFCARBITRARYPROFILEDEFWITHVOIDS" {
            let outer = reference(entity, 2).ok_or("missing-profile-curve")?;
            let holes = self.references(entity, 3)?;
            if holes.is_empty() { return Err("voided-profile-needs-inner-curve".into()); }
            let mut boundaries = vec![self.profile_curve(outer)?];
            for hole in holes { boundaries.push(self.profile_curve(hole)?); }
            return Ok(boundaries);
        }
        if entity.name == "IFCCIRCLEHOLLOWPROFILEDEF" {
            let radius = required_real(entity, 3)?;
            let wall = required_real(entity, 4)?;
            if wall <= 0. || wall >= radius { return Err("invalid-hollow-circle-wall".into()); }
            let position = optional_reference(entity, 2)?;
            return Ok(vec![self.ellipse_profile(radius, radius, position)?, self.ellipse_profile(radius - wall, radius - wall, position)?]);
        }
        Ok(vec![self.profile(id)?])
    }

    fn extrusion(&mut self, id: u64, transform: &Mat4, mesh: &mut GeometryMesh) -> ProjectionResult<()> {
        let entity = self.entity(id)?;
        let profile = reference(entity, 0).ok_or("missing-swept-area")?;
        let position = reference(entity, 1);
        let direction = reference(entity, 2).ok_or("missing-extrusion-direction")?;
        let depth = required_real(entity, 3)?;
        if depth <= 0. { return Err("invalid-extrusion-depth".into()); }
        let polygons = self.profile_rings(profile)?;
        let direction = normalize(self.point(direction, true)?)?;
        if direction[2].abs() < 1e-12 { return Err("extrusion-direction-in-profile-plane".into()); }
        let mut points = Vec::new();
        let mut boundaries = Vec::new();
        for mut polygon in polygons {
            if polygon.len() < 3 { return Err("profile-needs-three-points".into()); }
            let origin = polygon[0];
            let mut extent: f64 = 0.;
            for point in &polygon { self.tick()?; extent = extent.max((point[0] - origin[0]).abs()).max((point[1] - origin[1]).abs()); }
            let mut area = 0.;
            for i in 0..polygon.len() {
                self.tick()?; let a = polygon[i]; let b = polygon[(i + 1) % polygon.len()];
                area += (a[0] - origin[0]) / extent * ((b[1] - origin[1]) / extent) - (b[0] - origin[0]) / extent * ((a[1] - origin[1]) / extent);
            }
            if area * (if boundaries.is_empty() { 1. } else { -1. }) <= 0. { polygon.reverse(); }
            let mut indices = Vec::new();
            for point in polygon { self.tick()?; let index = u32::try_from(points.len()).map_err(|_| "profile-index-overflow")?; points.push(point); indices.push(index); }
            boundaries.push(indices);
        }
        let caps = rings::triangulate_rings(&points, &boundaries, &mut || self.tick())?;
        let transform = if let Some(id) = position { transform.mul(&self.axis(id)?) } else { transform.clone() };
        let mut bottom = Vec::new();
        let mut top = Vec::new();
        for point in &points { bottom.push(self.vertex(mesh, *point, &transform)?); }
        for point in &points { top.push(self.vertex(mesh, add(*point, scale(direction, depth)), &transform)?); }
        for triangle in caps {
            let [a, b, c] = triangle.map(|index| index as usize);
            self.triangle(mesh, if direction[2] > 0. { [bottom[c], bottom[b], bottom[a]] } else { [bottom[a], bottom[b], bottom[c]] })?;
            self.triangle(mesh, if direction[2] > 0. { [top[a], top[b], top[c]] } else { [top[c], top[b], top[a]] })?;
        }
        for boundary in boundaries { for i in 0..boundary.len() {
            self.tick()?; let a = boundary[i] as usize; let b = boundary[(i + 1) % boundary.len()] as usize;
            self.triangle(mesh, if direction[2] > 0. { [bottom[a], bottom[b], top[b]] } else { [bottom[b], bottom[a], top[a]] })?;
            self.triangle(mesh, if direction[2] > 0. { [bottom[a], top[b], top[a]] } else { [bottom[b], top[a], top[b]] })?;
        } }
        Ok(())
    }
}

fn reference<V:GeometryValue>(entity:GeometryEntity<'_,V>,index:usize)->Option<u64>{entity.args.get(index).and_then(V::as_reference)}
fn optional_reference<V:GeometryValue>(entity:GeometryEntity<'_,V>,index:usize)->ProjectionResult<Option<u64>>{match entity.args.get(index){None=>Ok(None),Some(value) if value.is_unset()=>Ok(None),Some(value)=>value.as_reference().map(Some).ok_or_else(||"invalid-optional-reference".into())}}
fn coordinates<V:GeometryValue>(value:&V)->ProjectionResult<Point>{let list=value.as_aggregate().ok_or("invalid-coordinates")?;if !(2..=3).contains(&list.len()){return Err("coordinate-dimension".into());}let mut result=[0.;3];for(i,value)in list.iter().enumerate(){result[i]=value.as_real().filter(|v|v.is_finite()).ok_or("nonfinite-coordinate")?;}Ok(result)}
fn required_real<V:GeometryValue>(entity:GeometryEntity<'_,V>,index:usize)->ProjectionResult<f64>{entity.args.get(index).and_then(V::as_real).filter(|v|v.is_finite()).ok_or_else(||"missing-finite-number".into())}
fn optional_real<V:GeometryValue>(entity:GeometryEntity<'_,V>,index:usize,fallback:f64)->ProjectionResult<f64>{match entity.args.get(index){None=>Ok(fallback),Some(value)if value.is_unset()=>Ok(fallback),_=>required_real(entity,index)}}
fn index_value<V:GeometryValue>(value:&V)->ProjectionResult<usize>{let index=value.as_integer().filter(|index|*index>0).ok_or("invalid-one-based-index")?;usize::try_from(index-1).map_err(|_|"index-overflow".into())}
fn add(a: Point, b: Point) -> Point { [a[0] + b[0], a[1] + b[1], a[2] + b[2]] }
pub(crate) fn subtract(a: Point, b: Point) -> Point { [a[0] - b[0], a[1] - b[1], a[2] - b[2]] }
fn scale(a: Point, s: f64) -> Point { [a[0] * s, a[1] * s, a[2] * s] }
pub(crate) fn dot(a: Point, b: Point) -> f64 { a[0] * b[0] + a[1] * b[1] + a[2] * b[2] }
pub(crate) fn cross(a: Point, b: Point) -> Point { [a[1] * b[2] - a[2] * b[1], a[2] * b[0] - a[0] * b[2], a[0] * b[1] - a[1] * b[0]] }
fn normalize(a:Point)->ProjectionResult<Point>{matrix::normalize_direction(a).ok_or_else(||"invalid-direction".into())}

fn cross2(a: [f64; 2], b: [f64; 2], c: [f64; 2]) -> f64 { (b[0] - a[0]) * (c[1] - a[1]) - (b[1] - a[1]) * (c[0] - a[0]) }
fn matrix(origin: Point, x: Point, y: Point, z: Point) -> Mat4 { Mat4([[x[0], y[0], z[0], origin[0]], [x[1], y[1], z[1], origin[1]], [x[2], y[2], z[2], origin[2]], [0., 0., 0., 1.]]) }
fn determinant(matrix: &Mat4) -> f64 {
    dot([matrix.0[0][0], matrix.0[1][0], matrix.0[2][0]], cross([matrix.0[0][1], matrix.0[1][1], matrix.0[2][1]], [matrix.0[0][2], matrix.0[1][2], matrix.0[2][2]]))
}


#[cfg(test)]
#[path="🧪️tests/🦀️.rs"]mod tests;

#[cfg(test)]
#[global_allocator]
static TEST_REQUESTED_ALLOCATOR: semio_framework_trace_memory_testing::RequestedAllocator = semio_framework_trace_memory_testing::RequestedAllocator;

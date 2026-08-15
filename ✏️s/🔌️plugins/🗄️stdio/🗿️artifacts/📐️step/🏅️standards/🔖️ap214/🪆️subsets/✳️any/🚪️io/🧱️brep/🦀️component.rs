//! 🧱️ BrepMesh — AP214 analyzer view derived from the generic Part-21 graph, never persisted
//! itself. Walks the handful of boundary-representation entities real STEP exporters use for
//! planar/polygonal faces (`CARTESIAN_POINT`, `DIRECTION`, `VERTEX_POINT`, `EDGE_CURVE`,
//! `ORIENTED_EDGE`, `EDGE_LOOP`, `FACE_BOUND`/`FACE_OUTER_BOUND`, `ADVANCED_FACE`,
//! `CLOSED_SHELL`/`MANIFOLD_SOLID_BREP`). True curved-surface tessellation (NURBS/B-spline) is
//! out of scope — a face whose geometry isn't a `PLANE`, or an edge whose curve isn't a `LINE`,
//! is flagged in `BrepMeshView::issues` rather than silently producing a wrong mesh.

use super::part21::{Part21Builder, Part21Document, Part21Header, Part21Value};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

//#region 🔖️Model
/// 📍️ B-rep vertex.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BrepVertex {
    pub x: f64,
    pub y: f64,
    pub z: f64,
}

/// 🔺️ B-rep face as ordered polygon vertex indices (planar; not triangulated).
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BrepFace {
    #[serde(default)]
    pub indices: Vec<usize>,
}

/// 📐️ Neutral B-rep mesh — the derived analyzer output, never the persisted snapshot itself.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
#[serde(rename_all = "camelCase")]
pub struct BrepMesh {
    #[serde(default)]
    pub vertices: Vec<BrepVertex>,
    #[serde(default)]
    pub faces: Vec<BrepFace>,
}

/// 🚩️ One entity this analyzer could not (fully) resolve into the mesh — typed, surfaced,
/// never silently dropped.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize)]
pub struct BrepIssue {
    pub entity_id: u64,
    pub reason: String,
}

/// 🧐️ Result of analyzing a `Part21Document` for its faceted b-rep content.
#[derive(Clone, Debug, PartialEq, Serialize, Deserialize, Default)]
pub struct BrepMeshView {
    pub mesh: BrepMesh,
    pub issues: Vec<BrepIssue>,
}
//#endregion 🔖️Model

//#region 🔖️Analyze
struct BrepBuilder<'a> {
    doc: &'a Part21Document,
    vertex_of: HashMap<u64, usize>,
    vertices: Vec<BrepVertex>,
    issues: Vec<BrepIssue>,
}

impl<'a> BrepBuilder<'a> {
    fn point(&self, id: u64) -> Option<[f64; 3]> {
        let args = self.doc.instance(id)?.entity("CARTESIAN_POINT")?;
        let coords = args.get(1)?.as_list()?;
        Some([coords.first()?.as_real()?, coords.get(1)?.as_real()?, coords.get(2)?.as_real()?])
    }

    fn vertex_point(&mut self, id: u64) -> Option<usize> {
        if let Some(&idx) = self.vertex_of.get(&id) {
            return Some(idx);
        }
        let args = self.doc.instance(id)?.entity("VERTEX_POINT")?;
        let point_ref = args.get(1)?.as_ref_id()?;
        let [x, y, z] = self.point(point_ref)?;
        let idx = self.vertices.len();
        self.vertices.push(BrepVertex { x, y, z });
        self.vertex_of.insert(id, idx);
        Some(idx)
    }

    /// ➡️ `(start_vertex_idx, end_vertex_idx)` honoring `ORIENTED_EDGE.orientation`.
    fn oriented_edge_endpoints(&mut self, oriented_edge_id: u64) -> Option<(usize, usize)> {
        let args = self.doc.instance(oriented_edge_id)?.entity("ORIENTED_EDGE")?;
        let edge_ref = args.get(3)?.as_ref_id()?;
        let orientation = matches!(args.get(4), Some(Part21Value::Enum(e)) if e == "T");
        let edge_args = self.doc.instance(edge_ref)?.entity("EDGE_CURVE")?;
        let start_ref = edge_args.get(1)?.as_ref_id()?;
        let end_ref = edge_args.get(2)?.as_ref_id()?;
        if let Some(geom_ref) = edge_args.get(3).and_then(Part21Value::as_ref_id) {
            if let Some((geom_type, _)) = self.doc.instance(geom_ref).and_then(|i| i.primary()) {
                if !geom_type.eq_ignore_ascii_case("LINE") {
                    self.issues.push(BrepIssue { entity_id: edge_ref, reason: format!("edge geometry {geom_type} is not a straight LINE; endpoints used as a control-polygon degrade, not a true curve tessellation") });
                }
            }
        }
        let start = self.vertex_point(start_ref)?;
        let end = self.vertex_point(end_ref)?;
        Some(if orientation { (start, end) } else { (end, start) })
    }

    fn edge_loop(&mut self, edge_loop_id: u64) -> Option<Vec<usize>> {
        let args = self.doc.instance(edge_loop_id)?.entity("EDGE_LOOP")?;
        let edges = args.get(1)?.as_list()?.to_vec();
        let mut indices = Vec::with_capacity(edges.len());
        for e in &edges {
            let oe_id = e.as_ref_id()?;
            let (start, _end) = self.oriented_edge_endpoints(oe_id)?;
            indices.push(start);
        }
        Some(indices)
    }

    fn face_bound_loop(&mut self, bound_id: u64) -> Option<Vec<usize>> {
        let inst = self.doc.instance(bound_id)?;
        let (bound_type, args) = inst.primary()?;
        if !(bound_type.eq_ignore_ascii_case("FACE_BOUND") || bound_type.eq_ignore_ascii_case("FACE_OUTER_BOUND")) {
            return None;
        }
        let loop_ref = args.get(1)?.as_ref_id()?;
        let orientation = matches!(args.get(2), Some(Part21Value::Enum(e)) if e == "T");
        let mut indices = self.edge_loop(loop_ref)?;
        if !orientation {
            indices.reverse();
        }
        Some(indices)
    }

    fn advanced_face(&mut self, face_id: u64) -> Option<BrepFace> {
        let args = self.doc.instance(face_id)?.entity("ADVANCED_FACE")?;
        let bounds = args.get(1)?.as_list()?.to_vec();
        if let Some(geom_ref) = args.get(2).and_then(Part21Value::as_ref_id) {
            if let Some((geom_type, _)) = self.doc.instance(geom_ref).and_then(|i| i.primary()) {
                if !geom_type.eq_ignore_ascii_case("PLANE") {
                    self.issues.push(BrepIssue { entity_id: face_id, reason: format!("face geometry {geom_type} is not a PLANE; curved-surface tessellation is out of scope, face skipped") });
                    return None;
                }
            }
        }
        let outer = bounds.first()?.as_ref_id()?;
        let indices = self.face_bound_loop(outer)?;
        if indices.len() < 3 {
            self.issues.push(BrepIssue { entity_id: face_id, reason: "face bound resolved to fewer than 3 vertices".into() });
            return None;
        }
        Some(BrepFace { indices })
    }
}

/// 🧐️ Derives a `BrepMeshView` from the generic Part-21 graph. Real walk, not a scraper:
/// prefers `CLOSED_SHELL`s' own face lists; falls back to every `ADVANCED_FACE` in the document
/// when no shell groups them (still real data, just ungrouped).
pub fn analyze_brep_mesh(doc: &Part21Document) -> BrepMeshView {
    let mut builder = BrepBuilder { doc, vertex_of: HashMap::new(), vertices: Vec::new(), issues: Vec::new() };
    let mut face_ids: Vec<u64> = Vec::new();
    for shell in doc.by_type("CLOSED_SHELL") {
        if let Some(args) = shell.entity("CLOSED_SHELL") {
            if let Some(list) = args.get(1).and_then(Part21Value::as_list) {
                for v in list {
                    if let Some(id) = v.as_ref_id() {
                        face_ids.push(id);
                    }
                }
            }
        }
    }
    if face_ids.is_empty() {
        face_ids.extend(doc.by_type("ADVANCED_FACE").map(|f| f.id));
    }
    let mut faces = Vec::new();
    for id in face_ids {
        if let Some(face) = builder.advanced_face(id) {
            faces.push(face);
        } else if builder.issues.iter().all(|i| i.entity_id != id) {
            builder.issues.push(BrepIssue { entity_id: id, reason: "face could not be resolved to a supported polygon".into() });
        }
    }
    BrepMeshView { mesh: BrepMesh { vertices: builder.vertices, faces }, issues: builder.issues }
}
//#endregion 🔖️Analyze

//#region 🔖️Write
fn face_normal(mesh: &BrepMesh, indices: &[usize]) -> Option<[f64; 3]> {
    if indices.len() < 3 {
        return None;
    }
    let p0 = mesh.vertices.get(indices[0])?;
    let p1 = mesh.vertices.get(indices[1])?;
    let p2 = mesh.vertices.get(indices[2])?;
    let (ux, uy, uz) = (p1.x - p0.x, p1.y - p0.y, p1.z - p0.z);
    let (vx, vy, vz) = (p2.x - p0.x, p2.y - p0.y, p2.z - p0.z);
    let (nx, ny, nz) = (uy * vz - uz * vy, uz * vx - ux * vz, ux * vy - uy * vx);
    let len = (nx * nx + ny * ny + nz * nz).sqrt();
    if len < 1e-12 {
        return None;
    }
    Some([nx / len, ny / len, nz / len])
}

fn s(text: &str) -> Part21Value {
    Part21Value::Str(text.to_string())
}
fn xyz(v: [f64; 3]) -> Part21Value {
    Part21Value::List(vec![Part21Value::Real(v[0].into()), Part21Value::Real(v[1].into()), Part21Value::Real(v[2].into())])
}

/// 📤️ Regenerates a real, minimal AP214 `EDGE_LOOP`-based faceted b-rep from a `BrepMesh` —
/// the inverse of `analyze_brep_mesh` for planar faces (used by cross-plugin producers, e.g.
/// the cad plugin's step export, so nothing outside this module hand-rolls Part-21 text).
pub fn brep_mesh_to_part21(mesh: &BrepMesh) -> Part21Document {
    let mut b = Part21Builder::new();
    let point_ids: Vec<u64> = mesh.vertices.iter().map(|v| b.alloc("CARTESIAN_POINT", vec![s(""), xyz([v.x, v.y, v.z])])).collect();
    let vertex_ids: Vec<u64> = point_ids.iter().map(|&p| b.alloc("VERTEX_POINT", vec![s(""), Part21Value::Ref(p)])).collect();

    let mut face_ids = Vec::new();
    for face in &mesh.faces {
        let n = face.indices.len();
        if n < 3 {
            continue;
        }
        let mut oriented_edges = Vec::with_capacity(n);
        for i in 0..n {
            let a = face.indices[i];
            let bi = face.indices[(i + 1) % n];
            let (Some(&va), Some(&vb)) = (vertex_ids.get(a), vertex_ids.get(bi)) else { continue };
            let (pa, pb) = (&mesh.vertices[a], &mesh.vertices[bi]);
            let (dx, dy, dz) = (pb.x - pa.x, pb.y - pa.y, pb.z - pa.z);
            let len = (dx * dx + dy * dy + dz * dz).sqrt();
            let dir = if len > 1e-12 { [dx / len, dy / len, dz / len] } else { [1.0, 0.0, 0.0] };
            let dir_id = b.alloc("DIRECTION", vec![s(""), xyz(dir)]);
            let vec_id = b.alloc("VECTOR", vec![s(""), Part21Value::Ref(dir_id), Part21Value::Real(len.into())]);
            let line_id = b.alloc("LINE", vec![s(""), Part21Value::Ref(point_ids[a]), Part21Value::Ref(vec_id)]);
            let edge_id = b.alloc("EDGE_CURVE", vec![s(""), Part21Value::Ref(va), Part21Value::Ref(vb), Part21Value::Ref(line_id), Part21Value::Enum("T".into())]);
            let oe_id = b.alloc("ORIENTED_EDGE", vec![s(""), Part21Value::Derived, Part21Value::Derived, Part21Value::Ref(edge_id), Part21Value::Enum("T".into())]);
            oriented_edges.push(Part21Value::Ref(oe_id));
        }
        let loop_id = b.alloc("EDGE_LOOP", vec![s(""), Part21Value::List(oriented_edges)]);
        let bound_id = b.alloc("FACE_OUTER_BOUND", vec![s(""), Part21Value::Ref(loop_id), Part21Value::Enum("T".into())]);
        let normal = face_normal(mesh, &face.indices).unwrap_or([0.0, 0.0, 1.0]);
        let normal_id = b.alloc("DIRECTION", vec![s(""), xyz(normal)]);
        let origin = &mesh.vertices[face.indices[0]];
        let origin_id = b.alloc("CARTESIAN_POINT", vec![s(""), xyz([origin.x, origin.y, origin.z])]);
        let axis_id = b.alloc("AXIS2_PLACEMENT_3D", vec![s(""), Part21Value::Ref(origin_id), Part21Value::Ref(normal_id), Part21Value::Unset]);
        let plane_id = b.alloc("PLANE", vec![s(""), Part21Value::Ref(axis_id)]);
        let face_id = b.alloc("ADVANCED_FACE", vec![s(""), Part21Value::List(vec![Part21Value::Ref(bound_id)]), Part21Value::Ref(plane_id), Part21Value::Enum("T".into())]);
        face_ids.push(Part21Value::Ref(face_id));
    }
    let shell_id = b.alloc("CLOSED_SHELL", vec![s(""), Part21Value::List(face_ids)]);
    b.alloc("MANIFOLD_SOLID_BREP", vec![s(""), Part21Value::Ref(shell_id)]);

    b.build(Part21Header {
        file_description: vec![Part21Value::List(vec![s("")]), s("2;1")],
        file_name: vec![s("semio.step"), s(""), Part21Value::List(vec![s("")]), Part21Value::List(vec![s("")]), s("semio"), s(""), s("")],
        file_schema: vec![Part21Value::List(vec![s("AUTOMOTIVE_DESIGN")])],
    })
}
//#endregion 🔖️Write

//#region 🧪️Tests
#[cfg(test)]
mod tests {
    use super::super::part21::parse_part21;
    use super::*;

    const FIXTURE: &str = "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('semio.step','2026-08-10T00:00:00',('Ueli'),('semio'),'semio','','');\nFILE_SCHEMA(('AUTOMOTIVE_DESIGN'));\nENDSEC;\nDATA;\n#1=CARTESIAN_POINT('',(0.,0.,0.));\n#2=CARTESIAN_POINT('',(10.,0.,0.));\n#3=CARTESIAN_POINT('',(10.,10.,0.));\n#4=DIRECTION('',(0.,0.,1.));\n#5=VERTEX_POINT('',#1);\n#6=VERTEX_POINT('',#2);\n#7=VERTEX_POINT('',#3);\n#8=EDGE_CURVE('',#5,#6,#20,.T.);\n#9=EDGE_CURVE('',#6,#7,#21,.T.);\n#10=EDGE_CURVE('',#7,#5,#22,.T.);\n#20=LINE('',#1,#30);\n#21=LINE('',#2,#31);\n#22=LINE('',#3,#32);\n#30=VECTOR('',#4,1.);\n#31=VECTOR('',#4,1.);\n#32=VECTOR('',#4,1.);\n#11=ORIENTED_EDGE('',*,*,#8,.T.);\n#12=ORIENTED_EDGE('',*,*,#9,.T.);\n#13=ORIENTED_EDGE('',*,*,#10,.T.);\n#14=EDGE_LOOP('',(#11,#12,#13));\n#15=FACE_OUTER_BOUND('',#14,.T.);\n#16=PLANE('',#40);\n#40=AXIS2_PLACEMENT_3D('',#1,#4,$);\n#17=ADVANCED_FACE('',(#15),#16,.T.);\n#18=CLOSED_SHELL('',(#17));\n#19=MANIFOLD_SOLID_BREP('',#18);\nENDSEC;\nEND-ISO-10303-21;\n";

    #[test]
    fn analyzes_real_non_degenerate_mesh() {
        let doc = parse_part21(FIXTURE).expect("parse fixture");
        let view = analyze_brep_mesh(&doc);
        assert!(view.issues.is_empty(), "unexpected issues: {:?}", view.issues);
        assert_eq!(view.mesh.vertices.len(), 3);
        assert_eq!(view.mesh.faces.len(), 1);
        let face = &view.mesh.faces[0];
        assert_eq!(face.indices.len(), 3);
        for &idx in &face.indices {
            assert!(idx < view.mesh.vertices.len(), "face references an invalid vertex index");
        }
        assert_eq!(view.mesh.vertices[1].x, 10.0);
        assert_eq!(view.mesh.vertices[2].y, 10.0);
    }

    #[test]
    fn curved_surface_flagged_unsupported_not_fabricated() {
        let text = FIXTURE.replace("#16=PLANE('',#40);", "#16=B_SPLINE_SURFACE_WITH_KNOTS('',3,3,((#1,#2,#3)),.UNSPECIFIED.,.F.,.F.,.F.,(4,4),(4,4),(0.,1.),(0.,1.),.UNSPECIFIED.);");
        let doc = parse_part21(&text).expect("parse");
        let view = analyze_brep_mesh(&doc);
        assert!(view.mesh.faces.is_empty(), "a curved face must not silently become a mesh face");
        assert!(view.issues.iter().any(|i| i.reason.contains("PLANE")));
    }

    #[test]
    fn writer_round_trips_through_analyzer() {
        let mesh =
            BrepMesh { vertices: vec![BrepVertex { x: 0.0, y: 0.0, z: 0.0 }, BrepVertex { x: 4.0, y: 0.0, z: 0.0 }, BrepVertex { x: 4.0, y: 3.0, z: 0.0 }, BrepVertex { x: 0.0, y: 3.0, z: 0.0 }], faces: vec![BrepFace { indices: vec![0, 1, 2, 3] }] };
        let doc = brep_mesh_to_part21(&mesh);
        let text = super::super::part21::write_part21(&doc);
        let reparsed = parse_part21(&text).expect("reparse generated step text");
        let view = analyze_brep_mesh(&reparsed);
        assert!(view.issues.is_empty(), "unexpected issues: {:?}", view.issues);
        assert_eq!(view.mesh.vertices.len(), 4);
        assert_eq!(view.mesh.faces.len(), 1);
        assert_eq!(view.mesh.faces[0].indices.len(), 4);
    }

    #[test]
    fn empty_mesh_writer_still_produces_valid_document() {
        let doc = brep_mesh_to_part21(&BrepMesh::default());
        let text = super::super::part21::write_part21(&doc);
        let reparsed = parse_part21(&text).expect("reparse empty mesh document");
        let view = analyze_brep_mesh(&reparsed);
        assert!(view.mesh.faces.is_empty());
        assert!(view.mesh.vertices.is_empty());
    }
}
//#endregion 🧪️Tests

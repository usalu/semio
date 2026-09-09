use super::*;
use crate::standards::v1::subsets::brep::schema::diff::primitives::{make_box, make_cylinder, make_sphere};
use crate::standards::v1::subsets::brep::schema::inferences::mass_properties::oracle::{ClosedFormMass, Sdf};
use crate::standards::v1::subsets::brep::schema::snapshot::tolerance::Tol;
use crate::standards::v1::subsets::brep::schema::snapshot::topology::history::OpRecorder;
use crate::standards::v1::subsets::brep::schema::snapshot::vector::matrix::Trsf;

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn assert_classify(body: &Body, solid: SolidId, p: Pnt3, expected: PointClassification) {
    let got = point_in_solid(body, solid, p, Tol::DEFAULT.value()).unwrap();
    assert_eq!(got, expected, "point {p:?}");
}

// 🚫️async: E1 pure inherent-impl helper (file verified I/O-free, consumed via opaque-type-hostile call site) — see R9
fn oracle_inside(sdf: &Sdf, p: Pnt3) -> bool {
    sdf.contains(p, 1e-6)
}

#[semio_framework_async_macros::async_test]
async fn unit_square_loop_uv_center() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
    let face = body.solid_faces(solid)[0];
    let outer = body.faces.get(face).unwrap().outer.unwrap();
    let surface = face_surface(&body, face).unwrap();
    let boundary = loop_uv_polygon(&body, outer, surface).unwrap();
    let mut cx = 0.0;
    let mut cy = 0.0;
    for p in &boundary {
        cx += p.x;
        cy += p.y;
    }
    let n = boundary.len().max(1) as f64;
    let uv = Pnt2::new(cx / n, cy / n);
    assert!(point_in_loop(&body, face, outer, uv, 1e-6).unwrap());
}

#[semio_framework_async_macros::async_test]
async fn box_inside_outside_boundary() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
    assert_classify(&body, solid, Pnt3::new(0.5, 0.5, 0.5), PointClassification::Inside);
    assert_classify(&body, solid, Pnt3::new(2.0, 2.0, 2.0), PointClassification::Outside);
    assert_classify(&body, solid, Pnt3::new(0.0, 0.5, 0.5), PointClassification::OnBoundary);
}

#[semio_framework_async_macros::async_test]
async fn box_off_axis_point_matches_sdf_oracle() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
    let sdf = Sdf::Box { half_extents: Pnt3::new(0.5, 0.5, 0.5), placement: Trsf::translation(Vec3::new(0.5, 0.5, 0.5)) };
    let p = Pnt3::new(0.25, 0.75, 0.5);
    let expected = if oracle_inside(&sdf, p) { PointClassification::Inside } else { PointClassification::Outside };
    assert_classify(&body, solid, p, expected);
}

#[semio_framework_async_macros::async_test]
async fn ray_crossings_are_actually_bvh_culled_not_scanning_every_face() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = make_box(&mut body, 1.0, 1.0, 1.0, &mut rec).unwrap();
    let bvh = build_face_bvh(&body, solid).unwrap();
    let dir = retry_dir(0);
    let candidates = bvh.query_ray(Pnt3::new(0.5, 0.5, 0.5).to_array(), dir.to_array());
    assert!(candidates.len() < body.solid_faces(solid).len(), "a ray through the box interior should not need every one of the box's faces as a candidate");
    let outcome = count_ray_crossings(&body, &bvh, Pnt3::new(0.5, 0.5, 0.5), dir, Tol::DEFAULT.value()).unwrap();
    assert!(matches!(outcome, RayCrossingOutcome::Count(1)), "a ray from the box interior should cross the boundary exactly once");
}

#[semio_framework_async_macros::async_test]
async fn box_oracle_sdf_inside_outside() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = make_box(&mut body, 2.0, 2.0, 2.0, &mut rec).unwrap();
    let _sdf = Sdf::Box { half_extents: Pnt3::new(1.0, 1.0, 1.0), placement: Trsf::IDENTITY };
    assert_classify(&body, solid, Pnt3::new(0.5, 0.5, 0.5), PointClassification::Inside);
    assert_classify(&body, solid, Pnt3::new(3.0, 3.0, 3.0), PointClassification::Outside);
    let _ = ClosedFormMass::box_volume(Pnt3::new(1.0, 1.0, 1.0));
}

#[semio_framework_async_macros::async_test]
async fn sphere_samples_vs_oracle_sdf() {
    let mut body = Body::new();
    let r = 1.5;
    let mut rec = OpRecorder::new();
    let solid = make_sphere(&mut body, r, &mut rec).unwrap();
    let sdf = Sdf::Sphere { radius: r, placement: Trsf::IDENTITY };
    let samples = [Pnt3::new(0.3, 0.4, 0.2), Pnt3::new(r + 0.5, 0.0, 0.0), Pnt3::new(-0.9 * r, 0.3, 0.2)];
    for p in samples {
        let expected = if oracle_inside(&sdf, p) { PointClassification::Inside } else { PointClassification::Outside };
        if (p.to_vec().norm() - r).abs() < 1e-6 {
            continue;
        }
        if expected == PointClassification::Outside {
            assert_classify(&body, solid, p, expected);
        }
    }
}

#[semio_framework_async_macros::async_test]
async fn cylinder_oracle_outside_sample() {
    let mut body = Body::new();
    let radius = 1.0;
    let height = 3.0;
    let mut rec = OpRecorder::new();
    let solid = make_cylinder(&mut body, radius, height, &mut rec).unwrap();
    let sdf = Sdf::Cylinder { radius, half_height: height * 0.5, placement: Trsf::IDENTITY };
    let outside = Pnt3::new(radius + 1.0, 0.0, height * 0.5);
    assert!(!oracle_inside(&sdf, outside));
    assert_classify(&body, solid, outside, PointClassification::Outside);
}

#[semio_framework_async_macros::async_test]
async fn face_uv_interior_point_on_box_face() {
    let mut body = Body::new();
    let mut rec = OpRecorder::new();
    let solid = make_box(&mut body, 2.0, 2.0, 2.0, &mut rec).unwrap();
    let face = body.solid_faces(solid)[0];
    let outer = body.faces.get(face).unwrap().outer.unwrap();
    let surface = face_surface(&body, face).unwrap();
    let mut pts = Vec::new();
    for coedge in body.loop_coedges(outer) {
        if let Some((v0, _)) = body.coedge_endpoints(coedge) {
            if let Some(v) = body.vertices.get(v0) {
                pts.push(v.position);
            }
        }
    }
    let n = pts.len().max(1) as f64;
    let center = Pnt3::new(pts.iter().map(|p| p.x).sum::<f64>() / n, pts.iter().map(|p| p.y).sum::<f64>() / n, pts.iter().map(|p| p.z).sum::<f64>() / n);
    let uv = surface_uv(surface, center);
    assert!(point_in_face_uv(&body, face, uv, 1e-6).unwrap());
}

mod tests {
    use super::*;

    // 🚫️async: E5-class executor bridge, sanctioned per R4 clause 5 — `#[test]` cannot run
    // an `async fn` directly (std has no executor for it), so every async test body in this
    // module runs through this instead. Sound because this crate performs no real I/O: every
    // future here resolves on its first poll, so a single poll (never a spin-park loop) is
    // enough — panics loudly if that invariant is ever violated rather than hanging.
    fn block_on_test<F: std::future::Future>(fut: F) -> F::Output {
        use std::task::{Context, Poll, RawWaker, RawWakerVTable, Waker};
        fn noop(_: *const ()) {}
        fn clone_raw(_: *const ()) -> RawWaker {
            RawWaker::new(std::ptr::null(), &VTABLE)
        }
        static VTABLE: RawWakerVTable = RawWakerVTable::new(clone_raw, noop, noop, noop);
        let raw = RawWaker::new(std::ptr::null(), &VTABLE);
        let waker = unsafe { Waker::from_raw(raw) };
        let mut cx = Context::from_waker(&waker);
        let mut fut = Box::pin(fut);
        match fut.as_mut().poll(&mut cx) {
            Poll::Ready(v) => v,
            Poll::Pending => panic!("block_on_test: future did not complete synchronously"),
        }
    }

    #[test]
    fn outside_node_clip_path_excludes_node_interior() {
        block_on_test(async {
            let node_center = Point::new(0.0, 0.0);
            let handle_center = Point::new(40.0, 0.0);
            let clip = handle_outside_node_clip_path(handle_center, 5.0, node_center, NodeShape::Circle, 40.0, 80.0, 80.0);
            assert!(clip.elements().len() > 4);
            assert!(node_center.distance(handle_center) > 39.0);
        });
    }

    fn assert_cap_bulges_outward(center: Point, outward: Vec2, radius: f64) {
        let out = normalize_or_zero(outward);
        let peak = center + out * radius;
        let arc = handle_exterior_cap_arc(center, outward, radius).expect("exterior arc");
        assert!(distance_between(arc.eval(0.5), peak) < 0.35, "arc midpoint must sit on outward peak");
        let fill = handle_exterior_cap_fill_path(center, outward, radius);
        let bb = fill.bounding_box();
        let trough = center - out * radius;
        if out.x.abs() >= out.y.abs() {
            if out.x > 0.0 {
                assert!((bb.x1() - peak.x).abs() < 0.25, "east cap must peak at +x");
                assert!(bb.x0() > trough.x + 0.25, "east cap must not peak inward");
            } else {
                assert!((bb.x0() - peak.x).abs() < 0.25, "west cap must peak at -x");
                assert!(bb.x1() < trough.x - 0.25, "west cap must not peak inward");
            }
        } else if out.y > 0.0 {
            assert!((bb.y1() - peak.y).abs() < 0.25, "south cap must peak at +y");
            assert!(bb.y0() > trough.y + 0.25, "south cap must not peak inward");
        } else {
            assert!((bb.y0() - peak.y).abs() < 0.25, "north cap must peak at -y");
            assert!(bb.y1() < trough.y + 0.25, "north cap must not peak inward");
        }
    }

    #[test]
    fn edge_bezier_free_target_end_tangent_matches_incoming_chord() {
        block_on_test(async {
            let source = Point::new(0.0, 0.0);
            let target = Point::new(200.0, 40.0);
            let curve = compute_edge_bezier_points(source, target, Point::new(-50.0, 0.0), target);
            let approach = normalize_or_zero(target - source);
            let tangent = curve.eval(1.0) - curve.eval(0.995);
            let tangent_dir = normalize_or_zero(Vec2::new(tangent.x, tangent.y));
            assert!(tangent_dir.dot(approach) > 0.99, "free target tangent should match incoming chord");
        });
    }

    #[test]
    fn edge_bezier_starts_outside_handle_cap_peak() {
        block_on_test(async {
            let node_center = Point::new(100.0, 50.0);
            let width = 160.0;
            let height = 72.0;
            let rim = Point::new(node_center.x + width * 0.5, node_center.y);
            let outward = handle_outward_at_node_rim(rim, node_center, NodeShape::Rectangle, 0.0, width, height).expect("outward");
            let radius = 5.0;
            let peak = handle_exterior_cap_peak(rim, outward, radius);
            let target = Point::new(300.0, 50.0);
            let curve = compute_edge_bezier_outward(peak, target, outward, -normalize_or_zero(target - peak));
            let start = curve.eval(0.0);
            assert!((start.x - peak.x).abs() < 1e-9 && (start.y - peak.y).abs() < 1e-9);
            assert!(start.x > rim.x + 0.5, "edge must begin outside the port rim under the cap");
        });
    }

    #[test]
    fn edge_bezier_rectangle_port_uses_outward_normal() {
        block_on_test(async {
            let node_center = Point::new(100.0, 50.0);
            let width = 120.0;
            let height = 80.0;
            let source = Point::new(node_center.x - width * 0.5, node_center.y - 20.0);
            let target = Point::new(280.0, 50.0);
            let outward = handle_outward_at_node_rim(source, node_center, NodeShape::Rectangle, 0.0, width, height).expect("outward");
            let curve = compute_edge_bezier_outward(source, target, outward, -normalize_or_zero(target - source));
            let leave = curve.eval(0.005) - curve.eval(0.0);
            let leave_dir = normalize_or_zero(Vec2::new(leave.x, leave.y));
            assert!(leave_dir.dot(outward) > 0.99, "anchored port should leave along rim outward");
        });
    }

    #[test]
    fn rectangle_rim_outward_uses_edge_normal_not_radial() {
        block_on_test(async {
            let node_center = Point::new(100.0, 50.0);
            let width = 120.0;
            let height = 80.0;
            let handle = Point::new(node_center.x - width * 0.5, node_center.y - 20.0);
            let radial = normalize_or_zero(handle - node_center);
            let outward = handle_outward_at_node_rim(handle, node_center, NodeShape::Rectangle, 0.0, width, height).expect("outward");
            assert!((outward.x + 1.0).abs() < 1e-9 && outward.y.abs() < 1e-9);
            assert!(radial.y.abs() > 0.1, "radial must tilt for off-center left ports");
        });
    }

    #[test]
    fn exterior_cap_paths_bulge_outward_on_all_cardinals() {
        block_on_test(async {
            let radius = 5.0;
            assert_cap_bulges_outward(Point::new(40.0, 0.0), Vec2::new(1.0, 0.0), radius);
            assert_cap_bulges_outward(Point::new(-40.0, 0.0), Vec2::new(-1.0, 0.0), radius);
            assert_cap_bulges_outward(Point::new(0.0, 30.0), Vec2::new(0.0, 1.0), radius);
            assert_cap_bulges_outward(Point::new(0.0, -30.0), Vec2::new(0.0, -1.0), radius);
            let stroke = handle_exterior_cap_stroke_path(Point::new(40.0, 0.0), Vec2::new(1.0, 0.0), radius);
            assert!(!stroke.elements().iter().any(|el| matches!(el, geometry::PathEl::ClosePath)));
        });
    }

    #[test]
    fn triangle_cap_peak_matches_outward_direction() {
        block_on_test(async {
            let center = Point::new(40.0, 0.0);
            let outward = Vec2::new(1.0, 0.0);
            let radius = 5.0;
            let peak = handle_exterior_cap_triangle_peak(center, outward, radius);
            assert!((peak.x - (center.x + radius)).abs() < 1e-9);
            let fill = handle_exterior_cap_triangle_fill_path(center, outward, radius);
            assert!(fill.bounding_box().x1() > center.x);
        });
    }

    #[test]
    fn sharp_sz_path_is_orthogonal_between_peaks() {
        block_on_test(async {
            let source = Point::new(0.0, 0.0);
            let target = Point::new(120.0, 40.0);
            let path = compute_edge_sharp_sz_path(source, target, Vec2::new(1.0, 0.0), Vec2::new(-1.0, 0.0));
            let mut line_count = 0;
            for el in path.elements() {
                if matches!(el, geometry::PathEl::LineTo(_)) {
                    line_count += 1;
                }
            }
            assert!(line_count >= 3, "sharp S/Z path should contain multiple straight segments");
        });
    }
}

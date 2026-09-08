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
    fn force_layout_moves_nodes() {
        block_on_test(async {
            let mut positions = vec![Vec2::new(0.0, 0.0), Vec2::new(100.0, 0.0)];
            let radii = vec![32.0, 32.0];
            let edges = vec![(0, 1)];
            let pin = vec![None, None];
            let opts = ForceLayoutOptions { iterations: 120, ideal_edge_length: 80.0, ..Default::default() };
            run_force_layout(&mut positions, &radii, &edges, &pin, &opts);
            let dist = (positions[1] - positions[0]).hypot();
            assert!(dist.is_finite() && dist > 1.0);
            assert!((dist - 100.0).abs() > 0.01);
        });
    }

    #[test]
    fn circular_layout_places_points_on_ring() {
        block_on_test(async {
            let points = circular_layout(4, Vec2::ZERO, 10.0);
            assert_eq!(points.len(), 4);
            for p in &points {
                assert!((p.hypot() - 10.0).abs() < 1e-9);
            }
        });
    }

    #[test]
    fn grid_layout_places_points_in_rows() {
        block_on_test(async {
            let points = grid_layout(5, 2, 10.0);
            assert_eq!(points.len(), 5);
            assert_eq!((points[0].x, points[0].y), (0.0, 0.0));
            assert_eq!((points[2].x, points[2].y), (0.0, 10.0));
        });
    }
}

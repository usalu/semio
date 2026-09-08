
use super::*;

#[test]
fn bespoke_surfaces_are_excluded_from_generic_dispatch() {
    for kind in [SurfaceKind::World3d, SurfaceKind::NodeGraph, SurfaceKind::TiledMap, SurfaceKind::Board2d] {
        assert!(scene_has_bespoke_pointer_dispatch(kind), "{kind:?} should stay on its own bespoke host");
    }
    for kind in [SurfaceKind::Canvas2d, SurfaceKind::Paint2d, SurfaceKind::TextEditor, SurfaceKind::InkCanvas, SurfaceKind::GraphTimeline, SurfaceKind::Table, SurfaceKind::VirtualFileSystem, SurfaceKind::DiffView, SurfaceKind::EventFeed] {
        assert!(!scene_has_bespoke_pointer_dispatch(kind), "{kind:?} previously received no interaction at all and must use the generic handlers");
    }
}

//! 🎓️ Scratch discovery pass (W7a).

use super::*;

#[test]
fn w7a_discovery_full_walk() {
    let theme = Theme::light();
    let mut shell = super::appearance_tour_and_footer_pill_tests::tour_shell(Some(super::appearance_tour_and_footer_pill_tests::tour_introduction()));
    shell.screen_w = 1440.0;
    shell.screen_h = 900.0;
    shell.auto_start_introduction("tour-app", false);
    assert!(shell.chrome_build.tour_state.is_some());

    let mut draw = DrawList::default();
    let mut overlay = DrawList::default();
    let mut atlas = FontAtlas::builtin();
    let icons = IconAtlas::default();
    let mut input = InputState::<ActionDescriptor>::default();
    let mut world_resources = infinite_world::world::World3dBuildContext::new(infinite_world::world::WorldCursorWakeAuthority::new());
    let mut cursor = ShellChromeFrameCursor::default();
    let mut steps = 0usize;
    let complete = (0..(1usize << 20)).any(|_| {
        steps += 1;
        shell.render_chrome_step(&mut cursor, &mut draw, &mut overlay, &mut atlas, &icons, &mut input, &theme, &mut world_resources)
    });
    let ids: Vec<String> = input.staged_hits().iter().filter_map(|hit| hit.control_id.clone()).collect();
    let tour_ids: Vec<&String> = ids.iter().filter(|id| id.starts_with("shell.tour")).collect();
    let overlay_quads: usize = overlay.layers.iter().map(|layer| layer.ui_instances.len()).sum();
    let full_screen = overlay.layers.iter().flat_map(|layer| layer.ui_instances.iter()).filter(|instance| instance.rect[2] >= 1439.0 && instance.rect[3] >= 899.0).count();
    let card: Vec<[f32; 4]> = overlay.layers.iter().flat_map(|layer| layer.ui_instances.iter()).map(|instance| instance.rect).filter(|rect| rect[0] >= 550.0 && rect[0] <= 900.0 && rect[1] >= 360.0 && rect[1] <= 540.0).collect();
    panic!("card={card:?} all_ids={ids:?} complete={complete} steps={steps} overlay_quads={overlay_quads} glass={} veil_quads={full_screen} tour_ids={tour_ids:?} tour_state={:?} error={:?}", overlay.glass_regions.len(), shell.chrome_build.tour_state.as_ref().map(|state| state.step_index), shell.error);
}

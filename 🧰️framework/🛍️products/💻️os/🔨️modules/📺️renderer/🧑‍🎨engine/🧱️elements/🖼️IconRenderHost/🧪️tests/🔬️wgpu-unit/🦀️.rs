//! 🔁️ The bounded icon re-rasterisation a post-boot density change drives — ticket
//! 26/09/17/WGPU-RENDERER-REACT-PARITY packet W2k, closing packet W1g's gap 1 ("icon atlas does not
//! re-rasterise on a post-boot scale change"). The glyph atlas already re-rasterises lazily per
//! glyph; this is the icon half, paid across frames instead of in one resize step.

use super::*;

#[test]
fn a_rebuild_quantises_its_cell_to_the_same_ramp_the_one_shot_build_uses() {
    assert_eq!(IconAtlasRebuild::new(1.0).cell_size(), ICON_SIZE);
    assert_eq!(IconAtlasRebuild::new(2.0).cell_size(), ICON_SIZE * 2);
    assert_eq!(IconAtlasRebuild::new(3.0).cell_size(), ICON_SIZE * 3);
    assert_eq!(IconAtlasRebuild::new(4.0).cell_size(), ICON_SIZE * 3, "the fixed 2048 UV space with 16 columns caps the raster scale at 3×");
    assert_eq!(IconAtlasRebuild::new(1.4).cell_size(), ICON_SIZE, "a fractional factor rounds, so 1.4× keeps the 1× cell");
    assert_eq!(IconAtlasRebuild::new(f32::NAN).cell_size(), ICON_SIZE, "a non-finite factor degrades to 1× rather than panicking");
}

#[test]
fn a_rebuild_reports_its_own_scale_and_starts_with_nothing_rasterised() {
    let rebuild = IconAtlasRebuild::new(2.0);
    assert_eq!(rebuild.scale_factor(), 2.0);
    assert_eq!(rebuild.progress(), (0, icon_atlas_source_count()));
    assert!(icon_atlas_source_count() > ICON_RASTER_STEP_BUDGET, "the budget must actually split the work, or the job is pointless");
}

#[test]
fn each_step_spends_at_most_its_budget_and_the_last_one_answers_the_packed_atlas() {
    let total = icon_atlas_source_count();
    let mut rebuild = IconAtlasRebuild::new(1.0);
    let mut steps = 0;
    let atlas = loop {
        steps += 1;
        assert!(steps <= total + 1, "a rebuild must terminate in at most one step per source");
        match rebuild.step(ICON_RASTER_STEP_BUDGET) {
            IconAtlasRebuildStep::Rasterized(spent) => {
                assert!(spent <= ICON_RASTER_STEP_BUDGET, "a step never overspends its budget");
                assert!(rebuild.progress().0 <= total);
            }
            IconAtlasRebuildStep::Complete(atlas) => break atlas,
        }
    };
    assert!(steps > 1, "the whole atlas must not land in a single step — that is the interactive deadline this job exists to protect");
    assert_eq!(rebuild.progress(), (total, total));
    assert_eq!(atlas.width, ATLAS_COLS * ICON_SIZE);
    assert!(atlas.height >= ICON_SIZE);
    assert!(atlas.icon_uv("semio-logo").is_some(), "the logo is the source the one-shot build appends last");
}

#[test]
fn a_zero_budget_still_makes_progress_so_a_rebuild_can_never_stall() {
    let mut rebuild = IconAtlasRebuild::new(1.0);
    assert!(matches!(rebuild.step(0), IconAtlasRebuildStep::Rasterized(1)));
    assert_eq!(rebuild.progress().0, 1);
}

#[test]
fn a_rebuild_driven_to_completion_is_byte_identical_to_the_one_shot_build() {
    let one_shot = build_icon_atlas_scaled(1.0);
    let mut rebuild = IconAtlasRebuild::new(1.0);
    let stepped = loop {
        if let IconAtlasRebuildStep::Complete(atlas) = rebuild.step(7) {
            break atlas;
        }
    };
    assert_eq!((stepped.width, stepped.height), (one_shot.width, one_shot.height));
    assert_eq!(stepped.pixels, one_shot.pixels, "the stepped and one-shot paths are the SAME rasterisation, so a density change cannot drift from a boot");
}

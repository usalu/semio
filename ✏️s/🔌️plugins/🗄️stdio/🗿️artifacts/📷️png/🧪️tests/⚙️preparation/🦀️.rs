//! 🧪️ Runtime law: the bespoke pixel `publication` preparation reaches `Prepared` through the real sealer, publishes the exact post snapshot and an inverse that restores the base.

#[global_allocator]
static ORIGINAL_OWNER_HEAP: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

use semio_framework_os_kernel as store;
use semio_framework_plugin::ArtifactEditor;
use semio_s_artifact_stdio_png::editor::png::PngEditor;
use semio_s_artifact_stdio_png::schema::mutations::ReplaceImage;
use semio_s_artifact_stdio_png::{apply_mutation, PngMutation, PngSnapshot};

#[test]
fn publication_preparation_publishes_the_post_snapshot_and_a_restoring_inverse() {
    let mut base = PngSnapshot::default();
    base.image.width = 2;
    base.image.height = 1;
    base.image.samples = vec![10, 20, 30, 255, 40, 50, 60, 255];
    let mut edited = base.clone();
    edited.image.samples[0] = 11;
    assert_ne!(base, edited);
    let mutation = PngMutation::ReplaceImage(ReplaceImage { image: edited.image.clone() });
    let factory = <PngEditor as ArtifactEditor>::build_artifact_store_one_item_preparation_factory().expect("the PNG document lane declares a preparation factory");
    let grant = store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: 1 << 16, maximum_capacity_bytes: 1 << 26, maximum_release_bytes: 1 << 26, maximum_depth: 4096 };
    let law = store::snapshot_clone_preparation::drive_one_item_preparation_law(&factory, base.clone(), mutation.clone(), grant);
    let mut expected = base.clone();
    apply_mutation(&mut expected, &mutation);
    assert_eq!(*law.post, expected);
    assert_eq!(law.post.image, edited.image);
    let mut restored = (*law.post).clone();
    for row in &law.inverse {
        apply_mutation(&mut restored, row);
    }
    assert_eq!(restored, base);
}

#[test]
fn publication_preparation_edits_an_image_beyond_the_one_item_cap() {
    use semio_s_artifact_stdio_png::schema::mutations::PaintNativeSamplesMutation;
    use semio_s_artifact_stdio_png::schema::operations::png_revision;
    use semio_s_artifact_stdio_png::schema::snapshot::{PngNativePaint, PngRegion};
    let mut base = PngSnapshot::default();
    base.image.width = 512;
    base.image.height = 264;
    base.image.samples = (0..512usize * 264 * 4).map(|index| (index % 251) as u16).collect();
    assert!(base.image.samples.len() * 2 > store::ARTIFACT_STORE_ONE_ITEM_MAXIMUM_BYTES, "fixture exceeds the one-item cap");
    let region = PngRegion { x: 3, y: 5, width: 2, height: 2 };
    let mutation = PngMutation::PaintNativeSamples(PaintNativeSamplesMutation { revision: png_revision(&base), region, paint: PngNativePaint::rgba(200, 100, 50, 255) });
    let factory = <PngEditor as ArtifactEditor>::build_artifact_store_one_item_preparation_factory().expect("the PNG document lane declares a preparation factory");
    let grant = store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: 1 << 16, maximum_capacity_bytes: 1 << 26, maximum_release_bytes: 1 << 26, maximum_depth: 4096 };
    let law = store::snapshot_clone_preparation::drive_one_item_preparation_law(&factory, base.clone(), mutation.clone(), grant);
    let mut expected = base.clone();
    apply_mutation(&mut expected, &mutation);
    assert_eq!(*law.post, expected);
    let mut restored = (*law.post).clone();
    for row in &law.inverse {
        apply_mutation(&mut restored, row);
    }
    assert_eq!(restored, base);
    eprintln!("[DEBUG] png {}-byte image edited in {} turns, peak live {}", base.image.samples.len() * 2, law.turns, law.peak_live_capacity_bytes);
}

#[test]
fn publication_preparation_turns_are_bounded_by_pages_at_real_image_size() {
    use semio_framework_value::retained_clone::RETAINED_CLONE_BULK_PAGE_BYTES;
    use semio_s_artifact_stdio_png::schema::mutations::PaintNativeSamplesMutation;
    use semio_s_artifact_stdio_png::schema::operations::png_revision;
    use semio_s_artifact_stdio_png::schema::snapshot::{PngNativePaint, PngRegion};
    let grant = store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: 1 << 16, maximum_capacity_bytes: 1 << 26, maximum_release_bytes: 1 << 26, maximum_depth: 4096 };
    let started = std::time::Instant::now();
    let mut turns_by_size = Vec::new();
    for (width, height) in [(256u32, 128u32), (512, 264), (1024, 512)] {
        let mut base = PngSnapshot::default();
        base.image.width = width;
        base.image.height = height;
        base.image.samples = (0..width as usize * height as usize * 4).map(|index| (index % 251) as u16).collect();
        let bytes = base.image.samples.len() * 2;
        let region = PngRegion { x: 3, y: 5, width: 2, height: 2 };
        let mutation = PngMutation::PaintNativeSamples(PaintNativeSamplesMutation { revision: png_revision(&base), region, paint: PngNativePaint::rgba(200, 100, 50, 255) });
        let factory = <PngEditor as ArtifactEditor>::build_artifact_store_one_item_preparation_factory().expect("the PNG document lane declares a preparation factory");
        let law = store::snapshot_clone_preparation::drive_one_item_preparation_law(&factory, base.clone(), mutation.clone(), grant);
        let mut expected = base.clone();
        apply_mutation(&mut expected, &mutation);
        assert_eq!(*law.post, expected);
        let mut restored = (*law.post).clone();
        for row in &law.inverse {
            apply_mutation(&mut restored, row);
        }
        assert_eq!(restored, base);
        let pages = bytes.div_ceil(RETAINED_CLONE_BULK_PAGE_BYTES);
        eprintln!("[DEBUG] png {width}x{height} RGBA16 {bytes} bytes = {pages} pages edited in {} turns (turns per page {:.1}), {:?}", law.turns, law.turns as f64 / pages as f64, started.elapsed());
        turns_by_size.push((pages, law.turns));
    }
    let (small_pages, small_turns) = turns_by_size[0];
    let (large_pages, large_turns) = turns_by_size[2];
    assert!(large_pages >= 8 * small_pages);
    assert!(large_turns - small_turns <= 8 * (large_pages - small_pages), "turns grow at most 8 per additional page: {turns_by_size:?}");
    assert!(large_turns < 12_000, "a 4 MiB image edit is O(pages), not O(samples): {large_turns} turns");
}

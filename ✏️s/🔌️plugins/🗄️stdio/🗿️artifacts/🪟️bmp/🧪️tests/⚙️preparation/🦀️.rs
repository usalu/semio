//! 🧪️ Runtime law: the bespoke pixel `publication` preparation reaches `Prepared` through the real sealer, publishes the exact post snapshot and an inverse that restores the base.

#[global_allocator]
static ORIGINAL_OWNER_HEAP: semio_framework_trace::HeapWitness = semio_framework_trace::HeapWitness;

use semio_framework_os_kernel as store;
use semio_framework_pack_json::{from_json_str, JsonMemberPolicy};
use semio_framework_plugin::ArtifactEditor;
use semio_s_artifact_stdio_bmp::editor::bmp::BmpEditor;
use semio_s_artifact_stdio_bmp::schema::mutations::replace_image::ReplaceImage;
use semio_s_artifact_stdio_bmp::{apply_mutation, BmpMutation, BmpSnapshot};

#[test]
fn publication_preparation_publishes_the_post_snapshot_and_a_restoring_inverse() {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️owned-native-samples/🔣️.json")).expect("fixture is JSON");
    let row = &fixture["cases"][0];
    let base: BmpSnapshot = from_json_str(&row["snapshot"].to_string(), JsonMemberPolicy::Reject).expect("fixture snapshot decodes");
    let mut edited_json = row["snapshot"].clone();
    edited_json["image"]["pixels"]["samples"][0]["red"] = serde_json::json!(1);
    let edited: BmpSnapshot = from_json_str(&edited_json.to_string(), JsonMemberPolicy::Reject).expect("edited snapshot decodes");
    assert_ne!(base, edited);
    let mutation = BmpMutation::ReplaceImage(ReplaceImage { image: edited.image.clone() });
    let factory = <BmpEditor as ArtifactEditor>::build_artifact_store_one_item_preparation_factory().expect("the BMP document lane declares a preparation factory");
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

fn direct_bitmap(width: u32, height: u32) -> BmpSnapshot {
    let fixture: serde_json::Value = serde_json::from_str(include_str!("../../🏅️standards/🔖️v3/🪆️subsets/✳️any/🧫️fixtures/🧬️owned-native-samples/🔣️.json")).expect("fixture is JSON");
    let mut json = fixture["cases"][0]["snapshot"].clone();
    json["image"]["width"] = serde_json::json!(width);
    json["image"]["height"] = serde_json::json!(height);
    json["image"]["pixels"]["samples"] = serde_json::Value::Array((0..width as usize * height as usize).map(|index| serde_json::json!({ "red": index % 32, "green": (index * 3) % 64, "blue": (index * 5) % 32, "alpha": 0, "reserved": 0 })).collect());
    from_json_str(&json.to_string(), JsonMemberPolicy::Reject).expect("scaled snapshot decodes")
}

#[test]
fn publication_preparation_turns_are_bounded_by_pages_and_paged_paints_restore() {
    use semio_framework_value::retained_clone::RETAINED_CLONE_BULK_PAGE_BYTES;
    use semio_s_artifact_stdio_bmp::schema::mutations::PaintDirectRegion;
    use semio_s_artifact_stdio_bmp::schema::operations::bmp_revision;
    let grant = store::ArtifactStoreOneItemGrant { maximum_items: 1, maximum_copy_bytes: 1 << 16, maximum_capacity_bytes: 1 << 26, maximum_release_bytes: 1 << 26, maximum_depth: 4096 };
    let sample_bytes = std::mem::size_of::<semio_s_artifact_stdio_bmp::schema::snapshot::BmpNativeSample>();
    let mut turns_by_size = Vec::new();
    for (width, height, region) in [(128u32, 64u32, (3u32, 5u32, 2u32, 2u32)), (512, 256, (3, 5, 2, 2)), (128, 64, (10, 4, 64, 40))] {
        let base = direct_bitmap(width, height);
        let mutation = BmpMutation::PaintDirectRegion(PaintDirectRegion { revision: bmp_revision(&base), x: region.0, y: region.1, width: region.2, height: region.3, red: 255, green: 0, blue: 0, alpha: 255 });
        let factory = <BmpEditor as ArtifactEditor>::build_artifact_store_one_item_preparation_factory().expect("the BMP document lane declares a preparation factory");
        let law = store::snapshot_clone_preparation::drive_one_item_preparation_law(&factory, base.clone(), mutation.clone(), grant);
        let mut expected = base.clone();
        apply_mutation(&mut expected, &mutation);
        assert_eq!(*law.post, expected);
        assert_ne!(*law.post, base);
        let mut restored = (*law.post).clone();
        for row in &law.inverse {
            apply_mutation(&mut restored, row);
        }
        assert_eq!(restored, base);
        let pages = (width as usize * height as usize * sample_bytes).div_ceil(RETAINED_CLONE_BULK_PAGE_BYTES);
        eprintln!("[DEBUG] bmp {width}x{height} paint {region:?}: {pages} pages edited in {} turns", law.turns);
        turns_by_size.push((pages, law.turns));
    }
    let (small_pages, small_turns) = turns_by_size[0];
    let (large_pages, large_turns) = turns_by_size[1];
    assert!(large_pages >= 8 * small_pages);
    assert!(large_turns.saturating_sub(small_turns) <= 8 * (large_pages - small_pages), "turns grow at most 8 per additional page: {turns_by_size:?}");
    assert!(large_turns < 30_000, "a 2.6 MiB bitmap edit is O(pages), not O(samples): {large_turns} turns");
}

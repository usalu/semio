/// 🔡️ The ladder half of the same defect: no draw tier may shorten a caption's CONTENT, so zooming
/// in never replaces a name with a shorter word.
#[test]
fn no_draw_tier_serves_a_shorter_caption_than_the_tier_below_it() {
    use super::{DagDrawLod, DagNodeLabel};
    let ladder = [DagDrawLod::Minimap, DagDrawLod::Overview, DagDrawLod::Compact, DagDrawLod::Normal, DagDrawLod::Detail, DagDrawLod::Micro];
    let mut captioned = false;
    for lod in ladder {
        match lod.node_label() {
            DagNodeLabel::None => assert!(!captioned, "{} dropped a caption a lower tier already showed", lod.label()),
            DagNodeLabel::Name => captioned = true,
        }
    }
    assert!(captioned, "no draw tier captions a node at all");
}

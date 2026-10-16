//! 🧪️ A host that delivers an unreadable icon pack leaves the guest port empty instead of failing: every accessor answers `None`, as on a
//! host that delivers nothing. One process, one test: the port installs once.

use semio_framework_canvas_assets::{emoji_font, icon_pack, label_font, register_guest_asset_reader, RuntimeAsset};

fn delivered(name: &str) -> Option<&'static [u8]> {
    if name == RuntimeAsset::IconPack.package_asset_name() {
        Some(&b"{"[..])
    } else {
        Some(&b"face"[..])
    }
}

#[test]
fn an_unparseable_delivered_icon_pack_is_refused_and_the_port_stays_empty() {
    register_guest_asset_reader(delivered);
    assert!(icon_pack().is_none());
    assert!(emoji_font().is_none() && label_font().is_none());
    assert!(icon_pack().is_none(), "a refused delivery is not retried");
}

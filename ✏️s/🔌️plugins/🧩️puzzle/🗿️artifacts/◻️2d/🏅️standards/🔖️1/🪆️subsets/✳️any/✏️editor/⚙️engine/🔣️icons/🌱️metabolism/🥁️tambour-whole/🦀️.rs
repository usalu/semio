//! 🖼️ Metabolism icons (tambour-whole) bundled as SVG sources.

/// 🧬️ Resolves one canonical metabolism icon identity of this family to its bundled SVG source.
pub fn icon_svg(key: &str) -> Option<&'static str> {
    match key {
        "cylindric-tambour" => Some(include_str!("../../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/🔋️cylindric-tambours/🧱️standard/🖋️cylindric-tambour.svg")),
        "tambour" => Some(include_str!("../../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/🥁️tambours/🧱️standard/🖋️tambour.svg")),
        _ => None,
    }
}

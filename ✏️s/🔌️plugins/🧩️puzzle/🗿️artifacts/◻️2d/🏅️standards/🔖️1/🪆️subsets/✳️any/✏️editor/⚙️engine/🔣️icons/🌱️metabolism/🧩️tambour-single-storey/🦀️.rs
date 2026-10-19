//! 🖼️ Metabolism icons (tambour-single-storey) bundled as SVG sources.

/// 🧬️ Resolves one canonical metabolism icon identity of this family to its bundled SVG source.
pub fn icon_svg(key: &str) -> Option<&'static str> {
    match key {
        "tambour_single-storey" => Some(include_str!("../../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/🥁️tambours/🏠️single-storey/🖋️tambour_single-storey.svg")),
        "cylindric-tambour_single-storey" => Some(include_str!("../../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/🔋️cylindric-tambours/🏠️single-storey/🖋️cylindric-tambour_single-storey.svg")),
        "base" => Some(include_str!("../../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/🧱️bases/⬛️plain-shape/🖋️base.svg")),
        "base_blob" => Some(include_str!("../../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/🧱️bases/🫧️blob/🖋️base_blob.svg")),
        "metabolism" => Some(include_str!("../../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/🌱️metabolism.svg")),
        "cylindric-capital" => Some(include_str!("../../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/🏛️capitals/🔘️cylindric/🖋️cylindric-capital.svg")),
        "capital" => Some(include_str!("../../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/🏛️capitals/▫️square/🖋️capital.svg")),
        _ => None,
    }
}

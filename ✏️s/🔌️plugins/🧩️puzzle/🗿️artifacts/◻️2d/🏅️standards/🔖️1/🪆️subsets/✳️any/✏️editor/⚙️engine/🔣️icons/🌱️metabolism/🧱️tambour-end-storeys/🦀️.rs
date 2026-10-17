//! 🖼️ Metabolism icons (tambour-end-storeys) bundled as SVG sources.

/// 🧬️ Resolves one canonical metabolism icon identity of this family to its bundled SVG source.
pub fn icon_svg(key: &str) -> Option<&'static str> {
    match key {
        "cylindric-tambour_last-storey" => Some(include_str!("../../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/🔋️cylindric-tambours/🏁️last-storey/🖋️cylindric-tambour_last-storey.svg")),
        "tambour_last-storey" => Some(include_str!("../../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/🥁️tambours/🏁️last-storey/🖋️tambour_last-storey.svg")),
        "cylindric-tambour_first-storey" => Some(include_str!("../../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/🔋️cylindric-tambours/🌱️first-storey/🖋️cylindric-tambour_first-storey.svg")),
        "tambour_first-storey" => Some(include_str!("../../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/🥁️tambours/🌱️first-storey/🖋️tambour_first-storey.svg")),
        _ => None,
    }
}

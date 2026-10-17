//! 🖼️ Metabolism icons (capsules) bundled as SVG sources.

/// 🧬️ Resolves one canonical metabolism icon identity of this family to its bundled SVG source.
pub fn icon_svg(key: &str) -> Option<&'static str> {
    match key {
        "capsule_slash" => Some(include_str!("../../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/💊️capsules/↗️slash/🖋️capsule_slash.svg")),
        "capsule_backslash" => Some(include_str!("../../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/💊️capsules/↘️backslash/🖋️capsule_backslash.svg")),
        "capsule_z" => Some(include_str!("../../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/💊️capsules/⚡️z/🖋️capsule_z.svg")),
        "capsule_p" => Some(include_str!("../../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/💊️capsules/🅿️p/🖋️capsule_p.svg")),
        "capsule_s" => Some(include_str!("../../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/💊️capsules/🐍️s/🖋️capsule_s.svg")),
        "capsule_L" => Some(include_str!("../../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/💊️capsules/📐️l/🖋️capsule_L.svg")),
        "capsule_q" => Some(include_str!("../../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/💊️capsules/🔎️q/🖋️capsule_q.svg")),
        "capsule_J" => Some(include_str!("../../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/💊️capsules/🪝️j/🖋️capsule_J.svg")),
        _ => None,
    }
}

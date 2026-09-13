//! 🖼️ Static metabolism icon catalogue for the Puzzle 2d editor.

/// 🧬️ Resolves a canonical metabolism icon identity to its bundled SVG source.
pub fn board_metabolism_icon_svg(key: &str) -> Option<&'static str> {
    match key.trim() {
        "metabolism" => Some(include_str!("../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/🌱️metabolism.svg")),
        "capital" => Some(include_str!("../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/🏛️capitals/▫️square/🖋️capital.svg")),
        "cylindric-capital" => Some(include_str!("../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/🏛️capitals/🔘️cylindric/🖋️cylindric-capital.svg")),
        "capsule_slash" => Some(include_str!("../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/💊️capsules/↗️slash/🖋️capsule_slash.svg")),
        "capsule_backslash" => Some(include_str!("../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/💊️capsules/↘️backslash/🖋️capsule_backslash.svg")),
        "capsule_z" => Some(include_str!("../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/💊️capsules/⚡️z/🖋️capsule_z.svg")),
        "capsule_p" => Some(include_str!("../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/💊️capsules/🅿️p/🖋️capsule_p.svg")),
        "capsule_s" => Some(include_str!("../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/💊️capsules/🐍️s/🖋️capsule_s.svg")),
        "capsule_L" => Some(include_str!("../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/💊️capsules/📐️l/🖋️capsule_L.svg")),
        "capsule_q" => Some(include_str!("../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/💊️capsules/🔎️q/🖋️capsule_q.svg")),
        "capsule_J" => Some(include_str!("../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/💊️capsules/🪝️j/🖋️capsule_J.svg")),
        "cylindric-tambour_first-storey" => Some(include_str!("../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/🔋️cylindric-tambours/🌱️first-storey/🖋️cylindric-tambour_first-storey.svg")),
        "cylindric-tambour_last-storey" => Some(include_str!("../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/🔋️cylindric-tambours/🏁️last-storey/🖋️cylindric-tambour_last-storey.svg")),
        "cylindric-tambour_single-storey" => Some(include_str!("../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/🔋️cylindric-tambours/🏠️single-storey/🖋️cylindric-tambour_single-storey.svg")),
        "cylindric-tambour" => Some(include_str!("../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/🔋️cylindric-tambours/🧱️standard/🖋️cylindric-tambour.svg")),
        "tambour_first-storey" => Some(include_str!("../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/🥁️tambours/🌱️first-storey/🖋️tambour_first-storey.svg")),
        "tambour_last-storey" => Some(include_str!("../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/🥁️tambours/🏁️last-storey/🖋️tambour_last-storey.svg")),
        "tambour_single-storey" => Some(include_str!("../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/🥁️tambours/🏠️single-storey/🖋️tambour_single-storey.svg")),
        "tambour" => Some(include_str!("../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/🥁️tambours/🧱️standard/🖋️tambour.svg")),
        "ellipsoid-capsule_slash" => Some(include_str!("../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/🥚️ellipsoids/↗️slash/🖋️ellipsoid-capsule_slash.svg")),
        "ellipsoid-capsule_backslash" => Some(include_str!("../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/🥚️ellipsoids/↘️backslash/🖋️ellipsoid-capsule_backslash.svg")),
        "ellipsoid-capsule_z" => Some(include_str!("../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/🥚️ellipsoids/⚡️z/🖋️ellipsoid-capsule_z.svg")),
        "ellipsoid-capsule_p" => Some(include_str!("../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/🥚️ellipsoids/🅿️p/🖋️ellipsoid-capsule_p.svg")),
        "ellipsoid-capsule_s" => Some(include_str!("../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/🥚️ellipsoids/🐍️s/🖋️ellipsoid-capsule_s.svg")),
        "ellipsoid-capsule_L" => Some(include_str!("../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/🥚️ellipsoids/📐️l/🖋️ellipsoid-capsule_L.svg")),
        "ellipsoid-capsule_q" => Some(include_str!("../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/🥚️ellipsoids/🔎️q/🖋️ellipsoid-capsule_q.svg")),
        "ellipsoid-capsule_J" => Some(include_str!("../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/🥚️ellipsoids/🪝️j/🖋️ellipsoid-capsule_J.svg")),
        "base" => Some(include_str!("../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/🧱️bases/⬛️base/🖋️base.svg")),
        "base_blob" => Some(include_str!("../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/🧱️bases/🫧️blob/🖋️base_blob.svg")),
        _ => None,
    }
}

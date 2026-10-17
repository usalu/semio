//! 🖼️ Metabolism icons (ellipsoid-capsules) bundled as SVG sources.

/// 🧬️ Resolves one canonical metabolism icon identity of this family to its bundled SVG source.
pub fn icon_svg(key: &str) -> Option<&'static str> {
    match key {
        "ellipsoid-capsule_slash" => Some(include_str!("../../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/🥚️ellipsoids/↗️slash/🖋️ellipsoid-capsule_slash.svg")),
        "ellipsoid-capsule_backslash" => Some(include_str!("../../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/🥚️ellipsoids/↘️backslash/🖋️ellipsoid-capsule_backslash.svg")),
        "ellipsoid-capsule_z" => Some(include_str!("../../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/🥚️ellipsoids/⚡️z/🖋️ellipsoid-capsule_z.svg")),
        "ellipsoid-capsule_p" => Some(include_str!("../../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/🥚️ellipsoids/🅿️p/🖋️ellipsoid-capsule_p.svg")),
        "ellipsoid-capsule_s" => Some(include_str!("../../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/🥚️ellipsoids/🐍️s/🖋️ellipsoid-capsule_s.svg")),
        "ellipsoid-capsule_L" => Some(include_str!("../../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/🥚️ellipsoids/📐️l/🖋️ellipsoid-capsule_L.svg")),
        "ellipsoid-capsule_q" => Some(include_str!("../../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/🥚️ellipsoids/🔎️q/🖋️ellipsoid-capsule_q.svg")),
        "ellipsoid-capsule_J" => Some(include_str!("../../../../../../../../../../../../../../🧰️framework/🔨️modules/🖼️assets/🌱️metabolism/🔣️icons/🥚️ellipsoids/🪝️j/🖋️ellipsoid-capsule_J.svg")),
        _ => None,
    }
}

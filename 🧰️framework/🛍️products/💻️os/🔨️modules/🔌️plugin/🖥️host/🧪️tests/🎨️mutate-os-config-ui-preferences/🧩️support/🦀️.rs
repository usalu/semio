/// 🏷️ Every committed vector's scenario id, in the catalog's own order — duplicated, not imported,
/// because the oracle-only build must not link the subject crate. The contract's mutation-coverage
/// gate keeps this list honest against the catalog. `sets-*` vectors are applied, `keeps-*` no-ops.
pub(super) const VECTORS: &[&str] = &[
    "sets-appearance",
    "keeps-appearance",
    "sets-layout",
    "keeps-layout",
    "sets-driver",
    "keeps-driver",
    "sets-custom-driver",
    "keeps-custom-driver",
    "sets-locale",
    "keeps-locale",
    "sets-terminology",
    "keeps-terminology",
    "sets-theme",
    "keeps-theme",
    "sets-custom-theme",
    "keeps-custom-theme",
    "sets-keybinding",
    "keeps-keybinding",
];

/// 🧫️ The committed `(before, mutation, after, outcome)` specification vector TEXT for one scenario —
/// this IS the independently handcrafted vector the no-oracle decision rests on, never recomputed.
pub(super) fn fixture_text(scenario: &str) -> (&'static str, &'static str, &'static str, &'static str) {
    match scenario {
        "sets-appearance" => (
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/🌗️set-appearance/✏️sets-appearance/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/🌗️set-appearance/✏️sets-appearance/🦠️mutation/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/🌗️set-appearance/✏️sets-appearance/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/🌗️set-appearance/✏️sets-appearance/🎯️outcome/🔣️.json"),
        ),
        "keeps-appearance" => (
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/🌗️set-appearance/🟰️keeps-appearance/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/🌗️set-appearance/🟰️keeps-appearance/🦠️mutation/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/🌗️set-appearance/🟰️keeps-appearance/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/🌗️set-appearance/🟰️keeps-appearance/🎯️outcome/🔣️.json"),
        ),
        "sets-layout" => (
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/📐️set-layout/✏️sets-layout/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/📐️set-layout/✏️sets-layout/🦠️mutation/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/📐️set-layout/✏️sets-layout/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/📐️set-layout/✏️sets-layout/🎯️outcome/🔣️.json"),
        ),
        "keeps-layout" => (
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/📐️set-layout/🟰️keeps-layout/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/📐️set-layout/🟰️keeps-layout/🦠️mutation/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/📐️set-layout/🟰️keeps-layout/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/📐️set-layout/🟰️keeps-layout/🎯️outcome/🔣️.json"),
        ),
        "sets-driver" => (
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/🕹️set-driver/✏️sets-driver/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/🕹️set-driver/✏️sets-driver/🦠️mutation/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/🕹️set-driver/✏️sets-driver/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/🕹️set-driver/✏️sets-driver/🎯️outcome/🔣️.json"),
        ),
        "keeps-driver" => (
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/🕹️set-driver/🟰️keeps-driver/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/🕹️set-driver/🟰️keeps-driver/🦠️mutation/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/🕹️set-driver/🟰️keeps-driver/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/🕹️set-driver/🟰️keeps-driver/🎯️outcome/🔣️.json"),
        ),
        "sets-custom-driver" => (
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/🚗️set-custom-driver/✏️sets-custom-driver/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/🚗️set-custom-driver/✏️sets-custom-driver/🦠️mutation/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/🚗️set-custom-driver/✏️sets-custom-driver/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/🚗️set-custom-driver/✏️sets-custom-driver/🎯️outcome/🔣️.json"),
        ),
        "keeps-custom-driver" => (
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/🚗️set-custom-driver/🟰️keeps-custom-driver/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/🚗️set-custom-driver/🟰️keeps-custom-driver/🦠️mutation/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/🚗️set-custom-driver/🟰️keeps-custom-driver/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/🚗️set-custom-driver/🟰️keeps-custom-driver/🎯️outcome/🔣️.json"),
        ),
        "sets-locale" => (
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/🗣️set-locale/✏️sets-locale/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/🗣️set-locale/✏️sets-locale/🦠️mutation/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/🗣️set-locale/✏️sets-locale/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/🗣️set-locale/✏️sets-locale/🎯️outcome/🔣️.json"),
        ),
        "keeps-locale" => (
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/🗣️set-locale/🟰️keeps-locale/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/🗣️set-locale/🟰️keeps-locale/🦠️mutation/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/🗣️set-locale/🟰️keeps-locale/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/🗣️set-locale/🟰️keeps-locale/🎯️outcome/🔣️.json"),
        ),
        "sets-terminology" => (
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/📖️set-terminology/✏️sets-terminology/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/📖️set-terminology/✏️sets-terminology/🦠️mutation/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/📖️set-terminology/✏️sets-terminology/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/📖️set-terminology/✏️sets-terminology/🎯️outcome/🔣️.json"),
        ),
        "keeps-terminology" => (
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/📖️set-terminology/🟰️keeps-terminology/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/📖️set-terminology/🟰️keeps-terminology/🦠️mutation/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/📖️set-terminology/🟰️keeps-terminology/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/📖️set-terminology/🟰️keeps-terminology/🎯️outcome/🔣️.json"),
        ),
        "sets-theme" => (
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/🖼️set-theme/✏️sets-theme/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/🖼️set-theme/✏️sets-theme/🦠️mutation/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/🖼️set-theme/✏️sets-theme/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/🖼️set-theme/✏️sets-theme/🎯️outcome/🔣️.json"),
        ),
        "keeps-theme" => (
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/🖼️set-theme/🟰️keeps-theme/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/🖼️set-theme/🟰️keeps-theme/🦠️mutation/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/🖼️set-theme/🟰️keeps-theme/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/🖼️set-theme/🟰️keeps-theme/🎯️outcome/🔣️.json"),
        ),
        "sets-custom-theme" => (
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/🎨️set-custom-theme/✏️sets-custom-theme/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/🎨️set-custom-theme/✏️sets-custom-theme/🦠️mutation/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/🎨️set-custom-theme/✏️sets-custom-theme/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/🎨️set-custom-theme/✏️sets-custom-theme/🎯️outcome/🔣️.json"),
        ),
        "keeps-custom-theme" => (
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/🎨️set-custom-theme/🟰️keeps-custom-theme/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/🎨️set-custom-theme/🟰️keeps-custom-theme/🦠️mutation/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/🎨️set-custom-theme/🟰️keeps-custom-theme/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/🎨️set-custom-theme/🟰️keeps-custom-theme/🎯️outcome/🔣️.json"),
        ),
        "sets-keybinding" => (
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/⌨️set-keybinding-override/✏️sets-keybinding/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/⌨️set-keybinding-override/✏️sets-keybinding/🦠️mutation/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/⌨️set-keybinding-override/✏️sets-keybinding/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/⌨️set-keybinding-override/✏️sets-keybinding/🎯️outcome/🔣️.json"),
        ),
        "keeps-keybinding" => (
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/⌨️set-keybinding-override/🟰️keeps-keybinding/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/⌨️set-keybinding-override/🟰️keeps-keybinding/🦠️mutation/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/⌨️set-keybinding-override/🟰️keeps-keybinding/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧪️tests/🧫️fixtures/🧬️mutations/⌨️set-keybinding-override/🟰️keeps-keybinding/🎯️outcome/🔣️.json"),
        ),
        other => panic!("mutate-os-config-ui-preferences: no specification vector registered for scenario {other:?}"),
    }
}

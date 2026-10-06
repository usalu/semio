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
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/✏️sets-appearance/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/✏️sets-appearance/🦠️mutation/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/✏️sets-appearance/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/✏️sets-appearance/🎯️outcome/🔣️.json"),
        ),
        "keeps-appearance" => (
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/🟰️keeps-appearance/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/🟰️keeps-appearance/🦠️mutation/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/🟰️keeps-appearance/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/🌗️set-appearance/🧫️fixtures/🟰️keeps-appearance/🎯️outcome/🔣️.json"),
        ),
        "sets-layout" => (
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/✏️sets-layout/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/✏️sets-layout/🦠️mutation/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/✏️sets-layout/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/✏️sets-layout/🎯️outcome/🔣️.json"),
        ),
        "keeps-layout" => (
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/🟰️keeps-layout/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/🟰️keeps-layout/🦠️mutation/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/🟰️keeps-layout/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/📐️set-layout/🧫️fixtures/🟰️keeps-layout/🎯️outcome/🔣️.json"),
        ),
        "sets-driver" => (
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/✏️sets-driver/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/✏️sets-driver/🦠️mutation/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/✏️sets-driver/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/✏️sets-driver/🎯️outcome/🔣️.json"),
        ),
        "keeps-driver" => (
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/🟰️keeps-driver/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/🟰️keeps-driver/🦠️mutation/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/🟰️keeps-driver/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/🕹️set-driver/🧫️fixtures/🟰️keeps-driver/🎯️outcome/🔣️.json"),
        ),
        "sets-custom-driver" => (
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/✏️sets-custom-driver/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/✏️sets-custom-driver/🦠️mutation/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/✏️sets-custom-driver/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/✏️sets-custom-driver/🎯️outcome/🔣️.json"),
        ),
        "keeps-custom-driver" => (
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/🟰️keeps-custom-driver/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/🟰️keeps-custom-driver/🦠️mutation/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/🟰️keeps-custom-driver/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/🚗️set-custom-driver/🧫️fixtures/🟰️keeps-custom-driver/🎯️outcome/🔣️.json"),
        ),
        "sets-locale" => (
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/✏️sets-locale/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/✏️sets-locale/🦠️mutation/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/✏️sets-locale/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/✏️sets-locale/🎯️outcome/🔣️.json"),
        ),
        "keeps-locale" => (
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/🟰️keeps-locale/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/🟰️keeps-locale/🦠️mutation/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/🟰️keeps-locale/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/🗣️set-locale/🧫️fixtures/🟰️keeps-locale/🎯️outcome/🔣️.json"),
        ),
        "sets-terminology" => (
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/✏️sets-terminology/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/✏️sets-terminology/🦠️mutation/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/✏️sets-terminology/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/✏️sets-terminology/🎯️outcome/🔣️.json"),
        ),
        "keeps-terminology" => (
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/🟰️keeps-terminology/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/🟰️keeps-terminology/🦠️mutation/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/🟰️keeps-terminology/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/📖️set-terminology/🧫️fixtures/🟰️keeps-terminology/🎯️outcome/🔣️.json"),
        ),
        "sets-theme" => (
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/✏️sets-theme/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/✏️sets-theme/🦠️mutation/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/✏️sets-theme/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/✏️sets-theme/🎯️outcome/🔣️.json"),
        ),
        "keeps-theme" => (
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/🟰️keeps-theme/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/🟰️keeps-theme/🦠️mutation/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/🟰️keeps-theme/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/🖼️set-theme/🧫️fixtures/🟰️keeps-theme/🎯️outcome/🔣️.json"),
        ),
        "sets-custom-theme" => (
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/✏️sets-custom-theme/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/✏️sets-custom-theme/🦠️mutation/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/✏️sets-custom-theme/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/✏️sets-custom-theme/🎯️outcome/🔣️.json"),
        ),
        "keeps-custom-theme" => (
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/🟰️keeps-custom-theme/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/🟰️keeps-custom-theme/🦠️mutation/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/🟰️keeps-custom-theme/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/🎨️set-custom-theme/🧫️fixtures/🟰️keeps-custom-theme/🎯️outcome/🔣️.json"),
        ),
        "sets-keybinding" => (
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/✏️sets-keybinding/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/✏️sets-keybinding/🦠️mutation/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/✏️sets-keybinding/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/✏️sets-keybinding/🎯️outcome/🔣️.json"),
        ),
        "keeps-keybinding" => (
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/🟰️keeps-keybinding/📸️snapshot/⬅️before/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/🟰️keeps-keybinding/🦠️mutation/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/🟰️keeps-keybinding/📸️snapshot/➡️after/🔣️.json"),
            include_str!("../../../../../../🎚️config/🧬️schema/🧬️mutations/⌨️set-keybinding-override/🧫️fixtures/🟰️keeps-keybinding/🎯️outcome/🔣️.json"),
        ),
        other => panic!("mutate-os-config-ui-preferences: no specification vector registered for scenario {other:?}"),
    }
}

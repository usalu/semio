use semio_repo_test_host::law::vector::Vector;

/// 🧫️ One committed vector of `apply-directory-page`, read literally from `✏️editor/🫧️transient/🧫️fixtures`.
pub(super) fn vector(id: &str) -> Result<Vector, String> {
    Ok(match id {
        "apply-directory-page-applied" => Vector {
                before: include_str!("../../../../✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/✅️apply/📸️snapshot/⬅️before/🔣️.json"),
                mutation: include_str!("../../../../✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/✅️apply/🦠️mutation/🔣️.json"),
                after: include_str!("../../../../✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/✅️apply/📸️snapshot/➡️after/🔣️.json"),
                diff: include_str!("../../../../✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/✅️apply/🔺️diff/🔣️.json"),
                outcome: include_str!("../../../../✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/✅️apply/🎯️outcome/🔣️.json"),
                observable: true,
            },
        "apply-directory-page-no-op" => Vector {
                before: include_str!("../../../../✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🟰️apply/📸️snapshot/⬅️before/🔣️.json"),
                mutation: include_str!("../../../../✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🟰️apply/🦠️mutation/🔣️.json"),
                after: include_str!("../../../../✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🟰️apply/📸️snapshot/➡️after/🔣️.json"),
                diff: include_str!("../../../../✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🟰️apply/🔺️diff/🔣️.json"),
                outcome: include_str!("../../../../✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🟰️apply/🎯️outcome/🔣️.json"),
                observable: false,
            },
        "apply-directory-page-rejected" => Vector {
                before: include_str!("../../../../✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🚫️apply/📸️snapshot/⬅️before/🔣️.json"),
                mutation: include_str!("../../../../✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🚫️apply/🦠️mutation/🔣️.json"),
                after: include_str!("../../../../✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🚫️apply/📸️snapshot/➡️after/🔣️.json"),
                diff: include_str!("../../../../✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🚫️apply/🔺️diff/🔣️.json"),
                outcome: include_str!("../../../../✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory/🚫️apply/🎯️outcome/🔣️.json"),
                observable: false,
            },
        other => return Err(format!("no committed vector for {other:?}")),
    })
}

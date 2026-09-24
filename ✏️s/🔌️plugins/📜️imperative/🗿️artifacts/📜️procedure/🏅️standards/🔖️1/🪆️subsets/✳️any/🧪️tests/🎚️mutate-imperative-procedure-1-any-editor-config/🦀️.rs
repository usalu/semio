//! 🎚️ `s.imperative.procedure` ✏️editor/🎚️config state-lane mutation case — Rust adapter.
//!
//! Recorded no-oracle decision `imperative-procedure-1-any-editor-config-state-lane-semantics`: the runner dispatches no oracle role, so every law is asserted inside
//! the subject handlers through `semio_s_plugin_stdio_test_oracle::law::vector` over the report of this crate's
//! production bridge `imperative_config_mutation_report_json`. The oracle handlers answer with the committed after- and before-snapshots read
//! literally, so the reference side exists the moment a second producer does. Handlers are registered by Scenario
//! Outline base id and read their kind from the row.

use semio_repo_test_host::{parse_json, Adapter, Context, Outcome};
use semio_s_plugin_stdio_test_oracle::law::vector::Vector;

//#region 🔖️Vectors
/// 🧫️ The committed applied vector of one kind, read literally from `✏️editor/🎚️config/🧫️fixtures`.
fn vector(kind: &str) -> Result<Vector, String> {
    Ok(match kind {
        "replace-config" => Vector {
            before: include_str!("../../✏️editor/🎚️config/🧫️fixtures/📸️replace-config/✅️replace-config-applied/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../✏️editor/🎚️config/🧫️fixtures/📸️replace-config/✅️replace-config-applied/🦠️mutation/🔣️.json"),
            after: include_str!("../../✏️editor/🎚️config/🧫️fixtures/📸️replace-config/✅️replace-config-applied/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../✏️editor/🎚️config/🧫️fixtures/📸️replace-config/✅️replace-config-applied/🔺️diff/🔣️.json"),
            outcome: include_str!("../../✏️editor/🎚️config/🧫️fixtures/📸️replace-config/✅️replace-config-applied/🎯️outcome/🔣️.json"),
            observable: true,
        },
        "set-run-output" => Vector {
            before: include_str!("../../✏️editor/🎚️config/🧫️fixtures/📤️set-run-output/✅️set-run-output-applied/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../✏️editor/🎚️config/🧫️fixtures/📤️set-run-output/✅️set-run-output-applied/🦠️mutation/🔣️.json"),
            after: include_str!("../../✏️editor/🎚️config/🧫️fixtures/📤️set-run-output/✅️set-run-output-applied/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../✏️editor/🎚️config/🧫️fixtures/📤️set-run-output/✅️set-run-output-applied/🔺️diff/🔣️.json"),
            outcome: include_str!("../../✏️editor/🎚️config/🧫️fixtures/📤️set-run-output/✅️set-run-output-applied/🎯️outcome/🔣️.json"),
            observable: true,
        },
        "set-contributions" => Vector {
            before: include_str!("../../✏️editor/🎚️config/🧫️fixtures/🧩️set-contributions/✅️set-contributions-applied/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../✏️editor/🎚️config/🧫️fixtures/🧩️set-contributions/✅️set-contributions-applied/🦠️mutation/🔣️.json"),
            after: include_str!("../../✏️editor/🎚️config/🧫️fixtures/🧩️set-contributions/✅️set-contributions-applied/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../✏️editor/🎚️config/🧫️fixtures/🧩️set-contributions/✅️set-contributions-applied/🔺️diff/🔣️.json"),
            outcome: include_str!("../../✏️editor/🎚️config/🧫️fixtures/🧩️set-contributions/✅️set-contributions-applied/🎯️outcome/🔣️.json"),
            observable: true,
        },
        other => return Err(format!("no committed vector for {other:?}")),
    })
}

/// 🟰️ The committed no-op vector of one kind: its before-snapshot already holds the value the mutation sets.
fn kept(kind: &str) -> Result<Vector, String> {
    Ok(match kind {
        "replace-config" => Vector {
            before: include_str!("../../✏️editor/🎚️config/🧫️fixtures/📸️replace-config/🟰️replace-config-no-op/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../✏️editor/🎚️config/🧫️fixtures/📸️replace-config/🟰️replace-config-no-op/🦠️mutation/🔣️.json"),
            after: include_str!("../../✏️editor/🎚️config/🧫️fixtures/📸️replace-config/🟰️replace-config-no-op/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../✏️editor/🎚️config/🧫️fixtures/📸️replace-config/🟰️replace-config-no-op/🔺️diff/🔣️.json"),
            outcome: include_str!("../../✏️editor/🎚️config/🧫️fixtures/📸️replace-config/🟰️replace-config-no-op/🎯️outcome/🔣️.json"),
            observable: false,
        },
        "set-run-output" => Vector {
            before: include_str!("../../✏️editor/🎚️config/🧫️fixtures/📤️set-run-output/🟰️set-run-output-no-op/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../✏️editor/🎚️config/🧫️fixtures/📤️set-run-output/🟰️set-run-output-no-op/🦠️mutation/🔣️.json"),
            after: include_str!("../../✏️editor/🎚️config/🧫️fixtures/📤️set-run-output/🟰️set-run-output-no-op/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../✏️editor/🎚️config/🧫️fixtures/📤️set-run-output/🟰️set-run-output-no-op/🔺️diff/🔣️.json"),
            outcome: include_str!("../../✏️editor/🎚️config/🧫️fixtures/📤️set-run-output/🟰️set-run-output-no-op/🎯️outcome/🔣️.json"),
            observable: false,
        },
        "set-contributions" => Vector {
            before: include_str!("../../✏️editor/🎚️config/🧫️fixtures/🧩️set-contributions/🟰️set-contributions-no-op/📸️snapshot/⬅️before/🔣️.json"),
            mutation: include_str!("../../✏️editor/🎚️config/🧫️fixtures/🧩️set-contributions/🟰️set-contributions-no-op/🦠️mutation/🔣️.json"),
            after: include_str!("../../✏️editor/🎚️config/🧫️fixtures/🧩️set-contributions/🟰️set-contributions-no-op/📸️snapshot/➡️after/🔣️.json"),
            diff: include_str!("../../✏️editor/🎚️config/🧫️fixtures/🧩️set-contributions/🟰️set-contributions-no-op/🔺️diff/🔣️.json"),
            outcome: include_str!("../../✏️editor/🎚️config/🧫️fixtures/🧩️set-contributions/🟰️set-contributions-no-op/🎯️outcome/🔣️.json"),
            observable: false,
        },
        other => return Err(format!("no committed no-op vector for {other:?}")),
    })
}
//#endregion 🔖️Vectors

//#region 🔖️Oracle
fn literal(text: &str) -> Result<Outcome, String> {
    Ok(Outcome::with_raw(text.as_bytes().to_vec(), parse_json(text)?))
}

fn mutate_oracle(ctx: &Context) -> Result<Outcome, String> {
    literal(vector(ctx.row()?)?.after)
}

fn inverse_oracle(ctx: &Context) -> Result<Outcome, String> {
    literal(vector(ctx.row()?)?.before)
}

fn keep_oracle(ctx: &Context) -> Result<Outcome, String> {
    literal(kept(ctx.row()?)?.after)
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::*;
    use semio_s_plugin_stdio_test_oracle::law::vector;
    use semio_s_artifact_imperative_procedure::editor::procedure::config::imperative_config_mutation_report_json;

    fn report(committed: &Vector) -> Result<String, String> {
        imperative_config_mutation_report_json(committed.before, committed.mutation, committed.after)
    }

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let kind = ctx.row()?;
        let committed = vector(kind)?;
        let applied = vector::mutate(kind, &report(&committed)?, &committed)?;
        Ok(Outcome::with_raw(applied.to_string().into_bytes(), applied))
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let kind = ctx.row()?;
        let restored = vector::inverse(kind, &report(&vector(kind)?)?)?;
        Ok(Outcome::with_raw(restored.to_string().into_bytes(), restored))
    }

    pub fn keep(ctx: &Context) -> Result<Outcome, String> {
        let kind = ctx.row()?;
        let committed = kept(kind)?;
        let unchanged = vector::mutate(kind, &report(&committed)?, &committed)?;
        Ok(Outcome::with_raw(unchanged.to_string().into_bytes(), unchanged))
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust").oracle("mutate", mutate_oracle).oracle("inverse", inverse_oracle).oracle("keep", keep_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("mutate", subject::mutate).subject("inverse", subject::inverse).subject("keep", subject::keep);
    }
    built
}
//#endregion 🔖️Registration

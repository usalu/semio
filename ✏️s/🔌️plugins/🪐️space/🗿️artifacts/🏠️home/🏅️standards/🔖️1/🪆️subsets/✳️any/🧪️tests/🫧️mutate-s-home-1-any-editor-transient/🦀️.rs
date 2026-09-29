//! 🫧️ `s.space.home` ✏️editor/🫧️transient state-lane mutation case — Rust adapter.
//!
//! Recorded no-oracle decision `s-home-1-any-editor-transient-state-lane-semantics`: the runner dispatches no oracle role, so
//! every law is asserted inside the subject handler through `semio_s_plugin_stdio_test_oracle::law::vector` over the report
//! of this crate's production bridge `home_transient_mutation_report_json`. The oracle handler answers with the committed
//! after- and before-snapshots read literally, so the reference side exists the moment a second producer does. The one verb
//! is non-invertible (a derived projection page is never undone): its inverse law is asserted on the vectors that fold
//! nothing, where no inverse step may exist and the projection must stay the committed before-snapshot.

use semio_repo_test_host::{parse_json, Adapter, Context, Outcome};
use semio_s_plugin_stdio_test_oracle::law::vector::Vector;

//#region 🔖️Vectors
/// 🧫️ One committed vector of `apply-directory-page`, read literally from `✏️editor/🫧️transient/🧫️fixtures`.
fn vector(id: &str) -> Result<Vector, String> {
    macro_rules! committed {
        ($name:literal, $observable:expr) => {
            Vector {
                before: include_str!(concat!("../../✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory-page/", $name, "/📸️snapshot/⬅️before/🔣️.json")),
                mutation: include_str!(concat!("../../✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory-page/", $name, "/🦠️mutation/🔣️.json")),
                after: include_str!(concat!("../../✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory-page/", $name, "/📸️snapshot/➡️after/🔣️.json")),
                diff: include_str!(concat!("../../✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory-page/", $name, "/🔺️diff/🔣️.json")),
                outcome: include_str!(concat!("../../✏️editor/🫧️transient/🧫️fixtures/📬️apply-directory-page/", $name, "/🎯️outcome/🔣️.json")),
                observable: $observable,
            }
        };
    }
    Ok(match id {
        "apply-directory-page-applied" => committed!("✅️apply-directory-page-applied", true),
        "apply-directory-page-no-op" => committed!("🟰️apply-directory-page-no-op", false),
        "apply-directory-page-rejected" => committed!("🚫️apply-directory-page-rejected", false),
        other => return Err(format!("no committed vector for {other:?}")),
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
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::*;
    use semio_s_artifact_space_home::editor::home::transient::mutations::home_transient_mutation_report_json;
    use semio_s_plugin_stdio_test_oracle::law::vector;

    pub fn mutate(ctx: &Context) -> Result<Outcome, String> {
        let id = ctx.row()?;
        let committed = vector(id)?;
        let report = home_transient_mutation_report_json(committed.before, committed.mutation, committed.after)?;
        let applied = vector::mutate(id, &report, &committed)?;
        Ok(Outcome::with_raw(applied.to_string().into_bytes(), applied))
    }

    pub fn inverse(ctx: &Context) -> Result<Outcome, String> {
        let id = ctx.row()?;
        let committed = vector(id)?;
        let report = home_transient_mutation_report_json(committed.before, committed.mutation, committed.after)?;
        let restored = vector::inverse(id, &report)?;
        Ok(Outcome::with_raw(restored.to_string().into_bytes(), restored))
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust").oracle("mutate", mutate_oracle).oracle("inverse", inverse_oracle);
    #[cfg(feature = "sut")]
    {
        built = built.subject("mutate", subject::mutate).subject("inverse", subject::inverse);
    }
    built
}
//#endregion 🔖️Registration

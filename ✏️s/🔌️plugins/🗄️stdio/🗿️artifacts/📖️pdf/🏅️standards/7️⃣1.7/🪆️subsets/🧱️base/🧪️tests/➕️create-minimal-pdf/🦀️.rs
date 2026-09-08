//! 🦀️ PDF creation case — Rust adapter.
//!
//! `oracle` drives the registered `pdf-writer` reference implementation; `subject` drives this
//! repository's own `encode_pdf`. Both results are read back by an INDEPENDENT parser (`lopdf`)
//! before the `semantic-pdf-v1` profile compares them, so neither producer is ever checked against
//! its own reading of what it wrote.
//!
//! The subject half is gated behind the generated host's `sut` feature: the oracle-only run must
//! pass WITHOUT compiling or invoking the local implementation, which is what makes "the reference
//! library really does support this case" provable before any repository code exists.

use semio_s_plugin_stdio_test_oracle::document::{oracle_create_pdf, project_pdf, PdfPageSpec, PdfSpec};
use semio_repo_test_host::{Adapter, Context, Outcome};

//#region 🔖️Specs
/// 📐️ The one owned description of each scenario's document, shared by the oracle and the subject
/// so both are provably driven by the same input.
fn spec_for(scenario: &str) -> Result<PdfSpec, String> {
    let a4 = |content: &str| PdfPageSpec { media_box: [0.0, 0.0, 595.0, 842.0], content: content.to_string() };
    match scenario {
        "one-empty-a4-page" => Ok(PdfSpec { version: (1, 7), pages: vec![a4("")], title: None, author: None }),
        "three-empty-pages" => Ok(PdfSpec { version: (1, 7), pages: vec![a4(""), a4(""), a4("")], title: None, author: None }),
        "document-title-and-author" => Ok(PdfSpec { version: (1, 7), pages: vec![a4("")], title: Some("Semio Conformance".to_string()), author: Some("semio".to_string()) }),
        other => Err(format!("no spec declared for scenario {}", other)),
    }
}

const SCENARIOS: [&str; 3] = ["one-empty-a4-page", "three-empty-pages", "document-title-and-author"];
//#endregion 🔖️Specs

//#region 🔖️Oracle
fn oracle(ctx: &Context) -> Result<Outcome, String> {
    let bytes = oracle_create_pdf(&spec_for(&ctx.scenario.id)?)?;
    let projection = project_pdf(&bytes)?;
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::{spec_for, PdfSpec};
    use semio_s_plugin_stdio_test_oracle::document::project_pdf;
    use semio_repo_test_host::{Context, Outcome};
    use semio_s_plugin_stdio::artifacts::pdf::standards::v1_7::subsets::base::io::encode_pdf;
    use semio_s_plugin_stdio::artifacts::pdf::standards::v1_7::subsets::base::schema::snapshot::{PdfInfo, PdfPage, PdfSnapshot};

    /// 🔁️ Translates the owned spec into this repository's typed PDF snapshot.
    fn snapshot_for(spec: &PdfSpec) -> PdfSnapshot {
        PdfSnapshot {
            schema: "stdio.pdf.1.7".to_string(),
            declared_version: format!("{}.{}", spec.version.0, spec.version.1),
            pages: spec.pages.iter().map(|page| PdfPage { media_box: [page.media_box[0] as f64, page.media_box[1] as f64, page.media_box[2] as f64, page.media_box[3] as f64], crop_box: None, rotate: 0, text: page.content.clone() }).collect(),
            info: PdfInfo { title: spec.title.clone(), author: spec.author.clone(), subject: None, keywords: None, creator: None, producer: None },
            objects: Vec::new(),
            trailer: Vec::new(),
        }
    }

    pub fn run(ctx: &Context) -> Result<Outcome, String> {
        let bytes = encode_pdf(&snapshot_for(&spec_for(&ctx.scenario.id)?)).map_err(|error| format!("encode_pdf failed: {:?}", error))?;
        let projection = project_pdf(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let mut built = Adapter::new("rust");
    for scenario in SCENARIOS {
        built = built.oracle(scenario, oracle);
        #[cfg(feature = "sut")]
        {
            built = built.subject(scenario, subject::run);
        }
    }
    built
}
//#endregion 🔖️Registration

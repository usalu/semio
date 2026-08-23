//! 🦀️ PDF editing case — Rust adapter.
//!
//! Every scenario copies the immutable input fixture into the case work directory first; the
//! committed fixture is never written to. `oracle` drives the registered `lopdf` reference
//! implementation, `subject` drives this repository's own decode/encode round trip, and both
//! results are read back by an independent parser before the `semantic-pdf-v1` profile compares
//! them. The subject half is gated behind the generated host's `sut` feature so the oracle-only run
//! never compiles the local implementation.

use semio_repo_test_host::oracle::{oracle_delete_page, oracle_replace_metadata, project_pdf};
use semio_repo_test_host::{Adapter, Context, Outcome};

//#region 🔖️Input
const INPUT: &str = "local://📄️two-pages.pdf";

/// 🧫️ Copies the immutable fixture into the work directory and returns the mutable copy's bytes.
fn mutable_input(ctx: &Context) -> Result<Vec<u8>, String> {
    let copy = ctx.copy_fixture(INPUT, Some("input.pdf"))?;
    std::fs::read(&copy).map_err(|error| error.to_string())
}
//#endregion 🔖️Input

//#region 🔖️Oracle
fn replace_metadata_oracle(ctx: &Context) -> Result<Outcome, String> {
    let bytes = oracle_replace_metadata(&mutable_input(ctx)?, Some("Replaced Title"), Some("Replaced Author"))?;
    let projection = project_pdf(&bytes)?;
    Ok(Outcome::with_raw(bytes, projection))
}

fn delete_page_oracle(ctx: &Context) -> Result<Outcome, String> {
    let bytes = oracle_delete_page(&mutable_input(ctx)?, 2)?;
    let projection = project_pdf(&bytes)?;
    Ok(Outcome::with_raw(bytes, projection))
}
//#endregion 🔖️Oracle

//#region 🔖️Subject
#[cfg(feature = "sut")]
mod subject {
    use super::mutable_input;
    use semio_repo_test_host::oracle::project_pdf;
    use semio_repo_test_host::{Context, Outcome};
    use semio_s_plugin_stdio::artifacts::pdf::standards::v1_7::subsets::any::io::{decode_pdf, encode_pdf};

    pub fn replace_metadata(ctx: &Context) -> Result<Outcome, String> {
        let mut snapshot = decode_pdf(&mutable_input(ctx)?).map_err(|error| format!("decode_pdf failed: {:?}", error))?;
        snapshot.info.title = Some("Replaced Title".to_string());
        snapshot.info.author = Some("Replaced Author".to_string());
        let bytes = encode_pdf(&snapshot).map_err(|error| format!("encode_pdf failed: {:?}", error))?;
        let projection = project_pdf(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }

    pub fn delete_page(ctx: &Context) -> Result<Outcome, String> {
        let mut snapshot = decode_pdf(&mutable_input(ctx)?).map_err(|error| format!("decode_pdf failed: {:?}", error))?;
        if snapshot.pages.len() < 2 {
            return Err(format!("input document has {} page(s), expected at least two", snapshot.pages.len()));
        }
        snapshot.pages.remove(1);
        snapshot.objects.clear();
        snapshot.trailer.clear();
        let bytes = encode_pdf(&snapshot).map_err(|error| format!("encode_pdf failed: {:?}", error))?;
        let projection = project_pdf(&bytes)?;
        Ok(Outcome::with_raw(bytes, projection))
    }
}
//#endregion 🔖️Subject

//#region 🔖️Registration
/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    let built = Adapter::new("rust").oracle("replace-document-metadata", replace_metadata_oracle).oracle("delete-the-second-page", delete_page_oracle);
    #[cfg(feature = "sut")]
    {
        return built.subject("replace-document-metadata", subject::replace_metadata).subject("delete-the-second-page", subject::delete_page);
    }
    #[cfg(not(feature = "sut"))]
    built
}
//#endregion 🔖️Registration

//! 🦀️ Note document whole-document identity round trip — Rust adapter. Relocated out of the
//! artifact-level `mutate-note-1` case in ticket
//! `26/09/02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION`.
//! This scenario carries no mutation kind and no vector, so it stays with `✳️any`, the owner of the
//! committed example document, rather than joining any of the eight mutation subsets.
//!
//! The Python reference beside this file REFUSES this scenario by clause (see its own docstring),
//! so this file registers the SUBJECT half only.

use semio_repo_test_host::Adapter;

/// 📄️ The real committed example document, in this subset's own `.dsl.semio` text envelope.
#[cfg(feature = "sut")]
const EXAMPLE_ASSET: &str = "asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio";

#[cfg(feature = "sut")]
mod subject {
    use super::EXAMPLE_ASSET;
    use semio_repo_test_host::{parse_json, Context, Outcome};
    use semio_s_plugin_stdio_test_oracle::law;
    use semio_s_plugin_note::artifacts::note::standards::v1::subsets::any::io::snapshot::text::{parse_note_dsl, print_note_dsl};
    use semio_s_plugin_note::artifacts::note::standards::v1::subsets::any::schema::mutations::encode_note_snapshot_json;

    fn projection(snapshot: &semio_s_plugin_note::artifacts::note::NoteSnapshot) -> Result<semio_repo_test_host::Json, String> {
        parse_json(&encode_note_snapshot_json(snapshot))
    }

    /// 🔁️ The real committed artifact, parsed and printed back. Two laws, both in role: the
    /// reparsed document must carry the same projection, and the printed text must reproduce the
    /// committed bytes EXACTLY. The exact-bytes half is `carrier_is_exact` rather than the wave's
    /// usual no-byte-pass-through tripwire because the committed `🗣️.dsl.semio` is this
    /// subset's OWN printer's output — the repository generated it from this very codec — so
    /// reproducing it is the correct answer and any drift between the committed artifact and the
    /// printer is the defect this scenario exists to catch.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let committed = ctx.fixture_bytes(EXAMPLE_ASSET)?;
        let text = String::from_utf8(committed.clone()).map_err(|error| format!("identity-round-trip: the committed artifact is not UTF-8: {error}"))?;
        let parsed = parse_note_dsl(&text)?;
        let printed = print_note_dsl(&parsed);
        let reparsed = parse_note_dsl(&printed)?;
        let (before, after) = (projection(&parsed)?, projection(&reparsed)?);
        law::round_trip_preserves(&after, &before)?;
        law::carrier_is_exact(printed.as_bytes(), &committed)?;
        Ok(Outcome::with_raw(printed.into_bytes(), after))
    }
}

/// 🧭️ Registration entry point the generated host calls.
pub fn adapter() -> Adapter {
    #[allow(unused_mut)]
    let mut built = Adapter::new("rust");
    #[cfg(feature = "sut")]
    {
        built = built.subject("identity-round-trip", subject::round_trip);
    }
    built
}

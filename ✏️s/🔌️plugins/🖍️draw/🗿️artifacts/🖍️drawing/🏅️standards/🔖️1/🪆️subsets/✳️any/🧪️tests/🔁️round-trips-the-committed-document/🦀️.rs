//! 🖍️ Drawing-document whole-document identity round trip — Rust adapter. Relocated out of the
//! artifact-level `mutate-drawing-1` case in ticket
//! `26/09/02/SEPARATE-ARTIFACT-STANDARD-SUBSET-IMPLEMENTATIONS-AND-FIXTURE-TEST-EVERY-MUTATION`.
//! This scenario carries no mutation kind and no vector, so it stays with `✳️any`, the owner of the
//! committed example document, rather than joining any of the four mutation subsets.
//!
//! Recorded no-oracle decision `drawing-mutation-semantics` (`../../🔮️oracle/🔣️.json`) covers this
//! scenario too: the committed grammar is the generic `family-scene` canvas grammar and the
//! committed artifact carries no `layers` block at all, so a second implementation would be refused
//! by clause.

use semio_repo_test_host::Adapter;

/// 🗣️ The real committed example this artifact ships — the identity law's input, owned by this
/// subset.
#[cfg(feature = "sut")]
const DSL_ASSET: &str = "asset://📚️examples/🎬️demo/🖼️assets/🗣️.dsl.semio";

#[cfg(feature = "sut")]
mod subject {
    use super::DSL_ASSET;
    use semio_repo_test_host::{parse_json, Context, Outcome};
    use semio_s_plugin_stdio_test_oracle::law;
    use semio_s_plugin_drawing::artifacts::drawing::standards::v1::subsets::any::schema::mutations::round_trip_drawing_dsl;

    /// 🔁️ The identity law in role, on the real committed example. Its two halves are asserted
    /// separately: the reparsed document must agree with the first parse, and the reprinted text
    /// must reproduce the committed bytes. The byte half is `carrier_is_exact` rather than the
    /// wave's usual no-pass-through tripwire because the committed `🗣️.dsl.semio` is this codec's
    /// OWN canonical output, committed as the artifact's example.
    pub fn round_trip(ctx: &Context) -> Result<Outcome, String> {
        let input = ctx.fixture_bytes(DSL_ASSET)?;
        let text = String::from_utf8(input.clone()).map_err(|error| format!("identity-round-trip: the committed example is not UTF-8: {error}"))?;
        let value = parse_json(&round_trip_drawing_dsl(&text)?)?;
        let parsed = value.get("snapshot").cloned().ok_or_else(|| "the bridge answer carries no snapshot".to_string())?;
        let reparsed = value.get("reparsed").cloned().ok_or_else(|| "the bridge answer carries no reparsed document".to_string())?;
        law::round_trip_preserves(&reparsed, &parsed)?;
        law::carrier_is_exact(value.str("printed").as_bytes(), &input)?;
        Ok(Outcome::with_raw(parsed.to_string().into_bytes(), parsed))
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
